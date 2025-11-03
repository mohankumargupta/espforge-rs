
use assert_cmd::{pkg_name, prelude::*};
use espforge_test_macros::{cli_test};
use std::process::Command;

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
    output
        .assert_file("blink/src/main.rs")
        .assert(predicate::str::contains("delay.delay_millis(2500);"));
}