use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[cfg(feature = "sigma-rack")]
use rumqttc::{Client, MqttOptions, QoS};
#[cfg(feature = "sigma-rack")]
use serde::Serialize;

#[derive(Debug, Clone)]
struct Config {
    pwm_chip: u32,
    pwm_channel: u32,
    period_ns: u32,
    min_pulse_ns: u32,
    max_pulse_ns: u32,
    angle_deg: f32,
    hold_ms: u64,
    disable_after_hold: bool,
    settle_ms: u64,
    #[cfg_attr(not(feature = "sigma-rack"), allow(dead_code))]
    job_id: String,
    #[cfg_attr(not(feature = "sigma-rack"), allow(dead_code))]
    node_name: String,
}

impl Config {
    fn from_env() -> Result<Self> {
        let pwm_chip = env_parse_u32("JETSON_PWM_CHIP", 0)?;
        let pwm_channel = env_parse_u32("JETSON_PWM_CHANNEL", 0)?;
        let period_ns = env_parse_u32("JETSON_PWM_PERIOD_NS", 20_000_000)?;
        let min_pulse_ns = env_parse_u32("JETSON_SERVO_MIN_PULSE_NS", 500_000)?;
        let max_pulse_ns = env_parse_u32("JETSON_SERVO_MAX_PULSE_NS", 2_500_000)?;
        let angle_deg = env_parse_f32("JETSON_SERVO_ANGLE_DEG", 90.0)?;
        let hold_ms = env_parse_u64("JETSON_SERVO_HOLD_MS", 2_000)?;
        let disable_after_hold = env_parse_bool("JETSON_SERVO_DISABLE_AFTER_HOLD", true)?;
        let settle_ms = env_parse_u64("JETSON_PWM_SETTLE_MS", 100)?;
        let node_name = env::var("SIGMA_NODE_NAME")
            .or_else(|_| env::var("HOSTNAME"))
            .unwrap_or_else(|_| String::from("jetson"));
        let job_id = env::var("SERVO_JOB_ID").unwrap_or_else(|_| default_job_id(&node_name));

        if !(0.0..=180.0).contains(&angle_deg) {
            bail!("JETSON_SERVO_ANGLE_DEG must be between 0 and 180");
        }
        if min_pulse_ns >= max_pulse_ns {
            bail!("JETSON_SERVO_MIN_PULSE_NS must be smaller than JETSON_SERVO_MAX_PULSE_NS");
        }
        if max_pulse_ns >= period_ns {
            bail!("JETSON_SERVO_MAX_PULSE_NS must be smaller than JETSON_PWM_PERIOD_NS");
        }

        Ok(Self {
            pwm_chip,
            pwm_channel,
            period_ns,
            min_pulse_ns,
            max_pulse_ns,
            angle_deg,
            hold_ms,
            disable_after_hold,
            settle_ms,
            job_id,
            node_name,
        })
    }

    fn chip_path(&self) -> PathBuf {
        PathBuf::from(format!("/sys/class/pwm/pwmchip{}", self.pwm_chip))
    }

    fn channel_path(&self) -> PathBuf {
        self.chip_path().join(format!("pwm{}", self.pwm_channel))
    }

    fn duty_cycle_ns(&self) -> u32 {
        let span = (self.max_pulse_ns - self.min_pulse_ns) as f32;
        self.min_pulse_ns + ((self.angle_deg / 180.0) * span).round() as u32
    }
}

fn env_parse_u32(key: &str, default: u32) -> Result<u32> {
    match env::var(key) {
        Ok(value) => value
            .parse::<u32>()
            .with_context(|| format!("failed to parse {key}='{value}' as u32")),
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

fn env_parse_f32(key: &str, default: f32) -> Result<f32> {
    match env::var(key) {
        Ok(value) => value
            .parse::<f32>()
            .with_context(|| format!("failed to parse {key}='{value}' as f32")),
        Err(_) => Ok(default),
    }
}

fn env_parse_bool(key: &str, default: bool) -> Result<bool> {
    match env::var(key) {
        Ok(value) => match value.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" => Ok(false),
            _ => bail!("failed to parse {key}='{value}' as bool"),
        },
        Err(_) => Ok(default),
    }
}

fn default_job_id(node_name: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{node_name}-servo-{ts}")
}

fn write_trimmed(path: &Path, value: impl ToString) -> Result<()> {
    fs::write(path, value.to_string())
        .with_context(|| format!("failed to write {}", path.display()))
}

fn wait_for_path(path: &Path, timeout: Duration) -> Result<()> {
    let start = Instant::now();
    while !path.exists() {
        if start.elapsed() > timeout {
            bail!("timed out waiting for {}", path.display());
        }
        thread::sleep(Duration::from_millis(25));
    }
    Ok(())
}

fn export_channel(config: &Config) -> Result<()> {
    let channel_path = config.channel_path();
    if channel_path.exists() {
        return Ok(());
    }

    let export_path = config.chip_path().join("export");
    write_trimmed(&export_path, config.pwm_channel)?;
    wait_for_path(&channel_path, Duration::from_secs(2))
}

fn set_enabled(config: &Config, enabled: bool) -> Result<()> {
    let enable_path = config.channel_path().join("enable");
    write_trimmed(&enable_path, if enabled { "1" } else { "0" })
}

fn configure_pwm(config: &Config) -> Result<()> {
    export_channel(config)?;

    let channel_path = config.channel_path();
    let period_path = channel_path.join("period");
    let duty_path = channel_path.join("duty_cycle");

    let _ = set_enabled(config, false);
    write_trimmed(&period_path, config.period_ns)?;
    write_trimmed(&duty_path, config.duty_cycle_ns())?;
    set_enabled(config, true)?;

    if config.settle_ms > 0 {
        thread::sleep(Duration::from_millis(config.settle_ms));
    }

    Ok(())
}

#[cfg(feature = "sigma-rack")]
#[derive(Serialize)]
struct RackStatus<'a> {
    node_name: &'a str,
    job_id: &'a str,
    state: &'a str,
    detail: String,
    angle_deg: f32,
    pwm_chip: u32,
    pwm_channel: u32,
    period_ns: u32,
    duty_cycle_ns: u32,
    timestamp_ms: u128,
}

