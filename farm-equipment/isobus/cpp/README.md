# AgIsoStack++ ISOBUS Library (C++)

Source: https://github.com/Open-Agriculture/AgIsoStack-plus-plus
License: MIT
Language: C++

## Purpose
ISOBUS (ISO 11783) implementation for modern farm equipment.
Used by John Deere, Case, New Holland, Fendt, AGCO and all major manufacturers.
Built on top of J1939/CAN bus.

## Key Examples
- `examples/virtual_terminal/` — ISOBUS Virtual Terminal (display on tractor cab)
- `examples/task_controller_client/` — Task controller for precision agriculture
- `examples/task_controller_server/` — Task controller server
- `examples/diagnostic_protocol/` — Equipment diagnostics
- `examples/guidance/` — GPS/guidance system integration
- `examples/nmea2000/` — NMEA 2000 (GPS/navigation data)
- `examples/seeder_example/` — Seeder/planter implement control
- `examples/pgn_requests/` — Parameter Group Number requests
- `examples/transport_layer/` — Transport layer examples

## Hardware Target
Raspberry Pi 5 or any Linux node with CAN interface.
Also supports Arduino and ESP32 via hardware_integration layer.

## Key Use Cases for Your Shop
- Diagnosing implement communication issues on tractors
- Building custom ISOBUS implements
- Precision agriculture GPS guidance systems
- Seeder/planter/sprayer control systems
