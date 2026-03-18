use anyhow::{bail, Context, Result};
use embedded_hal::i2c::I2c;
use linux_embedded_hal::I2cdev;
use std::env;
use std::thread;
use std::time::Duration;
#[cfg(feature = "sigma-rack")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "sigma-rack")]
use rumqttc::{Client, MqttOptions, QoS};
#[cfg(feature = "sigma-rack")]
use serde::Serialize;

const BMP280_CHIP_ID: u8 = 0x58;
const REG_CHIP_ID: u8 = 0xD0;
const REG_RESET: u8 = 0xE0;
const REG_STATUS: u8 = 0xF3;
const REG_CTRL_MEAS: u8 = 0xF4;
const REG_CONFIG: u8 = 0xF5;
const REG_PRESS_MSB: u8 = 0xF7;
const REG_CALIB_START: u8 = 0x88;

#[derive(Debug, Clone)]
struct Config {
    i2c_bus: String,
    address: u8,
    poll_ms: u64,
    max_reads: Option<u32>,
    node_name: String,
    #[cfg_attr(not(feature = "sigma-rack"), allow(dead_code))]
    topic: String,
}

impl Config {
    fn from_env() -> Result<Self> {
        let i2c_bus = env::var("BMP280_I2C_BUS").unwrap_or_else(|_| String::from("/dev/i2c-1"));
        let address = env_parse_u8("BMP280_I2C_ADDR", 0x76)?;
        let poll_ms = env_parse_u64("BMP280_POLL_MS", 1_000)?;
        let max_reads = match env::var("BMP280_MAX_READS") {
            Ok(raw) => Some(
                raw.parse::<u32>()
                    .with_context(|| format!("failed to parse BMP280_MAX_READS='{raw}' as u32"))?,
            ),
            Err(_) => None,
        };
        let node_name = env::var("SIGMA_NODE_NAME")
            .or_else(|_| env::var("HOSTNAME"))
            .unwrap_or_else(|_| String::from("pi"));
        let topic = env::var("BMP280_MQTT_TOPIC")
            .unwrap_or_else(|_| format!("rack/sensors/{node_name}/bmp280"));

        Ok(Self {
            i2c_bus,
            address,
            poll_ms,
            max_reads,
            node_name,
            topic,
        })
    }
}

fn env_parse_u8(key: &str, default: u8) -> Result<u8> {
    match env::var(key) {
        Ok(value) => {
            if let Some(hex) = value
                .strip_prefix("0x")
                .or_else(|| value.strip_prefix("0X"))
            {
                u8::from_str_radix(hex, 16)
                    .with_context(|| format!("failed to parse {key}='{value}' as hex u8"))
            } else {
                value
                    .parse::<u8>()
                    .with_context(|| format!("failed to parse {key}='{value}' as u8"))
            }
        }
        Err(_) => Ok(default),
    }
}

fn env_parse_u64(key: &str, default: u64) -> Result<u64> {
    match env::var(key) {
        Ok(value) => value
            .parse::<u64>()
            .with_context(|| format!("failed to parse {key}='{value}' as u64")),
        Err(_) => Ok(default),
    }
}

#[derive(Debug, Clone, Copy)]
struct Calibration {
    dig_t1: u16,
    dig_t2: i16,
    dig_t3: i16,
    dig_p1: u16,
    dig_p2: i16,
    dig_p3: i16,
    dig_p4: i16,
    dig_p5: i16,
    dig_p6: i16,
    dig_p7: i16,
    dig_p8: i16,
    dig_p9: i16,
}

#[derive(Debug, Clone, Copy)]
struct Reading {
    temperature_c: f64,
    pressure_pa: f64,
}

#[cfg(feature = "sigma-rack")]
#[derive(Serialize)]
struct SigmaReading<'a> {
    node_name: &'a str,
    sensor: &'a str,
    temperature_c: f64,
    pressure_pa: f64,
    pressure_hpa: f64,
    timestamp_ms: u128,
}

struct Bmp280<I2C> {
    i2c: I2C,
    address: u8,
    calibration: Calibration,
}

