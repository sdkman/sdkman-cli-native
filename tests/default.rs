#[cfg(test)]
use assert_cmd::Command;
use predicates::str::contains;
use serial_test::serial;
use std::{env, fs};
use support::{TestCandidate, VirtualEnv};

mod support;

#[test]
#[serial]
fn should_set_an_installed_version_as_default() -> Result<(), Box<dyn std::error::Error>> {
    let candidate = TestCandidate {
        name: "scala",
        versions: vec!["0.0.1", "0.0.2"],
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
    let expected_output = "setting scala 0.0.2 as the default version for all shells";
    Command::cargo_bin("default")?
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .success()
        .stdout(contains(expected_output))
        .code(0);

    let file = sdkman_dir
        .path()
        .join("candidates")
        .join("scala")
        .join("current")
        .join("bin")
        .join("scala");
    let content = fs::read_to_string(file).unwrap();
    assert!(content.contains("Running scala 0.0.2"));

    Ok(())
}

#[test]
#[serial]
fn should_reset_the_current_default_version_as_default() -> Result<(), Box<dyn std::error::Error>> {
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
    let expected_output = "setting scala 0.0.1 as the default version for all shells";
    Command::cargo_bin("default")?
        .arg("scala")
        .arg("0.0.1")
        .assert()
        .success()
        .stdout(contains(expected_output))
        .code(0);

    let file = sdkman_dir
        .path()
        .join("candidates")
        .join("scala")
        .join("current")
        .join("bin")
        .join("scala");
    let content = fs::read_to_string(file).unwrap();
    assert!(content.contains("Running scala 0.0.1"));

    Ok(())
}

#[test]
#[serial]
fn should_not_set_an_uninstalled_version_as_default() -> Result<(), Box<dyn std::error::Error>> {
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
    let expected_output = "scala 0.0.2 is not installed on your system";
    Command::cargo_bin("default")?
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .failure()
        .stderr(contains(expected_output))
        .code(1);
    Ok(())
}
