# UDP Telemetry Sender - ESP32

## Purpose

Sends sensor telemetry data over WiFi using UDP to a receiver (Raspberry Pi or laptop). Implements the Telemetry v1 schema defined in `common/protocols/telemetry_v1.json`.

This is the foundation for:
- Wireless robot telemetry
- ROS2 integration (via UDP bridge)
- Multi-device data aggregation
- Real-time monitoring and logging

---

## Hardware Required

**Essential:**
- ESP32-S3 DevKit (or ESP32-C3)
- WiFi network (2.4 GHz, WPA2)
- USB cable for programming

**Optional (for real sensor data):**
- MPU6050 IMU on I2C
- See `esp32/sensors/examples/mpu6050-basic.rs`

---

## Wiring Diagram

**No external wiring required for basic test** (uses simulated sensor data).

To add real IMU sensor later:
```
MPU6050    ESP32-S3
VCC    →   3.3V
GND    →   GND
SDA    →   GPIO21 (+ 2.2kΩ pull-up)
SCL    →   GPIO22 (+ 2.2kΩ pull-up)
```

---

## Software Dependencies
```toml
[dependencies]
esp-wifi = { version = "0.5", features = ["esp32s3", "wifi"] }
serde = { version = "1.0", default-features = false }
serde-json-core = "0.5"
heapless = "0.8"
smoltcp = "0.11"
```

---

## Configuration

**CRITICAL: Edit `src/main.rs` before building!**
```rust
// Line 30-31: Your WiFi credentials
const SSID: &str = "YOUR_WIFI_SSID";
const PASSWORD: &str = "YOUR_WIFI_PASSWORD";

// Line 34: Target receiver IP (your Raspberry Pi)
const TARGET_IP: [u8; 4] = [192, 168, 1, 100];
const TARGET_PORT: u16 = 8888;
```

**Find your Pi's IP:**
```bash
# On Raspberry Pi:
hostname -I
```

---

## Build and Flash
```bash
# Edit WiFi credentials first!
nano src/main.rs

# Build and flash
cargo espflash flash --release --monitor
```

---

## Expected Output

### On ESP32 Serial Monitor:
```
=== UDP Telemetry Sender ===
Connecting to WiFi: MyNetwork
WiFi set configuration returned Ok(())
WiFi started
WiFi connected!
Waiting for IP...
Got IP: 192.168.1.150
Sending telemetry to 192.168.1.100:8888
Sent seq 0 (156 bytes)
Sent seq 1 (156 bytes)
Sent seq 2 (156 bytes)
...
```

### On Receiver (Raspberry Pi):

See companion project: `raspberry-pi/utilities/udp-telemetry-receiver/`

Or test with netcat:
```bash
# On Raspberry Pi or laptop:
nc -ul 8888

# Should show JSON messages like:
{"header":{"device_id":"esp32-test-01","timestamp":1234567890,"sequence":0,"message_type":"telemetry"},"telemetry":{"imu":{"accel_x":0.02,"accel_y":0.01,"accel_z":9.81,"gyro_x":0.0,"gyro_y":-0.5,"gyro_z":0.1,"temperature_c":23.5}}}
```

---

## Troubleshooting

### "Failed to connect: Error"

**Check WiFi settings:**
- SSID spelled correctly (case-sensitive)
- Password correct
- 2.4 GHz network (ESP32 doesn't support 5 GHz)
- WPA2 (not WPA3)
- SSID is visible (not hidden)

---

### "No packets received on Pi"

**Debug steps:**
```bash
# 1. Check firewall
sudo ufw status
sudo ufw allow 8888/udp

# 2. Verify IP address
hostname -I  # Use this in ESP32 code

# 3. Test UDP listener
nc -ul 8888  # Should receive packets

# 4. Check both devices on same network
ping [ESP32_IP]
```

---

### "Build error: feature not enabled"

**Solution:**
```bash
# Make sure esp-wifi features match your chip
# For ESP32-C3:
esp-wifi = { version = "0.5", features = ["esp32c3", "wifi"] }
```

---

## Message Format

Follows `common/protocols/telemetry_v1.json`:
```json
{
  "header": {
    "device_id": "esp32-test-01",
    "timestamp": 1707782400000,
    "sequence": 0,
    "message_type": "telemetry"
  },
  "telemetry": {
    "imu": {
      "accel_x": 0.02,
      "accel_y": 0.01,
      "accel_z": 9.81,
      "gyro_x": 0.0,
      "gyro_y": -0.5,
      "gyro_z": 0.1,
      "temperature_c": 23.5
    }
  }
}
```

**Message size:** ~150-200 bytes (well under 512 byte UDP limit)

---

## Next Steps

1. **Test basic sending** - Run this code, verify packets arrive
2. **Add real sensor** - Replace simulated data with MPU6050
3. **Set up receiver** - `raspberry-pi/utilities/udp-telemetry-receiver/`
4. **ROS2 bridge** - Convert UDP → ROS2 topics

---

## Code Explanation

### WiFi Connection
```rust
let client_config = Configuration::Client(ClientConfiguration {
    ssid: SSID.try_into().unwrap(),
    password: PASSWORD.try_into().unwrap(),
    ..Default::default()
});
controller.set_configuration(&client_config);
controller.start();
controller.connect();  // Blocks until connected
```

### UDP Sending
```rust
// Serialize telemetry to JSON
let json_data = serde_json_core::to_slice(&msg, &mut buffer)?;

// Send UDP packet
socket.send_slice(json_data, remote_endpoint)?;
```

### Telemetry Rate
Currently 10 Hz (100ms delay). Adjust in main loop:
```rust
delay.delay_ms(100u32);  // 10 Hz
delay.delay_ms(50u32);   // 20 Hz
delay.delay_ms(20u32);   // 50 Hz
```

---

## Performance

- **Send rate:** 10 Hz (configurable)
- **Latency:** ~5-20ms on local network
- **Packet loss:** <1% on good WiFi
- **Power:** ~150mA during WiFi transmission

---

## Reference Documents

- Telemetry schema: `common/protocols/telemetry_v1.json`
- Platform setup: `docs/platforms/esp32-s3.md`
- Companion receiver: `raspberry-pi/utilities/udp-telemetry-receiver/`

---

## Changelog

- 2026-02-13: Initial version
- Implements telemetry_v1 schema
- WiFi + UDP sending at 10 Hz
- Simulated IMU data (replace with real sensor)
