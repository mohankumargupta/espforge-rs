use assert_cmd::{pkg_name, prelude::*};
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;

#[test]
fn test_compile_command_succeeds() {
    let mut cmd = Command::new(pkg_name!());
    cmd
        .arg("compile")
        .arg("blink.toml sadaasd");
        
    cmd.assert().success();
}

// #[test]
// fn test_show_help() {
//     let mut cmd = Command::new(pkg_name!());
//     cmd.current_dir("boo");
//     cmd.assert().failure();
// }

#[test]
fn test_compile_creates_project_and_main_rs() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup a temporary directory for the test.
    // This creates a new directory with a random name in the system's temp folder.
    let temp = assert_fs::TempDir::new()?;

    // The path to our original blink.toml configuration.
    // `cargo test` runs from the workspace root, so this path is correct.
    let source_config_path = "examples/blink.toml";
    
    // The destination path for the config file inside our temporary directory.
    let temp_config = temp.child("blink.toml");

    // 2. Copy the example config into the temporary directory.
    fs::copy(source_config_path, temp_config.path())?;

    // 3. Prepare the command.
    let mut cmd = Command::new(pkg_name!());

    // CRITICAL: Set the current directory for the command to our temp directory.
    // This ensures that the `blink` project is generated inside our sandbox.
    cmd.current_dir(temp.path());

    // Tell the command to compile the config file we just copied.
    cmd.arg("compile").arg(temp_config.path());

    // 4. Run the command and assert that it exits successfully.
    cmd.assert().success();

    // 5. Verify the output.
    // Check that a directory named "blink" was created inside our temp dir.
    temp.child("blink")
        .assert(predicate::path::exists())
        .assert(predicate::path::is_dir());

    // Check that the "main.rs" file was created inside "blink/src".
    temp.child("blink/src/main.rs")
        .assert(predicate::path::exists())
        .assert(predicate::path::is_file());

    // The `temp` directory and all its contents will be automatically
    // cleaned up when the `temp` variable goes out of scope.
    Ok(())
}