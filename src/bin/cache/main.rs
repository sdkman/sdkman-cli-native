use std::process;

use clap::Parser;
use colored::Colorize;

use sdkman_cli_native::constants::VAR_DIR;
use sdkman_cli_native::helpers::{infer_sdkman_dir, read_file_content};

#[derive(Parser, Debug)]
#[command(
    bin_name = "sdk cache",
    about = "sdk subcommand to validate the SDKMAN candidates cache"
)]
struct Args;

fn main() {
    let sdkman_dir = infer_sdkman_dir();
    let cache_path = sdkman_dir.join(VAR_DIR).join("candidates");

    if !cache_path.exists() || !cache_path.is_file() {
        print_corrupt_cache_message();
        process::exit(1);
    }

    match read_file_content(cache_path) {
        Some(_) => {}
        None => {
            print_corrupt_cache_message();
            process::exit(1);
        }
    }
}

fn print_corrupt_cache_message() {
    eprintln!(
        "{}",
        "WARNING: Cache is corrupt. SDKMAN cannot be used until updated.".red()
    );
    println!();
    println!("  $ sdk update");
    println!();
}
