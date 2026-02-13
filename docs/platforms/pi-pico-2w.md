# Raspberry Pi Pico 2W Platform Profile

**Official Name:** Raspberry Pi Pico 2W  
**Chip:** RP2350 (dual-core ARM Cortex-M33 OR dual-core RISC-V Hazard3)  
**Clock Speed:** Up to 150 MHz  
**RAM:** 520 KB SRAM  
**Flash:** 4MB external QSPI  
**WiFi:** CYW43439 (802.11n, 2.4 GHz only)  
**Bluetooth:** BLE 5.2  

**Best for:** Low-cost WiFi projects, learning embedded Rust, battery-powered sensors

---

## Quick Reference

### Default I2C Pins
- **I2C0 SDA:** GPIO4 (or GPIO0, GPIO8, GPIO12, GPIO16, GPIO20)
- **I2C0 SCL:** GPIO5 (or GPIO1, GPIO9, GPIO13, GPIO17, GPIO21)
- **I2C1 SDA:** GPIO6 (or GPIO2, GPIO10, GPIO14, GPIO18, GPIO26)
- **I2C1 SCL:** GPIO7 (or GPIO3, GPIO11, GPIO15, GPIO19, GPIO27)
- **Note:** I2C pins are flexible - many GPIO can be configured

### Default SPI Pins
- **SPI0 MISO:** GPIO16 (or GPIO0, GPIO4, GPIO12, GPIO20)
- **SPI0 MOSI:** GPIO19 (or GPIO3, GPIO7, GPIO15, GPIO23)
- **SPI0 SCK:** GPIO18 (or GPIO2, GPIO6, GPIO14, GPIO22)
- **SPI0 CS:** GPIO17 (or GPIO1, GPIO5, GPIO13, GPIO21)

### Default UART Pins
- **UART0 TX:** GPIO0 (or GPIO12, GPIO16, GPIO28)
- **UART0 RX:** GPIO1 (or GPIO13, GPIO17, GPIO29)
- **UART1 TX:** GPIO4 (or GPIO8, GPIO20, GPIO24)
- **UART1 RX:** GPIO5 (or GPIO9, GPIO21, GPIO25)

### PWM
- **All GPIO pins support PWM** (8 slices, 16 channels)
- Can control 16 independent PWM outputs

### ADC
- **ADC0:** GPIO26
- **ADC1:** GPIO27
- **ADC2:** GPIO28
- **ADC3:** GPIO29
- **ADC4:** Internal temperature sensor
- **Resolution:** 12-bit (0-4095)
- **Reference:** 3.3V

### Onboard Hardware
- **LED:** GPIO25 (but controlled via WiFi chip on 2W!)
- **BOOTSEL Button:** Used to enter bootloader mode
- **VBUS Sense:** GPIO24 (detects USB power)

---

## Toolchain Setup

### Install Rust Toolchain
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add Cortex-M33 target (for ARM core)
rustup target add thumbv8m.main-none-eabihf

# Or add RISC-V target (for RISC-V core)
rustup target add riscv32imac-unknown-none-elf

# Install tools
cargo install probe-rs --features cli
cargo install elf2uf2-rs
cargo install flip-link
```

### Install Debugger (Optional)
```bash
# For probe-rs with debug probe
cargo install probe-rs --features cli

# For picotool (official Raspberry Pi tool)
sudo apt install cmake gcc-arm-none-eabi build-essential
git clone https://github.com/raspberrypi/picotool.git
cd picotool
mkdir build && cd build
cmake ..
make
sudo make install
```

---

## Project Setup

### Create New Project
```bash
# Using rp-pico template (recommended)
cargo install cargo-generate
cargo generate rp-rs/rp2040-project-template

# Select:
# - Project name: your-project-name
# - Board: rp-pico-2w
```

### Minimal Cargo.toml
```toml
[package]
name = "pico2w-project"
version = "0.1.0"
edition = "2021"

[dependencies]
cortex-m = "0.7"
cortex-m-rt = "0.7"
embedded-hal = "1.0"
rp2040-hal = "0.9"
panic-halt = "0.2"

# For WiFi
cyw43 = "0.1"
cyw43-pio = "0.1"
embassy-executor = { version = "0.5", features = ["arch-cortex-m", "executor-thread"] }
embassy-time = "0.3"
embassy-rp = { version = "0.1", features = ["time-driver"] }

[profile.release]
opt-level = "z"
lto = true
```

### Memory Configuration (.cargo/config.toml)
```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "elf2uf2-rs -d"
rustflags = [
  "-C", "link-arg=--nmagic",
  "-C", "link-arg=-Tlink.x",
  "-C", "inline-threshold=5",
  "-C", "no-vectorize-loops",
]

