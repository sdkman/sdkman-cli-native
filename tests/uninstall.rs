#[cfg(test)]
use assert_cmd::Command;
use predicates::str::contains;
use serial_test::serial;
use std::env;
use support::{TestCandidate, VirtualEnv};

mod support;

#[test]
#[serial]
fn should_successfully_remove_unused_candidate_version() -> Result<(), Box<dyn std::error::Error>> {
    let candidate = TestCandidate {
        name: "scala",
        versions: vec!["0.0.1", "0.0.2"],
        current_version: "0.0.2",
    };
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidate: Some(candidate),
    };

    let sdkman_dir = support::virtual_env(env);
    let dir_string = sdkman_dir.path().to_str().unwrap();

    env::set_var("SDKMAN_DIR", dir_string);
    let expected_output = "removed scala 0.0.1";
    Command::cargo_bin("uninstall")?
        .arg("scala")
        .arg("0.0.1")
        .assert()
        .success()
        .stdout(contains(expected_output))
        .code(0);

    Ok(())
}

#[test]
#[serial]
fn should_fail_if_candidate_version_is_current() -> Result<(), Box<dyn std::error::Error>> {
    let candidate = TestCandidate {
        name: "scala",
        versions: vec!["0.0.1", "0.0.2"],
        current_version: "0.0.2",
    };

    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidate: Some(candidate),
    };

    let sdkman_dir = support::virtual_env(env);
    let dir_string = sdkman_dir.path().to_str().unwrap();

    env::set_var("SDKMAN_DIR", dir_string);
    let expected_output = format!("you are not permitted to delete the current version of scala.");
    Command::cargo_bin("uninstall")?
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .failure()
        .stderr(contains(expected_output))
        .code(1);
    Ok(())
}

#[test]
#[serial]
fn should_fail_if_candidate_is_invalid() -> Result<(), Box<dyn std::error::Error>> {
    let candidate = TestCandidate {
        name: "scala",
        versions: vec!["0.0.1"],
        current_version: "0.0.1",
    };
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidate: Some(candidate),
    };

    let sdkman_dir = support::virtual_env(env);
    let dir_string = sdkman_dir.path().to_str().unwrap();

    env::set_var("SDKMAN_DIR", dir_string);
    let expected_output = "zcala is not a valid candidate";
    Command::cargo_bin("uninstall")?
        .arg("zcala")
        .arg("0.0.2")
        .assert()
        .failure()
        .stderr(contains(expected_output))
        .code(1);
    Ok(())
}

#[test]
#[serial]
fn should_fail_if_candidate_version_is_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let candidate = TestCandidate {
        name: "scala",
        versions: vec!["0.0.1"],
        current_version: "0.0.1",
    };
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidate: Some(candidate),
    };

    let sdkman_dir = support::virtual_env(env);
    let dir_string = sdkman_dir.path().to_str().unwrap();

    env::set_var("SDKMAN_DIR", dir_string);
    let expected_output = format!("{} {} is not installed on your system", "scala", "0.0.2");
    Command::cargo_bin("uninstall")?
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .failure()
        .stderr(contains(expected_output))
        .code(1);
    Ok(())
}
