use std::env;
use std::path::{Path};
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
    let native_version = "0.1.0";

    let header = format!("\n{}", prefix);

    let sdkman_dir = support::virtual_env(version.to_string(), native_version.to_string());

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    let contains_header = predicate::str::starts_with(header);
    let contains_version = predicate::str::contains(format!("cli version: {}", version));
    let contains_native_version = predicate::str::contains(format!("native extensions: {}", native_version));

    Command::cargo_bin("version")?
        .assert()
        .success()
        .stdout(contains_header.and(contains_version.and(contains_native_version)))
        .code(0);

    Ok(())
}

#[test]
#[serial]
fn should_panic_if_version_file_not_present() -> Result<(), Box<dyn std::error::Error>> {
    let native_version = "0.1.0".to_string();

    let sdkman_dir = support::prepare_sdkman_dir();
    support::init_var_dir(sdkman_dir.path());

    let native_version_file = Path::new("var/version_native");
    support::write_file(sdkman_dir.path(), native_version_file, native_version);

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?.assert().failure().code(78);
    Ok(())
}

#[test]
#[serial]
fn should_panic_if_native_version_file_not_present() -> Result<(), Box<dyn std::error::Error>> {
    let version = "5.0.0".to_string();

    let sdkman_dir = support::prepare_sdkman_dir();
    support::init_var_dir(sdkman_dir.path());

    let version_file = Path::new("var/version");
    support::write_file(sdkman_dir.path(), version_file, version);

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?.assert().failure().code(78);
    Ok(())
}

#[test]
#[serial]
fn should_panic_if_version_file_empty() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();
    support::init_var_dir(sdkman_dir.path());

    let version_file = Path::new("var/version");
    support::write_file(sdkman_dir.path(), version_file, "".to_string());

    let native_version_file = Path::new("var/version_native");
    support::write_file(sdkman_dir.path(), native_version_file, "0.1.0".to_string());

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?.assert().failure().code(78);
    Ok(())
}

#[test]
#[serial]
fn should_panic_if_native_version_file_empty() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();
    support::init_var_dir(sdkman_dir.path());

    let version_file = Path::new("var/version");
    support::write_file(sdkman_dir.path(), version_file, "5.0.0".to_string());

    let native_version_file = Path::new("var/version_native");
    support::write_file(sdkman_dir.path(), native_version_file, "".to_string());

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?.assert().failure().code(78);
    Ok(())
}
