//! VL53L0X Time-of-Flight Distance Sensor
//!
//! Hardware: ESP32, VL53L0X
//! Connections:
//!   SDA -> GPIO21
//!   SCL -> GPIO22
//!   VCC -> 3.3V
//!   GND -> GND

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::IO,
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    Delay,
};
use esp_println::println;
use vl53l0x::VL53L0x;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut delay = Delay::new(&clocks);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        100u32.kHz(),
        &clocks,
    );

    println!("Initializing VL53L0X...");
    let mut sensor = VL53L0x::new(i2c).expect("Failed to create sensor");
    
    println!("VL53L0X ready!\n");

    loop {
        match sensor.read_range_mm() {
            Ok(distance) => {
                println!("Distance: {} mm ({:.2} cm)", 
                    distance, distance as f32 / 10.0);
                
                if distance < 200 {
                    println!("⚠️  OBSTACLE CLOSE!");
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }

        delay.delay_ms(100u32);
    }
}
