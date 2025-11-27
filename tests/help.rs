use assert_cmd::{cargo, prelude::*};
use predicates::prelude::*;
use rstest::rstest;
use std::process::Command;

fn sdk() -> Command {
    Command::new(cargo::cargo_bin!("sdkman"))
}

#[test]
fn should_render_base_help() -> Result<(), Box<dyn std::error::Error>> {
    sdk()
        .arg("help")
        .assert()
        .success()
        .code(0)
        // Don't hardcode ANSI/wrapping; just validate structure.
        .stdout(
            predicate::str::contains("\nNAME\n")
                .and(predicate::str::contains(
                    "sdk - The command line interface (CLI) for SDKMAN!",
                ))
                .and(predicate::str::contains("\nSYNOPSIS\n"))
                .and(predicate::str::contains("\nDESCRIPTION\n"))
                .and(predicate::str::contains("\nEXAMPLES\n")),
        );

    Ok(())
}

#[rstest]
#[case("config")]
#[case("current")]
#[case("default")]
#[case("env")]
#[case("flush")]
#[case("home")]
#[case("install")]
#[case("list")]
#[case("selfupdate")]
#[case("uninstall")]
#[case("update")]
#[case("upgrade")]
#[case("use")]
#[case("version")]
fn should_render_help_for_subcommand(
    #[case] subcommand: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let expected_name_line = format!("sdk {}", subcommand);

    sdk()
        .arg("help")
        .arg(subcommand)
        .assert()
        .success()
        .code(0)
        .stdout(
            predicate::str::contains("\nNAME\n")
                .and(predicate::str::contains(&expected_name_line))
                .and(predicate::str::contains("\nSYNOPSIS\n"))
                .and(predicate::str::contains("\nDESCRIPTION\n"))
                .and(predicate::str::contains("\nEXAMPLES\n")),
        );

    Ok(())
}

#[rstest]
#[case("help", None)]
#[case("help", Some("version"))]
#[case("help", Some("install"))]
fn should_not_panic_for_help_paths(
    #[case] a: &str,
    #[case] b: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = sdk();
    cmd.arg(a);
    if let Some(b) = b {
        cmd.arg(b);
    }

    cmd.assert().success();
    Ok(())
}

#[test]
fn should_not_panic_on_clap_help_flag() -> Result<(), Box<dyn std::error::Error>> {
    // This ensures your clap wiring (disable_help_subcommand etc) is not exploding.
    sdk().arg("--help").assert().success().code(0);
    Ok(())
}
