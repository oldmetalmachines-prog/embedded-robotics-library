# Getting Started with Your Rust Embedded Library

**Repository:** https://github.com/oldmetalmachines-prog/rust-embedded-library

A curated collection of Rust embedded code for ESP32, Raspberry Pi, and robotics projects.

---

## 📦 What's Inside

### ESP32 Development

#### Examples (5 complete projects)
- **esp32/examples/snake-game/** - OLED display + joystick game
- **esp32/examples/snake-complete/** - Complete version with full features
- **esp32/examples/wifi-tank/** - WiFi controlled robot with motors
- **esp32/examples/std-demo/** - WiFi + HTTP server + LED screen
- **esp32/sensors/temperature-logger/** - MQTT + BMP180 sensor

#### Sensor Examples (3 working examples)
- **mpu6050-basic.rs** - 6-axis IMU (accelerometer + gyroscope)
- **vl53l0x-distance.rs** - Time-of-Flight laser distance sensor
- Complete with wiring diagrams and dependencies

#### ESP32-P4 Specific
- Dual-core RISC-V robotics template
- Architecture guide for high-performance edge AI
- Core assignment recommendations

### Raspberry Pi Development

#### GPIO Examples (12 examples)
- LED control (basic, multithreaded, PWM)
- Button input with interrupts
- Servo control (hardware PWM + software PWM)
- I2C sensor (DS3231 RTC)
- SPI EEPROM
- UART serial communication

#### ROS2 Integration (2 examples)
- **ros2_imu_publisher.rs** - Publish IMU data to ROS2
- **ros2_motor_controller.rs** - Subscribe to /cmd_vel for motor control
- Complete setup guide for ROS2 Jazzy

### Dependency Templates (5 templates)

1. **esp32-robotics-nostd.toml** - Embassy async, sensors, motor control
2. **esp32-robotics-std.toml** - WiFi, MQTT, Tokio for networked robots
3. **esp32p4-robotics.toml** - Dual-core P4 with FPU and camera support
4. **raspberry-pi-robotics.toml** - RPPAL, ROS2-ready, full sensors
5. **esp32-project-template.toml** - Basic starter template

### Reference Documentation

- **ALGORITHMS.md** - Robotics algorithms (Kalman, PID, pathfinding)
- **awesome-esp-rust.md** - Comprehensive ESP32 Rust resources
- **ROS2 SETUP.md** - Complete ROS2 Jazzy installation guide

---

## 🚀 Quick Start

### 1. Clone and Explore

```bash
git clone git@github.com:oldmetalmachines-prog/rust-embedded-library.git
cd rust-embedded-library

# Browse examples
ls esp32/examples/
ls raspberry-pi/gpio/rppal-examples/
```

### 2. Start a New ESP32 Project

```bash
# Copy a template
cp dependencies/cargo-templates/esp32-robotics-nostd.toml my-robot/Cargo.toml

# Or use cargo-generate (install first)
cargo install cargo-generate
cargo generate esp-rs/esp-template
```

### 3. Start a New Raspberry Pi Project

```bash
# Create new project
cargo new rpi-robot
cd rpi-robot

# Copy template
cp ../dependencies/cargo-templates/raspberry-pi-robotics.toml Cargo.toml

# Reference examples
cp ../raspberry-pi/gpio/rppal-examples/gpio_blinkled.rs src/main.rs
```

### 4. Test a Sensor Example

```bash
# ESP32 IMU example
cd esp32/sensors/examples/
cat mpu6050-basic.rs  # Review wiring and code

# Add to your project's src/main.rs
# Wire up sensor according to comments
# cargo run --release
```

---

## 🎯 Recommended Learning Path

### Beginner (Week 1-2)
1. **Raspberry Pi LED Blink** - `raspberry-pi/gpio/rppal-examples/gpio_blinkled.rs`
2. **ESP32 Basic** - Get toolchain working, flash LED
3. **Read one sensor** - Start with MPU6050 IMU

### Intermediate (Week 3-4)
4. **Motor control** - PWM for servo or DC motor
5. **Combine sensors** - IMU + distance sensor
6. **Basic autonomous** - Obstacle avoidance with ToF sensor

### Advanced (Month 2+)
7. **ROS2 integration** - Publish sensor data
8. **Multi-device** - ESP32 + Raspberry Pi communication
9. **Full robot** - Navigation stack with sensor fusion

---

## 🔧 Hardware You'll Need

### Essential (Start Here)
- **ESP32-S3 DevKit** (~$10) - or ESP32-C3
- **Raspberry Pi 5** (8GB) - for ROS2 and main compute
- **MPU6050 IMU** (~$3) - 6-axis motion sensor
- **Breadboard + jumper wires**
- **USB cables** for programming

### Recommended Additions
- **VL53L0X ToF sensor** (~$5) - Distance measurement
- **L298N motor driver** (~$3) - For DC motors
- **Servo motors** (SG90) - Simple motion
- **ESP32-P4** - When available, for advanced projects

### Advanced Setup
- **Jetson Orin Nano** - Computer vision
- **LiDAR** - SLAM and mapping
- **Camera module** - Vision processing

---

## 📚 Key Resources

### ESP32 Rust
- [ESP-RS Book](https://esp-rs.github.io/book/) - Official guide
- [Awesome ESP Rust](https://github.com/esp-rs/awesome-esp-rust) - Curated list
- Your library: `docs/references/awesome-esp-rust.md`

### Raspberry Pi Rust
- [RPPAL Docs](https://docs.rs/rppal/) - GPIO library
- [Embedded Rust Book](https://docs.rust-embedded.org/book/)

### ROS2
- [ROS2 Jazzy Docs](https://docs.ros.org/en/jazzy/)
- [r2r Rust bindings](https://github.com/sequenceplanner/r2r)
- Your library: `raspberry-pi/ros2-integration/SETUP.md`

### Robotics Algorithms
- Your library: `common/ALGORITHMS.md`
- [Kalman Filter Guide](https://www.kalmanfilter.net/)

---

## 🏗️ Project Ideas

### Beginner Projects
1. **LED Mood Light** - WiFi-controlled RGB LED
2. **Temperature Monitor** - Log data over MQTT
3. **Distance Alarm** - Beep when object detected

### Intermediate Projects
4. **Line Following Robot** - Simple autonomous navigation
5. **Obstacle Avoiding Car** - ToF sensors + motor control
6. **Weather Station** - Multiple sensors + WiFi dashboard

### Advanced Projects
7. **SLAM Robot** - Mapping with LiDAR
8. **Computer Vision Rover** - Object detection + tracking
9. **Swarm Robots** - Multiple ESP32s coordinating
10. **ROS2 Mobile Base** - Full navigation stack

---

## 🤝 Contributing to Your Library

As you build projects, add them back:

```bash
cd ~/rust-embedded-library

# Create new example
mkdir -p esp32/examples/my-project
cp my-working-code/* esp32/examples/my-project/

# Add attribution
cat > esp32/examples/my-project/README.md << 'END'
# My Project
Hardware: ESP32-S3, [sensors]
Purpose: [what it does]
Tested: 2026-02-XX
END

# Commit
git add .
git commit -m "Add my-project example"
git push
```

---

## 🎓 Your Development Environment

### Installed Tools Needed

**For ESP32:**

```bash
# Install Rust ESP toolchain
cargo install espup
espup install

# Flash tool
cargo install espflash
cargo install cargo-espflash
```

**For Raspberry Pi:**

```bash
# Standard Rust (already have)
rustup target add aarch64-unknown-linux-gnu

# Cross-compile tools (if building on laptop)
cargo install cross
```

**For ROS2:**

```bash
# See: raspberry-pi/ros2-integration/SETUP.md
sudo apt install ros-jazzy-desktop
```

---

## 📊 Library Statistics

- **ESP32 Examples**: 5 complete projects
- **Sensor Drivers**: 3 working examples  
- **Raspberry Pi Examples**: 12 GPIO examples
- **ROS2 Examples**: 2 integration examples
- **Cargo Templates**: 5 ready-to-use templates
- **Total .rs Files**: 24+ working examples

---

## 🐛 Troubleshooting

### ESP32 Won't Flash
- Check USB cable (data-capable)
- Hold BOOT button while connecting
- Try different USB port
- `ls /dev/ttyUSB*` or `/dev/ttyACM*`

### Raspberry Pi GPIO Permission Denied

```bash
sudo usermod -a -G gpio $USER
# Logout and login
```

### ROS2 Can't Find r2r

```bash
# Source ROS2 environment
source /opt/ros/jazzy/setup.bash
# Set environment variables
export AMENT_PREFIX_PATH=/opt/ros/jazzy
```

### I2C Sensor Not Found
- Check wiring (SDA/SCL swapped?)
- Add pull-up resistors (2kΩ to 3.3V)
- Scan for devices: See sensor examples

---

## 📞 Where to Get Help

- **ESP32 Rust**: [Matrix Chat](https://matrix.to/#/#esp-rs:matrix.org)
- **Embedded Rust**: [Matrix Chat](https://matrix.to/#/#rust-embedded:matrix.org)
- **ROS2**: [ROS Discourse](https://discourse.ros.org/)
- **Your notes**: Document everything in this repo!

---

## ✅ Next Steps

1. ✅ Library created and organized
2. ✅ Examples collected and documented
3. ✅ Templates ready to use
4. ⬜ Install Rust ESP toolchain
5. ⬜ Test first ESP32 example
6. ⬜ Test first Raspberry Pi example
7. ⬜ Build your first robot!

**Happy building! 🤖🦀**
