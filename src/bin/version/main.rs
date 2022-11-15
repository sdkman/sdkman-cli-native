extern crate core;

use std::path::PathBuf;
use std::{env, fs};

use colored::Colorize;

const SDKMAN_DIR_ENV_VAR: &str = "SDKMAN_DIR";
const DEFAULT_SDKMAN_HOME: &str = ".sdkman";
const VAR_DIR: &str = "var";
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
            "\n{}: cli version: {}; native extensions: {}\n",
            "SDKMAN!".bold().yellow(),
            content,
            native
        ),
        _ => std::process::exit(exitcode::CONFIG),
    }
}

fn infer_sdkman_dir() -> PathBuf {
    match env::var(SDKMAN_DIR_ENV_VAR) {
        Ok(s) => PathBuf::from(s),
        Err(_) => fallback_sdkman_dir(),
    }
}

fn fallback_sdkman_dir() -> PathBuf {
    dirs::home_dir()
        .map(|dir| dir.join(DEFAULT_SDKMAN_HOME))
        .unwrap()
}

fn locate_file(base_dir: PathBuf, relative_path: PathBuf) -> Option<PathBuf> {
    Some(PathBuf::from(base_dir).join(relative_path))
}

fn read_file_content(path: PathBuf) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
    .filter(|s| !s.trim().is_empty())
    .map(|s| s.trim().to_string())
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::io::Write;
    use std::path::PathBuf;

    use serial_test::serial;
    use tempfile::NamedTempFile;

    use crate::{infer_sdkman_dir, read_file_content, SDKMAN_DIR_ENV_VAR};

    #[test]
    #[serial]
    fn should_infer_sdkman_dir_from_env_var() {
        let sdkman_dir = PathBuf::from("/home/someone/.sdkman");
        env::set_var(SDKMAN_DIR_ENV_VAR, sdkman_dir.to_owned());
        assert_eq!(sdkman_dir, infer_sdkman_dir());
    }

    #[test]
    #[serial]
    fn should_infer_fallback_dir() {
        env::remove_var(SDKMAN_DIR_ENV_VAR);
        let actual_sdkman_dir = dirs::home_dir().unwrap().join(".sdkman");
        assert_eq!(actual_sdkman_dir, infer_sdkman_dir());
    }

    #[test]
    #[serial]
    fn should_read_content_from_file() {
        let expected_version = "5.0.0";
        let mut file = NamedTempFile::new().unwrap();
        file.write(expected_version.as_bytes()).unwrap();
        let path = file.path().to_path_buf();
        let maybe_version = read_file_content(path);
        assert_eq!(maybe_version, Some(expected_version.to_string()));
    }

    #[test]
    #[serial]
    fn should_fail_reading_content_from_empty_file() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        let maybe_version = read_file_content(path);
        assert_eq!(maybe_version, None);
    }
}
