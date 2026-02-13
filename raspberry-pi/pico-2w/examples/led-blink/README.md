# Pico 2W LED Blink - Hello World

## Purpose

The simplest possible program for Raspberry Pi Pico 2W - blinks the onboard LED. This proves your toolchain is working and teaches the basics of Embassy async runtime.

Use cases:
- First program to run on new Pico 2W
- Verify toolchain installation
- Learn Embassy executor basics
- Test UF2 flashing process

**Special Note:** The Pico 2W LED is controlled via the WiFi chip (CYW43), not a direct GPIO pin. This makes it more complex than a simple GPIO toggle, but teaches you how to work with the WiFi chip.

---

## Hardware Required

**Essential:**
- Raspberry Pi Pico 2W
- USB cable (micro-USB, data-capable)

**No external components needed!** - Uses onboard LED.

**Where to buy:**
- Pico 2W: ~$7 - [Raspberry Pi approved resellers](https://www.raspberrypi.com/products/raspberry-pi-pico-2/)

---

## Wiring Diagram

**No wiring required!**
```
Raspberry Pi Pico 2W
┌────────────────┐
│                │
│   ┌────────┐   │
│   │        │   │
│   │ RP2350 │   │
│   │        │   │
│   └────────┘   │
│                │
│    [LED]  ←────┼─── Onboard LED (controlled via CYW43 WiFi chip)
│                │
└────────────────┘
```

**LED Location:** Small green LED on board near USB connector.

**Important Notes:**
- LED is GPIO25 internally, but controlled through CYW43 WiFi chip
- Cannot use simple GPIO toggle - must use cyw43 library
- This is why setup is more complex than typical LED blink

---

## Software Dependencies
```toml
[dependencies]
cortex-m = "0.7"
cortex-m-rt = "0.7"
embedded-hal = "1.0"
rp2040-hal = "0.9"
cyw43 = "0.1"
cyw43-pio = "0.1"
embassy-executor = "0.5"
embassy-time = "0.3"
embassy-rp = "0.1"
```

**Platform setup:** See `docs/platforms/pi-pico-2w.md`

**Additional Required Files:**
- WiFi firmware blobs (43439A0.bin, 43439A0_clm.bin)
- Download from: https://github.com/embassy-rs/embassy/tree/main/cyw43-firmware

---

## Build and Flash
```bash
# Install target (one-time)
rustup target add thumbv8m.main-none-eabihf

# Install flashing tool (one-time)
cargo install elf2uf2-rs

# Build release version
cargo build --release

# Flash to Pico:
# 1. Hold BOOTSEL button on Pico
# 2. Connect USB while holding button
# 3. Pico appears as USB drive "RPI-RP2"
# 4. Convert and copy:
elf2uf2-rs -d target/thumbv8m.main-none-eabihf/release/pico2w-led-blink

# OR manually copy:
elf2uf2-rs target/thumbv8m.main-none-eabihf/release/pico2w-led-blink pico2w-led-blink.uf2
cp pico2w-led-blink.uf2 /media/$USER/RPI-RP2/

# Pico automatically reboots and LED starts blinking!
```

---

## Expected Output

**Visual:**
- Green onboard LED blinks on/off every 500ms
- Pattern: ON (500ms) → OFF (500ms) → repeat

**Serial output:** None (this example has no serial/debug output)

**Success indicators:**
- LED is blinking steadily
- Blink rate is 1 Hz (once per second)
- LED brightness is consistent

**What you're seeing:**
- Embassy async runtime controlling timing
- CYW43 WiFi chip GPIO controlling LED
- Non-blocking delays using async/await

---

## Troubleshooting

### "Pico won't enter bootloader mode"

**Solution:**
```bash
# Disconnect USB
# Hold BOOTSEL button on Pico
# Connect USB while holding button
# Wait 2 seconds, release button
# Check for drive:
ls /media/$USER/RPI-RP2/
# Should show INFO_UF2.TXT and INDEX.HTM
```

---

### "Build error: target not found"

**Solution:**
```bash
rustup target add thumbv8m.main-none-eabihf
cargo clean
cargo build --release
```

---

### "elf2uf2-rs not found"

**Solution:**
```bash
cargo install elf2uf2-rs
# Add to PATH if needed:
export PATH="$HOME/.cargo/bin:$PATH"
```

---

### "LED not blinking"

**Check:**
1. Did Pico reboot after copying UF2? (drive should disappear)
2. Is correct Pico model? (must be Pico 2W, not regular Pico)
3. Look closely at LED - might be dim
4. Try re-flashing

**If still not working:**
```bash
# Try a simpler GPIO LED on pin GP2:
# Connect external LED with resistor to GP2
# Modify code to use Output::new(p.PIN_2, Level::Low)
```

---

### "Firmware blob files not found"

**Solution:**
```bash
# Download firmware files
mkdir -p cyw43-firmware
cd cyw43-firmware
wget https://github.com/embassy-rs/embassy/raw/main/cyw43-firmware/43439A0.bin
wget https://github.com/embassy-rs/embassy/raw/main/cyw43-firmware/43439A0_clm.bin
cd ..

# Update path in src/main.rs if needed
```

---

### "RPI-RP2 drive not appearing"

**Check:**
```bash
# 1. Verify USB cable supports data (not charge-only)
# 2. Try different USB port
# 3. Check dmesg:
dmesg | tail
# Should see: "usb-storage" device

# 4. Check if mounted:
mount | grep RPI
```

---

## Code Explanation

### Embassy Async Runtime
```rust
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Embassy manages async tasks
    // Allows non-blocking delays
}
```

### WiFi Chip Initialization
```rust
// Pico 2W LED is controlled via CYW43 WiFi chip
let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;

// Spawn task to run WiFi chip state machine
spawner.spawn(wifi_task(runner)).unwrap();
```

### LED Control
```rust
control.gpio_set(0, true).await;   // LED on (GPIO 0 of CYW43 chip)
Timer::after(Duration::from_millis(500)).await;  // Wait 500ms
control.gpio_set(0, false).await;  // LED off
```

### Blink Rate
Current: 1 Hz (500ms on, 500ms off)

Adjust timing:
```rust
Timer::after(Duration::from_millis(100)).await;  // 5 Hz (fast blink)
Timer::after(Duration::from_millis(1000)).await; // 0.5 Hz (slow blink)
Timer::after(Duration::from_millis(50)).await;   // 10 Hz (very fast)
```

---

## Next Steps

After getting LED blinking:

- [ ] **Add GPIO LED** - Use external LED on GP2 with resistor
- [ ] **Morse code** - Blink out "SOS" pattern
- [ ] **Button control** - Start/stop blink with button
- [ ] **Serial output** - Add debug logging
- [ ] **WiFi connection** - Connect to network (see platform guide)
- [ ] **Multiple LEDs** - RGB LED or pattern

**Related examples:**
- Platform guide: `docs/platforms/pi-pico-2w.md`
- GPIO examples: Coming soon!

---

## Why This Example is Complex

**Regular Pico (not 2W):**
```rust
// Simple GPIO LED - just 3 lines!
let mut led = Output::new(p.PIN_25, Level::Low);
led.set_high();  // On
led.set_low();   // Off
```

**Pico 2W (this example):**
- LED controlled by CYW43 WiFi chip (not direct GPIO)
- Requires Embassy async runtime
- Needs WiFi firmware blobs
- More setup, but teaches real-world patterns

**Good news:** Once you understand this, WiFi examples are easier!

---

## Pico 2W Specifications

- **LED:** Onboard green, controlled via CYW43 GPIO0
- **CPU:** Dual-core Cortex-M33 @ 150 MHz
- **RAM:** 520 KB
- **Flash:** 4 MB
- **WiFi:** 2.4 GHz 802.11n
- **Power:** ~100-150 mA when WiFi chip active

---

## Reference Documents

- Pico 2W Getting Started: https://datasheets.raspberrypi.com/picow/getting-started-with-picow.pdf
- RP2350 Datasheet: https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf
- Embassy Book: https://embassy.dev/book/
- Platform setup: `docs/platforms/pi-pico-2w.md`
- CYW43 Driver: https://github.com/embassy-rs/embassy/tree/main/cyw43

---

## Changelog

- 2026-02-13: Initial version
- Follows Example Contract template
- Basic LED blink using CYW43 WiFi chip
- Embassy async runtime
- UF2 bootloader flashing
