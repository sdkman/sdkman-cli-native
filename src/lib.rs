pub mod utils;

pub mod helpers {
    use colored::Colorize;
    use directories::UserDirs;
    use std::path::PathBuf;
    use std::{env, fs, process};

    use crate::utils::constants::{
        CANDIDATES_DIR, CANDIDATES_FILE, DEFAULT_SDKMAN_HOME, SDKMAN_DIR_ENV_VAR, VAR_DIR,
    };

    pub fn infer_sdkman_dir() -> PathBuf {
        match env::var(SDKMAN_DIR_ENV_VAR) {
            Ok(s) => PathBuf::from(s),
            Err(_) => fallback_sdkman_dir(),
        }
    }

    fn fallback_sdkman_dir() -> PathBuf {
        UserDirs::new()
            .map(|dir| dir.home_dir().join(DEFAULT_SDKMAN_HOME))
            .unwrap()
    }

    pub fn check_file_exists(path: PathBuf) -> PathBuf {
        if path.exists() && path.is_file() {
            path
        } else {
            panic!("not a valid path: {}", path.to_str().unwrap())
        }
    }

    pub fn read_file_content(path: PathBuf) -> Option<String> {
        match fs::read_to_string(path) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
    }

    pub fn known_candidates<'a>(sdkman_dir: PathBuf) -> Vec<&'static str> {
        let absolute_path = sdkman_dir.join(VAR_DIR).join(CANDIDATES_FILE);
        let verified_path = check_file_exists(absolute_path);
        let panic = format!(
            "the candidates file is missing: {}",
            verified_path.to_str().unwrap()
        );
        let content = read_file_content(verified_path).expect(&panic);
        let line_str: &'static str = Box::leak(content.into_boxed_str());
        let mut fields = Vec::new();
        for field in line_str.split(',') {
            fields.push(field.trim());
        }

        fields
    }

    pub fn validate_candidate(all_candidates: Vec<&str>, candidate: &str) -> String {
        if !all_candidates.contains(&candidate) {
            eprintln!("{} is not a valid candidate.", candidate.bold());
            process::exit(1);
        } else {
            candidate.to_string()
        }
    }

    pub fn validate_version_path(base_dir: PathBuf, candidate: &str, version: &str) -> PathBuf {
        let version_path = base_dir.join(CANDIDATES_DIR).join(candidate).join(version);
        if version_path.exists() && version_path.is_dir() {
            version_path
        } else {
            eprintln!(
                "{} {} is not installed on your system",
                candidate.bold(),
                version.bold()
            );
            process::exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::io::Write;
    use std::path::PathBuf;

    use serial_test::serial;
    use tempfile::NamedTempFile;

    use crate::helpers::infer_sdkman_dir;
    use crate::helpers::read_file_content;
    use crate::utils::constants::SDKMAN_DIR_ENV_VAR;

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
    fn should_fail_reading_file_content_from_empty_file() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path().to_path_buf();
        let maybe_version = read_file_content(path);
        assert_eq!(maybe_version, None);
    }
}
