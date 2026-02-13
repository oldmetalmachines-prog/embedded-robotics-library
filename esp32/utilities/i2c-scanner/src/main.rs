#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::IO,
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
};
use esp_println::println;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut delay = Delay::new(&clocks);

    // I2C pins
    let sda = io.pins.gpio21;
    let scl = io.pins.gpio22;

    let mut i2c = I2C::new(
        peripherals.I2C0,
        sda,
        scl,
        100u32.kHz(),
        &clocks,
    );

    println!("\n=== ESP32 I2C Scanner ===");
    println!("Scanning I2C bus on SDA=GPIO21, SCL=GPIO22\n");

    delay.delay_ms(1000u32);

    let mut devices_found = 0;

    for addr in 1..=127 {
        // Try to write to address
        match i2c.write(addr, &[]) {
            Ok(_) => {
                println!("✓ Device found at address 0x{:02X} ({})", addr, addr);
                devices_found += 1;
            }
            Err(_) => {
                // No device at this address, continue silently
            }
        }
        delay.delay_ms(5u32);
    }

    println!("\nScan complete!");
    println!("Total devices found: {}", devices_found);

    if devices_found == 0 {
        println!("\n⚠️  No I2C devices found!");
        println!("Check:");
        println!("  - Wiring (SDA/SCL not swapped?)");
        println!("  - Pull-up resistors (2.2kΩ to 3.3V)");
        println!("  - Power to sensor (3.3V)");
    }

    loop {
        delay.delay_ms(1000u32);
    }
}
