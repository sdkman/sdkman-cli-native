extern crate clap;

use clap::App;

use help_messages::CONFIG_HELP;
use help_messages::DEFAULT_HELP;
use help_messages::ENV_HELP;
use help_messages::HOME_HELP;
use help_messages::INSTALL_HELP;
use help_messages::LIST_HELP;
use help_messages::MAIN_HELP;
use help_messages::UNINSTALL_HELP;
use help_messages::USE_HELP;

fn main() {
    let args = App::new("help")
        .override_help(MAIN_HELP)
        .subcommand(App::new("config"))
        .subcommand(App::new("default").alias("d"))
        .subcommand(App::new("env").alias("e"))
        .subcommand(App::new("home").alias("h"))
        .subcommand(App::new("install").alias("i"))
        .subcommand(App::new("list").alias("ls"))
        .subcommand(App::new("uninstall").alias("rm"))
        .subcommand(App::new("use").alias("u"))
        .get_matches();

    let help = match args.subcommand_name() {
        Some("config") => CONFIG_HELP,
        Some("default") => DEFAULT_HELP,
        Some("env") => ENV_HELP,
        Some("home") => HOME_HELP,
        Some("install") => INSTALL_HELP,
        Some("list") => LIST_HELP,
        Some("uninstall") => UNINSTALL_HELP,
        Some("use") => USE_HELP,
        _ => MAIN_HELP,
    };

    println!("{}\n", help);
}
