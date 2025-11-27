use assert_cmd::{cargo, prelude::*};
use predicates::prelude::*;
use rstest::rstest;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use support::{TestCandidate, VirtualEnv};

mod support;

fn cmd_with_env(sdkman_dir: &Path) -> Command {
    let mut cmd = Command::new(cargo::cargo_bin!("sdkman"));
    cmd.env("SDKMAN_DIR", sdkman_dir);
    // deterministic output
    cmd.env("NO_COLOR", "1");
    cmd.env("CLICOLOR", "0");
    cmd
}

fn current_scala_bin(sdkman_dir: &Path) -> PathBuf {
    sdkman_dir
        .join("candidates")
        .join("scala")
        .join("current")
        .join("bin")
        .join("scala")
}

fn current_scala_dir(sdkman_dir: &Path) -> PathBuf {
    sdkman_dir.join("candidates").join("scala").join("current")
}

fn remove_any_path(p: &Path) {
    // Handles: file, dir, symlink-to-file, symlink-to-dir (cross-platform best effort)
    if fs::remove_file(p).is_ok() {
        return;
    }
    let _ = fs::remove_dir_all(p);
}

#[rstest]
#[case(
    vec!["0.0.1", "0.0.2"],
    "0.0.1",
    "0.0.2",
    "Running scala 0.0.2"
)]
#[case(
    vec!["0.0.1"],
    "0.0.1",
    "0.0.1",
    "Running scala 0.0.1"
)]
fn should_set_an_installed_version_as_default(
    #[case] versions: Vec<&'static str>,
    #[case] initial_current: &'static str,
    #[case] set_to: &'static str,
    #[case] expected_script_contains: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidates: vec![TestCandidate {
            name: "scala",
            versions,
            current_version: initial_current,
        }],
    };

    let sdkman_dir = support::virtual_env(env);

    let expected_output = format!(
        "setting scala {} as the default version for all shells.",
        set_to
    );

    cmd_with_env(sdkman_dir.path())
        .arg("default")
        .arg("scala")
        .arg(set_to)
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains(expected_output));

    // current should exist (symlink-to-dir or real dir fallback)
    let current_dir = current_scala_dir(sdkman_dir.path());
    assert!(current_dir.exists());
    assert!(current_dir.is_dir());

    let content = fs::read_to_string(current_scala_bin(sdkman_dir.path()))?;
    assert!(
        content.contains(expected_script_contains),
        "expected current scala script to contain '{expected_script_contains}', got:\n{content}"
    );

    Ok(())
}

#[test]
fn should_not_set_an_uninstalled_version_as_default() -> Result<(), Box<dyn std::error::Error>> {
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidates: vec![TestCandidate {
            name: "scala",
            versions: vec!["0.0.1"],
            current_version: "0.0.1",
        }],
    };

    let sdkman_dir = support::virtual_env(env);

    cmd_with_env(sdkman_dir.path())
        .arg("default")
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .failure()
        .code(1)
        .stderr(
            predicate::str::contains("scala")
                .and(predicate::str::contains("0.0.2"))
                .and(predicate::str::contains("is not installed on your system")),
        );

    Ok(())
}

#[test]
fn should_error_for_invalid_candidate() -> Result<(), Box<dyn std::error::Error>> {
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidates: vec![TestCandidate {
            name: "scala",
            versions: vec!["0.0.1"],
            current_version: "0.0.1",
        }],
    };

    let sdkman_dir = support::virtual_env(env);

    cmd_with_env(sdkman_dir.path())
        .arg("default")
        .arg("notreal")
        .arg("0.0.1")
        .assert()
        .failure()
        .code(1)
        .stderr(
            predicate::str::contains("notreal")
                .and(predicate::str::contains("is not a valid candidate")),
        );

    Ok(())
}

#[test]
fn should_fail_when_candidates_file_missing() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();

    // Remove var/candidates to force the known_candidates error path.
    let candidates_file = sdkman_dir.path().join("var").join("candidates");
    remove_any_path(&candidates_file);

    cmd_with_env(sdkman_dir.path())
        .arg("default")
        .arg("scala")
        .arg("0.0.1")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("the candidates file is missing"));

    Ok(())
}

#[test]
fn should_replace_current_when_current_is_a_real_directory(
) -> Result<(), Box<dyn std::error::Error>> {
    // Start with a normal env, then force current/ to be a real directory.
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidates: vec![TestCandidate {
            name: "scala",
            versions: vec!["0.0.1", "0.0.2"],
            current_version: "0.0.1",
        }],
    };

    let sdkman_dir = support::virtual_env(env);
    let current_dir = current_scala_dir(sdkman_dir.path());

    // Remove any symlink and replace with a real directory + dummy file.
    remove_any_path(&current_dir);
    fs::create_dir_all(current_dir.join("bin"))?;
    fs::write(current_dir.join("bin").join("scala"), "Running scala OLD\n")?;

    cmd_with_env(sdkman_dir.path())
        .arg("default")
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .success()
        .code(0);

    let content = fs::read_to_string(current_scala_bin(sdkman_dir.path()))?;
    assert!(content.contains("Running scala 0.0.2"));

    Ok(())
}
