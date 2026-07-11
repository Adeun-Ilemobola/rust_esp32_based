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

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut modules: HashMap<String, ModuleHandle<'_>> = HashMap::new();

    let peripherals = Peripherals::take()?;
    let hardware = HardwareContext::new(peripherals)?;

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