impl<I2C> Bmp280<I2C>
where
    I2C: I2c,
    <I2C as embedded_hal::i2c::ErrorType>::Error: std::error::Error + Send + Sync + 'static,
{
    fn new(mut i2c: I2C, address: u8) -> Result<Self> {
        let chip_id = read_u8(&mut i2c, address, REG_CHIP_ID)?;
        if chip_id != BMP280_CHIP_ID {
            bail!("unexpected BMP280 chip id 0x{chip_id:02x}");
        }

        write_u8(&mut i2c, address, REG_RESET, 0xB6)?;
        thread::sleep(Duration::from_millis(10));

        let calibration = read_calibration(&mut i2c, address)?;
        write_u8(&mut i2c, address, REG_CONFIG, 0b100_101_00)?;
        write_u8(&mut i2c, address, REG_CTRL_MEAS, 0b101_101_11)?;
        wait_until_ready(&mut i2c, address)?;

        Ok(Self {
            i2c,
            address,
            calibration,
        })
    }

    fn read(&mut self) -> Result<Reading> {
        wait_until_ready(&mut self.i2c, self.address)?;

        let mut raw = [0u8; 6];
        self.i2c
            .write_read(self.address, &[REG_PRESS_MSB], &mut raw)
            .context("failed to read BMP280 measurement registers")?;

        let adc_p = ((raw[0] as i32) << 12) | ((raw[1] as i32) << 4) | ((raw[2] as i32) >> 4);
        let adc_t = ((raw[3] as i32) << 12) | ((raw[4] as i32) << 4) | ((raw[5] as i32) >> 4);

        let (temperature_c, t_fine) = compensate_temperature(adc_t, &self.calibration);
        let pressure_pa = compensate_pressure(adc_p, t_fine, &self.calibration)?;

        Ok(Reading {
            temperature_c,
            pressure_pa,
        })
    }
}

fn wait_until_ready<I2C>(i2c: &mut I2C, address: u8) -> Result<()>
where
    I2C: I2c,
    <I2C as embedded_hal::i2c::ErrorType>::Error: std::error::Error + Send + Sync + 'static,
{
    for _ in 0..20 {
        let status = read_u8(i2c, address, REG_STATUS)?;
        if status & 0b1001 == 0 {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(5));
    }
    bail!("BMP280 stayed busy for too long")
}

fn read_u8<I2C>(i2c: &mut I2C, address: u8, reg: u8) -> Result<u8>
where
    I2C: I2c,
    <I2C as embedded_hal::i2c::ErrorType>::Error: std::error::Error + Send + Sync + 'static,
{
    let mut buf = [0u8; 1];
    i2c.write_read(address, &[reg], &mut buf)
        .with_context(|| format!("failed to read register 0x{reg:02x}"))?;
    Ok(buf[0])
}

fn write_u8<I2C>(i2c: &mut I2C, address: u8, reg: u8, value: u8) -> Result<()>
where
    I2C: I2c,
    <I2C as embedded_hal::i2c::ErrorType>::Error: std::error::Error + Send + Sync + 'static,
{
    i2c.write(address, &[reg, value])
        .with_context(|| format!("failed to write register 0x{reg:02x}"))?;
    Ok(())
}

fn read_calibration<I2C>(i2c: &mut I2C, address: u8) -> Result<Calibration>
where
    I2C: I2c,
    <I2C as embedded_hal::i2c::ErrorType>::Error: std::error::Error + Send + Sync + 'static,
{
    let mut buf = [0u8; 24];
    i2c.write_read(address, &[REG_CALIB_START], &mut buf)
        .context("failed to read BMP280 calibration data")?;

    Ok(Calibration {
        dig_t1: u16::from_le_bytes([buf[0], buf[1]]),
        dig_t2: i16::from_le_bytes([buf[2], buf[3]]),
        dig_t3: i16::from_le_bytes([buf[4], buf[5]]),
        dig_p1: u16::from_le_bytes([buf[6], buf[7]]),
        dig_p2: i16::from_le_bytes([buf[8], buf[9]]),
        dig_p3: i16::from_le_bytes([buf[10], buf[11]]),
        dig_p4: i16::from_le_bytes([buf[12], buf[13]]),
        dig_p5: i16::from_le_bytes([buf[14], buf[15]]),
        dig_p6: i16::from_le_bytes([buf[16], buf[17]]),
        dig_p7: i16::from_le_bytes([buf[18], buf[19]]),
        dig_p8: i16::from_le_bytes([buf[20], buf[21]]),
        dig_p9: i16::from_le_bytes([buf[22], buf[23]]),
    })
}

