# IMU Sensor Driver (MPU6050)
Rust driver for MPU6050 IMU on ESP32.

## Hardware
- ESP32-S3/P4
- MPU6050 IMU

## Connections
- SCL -> GPIO22
- SDA -> GPIO21
- VCC -> 3.3V
- GND -> GND

## Usage
```rust
use mpu6050::Mpu6050;

let mut imu = Mpu6050::new(i2c);
let accel = imu.read_accel()?;
```

## Dependencies
```toml
mpu6050 = "0.1"
embedded-hal = "0.2"
```

## Example
See `examples/basic.rs`
