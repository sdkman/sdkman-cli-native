use crate::utils::{
    constants::CANDIDATES_DIR,
    directory_utils::infer_sdkman_dir,
    helpers::{known_candidates, validate_candidate},
};
use colored::Colorize;

#[derive(clap::Args, Debug)]
#[command(about = "Output the path of a specific candidate version")]
pub struct Args {
    #[arg(required = true)]
    pub candidate: String,

    #[arg(required = true)]
    pub version: String,
}

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
