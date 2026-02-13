# I2C Scanner - Debug Tool for ESP32

## Purpose

Scans the I2C bus and reports all detected devices with their addresses. This is the FIRST tool you run when debugging I2C sensor issues. It proves your wiring is correct before you try reading actual sensor data.

Use this whenever:
- Setting up a new I2C sensor
- Debugging "sensor not found" errors
- Verifying pull-up resistors are working
- Confirming I2C address (0x68 vs 0x69, etc.)

---

## Hardware Required

**Essential:**
- ESP32-S3 DevKit (or ESP32-C3/S2)
- Breadboard
- Jumper wires
- 2x 2.2kΩ resistors (pull-ups for I2C)

**Optional I2C devices to test:**
- MPU6050 IMU (address 0x68 or 0x69)
- BME280 (0x76 or 0x77)
- BMP280 (0x76 or 0x77)
- Any I2C sensor

---

## Wiring Diagram
```
I2C Device    ESP32-S3        Notes
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
VCC        →  3.3V           Never use 5V!
GND        →  GND            Common ground
SDA        →  GPIO21         With 2.2kΩ pull-up to 3.3V
SCL        →  GPIO22         With 2.2kΩ pull-up to 3.3V
```

**Pull-up Resistor Wiring:**
```
     3.3V
      |
    [2.2kΩ]
      |
    SDA (GPIO21) -----> to sensor SDA
      
     3.3V
      |
    [2.2kΩ]
      |
    SCL (GPIO22) -----> to sensor SCL
```

**Critical:**
- Pull-up resistors are REQUIRED for I2C
- 2.2kΩ to 4.7kΩ range works (2.2kΩ recommended)
- Connect pull-ups to 3.3V, NOT 5V

---

## Software Dependencies

Add to `Cargo.toml`:
```toml
[dependencies]
esp-hal = { version = "0.17", features = ["esp32s3"] }
esp-backtrace = { version = "0.11", features = ["esp32s3", "panic-handler", "println"] }
esp-println = { version = "0.9", features = ["esp32s3"] }
```

**Platform setup:** See `docs/platforms/esp32-s3.md`

---

## Build and Flash
```bash
# Build release version
cargo build --release

# Flash and monitor
espflash flash --monitor target/xtensa-esp32s3-none-elf/release/i2c-scanner

# Or use cargo-espflash
cargo espflash flash --release --monitor
```

---

## Expected Output

### With MPU6050 Connected:
```
=== ESP32 I2C Scanner ===
Scanning I2C bus on SDA=GPIO21, SCL=GPIO22

✓ Device found at address 0x68 (104)

Scan complete!
Total devices found: 1
```

### With No Devices:
```
=== ESP32 I2C Scanner ===
Scanning I2C bus on SDA=GPIO21, SCL=GPIO22

Scan complete!
Total devices found: 0

⚠️  No I2C devices found!
Check:
  - Wiring (SDA/SCL not swapped?)
  - Pull-up resistors (2.2kΩ to 3.3V)
  - Power to sensor (3.3V)
```

**Success indicators:**
- Device shows at expected address (check datasheet)
- MPU6050: Usually 0x68 (or 0x69 if AD0 pin is high)
- BME280/BMP280: Usually 0x76 or 0x77

---

## Troubleshooting

### "No devices found" but sensor is connected

**Step 1: Check wiring**
```bash
# Verify connections with multimeter:
# - SDA should have ~3.3V when idle (pull-up working)
# - SCL should have ~3.3V when idle
# - Sensor VCC should be 3.3V (NOT 5V!)
```

**Step 2: Check pull-up resistors**
- REQUIRED for I2C to work
- Measure resistance: Should be 2-5kΩ between SDA→3.3V and SCL→3.3V
- If no pull-ups: I2C will not work at all

**Step 3: Verify sensor power**
- LED on sensor board should be lit
- Measure voltage: VCC pin should be 3.3V
- Check ground connection

**Step 4: Try different pins**
```rust
// In main.rs, try different GPIO pins:
let sda = io.pins.gpio8;   // Instead of gpio21
let scl = io.pins.gpio9;   // Instead of gpio22
```

---

### "Build error: target not found"

**Solution:**
```bash
rustup target add xtensa-esp32s3-none-elf
```

For ESP32-C3 (RISC-V):
```bash
rustup target add riscv32imc-unknown-none-elf
# Change Cargo.toml features to "esp32c3"
```

---

### Device found at unexpected address

**This is normal!**
- Check sensor datasheet for all possible addresses
- MPU6050: 0x68 (AD0=low) or 0x69 (AD0=high)
- BME280: 0x76 or 0x77
- Use the detected address in your sensor code

---

## Common I2C Addresses (Reference)
```
0x68  MPU6050 (AD0=GND), DS1307 RTC
0x69  MPU6050 (AD0=VCC)
0x76  BMP280, BME280 (SDO=GND)
0x77  BMP280, BME280 (SDO=VCC)
0x29  VL53L0X ToF sensor
0x3C  SSD1306 OLED display (128x64)
0x3D  SSD1306 OLED display (128x32)
0x48  ADS1115 ADC
0x50  AT24C32 EEPROM
```

---

## Next Steps

After confirming your I2C device is detected:

1. **Note the address** - You'll need it for the sensor driver
2. **Try a basic sensor example** - Like `esp32/sensors/examples/mpu6050-basic.rs`
3. **If address doesn't match code** - Update the address constant in sensor driver
4. **Build something!** - You've proven I2C works

---

## Code Explanation

### Initialization
```rust
// Create I2C peripheral with 100kHz clock
let mut i2c = I2C::new(
    peripherals.I2C0,
    sda,
    scl,
    100u32.kHz(),  // Standard I2C speed
    &clocks,
);
```

### Scanning Logic
```rust
// Try all possible 7-bit I2C addresses (1-127)
for addr in 1..=127 {
    match i2c.write(addr, &[]) {
        Ok(_) => println!("Device found!"),  // ACK received
        Err(_) => {},  // NACK - no device
    }
}
```

---

## Reference Documents

- I2C Specification: https://www.nxp.com/docs/en/user-guide/UM10204.pdf
- Platform setup: `docs/platforms/esp32-s3.md`
- ESP-HAL I2C docs: https://docs.rs/esp-hal/

---

## Changelog

- 2026-02-13: Initial version
- Scans addresses 1-127 at 100kHz
- Detects and reports all I2C devices
