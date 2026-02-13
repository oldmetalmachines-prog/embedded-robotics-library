# MPU6050 IMU Sensor - Basic Reading

## Purpose

Demonstrates reading accelerometer and gyroscope data from an MPU6050 6-axis IMU sensor using I2C communication. This is a fundamental building block for robotics projects requiring orientation sensing, motion detection, or stabilization.

Use cases:
- Robot balance and tilt detection
- Motion sensing for autonomous navigation
- Vibration monitoring
- Foundation for sensor fusion (Kalman filter, complementary filter)

---

## Hardware Required

**Essential:**
- ESP32-S3 DevKit (or ESP32-C3, ESP32-S2)
- MPU6050 IMU sensor module (GY-521 breakout board)
- Breadboard
- Jumper wires (male-to-male)
- 2x 2.2kΩ resistors (I2C pull-ups)

**Optional:**
- USB cable for programming and serial monitor

**Where to buy:**
- MPU6050 GY-521: ~$3 - Search "MPU6050 GY-521" on Amazon/AliExpress
- Comes with voltage regulator and pull-up resistors on some boards

---

## Wiring Diagram
```
MPU6050       ESP32-S3        Notes
(GY-521)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
VCC        →  3.3V           DO NOT use 5V on bare MPU6050!
                             (GY-521 board accepts 3.3V or 5V)
GND        →  GND            Common ground
SDA        →  GPIO21         I2C data (+ 2.2kΩ pull-up to 3.3V)
SCL        →  GPIO22         I2C clock (+ 2.2kΩ pull-up to 3.3V)
XDA        →  (not used)     Auxiliary I2C for magnetometer
XCL        →  (not used)
AD0        →  GND            Sets I2C address to 0x68
                             (Connect to 3.3V for address 0x69)
INT        →  (not used)     Interrupt pin (optional)
```

**Pull-up Resistor Wiring:**
```
     3.3V                    3.3V
      |                       |
    [2.2kΩ]                 [2.2kΩ]
      |                       |
   GPIO21 (SDA) ───────────> MPU6050 SDA
   GPIO22 (SCL) ───────────> MPU6050 SCL
```

**Important Notes:**
- **Pull-up resistors are REQUIRED** - I2C will not work without them
- Some GY-521 boards have onboard pull-ups (check board, may not need external)
- AD0 pin controls I2C address: GND=0x68, 3.3V=0x69
- MPU6050 chip accepts 2.375V-3.46V only (GY-521 board has regulator for 5V)

---

## Software Dependencies

Add to `Cargo.toml`:
```toml
[dependencies]
esp-hal = { version = "0.17", features = ["esp32s3"] }
esp-backtrace = { version = "0.11", features = ["esp32s3", "panic-handler", "println"] }
esp-println = { version = "0.9", features = ["esp32s3"] }
mpu6050 = "0.1"
```

**Platform setup:** See `docs/platforms/esp32-s3.md` for toolchain installation.

---

## Build and Flash
```bash
# Navigate to example directory
cd esp32/sensors/examples/

# Build release version
cargo build --release

# Flash and monitor serial output
espflash flash --monitor target/xtensa-esp32s3-none-elf/release/mpu6050-basic

# Or use cargo-espflash
cargo espflash flash --release --monitor
```

**Note:** This is a standalone example file, not a full cargo project.  
To build, you'll need to create a project structure or use the I2C scanner as template.

---

## Expected Output

When working correctly:
```
Initializing MPU6050...
✓ MPU6050 initialized at address 0x68

Reading sensor data...

Accel: X=  0.02, Y=  0.01, Z=  9.81 m/s²
Gyro:  X=  0.00, Y= -0.01, Z=  0.00 deg/s
Temp:  23.5°C

Accel: X= -0.01, Y=  0.00, Z=  9.80 m/s²
Gyro:  X=  0.50, Y=  0.00, Z= -0.20 deg/s
Temp:  23.6°C
```

**Success indicators:**
- Z-axis acceleration should be ~9.8 m/s² when sensor is flat (gravity)
- X and Y acceleration should be near 0 when flat
- Gyroscope values should be near 0 when sensor is still
- Temperature should be reasonable (20-30°C in normal conditions)

**What the values mean:**
- **Accel**: Linear acceleration in m/s² (includes gravity)
  - Flat on table: Z ≈ 9.8, X ≈ 0, Y ≈ 0
  - Tilted 90° on side: X or Y ≈ 9.8, Z ≈ 0
- **Gyro**: Rotational velocity in degrees/second
  - Still: All values near 0
  - Rotating: Shows rotation rate
- **Temp**: Die temperature (slightly higher than ambient)

---

## Troubleshooting

### "Failed to initialize MPU6050"

**Possible causes:**
1. Incorrect wiring (SDA/SCL swapped)
2. Missing pull-up resistors
3. Wrong I2C address
4. Bad power connection

**Debug steps:**
```bash
# Step 1: Run I2C scanner
cd esp32/utilities/i2c-scanner
cargo espflash flash --release --monitor

# Should show: "Device found at address 0x68"
# If nothing found → wiring or pull-up issue
# If shows 0x69 → AD0 pin is high (change in code)
```

