use anyhow::Result;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _;
use log::info;
use rgb_led::Ws2812Rmt;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    // ESP32-C3-DevKit-RUST-1 gpio2,  ESP32-C3-DevKitC-02 gpio8
    let led = peripherals.pins.gpio2;
    let channel = peripherals.rmt.channel0;
    let mut ws2812 = Ws2812Rmt::new(led, channel)?;
    loop {
        info!("Red!");
        ws2812.set_pixel(rgb::RGB8::new(255, 0, 0))?;
        FreeRtos::delay_ms(1000);
        info!("Green!");
        ws2812.set_pixel(rgb::RGB8::new(0, 255, 0))?;
        FreeRtos::delay_ms(1000);
        info!("Blue!");
        ws2812.set_pixel(rgb::RGB8::new(0, 0, 255))?;
        FreeRtos::delay_ms(1000);
    }
}
