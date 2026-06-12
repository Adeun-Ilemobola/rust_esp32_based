pub mod core;
pub mod module;
pub mod utilities;
use crate::core::hardware::*;
use module::ledmodule::Ledmodule;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting simple GPIO15 blink test...");

    let peripherals = Peripherals::take()?;

    let mut led_module = Ledmodule::new(15, peripherals.pins.gpio15)?;

    loop {
      
        FreeRtos::delay_ms(1000);
        led_module.set_state(true);

        led_module.set_state(false);
        FreeRtos::delay_ms(1000);
    }
}
