use esp_idf_hal::prelude::*;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_sys::EspError;

fn main() -> Result<(), EspError> {
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pin = PinDriver::input(peripherals.pins.gpio22)?;

        if pin.is_high() {
                    
        } else {
            println!("HelloButton!")
        }
        Ok(())
}