#[cfg(feature = "sigma-rack")]
fn mqtt_publish(config: &Config, state: &str, detail: impl Into<String>) -> Result<()> {
    let host = env::var("MQTT_HOST").unwrap_or_else(|_| String::from("192.168.50.1"));
    let port = env_parse_u32("MQTT_PORT", 1883)? as u16;
    let user = env::var("MQTT_USER")
        .context("MQTT_USER must be set when sigma-rack feature is enabled")?;
    let pass = env::var("MQTT_PASS")
        .context("MQTT_PASS must be set when sigma-rack feature is enabled")?;
    let topic = env::var("JETSON_SERVO_MQTT_TOPIC")
        .unwrap_or_else(|_| format!("rack/status/{}", config.job_id));

    let payload = RackStatus {
        node_name: &config.node_name,
        job_id: &config.job_id,
        state,
        detail: detail.into(),
        angle_deg: config.angle_deg,
        pwm_chip: config.pwm_chip,
        pwm_channel: config.pwm_channel,
        period_ns: config.period_ns,
        duty_cycle_ns: config.duty_cycle_ns(),
        timestamp_ms: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
    };

    let mut options = MqttOptions::new(format!("jetson-pwm-servo-{}", config.job_id), host, port);
    options.set_credentials(user, pass);
    options.set_keep_alive(Duration::from_secs(5));

    let (client, mut connection) = Client::new(options, 10);
    client
        .publish(
            topic,
            QoS::AtLeastOnce,
            false,
            serde_json::to_vec(&payload)?,
        )
        .context("failed to publish MQTT status")?;
    let _ = connection.iter().next();
    Ok(())
}

#[cfg(not(feature = "sigma-rack"))]
fn mqtt_publish(_config: &Config, _state: &str, _detail: impl Into<String>) -> Result<()> {
    Ok(())
}

#[cfg(feature = "sigma-rack")]
fn http_notify(config: &Config, state: &str, detail: impl Into<String>) -> Result<()> {
    let url = match env::var("SIGMA_V4_STATUS_URL") {
        Ok(url) if !url.trim().is_empty() => url,
        _ => return Ok(()),
    };

    let payload = serde_json::json!({
        "node_name": config.node_name,
        "job_id": config.job_id,
        "state": state,
        "detail": detail.into(),
        "angle_deg": config.angle_deg,
        "duty_cycle_ns": config.duty_cycle_ns(),
    });

    reqwest::blocking::Client::new()
        .post(url)
        .json(&payload)
        .send()
        .context("failed to POST status to SIGMA_V4_STATUS_URL")?
        .error_for_status()
        .context("status endpoint returned an error")?;

    Ok(())
}

#[cfg(not(feature = "sigma-rack"))]
fn http_notify(_config: &Config, _state: &str, _detail: impl Into<String>) -> Result<()> {
    Ok(())
}

fn run(config: &Config) -> Result<()> {
    mqtt_publish(config, "starting", "exporting PWM channel")?;
    configure_pwm(config)?;

    let detail = format!(
        "Set pwmchip{} channel {} to {:.1} degrees ({} ns duty)",
        config.pwm_chip,
        config.pwm_channel,
        config.angle_deg,
        config.duty_cycle_ns()
    );
    mqtt_publish(config, "active", detail.clone())?;
    http_notify(config, "active", detail.clone())?;

    println!("{detail}");
    println!("Holding signal for {} ms", config.hold_ms);
    thread::sleep(Duration::from_millis(config.hold_ms));

    if config.disable_after_hold {
        set_enabled(config, false)?;
        mqtt_publish(config, "completed", "PWM disabled after hold interval")?;
        http_notify(config, "completed", "PWM disabled after hold interval")?;
        println!("PWM disabled after hold interval");
    } else {
        mqtt_publish(config, "completed", "PWM left enabled after hold interval")?;
        http_notify(config, "completed", "PWM left enabled after hold interval")?;
        println!("PWM left enabled after hold interval");
    }

    Ok(())
}

fn main() -> Result<()> {
    let config = Config::from_env()?;
    match run(&config) {
        Ok(()) => Ok(()),
        Err(err) => {
            let _ = mqtt_publish(&config, "error", err.to_string());
            let _ = http_notify(&config, "error", err.to_string());
            Err(err)
        }
    }
}
