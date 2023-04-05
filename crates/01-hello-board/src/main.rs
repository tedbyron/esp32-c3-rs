use std::thread;
use std::time::Duration;

use anyhow::{bail, Result};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::log::EspLogger;
use log::info;
use rgb_led::{Ws2812Rmt, RGB8};
use wifi::wifi;

#[toml_cfg::toml_config]
#[derive(Debug)]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    let Config {
        wifi_ssid,
        wifi_psk,
    } = CONFIG;

    info!("Hello, world!");

    // ESP32-C3-DevKit-RUST-1 gpio2,  ESP32-C3-DevKitC-02 gpio8
    let mut led = Ws2812Rmt::new(peripherals.pins.gpio2, peripherals.rmt.channel0)?;
    led.set_pixel(RGB8::new(50, 50, 0))?;

    // don't drop the result or wifi will disconnect
    let _wifi = match wifi(wifi_ssid, wifi_psk, peripherals.modem, sysloop) {
        Ok(wifi) => wifi,
        Err(err) => {
            led.set_pixel(RGB8::new(50, 0, 0))?;
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    loop {
        led.set_pixel(RGB8::new(0, 50, 0))?;
        thread::sleep(Duration::from_secs(1));

        led.set_pixel(RGB8::new(0, 0, 50))?;
        thread::sleep(Duration::from_secs(1));
    }
}
