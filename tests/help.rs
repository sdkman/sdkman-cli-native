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

fn sdk_help_main() -> String {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("help"))
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

#[test]
fn should_render_sdk_main_help_page() {
    insta::assert_snapshot!(sdk_help_main());
}

#[test]
fn should_render_install_help_page() {
    insta::assert_snapshot!(sdk_help_stdout("install"));
}

#[test]
fn should_render_list_help_page() {
    insta::assert_snapshot!(sdk_help_stdout("list"));
}

#[test]
fn should_render_use_help_page() {
    insta::assert_snapshot!(sdk_help_stdout("use"));
}

#[test]
fn should_render_env_help_page() {
    insta::assert_snapshot!(sdk_help_stdout("env"));
}

#[test]
fn should_render_upgrade_help_page() {
    insta::assert_snapshot!(sdk_help_stdout("upgrade"));
}

#[test]
fn should_render_update_help_page() {
    insta::assert_snapshot!(sdk_help_stdout("update"));
}

#[test]
fn should_render_selfupdate_help_page() {
    insta::assert_snapshot!(sdk_help_stdout("selfupdate"));
}

#[test]
fn should_render_flush_help_page() {
    insta::assert_snapshot!(sdk_help_stdout("flush"));
}

#[test]
fn should_render_config_help_page() {
    insta::assert_snapshot!(sdk_help_stdout("config"));
}

#[test]
fn should_render_help_help_page() {
    insta::assert_snapshot!(sdk_help_stdout("help"));
}

#[test]
fn should_report_unknown_command_with_suggestion() {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("help"))
        .arg("instal")
        .env("NO_COLOR", "1")
        .env_remove("CLICOLOR_FORCE")
        .env_remove("CLICOLOR")
        .output()
        .expect("failed to run help binary");
    let stderr = String::from_utf8(output.stderr).expect("stderr is not valid utf-8");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(
        stderr,
        "error: unknown command instal\n  hint: did you mean install?\n"
    );
}
