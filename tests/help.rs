use std::path::Path;
use std::process::Command;

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

fn sdk_help_stdout(command: &str) -> String {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("help"))
        .arg(command)
        .env("NO_COLOR", "1")
        .env_remove("CLICOLOR_FORCE")
        .env_remove("CLICOLOR")
        .output()
        .expect("failed to run help binary");
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
fn should_render_current_help_from_help_binary() {
    insta::assert_snapshot!(sdk_help_stdout("current"));
}

#[test]
fn should_render_default_help_from_help_binary() {
    insta::assert_snapshot!(sdk_help_stdout("default"));
}

#[test]
fn should_render_home_help_from_help_binary() {
    insta::assert_snapshot!(sdk_help_stdout("home"));
}

#[test]
fn should_render_uninstall_help_from_help_binary() {
    insta::assert_snapshot!(sdk_help_stdout("uninstall"));
}

#[test]
fn should_render_version_help_from_help_binary() {
    insta::assert_snapshot!(sdk_help_stdout("version"));
}

#[test]
fn sdk_help_current_matches_current_help() {
    assert_eq!(
        sdk_help_stdout("current"),
        help_stdout(assert_cmd::cargo::cargo_bin!("current"))
    );
}

#[test]
fn sdk_help_default_matches_default_help() {
    assert_eq!(
        sdk_help_stdout("default"),
        help_stdout(assert_cmd::cargo::cargo_bin!("default"))
    );
}

#[test]
fn sdk_help_home_matches_home_help() {
    assert_eq!(
        sdk_help_stdout("home"),
        help_stdout(assert_cmd::cargo::cargo_bin!("home"))
    );
}

#[test]
fn sdk_help_uninstall_matches_uninstall_help() {
    assert_eq!(
        sdk_help_stdout("uninstall"),
        help_stdout(assert_cmd::cargo::cargo_bin!("uninstall"))
    );
}

#[test]
fn sdk_help_version_matches_version_help() {
    assert_eq!(
        sdk_help_stdout("version"),
        help_stdout(assert_cmd::cargo::cargo_bin!("version"))
    );
}

#[test]
fn sdk_help_resolves_an_alias_to_its_command() {
    assert_eq!(
        sdk_help_stdout("rm"),
        help_stdout(assert_cmd::cargo::cargo_bin!("uninstall"))
    );
}
