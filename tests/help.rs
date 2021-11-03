#[cfg(test)]
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("help")?;

    cmd.assert().success().stdout(predicate::str::starts_with(
        "sdk - The command line interface (CLI) for SDKMAN!",
    ));
    Ok(())
}

#[test]
fn help_all() -> Result<(), Box<dyn std::error::Error>> {
    let args = [
        "install",
        "uninstall",
        "list",
        "use",
        "config",
        "default",
        "home",
    ];

    for arg in &args {
        let mut cmd = Command::cargo_bin("help")?;
        cmd.arg(arg);
        let header = format!("{} {}", "sdk", &arg);
        cmd.assert()
            .success()
            .stdout(predicate::str::starts_with(header));
    }

    Ok(())
}
