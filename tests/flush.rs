#[cfg(test)]
use std::env;
use std::fs;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use serial_test::serial;
use support::{write_file, VirtualEnv};

mod support;

fn create_tmp_archive(sdkman_dir: &tempfile::TempDir, name: &str) {
    write_file(
        sdkman_dir.path(),
        std::path::Path::new("tmp"),
        name,
        "fake archive content".to_string(),
    );
}

fn create_metadata_file(sdkman_dir: &tempfile::TempDir, name: &str) {
    write_file(
        sdkman_dir.path(),
        std::path::Path::new("var/metadata"),
        name,
        "fake metadata content".to_string(),
    );
}

#[test]
#[serial]
fn should_flush_tmp_and_metadata_by_default() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::virtual_env(VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![],
    });
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    create_tmp_archive(&sdkman_dir, "java-17.0.0-tem.zip");
    create_metadata_file(&sdkman_dir, "java-metadata.json");

    Command::new(assert_cmd::cargo::cargo_bin!("flush"))
        .assert()
        .success()
        .stdout(predicate::str::contains("archive(s) flushed"))
        .stdout(predicate::str::contains("tmp"))
        .stdout(predicate::str::contains("var/metadata"));

    assert!(sdkman_dir.path().join("tmp").exists());
    assert!(sdkman_dir.path().join("var/metadata").exists());
    assert!(fs::read_dir(sdkman_dir.path().join("tmp"))?.next().is_none());
    assert!(fs::read_dir(sdkman_dir.path().join("var/metadata"))?.next().is_none());

    Ok(())
}

#[test]
#[serial]
fn should_flush_tmp_only() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::virtual_env(VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![],
    });
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    create_tmp_archive(&sdkman_dir, "java-17.0.0-tem.zip");

    Command::new(assert_cmd::cargo::cargo_bin!("flush"))
        .arg("tmp")
        .assert()
        .success()
        .stdout(predicate::str::contains("archive(s) flushed"))
        .stdout(predicate::str::contains("tmp"));

    assert!(sdkman_dir.path().join("tmp").exists());
    assert!(fs::read_dir(sdkman_dir.path().join("tmp"))?.next().is_none());

    Ok(())
}

#[test]
#[serial]
fn should_flush_metadata_only() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::virtual_env(VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![],
    });
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    create_metadata_file(&sdkman_dir, "java-metadata.json");

    Command::new(assert_cmd::cargo::cargo_bin!("flush"))
        .arg("metadata")
        .assert()
        .success()
        .stdout(predicate::str::contains("archive(s) flushed"))
        .stdout(predicate::str::contains("var/metadata"));

    assert!(sdkman_dir.path().join("var/metadata").exists());
    assert!(fs::read_dir(sdkman_dir.path().join("var/metadata"))?.next().is_none());

    Ok(())
}

#[test]
#[serial]
fn should_flush_version_file() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::virtual_env(VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![],
    });
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    assert!(sdkman_dir.path().join("var/version").exists());

    Command::new(assert_cmd::cargo::cargo_bin!("flush"))
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("Version file has been flushed"));

    assert!(!sdkman_dir.path().join("var/version").exists());

    Ok(())
}

#[test]
#[serial]
fn should_handle_empty_tmp_dir() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::virtual_env(VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![],
    });
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::new(assert_cmd::cargo::cargo_bin!("flush"))
        .arg("tmp")
        .assert()
        .success()
        .stdout(predicate::str::contains("0 archive(s) flushed"));

    Ok(())
}
