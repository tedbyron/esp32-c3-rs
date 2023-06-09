use std::fs::File;
use std::io::Write;

use anyhow::Result;
use uuid::Uuid;

fn main() -> Result<()> {
    if let Ok(_already_exists) = File::open("uuid.toml") {
        return Ok(());
    }

    let mut uuid_file = File::create("uuid.toml")?;
    uuid_file.write_all(b"[get-uuid]\n")?;
    let uuid_val = Uuid::new_v4().to_string();
    uuid_file.write_fmt(format_args!("uuid = \"{}\"\n", uuid_val))?;

    let package_root = env!("CARGO_MANIFEST_DIR");
    let uuid_rs = format!("{}/_uuid.rs", package_root);
    let mut uuid_file = File::create(uuid_rs)?;
    uuid_file.write_fmt(format_args!(
        "const UUID: &'static str = \"{}\";\n",
        uuid_val
    ))?;

    Ok(())
}