**Step 2: Check wiring with multimeter**
- SDA (GPIO21) should measure ~3.3V when idle (pull-up working)
- SCL (GPIO22) should measure ~3.3V when idle
- MPU6050 VCC should be 3.3V (or 5V if using GY-521 board)
- Verify ground connection

**Step 3: Verify pull-up resistors**
- Measure resistance: SDA→3.3V should be 2-5kΩ
- Measure resistance: SCL→3.3V should be 2-5kΩ
- If no pull-ups: I2C cannot work

**Step 4: Check I2C address**
```rust
// In code, try alternate address:
const MPU_ADDR: u8 = 0x69;  // Instead of 0x68
```

---

### "Garbage values" or "Values don't change"

**Check sensor orientation:**
- Flat on table → Z should be ~9.8
- On edge → X or Y should be ~9.8
- Rotate sensor → Gyro values should change

**Verify sensor is working:**
```bash
# Gently shake sensor - all values should change
# If values never change → sensor may be damaged
```

**Check I2C speed:**
```rust
// Try slower I2C speed (in initialization):
let i2c = I2C::new(peripherals.I2C0, sda, scl, 100u32.kHz(), &clocks);
// Some sensors need 100kHz instead of 400kHz
```

---

### "Build error: mpu6050 crate not found"

**Solution:**
```bash
# Make sure mpu6050 is in Cargo.toml dependencies
cargo clean
cargo build --release
```

---

### "Values drift over time"

**This is normal** - MEMS sensors have inherent drift.

**Solutions:**
- Implement sensor fusion (Kalman or complementary filter)
- Calibrate on startup (zero gyro offsets)
- Use external reference (magnetometer, GPS)
- See `common/ALGORITHMS.md` for filtering crates

---

## Code Explanation

### Initialization
```rust
// Create I2C peripheral
let mut i2c = I2C::new(
    peripherals.I2C0,
    sda,               // GPIO21
    scl,               // GPIO22
    100u32.kHz(),      // I2C speed (100kHz or 400kHz)
    &clocks,
);

// Initialize MPU6050 with default settings
let mut mpu = Mpu6050::new(i2c);
mpu.init(&mut delay)?;  // Wake up sensor, set defaults
```

### Reading Sensor Data
```rust
// Read accelerometer (returns m/s²)
let acc = mpu.get_acc()?;
println!("Accel: X={}, Y={}, Z={}", acc.x, acc.y, acc.z);

// Read gyroscope (returns degrees/second)
let gyro = mpu.get_gyro()?;
println!("Gyro: X={}, Y={}, Z={}", gyro.x, gyro.y, gyro.z);

// Read temperature (returns °C)
let temp = mpu.get_temp()?;
println!("Temp: {}°C", temp);
```

### Update Rate
Currently 500ms (2 Hz). Adjust for your needs:
```rust
delay.delay_ms(500u32);  // 2 Hz
delay.delay_ms(100u32);  // 10 Hz (typical for robotics)
delay.delay_ms(50u32);   // 20 Hz
delay.delay_ms(10u32);   // 100 Hz (aggressive, max for I2C)
```

---

## Next Steps

After getting basic readings working:

- [ ] **Calibrate sensor** - Zero gyro offsets, measure accel bias
- [ ] **Implement filtering** - Complementary or Kalman filter for orientation
- [ ] **Send via WiFi** - Use UDP telemetry sender to send IMU data
- [ ] **Integrate with motors** - Use tilt to control balance robot
- [ ] **Add magnetometer** - HMC5883L or QMC5883L for absolute heading
- [ ] **ROS2 integration** - Publish sensor_msgs/Imu topic

**Related examples:**
- UDP telemetry: `esp32/communication/wifi/udp-telemetry-sender/`
- Sensor fusion: See `common/ALGORITHMS.md` (adskalman, ukf)

---

## Advanced: Sensor Fusion

**Why you need it:**
- Raw gyro drifts over time
- Raw accel is noisy and affected by motion
- Fusion combines best of both sensors

**Complementary Filter (simple):**
```rust
// Pseudo-code
angle = 0.98 * (angle + gyro * dt) + 0.02 * accel_angle;
```

**Kalman Filter (better):**
- See `common/ALGORITHMS.md`
- Use `adskalman` crate for state estimation

---

## MPU6050 Specifications

- **Accelerometer Range:** ±2g, ±4g, ±8g, ±16g (default ±2g)
- **Gyroscope Range:** ±250°/s, ±500°/s, ±1000°/s, ±2000°/s (default ±250°/s)
- **Temperature Range:** -40°C to +85°C
- **I2C Speed:** Up to 400 kHz
- **Update Rate:** Up to 1 kHz
- **Power:** 3.3mA typical

---

## Reference Documents

- MPU6050 Datasheet: https://invensense.tdk.com/wp-content/uploads/2015/02/MPU-6000-Datasheet1.pdf
- Register Map: https://invensense.tdk.com/wp-content/uploads/2015/02/MPU-6000-Register-Map1.pdf
- Platform setup: `docs/platforms/esp32-s3.md`
- I2C debugging: `esp32/utilities/i2c-scanner/`
- Telemetry schema: `common/protocols/telemetry_v1.json`

---

## Changelog

- 2026-02-13: Retrofitted with Example Contract template
- Original: Basic MPU6050 reading example
- Added comprehensive wiring, troubleshooting, and explanations
