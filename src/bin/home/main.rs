use clap::Parser;
use colored::Colorize;
use sdkman_cli_native::helpers::{infer_sdkman_dir, known_candidates};
use std::path::{Path, PathBuf};
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
    let sdkman_dir: PathBuf = infer_sdkman_dir();
    let os_string = sdkman_dir.to_owned().into_os_string();
    let all_candidates = known_candidates(sdkman_dir);

    if !all_candidates.contains(&candidate.as_str()) {
        eprint!("{} is not a valid candidate!", candidate.bold());
        process::exit(1);
    }

    let os_str = os_string.to_str().expect("could not interpret os string");
    let candidate_home = format!("{}/candidates/{}/{}", os_str, candidate, version);
    let candidate_path = Path::new(candidate_home.as_str());
    if candidate_path.is_dir() {
        println!("{}", candidate_home);
    } else {
        eprintln!(
            "{} {} is not installed on your system",
            candidate.bold(),
            version.bold()
        );
        process::exit(1);
    }
}
