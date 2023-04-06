use anyhow::{bail, Result};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_sys as _;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    let cfg = wifi::CONFIG;
    let _wifi = match wifi::wifi(cfg.wifi_ssid, cfg.wifi_psk, peripherals.modem, sysloop) {
        Ok(wifi) => {
            println!("Connected to Wi-Fi network!");
            wifi
        }
        Err(err) => bail!("Could not connect to Wi-Fi network: {:?}", err),
    };

    Ok(())
}
