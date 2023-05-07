#[cfg(test)]
use std::env;
use std::{path::Path, process::Command};

use assert_cmd::prelude::*;
use predicates::prelude::*;
use serial_test::serial;
use support::VirtualEnv;

mod support;

#[test]
#[serial]
fn should_successfully_render_version() -> Result<(), Box<dyn std::error::Error>> {
    let prefix = "SDKMAN!";
    let cli_version = "5.0.0";
    let native_version = "0.1.0";

    let header = format!("\n{}", prefix);
    let env = VirtualEnv {
        cli_version: cli_version.to_string(),
        native_version: native_version.to_string(),
        ..Default::default()
    };

    let sdkman_dir = support::virtual_env(env);

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    let contains_header = predicate::str::starts_with(header);
    let contains_version = predicate::str::contains(format!("script: {}", cli_version));
    let contains_native_version = predicate::str::contains(format!("native: {}", native_version));

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
    let var_path = Path::new("var");

    support::write_file(
        sdkman_dir.path(),
        var_path,
        "version_native",
        native_version,
    );

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?.assert().failure().code(101);
    Ok(())
}

#[test]
#[serial]
fn should_panic_if_native_version_file_not_present() -> Result<(), Box<dyn std::error::Error>> {
    let version = "5.0.0".to_string();

    let sdkman_dir = support::prepare_sdkman_dir();
    let var_path = Path::new("var");

    support::write_file(sdkman_dir.path(), var_path, "version", version);

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?.assert().failure().code(101);
    Ok(())
}

#[test]
#[serial]
fn should_panic_if_version_file_empty() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();
    let var_path = Path::new("var");

    support::write_file(sdkman_dir.path(), var_path, "version", "".to_string());

    support::write_file(
        sdkman_dir.path(),
        var_path,
        "version_native",
        "0.1.0".to_string(),
    );

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?.assert().failure().code(78);
    Ok(())
}

#[test]
#[serial]
fn should_panic_if_native_version_file_empty() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();
    let var_path = Path::new("var");

    support::write_file(sdkman_dir.path(), var_path, "version", "5.0.0".to_string());

    support::write_file(
        sdkman_dir.path(),
        var_path,
        "version_native",
        "".to_string(),
    );

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?.assert().failure().code(78);
    Ok(())
}
