use clap::Parser;
use colored::Colorize;
use sdkman_cli_native::helpers::{infer_sdkman_dir, known_candidates, validate_candidate};
use std::path::PathBuf;
use std::process;

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
    let sdkman_path = infer_sdkman_dir();

    validate_candidate(known_candidates(sdkman_path.to_owned()), &candidate);

    let sdkman_dir = sdkman_path.to_str().unwrap();
    let candidate_home = format!("{}/candidates/{}/{}", sdkman_dir, candidate, version);
    let candidate_path = PathBuf::from(&candidate_home);
    if candidate_path.is_dir() {
        println!("{}", candidate_home);
    } else {
        eprintln!(
            "{} {} is not installed on your system.",
            candidate.bold(),
            version.bold()
        );
        process::exit(1);
    }
}
