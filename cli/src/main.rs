use std::{fs::metadata, path::PathBuf};
use anyhow::{Context, Error, Result};
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

pub fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Compile { file } => {
            metadata(&file)
                .with_context(|| format!("Config file {} not found", &file.display()))?;
            if !file.is_file() {
                anyhow::bail!("Path {} is not a file", file.display());
            }
            compile_command::compile(&file);
            Ok(())
        },
    }
}

