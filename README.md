# Rust Embedded Library

Personal collection of Rust code for embedded systems, robotics, and IoT projects.

## 🎯 Purpose

Centralized repository of tested, working Rust code for:
- ESP32/ESP32-P4 embedded development
- Raspberry Pi projects
- ROS2 robotics integration
- Sensor drivers and motor control
- Reusable algorithms and utilities

## 📁 Structure

### ESP32
```
esp32/
├── sensors/          # Sensor drivers (IMU, GPS, LiDAR, etc.)
├── motors/           # Motor control (PWM, stepper, servo)
├── communication/    # UART, I2C, SPI, WiFi, Bluetooth
└── examples/         # Complete working examples
```

### Raspberry Pi
```
raspberry-pi/
├── gpio/             # GPIO control and interfacing
├── peripherals/      # Hardware peripheral drivers
└── ros2-integration/ # ROS2 node examples
```

### Common
```
common/
├── math/             # Mathematical utilities
├── filters/          # Signal processing (Kalman, complementary, etc.)
├── algorithms/       # Pathfinding, SLAM, etc.
└── pid-control/      # PID controllers
```

### Dependencies
```
dependencies/
├── crate-configs/    # Common Cargo.toml configurations
└── cargo-templates/  # Project templates
```

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
