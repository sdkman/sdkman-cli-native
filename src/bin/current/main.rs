use std::fs;
use std::path::PathBuf;
use std::process;

use clap::Parser;
use colored::Colorize;

use sdkman_cli_native::constants::{CANDIDATES_DIR, CURRENT_DIR};
use sdkman_cli_native::helpers::{infer_sdkman_dir, known_candidates, validate_candidate};

#[derive(Parser, Debug)]
#[command(
    bin_name = "sdk current",
    about = "sdk subcommand to display the current version in use for one or all candidates"
)]
struct Args {
    #[arg(required(false))]
    candidate: Option<String>,
}

fn main() {
    let args = Args::parse();
    let sdkman_dir = infer_sdkman_dir();
    let all_candidates = known_candidates(sdkman_dir.to_owned());

    match args.candidate {
        Some(candidate) => {
            // Show current version for a specific candidate
            let candidate = validate_candidate(all_candidates, &candidate);
            let current_version = get_current_version(sdkman_dir.to_owned(), &candidate);
            match current_version {
                Some(version) => println!("Using {} version {}", candidate.bold(), version.bold()),
                _ => {
                    eprintln!("No current version of {} configured.", candidate.bold());
                    process::exit(1);
                }
            }
        }
        _ => {
            // Show current version for all candidates
            let mut found_any = false;
            let mut candidates_with_versions = Vec::new();

            // Collect all candidates with their versions first
            for candidate in all_candidates {
                let current_version = get_current_version(sdkman_dir.to_owned(), candidate);
                if let Some(version) = current_version {
                    candidates_with_versions.push((candidate, version));
                    found_any = true;
                }
            }

            if found_any {
                // Print header
                println!("{}", "Current versions in use:".bold());

                // Print all candidate versions
                for (candidate, version) in candidates_with_versions {
                    println!("{} {}", candidate, version);
                }
            } else {
                eprintln!("No candidates are in use.");
                process::exit(0);
            }
        }
    }
}

fn get_current_version(base_dir: PathBuf, candidate: &str) -> Option<String> {
    // First check if the candidate is installed
    let candidate_dir = base_dir.join(CANDIDATES_DIR).join(candidate);
    if !candidate_dir.exists() || !candidate_dir.is_dir() {
        return None;
    }

    // Check for current symlink
    let current_link = candidate_dir.join(CURRENT_DIR);
    if !current_link.exists() {
        return None;
    }

    // Get the symlink target (which should be the version)
    if let Ok(target) = fs::read_link(&current_link) {
        // Extract the version from the path
        return target
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string());
    }

    // If this is not a symlink but a directory (fallback case)
    if current_link.is_dir() {
        return current_link
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string());
    }

    None
}
