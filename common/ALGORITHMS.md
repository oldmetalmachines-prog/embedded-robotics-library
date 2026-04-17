# Common Robotics Algorithms & Crates

Essential Rust crates for robotics projects.

## Math & Linear Algebra

- **nalgebra** - Linear algebra (vectors, matrices, quaternions)
- **micromath** - Fast embedded math (sin, cos, sqrt)
- **fixed** - Fixed-point arithmetic (no floating point)
- **libm** - Math functions for no_std

## Sensor Fusion & Filtering

- **adskalman** - Kalman filter implementation
- **ukf** - Unscented Kalman Filter
- **complementary-filter** - Simple sensor fusion

## Control Systems

- **pid** - PID controller
- **simple-pid** - Lightweight PID
- **control** - Control theory primitives

## Motion & Pathfinding

- **pathfinding** - A* and Dijkstra algorithms
- **collision** - Collision detection
- **ncollide** - 3D collision detection

## Communication Protocols

- **embedded-hal** - Hardware abstraction (I2C, SPI, GPIO)
- **postcard** - Serialization for embedded
- **serde** - Serialization framework
- **heapless** - No-heap data structures

## Robotics-Specific

- **r2r** - ROS2 bindings for Rust
- **rosrust** - ROS1 bindings
- **rerun** - Visualization/logging

## Motor Drivers

- **pwm-pca9685** - I2C PWM driver (servos)
- Custom implementations for L298N, TB6612FNG

## Sensor Drivers (embedded-hal)

### IMU (Inertial Measurement)

- **mpu6050** - 6-axis IMU
- **mpu9250** - 9-axis IMU
- **bno055** - 9-axis with fusion
- **lsm303agr** - Accelerometer + magnetometer

### Distance

- **vl53l0x** - Time-of-Flight laser
- **vl53l1x** - ToF (longer range)
- **hcsr04** - Ultrasonic

### Environment

- **bmp280** - Pressure/temperature
- **bme280** - Pressure/temp/humidity
- **dht11** / **dht22** - Temp/humidity

### GPS

- **nmea** - NMEA parser
- **ublox** - u-blox GPS

### Vision

- Camera interfacing typically via OpenCV bindings

## Installation

Add to `Cargo.toml`:
```toml
[dependencies]
nalgebra = { version = "0.32", default-features = false, features = ["libm"] }
micromath = "2.0"
pid = "4.0"
adskalman = "0.5"
```

For no_std, disable default features and enable `libm`.
