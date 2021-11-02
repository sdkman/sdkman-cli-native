#[cfg(test)]
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn home_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("help")?;

    cmd.arg("home");
    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with("sdk home"));
    Ok(())
}
