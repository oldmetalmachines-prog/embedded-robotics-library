# [Example Name] - [One Line Description]

## Purpose

[2-3 sentences explaining what this example demonstrates and why it's useful]

Example:
> This example demonstrates reading accelerometer and gyroscope data from an MPU6050 IMU sensor using I2C communication. It's a foundation for robotics projects that need orientation and motion sensing.

---

## Hardware Required

**Essential:**
- [Main board - e.g., ESP32-S3 DevKit]
- [Sensor/component - e.g., MPU6050 IMU]
- Breadboard
- Jumper wires (male-to-male)

**Optional:**
- [Any optional components]
- USB cable for programming

**Where to buy:**
- [Main component]: ~$[price] - [link or search term]

---

## Wiring Diagram
```
[Component]    [Board]         Notes
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
VCC        →   3.3V           DO NOT use 5V!
GND        →   GND            Common ground
SDA        →   GPIO21         I2C data (with pull-up)
SCL        →   GPIO22         I2C clock (with pull-up)
```

**Important Notes:**
- [Any critical wiring notes - e.g., "I2C requires 2.2kΩ pull-up resistors to 3.3V"]
- [Pin alternatives - e.g., "For ESP32-P4 use GPIO8/GPIO9"]

---

## Software Dependencies

Add to your `Cargo.toml`:
```toml
[dependencies]
esp-hal = "0.17"
[other-crate] = "x.y"
```

**Platform-specific notes:**
- See `docs/platforms/[your-platform].md` for toolchain setup

---

## Build and Flash
```bash
# Build
cargo build --release

# Flash and monitor
espflash flash --monitor target/[target-triple]/release/[example-name]

# Or use cargo-espflash
cargo espflash flash --release --monitor
```

---

## Expected Output

When working correctly, you should see:
```
[Paste actual serial output here]

Example:
Initializing MPU6050...
✓ MPU6050 initialized at address 0x68
Reading sensor data...
Accel: X=0.02, Y=0.01, Z=9.81 m/s²
Gyro:  X=0.00, Y=-0.01, Z=0.00 deg/s
Temp:  23.5°C
```

**Success indicators:**
- [What tells you it's working - e.g., "Z-axis accel should be ~9.8 when flat"]
- [Data ranges - e.g., "Gyro should be near 0 when still"]

---

## Troubleshooting

### "Error: Failed to initialize [component]"

**Possible causes:**
- Incorrect wiring (check SDA/SCL not swapped)
- Missing pull-up resistors on I2C lines
- Wrong I2C address (try 0x68 or 0x69)

**How to debug:**
```bash
# On Raspberry Pi, scan I2C bus:
i2cdetect -y 1

# Should show device at 0x68 or 0x69
```

---

### "Build error: target not found"

**Solution:**
Install the correct target:
```bash
rustup target add [target-triple]
# Example: rustup target add riscv32imc-unknown-none-elf
```

---

### "Device not responding"

**Checklist:**
1. Check power - is LED lit on component?
2. Verify ground connection (use multimeter)
3. Check 3.3V rail (NOT 5V!)
4. Reseat all connections
5. Try different GPIO pins

---

## Code Explanation

**Key sections:**

### Initialization
```rust
// [Brief explanation of setup code]
```

### Main Loop
```rust
// [Brief explanation of loop logic]
```

### Error Handling
```rust
// [Brief explanation of how errors are handled]
```

---

## Next Steps

After getting this working, try:
- [ ] [Enhancement 1 - e.g., "Add temperature compensation"]
- [ ] [Enhancement 2 - e.g., "Implement complementary filter"]
- [ ] [Integration idea - e.g., "Send data via WiFi to ROS2"]

---

## Reference Documents

- Component datasheet: [link]
- Related examples: `[path to similar examples]`
- Platform setup: `docs/platforms/[platform].md`
- Telemetry format: `common/protocols/telemetry_v1.json`

---

## Changelog

- YYYY-MM-DD: Initial version
- YYYY-MM-DD: [Any updates made]

---

## License

MIT (same as parent repository)
