use std::process::ExitCode;

use clap::Parser;

use sdkman_cli_native::helpers::{
    infer_sdkman_dir, known_candidates, require_candidate, require_version_path,
};
use sdkman_cli_native::ui::{self, CliError};

#[derive(Parser, Debug)]
#[command(
    bin_name = "sdk home",
    about = "sdk subcommand to output the path of a specific candidate version"
)]
struct Args {
    #[arg(required(true))]
    candidate: String,

    #[arg(required(true))]
    version: String,
}

fn main() -> ExitCode {
    let args = Args::parse();
    match run(&args.candidate, &args.version) {
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
    let version_path = require_version_path(sdkman_dir, &candidate, version)?;

    ui::value(version_path.display());
    Ok(())
}
