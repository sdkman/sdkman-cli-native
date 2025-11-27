mod commands;
mod utils;

use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(
    name = "sdk",
    about = "The command line interface (CLI) for SDKMAN!",
    version,
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Help(commands::help::Args),
    Version(commands::version::Args),
    Default(commands::default::Args),
    Current(commands::current::Args),
    Uninstall(commands::uninstall::Args),
    Home(commands::home::Args),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Version(args) => commands::version::run(args),
        Commands::Help(args) => commands::help::run(args),
        Commands::Default(args) => commands::default::run(args),
        Commands::Current(args) => commands::current::run(args),
        Commands::Uninstall(args) => commands::uninstall::run(args),
        Commands::Home(args) => commands::home::run(args),
    };

    if let Err(code) = result {
        exit(code);
    }
}
