pub mod core;
pub mod module;
pub mod utilities;
use crate::core::hardware::*;
use crate::core::hardware::{
    ledc::{
        config::TimerConfig,
        LedcTimerDriver,
        Resolution
    }
};
use module::ledmodule::Ledmodule;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting simple GPIO15 blink test...");

    let peripherals = Peripherals::take()?;

     let timer_config = TimerConfig::new()
        .frequency(5_u32.kHz().into())
        .resolution(Resolution::Bits13);
    let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config)?;

    let mut led_module = Ledmodule::new(peripherals.pins.gpio15 ,peripherals.ledc.channel0 ,&timer)?;

    loop {
       for duty in (0..=100).step_by(1) {
            led_module.set_state(duty)?;
            sleep_time(25);
        }
        for duty in (0..=100).rev().step_by(1) {
             led_module.set_state(duty)?;
           sleep_time(25);
        }
    }
}
