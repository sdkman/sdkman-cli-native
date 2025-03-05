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
    let native_version = env!("CARGO_PKG_VERSION");

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
    let sdkman_dir = support::prepare_sdkman_dir();

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

    Command::cargo_bin("version")?.assert().failure().code(101);
    Ok(())
}

#[test]
#[serial]
fn should_include_os_and_arch_info() -> Result<(), Box<dyn std::error::Error>> {
    let cli_version = "5.0.0";
    let native_version = env!("CARGO_PKG_VERSION");

    let env = VirtualEnv {
        cli_version: cli_version.to_string(),
        native_version: native_version.to_string(),
        ..Default::default()
    };

    let sdkman_dir = support::virtual_env(env);
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    // Get the expected OS and architecture strings
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let contains_os = predicate::str::contains(format!("{}", os));
    let contains_arch = predicate::str::contains(format!("{}", arch));

    Command::cargo_bin("version")?
        .assert()
        .success()
        .stdout(contains_os.and(contains_arch))
        .code(0);

    Ok(())
}
