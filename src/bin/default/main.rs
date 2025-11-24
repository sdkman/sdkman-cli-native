use clap::Parser;
use colored::Colorize;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::fs;
use std::fs::remove_dir_all;
use symlink::{remove_symlink_dir, symlink_dir};

use sdkman::constants::{CANDIDATES_DIR, CURRENT_DIR, TMP_DIR};
use sdkman::helpers::{
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
    let tmp_dir = sdkman_dir.join(TMP_DIR);
    let candidate = validate_candidate(known_candidates(sdkman_dir.to_owned()), &candidate);
    let version_path = validate_version_path(sdkman_dir.to_owned(), &candidate, &version);
    let current_link_path = sdkman_dir
        .join(CANDIDATES_DIR)
        .join(&candidate)
        .join(CURRENT_DIR);

    if current_link_path.exists() {
        remove_symlink_dir(&current_link_path).unwrap_or_else(|_| {
            remove_dir_all(&current_link_path).expect(&format!(
                "cannot remove current directory for {}.",
                candidate
            ))
        })
    }
    println!(
        "setting {} {} as the {} version for all shells.",
        &candidate.bold(),
        &version.bold(),
        "default".italic()
    );
    symlink_dir(&version_path, &current_link_path).unwrap_or_else(|_| {
        let options = CopyOptions::new();
        let mut version_paths = Vec::new();
        let version_path_string = version_path.into_os_string().into_string().unwrap();
        version_paths.push(version_path_string);

        copy_items(&version_paths, &tmp_dir, &options).expect("cannot copy to tmp folder.");
        let tmp_version_path = tmp_dir.join(&version);
        fs::rename(tmp_version_path, current_link_path).expect("cannot rename copied folder.");
        let error_message = format!(
            "cannot create {} symlink, fall back to copy!",
            "current".italic()
        );
        println!("{}", error_message.bold())
    })
}
