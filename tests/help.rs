#[cfg(test)]
use std::path::Path;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

fn help_stdout(bin: &Path) -> String {
    let output = Command::new(bin)
        .arg("--help")
        .env("NO_COLOR", "1")
        .env_remove("CLICOLOR_FORCE")
        .env_remove("CLICOLOR")
        .output()
        .expect("failed to run native binary");
    String::from_utf8(output.stdout).expect("help output is not valid utf-8")
}

#[test]
fn should_render_current_help_page() {
    insta::assert_snapshot!(help_stdout(assert_cmd::cargo::cargo_bin!("current")));
}

#[test]
fn should_render_default_help_page() {
    insta::assert_snapshot!(help_stdout(assert_cmd::cargo::cargo_bin!("default")));
}

#[test]
fn should_render_home_help_page() {
    insta::assert_snapshot!(help_stdout(assert_cmd::cargo::cargo_bin!("home")));
}

#[test]
fn should_render_uninstall_help_page() {
    insta::assert_snapshot!(help_stdout(assert_cmd::cargo::cargo_bin!("uninstall")));
}

#[test]
fn should_render_version_help_page() {
    insta::assert_snapshot!(help_stdout(assert_cmd::cargo::cargo_bin!("version")));
}

#[test]
fn should_render_base_help() -> Result<(), Box<dyn std::error::Error>> {
    let header = "\nNAME\n    sdk - The command line interface (CLI) for SDKMAN!";
    Command::new(assert_cmd::cargo::cargo_bin!("help"))
        .assert()
        .success()
        .stdout(predicate::str::starts_with(header))
        .code(0);
    println!("Tested: {}", header);
    Ok(())
}

#[test]
fn should_render_help_for_all_subcommands() -> Result<(), Box<dyn std::error::Error>> {
    let args = [
        "config",
        "current",
        "default",
        "env",
        "flush",
        "home",
        "install",
        "list",
        "selfupdate",
        "uninstall",
        "update",
        "upgrade",
        "use",
        "version",
    ];

    for arg in &args {
        let header = format!("\n{} {} - ", "NAME\n    sdk", &arg);
        Command::new(assert_cmd::cargo::cargo_bin!("help"))
            .arg(arg)
            .assert()
            .success()
            .stdout(predicate::str::starts_with(&header))
            .code(0);
        println!("Success: sdk {}", arg);
    }
    Ok(())
}
