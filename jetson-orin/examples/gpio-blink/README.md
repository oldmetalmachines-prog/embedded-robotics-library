# Jetson GPIO LED Blink

## Purpose

Simple LED blink using Jetson GPIO. Proves GPIO access works and demonstrates sysfs_gpio usage on Jetson Orin Nano.

Use cases:
- First Rust program on Jetson
- Verify GPIO permissions
- Test external LED wiring
- Foundation for GPIO control

---

## Hardware Required

**Essential:**
- Jetson Orin Nano Super Developer Kit
- LED (any color)
- 220Ω-1kΩ resistor
- Breadboard
- Jumper wires (female-to-male)

---

## Wiring Diagram
```
Jetson Pin 12 (GPIO12) ──┬──[220Ω]──[LED+]──[LED-]──┬── GND (Pin 6)
                         │                          │
                    GPIO 12                        GND

LED orientation:
  Longer leg (anode)  = +  (to resistor)
  Shorter leg (cathode) = - (to GND)
```

**Connection:**
- GPIO12 (Pin 12) → 220Ω resistor → LED positive (long leg)
- LED negative (short leg) → GND (Pin 6, 9, 14, or 20)

---

## Software Dependencies
```toml
[dependencies]
gpio-cdev = "0.6"
anyhow = "1.0"
```

**Platform:** See `docs/platforms/jetson-orin-nano-super.md`

---

## Build and Run
```bash
# Build
cargo build --release

# Run (needs GPIO permissions)
cargo run --release

# Or run binary directly
./target/release/jetson-gpio-blink
```

---

## Expected Output
```
Jetson GPIO Blink Example
Blinking GPIO 12 (Pin 12)
Connect LED with resistor between Pin 12 and GND
Press Ctrl+C to stop

LED ON
LED OFF
LED ON
LED OFF
...
```

**LED should blink at 1 Hz (500ms on, 500ms off).**

---

## Troubleshooting

### "Permission denied" on /dev/gpiochip0

**Solution:**
```bash
# Add user to gpio group
sudo groupadd gpio
sudo usermod -a -G gpio $USER
sudo chgrp gpio /dev/gpiochip*
sudo chmod g+rw /dev/gpiochip*

# Logout and login
```

**Permanent fix (survives reboot):**
```bash
# Create udev rule
sudo nano /etc/udev/rules.d/99-gpio.rules

# Add:
SUBSYSTEM=="gpio", KERNEL=="gpiochip*", GROUP="gpio", MODE="0660"

# Reload udev
sudo udevadm control --reload-rules
sudo udevadm trigger
```

---

### LED not blinking

**Check:**
1. Correct pin - GPIO12 is physical pin 12
2. LED polarity - long leg to resistor
3. Resistor value - 220Ω to 1kΩ
4. GND connection solid
5. Jetson powered on and booted

---

### Build error: "gpio-cdev not found"
```bash
cargo clean
cargo update
cargo build --release
```

---

## Code Explanation
```rust
// Open GPIO chip (Jetson has /dev/gpiochip0)
let mut chip = Chip::new("/dev/gpiochip0")?;

// Get line 12 (maps to header pin 12)
let line = chip.get_line(12)?;

// Request as output
let handle = line.request(LineRequestFlags::OUTPUT, 0, "gpio-blink")?;

// Set high/low
handle.set_value(1)?;  // LED on
handle.set_value(0)?;  // LED off
```

---

## Next Steps

- [ ] Add button input (read GPIO)
- [ ] Control multiple LEDs
- [ ] PWM for LED brightness
- [ ] Integrate with ROS2
- [ ] Camera trigger via GPIO

---

## Reference Documents

- Platform guide: `docs/platforms/jetson-orin-nano-super.md`
- gpio-cdev docs: https://docs.rs/gpio-cdev/

---

## Changelog

- 2026-02-13: Initial version
- Basic GPIO output using gpio-cdev
- Follows Example Contract template
