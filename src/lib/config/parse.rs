use std::{fs::read_to_string, path::PathBuf};
use crate::config::espforge::{EspConfig, EspForgeConfig};

pub fn parse_config(espforge_config: &PathBuf) -> EspConfig{
    let contents = read_to_string(&espforge_config).expect("file corrupt");
    let config: EspForgeConfig = toml::from_str(&contents).expect("toml deserial failed");
    let espforge_config = config.espforge;
    return espforge_config;
    //let esp_config: EspConfig = config.espforge;
    //print!("{:?}\n", &esp_config);
}

