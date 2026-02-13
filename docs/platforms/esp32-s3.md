# ESP32-S3 Platform Profile

**Official Name:** ESP32-S3  
**Architecture:** Xtensa LX7 (dual-core)  
**Clock Speed:** Up to 240 MHz  
**RAM:** 512 KB SRAM (+ optional external PSRAM)  
**Flash:** 4MB-16MB (board dependent)  
**WiFi:** 802.11 b/g/n (2.4 GHz)  
**Bluetooth:** BLE 5.0  

**Best for:** WiFi robotics, sensor hubs, IoT devices

---

## Quick Reference

### Default I2C Pins
- **SDA:** GPIO21 (configurable)
- **SCL:** GPIO22 (configurable)
- **Note:** Requires external 2.2kΩ-4.7kΩ pull-up resistors

### Default SPI Pins
- **MOSI:** GPIO23
- **MISO:** GPIO19
- **SCK:** GPIO18
- **CS:** GPIO5 (configurable)

### Default UART Pins
- **TX:** GPIO43 (USB serial)
- **RX:** GPIO44 (USB serial)
- **Additional UARTs available on GPIO16/17**

### PWM
- Any GPIO can be configured for PWM
- **Recommended for servos:** GPIO2, GPIO4, GPIO5
- **LEDC channels:** 8 available

### ADC
- **ADC1:** GPIO1-GPIO10 (can use with WiFi)
- **ADC2:** GPIO11-GPIO20 (avoid - conflicts with WiFi)
- **Resolution:** 12-bit (0-4095)
- **Note:** Use ADC1 pins for robotics (ADC2 stops working when WiFi is active)

---

## Toolchain Setup

### Install Rust ESP Toolchain
```bash
# Install espup
cargo install espup

# Install ESP Rust toolchain
espup install

# Source the environment (add to ~/.bashrc)
. $HOME/export-esp.sh

# Verify installation
rustc --version --verbose | grep host
```

### Install Flash Tools
```bash
# Install espflash
cargo install espflash

# Install cargo-espflash (integrated with cargo)
cargo install cargo-espflash

# Verify
espflash --version
```

### Add Rust Target
```bash
# For ESP32-S3 (Xtensa)
rustup target add xtensa-esp32s3-none-elf

# Verify
rustup target list | grep xtensa
```

---

## Project Setup

### Create New Project
```bash
# Using esp-template (recommended)
cargo generate esp-rs/esp-template

# Select options:
# - MCU: esp32s3
# - Advanced: false (for beginners)
```

### Minimal Cargo.toml
```toml
[package]
name = "esp32s3-project"
version = "0.1.0"
edition = "2021"

[dependencies]
esp-hal = { version = "0.17", features = ["esp32s3"] }
esp-backtrace = { version = "0.11", features = ["esp32s3", "panic-handler", "println"] }
esp-println = { version = "0.9", features = ["esp32s3"] }

[profile.release]
opt-level = "z"
lto = true
```

---

## Build and Flash

### Build
```bash
# Debug build
cargo build

# Release build (optimized, smaller)
cargo build --release
```

### Flash
```bash
# Flash and monitor serial output
espflash flash --monitor target/xtensa-esp32s3-none-elf/release/[project-name]

# Or use cargo-espflash
cargo espflash flash --release --monitor

# Flash specific port
espflash flash --port /dev/ttyUSB0 --monitor [binary]
```

### Monitor Serial Only
```bash
# Using espflash
espflash monitor /dev/ttyUSB0

# Using screen
screen /dev/ttyUSB0 115200

# Using minicom
minicom -D /dev/ttyUSB0 -b 115200
```

---

## GPIO Pin Map (ESP32-S3 DevKit)
```
                    ESP32-S3 DevKit
         ╔════════════════════════════════╗
    3V3  ║ 1  ●                      ● 2  ║ GND
   EN    ║ 3  ●                      ● 4  ║ GPIO46
 GPIO4   ║ 5  ●                      ● 6  ║ GPIO45
 GPIO5   ║ 7  ●                      ● 8  ║ GPIO0
 GPIO6   ║ 9  ●                      ● 10 ║ GPIO35
 GPIO7   ║ 11 ●                      ● 12 ║ GPIO36
 GPIO15  ║ 13 ●                      ● 14 ║ GPIO37
 GPIO16  ║ 15 ●                      ● 16 ║ GPIO38
 GPIO17  ║ 17 ●                      ● 18 ║ GPIO39
 GPIO18  ║ 19 ●                      ● 20 ║ GPIO40
 GPIO8   ║ 21 ●      [USB]           ● 22 ║ GPIO41
 GPIO19  ║ 23 ●                      ● 24 ║ GPIO42
 GPIO20  ║ 25 ●                      ● 26 ║ TXD0 (GPIO43)
 GPIO3   ║ 27 ●                      ● 28 ║ RXD0 (GPIO44)
 GPIO46  ║ 29 ●                      ● 30 ║ GPIO2
 GPIO9   ║ 31 ●                      ● 32 ║ GPIO1
 GPIO10  ║ 33 ●                      ● 34 ║ GPIO14
 GPIO11  ║ 35 ●                      ● 36 ║ GPIO21 (I2C SDA)
 GPIO12  ║ 37 ●                      ● 38 ║ GPIO47
 GPIO13  ║ 39 ●                      ● 40 ║ GPIO48
   GND   ║ 41 ●                      ● 42 ║ GPIO22 (I2C SCL)
    5V   ║ 43 ●                      ● 44 ║ 5V
         ╚════════════════════════════════╝
```

