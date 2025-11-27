//! `sdk current` command.
//!
//! Shows the version currently in use for a candidate, based on
//! `${SDKMAN_DIR}/candidates/<candidate>/current`.
//!
//! If a candidate is provided, prints a single line in the form:
//! `Using <candidate> version <version>`.
//!
//! If no candidate is provided, prints all candidates that have a resolvable
//! current version (sorted by candidate name) as `<candidate> <version>`.
//!
//! ## Exit codes
//! - `0` on success (including the “no candidates are in use” case)
//! - `1` if a specific candidate is requested but has no configured current version,
//!   or on invalid candidate / filesystem errors
//!
//! ## Examples
//! ```no_run
//! # use std::process::Command;
//! // Show current version for all candidates
//! Command::new("sdk").arg("current").status().unwrap();
//!
//! // Show current version for a specific candidate
//! Command::new("sdk").args(["current", "java"]).status().unwrap();
//! ```

use crate::utils::{
    constants::{CANDIDATES_DIR, CURRENT_DIR},
    directory_utils::infer_sdkman_dir,
    helpers::{known_candidates, validate_candidate},
};
use colored::Colorize;
use std::{fs, path::Path};

/// Arguments for `sdk current`.
#[derive(clap::Args, Debug)]
#[command(about = "Display the current version in use for one or all candidates")]
pub struct Args {
    /// Optional candidate name to query (otherwise prints all current versions).
    pub candidate: Option<String>,
}

/// Run `sdk current`.
///
/// Behavior:
/// - With a candidate: prints `Using <candidate> version <version>`.
/// - Without a candidate: prints a sorted list of candidates with a resolvable `current`.
pub fn run(args: Args) -> Result<(), i32> {
    let sdkman_dir = infer_sdkman_dir().map_err(|e| {
        eprintln!("failed to infer SDKMAN_DIR: {e}");
        1
    })?;

    let all_candidates = known_candidates(&sdkman_dir);

    match args.candidate {
        Some(candidate) => {
            let candidate = validate_candidate(&all_candidates, &candidate);
            match get_current_version(&sdkman_dir, &candidate) {
                Some(version) => {
                    println!("Using {} version {}", candidate.bold(), version.bold());
                    Ok(())
                }
                None => {
                    eprintln!("No current version of {} configured.", candidate.bold());
                    Err(1)
                }
            }
        }
        None => {
            let mut rows: Vec<(String, String)> = all_candidates
                .into_iter()
                .filter_map(|cand| get_current_version(&sdkman_dir, &cand).map(|v| (cand, v)))
                .collect();

            if rows.is_empty() {
                eprintln!("No candidates are in use.");
                return Ok(());
            }

            // Stable output.
            rows.sort_by(|a, b| a.0.cmp(&b.0));

            println!("{}", "Current versions in use:".bold());
            for (candidate, version) in rows {
                println!("{} {}", candidate, version);
            }
            Ok(())
        }
    }
}

/// Resolve the "current" version for `candidate`.
///
/// This prefers reading the `current` symlink target (the normal SDKMAN! layout).
/// If `current` is a real directory, it falls back to using the directory name.
///
/// Returns `None` if the candidate directory or `current` entry does not exist, or
/// if the current version cannot be determined.
fn get_current_version(base_dir: &Path, candidate: &str) -> Option<String> {
    let candidate_dir = base_dir.join(CANDIDATES_DIR).join(candidate);
    if !candidate_dir.is_dir() {
        return None;
    }

    let current_path = candidate_dir.join(CURRENT_DIR);
    if !current_path.exists() {
        return None;
    }

    // Primary: read symlink target.
    if let Ok(target) = fs::read_link(&current_path) {
        return target
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string());
    }

    // Fallback: if `current` is a directory, attempt to treat its name as version.
    if current_path.is_dir() {
        return current_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string());
    }

    None
}
