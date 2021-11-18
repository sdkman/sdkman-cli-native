extern crate clap;

use clap::{App, SubCommand};

use help_messages::CONFIG_HELP;
use help_messages::DEFAULT_HELP;
use help_messages::HOME_HELP;
use help_messages::INSTALL_HELP;
use help_messages::LIST_HELP;
use help_messages::MAIN_HELP;
use help_messages::UNINSTALL_HELP;
use help_messages::USE_HELP;

fn main() {
    let args = App::new("help")
        .help(MAIN_HELP)
        .subcommand(SubCommand::with_name("config"))
        .subcommand(SubCommand::with_name("default").alias("d"))
        .subcommand(SubCommand::with_name("home").alias("h"))
        .subcommand(SubCommand::with_name("install").alias("i"))
        .subcommand(SubCommand::with_name("list").alias("ls"))
        .subcommand(SubCommand::with_name("uninstall").alias("rm"))
        .subcommand(SubCommand::with_name("use").alias("u"))
        .get_matches();

    let help = match args.subcommand_name() {
        Some("config") => CONFIG_HELP,
        Some("default") => DEFAULT_HELP,
        Some("home") => HOME_HELP,
        Some("install") => INSTALL_HELP,
        Some("list") => LIST_HELP,
        Some("uninstall") => UNINSTALL_HELP,
        Some("use") => USE_HELP,
        _ => MAIN_HELP,
    };

    println!("{}\n", help);
}