**Safe GPIO for general use:** 1-14, 21, 47, 48  
**Avoid:** GPIO43, GPIO44 (USB serial)

---

## Common Pin Configurations

### I2C Sensor (e.g., MPU6050, BME280)
```
Sensor  →  ESP32-S3
VCC     →  3.3V
GND     →  GND
SDA     →  GPIO21 (+ 2.2kΩ pull-up to 3.3V)
SCL     →  GPIO22 (+ 2.2kΩ pull-up to 3.3V)
```

### SPI Display (e.g., ST7789)
```
Display →  ESP32-S3
VCC     →  3.3V
GND     →  GND
SCK     →  GPIO18
MOSI    →  GPIO23
CS      →  GPIO5
DC      →  GPIO2
RST     →  GPIO4
```

### Servo Motor
```
Servo   →  ESP32-S3
Red     →  5V (external power!)
Brown   →  GND
Orange  →  GPIO2 (PWM)

⚠️  DO NOT power servos from ESP32 - use external 5V supply
```

### Motor Driver (L298N)
```
L298N   →  ESP32-S3
IN1     →  GPIO5
IN2     →  GPIO6
IN3     →  GPIO7
IN4     →  GPIO8
ENA     →  GPIO2 (PWM)
ENB     →  GPIO3 (PWM)
GND     →  GND (common ground)

Power: Separate 5-12V supply for motors
```

---

## Power Requirements

- **Operating Voltage:** 3.0V - 3.6V (typically 3.3V)
- **USB Power:** 5V via USB-C
- **Current Draw:**
  - Idle: ~20mA
  - WiFi active: ~100-300mA
  - Peak: ~500mA
- **Power Supply Recommendation:** 
  - Development: USB (500mA sufficient)
  - Production: Dedicated 3.3V regulator, 1A+

**⚠️  WARNING:** Do NOT power servos or motors from ESP32 3.3V/5V pins - use external supply!

---

## Troubleshooting

### "No serial port found"

**Check:**
```bash
# List USB devices
ls /dev/ttyUSB* /dev/ttyACM*

# Add user to dialout group (one-time)
sudo usermod -a -G dialout $USER
# Logout and login for changes to take effect
```

### "Failed to connect to ESP32"

**Solutions:**
1. Hold BOOT button while plugging in USB
2. Try different USB cable (must be data cable, not charge-only)
3. Try different USB port
4. Check with: `lsusb` (should see CP2102 or CH340)

### "Build error: Xtensa toolchain not found"

**Solution:**
```bash
# Re-run espup
espup install

# Source environment
. $HOME/export-esp.sh

# Add to ~/.bashrc permanently
echo '. $HOME/export-esp.sh' >> ~/.bashrc
```

### "I2C sensor not detected"

**Debug steps:**
1. Check wiring (SDA/SCL not swapped?)
2. Verify pull-up resistors installed (2.2kΩ to 3.3V)
3. Check power: LED on sensor board lit?
4. Verify 3.3V (NOT 5V!) with multimeter
5. Try I2C scanner code (see examples)

### "WiFi won't connect"

**Common issues:**
- 5GHz network (ESP32-S3 only supports 2.4GHz)
- WPA3 (use WPA2 instead)
- Hidden SSID (make SSID visible)
- Credentials in code (check SSID/password spelling)

---

## Performance Notes

- **WiFi + BLE:** Can run simultaneously (dual radio)
- **WiFi Range:** ~50-100m open air, ~10-30m indoor
- **I2C Speed:** Typically 100kHz or 400kHz (400kHz for most sensors)
- **SPI Speed:** Up to 80 MHz
- **PWM Resolution:** 13-bit max (8192 levels)
- **ADC Sampling:** ~1 kHz typical

---

## Datasheets and Resources

- [ESP32-S3 Datasheet (PDF)](https://www.espressif.com/sites/default/files/documentation/esp32-s3_datasheet_en.pdf)
- [ESP32-S3 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf)
- [ESP-RS Book](https://esp-rs.github.io/book/)
- [ESP-HAL Documentation](https://docs.rs/esp-hal/)

---

## Example Projects in This Library

- `esp32/examples/wifi-tank/` - WiFi controlled robot
- `esp32/sensors/examples/mpu6050-basic.rs` - I2C sensor reading
- `esp32/sensors/examples/vl53l0x-distance.rs` - ToF sensor

---

## Changelog

- 2026-02-13: Initial platform profile
