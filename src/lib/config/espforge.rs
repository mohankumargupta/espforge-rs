use serde::{self, Deserialize};

#[derive(Deserialize, Debug)]
pub struct EspConfig {
    name: String,
    chip: String,
}

#[derive(Deserialize, Debug)]
pub struct EspForgeConfig {
    pub espforge: EspConfig,
}