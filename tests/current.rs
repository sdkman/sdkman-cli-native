#[cfg(test)]
use std::env;
use std::path::Path;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;
use serial_test::serial;
use support::{TestCandidate, VirtualEnv};

mod support;

#[test]
#[serial]
fn should_show_current_version_for_specific_candidate() -> Result<(), Box<dyn std::error::Error>> {
    let name = "java";
    let current_version = "11.0.15-tem";
    let versions = vec!["11.0.15-tem", "17.0.3-tem"];

    let env = VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![TestCandidate {
            name,
            versions: versions.clone(),
            current_version,
        }],
    };

    let sdkman_dir = support::virtual_env(env);
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    let expected_output = format!("Current default {} version {}", name, current_version);
    let contains_expected = predicate::str::contains(expected_output);

    Command::new(assert_cmd::cargo::cargo_bin!("current"))
        .arg(name)
        .assert()
        .success()
        .stdout(contains_expected)
        .code(0);

    Ok(())
}

#[test]
#[serial]
fn should_show_current_versions_for_all_candidates() -> Result<(), Box<dyn std::error::Error>> {
    // Define multiple candidates with their versions
    let java_name = "java";
    let java_current_version = "11.0.15-tem";
    let java_versions = vec!["11.0.15-tem", "17.0.3-tem"];

    let kotlin_name = "kotlin";
    let kotlin_current_version = "1.7.22";
    let kotlin_versions = vec!["1.6.21", "1.7.22"];

    let env = VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![
            TestCandidate {
                name: java_name,
                versions: java_versions.clone(),
                current_version: java_current_version,
            },
            TestCandidate {
                name: kotlin_name,
                versions: kotlin_versions.clone(),
                current_version: kotlin_current_version,
            },
        ],
    };

    let sdkman_dir = support::virtual_env(env);
    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    // Expected output patterns for the simple format (candidate version)
    let expected_java_output = format!("{} {}", java_name, java_current_version);
    let expected_kotlin_output = format!("{} {}", kotlin_name, kotlin_current_version);

    // Check for both expected outputs
    let contains_java_output = predicate::str::contains(expected_java_output);
    let contains_kotlin_output = predicate::str::contains(expected_kotlin_output);

    Command::new(assert_cmd::cargo::cargo_bin!("current"))
        .assert()
        .success()
        .stdout(contains_java_output.and(contains_kotlin_output))
        .code(0);

    Ok(())
}

#[test]
#[serial]
fn should_show_error_for_non_existent_candidate() -> Result<(), Box<dyn std::error::Error>> {
    let invalid_name = "invalid";

    // Create a simple environment with an empty candidates file
    let env = VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![],
    };

    let sdkman_dir = support::virtual_env(env);

    // Write at least one valid candidate to avoid empty candidates list error
    support::write_file(
        sdkman_dir.path(),
        Path::new("var"),
        "candidates",
        "java".to_string(),
    );

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    let contains_error = predicate::str::contains(invalid_name);

    Command::new(assert_cmd::cargo::cargo_bin!("current"))
        .arg(invalid_name)
        .assert()
        .failure()
        .stderr(contains_error)
        .code(1);

    Ok(())
}

#[test]
#[serial]
fn should_show_error_for_candidate_with_no_current_version(
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a candidate entry in candidates file, but no directory structure
    let sdkman_dir = support::prepare_sdkman_dir();

    // Write candidates file with a candidate
    let candidate_name = "kotlin";
    support::write_file(
        sdkman_dir.path(),
        Path::new("var"),
        "candidates",
        candidate_name.to_string(),
    );

    // Create candidate directory but no current symlink
    let candidate_dir = Path::new("candidates").join(candidate_name);
    std::fs::create_dir_all(sdkman_dir.path().join(&candidate_dir))
        .expect("Failed to create candidate directory");

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    let contains_error = predicate::str::contains("No current version of");

    Command::new(assert_cmd::cargo::cargo_bin!("current"))
        .arg(candidate_name)
        .assert()
        .failure()
        .stderr(contains_error)
        .code(1);

    Ok(())
}

#[test]
#[serial]
fn should_show_message_when_no_candidates_in_use() -> Result<(), Box<dyn std::error::Error>> {
    // Create empty candidates file, but ensure it has at least one character (e.g., "kotlin")
    // to avoid causing a panic in the known_candidates function
    let sdkman_dir = support::prepare_sdkman_dir();
    support::write_file(
        sdkman_dir.path(),
        Path::new("var"),
        "candidates",
        "kotlin".to_string(),
    );

    // Create candidates dir structure but without current symlinks
    std::fs::create_dir_all(sdkman_dir.path().join("candidates/kotlin"))
        .expect("Failed to create candidate directory");

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    let contains_message = predicate::str::contains("No candidates are in use");

    Command::new(assert_cmd::cargo::cargo_bin!("current"))
        .assert()
        .stderr(contains_message)
        .code(0);

    Ok(())
}
