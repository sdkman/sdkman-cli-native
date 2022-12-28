use std::fs;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

use tempfile::{Builder, TempDir};

pub struct TestCandidate {
    pub name: String,
    pub version: String,
}

#[derive(Default)]
pub struct VirtualEnv {
    pub cli_version: String,
    pub native_version: String,
    pub candidate: Option<TestCandidate>,
    pub known_candidates: Vec<String>
}

pub fn virtual_env(virtual_env: VirtualEnv) -> TempDir {
    let sdkman_dir = prepare_sdkman_dir();
    let var_path = Path::new("var");
    write_file(sdkman_dir.path(), var_path, "version", virtual_env.cli_version);
    write_file(
        sdkman_dir.path(),
        var_path,
        "version_native",
        virtual_env.native_version,
    );

    let known_candidates = virtual_env.known_candidates.join(",");
    write_file(sdkman_dir.path(), Path::new("var"), "candidates", known_candidates);

    virtual_env.candidate.map(|c| {
        let location = format!("candidates/{}/{}/bin/", c.name, c.version);
        let content = format!(
            "\
#!/bin/bash
echo Running {}
",
            c.name
        );
        let file = write_file(sdkman_dir.path(), Path::new(&location), c.name.as_str(), content);
        let mut perms = fs::metadata(file.as_path())
            .expect("could not access file metadata")
            .permissions();
        perms.set_mode(0o744);
        fs::set_permissions(file, perms).expect("could not set file permissions");
    });

    return sdkman_dir;
}

pub fn prepare_sdkman_dir() -> TempDir {
    Builder::new()
        .prefix(".sdkman-")
        .tempdir()
        .expect("could not prepare SDKMAN_DIR")
}

pub fn write_file(
    temp_dir: &Path,
    relative_path: &Path,
    file_name: &str,
    content: String,
) -> PathBuf {
    let absolute_path = temp_dir.join(relative_path);
    create_dir_all(absolute_path.to_owned()).expect("could not create nested dirs");

    let file_path = absolute_path.join(file_name);
    let mut file = File::create(&file_path).expect("could not create file");
    write!(file, "{}", content.to_string()).expect("could not write to file");

    file_path
}
