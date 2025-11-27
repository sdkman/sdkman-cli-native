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

fn sdk_with_sdkman_dir(sdkman_dir: &Path) -> Command {
    let mut cmd = Command::new(cargo::cargo_bin!("sdkman"));
    cmd.env("SDKMAN_DIR", sdkman_dir);
    cmd.env("NO_COLOR", "1");
    cmd.env("CLICOLOR", "0");
    cmd
}

fn sdk_with_home_fallback(home_dir: &Path) -> Command {
    let mut cmd = Command::new(cargo::cargo_bin!("sdkman"));
    // Ensure we test fallback path resolution, not an inherited SDKMAN_DIR.
    cmd.env_remove("SDKMAN_DIR");

    // directories::UserDirs uses HOME on unix and USERPROFILE on windows.
    // Setting both makes this deterministic across platforms.
    cmd.env("HOME", home_dir);
    cmd.env("USERPROFILE", home_dir);

    cmd.env("NO_COLOR", "1");
    cmd.env("CLICOLOR", "0");
    cmd
}

fn version_dir(sdkman_dir: &Path, candidate: &str, version: &str) -> PathBuf {
    sdkman_dir.join("candidates").join(candidate).join(version)
}

fn candidates_file(sdkman_dir: &Path) -> PathBuf {
    sdkman_dir.join("var").join("candidates")
}

#[rstest]
#[case(vec!["0.0.1"], "0.0.1")]
#[case(vec!["0.0.1", "0.0.2"], "0.0.2")]
fn home_prints_path_for_installed_version(
    #[case] versions: Vec<&'static str>,
    #[case] query_version: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidates: vec![TestCandidate {
            name: "scala",
            versions,
            current_version: "0.0.1",
        }],
    };
    let sdkman_dir = support::virtual_env(env);

    let expected = version_dir(sdkman_dir.path(), "scala", query_version);

    sdk_with_sdkman_dir(sdkman_dir.path())
        .arg("home")
        .arg("scala")
        .arg(query_version)
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains(expected.display().to_string()));

    Ok(())
}

#[test]
fn home_fails_for_uninstalled_version() -> Result<(), Box<dyn std::error::Error>> {
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

    sdk_with_sdkman_dir(sdkman_dir.path())
        .arg("home")
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
fn home_fails_for_invalid_candidate_even_if_dirs_exist() -> Result<(), Box<dyn std::error::Error>> {
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

    // Create an on-disk directory for a candidate that is NOT in var/candidates.
    fs::create_dir_all(version_dir(sdkman_dir.path(), "notreal", "1.0.0"))?;

    sdk_with_sdkman_dir(sdkman_dir.path())
        .arg("home")
        .arg("notreal")
        .arg("1.0.0")
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
fn home_fails_when_candidates_file_missing() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();

    // Ensure var exists, but remove var/candidates.
    fs::create_dir_all(sdkman_dir.path().join("var"))?;
    let _ = fs::remove_file(candidates_file(sdkman_dir.path()));

    sdk_with_sdkman_dir(sdkman_dir.path())
        .arg("home")
        .arg("scala")
        .arg("0.0.1")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("the candidates file is missing"));

    Ok(())
}

#[test]
fn home_works_with_candidates_file_whitespace_and_commas() -> Result<(), Box<dyn std::error::Error>>
{
    let sdkman_dir = support::prepare_sdkman_dir();

    // Write a candidates file with extra commas/whitespace.
    support::write_file(
        sdkman_dir.path(),
        Path::new("var"),
        "candidates",
        " , scala ,  java, , ".to_string(),
    );

    // Create scala version directory
    fs::create_dir_all(version_dir(sdkman_dir.path(), "scala", "0.0.1"))?;

    let expected = version_dir(sdkman_dir.path(), "scala", "0.0.1");

    sdk_with_sdkman_dir(sdkman_dir.path())
        .arg("home")
        .arg("scala")
        .arg("0.0.1")
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains(expected.display().to_string()));

    Ok(())
}

#[test]
fn home_treats_version_path_that_is_a_file_as_not_installed(
) -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();
    support::write_file(
        sdkman_dir.path(),
        Path::new("var"),
        "candidates",
        "scala".to_string(),
    );

    // Create a FILE at candidates/scala/0.0.1 (not a directory)
    let version_path = version_dir(sdkman_dir.path(), "scala", "0.0.1");
    fs::create_dir_all(version_path.parent().unwrap())?;
    fs::write(&version_path, "not a dir")?;

    sdk_with_sdkman_dir(sdkman_dir.path())
        .arg("home")
        .arg("scala")
        .arg("0.0.1")
        .assert()
        .failure()
        .code(1)
        .stderr(
            predicate::str::contains("scala")
                .and(predicate::str::contains("0.0.1"))
                .and(predicate::str::contains("is not installed on your system")),
        );

    Ok(())
}

#[test]
fn home_works_via_fallback_sdkman_dir_when_sdkman_dir_env_not_set(
) -> Result<(), Box<dyn std::error::Error>> {
    // Build a fake HOME and put .sdkman under it, since fallback_sdkman_dir = HOME/.sdkman
    let home = tempfile::TempDir::new()?;
    let sdkman_root = home.path().join(".sdkman");

    // Minimal structure used by known_candidates + home:
    // .sdkman/var/candidates
    // .sdkman/candidates/scala/0.0.1
    fs::create_dir_all(sdkman_root.join("var"))?;
    fs::create_dir_all(sdkman_root.join("candidates").join("scala").join("0.0.1"))?;
    fs::write(sdkman_root.join("var").join("candidates"), "scala")?;

    let expected = sdkman_root.join("candidates").join("scala").join("0.0.1");

    sdk_with_home_fallback(home.path())
        .arg("home")
        .arg("scala")
        .arg("0.0.1")
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains(expected.display().to_string()));

    Ok(())
}

#[test]
fn home_fails_when_candidates_file_is_empty() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();

    // Empty candidates file => known_candidates() returns empty vec => validate_candidate exits(1)
    support::write_file(
        sdkman_dir.path(),
        Path::new("var"),
        "candidates",
        " , ,  ,".to_string(),
    );

    sdk_with_sdkman_dir(sdkman_dir.path())
        .arg("home")
        .arg("scala")
        .arg("0.0.1")
        .assert()
        .failure()
        .code(1)
        .stderr(
            predicate::str::contains("scala")
                .and(predicate::str::contains("not a valid candidate")),
        );

    Ok(())
}
