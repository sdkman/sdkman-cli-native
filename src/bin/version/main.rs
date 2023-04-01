extern crate core;

use std::path::PathBuf;

use colored::Colorize;
use sdkman_cli_native::{
    constants::VAR_DIR,
    helpers::{infer_sdkman_dir, read_file_content},
};

const CLI_VERSION_FILE: &str = "version";
const NATIVE_VERSION_FILE: &str = "version_native";

fn main() {
    let sdkman_dir = infer_sdkman_dir();
    let var_dir = PathBuf::from(VAR_DIR);

    let version_file = var_dir.join(CLI_VERSION_FILE);
    let native_version_file = var_dir.join(NATIVE_VERSION_FILE);

    let version = locate_file(sdkman_dir.to_owned(), version_file).and_then(read_file_content);
    let native_version =
        locate_file(sdkman_dir.to_owned(), native_version_file).and_then(read_file_content);

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

fn locate_file(base_dir: PathBuf, relative_path: PathBuf) -> Option<PathBuf> {
    Some(PathBuf::from(base_dir).join(relative_path))
}
