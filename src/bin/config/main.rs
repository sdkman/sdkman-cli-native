use std::env;
use std::process::Command;

use clap::Parser;
use colored::Colorize;

use sdkman_cli_native::helpers::infer_sdkman_dir;

#[derive(Parser, Debug)]
#[command(
    bin_name = "sdk config",
    about = "sdk subcommand to edit the SDKMAN configuration file"
)]
struct Args;

fn main() {
    let sdkman_dir = infer_sdkman_dir();
    let config_path = sdkman_dir.join("etc").join("config");

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    let mut parts = editor.split_whitespace();
    let cmd = match parts.next() {
        Some(c) => c,
        None => {
            eprintln!("{}", "No default editor configured.".red());
            println!(
                "{}",
                "Please set the default editor with the EDITOR environment variable.".yellow()
            );
            std::process::exit(1);
        }
    };
    let extra_args: Vec<&str> = parts.collect();

    if which_exists(cmd) {
        let mut command = Command::new(cmd);
        command.args(&extra_args);
        command.arg(&config_path);
        let status = command.status().expect("failed to execute editor");
        std::process::exit(status.code().unwrap_or(1));
    } else {
        eprintln!(
            "{}",
            format!("Editor '{}' not found. Please set EDITOR to a valid editor.", cmd).red()
        );
        println!(
            "{}",
            "Please set the default editor with the EDITOR environment variable.".yellow()
        );
        std::process::exit(1);
    }
}

fn which_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
