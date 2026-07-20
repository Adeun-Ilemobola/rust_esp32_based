use anyhow::Ok;
pub use esp_idf_svc::hal::delay::FreeRtos;
pub use esp_idf_svc::hal::gpio::*;
pub use esp_idf_svc::hal::i2c;
pub use esp_idf_svc::hal::i2c::{I2c, I2cConfig, I2cDriver};
pub use esp_idf_svc::hal::ledc;
use esp_idf_svc::hal::ledc::config::TimerConfig;
use esp_idf_svc::hal::ledc::Resolution;
pub use esp_idf_svc::hal::peripherals::Peripherals;
pub use esp_idf_svc::hal::uart::UartDriver;
pub use esp_idf_svc::hal::units::*;
pub use esp_idf_svc::partition::*;
use pwm_pca9685::{Address, Pca9685};
use std::cell::RefCell;
use std::rc::Rc;
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
    pub fn new<T>(pin: T, pull_mode: Pull) -> anyhow::Result<Self>
    where
        T: InputPin + 'd,
    {
        let pin_number = pin.pin() as u8;
        let driver = PinDriver::input(pin, pull_mode)?;

        Ok(Self {
            pin_number,
            driver,
            mode: pull_mode,
        })
    }
    pub fn pin_number(&self) -> u8 {
        self.pin_number
    }

    pub fn high(&self) -> anyhow::Result<bool> {
        Ok(self.driver.is_high())
    }
    pub fn low(&self) -> anyhow::Result<bool> {
        Ok(self.driver.is_low())
    }
    pub fn now(&self) -> anyhow::Result<Level> {
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

pub fn sleep_time(ms: u32) {
    FreeRtos::delay_ms(ms);
}
pub fn sleep_ms(ms: u64) {
    std::thread::sleep(Duration::from_millis(ms));
}

pub type SharedPwm<'d> = Rc<RefCell<Pca9685<I2cDriver<'d>>>>;
pub type LedTimer<'d> = ledc::LedcTimerDriver<'d, ledc::LowSpeed>;

pub struct HardwareContext<'d> {
    pub servo_pwm: SharedPwm<'d>,
    pub led_timer: LedTimer<'d>,
}

impl<'d> HardwareContext<'d> {
    pub fn new<I2C, SDA, SCL, TIMER>(
        i2c: I2C,
        sda: SDA,
        scl: SCL,
        timer: TIMER,
    ) -> anyhow::Result<HardwareContext<'d>>
    where
        I2C: I2c + 'd,
        SDA: InputPin + OutputPin + 'd,
        SCL: InputPin + OutputPin + 'd,
        TIMER: ledc::LedcTimer<SpeedMode = ledc::LowSpeed> + 'd,
    {
        Ok(Self {
            servo_pwm: Self::create_shared_pwm(i2c, sda, scl)?,
            led_timer: Self::create_led_timer(timer)?,
        })
    }
    pub fn create_shared_pwm<I2C, SDA, SCL>(
        i2c: I2C,
        sda: SDA,
        scl: SCL,
    ) -> anyhow::Result<SharedPwm<'d>>
    where
        I2C: I2c + 'd,
        SDA: InputPin + OutputPin + 'd,
        SCL: InputPin + OutputPin + 'd,
    {
        let i2c_config = I2cConfig::new().baudrate(400.kHz().into());
        let i2c = I2cDriver::new(i2c, sda, scl, &i2c_config)?;

        let mut pwm = Pca9685::new(i2c, Address::default())
            .map_err(|e| anyhow::anyhow!("pca9685 init: {:?}", e))?;
        pwm.set_prescale(100)
            .map_err(|e| anyhow::anyhow!("set_prescale: {:?}", e))?;
        pwm.enable()
            .map_err(|e| anyhow::anyhow!("enable: {:?}", e))?;

        Ok(Rc::new(RefCell::new(pwm)))
    }

    // pub fn create_i2c<I2C ,SDA, SCL>(i2c: I2C, sda: SDA, scl: SCL,)-> anyhow::Result<I2cDriver<'d>>
    // where
    //  I2C: I2c + 'd,
    //     SDA: InputPin + OutputPin + 'd,
    //     SCL: InputPin + OutputPin + 'd,
    // {
    //      let i2c_config = I2cConfig::new().baudrate(400.kHz().into());
    //      let i2c = I2cDriver::new(i2c, sda, scl, &i2c_config)?;

    //      ok(i2c)

    // }

    pub fn create_led_timer<T>(timer: T) -> anyhow::Result<LedTimer<'d>>
    where
        T: ledc::LedcTimer<SpeedMode = ledc::LowSpeed> + 'd,
    {
        let timer_config = TimerConfig::new()
            .frequency(5_u32.kHz().into())
            .resolution(Resolution::Bits13);

        Ok(ledc::LedcTimerDriver::new(timer, &timer_config)?)
    }
}
