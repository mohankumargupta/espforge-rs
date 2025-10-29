use std::path::PathBuf;

use crate::config::parse::parse_config;


pub fn compile(path: &PathBuf) {
    parse_config(path);
    
}