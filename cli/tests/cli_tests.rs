use assert_cmd::{pkg_name, prelude::*};
//use assert_cmd::cargo::cargo_bin;

use std::process::Command;

#[test]
fn test_compile_command_succeeds() {
    let mut cmd = Command::new(pkg_name!());
    cmd
        .arg("compile")
        .arg("../../examples/bare.toml");
    let output = cmd.unwrap();
    println!("{:?}", output.stdout);
}

