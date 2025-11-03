




pub mod testing_macros {

use std::process::Command;
use assert_cmd::pkg_name;
use anyhow::{Error, Result};
use assert_cmd::prelude::*;
use predicates::prelude::*;
use assert_fs::{prelude::*, TempDir};
use assert_fs::fixture::PathChild;

fn get_cli_command() -> Command {
    Command::new(pkg_name!())
}

#[macro_export]
macro_rules! test_happy_path {
    ($test_name:ident, $name:expr, $config_path:expr, $expect_snippet:expr, $assert_error: expr) => {
        #[test]
        fn $test_name() -> Result<(), Error> {
use assert_cmd::prelude::*;
use predicates::prelude::*;
use assert_fs::{prelude::*, TempDir};
use assert_fs::fixture::PathChild;

            //Arrange
            let temp = assert_fs::TempDir::new()?;
            let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let configuration_path = manifest_dir.join($config_path);
            let temp_config = temp.child(format!("{}.toml", $name));
            fs::copy(configuration_path, temp_config.path())?;

            //Act
            let mut cmd = Command::new(pkg_name!());
            cmd.current_dir(temp.path());
            cmd.arg("compile").arg(temp_config.path());
            
            //Assert
            //Assert files are generated
            temp.child($name)
                .assert(predicate::path::exists())
                .assert(predicate::path::is_dir());
            // temp.child(format!("{}/src/main.rs", $name))
            //     .assert(predicate::path::exists())
            //     .assert(predicate::path::is_file());      

           // let content = fs::read_to_string(temp.child(format!("{}/src/main.rs", $name)).path())?;
            // assert!(
            //     content.contains(#expect_snippet),
            //     #assert_error
            // );      

            
            // let temp = TempDir::new()?;
            // let project_name = std::path::Path::new($config_path)
            //     .file_stem()
            //     .unwrap()
            //     .to_str()
            //     .unwrap();

            // let mut cmd = get_cli_command();
            // cmd.arg("compile")
            //     .arg($config_path)
            //     .current_dir(temp.path());

            // cmd.assert().success();

            // let main_rs_path = temp.child(project_name).child("src/main.rs");
            // main_rs_path.assert(predicate::path::is_file());
            // let main_rs_content = std::fs::read_to_string(main_rs_path.path())?;
            
            // The snapshot name is derived from the test function name
            //insta::assert_snapshot!(main_rs_content);

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! test_error_case {
    ($test_name:ident, $config_content:expr, $expected_error:expr) => {
        #[test]
        fn $test_name() -> Result<()> {
            let temp = TempDir::new()?;
            let config_file = temp.child("error.toml");
            config_file.write_str($config_content)?;

            let mut cmd = get_cli_command();
            cmd.arg("compile").arg(config_file.path());
            
            // cmd.assert()
            //     .failure()
            //     .stderr(predicate::str::contains($expected_error));

            Ok(())
        }
    };
}
}