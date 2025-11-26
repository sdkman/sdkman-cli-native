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
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Version(args) => commands::version::run(args),
        Commands::Help(args) => commands::help::run(args),
    };

    if let Err(code) = result {
        exit(code);
    }
}
