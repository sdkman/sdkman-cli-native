use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;

use tempfile::{Builder, TempDir};

pub fn virtual_env(version: String, native_version: String) -> TempDir {
    let sdkman_dir = prepare_sdkman_dir();
    init_var_dir(sdkman_dir.path());

    let version_file = Path::new("var/version");
    write_file(sdkman_dir.path(), version_file, version.to_owned());

    let native_version_file = Path::new("var/version_native");
    write_file(
        sdkman_dir.path(),
        native_version_file,
        native_version.to_owned(),
    );

    return sdkman_dir;
}

pub fn prepare_sdkman_dir() -> TempDir {
    Builder::new().prefix(".sdkman-").tempdir().unwrap()
}

pub fn init_var_dir(temp_dir: &Path) {
    create_dir(temp_dir.join("var")).unwrap();
}

pub fn write_file(temp_dir: &Path, relative_path: &Path, content: String) {
    let absolute_path = temp_dir.join(relative_path);
    let mut version_file = File::create(absolute_path).expect("could not create file");
    write!(version_file, "{}", content.to_string()).unwrap();
}
