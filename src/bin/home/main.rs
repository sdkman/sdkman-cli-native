use std::process;

use clap::Parser;
use colored::Colorize;

use sdkman_cli_native::constants::CANDIDATES_DIR;
use sdkman_cli_native::helpers::{infer_sdkman_dir, known_candidates, validate_candidate};

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

    validate_candidate(known_candidates(sdkman_dir.to_owned()), &candidate);

    let os_str = os_string.to_str().expect("could not interpret os string");
    let candidate_home = format!("{}/candidates/{}/{}", os_str, candidate, version);
    let candidate_path = Path::new(candidate_home.as_str());
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
