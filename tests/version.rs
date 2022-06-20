use std::env;
#[cfg(test)]
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use serial_test::serial;

mod support;

#[test]
#[serial]
fn should_successfully_render_version() -> Result<(), Box<dyn std::error::Error>> {
    let prefix = "SDKMAN";
    let version = "5.0.0";

    let header = format!("\n{}", prefix);

    let sdkman_dir = support::virtual_env(version.to_string());

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?
        .assert()
        .success()
        .stdout(predicate::str::starts_with(header).and(predicate::str::contains(version)))
        .code(0);

    Ok(())
}

#[test]
#[serial]
fn should_panic_if_version_file_not_present() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?.assert().failure().code(78);
    Ok(())
}

#[test]
#[serial]
fn should_panic_if_version_file_empty() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();
    support::init_var_dir(sdkman_dir.path());
    support::write_version_file(sdkman_dir.path(), "".to_string());

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?.assert().failure().code(78);
    Ok(())
}
