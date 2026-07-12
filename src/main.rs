pub mod core;
pub mod module;
pub mod utilities;
use crate::core::hardware::ledc::{config::TimerConfig, LedcTimerDriver, Resolution};
use crate::core::hardware::*;
use crate::core::modulecore::Module;
// use crate::module::servomodule::ServoModule;
use crate::utilities::moduleconflg::ServoConfig;
use crate::utilities::serdeprotocol::IncomingCommand;
use module::ledmodule::Ledmodule;
use pwm_pca9685::{ Channel};
use std::io;
use std::io::{BufRead, ErrorKind};
use std::sync::mpsc;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type ModuleHandle<'a> = Rc<RefCell<dyn Module + 'a>>;

/// Prints a compact startup report to the serial logger.
///
/// Update the Wi-Fi and Bluetooth values when those services are initialized.
fn print_welcome_message(wifi_status: &str, bluetooth_status: &str) {
    use esp_idf_svc::sys;

    // These values come directly from ESP-IDF after the hardware has initialized.
    let (total_ram, free_ram, minimum_free_ram, flash_size) = unsafe {
        let total_ram = sys::heap_caps_get_total_size(sys::MALLOC_CAP_8BIT);
        let free_ram = sys::esp_get_free_heap_size() as usize;
        let minimum_free_ram = sys::esp_get_minimum_free_heap_size() as usize;

        let mut flash_size = 0_u32;
        let flash_result = sys::esp_flash_get_physical_size(
            sys::esp_flash_default_chip,
            &mut flash_size,
        );

        (
            total_ram,
            free_ram,
            minimum_free_ram,
            (flash_result == sys::ESP_OK).then_some(flash_size as usize),
        )
    };

    let used_ram = total_ram.saturating_sub(free_ram);
    let ram_usage = if total_ram == 0 {
        0.0
    } else {
        used_ram as f64 * 100.0 / total_ram as f64
    };

    log::info!("==================================================");
    log::info!("Welcome! Your ESP32 is up and running.");
    log::info!("Firmware: {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    log::info!(
        "ESP-IDF: {}.{}.{}",
        sys::ESP_IDF_VERSION_MAJOR,
        sys::ESP_IDF_VERSION_MINOR,
        sys::ESP_IDF_VERSION_PATCH
    );
    match flash_size {
        Some(bytes) => log::info!("Flash storage: {}", format_bytes(bytes)),
        None => log::warn!("Flash storage: unavailable"),
    }
    log::info!(
        "RAM: {} free / {} total ({:.1}% used)",
        format_bytes(free_ram),
        format_bytes(total_ram),
        ram_usage
    );
    log::info!(
        "Lowest free RAM since boot: {}",
        format_bytes(minimum_free_ram)
    );
    log::info!("Wi-Fi: {}", wifi_status);
    log::info!("Bluetooth: {}", bluetooth_status);
    log::info!("System status: ready");
    log::info!("==================================================");
}

fn format_bytes(bytes: usize) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;

    let bytes = bytes as f64;
    if bytes >= GIB {
        format!("{:.2} GiB", bytes / GIB)
    } else if bytes >= MIB {
        format!("{:.2} MiB", bytes / MIB)
    } else if bytes >= KIB {
        format!("{:.2} KiB", bytes / KIB)
    } else {
        format!("{} B", bytes as usize)
    }
}

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut modules: HashMap<String, ModuleHandle<'_>> = HashMap::new();

    let peripherals = Peripherals::take()?;
    let hardware = HardwareContext::new(peripherals)?;

    print_welcome_message("not initialized", "not initialized");

    // let timer_config = TimerConfig::new()
    //     .frequency(5_u32.kHz().into())
    //     .resolution(Resolution::Bits13);
    // let timer: LedcTimerDriver<'_, ledc::LowSpeed> = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config)?;

    let pwm = hardware.servo_pwm;

    // let sorvo = Rc::new(RefCell::new(ServoModule::new(
    //     pwm.clone(),
    //     "sorvo1".to_string(),
    //     Channel::C0,
    //     ServoConfig {
    //         max_angle: 180,
    //         min_angle: 0,
    //         max_pivot: 35,
    //         min_pivot: -35,
    //         pulse_max: 2500,
    //         pulse_min: 500,
    //         offset: (180 / 2),
    //     },
    //     None,
    // )?));
    // modules.insert(sorvo.borrow().id().to_string(), sorvo.clone());
    // let sorvo1 = Rc::new(RefCell::new(ServoModule::new(
    //     pwm.clone(),
    //     "sorvo2".to_string(),
    //     Channel::C1,
    //     ServoConfig {
    //         max_angle: 180,
    //         min_angle: 0,
    //         max_pivot: 35,
    //         min_pivot: -35,
    //         pulse_max: 2500,
    //         pulse_min: 500,
    //         offset: (180 / 2),
    //     },
    //     None,
    // )?));
    // modules.insert(sorvo1.borrow().id().to_string(), sorvo1.clone());

    let (command_sender, command_receiver) = mpsc::channel::<IncomingCommand>();
    std::thread::spawn(move || {
        serial_command_reader(command_sender);
    });

    loop {
        // if btu.poll()? {
        //     led_module.borrow_mut().toggle()?
        // }

        if let Ok(command) = command_receiver.try_recv() {
            if let Some(m) = modules.get_mut(&command.id) {
                m.borrow_mut().handle_command(&command.command)?;
            }
        }
        sleep_ms(10);
    }
}

fn serial_command_reader(command_sender: mpsc::Sender<IncomingCommand>) {
    let stdin = io::stdin();

    for line_result in stdin.lock().lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(err) if err.kind() == ErrorKind::WouldBlock => {
                // Not an error — just no data yet. Back off and retry.
                std::thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }
            Err(err) => {
                log::error!("Real serial read error: {:?}", err);
                continue;
            }
        };

        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        match serde_json::from_str::<IncomingCommand>(line) {
            Ok(command) => {
                log::info!("Parsed command: {:?}", command);

                let _ = command_sender.send(command);
            }

            Err(err) => {
                log::error!("Failed to parse command: {:?}", err);
                log::error!("Raw line was: {}", line);
            }
        }
    }
}
