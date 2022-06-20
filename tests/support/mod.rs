use std::fmt::Error;
use std::fs::{create_dir, File};
use std::io::Write;
use tempfile::{Builder, TempDir};

pub fn virtual_env(version: String) -> Result<TempDir, Error> {
    let sdkman_dir = Builder::new().prefix(".sdkman-").tempdir().unwrap();
    create_dir(sdkman_dir.path().join("var")).unwrap();
    let version_file_path = sdkman_dir.path().join("var").join("version");
    let mut version_file = File::create(version_file_path).expect("could not create file");
    write!(version_file, "{}", version.to_string()).unwrap();
    return Ok(sdkman_dir);
}