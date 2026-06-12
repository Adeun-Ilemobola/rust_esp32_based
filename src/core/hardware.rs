pub use esp_idf_svc::hal::delay::FreeRtos;
pub use esp_idf_svc::hal::gpio::*;
pub use esp_idf_svc::hal::peripherals::Peripherals;

pub struct OutputPinCore<'d> {
    pin_number: u8,
    driver: PinDriver<'d, Output>,
}

impl<'d> OutputPinCore<'d> {
    pub fn new<T>(pin_number: u8, pin: T) -> anyhow::Result<Self>
    where
        T: OutputPin + 'd,
    {
        let driver = PinDriver::output(pin)?;

        Ok(Self {
            pin_number,
            driver,
        })
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