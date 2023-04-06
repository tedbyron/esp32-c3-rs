use std::thread;
use std::time::Duration;

use anyhow::{bail, Result};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::log::EspLogger;
use esp_idf_sys as _;
use esp_idf_sys::EspError;
use rgb_led::{Ws2812Rmt, RGB8};

fn main() -> Result<()> {
    EspError::convert(unsafe { esp_idf_sys::nvs_flash_init() })?;
    esp_idf_sys::link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    let cfg = wifi::CONFIG;

    // ESP32-C3-DevKit-RUST-1 gpio2,  ESP32-C3-DevKitC-02 gpio8
    let mut led = Ws2812Rmt::new(peripherals.pins.gpio2, peripherals.rmt.channel0)?;
    led.set_pixel(RGB8::new(10, 10, 0))?;

    // don't drop the result or wifi will disconnect
    let _wifi = match wifi::wifi(cfg.ssid, cfg.psk, peripherals.modem, sysloop) {
        Ok(wifi) => wifi,
        Err(err) => {
            led.set_pixel(RGB8::new(10, 0, 0))?;
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    log::info!("Hello, world!");

    loop {
        led.set_pixel(RGB8::new(0, 10, 0))?;
        thread::sleep(Duration::from_secs(1));

        led.set_pixel(RGB8::new(0, 0, 10))?;
        thread::sleep(Duration::from_secs(1));
    }
}
