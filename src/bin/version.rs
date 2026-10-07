use std::fs;
use std::path::Path;
use std::process::ExitCode;

use sdkman_cli_native::{
    cli,
    constants::VAR_DIR,
    helpers::{infer_sdkman_dir, os_reason},
    ui::{self, CliError},
};

const CLI_VERSION_FILE: &str = "version";
const NATIVE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> ExitCode {
    if let Err(error) = cli::version().try_get_matches() {
        error.exit();
    }
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            error.report();
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), CliError> {
    let sdkman_dir = infer_sdkman_dir();
    ui::init(&sdkman_dir);

    let cli_version_file = sdkman_dir.join(VAR_DIR).join(CLI_VERSION_FILE);
    let cli_version = read_version(&cli_version_file)?;

    let body = format!(
        "\n{brand}\n{core:<width$} {cli}\n{native:<width$} {nat} ({os} {arch})\n",
        brand = ui::brand("SDKMAN!"),
        core = "core:",
        cli = cli_version,
        native = "native:",
        nat = NATIVE_VERSION,
        os = std::env::consts::OS,
        arch = std::env::consts::ARCH,
        width = "native:".len(),
    );
    ui::value(body);
    Ok(())
}

fn read_version(path: &Path) -> Result<String, CliError> {
    let content = fs::read_to_string(path).map_err(|error| CliError {
        message: format!("cannot read {}: {}", ui::path(path), os_reason(&error)),
        hints: vec![],
    })?;
    if content.trim().is_empty() {
        return Err(CliError {
            message: format!("cannot read {}: file is empty", ui::path(path)),
            hints: vec![],
        });
    }
    Ok(content.trim().to_string())
}
