# BMP280 I2C Reader

Read temperature and pressure from a BMP280 over Linux I2C using `linux-embedded-hal`.

This project is designed for direct use on the sigma rack:
- it can run on a Raspberry Pi or other Linux SBC with `/dev/i2c-*`
- the optional `sigma-rack` feature publishes readings to the rack MQTT broker
- topic names default to rack-friendly paths that Grafana or downstream consumers can scrape or bridge

## Features

- validates the BMP280 chip ID before reading
- programs oversampling and standby registers before sampling
- uses the datasheet compensation formulas for temperature and pressure
- configurable bus path, device address, poll interval, and maximum reads
- optional MQTT publication for rack integration

## Environment variables

| Variable | Default | Purpose |
| --- | --- | --- |
| `BMP280_I2C_BUS` | `/dev/i2c-1` | I2C bus path |
| `BMP280_I2C_ADDR` | `0x76` | BMP280 address as hex or decimal |
| `BMP280_POLL_MS` | `1000` | Delay between samples |
| `BMP280_MAX_READS` | unset | Stop after this many reads |
| `SIGMA_NODE_NAME` | hostname | Node name used in MQTT payloads |
| `BMP280_MQTT_TOPIC` | `rack/sensors/{node}/bmp280` | Override MQTT topic |
| `MQTT_HOST` | `192.168.50.1` | Rack MQTT broker host |
| `MQTT_PORT` | `1883` | Rack MQTT broker port |
| `MQTT_USER` | required with `sigma-rack` | MQTT username |
| `MQTT_PASS` | required with `sigma-rack` | MQTT password |

## Build

```bash
cargo check --manifest-path raspberry-pi/sensors/bmp280-i2c/Cargo.toml
cargo check --manifest-path raspberry-pi/sensors/bmp280-i2c/Cargo.toml --features sigma-rack
```

## Run

Single sample:

```bash
BMP280_MAX_READS=1 \
SIGMA_NODE_NAME=pi1 \
cargo run --manifest-path raspberry-pi/sensors/bmp280-i2c/Cargo.toml
```

Continuous sampling with rack integration:

```bash
MQTT_USER=your-user \
MQTT_PASS=your-pass \
SIGMA_NODE_NAME=pi1 \
BMP280_POLL_MS=2000 \
cargo run --manifest-path raspberry-pi/sensors/bmp280-i2c/Cargo.toml --features sigma-rack
```

## Example output

```text
BMP280 pi1 temp=23.41 C pressure=1008.72 hPa (100872 Pa)
```

## MQTT payload

With `sigma-rack` enabled, each reading is published as JSON:

```json
{
  "node_name": "pi1",
  "sensor": "bmp280",
  "temperature_c": 23.41,
  "pressure_pa": 100872.0,
  "pressure_hpa": 1008.72,
  "timestamp_ms": 1710000000000
}
```

## Wiring

Typical Raspberry Pi wiring:
- BMP280 `VIN` to `3V3`
- BMP280 `GND` to `GND`
- BMP280 `SCL` to `GPIO3 / SCL`
- BMP280 `SDA` to `GPIO2 / SDA`

If your module uses address `0x77`, set `BMP280_I2C_ADDR=0x77`.
