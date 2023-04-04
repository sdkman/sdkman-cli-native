use std::fs;
use std::path::PathBuf;
use std::process;

use clap::Parser;
use colored::Colorize;

use sdkman_cli_native::helpers::{infer_sdkman_dir, known_candidates};

#[derive(Parser, Debug)]
#[command(
    bin_name = "sdk uninstall",
    about = "sdk subcommand to remove a specific candidate version"
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

    let all_candidates = known_candidates(sdkman_dir.to_owned());
    if !all_candidates.contains(&candidate.as_str()) {
        eprint!("{} is not a valid candidate.", candidate.bold());
        process::exit(1);
    }

    let os_string = sdkman_dir.into_os_string();
    let os_str = os_string
        .to_str()
        .expect("panic! could not interpret os string");

    let candidate_version_path =
        PathBuf::from(format!("{}/candidates/{}/{}", os_str, candidate, version));
    let current_link_path = PathBuf::from(format!("{}/candidates/{}/current", os_str, candidate));
    if current_link_path.is_dir() {
        let read_link = fs::read_link(current_link_path).expect("panic! can't read link");
        let canonical_link =
            PathBuf::from(format!("{}/candidates/{}", os_str, candidate)).join(read_link);
        if candidate_version_path == canonical_link {
            eprint!(
                "you are not permitted to delete the {} version of {}.",
                "current".bold(),
                candidate.bold()
            );
            process::exit(1);
        }
    }

    if candidate_version_path.is_dir() {
        fs::remove_dir_all(candidate_version_path).expect("panic! could not delete directory");
        println!("Removed {} {}", candidate.bold(), version.bold());
    } else {
        eprintln!(
            "{} {} is not installed on your system.",
            candidate.bold(),
            version.bold()
        );
        process::exit(1);
    }
}
