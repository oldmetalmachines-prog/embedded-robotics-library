# Raspberry Pi Servo Controller (Rust)

A simple Rust project to move a hobby servo to a target angle using Raspberry Pi GPIO.

## Credit / Attribution
This project uses the `rppal` crate for Raspberry Pi peripheral access.

- Dependency: `rppal`
- Upstream: https://github.com/golemparts/rppal
- License: See upstream / crate license

This example project is authored/curated in this repository.

## Hardware
- Raspberry Pi with GPIO header
- Hobby servo (SG90/MG90S/MG996R, etc.)
- External 5V supply for servo (recommended)
- Common ground between Pi and servo supply

## Wiring
Typical servo wires:
- Red: +5V (external supply)
- Brown/Black: GND (external supply)
- Orange/Yellow/White: Signal (to Pi GPIO)

**Important:** Connect Pi GND to servo power GND (common ground).

Example:
- Signal: BCM GPIO 18 (physical pin 12)

## Build
```bash
cd raspberry-pi/gpio/servo-controller
cargo build --release

