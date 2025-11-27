//! `sdk uninstall` command.
//!
//! Removes an installed candidate version from `$SDKMAN_DIR/candidates/<candidate>/<version>`.
//! If the target version is currently selected via the `current` link, removal is blocked unless
//! `--force` is provided.
//!
//! ## Flags
//! - `-f, --force`: remove even if the target is the current version (may leave the candidate unusable).
//!
//! ## Examples
//! ```no_run
//! # use std::process::Command;
//! Command::new("sdk")
//!     .args(["uninstall", "java", "17.0.0-tem"])
//!     .status()
//!     .unwrap();
//! ```
//!
//! ```no_run
//! # use std::process::Command;
//! Command::new("sdk")
//!     .args(["uninstall", "java", "17.0.0-tem", "--force"])
//!     .status()
//!     .unwrap();
//! ```

use crate::utils::{
    constants::{CANDIDATES_DIR, CURRENT_DIR},
    directory_utils::infer_sdkman_dir,
    helpers::{known_candidates, validate_candidate, validate_version_path},
};
use colored::Colorize;
use std::{fs, fs::remove_dir_all};
use symlink::remove_symlink_dir;

/// Arguments for `sdk uninstall`.
#[derive(clap::Args, Debug)]
#[command(about = "Remove a specific candidate version")]
pub struct Args {
    /// Remove even if this version is currently selected (may leave the candidate unusable).
    #[arg(short = 'f', long = "force")]
    pub force: bool,

    /// Candidate name (e.g. `java`).
    #[arg(required = true)]
    pub candidate: String,

    /// Candidate version to remove (e.g. `17.0.0-tem`).
    #[arg(required = true)]
    pub version: String,
}

/// Run `sdk uninstall`.
///
/// Returns `Ok(())` on success, or an exit code (`Err(code)`) on failure.
///
/// Behavior notes:
/// - Validates the candidate against the local candidates list.
/// - Refuses to remove the target if it is the `current` version unless `--force` is set.
/// - Attempts to remove the `current` symlink first (when forced), falling back to removing a
///   real directory if `current` is not a symlink.
pub fn run(args: Args) -> Result<(), i32> {
    let sdkman_dir = infer_sdkman_dir().map_err(|e| {
        eprintln!("failed to infer SDKMAN_DIR: {e}");
        1
    })?;

    let candidate = validate_candidate(&known_candidates(&sdkman_dir), &args.candidate);

    let candidate_path = sdkman_dir.join(CANDIDATES_DIR).join(&candidate);
    let version_path = validate_version_path(&sdkman_dir, &candidate, &args.version);
    let current_link_path = candidate_path.join(CURRENT_DIR);

    // if "current" points at the version we’re removing, enforce --force
    if current_link_path.is_dir() {
        match fs::read_link(&current_link_path) {
            Ok(relative_target) => {
                let resolved_link_path = candidate_path.join(relative_target);

                if version_path == resolved_link_path && args.force {
                    // remove the current symlink; fall back to removing a directory if needed
                    remove_symlink_dir(&current_link_path).unwrap_or_else(|_| {
                        remove_dir_all(&current_link_path).unwrap_or_else(|e| {
                            eprintln!(
                                "cannot remove current directory for {}: {}",
                                candidate.bold(),
                                e
                            );
                            std::process::exit(1);
                        })
                    });
                } else if version_path == resolved_link_path && !args.force {
                    eprintln!(
                        "\n{} {} is the {} version and should not be removed.",
                        candidate.bold(),
                        args.version.bold(),
                        "current".italic(),
                    );
                    eprintln!(
                        "\nOverride with {}, but leaves the candidate unusable!",
                        "--force".italic()
                    );
                    return Err(1);
                }
            }
            Err(e) => {
                eprintln!("current link broken, stepping over: {}", e);
            }
        }
    }

    remove_dir_all(&version_path)
        .map(|_| println!("removed {} {}.", candidate.bold(), args.version.bold()))
        .map_err(|e| {
            eprintln!(
                "could not delete directory {}: {}",
                version_path.display(),
                e
            );
            1
        })?;

    Ok(())
}
