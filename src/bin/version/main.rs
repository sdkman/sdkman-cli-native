extern crate core;

use colored::Colorize;

use sdkman_cli_native::helpers::check_exists;
use sdkman_cli_native::{
    constants::VAR_DIR,
    helpers::{infer_sdkman_dir, read_file_content},
};

const CLI_VERSION_FILE: &str = "version";
const NATIVE_VERSION_FILE: &str = "version_native";

fn main() {
    let sdkman_dir = infer_sdkman_dir();

    let cli_version_file = sdkman_dir.join(VAR_DIR).join(CLI_VERSION_FILE);
    let cli_version = read_file_content(check_exists(cli_version_file));

    let native_version_file = sdkman_dir.join(VAR_DIR).join(NATIVE_VERSION_FILE);
    let native_version = read_file_content(check_exists(native_version_file));

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
