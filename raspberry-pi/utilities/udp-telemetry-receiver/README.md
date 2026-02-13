# UDP Telemetry Receiver - Raspberry Pi

## Purpose

Receives telemetry data sent over WiFi from ESP32 devices using UDP. Parses and displays sensor data according to the Telemetry v1 schema.

This is the companion to `esp32/communication/wifi/udp-telemetry-sender/`.

Use cases:
- Monitor robot telemetry in real-time
- Log sensor data for analysis
- Foundation for ROS2 bridge
- Debug wireless communication

---

## Hardware Required

**Essential:**
- Raspberry Pi 5 (or Pi Zero 2W, or any Linux machine)
- Network connection (WiFi or Ethernet)
- Same network as ESP32 sender

**No external wiring required** - runs purely in software.

---

## Software Dependencies
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
colored = "2.1"
```

**Platform:** Any Linux system with Rust installed.

---

## Build and Run
```bash
# Build release version
cargo build --release

# Run
cargo run --release

# Or run the binary directly
./target/release/udp-telemetry-receiver
```

---

## Expected Output

### When Receiving Data:
```
=== UDP Telemetry Receiver ===
Listening on 0.0.0.0:8888 for telemetry packets

✓ UDP socket bound successfully
Waiting for telemetry data...

[14:23:45.123] esp32-test-01 telemetry seq=0 from 192.168.1.150:54321
  📊 Accel: (  0.02,   0.01,   9.81) m/s²
  🔄 Gyro:  (  0.00,  -0.50,   0.10) deg/s
  🌡️  Temp:  23.5°C

[14:23:45.223] esp32-test-01 telemetry seq=1 from 192.168.1.150:54321
  📊 Accel: (  0.02,   0.01,   9.81) m/s²
  🔄 Gyro:  (  0.00,  -0.50,   0.10) deg/s
  🌡️  Temp:  23.5°C

📈 Received: 100, Errors: 0, Success: 100.0%
```

### When Packet Loss Detected:
```
[14:23:45.523] esp32-test-01 telemetry seq=5 from 192.168.1.150:54321
⚠️  Lost 2 packets
  📊 Accel: (  0.02,   0.01,   9.81) m/s²
```

### On Parse Error:
```
❌ JSON parse error: expected value at line 1 column 1 (size: 45 bytes)
   Raw: {incomplete json data...
```

---

## Troubleshooting

### "No packets received"

**Check ESP32 sender:**
```bash
# Verify ESP32 is sending
# Check serial monitor on ESP32 for "Sent seq X"
```

**Check network:**
```bash
# Verify both devices on same network
ping [ESP32_IP]

# Check firewall
sudo ufw status
sudo ufw allow 8888/udp

# Test with netcat
nc -ul 8888
```

**Check IP in ESP32 code:**
```rust
// In udp-telemetry-sender/src/main.rs
// Make sure TARGET_IP matches this Pi's IP
hostname -I  # Get your Pi's IP
```

---

### "Address already in use"

**Another program using port 8888:**
```bash
# Find what's using the port
sudo lsof -i :8888

# Kill it if needed
sudo kill [PID]

# Or use different port
# Edit both sender and receiver code
```

---

### "Permission denied"

**Ports <1024 need sudo (but 8888 doesn't):**
```bash
# If you changed to port 80:
sudo ./target/release/udp-telemetry-receiver
```

---

### "Parse errors on valid JSON"

**Check sender message size:**
```bash
# On ESP32, verify messages are <512 bytes
# Large messages may get fragmented
```

---

## Message Format

Expects JSON following `common/protocols/telemetry_v1.json`:
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

---

## Features

✅ **Packet loss detection** - Tracks sequence numbers  
✅ **Colored output** - Easy to read in terminal  
✅ **Statistics** - Success rate, error count  
✅ **Timestamp display** - Local time for each message  
✅ **Error handling** - Shows malformed packets for debugging  

---

## Next Steps

After confirming data reception:

1. **Add logging to file:**
```rust
   // Add to Cargo.toml: csv = "1.3"
   // Write telemetry to CSV for analysis
```

2. **ROS2 bridge:**
   - See `raspberry-pi/ros2-integration/examples/`
   - Convert UDP → ROS2 topics

3. **Multiple devices:**
   - Track device_id
   - Aggregate data from multiple ESP32s

4. **Data visualization:**
   - Plot IMU data with matplotlib
   - Real-time graphs

---

## Code Explanation

### UDP Socket Binding
```rust
// Bind to all interfaces on port 8888
let socket = UdpSocket::bind("0.0.0.0:8888")?;

// Receive packet (blocking)
let (size, src) = socket.recv_from(&mut buffer)?;
```

### JSON Parsing
```rust
// Deserialize using serde
let msg: TelemetryMessage = serde_json::from_slice(&buffer[..size])?;
```

### Packet Loss Detection
```rust
// Check if sequence jumped
if msg.sequence != expected_seq {
    let lost = msg.sequence - expected_seq;
    println!("Lost {} packets", lost);
}
```

---

## Performance

- **CPU Usage:** <1% on Pi 5
- **Memory:** ~2MB
- **Latency:** <5ms processing time
- **Max Rate:** >1000 packets/second

---

## Testing Without ESP32

**Send test packet from command line:**
```bash
echo '{"header":{"device_id":"test","timestamp":1707782400000,"sequence":0,"message_type":"telemetry"},"telemetry":{"imu":{"accel_x":0.0,"accel_y":0.0,"accel_z":9.81,"gyro_x":0.0,"gyro_y":0.0,"gyro_z":0.0,"temperature_c":23.5}}}' | nc -u localhost 8888
```

**Should see output in receiver.**

---

## Reference Documents

- Telemetry schema: `common/protocols/telemetry_v1.json`
- Companion sender: `esp32/communication/wifi/udp-telemetry-sender/`
- ROS2 integration: `raspberry-pi/ros2-integration/SETUP.md`

---

## Changelog

- 2026-02-13: Initial version
- Parses telemetry_v1 schema
- Colored terminal output
- Packet loss detection
- Statistics tracking