[build]
target = "thumbv8m.main-none-eabihf"

[alias]
rb = "build --release"
rr = "run --release"
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

### Flash Methods

**Method 1: UF2 Bootloader (Easiest)**
```bash
# 1. Hold BOOTSEL button while plugging in USB
# 2. Pico appears as USB drive (RPI-RP2)
# 3. Build and convert to UF2
cargo build --release
elf2uf2-rs target/thumbv8m.main-none-eabihf/release/your-project

# 4. Copy to drive (or use runner in config)
cp target/thumbv8m.main-none-eabihf/release/your-project.uf2 /media/$USER/RPI-RP2/

# Pico automatically reboots and runs code
```

**Method 2: Using probe-rs (with debug probe)**
```bash
probe-rs run --chip RP2350 target/thumbv8m.main-none-eabihf/release/your-project
```

**Method 3: Using picotool**
```bash
picotool load -v your-project.uf2
picotool reboot
```

### Monitor Serial Output
```bash
# Using screen
screen /dev/ttyACM0 115200

# Using minicom
minicom -D /dev/ttyACM0 -b 115200

# Exit screen: Ctrl+A, then K, then Y
```

---

## GPIO Pin Map (Pico 2W)
```
                  Raspberry Pi Pico 2W
         ╔════════════════════════════════╗
    GP0  ║ 1  ●     [USB]          ● 40  ║ VBUS
    GP1  ║ 2  ●                    ● 39  ║ VSYS
    GND  ║ 3  ●                    ● 38  ║ GND
    GP2  ║ 4  ●                    ● 37  ║ 3V3_EN
    GP3  ║ 5  ●                    ● 36  ║ 3V3(OUT)
    GP4  ║ 6  ●   ┌────────────┐   ● 35  ║ ADC_VREF
    GP5  ║ 7  ●   │            │   ● 34  ║ GP28 (ADC2)
    GND  ║ 8  ●   │   RP2350   │   ● 33  ║ GND
    GP6  ║ 9  ●   │            │   ● 32  ║ GP27 (ADC1)
    GP7  ║ 10 ●   │            │   ● 31  ║ GP26 (ADC0)
    GP8  ║ 11 ●   └────────────┘   ● 30  ║ RUN
    GP9  ║ 12 ●                    ● 29  ║ GP22
    GND  ║ 13 ●                    ● 28  ║ GND
    GP10 ║ 14 ●                    ● 27  ║ GP21
    GP11 ║ 15 ●                    ● 26  ║ GP20
    GP12 ║ 16 ●                    ● 25  ║ GP19
    GP13 ║ 17 ●                    ● 24  ║ GP18
    GND  ║ 18 ●                    ● 23  ║ GND
    GP14 ║ 19 ●                    ● 22  ║ GP17
    GP15 ║ 20 ●                    ● 21  ║ GP16
         ╚════════════════════════════════╝

Special pins:
- GP25: Onboard LED (via CYW43 WiFi chip - not direct GPIO!)
- GP23: WiFi power save
- GP24: VBUS sense (USB power detection)
- GP29: ADC3 / VSYS voltage monitor
- BOOTSEL: Hold on power-up to enter bootloader
```

**Safe GPIO for general use:** 0-22, 26-28  
**Special/Reserved:** 23 (WiFi), 24 (VBUS), 25 (LED), 29 (VSYS monitor)

---

## Common Pin Configurations

### I2C Sensor (e.g., MPU6050, BME280)
```
Sensor  →  Pico 2W
VCC     →  3.3V (pin 36)
GND     →  GND (pin 38)
SDA     →  GP4 (pin 6)   [+ 2.2kΩ pull-up to 3.3V]
SCL     →  GP5 (pin 7)   [+ 2.2kΩ pull-up to 3.3V]
```

### SPI Display (e.g., ST7789)
```
Display →  Pico 2W
VCC     →  3.3V
GND     →  GND
SCK     →  GP18 (pin 24)
MOSI    →  GP19 (pin 25)
CS      →  GP17 (pin 22)
DC      →  GP16 (pin 21)
RST     →  GP15 (pin 20)
```

### Servo Motor
```
Servo   →  Pico 2W
Red     →  VBUS (5V, pin 40) - External power better!
Brown   →  GND
Orange  →  GP2 (pin 4) - PWM

⚠️  Use external 5V supply for multiple servos
```

### Motor Driver (L298N)
```
L298N   →  Pico 2W
IN1     →  GP6  (pin 9)
IN2     →  GP7  (pin 10)
IN3     →  GP8  (pin 11)
IN4     →  GP9  (pin 12)
ENA     →  GP2  (pin 4) - PWM
ENB     →  GP3  (pin 5) - PWM
GND     →  GND
```

