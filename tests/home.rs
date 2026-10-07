#[cfg(test)]
use assert_cmd::Command;
use predicates::prelude::PredicateBooleanExt;
use predicates::str::contains;
use serial_test::serial;
use std::env;
use support::{TestCandidate, VirtualEnv};

mod support;

#[test]
#[serial]
fn should_successfully_display_current_candidate_home() -> Result<(), Box<dyn std::error::Error>> {
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidates: vec![TestCandidate {
            name: "scala",
            versions: vec!["0.0.1"],
            current_version: "0.0.1",
        }],
        config: None,
    };

    let sdkman_dir = support::virtual_env(env);
    let dir_string = sdkman_dir.path().to_str().unwrap();

    env::set_var("SDKMAN_DIR", dir_string);
    let expected_output = format!("{}/candidates/scala/0.0.1\n", dir_string);
    Command::new(assert_cmd::cargo::cargo_bin!("home"))
        .arg("scala")
        .arg("0.0.1")
        .assert()
        .success()
        .stdout(expected_output)
        .stderr("")
        .code(0);

    Ok(())
}

#[test]
#[serial]
fn should_fail_if_candidate_home_is_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidates: vec![TestCandidate {
            name: "scala",
            versions: vec!["0.0.1"],
            current_version: "0.0.1",
        }],
        config: None,
    };

    let sdkman_dir = support::virtual_env(env);
    let dir_string = sdkman_dir.path().to_str().unwrap();

    env::set_var("SDKMAN_DIR", dir_string);
    Command::new(assert_cmd::cargo::cargo_bin!("home"))
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .failure()
        .stderr(contains("error: scala 0.0.2 is not installed"))
        .stderr(contains("hint: run sdk install scala 0.0.2"))
        .code(1);
    Ok(())
}

fn scala_env(config: Option<String>) -> tempfile::TempDir {
    support::virtual_env(VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidates: vec![TestCandidate {
            name: "scala",
            versions: vec!["0.0.1"],
            current_version: "0.0.1",
        }],
        config,
    })
}

#[test]
#[serial]
fn should_style_error_line_under_clicolor_force() {
    let sdkman_dir = scala_env(None);
    env::set_var("SDKMAN_DIR", sdkman_dir.path().to_str().unwrap());

    Command::new(assert_cmd::cargo::cargo_bin!("home"))
        .env("CLICOLOR_FORCE", "1")
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .failure()
        .stderr(contains("\u{1b}["))
        .stderr(contains("is not installed"))
        .code(1);
}

#[test]
#[serial]
fn should_keep_value_plain_under_clicolor_force() {
    let sdkman_dir = scala_env(None);
    let dir_string = sdkman_dir.path().to_str().unwrap();
    env::set_var("SDKMAN_DIR", dir_string);

    let expected_output = format!("{}/candidates/scala/0.0.1\n", dir_string);
    Command::new(assert_cmd::cargo::cargo_bin!("home"))
        .env("CLICOLOR_FORCE", "1")
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .arg("scala")
        .arg("0.0.1")
        .assert()
        .success()
        .stdout(expected_output)
        .stderr("")
        .code(0);
}

#[test]
#[serial]
fn should_keep_output_plain_under_no_color() {
    let sdkman_dir = scala_env(None);
    env::set_var("SDKMAN_DIR", sdkman_dir.path().to_str().unwrap());

    Command::new(assert_cmd::cargo::cargo_bin!("home"))
        .env("NO_COLOR", "1")
        .env("CLICOLOR_FORCE", "0")
        .env_remove("CLICOLOR")
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .failure()
        .stderr("error: scala 0.0.2 is not installed\n  hint: run sdk install scala 0.0.2\n")
        .stderr(contains("\u{1b}").not())
        .code(1);
}

#[test]
#[serial]
fn should_keep_output_plain_when_config_disables_colour() {
    let sdkman_dir = scala_env(Some("sdkman_colour_enable=false\n".to_string()));
    env::set_var("SDKMAN_DIR", sdkman_dir.path().to_str().unwrap());

    Command::new(assert_cmd::cargo::cargo_bin!("home"))
        .env_remove("CLICOLOR_FORCE")
        .env_remove("NO_COLOR")
        .env_remove("CLICOLOR")
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .failure()
        .stderr("error: scala 0.0.2 is not installed\n  hint: run sdk install scala 0.0.2\n")
        .stderr(contains("\u{1b}").not())
        .code(1);
}
