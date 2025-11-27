use colored::Colorize;
use std::{fs, path::Path};

use crate::utils::{
    constants::{CANDIDATES_DIR, CURRENT_DIR},
    directory_utils::infer_sdkman_dir,
    helpers::{known_candidates, validate_candidate},
};

#[derive(clap::Args, Debug)]
#[command(about = "Display the current version in use for one or all candidates")]
pub struct Args {
    /// optional candidate name to query (otherwise prints all current versions)
    pub candidate: Option<String>,
}

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

            // stable output
            rows.sort_by(|a, b| a.0.cmp(&b.0));

            println!("{}", "Current versions in use:".bold());
            for (candidate, version) in rows {
                println!("{} {}", candidate, version);
            }
            Ok(())
        }
    }
}

/// returns the "current" version for a candidate by reading the `current` symlink,
/// or (fallback) extracting a directory name
fn get_current_version(base_dir: &Path, candidate: &str) -> Option<String> {
    let candidate_dir = base_dir.join(CANDIDATES_DIR).join(candidate);
    if !candidate_dir.is_dir() {
        return None;
    }

    let current_path = candidate_dir.join(CURRENT_DIR);
    if !current_path.exists() {
        return None;
    }

    // primary: read symlink target
    if let Ok(target) = fs::read_link(&current_path) {
        return target
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string());
    }

    // fallback: if `current` is a directory, attempt to treat its name as version
    // (matches your previous behavior, even though it’s unusual).
    if current_path.is_dir() {
        return current_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string());
    }

    None
}
