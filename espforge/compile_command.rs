use std::{fs,path::{Path, PathBuf}};

use crate::{config::parse::parse_config, core::generate::generate, Example};

pub fn compile(path: &PathBuf) {
    // 1. Parse the configuration file
    match parse_config(path) {
        Ok(parsed_config) => {
            let project_name = parsed_config.esp_config.name.clone();

            // 2. Run esp-generate to create the project directory
            println!("Generating project '{}'...", project_name);
            generate(parsed_config.esp_config);
            println!("Project generation complete.");

            // 3. If an example is specified, render its template and overwrite main.rs
            if let Some(example) = parsed_config.example {
                println!("Applying example template...");
                match example.render() {
                    Ok(rendered_content) => {
                        let main_rs_path = Path::new(&project_name).join("src").join("main.rs");
                        println!("Writing template to {:?}", main_rs_path);

                        if let Err(e) = fs::write(&main_rs_path, rendered_content) {
                            eprintln!("Error: Failed to write to {:?}: {}", main_rs_path, e);
                        } else {
                            println!("Successfully updated src/main.rs!");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: Failed to render Askama template: {}", e);
                    }
                }
            }
        }
        Err(e) => eprintln!("Error: Failed to parse configuration file {:?}: {}", path, e),
    }

    
}
