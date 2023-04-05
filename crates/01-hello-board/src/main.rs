use std::thread;
use std::time::Duration;

use anyhow::{bail, Result};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::log::EspLogger;
use esp_idf_sys as _;
use log::info;
use rgb_led::{RGB8, WS2812RMT};
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

    info!("Hello, world!");

    let mut led = WS2812RMT::new(peripherals.pins.gpio2, peripherals.rmt.channel0)?;
    led.set_pixel(RGB8::new(50, 50, 0))?;

    let app_config = CONFIG;

    match wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => inner,
        Err(err) => {
            led.set_pixel(RGB8::new(50, 0, 0))?;
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    loop {
        led.set_pixel(RGB8::new(0, 0, 50))?;
        thread::sleep(Duration::from_secs(1));

        led.set_pixel(RGB8::new(0, 50, 0))?;
        thread::sleep(Duration::from_secs(1));
    }
}
