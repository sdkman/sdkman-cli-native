use std::process::ExitCode;

use clap::Command;
use sdkman_cli_native::cli;

fn main() -> ExitCode {
    let sdk = sdk_command();
    match std::env::args().nth(1) {
        Some(name) => match sdk.find_subcommand(&name) {
            Some(command) => {
                let _ = command.clone().print_long_help();
                ExitCode::SUCCESS
            }
            None => {
                let _ = sdk_command().print_long_help();
                ExitCode::SUCCESS
            }
        },
        None => {
            let _ = sdk_command().print_long_help();
            ExitCode::SUCCESS
        }
    }
}

fn sdk_command() -> Command {
    Command::new("sdk")
        .override_usage("sdk <COMMAND> [ARGUMENTS]")
        .subcommand(cli::current())
        .subcommand(cli::default())
        .subcommand(cli::home())
        .subcommand(cli::uninstall())
        .subcommand(cli::version())
}
