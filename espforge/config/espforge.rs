use std::collections::HashMap;

use serde::{self, Deserialize};
use toml::Value;

#[derive(Deserialize, Debug)]
pub struct EspConfig {
    pub name: String,
    pub chip: String,
}

#[derive(Deserialize, Debug)]
pub struct EspForgeConfig {
    pub espforge: EspConfig,
    #[serde(default)]
    pub example: HashMap<String, Value>,
}

