use anyhow::Result;
use embuild::build::{CfgArgs, LinkArgs};

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    // https://github.com/rust-lang/cargo/issues/9641
    CfgArgs::output_propagated("ESP_IDF")?;
    LinkArgs::output_propagated("ESP_IDF")
}
