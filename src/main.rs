mod Utilities;

use std::thread::sleep;
use std::time::Duration;



fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting ESP32 Rust app...");
    setup();

    loop {
        update();
        sleep(Duration::from_millis(1000));
    }
}
fn setup() {
    log::info!("Setup running...");

    let raw_value = 512.0;

    let mapped = Utilities::math::map_range(
        raw_value,
        0.0,
        1023.0,
        0.0,
        100.0,
    );

    let limited = Utilities::math::constrain_f32(
        mapped,
        0.0,
        100.0,
    );

    log::info!("Mapped value: {}", mapped);
    log::info!("Limited value: {}", limited);
}

fn update() {
    log::info!("Loop tick...");
}
