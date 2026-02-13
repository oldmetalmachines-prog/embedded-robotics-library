# Telemetry Protocol Documentation

## Overview

This directory contains the canonical message formats for device communication.

**Current Version:** v1.0.0  
**Format:** JSON (human-readable) and binary (embedded)  
**Transport:** UDP (primary), TCP (fallback), ROS2 topics

---

## Using in Rust (ESP32)

### Dependencies
```toml
[dependencies]
serde = { version = "1.0", default-features = false, features = ["derive"] }
serde-json-core = "0.5"  # For no_std
heapless = "0.8"
```

### Struct Definition
```rust
use serde::{Deserialize, Serialize};
use heapless::String;

#[derive(Serialize, Deserialize, Debug)]
pub struct TelemetryHeader {
    pub device_id: String<32>,      // Max 32 chars
    pub timestamp: u64,              // Unix timestamp (ms)
    pub sequence: u32,               // Message counter
    pub message_type: MessageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Telemetry,
    Command,
    Status,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImuData {
    pub accel_x: f32,
    pub accel_y: f32,
    pub accel_z: f32,
    pub gyro_x: f32,
    pub gyro_y: f32,
    pub gyro_z: f32,
    pub temperature_c: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelemetryMessage {
    pub header: TelemetryHeader,
    pub imu: Option<ImuData>,
    // Add other fields as needed
}
```

### Sending Telemetry (ESP32)
```rust
use esp_wifi::wifi::WifiDevice;

let mut seq = 0u32;

loop {
    // Read sensor
    let accel = imu.get_acc()?;
    let gyro = imu.get_gyro()?;
    
    // Create message
    let msg = TelemetryMessage {
        header: TelemetryHeader {
            device_id: String::from("esp32-rover-01"),
            timestamp: get_timestamp_ms(),
            sequence: seq,
            message_type: MessageType::Telemetry,
        },
        imu: Some(ImuData {
            accel_x: accel.x,
            accel_y: accel.y,
            accel_z: accel.z,
            gyro_x: gyro.x,
            gyro_y: gyro.y,
            gyro_z: gyro.z,
            temperature_c: 23.5,
        }),
    };
    
    // Serialize to JSON
    let mut buffer = [0u8; 512];
    let json = serde_json_core::to_slice(&msg, &mut buffer)?;
    
    // Send via UDP
    socket.send_to(json, remote_addr)?;
    
    seq = seq.wrapping_add(1);
    delay.delay_ms(100);
}
```

---

## Using in Python (Raspberry Pi)

### Receiving Telemetry
```python
import socket
import json
from datetime import datetime

# Create UDP socket
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind(('0.0.0.0', 8888))

print("Listening for telemetry on port 8888...")

while True:
    data, addr = sock.recvfrom(1024)
    
    try:
        msg = json.loads(data)
        
        # Extract data
        device_id = msg['header']['device_id']
        timestamp = msg['header']['timestamp']
        
        if 'imu' in msg:
            imu = msg['imu']
            print(f"[{device_id}] Accel: ({imu['accel_x']:.2f}, "
                  f"{imu['accel_y']:.2f}, {imu['accel_z']:.2f})")
        
    except json.JSONDecodeError:
        print(f"Invalid JSON from {addr}")
```

---

## Message Size Guidelines

**UDP (recommended for telemetry):**
- Max safe size: 512 bytes
- Typical message: 100-200 bytes
- Advantage: Low latency, no connection overhead

**TCP (for commands/config):**
- Max message: 4096 bytes
- Advantage: Guaranteed delivery
- Use for: Commands, large config updates

---

## Versioning

When making changes:

1. **Minor changes** (add optional fields):
   - Increment patch version (1.0.0 → 1.0.1)
   - Maintain backward compatibility

2. **Breaking changes** (rename/remove fields):
   - Create `telemetry_v2.json`
   - Update all devices together
   - Deprecate v1 after migration

---

## Best Practices

✅ **DO:**
- Include only needed fields (reduce message size)
- Use consistent units (SI units preferred)
- Add timestamp to every message
- Increment sequence number

❌ **DON'T:**
- Send empty/null fields (omit them instead)
- Use ambiguous units (specify in field name)
- Exceed 512 bytes for UDP
- Forget error handling on parse

---

## Testing

### Validate JSON
```bash
# Install jsonlint
sudo apt install jsonlint

# Validate schema
jsonlint common/protocols/telemetry_v1.json
```

### Test UDP Locally

**Sender (ESP32 simulator on laptop):**
```bash
echo '{"header":{"device_id":"test","timestamp":1707782400000,"sequence":1,"message_type":"telemetry"},"imu":{"accel_x":0.0,"accel_y":0.0,"accel_z":9.81}}' | nc -u localhost 8888
```

**Receiver:**
```bash
nc -ul 8888
```

---

## Migration Path

**Phase 1:** ESP32 → UDP → Pi (print to terminal)  
**Phase 2:** Pi → ROS2 topics (bridge)  
**Phase 3:** Multiple ESP32s → aggregate on Pi  
**Phase 4:** Add Jetson for vision data
