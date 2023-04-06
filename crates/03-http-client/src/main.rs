use core::str;

use anyhow::{bail, Result};
use embedded_svc::http::{client::Client, Status};
use embedded_svc::io::Read;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use esp_idf_sys as _;
use esp_idf_sys::EspError;

fn main() -> Result<()> {
    EspError::convert(unsafe { esp_idf_sys::nvs_flash_init() })?;
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    let cfg = wifi::CONFIG;
    let _wifi = wifi::wifi(cfg.ssid, cfg.psk, peripherals.modem, sysloop)?;

    let cfg = Configuration::default();
    let conn = EspHttpConnection::new(&cfg)?;
    let mut client = Client::wrap(conn);

    get(&mut client, "http://example.com/")?;

    Ok(())
}

fn get(client: &mut Client<EspHttpConnection>, url: impl AsRef<str>) -> Result<()> {
    let req = client.get(url.as_ref())?;
    let mut res = req.submit()?;
    let status = res.status();
    let status_msg = res.status_message();
    println!("response status: {status}");

    match status {
        200..=299 => {
            let mut buf = [0; 256];
            let mut body = String::with_capacity(256);
            while let Ok(n) = res.read(&mut buf) {
                if n == 0 {
                    break;
                }
                body += str::from_utf8(&buf[..n])?;
            }
            println!("{body}");
        }
        _ => bail!(
            "unexpected response{}",
            status_msg.map_or_else(String::new, |msg| format!(": {}", msg))
        ),
    }

    Ok(())
}
