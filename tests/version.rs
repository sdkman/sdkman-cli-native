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
    let cli_version = "5.0.0";
    let native_version = env!("CARGO_PKG_VERSION");

    let env = VirtualEnv {
        cli_version: cli_version.to_string(),
        native_version: native_version.to_string(),
        ..Default::default()
    };

    let sdkman_dir = support::virtual_env(env);

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    let expected = format!(
        "\nSDKMAN!\ncore:   {}\nnative: {} ({} {})\n\n",
        cli_version,
        native_version,
        std::env::consts::OS,
        std::env::consts::ARCH,
    );

    Command::new(assert_cmd::cargo::cargo_bin!("version"))
        .assert()
        .success()
        .stdout(expected)
        .stderr("")
        .code(0);

    Ok(())
}

#[test]
#[serial]
fn should_error_if_version_file_not_present() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::new(assert_cmd::cargo::cargo_bin!("version"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: cannot read"))
        .stderr(predicate::str::contains("var/version"))
        .stderr(predicate::str::contains("no such file or directory"))
        .stdout("")
        .code(1);
    Ok(())
}

#[test]
#[serial]
fn should_error_if_version_file_empty() -> Result<(), Box<dyn std::error::Error>> {
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

    Command::new(assert_cmd::cargo::cargo_bin!("version"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: cannot read"))
        .stderr(predicate::str::contains("var/version"))
        .stderr(predicate::str::contains("file is empty"))
        .stdout("")
        .code(1);
    Ok(())
}

#[test]
#[serial]
fn should_reject_extra_arguments() -> Result<(), Box<dyn std::error::Error>> {
    Command::new(assert_cmd::cargo::cargo_bin!("version"))
        .arg("extra")
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: unexpected argument extra"))
        .stderr(predicate::str::contains("hint: run sdk version --help"))
        .stdout("")
        .code(2);
    Ok(())
}

#[test]
#[serial]
fn should_reject_unknown_option() -> Result<(), Box<dyn std::error::Error>> {
    Command::new(assert_cmd::cargo::cargo_bin!("version"))
        .arg("--bogus")
        .assert()
        .failure()
        .stderr(predicate::str::starts_with("error: unknown option --bogus"))
        .stderr(predicate::str::contains("hint: run sdk version --help"))
        .stdout("")
        .code(2);
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

    let contains_os = predicate::str::contains(os.to_string());
    let contains_arch = predicate::str::contains(arch.to_string());

    Command::new(assert_cmd::cargo::cargo_bin!("version"))
        .assert()
        .success()
        .stdout(contains_os.and(contains_arch))
        .code(0);

    Ok(())
}
