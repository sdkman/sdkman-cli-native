use std::{env, io, process};
use std::env::VarError;
use std::path::PathBuf;
use edit::edit_file;

enum CliError {
    EnvError,
    EditorError,
}

fn main() {
    process::exit(match edit_config() {
        Ok(()) => 0,
        Err(CliError::EnvError) => {
            println!("SDKMAN_DIR env variable not set.");
            1
        },
        Err(CliError::EditorError) => {
            println!("Unable to open editor.");
            1
        }
    });
}

impl From<VarError> for CliError {
    fn from(_: VarError) -> Self {
        CliError::EnvError
    }
}

impl From<io::Error> for CliError {
    fn from(_: io::Error) -> Self {
        CliError::EditorError
    }
}

fn edit_config() -> Result<(), CliError> {
    let config_path = build_config_path()?;

    Ok(edit_file(config_path)?)
}

fn build_config_path() -> Result<PathBuf, VarError> {
    let sdkman_dir = env::var("SDKMAN_DIR")?;
    let mut config = PathBuf::from(sdkman_dir);

    config.push("etc");
    config.push("config");

    Ok(config)
}
