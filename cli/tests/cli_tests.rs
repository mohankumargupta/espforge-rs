//use anyhow::Error;
use assert_cmd::{pkg_name, prelude::*};
use espforge_test_macros::{cli_test};
//use std::path::PathBuf;
use std::process::Command;
use assert_fs::{assert::PathAssert, fixture::PathChild};



fn get_cli_command() -> Command {
    Command::new(pkg_name!())
}

/// Test for configuration file not found 
#[test]
fn test_invalid_config_file() {
    let mut cmd = get_cli_command();
    cmd
        .arg("compile")
        .arg("invalid.toml")
        .assert()
        .failure();    
}

#[cli_test("../examples/blink.toml")]
fn test_blink_compilation(output: Output) {
    output.assert_file("output.txt").contains("expected content");
}