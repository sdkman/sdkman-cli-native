#[cfg(test)]
use std::env;
use std::fs;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use serial_test::serial;
use support::{prepare_sdkman_dir, write_file, VirtualEnv};

mod support;

#[test]
#[serial]
fn should_pass_with_valid_cache() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::virtual_env(VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![support::TestCandidate {
            name: "java",
            versions: vec!["17.0.0-tem"],
            current_version: "17.0.0-tem",
        }],
    });
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::new(assert_cmd::cargo::cargo_bin!("cache"))
        .assert()
        .success();

    Ok(())
}

#[test]
#[serial]
fn should_fail_with_empty_cache() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::virtual_env(VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![],
    });
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    let cache_path = sdkman_dir.path().join("var/candidates");
    fs::write(&cache_path, "")?;

    Command::new(assert_cmd::cargo::cargo_bin!("cache"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cache is corrupt"))
        .stdout(predicate::str::contains("sdk update"));

    Ok(())
}

#[test]
#[serial]
fn should_fail_with_missing_cache() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = prepare_sdkman_dir();
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::new(assert_cmd::cargo::cargo_bin!("cache"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cache is corrupt"));

    Ok(())
}

#[test]
#[serial]
fn should_fail_with_whitespace_only_cache() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = prepare_sdkman_dir();
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    write_file(
        sdkman_dir.path(),
        std::path::Path::new("var"),
        "candidates",
        "  \n  ".to_string(),
    );

    Command::new(assert_cmd::cargo::cargo_bin!("cache"))
        .assert()
        .failure()
        .stderr(predicate::str::contains("Cache is corrupt"));

    Ok(())
}
