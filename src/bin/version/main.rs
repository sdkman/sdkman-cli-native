use colored::Colorize;

use sdkman::{
    constants::VAR_DIR,
    helpers::{check_file_exists, infer_sdkman_dir, read_file_content},
};
const CLI_VERSION_FILE: &str = "version";
const NATIVE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let sdkman_dir = infer_sdkman_dir();
    let cli_version_file = sdkman_dir.join(VAR_DIR).join(CLI_VERSION_FILE);
    let cli_version = read_file_content(check_file_exists(cli_version_file));

    println!(
        "\n{}\nscript: {}\nnative: {} ({} {})\n",
        "SDKMAN!".bold().yellow(),
        cli_version.expect("Failed to read CLI version file"),
        NATIVE_VERSION,
        std::env::consts::OS,
        std::env::consts::ARCH
    );
}
