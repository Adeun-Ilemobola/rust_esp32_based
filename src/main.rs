pub mod core;
pub mod module;
pub mod utilities;
use crate::core::hardware::ledc::{config::TimerConfig, LedcTimerDriver, Resolution};
use crate::core::hardware::*;
use module::ledmodule::Ledmodule;
use serde::Deserialize;
use std::io;
use std::io::{ BufRead, ErrorKind};
use std::sync::mpsc;

#[derive(serde::Deserialize)]
enum EspCommand {
    SetLed { id: u8, value: u32 },
    Ping,
}

#[derive(Debug, Deserialize)]
pub struct IncomingCommand {
    pub kind: String,
    pub id: String,

    #[serde(flatten)]
    pub command: ModuleCommand,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "moduletype", content = "payload", rename_all = "snake_case")]
pub enum ModuleCommand {
    Led(LedCommandPayload),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum LedCommandPayload {
    SetState { state: u32 },
    Toggle,
}

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Starting simple GPIO15 blink test...");

    let peripherals = Peripherals::take()?;

    let timer_config = TimerConfig::new()
        .frequency(5_u32.kHz().into())
        .resolution(Resolution::Bits13);
    let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config)?;

    let mut led_module =
        Ledmodule::new(peripherals.pins.gpio15, peripherals.ledc.channel0, &timer)?;

    let (command_sender, command_receiver) = mpsc::channel::<IncomingCommand>();

    std::thread::spawn(move || {
        serial_command_reader(command_sender);
    });

    loop {
       
        // for duty in (0..=100).step_by(1) {
        //     led_module.set_state(duty)?;
        //     sleep_time(25);
        // }
        // for duty in (0..=100).rev().step_by(1) {
        //     led_module.set_state(duty)?;
        //     sleep_time(25);
        // }

         if let Ok(command) = command_receiver.try_recv() {
            match command.command {
                ModuleCommand::Led(led_command) => {
                    match led_command {
                        LedCommandPayload::SetState { state } => {
                            led_module.set_state(state)?;
                        }

                        LedCommandPayload::Toggle => {
                            led_module.toggle()?;
                        }
                    }

                    
                }
            }
        }
        sleep_time(15);
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
