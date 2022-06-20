use std::env;
#[cfg(test)]
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

mod support;

#[test]
fn should_successfully_render_version() -> Result<(), Box<dyn std::error::Error>> {
    let prefix = "SDKMAN";
    let version = "5.0.0";

    let header = format!("\n{}", prefix);

    let sdkman_dir = support::virtual_env(version.to_string()).unwrap();

    env::set_var("SDKMAN_DIR", sdkman_dir.path().as_os_str());

    Command::cargo_bin("version")?
        .assert()
        .success()
        .stdout(predicate::str::starts_with(header).and(predicate::str::contains(version)))
        .code(0);
    Ok(())
}
