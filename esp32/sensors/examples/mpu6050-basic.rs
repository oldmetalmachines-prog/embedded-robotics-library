//! MPU6050 IMU Basic Example
//!
//! Hardware: ESP32-S3/P4, MPU6050
//! Connections:
//!   SDA -> GPIO21 (or GPIO8 for P4)
//!   SCL -> GPIO22 (or GPIO9 for P4)
//!   VCC -> 3.3V
//!   GND -> GND
//!
//! Dependencies (add to Cargo.toml):
//! ```toml
//! mpu6050 = "0.1"
//! embedded-hal = "1.0"
//! ```

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
use mpu6050::Mpu6050;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut delay = Delay::new(&clocks);

    // I2C setup
    let sda = io.pins.gpio21;
    let scl = io.pins.gpio22;

    let i2c = I2C::new(
        peripherals.I2C0,
        sda,
        scl,
        100u32.kHz(),
        &clocks,
    );

    // Initialize MPU6050
    println!("Initializing MPU6050...");
    let mut mpu = Mpu6050::new(i2c);
    
    mpu.init(&mut delay).expect("Failed to init MPU6050");
    println!("MPU6050 initialized!");

    loop {
        if let Ok(accel) = mpu.get_acc() {
            println!("Accel: X={:.2}, Y={:.2}, Z={:.2}", 
                accel.x, accel.y, accel.z);
        }

        if let Ok(gyro) = mpu.get_gyro() {
            println!("Gyro:  X={:.2}, Y={:.2}, Z={:.2}", 
                gyro.x, gyro.y, gyro.z);
        }

        delay.delay_ms(500u32);
    }
}
