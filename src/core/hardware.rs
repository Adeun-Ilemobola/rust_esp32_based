pub use esp_idf_svc::hal::delay::FreeRtos;
pub use esp_idf_svc::hal::gpio::*;
pub use esp_idf_svc::hal::peripherals::Peripherals;
pub use esp_idf_svc::hal::ledc;
pub use esp_idf_svc::partition::*;
pub use esp_idf_svc::hal::units::*;
pub use esp_idf_svc::hal::uart::UartDriver;  
use std::time::Duration;

pub struct OutputPinCore<'d> {
    pin_number: u8,
    driver: PinDriver<'d, Output>,
}
pub struct InputPinCore<'d> {
    pin_number: u8,
    driver: PinDriver<'d, Input>,
    mode: Pull,
}


impl<'d> InputPinCore<'d> {
    pub fn new<T >(pin: T , pull_mode: Pull,) -> anyhow::Result<Self>
    where
        T: InputPin + 'd,
    {
        let pin_number = pin.pin() as u8; 
        let driver = PinDriver::input(pin , pull_mode)?;
       
        Ok(Self {
            pin_number,
            driver,
            mode: pull_mode,
        })
    }
    pub fn pin_number(&self) -> u8 {
        self.pin_number
    }

    pub  fn high(&self)-> anyhow::Result<bool> {
        Ok(self.driver.is_high())
    }
    pub  fn low(&self)-> anyhow::Result<bool> {
        Ok(self.driver.is_low())
    }
    pub  fn now(&self)-> anyhow::Result<Level> {
        Ok(self.driver.get_level())
    }

}

impl<'d> OutputPinCore<'d> {
    pub fn new<T>(pin: T) -> anyhow::Result<Self>
    where
        T: OutputPin + 'd,
    {
        let pin_number = pin.pin() as u8; 
        let driver = PinDriver::output(pin)?; 
        Ok(Self { pin_number, driver })
    }

    pub fn pin_number(&self) -> u8 {
        self.pin_number
    }

    pub fn set_state(&mut self, state: bool) -> anyhow::Result<()> {
        if state {
            self.driver.set_high()?;
        } else {
            self.driver.set_low()?;
        }

        Ok(())
    }

    pub fn toggle(&mut self) -> anyhow::Result<()> {
        self.driver.toggle()?;
        Ok(())
    }
}

pub fn sleep_time(ms:u32){
    FreeRtos::delay_ms(ms);
}
pub fn sleep_ms(ms:u64){
    std::thread::sleep(Duration::from_millis(ms));
}