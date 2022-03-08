extern crate clap;

use clap::Command;

use help_messages::{
    BROADCAST_HELP, CONFIG_HELP, CURRENT_HELP, DEFAULT_HELP, ENV_HELP, HOME_HELP, INSTALL_HELP,
    LIST_HELP, MAIN_HELP, UNINSTALL_HELP, UPGRADE_HELP, USE_HELP, VERSION_HELP,
};

fn main() {
    let args = Command::new("help")
        .override_help(MAIN_HELP)
        .subcommand(Command::new("broadcast").alias("b"))
        .subcommand(Command::new("config"))
        .subcommand(Command::new("current"))
        .subcommand(Command::new("default").alias("d"))
        .subcommand(Command::new("env").alias("e"))
        .subcommand(Command::new("home").alias("h"))
        .subcommand(Command::new("install").alias("i"))
        .subcommand(Command::new("list").alias("ls"))
        .subcommand(Command::new("uninstall").alias("rm"))
        .subcommand(Command::new("upgrade").alias("rm"))
        .subcommand(Command::new("use").alias("u"))
        .subcommand(Command::new("version").alias("v"))
        .get_matches();

    let help = match args.subcommand_name() {
        Some("broadcast") => BROADCAST_HELP,
        Some("config") => CONFIG_HELP,
        Some("current") => CURRENT_HELP,
        Some("default") => DEFAULT_HELP,
        Some("env") => ENV_HELP,
        Some("home") => HOME_HELP,
        Some("install") => INSTALL_HELP,
        Some("list") => LIST_HELP,
        Some("uninstall") => UNINSTALL_HELP,
        Some("upgrade") => UPGRADE_HELP,
        Some("use") => USE_HELP,
        Some("version") => VERSION_HELP,
        _ => MAIN_HELP,
    };

    println!("{}\n", help);
}
