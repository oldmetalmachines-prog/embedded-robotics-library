# EtherCAT Examples

Source: https://github.com/ethercrab-rs/ethercrab
Author: James Waples
License: MIT/Apache-2.0

## Files

- `ek1100.rs` — Basic EK1100 coupler example, best starting point
- `discover.rs` — Discover all SubDevices on the network
- `dc.rs` — Distributed clocks (sync timing across slaves)
- `multiple-groups.rs` — Multiple device groups
- `sdo-info.rs` — Read SDO object dictionary from drives
- `dump-eeprom.rs` — Dump drive EEPROM data
- `performance.rs` — Performance/latency testing
- `io-uring.rs` — Linux io_uring based TX/RX (best for Pi5)
- `release.rs` — Safe release of SubDevices
- `akd.rs.disabled` — Kollmorgen AKD servo drive example
- `ec400.rs.disabled` — EC400 example
- `c5-e.rs.disabled` — C5-E example

## Hardware Target
Raspberry Pi 5 with dedicated Ethernet port for EtherCAT network.
Start with `discover.rs` to verify your drives are detected, then `ek1100.rs`.

## Note
The `.disabled` examples are for specific drive hardware — rename to `.rs` when working with that hardware.
