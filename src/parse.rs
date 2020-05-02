use failure::Error;
use std::fs;
use toml::Value;

pub fn config() -> Result<Value, Error> {
    let toml_content = fs::read_to_string("pndev.toml")?;
    let package_info: Value = toml::from_str(&toml_content)?;

    Ok(package_info)
}
