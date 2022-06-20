use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;

use tempfile::{Builder, TempDir};

pub fn virtual_env(version: String) -> TempDir {
    let sdkman_dir = prepare_sdkman_dir();
    init_var_dir(sdkman_dir.path());
    write_version_file(sdkman_dir.path(), version);
    return sdkman_dir;
}

pub fn prepare_sdkman_dir() -> TempDir {
    Builder::new().prefix(".sdkman-").tempdir().unwrap()
}

pub fn init_var_dir(temp_dir: &Path) {
    create_dir(temp_dir.join("var")).unwrap();
}

pub fn write_version_file(temp_dir: &Path, version: String) {
    let version_file_path = temp_dir.join("var").join("version");
    let mut version_file = File::create(version_file_path).expect("could not create file");
    write!(version_file, "{}", version.to_string()).unwrap();
}
