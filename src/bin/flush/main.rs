use std::fs;
use std::path::PathBuf;
use std::process;

use clap::Parser;
use colored::Colorize;

use sdkman_cli_native::constants::{TMP_DIR, VAR_DIR};
use sdkman_cli_native::helpers::infer_sdkman_dir;

#[derive(Parser, Debug)]
#[command(
    bin_name = "sdk flush",
    about = "sdk subcommand to flush temporary files and metadata"
)]
struct Args {
    #[arg(required(false))]
    qualifier: Option<String>,
}

fn main() {
    let args = Args::parse();
    let sdkman_dir = infer_sdkman_dir();

    match args.qualifier.as_deref() {
        Some("version") => flush_version(sdkman_dir),
        Some("tmp") | Some("temp") => cleanup_folder(sdkman_dir, TMP_DIR),
        Some("metadata") => cleanup_folder(sdkman_dir, &format!("{}/metadata", VAR_DIR)),
        _ => {
            cleanup_folder(sdkman_dir.clone(), TMP_DIR);
            cleanup_folder(sdkman_dir, &format!("{}/metadata", VAR_DIR));
        }
    }
}

fn flush_version(sdkman_dir: PathBuf) {
    let version_file = sdkman_dir.join(VAR_DIR).join("version");
    if version_file.exists() {
        fs::remove_file(&version_file).expect("could not remove version file");
        println!("{}", "Version file has been flushed.".green());
    }
}

fn cleanup_folder(sdkman_dir: PathBuf, folder: &str) {
    let cleanup_dir = sdkman_dir.join(folder);

    if !cleanup_dir.exists() {
        fs::create_dir_all(&cleanup_dir).unwrap_or_else(|e| {
            eprintln!("could not create directory {}: {}", folder, e);
            process::exit(1);
        });
        println!(
            "{}",
            format!("0 archive(s) flushed, freeing 0B for {}.", folder).green()
        );
        return;
    }

    let count = fs::read_dir(&cleanup_dir)
        .map(|entries| entries.count())
        .unwrap_or(0);

    let disk_usage = get_disk_usage(&cleanup_dir);

    fs::remove_dir_all(&cleanup_dir).unwrap_or_else(|e| {
        eprintln!("could not remove directory {}: {}", folder, e);
        process::exit(1);
    });

    fs::create_dir_all(&cleanup_dir).unwrap_or_else(|e| {
        eprintln!("could not recreate directory {}: {}", folder, e);
        process::exit(1);
    });

    println!(
        "{}",
        format!(
            "{} archive(s) flushed, freeing {} for {}.",
            count, disk_usage, folder
        )
        .green()
    );
}

fn get_disk_usage(path: &PathBuf) -> String {
    let total: u64 = fs::read_dir(path)
        .ok()
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter_map(|e| e.metadata().ok())
                .map(|m| m.len())
                .sum()
        })
        .unwrap_or(0);
    format_size(total)
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.1}G", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1}M", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1}K", bytes as f64 / KB as f64)
    } else {
        format!("{}B", bytes)
    }
}
