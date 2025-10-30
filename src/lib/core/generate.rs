use std::process::Command;

use crate::config::espforge::EspConfig;

pub fn generate(espforge_config: EspConfig) {
    let name = espforge_config.name;
    let chip = espforge_config.chip;
    
    Command::new("esp-generate")
        .arg("--headless")
        .arg("--chip")
        .arg(chip)
        .arg("-o")
        .arg("unstable-hal")
        .arg("-o")
        .arg("esp-backtrace")
        .arg("-o")
        .arg("wokwi")
        .arg("-o")
        .arg("vscode")
        .arg(name)
        .output()
        .expect("Something went wrong!");
}


