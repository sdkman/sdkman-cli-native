extern crate core;

use std::path::PathBuf;

use colored::Colorize;
use sdkman_cli_native::{
    constants::VAR_DIR,
    helpers::{absolute_path, infer_sdkman_dir, read_file_content},
};

const CLI_VERSION_FILE: &str = "version";
const NATIVE_VERSION_FILE: &str = "version_native";

fn main() {
    let sdkman_dir = infer_sdkman_dir();
    let var_dir = PathBuf::from(VAR_DIR);

    let version_file = var_dir.join(CLI_VERSION_FILE);
    let native_version_file = var_dir.join(NATIVE_VERSION_FILE);

    let absolute_version = absolute_path(sdkman_dir.to_owned(), version_file);
    let version = read_file_content(absolute_version);

    let absolute_native_version = absolute_path(sdkman_dir.to_owned(), native_version_file);
    let native_version = read_file_content(absolute_native_version);

    match (version, native_version) {
        (Some(content), Some(native)) => println!(
            "\n{}\nscript: {}\nnative: {}\n",
            "SDKMAN!".bold().yellow(),
            content,
            native
        ),
        _ => std::process::exit(exitcode::CONFIG),
    }
}
