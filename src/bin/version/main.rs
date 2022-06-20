extern crate core;

use std::path::{Path};
use std::{env, fs};
use std::ffi::OsString;

const SDKMAN_DIR_ENV_VAR: &str = "SDKMAN_DIR";
const VERSION_FILE: &str = "var/version";

fn main() {
    let version = infer_sdkman_dir()
        .and_then(prepare_version_file_path)
        .and_then(read_content);

    match version {
        Some(content) => println!("\nSDKMAN {}", content),
        None => panic!(),
    }
}

fn infer_sdkman_dir() -> Option<String> {
    match env::var(SDKMAN_DIR_ENV_VAR) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

fn prepare_version_file_path(base_dir: String) -> Option<OsString> {
    Some(Path::new(base_dir.as_str()).join(VERSION_FILE).into_os_string())
}

fn read_content(path: OsString) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}
