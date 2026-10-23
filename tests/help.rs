use std::path::Path;
use std::process::{Command, Output};

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

fn sdk_help_flag_stdout(command: &str, flag: &str) -> String {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("help"))
        .arg(command)
        .arg(flag)
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

fn help_output(args: &[&str]) -> Output {
    Command::new(assert_cmd::cargo::cargo_bin!("help"))
        .args(args)
        .env("NO_COLOR", "1")
        .env_remove("CLICOLOR_FORCE")
        .env_remove("CLICOLOR")
        .output()
        .expect("failed to run help binary")
}

fn assert_page(args: &[&str], snapshot_name: &str) {
    let output = help_output(args);
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).expect("help output is not valid utf-8");
    insta::assert_snapshot!(snapshot_name, stdout);
}

#[test]
fn help_without_arguments_prints_the_full_main_page() {
    assert_page(&[], "should_render_sdk_main_help_page");
}

#[test]
fn help_long_flag_prints_the_full_main_page() {
    assert_page(&["--help"], "should_render_sdk_main_help_page");
}

#[test]
fn help_short_flag_prints_the_short_main_page() {
    assert_page(&["-h"], "should_render_sdk_main_short_help_page");
}

fn help_stdout_of(args: &[&str]) -> String {
    String::from_utf8(help_output(args).stdout).expect("help output is not valid utf-8")
}

#[test]
fn help_command_prints_the_full_page() {
    assert_page(&["install"], "should_render_install_help_page");
}

#[test]
fn help_command_long_flag_prints_the_full_page() {
    assert_page(&["install", "--help"], "should_render_install_help_page");
}

#[test]
fn help_command_short_flag_prints_the_short_page() {
    assert_page(&["install", "-h"], "should_render_install_short_help_page");
}

#[test]
fn help_flag_before_command_matches_flag_after_command() {
    assert_eq!(
        help_stdout_of(&["-h", "install"]),
        help_stdout_of(&["install", "-h"])
    );
}

#[test]
fn help_ignores_arguments_after_the_command() {
    assert_eq!(
        help_stdout_of(&["env", "init", "-h"]),
        help_stdout_of(&["env", "-h"])
    );
}

#[test]
fn help_last_flag_wins() {
    assert_eq!(
        help_stdout_of(&["install", "-h", "--help"]),
        help_stdout_of(&["install"])
    );
    assert_eq!(
        help_stdout_of(&["install", "--help", "-h"]),
        help_stdout_of(&["install", "-h"])
    );
}

#[test]
fn should_render_sdk_main_short_help_page() {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("help"))
        .arg("-h")
        .env("NO_COLOR", "1")
        .env_remove("CLICOLOR_FORCE")
        .env_remove("CLICOLOR")
        .output()
        .expect("failed to run help binary");
    let stdout = String::from_utf8(output.stdout).expect("help output is not valid utf-8");
    insta::assert_snapshot!(stdout);
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

const INSTAL_ERROR: &str = "error: unknown command instal\n  hint: did you mean install?\n";

