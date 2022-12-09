extern crate core;

use clap::Parser;
use std::ops::Add;
use std::path::PathBuf;
use std::{env, fs};

use colored::Colorize;

const SDKMAN_DIR_ENV_VAR: &str = "SDKMAN_DIR";
const DEFAULT_SDKMAN_HOME: &str = ".sdkman";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(index = 1)]
    candidate: String,
    #[clap(index = 2)]
    version: String,
}

fn main() {
    let sdkman_dir = infer_sdkman_dir();

    let args = Args::parse();

    let sdkman_canidates_dir = sdkman_dir.join("candidates");
    let sdkman_canidate_dir = sdkman_canidates_dir.join(args.candidate.clone());
    let current_canidate_file = sdkman_canidate_dir.join("current");

    let current_version = fs::read_link(current_canidate_file.clone()).and_then(|p| {
        let path_str = p.to_string_lossy();
        let current_version = path_str
            .strip_prefix(&String::from(&*sdkman_canidate_dir.to_string_lossy()).add("/"))
            .and_then(|f| Some(String::from(f)));
        Ok(current_version.unwrap_or(String::from(path_str)))
    });
    let is_current_version = current_version
        .and_then(|i| Ok(i.eq(&args.version)))
        .unwrap_or(false);
    if is_current_version {
        println!(
            "{}",
            format!("Deselecting {} {}...", args.candidate, args.version).green()
        );
        fs::remove_file(current_canidate_file.clone()).unwrap_or_else(|e| {
            eprintln!(
                "remove_file {} failed! {e}",
                &current_canidate_file.to_string_lossy()
            );
        });
    }

    let version_canidate_file = sdkman_canidate_dir.join(args.version.clone());
    println!(
        "{}",
        format!("Uninstalling {} {}", args.candidate, args.version).green()
    );
    fs::remove_dir_all(version_canidate_file.clone()).unwrap_or_else(|e| {
        eprintln!(
            "remove_dir_all {} failed! {e}",
            version_canidate_file.to_string_lossy()
        );
    });
}

fn infer_sdkman_dir() -> PathBuf {
    match env::var(SDKMAN_DIR_ENV_VAR) {
        Ok(s) => PathBuf::from(s),
        Err(_) => fallback_sdkman_dir(),
    }
}

fn fallback_sdkman_dir() -> PathBuf {
    dirs::home_dir()
        .map(|dir| dir.join(DEFAULT_SDKMAN_HOME))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;

    use serial_test::serial;

    use crate::{infer_sdkman_dir, SDKMAN_DIR_ENV_VAR};

    #[test]
    #[serial]
    fn should_infer_sdkman_dir_from_env_var() {
        let sdkman_dir = PathBuf::from("/home/someone/.sdkman");
        env::set_var(SDKMAN_DIR_ENV_VAR, sdkman_dir.to_owned());
        assert_eq!(sdkman_dir, infer_sdkman_dir());
    }

    #[test]
    #[serial]
    fn should_infer_fallback_dir() {
        env::remove_var(SDKMAN_DIR_ENV_VAR);
        let actual_sdkman_dir = dirs::home_dir().unwrap().join(".sdkman");
        assert_eq!(actual_sdkman_dir, infer_sdkman_dir());
    }
}
