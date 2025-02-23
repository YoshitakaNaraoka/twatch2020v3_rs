use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::*;
use esp_idf_sys::EspError;
use esp_idf_svc::log::*;
use log::*;
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone, Copy)]
enum State {
    On,
    Off,
}

fn main() -> Result<(), EspError> {
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pin = PinDriver::input(peripherals.pins.gpio35)?;

    let mut last_state = match pin.is_low() {
        Ok(true) => State::On,
        Ok(false) => State::Off,
        Err(e) => return Err(e), // エラーが発生した場合は早期リターン
    };
    let debounce_delay = Duration::from_millis(50);

    loop {
        let current_state = match pin.is_low() {
            Ok(true) => State::On,
            Ok(false) => State::Off,
            Err(e) => return Err(e),
        };

        match current_state {
            state if state == last_state => {
                thread::sleep(Duration::from_millis(10));
            }
            state => {
                thread::sleep(debounce_delay);
                let debounced_state = match pin.is_low() {
                    Ok(true) => State::On,
                    Ok(false) => State::Off,
                    Err(e) => return Err(e),
                };
                if debounced_state == state {
                    match state {
                        State::On => info!("HelloButton!"),
                        State::Off => {}
                    }
                    last_state = state;
                }
            }
        }
    }
}