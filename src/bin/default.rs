use std::fs;
use std::fs::remove_dir_all;
use std::io;
use std::process::ExitCode;

use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use symlink::{remove_symlink_dir, symlink_dir};

use sdkman_cli_native::cli;
use sdkman_cli_native::constants::{CANDIDATES_DIR, CURRENT_DIR, TMP_DIR};
use sdkman_cli_native::helpers::{
    infer_sdkman_dir, known_candidates, os_reason, require_candidate, require_version_path,
};
use sdkman_cli_native::ui::{self, CliError};

fn main() -> ExitCode {
    let matches = match cli::default().try_get_matches() {
        Ok(matches) => matches,
        Err(error) => return cli::report_parse_error("default", error),
    };
    let (Some(candidate), Some(version)) = (
        matches.get_one::<String>("candidate"),
        matches.get_one::<String>("version"),
    ) else {
        return ExitCode::from(1);
    };
    match run(candidate, version) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error.report();
            ExitCode::from(1)
        }
    }
}

fn run(candidate: &str, version: &str) -> Result<(), CliError> {
    let sdkman_dir = infer_sdkman_dir();
    ui::init(&sdkman_dir);

    let all_candidates = known_candidates(sdkman_dir.clone())?;
    let candidate = require_candidate(&all_candidates, candidate)?;
    let version_path = require_version_path(sdkman_dir.clone(), &candidate, version)?;

    let tmp_dir = sdkman_dir.join(TMP_DIR);
    let current_link_path = sdkman_dir
        .join(CANDIDATES_DIR)
        .join(&candidate)
        .join(CURRENT_DIR);

    if current_link_path.exists() {
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
    }

    let copied = symlink_dir(&version_path, &current_link_path).is_err();
    if copied {
        copy_items(&[&version_path], &tmp_dir, &CopyOptions::new()).map_err(|error| CliError {
            message: format!(
                "cannot copy {} to {}: {}",
                ui::path(&version_path),
                ui::path(&tmp_dir),
                copy_reason(&error)
            ),
            hints: vec![],
        })?;
        let tmp_version_path = tmp_dir.join(version);
        fs::rename(&tmp_version_path, &current_link_path).map_err(|error| CliError {
            message: format!(
                "cannot move {} to {}: {}",
                ui::path(&tmp_version_path),
                ui::path(&current_link_path),
                os_reason(&error)
            ),
            hints: vec![],
        })?;
    }

    ui::success(format!(
        "Set {} as the default version",
        ui::sdk_version(&candidate, version)
    ));
    if copied {
        ui::warning(format!(
            "cannot create a symlink here, so SDKMAN! copied {} instead",
            ui::sdk_version(&candidate, version)
        ));
    }
    Ok(())
}

fn copy_reason(error: &fs_extra::error::Error) -> String {
    match &error.kind {
        fs_extra::error::ErrorKind::Io(io_error) => os_reason(io_error),
        _ => os_reason(&io::Error::other(error.to_string())),
    }
}
