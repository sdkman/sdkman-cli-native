use assert_cmd::{assert::OutputAssertExt, cargo};
use predicates::prelude::PredicateBooleanExt;
use rstest::rstest;
use std::path::Path;
use std::process::Command;

use support::{TestCandidate, VirtualEnv};

mod support;

fn sdk_cmd(sdkman_dir: &Path) -> Result<Command, Box<dyn std::error::Error>> {
    let mut cmd = Command::new(cargo::cargo_bin!("sdkman"));
    cmd.env("SDKMAN_DIR", sdkman_dir);
    cmd.env("NO_COLOR", "1");
    cmd.env("CLICOLOR", "0");
    Ok(cmd)
}

#[rstest]
#[case("java", "11.0.15-tem", vec!["11.0.15-tem", "17.0.3-tem"])]
#[case("kotlin", "1.7.22", vec!["1.6.21", "1.7.22"])]
fn should_show_current_version_for_specific_candidate(
    #[case] name: &'static str,
    #[case] current_version: &'static str,
    #[case] versions: Vec<&'static str>,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let env = VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![TestCandidate {
            name,
            versions,
            current_version,
        }],
    };

    let sdkman_dir = support::virtual_env(env);
    let expected = format!("Using {} version {}", name, current_version);

    sdk_cmd(sdkman_dir.path())?
        .arg("current")
        .arg(name)
        .assert()
        .success()
        .stdout(predicates::str::contains(expected));

    Ok(())
}

#[test]
fn should_show_current_versions_for_all_candidates() -> Result<(), Box<dyn std::error::Error>> {
    let env = VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![
            TestCandidate {
                name: "java",
                versions: vec!["11.0.15-tem", "17.0.3-tem"],
                current_version: "11.0.15-tem",
            },
            TestCandidate {
                name: "kotlin",
                versions: vec!["1.6.21", "1.7.22"],
                current_version: "1.7.22",
            },
        ],
    };

    let sdkman_dir = support::virtual_env(env);

    sdk_cmd(sdkman_dir.path())?
        .arg("current")
        .assert()
        .success()
        .stdout(
            predicates::str::contains("Current versions in use:")
                .and(predicates::str::contains("java 11.0.15-tem"))
                .and(predicates::str::contains("kotlin 1.7.22")),
        );

    Ok(())
}

#[rstest]
#[case("invalid")]
#[case("not-a-candidate")]
fn should_error_for_non_existent_candidate(
    #[case] invalid_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let env = VirtualEnv {
        cli_version: "5.0.0".to_string(),
        native_version: "0.1.0".to_string(),
        candidates: vec![],
    };

    let sdkman_dir = support::virtual_env(env);

    // Ensure candidates file is non-empty and valid.
    support::write_file(
        sdkman_dir.path(),
        Path::new("var"),
        "candidates",
        "java".to_string(),
    );

    sdk_cmd(sdkman_dir.path())?
        .arg("current")
        .arg(invalid_name)
        .assert()
        .failure()
        .code(1)
        .stderr(predicates::str::contains(invalid_name));

    Ok(())
}

#[test]
fn should_error_for_candidate_with_no_current_version() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();

    let candidate_name = "kotlin";
    support::write_file(
        sdkman_dir.path(),
        Path::new("var"),
        "candidates",
        candidate_name.to_string(),
    );

    // Candidate dir exists, but no `current` link/dir
    std::fs::create_dir_all(sdkman_dir.path().join("candidates").join(candidate_name))?;

    sdk_cmd(sdkman_dir.path())?
        .arg("current")
        .arg(candidate_name)
        .assert()
        .failure()
        .code(1)
        .stderr(predicates::str::contains("No current version of"));

    Ok(())
}

#[test]
fn should_show_message_when_no_candidates_in_use() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();

    // known_candidates() needs at least one candidate entry
    support::write_file(
        sdkman_dir.path(),
        Path::new("var"),
        "candidates",
        "kotlin".to_string(),
    );

    // Candidate exists but no current link set
    std::fs::create_dir_all(sdkman_dir.path().join("candidates/kotlin"))?;

    sdk_cmd(sdkman_dir.path())?
        .arg("current")
        .assert()
        .success()
        .code(0)
        .stderr(predicates::str::contains("No candidates are in use."));

    Ok(())
}

#[test]
fn should_fail_when_candidates_file_missing() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();

    // Ensure var exists but candidates file is missing
    let var_dir = sdkman_dir.path().join("var");
    std::fs::create_dir_all(&var_dir)?;
    let candidates_path = var_dir.join("candidates");
    let _ = std::fs::remove_file(candidates_path);

    sdk_cmd(sdkman_dir.path())?
        .arg("current")
        .assert()
        .failure()
        .code(1)
        .stderr(predicates::str::contains("candidates"));

    Ok(())
}
