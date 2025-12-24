//! `sdk home` command.
//!
//! Prints the absolute path to an installed candidate version under
//! `$SDKMAN_DIR/candidates/<candidate>/<version>`.
//!
//! Exits with a non-zero code if the candidate/version directory does not exist.
//!
//! ## Examples
//! ```no_run
//! # use std::process::Command;
//! Command::new("sdk")
//!     .args(["home", "java", "17.0.0-tem"])
//!     .status()
//!     .unwrap();
//! ```

use crate::utils::{
    constants::CANDIDATES_DIR,
    directory_utils::infer_sdkman_dir,
    helpers::{known_candidates, validate_candidate},
};
use colored::Colorize;

/// Arguments for `sdk home`.
#[derive(clap::Args, Debug)]
#[command(about = "Output the path of a specific candidate version")]
pub struct Args {
    /// Candidate name (e.g. `java`).
    #[arg(required = true)]
    pub candidate: String,

    /// Candidate version (e.g. `17.0.0-tem`).
    #[arg(required = true)]
    pub version: String,
}

/// Run `sdk home`.
///
/// Returns `Ok(())` on success, or an exit code (`Err(code)`) on failure.
pub fn run(args: Args) -> Result<(), i32> {
    let sdkman_dir = infer_sdkman_dir().map_err(|e| {
        eprintln!("failed to infer SDKMAN_DIR: {e}");
        1
    })?;

    let candidate = validate_candidate(&known_candidates(&sdkman_dir), &args.candidate);

    let candidate_path = sdkman_dir
        .join(CANDIDATES_DIR)
        .join(&candidate)
        .join(&args.version);

    if candidate_path.is_dir() {
        // print absolute path to the version directory
        println!("{}", candidate_path.display());
        Ok(())
    } else {
        eprintln!(
            "{} {} is not installed on your system.",
            candidate.bold(),
            args.version.bold()
        );
        Err(1)
    }
}
