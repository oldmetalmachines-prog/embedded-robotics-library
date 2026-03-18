# Rust Embedded Library

Personal collection of embedded systems, robotics, automotive, and IoT code. The repo is Rust-first, but it also contains C, C++, Python, and ROS2 reference implementations that are useful when porting drivers and hardware integrations.

## Purpose

Centralized repository of tested or in-progress code for:
- ESP32 and ESP32-P4 embedded development
- Raspberry Pi utilities, protocols, and GPIO projects
- Jetson Orin examples and Jetson ROS2 integrations
- Sensor drivers and motor control
- Automotive diagnostics and CAN/J1939 examples
- Reusable protocols and utilities

## Repository structure

Top-level directories currently in use:

```text
common/             Shared protocol definitions and reusable assets
drivers/            Rust drivers and hardware support crates
esp32/              ESP32 communication, utilities, sensors, and examples
farm-equipment/     ISOBUS and agricultural protocol references
j1939-automotive/   J1939 examples in C, C++, and Python
jetson-orin/        Native Jetson Orin Rust examples
jetson-ros2/        Jetson ROS2 packages and integrations
obd2-diagnostics/   OBD-II diagnostics code in multiple languages
raspberry-pi/       Raspberry Pi GPIO, protocol, utility, and ROS2 projects
servo-control/      Servo and EtherCAT reference code
```

## Rust workspace

The Rust workspace is defined in the repository root `Cargo.toml` and currently includes these crates:

```text
drivers/bme280
esp32/communication/wifi/udp-telemetry-sender
esp32/utilities/i2c-scanner
jetson-orin/examples/gpio-blink
jetson-orin/examples/rtsp-camera-streamer
raspberry-pi/gpio/servo-controller
raspberry-pi/pico-2w/examples/led-blink
raspberry-pi/ros2-integration
raspberry-pi/utilities/udp-telemetry-receiver
```

Projects outside the workspace are still valuable references, but they may be vendor drops, upstream examples, or language-specific experiments rather than first-class Rust crates.

## 🔧 Hardware Targets

- **ESP32-S3** - Sensor processing, motor control
- **ESP32-P4** - High-performance edge computing
- **Raspberry Pi Zero 2W** - Lightweight coordination
- **Raspberry Pi 5** - Main compute, ROS2 nodes
- **Jetson Orin Nano** - AI/vision processing

## 📖 Usage

Each project folder contains:
- `README.md` - Overview, hardware requirements, usage
- `Cargo.toml` - Dependencies
- `src/` - Source code
- `examples/` - Working examples with comments
- `.cargo/config.toml` - Build configuration (if needed)

## 🏷️ Naming Convention

**Projects:** `platform-function-hardware`
- `esp32-imu-mpu6050`
- `esp32-motor-pwm-control`
- `rpi-gpio-led-blink`

**Files:** `snake_case`
- `sensor_reader.rs`
- `motor_controller.rs`
- `pid_controller.rs`

## 📝 Code Header Template
```rust
//! Short description
//!
//! Hardware: ESP32-S3, MPU6050
//! Tested: YYYY-MM-DD
//! Dependencies: embedded-hal 0.2, mpu6050 0.1
//! Author: Michael
//! Purpose: Detailed purpose description
```

## 🚀 Getting Started

1. Clone the repo
2. Navigate to the project you need
3. Check the README for hardware requirements
4. Review dependencies in Cargo.toml
5. Run examples to test

## 📚 Resources

- [ESP32 Rust Book](https://esp-rs.github.io/book/)
- [Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [ROS2 Documentation](https://docs.ros.org/en/jazzy/)

## 🔒 License

MIT License - See LICENSE file
