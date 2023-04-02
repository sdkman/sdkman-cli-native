#[cfg(test)]
use assert_cmd::Command;
use predicates::str::contains;
use serial_test::serial;
use std::env;
use support::{TestCandidate, VirtualEnv};

mod support;

#[test]
#[serial]
fn should_successfully_display_current_candidate_home() -> Result<(), Box<dyn std::error::Error>> {
    let candidate = TestCandidate {
        name: "scala".to_string(),
        version: "0.0.1".to_string(),
    };
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidate: Some(candidate),
        known_candidates: vec!["scala"],
    };

    let sdkman_dir = support::virtual_env(env);
    let dir_string = sdkman_dir.path().to_str().unwrap();

    env::set_var("SDKMAN_DIR", dir_string);
    let expected_output = format!("{}/candidates/scala/0.0.1", dir_string);
    Command::cargo_bin("home")?
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
fn should_fail_if_candidate_home_is_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidate: None,
        known_candidates: vec!["scala"],
    };

    let sdkman_dir = support::virtual_env(env);
    let dir_string = sdkman_dir.path().to_str().unwrap();

    env::set_var("SDKMAN_DIR", dir_string);
    let expected_output = format!("{} {} is not installed on your system", "scala", "0.0.2");
    Command::cargo_bin("home")?
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .failure()
        .stderr(contains(expected_output))
        .code(1);
    Ok(())
}
