use std::process::ExitCode;

use clap::Command;
use sdkman_cli_native::cli;
use sdkman_cli_native::helpers::closest_candidate;
use sdkman_cli_native::ui::{self, CliError};

fn main() -> ExitCode {
    let mut sdk = cli::sdk();
    match std::env::args().nth(1) {
        Some(name) => match sdk.find_subcommand(&name).cloned() {
            Some(mut command) => {
                let _ = command.print_long_help();
                ExitCode::SUCCESS
            }
            None => report_unknown_command(&sdk, &name),
        },
        None => {
            let _ = sdk.print_long_help();
            ExitCode::SUCCESS
        }
    }
}

fn report_unknown_command(sdk: &Command, name: &str) -> ExitCode {
    let names: Vec<String> = sdk
        .get_subcommands()
        .map(|command| command.get_name().to_string())
        .collect();
    let hint = match closest_candidate(&names, name) {
        Some(suggestion) => format!("did you mean {}?", ui::cmd(&suggestion)),
        None => format!("run {} to see all commands", ui::cmd("sdk help")),
    };
    CliError {
        message: format!("unknown command {name}"),
        hints: vec![hint],
    }
    .report();
    ExitCode::from(2)
}
