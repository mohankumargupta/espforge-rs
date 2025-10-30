use std::path::PathBuf;
use crate::{config::parse::parse_config, core::generate::generate};

pub fn compile(path: &PathBuf) {
    let espforge_config = parse_config(path);
    generate(espforge_config);
        
}