fn compensate_temperature(adc_t: i32, cal: &Calibration) -> (f64, i32) {
    let var1 = (((adc_t >> 3) - ((cal.dig_t1 as i32) << 1)) * (cal.dig_t2 as i32)) >> 11;
    let var2 = (((((adc_t >> 4) - (cal.dig_t1 as i32)) * ((adc_t >> 4) - (cal.dig_t1 as i32)))
        >> 12)
        * (cal.dig_t3 as i32))
        >> 14;
    let t_fine = var1 + var2;
    let temperature = ((t_fine * 5 + 128) >> 8) as f64 / 100.0;
    (temperature, t_fine)
}

fn compensate_pressure(adc_p: i32, t_fine: i32, cal: &Calibration) -> Result<f64> {
    let mut var1: i64 = t_fine as i64 - 128_000;
    let mut var2: i64 = var1 * var1 * cal.dig_p6 as i64;
    var2 += (var1 * cal.dig_p5 as i64) << 17;
    var2 += (cal.dig_p4 as i64) << 35;
    var1 = ((var1 * var1 * cal.dig_p3 as i64) >> 8) + ((var1 * cal.dig_p2 as i64) << 12);
    var1 = ((((1_i64) << 47) + var1) * cal.dig_p1 as i64) >> 33;

    if var1 == 0 {
        bail!("invalid BMP280 calibration: dig_p1 produced division by zero")
    }

    let mut pressure: i64 = 1_048_576 - adc_p as i64;
    pressure = (((pressure << 31) - var2) * 3_125) / var1;
    var1 = (cal.dig_p9 as i64 * (pressure >> 13) * (pressure >> 13)) >> 25;
    var2 = (cal.dig_p8 as i64 * pressure) >> 19;
    pressure = ((pressure + var1 + var2) >> 8) + ((cal.dig_p7 as i64) << 4);

    Ok(pressure as f64 / 256.0)
}

#[cfg(feature = "sigma-rack")]
fn publish_reading(config: &Config, reading: Reading) -> Result<()> {
    let host = env::var("MQTT_HOST").unwrap_or_else(|_| String::from("192.168.50.1"));
    let port = env_parse_u64("MQTT_PORT", 1883)? as u16;
    let user = env::var("MQTT_USER")
        .context("MQTT_USER must be set when sigma-rack feature is enabled")?;
    let pass = env::var("MQTT_PASS")
        .context("MQTT_PASS must be set when sigma-rack feature is enabled")?;

    let payload = SigmaReading {
        node_name: &config.node_name,
        sensor: "bmp280",
        temperature_c: reading.temperature_c,
        pressure_pa: reading.pressure_pa,
        pressure_hpa: reading.pressure_pa / 100.0,
        timestamp_ms: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
    };

    let mut options = MqttOptions::new(format!("bmp280-{}", config.node_name), host, port);
    options.set_credentials(user, pass);
    options.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(options, 10);
    client
        .publish(
            config.topic.clone(),
            QoS::AtLeastOnce,
            false,
            serde_json::to_vec(&payload)?,
        )
        .context("failed to publish MQTT reading")?;
    let _ = connection.iter().next();
    Ok(())
}

#[cfg(not(feature = "sigma-rack"))]
fn publish_reading(_config: &Config, _reading: Reading) -> Result<()> {
    Ok(())
}

fn main() -> Result<()> {
    let config = Config::from_env()?;
    let i2c = I2cdev::new(&config.i2c_bus)
        .with_context(|| format!("failed to open I2C bus {}", config.i2c_bus))?;
    let mut sensor = Bmp280::new(i2c, config.address)?;

    let mut count = 0u32;
    loop {
        let reading = sensor.read()?;
        println!(
            "BMP280 {} temp={:.2} C pressure={:.2} hPa ({:.0} Pa)",
            config.node_name,
            reading.temperature_c,
            reading.pressure_pa / 100.0,
            reading.pressure_pa,
        );

        publish_reading(&config, reading)?;

        count = count.saturating_add(1);
        if config.max_reads.is_some_and(|limit| count >= limit) {
            break;
        }

        thread::sleep(Duration::from_millis(config.poll_ms));
    }

    Ok(())
}
