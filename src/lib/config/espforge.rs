use serde::{self, Deserialize};

#[derive(Deserialize, Debug)]
pub struct EspConfig {
    pub name: String,
    pub chip: String,
}

#[derive(Deserialize, Debug)]
pub struct EspForgeConfig {
    pub espforge: EspConfig,
}