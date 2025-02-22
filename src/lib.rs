use std::time::{Duration, Instant};

use button_driver::{Button, ButtonConfig, InstantProvider};
use esp_idf_hal::{gpio::PinDriver, prelude::Peripherals};
use esp_idf_sys::EspError;
use log::info;

fn main() -> Result<(), EspError> {
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pin = PinDriver::input(peripherals.pins.gpio35)?;

    let mut button = Button::<_, Duration>::new(pin, ButtonConfig::default());

    loop {
        button.tick();

        if let Some(dur) = button.held_time() {
            info!("Total holding time {:?}", dur);

            if button.is_clicked() {
                info!("Clicked + held");
            } else if button.is_double_clicked() {
                info!("Double clicked + held");
            } else if button.holds() == 2 && button.clicks() > 0 {
                info!("Held twice with {} clicks", button.clicks());
            } else if button.holds() == 2 {
                info!("Held twice");
            }
        } else {
            if button.is_clicked() {
                info!("Click");
            } else if button.is_double_clicked() {
                info!("Double click");
            } else if button.is_triple_clicked() {
                info!("Triple click");
            } else if let Some(dur) = button.current_holding_time() {
                info!("Held for {:?}", dur);
            }
        }

        button.reset();
    }
}