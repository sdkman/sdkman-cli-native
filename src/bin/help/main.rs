extern crate clap;

use clap::Command;

use help_messages::CONFIG_HELP;
use help_messages::CURRENT_HELP;
use help_messages::DEFAULT_HELP;
use help_messages::ENV_HELP;
use help_messages::HOME_HELP;
use help_messages::INSTALL_HELP;
use help_messages::LIST_HELP;
use help_messages::MAIN_HELP;
use help_messages::UNINSTALL_HELP;
use help_messages::UPGRADE_HELP;
use help_messages::USE_HELP;

fn main() {
    let args = Command::new("help")
        .override_help(MAIN_HELP)
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
        .get_matches();

    let help = match args.subcommand_name() {
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
        _ => MAIN_HELP,
    };

    println!("{}\n", help);
}
