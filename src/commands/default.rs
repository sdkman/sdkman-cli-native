use crate::utils::{
    constants::{CANDIDATES_DIR, CURRENT_DIR, TMP_DIR},
    directory_utils::infer_sdkman_dir,
    helpers::{known_candidates, validate_candidate, validate_version_path},
};
use colored::Colorize;
use fs_extra::{copy_items, dir::CopyOptions};
use std::{
    fs::{self, remove_dir_all},
    process::exit,
};
use symlink::{remove_symlink_dir, symlink_dir};

#[derive(clap::Args, Debug)]
#[command(about = "Set the local default version of a candidate")]
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

    let tmp_dir = sdkman_dir.join(TMP_DIR);
    let candidate = validate_candidate(&known_candidates(sdkman_dir.to_owned()), &args.candidate);
    let version_path = validate_version_path(sdkman_dir.to_owned(), &candidate, &args.version);

    let current_link_path = sdkman_dir
        .join(CANDIDATES_DIR)
        .join(&candidate)
        .join(CURRENT_DIR);

    // remove existing "current" (symlink or dir)
    if current_link_path.exists() {
        remove_symlink_dir(&current_link_path).unwrap_or_else(|_| {
            remove_dir_all(&current_link_path).unwrap_or_else(|e| {
                eprintln!(
                    "cannot remove current directory for {}: {}",
                    candidate.bold(),
                    e
                );
                exit(1);
            })
        });
    }

    println!(
        "setting {} {} as the {} version for all shells.",
        candidate.bold(),
        args.version.bold(),
        "default".italic()
    );

    // prefer symlink; fallback to copying into place if symlinks fail
    symlink_dir(&version_path, &current_link_path).unwrap_or_else(|_| {
        let options = CopyOptions::new();

        copy_items(&[version_path.clone()], &tmp_dir, &options).unwrap_or_else(|e| {
            eprintln!("cannot copy to tmp folder: {e}");
            exit(1);
        });

        let tmp_version_path = tmp_dir.join(&args.version);
        fs::rename(&tmp_version_path, &current_link_path).unwrap_or_else(|e| {
            eprintln!("cannot rename copied folder into place: {e}");
            exit(1);
        });

        println!(
            "{}",
            format!(
                "cannot create {} symlink, falling back to copy!",
                "current".italic()
            )
            .bold()
        );
    });

    Ok(())
}
