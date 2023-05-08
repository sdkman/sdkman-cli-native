extern crate core;

use colored::Colorize;
use sdkman_cli_native::helpers::check_exists;
use sdkman_cli_native::{
    constants::VAR_DIR,
    helpers::{infer_sdkman_dir, read_file_content},
};
use std::path::PathBuf;

const CLI_VERSION_FILE: &str = "version";
const NATIVE_VERSION_FILE: &str = "version_native";

fn main() {
    let inferred_dir = infer_sdkman_dir();
    let sdkman_dir = inferred_dir.to_str().unwrap();

    let cli_version = read_version_file(format!("{}/{}/{}", sdkman_dir, VAR_DIR, CLI_VERSION_FILE));
    let native_version = read_version_file(format!(
        "{}/{}/{}",
        sdkman_dir, VAR_DIR, NATIVE_VERSION_FILE
    ));

    match (cli_version, native_version) {
        (Some(cli), Some(native)) => println!(
            "\n{}\nscript: {}\nnative: {}\n",
            "SDKMAN!".bold().yellow(),
            cli,
            native
        ),
        _ => std::process::exit(exitcode::CONFIG),
    }
}

fn read_version_file(file_location: String) -> Option<String> {
    let version_path = PathBuf::from(file_location);
    let verified_path = check_exists(version_path);
    read_file_content(verified_path)
}
