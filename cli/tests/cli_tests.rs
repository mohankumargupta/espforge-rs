use anyhow::Error;
use assert_cmd::{pkg_name, prelude::*};
use std::path::PathBuf;
use std::process::Command;
use assert_fs::fixture::PathChild;
use assert_fs::assert::PathAssert;
use predicates::prelude::*;
use std::fs;
mod test_macros;

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

/// Happy path, blink example
#[test]
fn test_blink() -> Result<(), Error> {
    test_happy_path!(blink, "../examples/blink.toml", 
                    content.contains("delay.delay_millis(2500);"),
                    "blink_rate_ms not inserted."            
    );
    // //Arrange
    // let temp = assert_fs::TempDir::new()?;
    // let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // let configuration_path = manifest_dir.join( "../examples/blink.toml");
    // let temp_config = temp.child("blink.toml");
    // fs::copy(configuration_path, temp_config.path())?;

    // //Act
    // let mut cmd = Command::new(pkg_name!());
    // cmd.current_dir(temp.path());
    // cmd.arg("compile").arg(temp_config.path());
    // cmd.assert().success();

    // //Assert
    // temp.child("blink")
    //     .assert(predicate::path::exists())
    //     .assert(predicate::path::is_dir());
    // temp.child("blink/src/main.rs")
    //     .assert(predicate::path::exists())
    //     .assert(predicate::path::is_file());

    // let content = fs::read_to_string(temp.child("blink/src/main.rs").path())?;
    // assert!(
    //     content.contains("delay.delay_millis(2500);"),
    //     "blink_rate_ms not inserted."
    // );

    Ok(())
}