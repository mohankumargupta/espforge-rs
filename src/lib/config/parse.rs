use std::{fs::read_to_string, path::PathBuf};



pub fn parse_config(espforge_config: &PathBuf) {
    let contents = read_to_string(&espforge_config).expect("file corrupt");
    print!("{}\n", contents);
}