use std::io::Write;
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
            print_main_help(&mut sdk);
            ExitCode::SUCCESS
        }
    }
}

/// clap writes a single command alias as `[alias: X]`, but the Help Catalogue
/// lists every alias as `[aliases: X]`, so the plural form is restored here.
fn print_main_help(sdk: &mut Command) {
    let page = sdk
        .render_long_help()
        .ansi()
        .to_string()
        .replace("[alias: ", "[aliases: ");
    let _ = write!(anstream::stdout(), "{page}");
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
