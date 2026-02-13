use gpio_cdev::{Chip, LineRequestFlags};
use std::thread;
use std::time::Duration;
use anyhow::Result;

fn main() -> Result<()> {
    println!("Jetson GPIO Blink Example");
    println!("Blinking GPIO 12 (Pin 12)");
    
    // Open GPIO chip
    let mut chip = Chip::new("/dev/gpiochip0")?;
    
    // Get GPIO line 12 (Pin 12 on header)
    let line = chip.get_line(12)?;
    
    // Request line as output
    let handle = line.request(LineRequestFlags::OUTPUT, 0, "gpio-blink")?;
    
    println!("Connect LED with resistor between Pin 12 and GND");
    println!("Press Ctrl+C to stop\n");
    
    let mut state = false;
    
    loop {
        // Toggle LED
        handle.set_value(state as u8)?;
        
        if state {
            println!("LED ON");
        } else {
            println!("LED OFF");
        }
        
        state = !state;
        thread::sleep(Duration::from_millis(500));
    }
}
