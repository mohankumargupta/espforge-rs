use std::path::PathBuf;

use clap::{Parser, Subcommand};
use espforge_lib::compile_command;

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
                compile_command::compile(&file);
            }
            else {
                eprintln!("file {:?} does not exist", &file);
            }
        },
    }
}