---

## WiFi Configuration

### Basic WiFi Setup
```rust
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0};
use cyw43_pio::PioSpi;
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, Output<'static, PIN_23>, PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>>
) -> ! {
    runner.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize WiFi chip
    let p = embassy_rp::init(Default::default());
    
    // WiFi configuration
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");
    
    // Spawn WiFi task
    spawner.spawn(wifi_task(runner)).unwrap();
    
    // Connect to network
    control.join_wpa2("SSID", "PASSWORD").await.unwrap();
}
```

**Note:** WiFi on Pico 2W is more complex than ESP32 - requires Embassy async runtime.

---

## Power Requirements

- **Operating Voltage:** 1.8V - 5.5V (typical 3.3V or 5V via USB)
- **USB Power:** 5V via micro-USB
- **Current Draw:**
  - Idle: ~1-2mA
  - WiFi active: ~100-150mA
  - Peak: ~200mA
- **Power Supply Recommendation:** 
  - USB: 500mA sufficient for development
  - Battery: 3.7V LiPo with regulator, or 3x AA (4.5V)

**⚠️  WARNING:** Do NOT power servos or motors from Pico's 3.3V/5V pins!

---

## Troubleshooting

### "No serial port found"

**Check:**
```bash
# List USB devices
ls /dev/ttyACM* /dev/ttyUSB*

# Add user to dialout group (one-time)
sudo usermod -a -G dialout $USER
# Logout and login

# Check if device appears
lsusb | grep Pico
```

---

### "Pico not entering bootloader mode"

**Solution:**
1. Disconnect USB
2. Hold BOOTSEL button
3. Connect USB while holding button
4. Release button after 2 seconds
5. Drive should appear: `/media/$USER/RPI-RP2/`

---

### "Build error: target not found"

**Solution:**
```bash
# For ARM core
rustup target add thumbv8m.main-none-eabihf

# For RISC-V core (experimental)
rustup target add riscv32imac-unknown-none-elf
```

---

### "I2C sensor not detected"

**Debug steps:**
1. Check wiring (SDA/SCL not swapped?)
2. Verify pull-up resistors (2.2kΩ to 3.3V)
3. Check power: 3.3V on sensor VCC
4. Try I2C scanner code
5. Verify GPIO pin selection in code

---

### "WiFi won't connect"

**Common issues:**
- 5GHz network (Pico 2W only supports 2.4GHz)
- WPA3 encryption (use WPA2)
- Firmware files missing (need cyw43 blobs)
- Hidden SSID (make network visible)
- Embassy executor not configured

---

## Performance Notes

- **CPU Speed:** 150 MHz max (overclockable to ~300 MHz unofficially)
- **WiFi Range:** ~50m open air, ~10-20m indoor
- **I2C Speed:** Typically 100kHz or 400kHz
- **SPI Speed:** Up to 62.5 MHz
- **PWM Resolution:** 16-bit (65535 levels)
- **ADC Sampling:** Up to 500 kSPS

---

## Pico vs Pico W vs Pico 2W

| Feature | Pico | Pico W | Pico 2W |
|---------|------|--------|---------|
| Chip | RP2040 | RP2040 | RP2350 |
| CPU | Dual M0+ 133MHz | Dual M0+ 133MHz | Dual M33/RISC-V 150MHz |
| WiFi | ❌ | ✅ 2.4GHz | ✅ 2.4GHz |
| Bluetooth | ❌ | ✅ BLE 5.2 | ✅ BLE 5.2 |
| RAM | 264KB | 264KB | 520KB |
| Flash | 2MB | 2MB | 4MB |
| ADC | 12-bit | 12-bit | 12-bit |
| Price | ~$4 | ~$6 | ~$7 |

**Use Pico 2W for:** More RAM/flash, WiFi robotics, dual-core processing  
**Use Pico W for:** Budget WiFi projects  
**Use Pico for:** No WiFi needed, lowest cost

---

## Datasheets and Resources

- [RP2350 Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)
- [Pico 2W Pinout](https://datasheets.raspberrypi.com/picow/pico-2-w-pinout.pdf)
- [Getting Started with Pico](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf)
- [RP2040 HAL Docs](https://docs.rs/rp2040-hal/)
- [Embassy RP Docs](https://docs.embassy.dev/embassy-rp/)

---

## Example Projects in This Library

- TBD: Add Pico 2W examples as they're created

---

## Changelog

- 2026-02-13: Initial platform profile for Pico 2W
- Comprehensive pin maps, toolchain setup, WiFi notes
- Troubleshooting and configuration examples

