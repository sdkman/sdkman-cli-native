extern crate core;

use std::ffi::OsString;
use std::path::Path;
use std::{env, fs};
use colored::Colorize;

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

fn infer_sdkman_dir() -> String {
    match env::var(SDKMAN_DIR_ENV_VAR) {
        Ok(s) => s,
        Err(_) => fallback_sdkman_dir(),
    }
}

fn fallback_sdkman_dir() -> String {
    dirs::home_dir()
        .map(|dir| dir.join(DEFAULT_SDKMAN_HOME))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn locate_version_file(base_dir: String) -> Option<OsString> {
    Some(
        Path::new(base_dir.as_str())
            .join(VERSION_FILE)
            .into_os_string(),
    )
}

fn read_content(path: OsString) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
    .filter(|s| !s.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::env;
    use std::io::Write;
    use tempfile::NamedTempFile;

    use crate::{infer_sdkman_dir, read_content, SDKMAN_DIR_ENV_VAR};

    #[test]
    #[serial]
    fn should_infer_sdkman_dir_from_env_var() {
        let sdkman_dir = "/home/someone/.sdkman";
        env::set_var(SDKMAN_DIR_ENV_VAR, sdkman_dir);
        assert_eq!(sdkman_dir, infer_sdkman_dir());
    }

    #[test]
    #[serial]
    fn should_infer_fallback_dir() {
        env::remove_var(SDKMAN_DIR_ENV_VAR);
        let actual_sdkman_dir = dirs::home_dir()
            .unwrap()
            .join(".sdkman")
            .to_str()
            .unwrap()
            .to_string();
        assert_eq!(actual_sdkman_dir, infer_sdkman_dir());
    }

    #[test]
    #[serial]
    fn should_read_content_from_version_file() {
        let mut file = NamedTempFile::new().unwrap();
        file.write("5.0.0".as_bytes()).unwrap();
        let os_string = file.path().as_os_str().to_os_string();
        let maybe_version = read_content(os_string);
        assert_eq!(maybe_version, Some("5.0.0".to_string()));
    }

    #[test]
    #[serial]
    fn should_fail_reading_content_from_empty_version_file() {
        let file = NamedTempFile::new().unwrap();
        let os_string = file.path().as_os_str().to_os_string();
        let maybe_version = read_content(os_string);
        assert_eq!(maybe_version, None);
    }
}
