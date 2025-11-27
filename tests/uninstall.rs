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

fn sdk(sdkman_dir: &Path) -> Command {
    let mut cmd = Command::new(cargo::cargo_bin!("sdkman"));
    cmd.env("SDKMAN_DIR", sdkman_dir);
    cmd.env("NO_COLOR", "1");
    cmd.env("CLICOLOR", "0");
    cmd
}

fn version_path(sdkman_dir: &Path, candidate: &str, version: &str) -> PathBuf {
    sdkman_dir.join("candidates").join(candidate).join(version)
}

fn current_path(sdkman_dir: &Path, candidate: &str) -> PathBuf {
    sdkman_dir
        .join("candidates")
        .join(candidate)
        .join("current")
}

#[rstest]
#[case("0.0.1", "0.0.2", false, 0, "removed scala 0.0.1.")]
#[case("0.0.2", "0.0.2", true, 0, "removed scala 0.0.2.")]
fn uninstall_removes_version_when_allowed(
    #[case] target_version: &str,
    #[case] current_version: &'static str,
    #[case] force: bool,
    #[case] expected_code: i32,
    #[case] expected_stdout: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidates: vec![TestCandidate {
            name: "scala",
            versions: vec!["0.0.1", "0.0.2"],
            current_version,
        }],
    };

    let sdkman_dir = support::virtual_env(env);

    let mut cmd = sdk(sdkman_dir.path());
    cmd.arg("uninstall").arg("scala").arg(target_version);
    if force {
        cmd.arg("--force");
    }

    cmd.assert()
        .success()
        .code(expected_code)
        .stdout(predicate::str::contains(expected_stdout));

    // Version directory should be gone.
    assert!(
        !version_path(sdkman_dir.path(), "scala", target_version).exists(),
        "expected version dir to be removed"
    );

    // If we removed the current version with --force, current should be gone too.
    if force && target_version == current_version {
        assert!(
            !current_path(sdkman_dir.path(), "scala").exists(),
            "expected current link/dir to be removed when forcing uninstall of current"
        );
    }

    Ok(())
}

#[test]
fn uninstall_fails_when_target_is_current_without_force() -> Result<(), Box<dyn std::error::Error>>
{
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidates: vec![TestCandidate {
            name: "scala",
            versions: vec!["0.0.1", "0.0.2"],
            current_version: "0.0.2",
        }],
    };

    let sdkman_dir = support::virtual_env(env);

    sdk(sdkman_dir.path())
        .arg("uninstall")
        .arg("scala")
        .arg("0.0.2")
        .assert()
        .failure()
        .code(1)
        .stderr(
            predicate::str::contains("scala")
                .and(predicate::str::contains("0.0.2"))
                .and(predicate::str::contains("is the"))
                .and(predicate::str::contains("current"))
                .and(predicate::str::contains("should not be removed")),
        );

    // Ensure version still exists.
    assert!(version_path(sdkman_dir.path(), "scala", "0.0.2").exists());

    Ok(())
}

#[rstest]
#[case("zcala", "0.0.2", "is not a valid candidate", false)]
#[case("scala", "0.0.9", "is not installed on your system", true)]
fn uninstall_fails_for_invalid_inputs(
    #[case] candidate: &str,
    #[case] version: &str,
    #[case] expected_msg: &str,
    #[case] expect_version_in_stderr: bool,
) -> Result<(), Box<dyn std::error::Error>> {
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

    let mut cmd = sdk(sdkman_dir.path());
    cmd.arg("uninstall").arg(candidate).arg(version);

    if expect_version_in_stderr {
        cmd.assert().failure().code(1).stderr(
            predicate::str::contains(candidate)
                .and(predicate::str::contains(version))
                .and(predicate::str::contains(expected_msg)),
        );
    } else {
        cmd.assert().failure().code(1).stderr(
            predicate::str::contains(candidate).and(predicate::str::contains(expected_msg)),
        );
    }

    Ok(())
}

#[test]
fn uninstall_fails_when_candidates_file_missing() -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();

    // Ensure var exists but candidates file is missing.
    fs::create_dir_all(sdkman_dir.path().join("var"))?;
    let _ = fs::remove_file(sdkman_dir.path().join("var").join("candidates"));

    sdk(sdkman_dir.path())
        .arg("uninstall")
        .arg("scala")
        .arg("0.0.1")
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("the candidates file is missing"));

    Ok(())
}
