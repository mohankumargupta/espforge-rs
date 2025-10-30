use std::{collections::HashMap, fs::read_to_string, path::PathBuf};
use toml::Value;

use crate::config::espforge::{EspConfig, EspForgeConfig};

pub fn parse_config(espforge_config: &PathBuf) -> EspConfig{
    let contents = read_to_string(&espforge_config).expect("file corrupt");
    let config: EspForgeConfig = toml::from_str(&contents).expect("toml deserial failed");
    let espforge_config = config.espforge;
    let examples: HashMap<String, Value> = config.example; 
    let examples_length = &examples.len();
    
    if *examples_length > 0 {
        let example_keys = examples.keys();
        let _ = &example_keys.for_each(|key| {
            println!("example: {:?}\n", key);
        });
    }
    
    return espforge_config;
    //let esp_config: EspConfig = config.espforge;
    //print!("{:?}\n", &esp_config);
}

pub fn parse_example() {

}