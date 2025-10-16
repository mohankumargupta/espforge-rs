//use proc_macro2::TokenStream;
//use quote::quote;
use serde::{self, Deserialize, Serialize};
use std::{fs, process::Command};
use toml;


#[derive(Serialize, Deserialize, Debug)]
struct EspConfig {
    name: String,
    chip: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EspForgeConfig {
    espforge: EspConfig,
}


// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //println!("cargo:rerun-if-changed=main.toml");
    
    //println!("cargo:warning=Running custom build script!");
    // Example: Tell Cargo to link against a library
    // println!("cargo:rustc-link-lib=mylibrary");
    // Example: Regenerate if a specific file changes
    // println!("cargo:rerun-if-changed=src/schema.sql");

    let toml_content = fs::read_to_string("main.toml")?;
    let config: EspForgeConfig = toml::from_str(&toml_content)?;
    let esp_config: EspConfig = config.espforge;
    //println!("cargo:warning={:?}", config);
    generate(esp_config.name, esp_config.chip);
    Ok(())
}
//-o  -o  -o  -o vscode boo
fn generate(name: String, chip: String)  {
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
        .expect("Something went wrong!")
        ;


    //println!("cargo:warning={:?}", result);
   

}