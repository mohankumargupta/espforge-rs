use anyhow::Error;
use assert_cmd::{pkg_name, prelude::*};
use std::process::Command;
use assert_fs::fixture::PathChild;
use assert_fs::assert::PathAssert;
use predicates::prelude::*;
use std::fs;


#[test]
fn test_invalid_config_file() {
    let mut cmd = Command::new(pkg_name!());
    cmd
        .arg("compile")
        .arg("invalid.toml")
        .assert()
        .failure();
}

#[test]
fn test_blink() -> Result<(), Error> {
    let temp = assert_fs::TempDir::new()?;
    let source_config_path = "examples/blink.toml";
    let temp_config = temp.child("blink.toml");
    fs::copy(source_config_path, temp_config.path())?;

    let mut cmd = Command::new(pkg_name!());
    cmd.current_dir(temp.path());
    cmd.arg("compile").arg(temp_config.path());
    cmd.assert().success();

    temp.child("blink")
        .assert(predicate::path::exists())
        .assert(predicate::path::is_dir());
    temp.child("blink/src/main.rs")
        .assert(predicate::path::exists())
        .assert(predicate::path::is_file());

    Ok(())
}