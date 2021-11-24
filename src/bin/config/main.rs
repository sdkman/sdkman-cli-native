use std::env;
use std::env::{VarError};
use std::path::{PathBuf};

use edit::edit_file;

fn main() {
    let config_path = match build_config_path() {
        Ok(config_path) => config_path,
        Err(error) => {
            eprintln!("The environment variable `SDKMAN_DIR` is not present: {}", error);
            std::process::exit(1);
        }
    };

    if edit_file(config_path).is_err() {
        eprintln!("Unable to open editor.");
        std::process::exit(1);
    }
}

fn build_config_path() -> Result<PathBuf, VarError> {
    let sdkman_dir = env::var("SDKMAN_DIR")?;
    let mut config = PathBuf::from(sdkman_dir);

    config.push("etc");
    config.push("config");

    Ok(config)
}
