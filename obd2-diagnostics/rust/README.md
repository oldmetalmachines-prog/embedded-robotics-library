# OBDium - OBD-II Diagnostics (Rust)

Source: https://github.com/provrb/obdium
Author: provrb
License: MIT
Language: Rust

## Purpose
Full OBD-II diagnostic tool in Rust. Live vehicle sensor data, DTC fault codes,
VIN decoding. Works with ELM327 adapters. Offline capable.

## Key Features
- Live PIDs: engine temp, RPM, speed, fuel, O2 sensors, etc
- Read and clear DTC fault codes (Powertrain, Body, Chassis, Network)
- Offline VIN decoding
- ELM327 serial adapter support

## Hardware
ELM327 USB adapter connected to vehicle OBD-II port.
Runs on any Linux machine including Raspberry Pi.

## Build
cargo build --release
