# Third-Party Notices

This repository includes or references third-party projects. Each project retains its own license.

If a third-party project is included as a vendored copy (present in this repo as a directory), its LICENSE file
must remain intact in that directory. When possible, prefer using upstream links or git submodules instead of
vendoring full repositories.

## Vendored directories (currently in this repo)

- awesome-embedded-rust — https://github.com/rust-embedded/awesome-embedded-rust
- awesome-esp-rust — https://github.com/esp-rs/awesome-esp-rust
- rppal — https://github.com/golemparts/rppal

## Notes

If you remove a vendored directory in favor of a submodule or external link, update this file accordingly.

## Industrial Protocol Crates (referenced/adapted code)

- ethercrab — https://github.com/ethercrab-rs/ethercrab — Author: James Waples — License: MIT/Apache-2.0
- etherage — https://github.com/jimy-byerley/etherage — License: LGPL-3.0
- tokio-modbus — https://github.com/slowtec/tokio-modbus — License: MIT/Apache-2.0
- socketcan — https://github.com/socketcan-rs/socketcan-rs — License: MIT/Apache-2.0
- embedded-can — https://github.com/rust-embedded/embedded-hal — License: MIT/Apache-2.0

## Hobby Motor Driver Crates (referenced/adapted code)

- uln2003 — https://github.com/Obirvalger/uln2003 — License: MIT
- stepper — https://github.com/flott-motion/stepper — License: MIT/Apache-2.0
- tb6612fng — https://github.com/jacobrosenthal/tb6612fng-rs — License: MIT
- tmc2209_pi — https://github.com/andrewseidl/tmc2209_pi — License: MIT
- esp-drv8833 — https://github.com/nguterresn/esp-drv8833 — License: MIT

## J1939 / Automotive CAN (referenced/adapted code)

- Open-SAE-J1939 — https://github.com/DanielMartensson/Open-SAE-J1939 — Author: Daniel Mårtensson — License: MIT
- J1939-Framework — https://github.com/famez/J1939-Framework — Author: famez — License: GPL-3.0
- python-can-j1939 — https://github.com/juergenH87/python-can-j1939 — Author: juergenH87 — License: MIT

## Yaskawa / Industrial Servo (referenced/adapted code)

- IGH-EtherCAT-motor-control-sample — https://github.com/CalvinHsu1223/IGH-EtherCAT-motor-control-sample — Author: CalvinHsu1223 — License: See repository

---

## OBD9141
- **Location:** obd2-diagnostics/cpp/obd9141/
- **Source:** https://github.com/iwanders/OBD9141
- **Author:** Ivor Wanders
- **License:** MIT
- **Purpose:** ISO 9141-2 and ISO 14230 (KWP2000) K-Line OBD-II diagnostics for pre-CAN vehicles. Supports Arduino, ESP32, Teensy.
