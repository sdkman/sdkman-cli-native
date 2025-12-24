//! `sdk` — a native CLI companion for SDKMAN!.
//!
//! A small, fast command-line interface that reads the local SDKMAN! directory
//! layout (typically `$SDKMAN_DIR`) and implements a subset of `sdk` commands in Rust.
//!
//! ## Commands
//! - `sdk help [subcommand]`
//! - `sdk version [--native-only]`
//! - `sdk default <candidate> <version>`
//! - `sdk current [candidate]`
//! - `sdk uninstall <candidate> <version> [--force]`
//! - `sdk home <candidate> <version>`
//!
//! ## Doctest examples
//!
//! Print the script + native versions (reads `$SDKMAN_DIR/var/version`):
//! ```no_run
//! # use std::process::Command;
//! Command::new("sdk")
//!     .arg("version")
//!     .status()
//!     .expect("failed to run sdk");
//! ```
//!
//! Print only the native binary version:
//! ```no_run
//! # use std::process::Command;
//! Command::new("sdk")
//!     .args(["version", "--native-only"])
//!     .status()
//!     .expect("failed to run sdk");
//! ```
//!
//! Set a default version (updates `$SDKMAN_DIR/candidates/<candidate>/current`):
//! ```no_run
//! # use std::process::Command;
//! Command::new("sdk")
//!     .args(["default", "java", "17.0.0-tem"])
//!     .status()
//!     .expect("failed to run sdk");
//! ```
//!
//! Query current version for one candidate:
//! ```no_run
//! # use std::process::Command;
//! Command::new("sdk")
//!     .args(["current", "java"])
//!     .status()
//!     .expect("failed to run sdk");
//! ```
//!
//! Use a specific SDKMAN directory for a single call:
//! ```no_run
//! # use std::process::Command;
//! Command::new("sdk")
//!     .env("SDKMAN_DIR", "/home/me/.sdkman")
//!     .args(["home", "kotlin", "1.9.24"])
//!     .status()
//!     .expect("failed to run sdk");
//! ```
mod commands;
mod utils;

use clap::{Parser, Subcommand};
use std::process::exit;

/// Top-level CLI parser for the `sdk` binary.
#[derive(Parser, Debug)]
#[command(
    name = "sdk",
    about = "The command line interface (CLI) for SDKMAN!",
    version,
    propagate_version = true,
    disable_version_flag = true,
    disable_help_subcommand = true
)]
struct Cli {
    /// The subcommand to execute.
    #[command(subcommand)]
    command: Commands,
}

/// Supported `sdk` subcommands.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Show detailed help for a subcommand.
    Help(commands::help::Args),
    /// Display the installed SDKMAN! version (script + native).
    Version(commands::version::Args),
    /// Set the local default version of a candidate.
    Default(commands::default::Args),
    /// Display the current version in use for one or all candidates.
    Current(commands::current::Args),
    /// Remove a specific candidate version.
    Uninstall(commands::uninstall::Args),
    /// Output the absolute path of a specific candidate version.
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
