use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use symlink::symlink_dir;

use tempfile::{Builder, TempDir};

pub struct TestCandidate {
    pub name: &'static str,
    pub versions: Vec<&'static str>,
    pub current_version: &'static str,
}

#[derive(Default)]
pub struct VirtualEnv {
    pub cli_version: String,
    pub native_version: String,
    pub candidate: Option<TestCandidate>,
    pub candidates: Vec<TestCandidate>,
}

pub fn virtual_env(virtual_env: VirtualEnv) -> TempDir {
    let sdkman_dir = prepare_sdkman_dir();
    let var_path = Path::new("var");

    // script version file
    write_file(
        sdkman_dir.path(),
        var_path,
        "version",
        virtual_env.cli_version,
    );

    // native version file
    write_file(
        sdkman_dir.path(),
        var_path,
        "version_native",
        virtual_env.native_version,
    );

    // Build a list of all candidates to register
    let mut all_candidates = Vec::new();

    // Add the single candidate if provided (for backward compatibility)
    if let Some(candidate) = &virtual_env.candidate {
        all_candidates.push(candidate);
    }

    // Add all candidates from the vector
    for candidate in &virtual_env.candidates {
        all_candidates.push(candidate);
    }

    // Write candidates to the candidates file
    let candidates_str = all_candidates
        .iter()
        .map(|c| c.name)
        .collect::<Vec<&str>>()
        .join(",");

    write_file(
        sdkman_dir.path(),
        Path::new("var"),
        "candidates",
        candidates_str,
    );

    // Process each candidate
    for candidate in all_candidates {
        for version in &candidate.versions {
            let location = format!("candidates/{}/{}/bin/", candidate.name, version);
            let content = format!(
                "\
#!/bin/bash
echo Running {} {}
",
                candidate.name, version
            );
            write_file(
                sdkman_dir.path(),
                Path::new(&location),
                candidate.name,
                content,
            );
        }

        let version_location = PathBuf::from(format!(
            "candidates/{}/{}",
            candidate.name, candidate.current_version
        ));
        let current_link_location = PathBuf::from(format!("candidates/{}/current", candidate.name));
        let absolute_version = sdkman_dir.path().join(version_location.as_path());
        let absolute_current_link = sdkman_dir.path().join(current_link_location.as_path());
        symlink_dir(absolute_version, absolute_current_link)
            .expect("cannot create current symlink");
    }

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
