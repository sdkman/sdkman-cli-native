use assert_cmd::{cargo, prelude::*};
use predicates::prelude::*;
use rstest::rstest;
use std::{path::Path, process::Command};

use support::VirtualEnv;

mod support;

fn sdk(sdkman_dir: &std::path::Path) -> Command {
    let mut cmd = Command::new(cargo::cargo_bin!("sdkman"));
    cmd.env("SDKMAN_DIR", sdkman_dir);
    cmd.env("NO_COLOR", "1");
    cmd.env("CLICOLOR", "0");
    cmd
}

#[test]
fn should_successfully_render_version() -> Result<(), Box<dyn std::error::Error>> {
    let prefix = "SDKMAN!";
    let cli_version = "5.0.0";
    let native_version = env!("CARGO_PKG_VERSION");

    let env = VirtualEnv {
        cli_version: cli_version.to_string(),
        native_version: native_version.to_string(),
        ..Default::default()
    };

    let sdkman_dir = support::virtual_env(env);

    let contains_header = predicate::str::contains(format!("\n{}", prefix));
    let contains_cli = predicate::str::contains(format!("script: {}", cli_version));
    let contains_native = predicate::str::contains(format!("native: {}", native_version));

    sdk(sdkman_dir.path())
        .arg("version")
        .assert()
        .success()
        .code(0)
        .stdout(contains_header.and(contains_cli).and(contains_native));

    Ok(())
}

#[rstest]
#[case(true, false, "Not a valid file path")] // missing version file
#[case(false, true, "File is empty")] // empty version file
fn should_fail_if_version_file_missing_or_empty(
    #[case] missing_version_file: bool,
    #[case] empty_version_file: bool,
    #[case] expected_reason: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let sdkman_dir = support::prepare_sdkman_dir();
    let var_path = Path::new("var");

    if missing_version_file {
        let _ = std::fs::remove_file(sdkman_dir.path().join("var").join("version"));
    }

    if empty_version_file {
        support::write_file(sdkman_dir.path(), var_path, "version", "".to_string());
    }

    // Keep this present so we isolate failures to the script version path.
    support::write_file(
        sdkman_dir.path(),
        var_path,
        "version_native",
        "0.1.0".to_string(),
    );

    sdk(sdkman_dir.path())
        .arg("version")
        .assert()
        .failure()
        .code(1)
        .stderr(
            predicate::str::contains("failed to read SDKMAN! script version")
                // match "/var/version" on unix OR "\var\version" on windows
                .and(predicate::str::is_match(r"[/\\]var[/\\]version").unwrap())
                .and(predicate::str::contains(expected_reason)),
        );

    Ok(())
}

#[test]
fn should_include_os_and_arch_info() -> Result<(), Box<dyn std::error::Error>> {
    let cli_version = "5.0.0";
    let native_version = env!("CARGO_PKG_VERSION");

    let env = VirtualEnv {
        cli_version: cli_version.to_string(),
        native_version: native_version.to_string(),
        ..Default::default()
    };

    let sdkman_dir = support::virtual_env(env);

    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    sdk(sdkman_dir.path())
        .arg("version")
        .assert()
        .success()
        .code(0)
        .stdout(predicate::str::contains(os).and(predicate::str::contains(arch)));

    Ok(())
}

#[test]
fn should_not_panic_when_sdkman_dir_missing_defaults_to_failure(
) -> Result<(), Box<dyn std::error::Error>> {
    // If SDKMAN_DIR points somewhere invalid, the command should fail.
    // (Exact code depends on your impl; this asserts "non-zero".)
    let mut cmd = Command::new(cargo::cargo_bin!("sdkman"));
    cmd.env("SDKMAN_DIR", "__definitely_missing_sdkman_dir__");
    cmd.env("NO_COLOR", "1");

    cmd.arg("version").assert().failure();

    Ok(())
}
