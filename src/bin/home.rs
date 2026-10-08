use std::process::ExitCode;

use sdkman_cli_native::cli;
use sdkman_cli_native::helpers::{
    infer_sdkman_dir, known_candidates, require_candidate, require_version_path,
};
use sdkman_cli_native::ui::{self, CliError};

fn main() -> ExitCode {
    let matches = match cli::home().try_get_matches() {
        Ok(matches) => matches,
        Err(error) => return cli::report_parse_error("home", error),
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
    let sdkman_dir = infer_sdkman_dir()?;
    ui::init(&sdkman_dir);

    let all_candidates = known_candidates(sdkman_dir.clone())?;
    let candidate = require_candidate(&all_candidates, candidate)?;
    let version_path = require_version_path(sdkman_dir, &candidate, version)?;

    ui::value(version_path.display());
    Ok(())
}
