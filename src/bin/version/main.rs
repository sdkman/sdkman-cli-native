extern crate core;

use colored::Colorize;
use std::path::{Path, PathBuf};
use std::{env, fs};

const SDKMAN_DIR_ENV_VAR: &str = "SDKMAN_DIR";
const DEFAULT_SDKMAN_HOME: &str = ".sdkman";
const VERSION_FILE: &str = "var/version";

fn main() {
    let sdkman_dir = infer_sdkman_dir();
    let version = locate_version_file(sdkman_dir).and_then(read_content);
    match version {
        Some(content) => println!("\n{} {}", "SDKMAN".yellow(), content.yellow()),
        None => std::process::exit(exitcode::CONFIG),
    }
}

fn infer_sdkman_dir() -> PathBuf {
    match env::var(SDKMAN_DIR_ENV_VAR) {
        Ok(s) => Path::new(&s).to_path_buf(),
        Err(_) => fallback_sdkman_dir(),
    }
}

fn fallback_sdkman_dir() -> PathBuf {
    dirs::home_dir()
        .map(|dir| dir.join(DEFAULT_SDKMAN_HOME))
        .unwrap()
}

fn locate_version_file(base_dir: PathBuf) -> Option<PathBuf> {
    Some(Path::new(&base_dir).join(VERSION_FILE))
}

fn read_content(path: PathBuf) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
    .filter(|s| !s.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::io::Write;
    use std::path::Path;

    use serial_test::serial;
    use tempfile::NamedTempFile;

    use crate::{infer_sdkman_dir, read_content, SDKMAN_DIR_ENV_VAR};

    #[test]
    #[serial]
    fn should_infer_sdkman_dir_from_env_var() {
        let sdkman_dir = Path::new("/home/someone/.sdkman");
        env::set_var(SDKMAN_DIR_ENV_VAR, sdkman_dir);
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
    fn should_read_content_from_version_file() {
        let mut file = NamedTempFile::new().unwrap();
        file.write("5.0.0".as_bytes()).unwrap();
        let path = file.path().to_path_buf();
        let maybe_version = read_content(path);
        assert_eq!(maybe_version, Some("5.0.0".to_string()));
    }

    #[test]
    #[serial]
    fn should_fail_reading_content_from_empty_version_file() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        let maybe_version = read_content(path);
        assert_eq!(maybe_version, None);
    }
}
