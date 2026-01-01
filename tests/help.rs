#[cfg(test)]
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

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
