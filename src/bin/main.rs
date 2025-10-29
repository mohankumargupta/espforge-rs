use std::path::PathBuf;

use clap::{Parser, Subcommand};
use espforge_lib::config::parse::parse_config;

#[derive(Parser)]
#[command(about = "Example tool with a compile subcommand")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        file: PathBuf,
    } 
}


pub fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Compile { file } => {
            if file.is_file() {
                parse_config(&file);
            }
            else {
                eprintln!("file {:?} does not exist", &file);
            }
        },
    }
}

