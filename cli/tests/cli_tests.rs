use assert_cmd::{pkg_name, prelude::*};
//use assert_cmd::cargo::cargo_bin;

use std::{env::current_dir, process::Command};

#[test]
fn test_compile_command_succeeds() {
    let mut cmd = Command::new(pkg_name!());
    cmd
        .arg("compile")
        .arg("../../examples/blink.toml")
        .current_dir(".");
    cmd.assert().success();
}

// #[test]
// fn test_show_help() {
//     let mut cmd = Command::new(pkg_name!());
//     cmd.current_dir("boo");
//     cmd.assert().failure();
// }