#[test]
fn should_report_unknown_command_with_suggestion() {
    let output = help_output(&["instal"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(String::from_utf8(output.stderr).unwrap(), INSTAL_ERROR);
}

#[test]
fn should_report_unknown_command_with_short_flag() {
    let output = help_output(&["instal", "-h"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(String::from_utf8(output.stderr).unwrap(), INSTAL_ERROR);
}

#[test]
fn should_report_unknown_flag_as_unknown_command() {
    let output = help_output(&["--foo"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr).unwrap(),
        "error: unknown command --foo\n  hint: run sdk help to see all commands\n"
    );
}

#[test]
fn should_render_install_short_help_page() {
    insta::assert_snapshot!(sdk_help_flag_stdout("install", "-h"));
}

#[test]
fn should_render_list_short_help_page() {
    insta::assert_snapshot!(sdk_help_flag_stdout("list", "-h"));
}

#[test]
fn should_render_use_short_help_page() {
    insta::assert_snapshot!(sdk_help_flag_stdout("use", "-h"));
}

#[test]
fn should_render_env_short_help_page() {
    insta::assert_snapshot!(sdk_help_flag_stdout("env", "-h"));
}

#[test]
fn should_render_upgrade_short_help_page() {
    insta::assert_snapshot!(sdk_help_flag_stdout("upgrade", "-h"));
}

#[test]
fn should_render_update_short_help_page() {
    insta::assert_snapshot!(sdk_help_flag_stdout("update", "-h"));
}

#[test]
fn should_render_selfupdate_short_help_page() {
    insta::assert_snapshot!(sdk_help_flag_stdout("selfupdate", "-h"));
}

#[test]
fn should_render_flush_short_help_page() {
    insta::assert_snapshot!(sdk_help_flag_stdout("flush", "-h"));
}

#[test]
fn should_render_config_short_help_page() {
    insta::assert_snapshot!(sdk_help_flag_stdout("config", "-h"));
}

#[test]
fn should_render_help_short_help_page() {
    insta::assert_snapshot!(sdk_help_flag_stdout("help", "-h"));
}

fn native_stdout(bin: &Path, flag: &str) -> String {
    let output = Command::new(bin)
        .arg(flag)
        .env("NO_COLOR", "1")
        .env_remove("CLICOLOR_FORCE")
        .env_remove("CLICOLOR")
        .output()
        .expect("failed to run native binary");
    assert!(!output.stdout.is_empty(), "native help printed nothing");
    String::from_utf8(output.stdout).expect("help output is not valid utf-8")
}

#[test]
fn sdk_help_current_short_matches_current_short_help() {
    assert_eq!(
        sdk_help_flag_stdout("current", "-h"),
        native_stdout(assert_cmd::cargo::cargo_bin!("current"), "-h")
    );
}

#[test]
fn sdk_help_current_long_matches_current_long_help() {
    assert_eq!(
        sdk_help_flag_stdout("current", "--help"),
        native_stdout(assert_cmd::cargo::cargo_bin!("current"), "--help")
    );
}

#[test]
fn sdk_help_default_short_matches_default_short_help() {
    assert_eq!(
        sdk_help_flag_stdout("default", "-h"),
        native_stdout(assert_cmd::cargo::cargo_bin!("default"), "-h")
    );
}

#[test]
fn sdk_help_default_long_matches_default_long_help() {
    assert_eq!(
        sdk_help_flag_stdout("default", "--help"),
        native_stdout(assert_cmd::cargo::cargo_bin!("default"), "--help")
    );
}

#[test]
fn sdk_help_home_short_matches_home_short_help() {
    assert_eq!(
        sdk_help_flag_stdout("home", "-h"),
        native_stdout(assert_cmd::cargo::cargo_bin!("home"), "-h")
    );
}

#[test]
fn sdk_help_home_long_matches_home_long_help() {
    assert_eq!(
        sdk_help_flag_stdout("home", "--help"),
        native_stdout(assert_cmd::cargo::cargo_bin!("home"), "--help")
    );
}

#[test]
fn sdk_help_uninstall_short_matches_uninstall_short_help() {
    assert_eq!(
        sdk_help_flag_stdout("uninstall", "-h"),
        native_stdout(assert_cmd::cargo::cargo_bin!("uninstall"), "-h")
    );
}

#[test]
fn sdk_help_uninstall_long_matches_uninstall_long_help() {
    assert_eq!(
        sdk_help_flag_stdout("uninstall", "--help"),
        native_stdout(assert_cmd::cargo::cargo_bin!("uninstall"), "--help")
    );
}

#[test]
fn sdk_help_version_short_matches_version_short_help() {
    assert_eq!(
        sdk_help_flag_stdout("version", "-h"),
        native_stdout(assert_cmd::cargo::cargo_bin!("version"), "-h")
    );
}

#[test]
fn sdk_help_version_long_matches_version_long_help() {
    assert_eq!(
        sdk_help_flag_stdout("version", "--help"),
        native_stdout(assert_cmd::cargo::cargo_bin!("version"), "--help")
    );
}

#[test]
fn sdk_help_l_matches_sdk_help_list() {
    let list_page = sdk_help_stdout("list");
    assert!(!list_page.is_empty());
    assert_eq!(sdk_help_stdout("l"), list_page);
}
