use clap::Parser;
use colored::Colorize;
use std::fs::{copy, remove_dir_all};
use symlink::{remove_symlink_dir, symlink_dir};

use sdkman_cli_native::constants::{CANDIDATES_DIR, CURRENT_DIR};
use sdkman_cli_native::helpers::{
    infer_sdkman_dir, known_candidates, validate_candidate, validate_version_path,
};

#[derive(Parser, Debug)]
#[command(
    bin_name = "sdk default",
    about = "sdk subcommand to set the local default version of the candidate"
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
    let version_path = validate_version_path(sdkman_dir.to_owned(), &candidate, &version);
    let current_link_path = sdkman_dir
        .join(CANDIDATES_DIR)
        .join(&candidate)
        .join(CURRENT_DIR);

    if current_link_path.exists() {
        remove_symlink_dir(current_link_path.to_owned()).unwrap_or_else(|_| {
            remove_dir_all(current_link_path.to_owned()).expect(&format!(
                "cannot remove current directory for {}",
                candidate
            ))
        })
    }
    symlink_dir(version_path.to_owned(), current_link_path.to_owned())
        .map(|_| {
            println!(
                "set {} {} as {} version",
                &candidate.bold(),
                &version.bold(),
                "default".italic()
            )
        })
        .unwrap_or_else(|_| {
            copy(version_path, current_link_path).expect("cannot copy directory");
        })
}
