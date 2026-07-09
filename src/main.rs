pub mod core;
pub mod module;
pub mod utilities;
use pwm_pca9685::{Address, Channel, Pca9685};
use crate::core::hardware::ledc::{config::TimerConfig, LedcTimerDriver, Resolution};
use crate::core::hardware::i2c::{
    I2cConfig,
    I2cDriver

};
use crate::core::hardware::*;
use crate::core::modulecore::Module;
use crate::module::buttonmodule::Buttonmodule;
use crate::utilities::serdeprotocol::IncomingCommand;
use module::ledmodule::Ledmodule;
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
    let config = I2cConfig::new().baudrate(400.kHz().into());

    let timer_config = TimerConfig::new()
        .frequency(5_u32.kHz().into())
        .resolution(Resolution::Bits13);
    let timer = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config)?;

    let mut i2c = I2cDriver::new(peripherals.i2c0,peripherals.pins.gpio2, // SDA
        peripherals.pins.gpio3, // SCL
        &config,
    )?;




    let led_module = Rc::new(RefCell::new(Ledmodule::new(
        peripherals.pins.gpio15,
        peripherals.ledc.channel0,
        &timer,
        None
    )?));
    modules.insert(led_module.borrow().id().to_string(), led_module.clone());


  
    // let  but = Rc::new(RefCell::new(Buttonmodule::new(peripherals.pins.gpio0)?));
    let mut  btu = Buttonmodule::new(peripherals.pins.gpio12)?;


    //   modules.insert(but.borrow().id().to_string(), but.clone());
    let (command_sender, command_receiver) = mpsc::channel::<IncomingCommand>();

    std::thread::spawn(move || {
        serial_command_reader(command_sender);
    });

    loop {
        if btu.poll()? {
            led_module.borrow_mut().toggle()?
        }

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
