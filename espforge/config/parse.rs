use std::{fs::read_to_string, path::PathBuf};
use thiserror::Error;
use crate::config::espforge::{EspConfig, EspForgeConfig};
use crate::ParsedExample;

pub struct ParsedConfig {
    pub esp_config: EspConfig,
    pub example: Option<ParsedExample>,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read the configuration file")]
    FileRead(#[from] std::io::Error),
    #[error("Failed to parse the TOML configuration")]
    TomlParse(#[from] toml::de::Error),
}

pub fn parse_config(config_path: &PathBuf) -> Result<ParsedConfig, ConfigError> {
    let contents = read_to_string(config_path)?;
    let config: EspForgeConfig = toml::from_str(&contents)?;

    let esp_config = config.espforge;
    let mut parsed_example: Option<ParsedExample> = None;

    // Take the first example found in the config file.
    if let Some((name, value)) = config.example.into_iter().next() {
        parsed_example = ParsedExample::handle_example(&name, &value);
    }

    Ok(ParsedConfig {
        esp_config,
        example: parsed_example,
    })
}
