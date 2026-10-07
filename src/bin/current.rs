use std::fs;
use std::path::Path;
use std::process::ExitCode;

use clap::Parser;

use sdkman_cli_native::constants::{CANDIDATES_DIR, CURRENT_DIR};
use sdkman_cli_native::helpers::{infer_sdkman_dir, known_candidates, require_candidate};
use sdkman_cli_native::ui::{self, CliError};

#[derive(Parser, Debug)]
#[command(
    bin_name = "sdk current",
    about = "sdk subcommand to display the current version in use for one or all candidates"
)]
struct Args {
    #[arg(required(false))]
    candidate: Option<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    match run(args.candidate) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error.report();
            ExitCode::from(1)
        }
    }
}

fn run(candidate: Option<String>) -> Result<(), CliError> {
    let sdkman_dir = infer_sdkman_dir();
    ui::init(&sdkman_dir);

    let all_candidates = known_candidates(sdkman_dir.clone())?;

    match candidate {
        Some(candidate) => {
            let candidate = require_candidate(&all_candidates, &candidate)?;
            match get_current_version(&sdkman_dir, &candidate) {
                Some(version) => ui::value(version),
                None => {
                    return Err(CliError {
                        message: format!("{} has no default version", ui::sdk(&candidate)),
                        hints: vec![format!(
                            "run {}",
                            ui::cmd(&format!("sdk default {candidate} <version>"))
                        )],
                    })
                }
            }
        }
        None => {
            let rows: Vec<(String, String)> = all_candidates
                .into_iter()
                .filter_map(|candidate| {
                    get_current_version(&sdkman_dir, &candidate).map(|version| (candidate, version))
                })
                .collect();
            if rows.is_empty() {
                ui::info("No SDK has a default version");
                ui::hint(format!("run {}", ui::cmd("sdk install <sdk>")));
            } else {
                ui::table("Default versions", &rows);
            }
        }
    }
    Ok(())
}

fn get_current_version(base_dir: &Path, candidate: &str) -> Option<String> {
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
