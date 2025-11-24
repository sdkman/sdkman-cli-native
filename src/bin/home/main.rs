use std::process;

use clap::Parser;
use colored::Colorize;

use sdkman::utils::constants::CANDIDATES_DIR;
use sdkman::helpers::{infer_sdkman_dir, known_candidates, validate_candidate};

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

fn main() {
    let args = Args::parse();
    let candidate = args.candidate;
    let version = args.version;
    let sdkman_dir = infer_sdkman_dir();

    let candidate = validate_candidate(known_candidates(sdkman_dir.to_owned()), &candidate);

    let candidate_path = sdkman_dir
        .join(CANDIDATES_DIR)
        .join(&candidate)
        .join(&version);
    if candidate_path.exists() && candidate_path.is_dir() {
        println!(
            "{}/{}/{}/{}",
            sdkman_dir.to_str().unwrap(),
            CANDIDATES_DIR,
            &candidate,
            &version
        );
    } else {
        eprintln!(
            "{} {} is not installed on your system.",
            candidate.bold(),
            version.bold()
        );
        process::exit(1);
    }
}
