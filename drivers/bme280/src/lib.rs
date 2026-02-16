#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead};

const BME280_I2C_ADDR: u8 = 0x76;

const REG_ID: u8 = 0xD0;
const REG_RESET: u8 = 0xE0;
const REG_CTRL_MEAS: u8 = 0xF4;
const REG_CONFIG: u8 = 0xF5;
const REG_PRESS_MSB: u8 = 0xF7;

#[derive(Debug)]
pub enum Error<E> {
    I2c(E),
    InvalidDevice,
}

pub struct Bme280<I2C> {
    i2c: I2C,
}

impl<I2C, E> Bme280<I2C>
where
    I2C: Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn init(&mut self) -> Result<(), Error<E>> {
        let mut id = [0];
        self.i2c
            .write_read(BME280_I2C_ADDR, &[REG_ID], &mut id)
            .map_err(Error::I2c)?;

        if id[0] != 0x60 {
            return Err(Error::InvalidDevice);
        }

        // Soft reset
        self.i2c
            .write(BME280_I2C_ADDR, &[REG_RESET, 0xB6])
            .map_err(Error::I2c)?;

        // ctrl_meas: temp x1, press x1, normal mode
        self.i2c
            .write(BME280_I2C_ADDR, &[REG_CTRL_MEAS, 0x27])
            .map_err(Error::I2c)?;

        // config: standby 1000ms
        self.i2c
            .write(BME280_I2C_ADDR, &[REG_CONFIG, 0xA0])
            .map_err(Error::I2c)?;

        Ok(())
    }

    pub fn read_raw(&mut self) -> Result<(u32, u32, u32), Error<E>> {
        let mut data = [0u8; 8];

        self.i2c
            .write_read(BME280_I2C_ADDR, &[REG_PRESS_MSB], &mut data)
            .map_err(Error::I2c)?;

        let pressure = ((data[0] as u32) << 12)
            | ((data[1] as u32) << 4)
            | ((data[2] as u32) >> 4);

        let temperature = ((data[3] as u32) << 12)
            | ((data[4] as u32) << 4)
            | ((data[5] as u32) >> 4);

        let humidity = ((data[6] as u32) << 8) | (data[7] as u32);

        Ok((temperature, pressure, humidity))
    }
}
