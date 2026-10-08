use std::fs;
use std::fs::remove_dir_all;
use std::process::ExitCode;

use symlink::remove_symlink_dir;

use sdkman_cli_native::cli;
use sdkman_cli_native::constants::{CANDIDATES_DIR, CURRENT_DIR};
use sdkman_cli_native::helpers::{
    infer_sdkman_dir, known_candidates, os_reason, require_candidate, require_version_path,
};
use sdkman_cli_native::ui::{self, CliError};

fn main() -> ExitCode {
    let matches = match cli::uninstall().try_get_matches() {
        Ok(matches) => matches,
        Err(error) => return cli::report_parse_error("uninstall", error),
    };
    let force = matches.get_flag("force");
    let (Some(candidate), Some(version)) = (
        matches.get_one::<String>("candidate"),
        matches.get_one::<String>("version"),
    ) else {
        return ExitCode::from(1);
    };
    match run(candidate, version, force) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error.report();
            ExitCode::from(1)
        }
    }
}

fn run(candidate: &str, version: &str, force: bool) -> Result<(), CliError> {
    let sdkman_dir = infer_sdkman_dir()?;
    ui::init(&sdkman_dir);

    let all_candidates = known_candidates(sdkman_dir.clone())?;
    let candidate = require_candidate(&all_candidates, candidate)?;

    let candidate_path = sdkman_dir.join(CANDIDATES_DIR).join(&candidate);
    let version_path =
        require_version_path(sdkman_dir, &candidate, version).map_err(|mut error| {
            error.hints = vec![format!(
                "run {} to see installed versions",
                ui::cmd(&format!("sdk list {candidate}"))
            )];
            error
        })?;
    let current_link_path = candidate_path.join(CURRENT_DIR);

    let mut removed_default = false;
    if current_link_path.is_dir() {
        match fs::read_link(&current_link_path) {
            Ok(relative_resolved_dir) => {
                let resolved_link_path = candidate_path.join(relative_resolved_dir);
                if version_path == resolved_link_path {
                    if !force {
                        return Err(CliError {
                            message: format!(
                                "{} is the default version",
                                ui::sdk_version(&candidate, version)
                            ),
                            hints: vec![
                                format!(
                                    "run {} first",
                                    ui::cmd(&format!("sdk default {candidate} <version>"))
                                ),
                                format!(
                                    "or run {}",
                                    ui::cmd(&format!(
                                        "sdk uninstall --force {candidate} {version}"
                                    ))
                                ),
                            ],
                        });
                    }
                    remove_symlink_dir(&current_link_path)
                        .or_else(|_| remove_dir_all(&current_link_path))
                        .map_err(|error| CliError {
                            message: format!(
                                "cannot remove {}: {}",
                                ui::path(&current_link_path),
                                os_reason(&error)
                            ),
                            hints: vec![],
                        })?;
                    removed_default = true;
                }
            }
            Err(error) => {
                ui::warning(format!(
                    "cannot tell which version is the default version of {}: {}",
                    ui::sdk(&candidate),
                    os_reason(&error)
                ));
            }
        }
    }

    remove_dir_all(&version_path).map_err(|error| CliError {
        message: format!(
            "cannot remove {}: {}",
            ui::path(&version_path),
            os_reason(&error)
        ),
        hints: vec![],
    })?;

    ui::success(format!(
        "Uninstalled {}",
        ui::sdk_version(&candidate, version)
    ));
    if removed_default {
        ui::warning(format!(
            "{} has no default version now",
            ui::sdk(&candidate)
        ));
    }
    Ok(())
}
