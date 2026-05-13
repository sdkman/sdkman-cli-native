#[cfg(test)]
use std::env;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use serial_test::serial;
use support::{VirtualEnv, write_file};

mod support;

fn setup_with_config() -> tempfile::TempDir {
    let sdkman_dir = support::virtual_env(VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![],
    });
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    // Create the config file that the editor will open
    write_file(
        sdkman_dir.path(),
        std::path::Path::new("etc"),
        "config",
        "sdkman_auto_answer=false\n".to_string(),
    );

    sdkman_dir
}

#[test]
#[serial]
fn should_fail_when_editor_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let _sdkman_dir = setup_with_config();
    env::set_var("EDITOR", "nonexistent_editor_xyz");

    Command::new(assert_cmd::cargo::cargo_bin!("config"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));

    Ok(())
}

#[test]
#[serial]
fn should_open_config_with_cat_as_editor() -> Result<(), Box<dyn std::error::Error>> {
    let _sdkman_dir = setup_with_config();
    env::set_var("EDITOR", "cat");

    let assert = Command::new(assert_cmd::cargo::cargo_bin!("config"))
        .assert()
        .success();

    // cat prints the config file contents to stdout
    assert.stdout(predicate::str::contains("sdkman_auto_answer=false"));

    Ok(())
}
