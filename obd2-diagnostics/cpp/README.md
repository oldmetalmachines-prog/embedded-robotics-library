# OBD2 K-Line Diagnostics (C++)

ISO 9141-2 and ISO 14230 (KWP2000) K-Line protocol implementation for embedded systems.

## Library: OBD9141

**Source:** https://github.com/iwanders/OBD9141  
**Author:** Ivor Wanders  
**License:** MIT  

### Supported Protocols
- ISO 9141-2 (5-baud init)
- ISO 14230 / KWP2000 (slow and fast init)

### Supported Hardware
- Arduino (via AltSoftSerial)
- ESP32
- Teensy 3.x / LC

### Use Cases
- Pre-CAN vehicles (pre-2008 cars, older farm equipment)
- Reading live PIDs (RPM, temp, speed, O2 sensors)
- Reading/clearing DTCs
- Any vehicle with Pin 7 on the OBD-II connector

### Structure
- `obd9141/src/` - Library source
- `obd9141/examples/` - Arduino/ESP32 examples
