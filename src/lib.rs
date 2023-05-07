pub mod constants {
    pub const SDKMAN_DIR_ENV_VAR: &str = "SDKMAN_DIR";
    pub const DEFAULT_SDKMAN_HOME: &str = ".sdkman";
    pub const VAR_DIR: &str = "var";
    pub const CANDIDATES_DIR: &str = "candidates";
    pub const CURRENT_DIR: &str = "current";
}

pub mod helpers {
    use directories::UserDirs;
    use std::path::PathBuf;
    use std::{env, fs};

    use crate::constants::{DEFAULT_SDKMAN_HOME, SDKMAN_DIR_ENV_VAR, VAR_DIR};

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

    pub fn absolute_path(base_dir: PathBuf, relative_path: PathBuf) -> PathBuf {
        let absolute_path = base_dir.join(relative_path);
        if absolute_path.exists() && absolute_path.is_absolute() {
            absolute_path
        } else {
            panic!("not a valid path: {}", absolute_path.to_str().unwrap())
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
        let relative_path = PathBuf::from(VAR_DIR).join(PathBuf::from("candidates"));
        let absolute_path = absolute_path(sdkman_dir, relative_path).to_owned();
        let panic = format!(
            "the candidates file is missing: {}",
            absolute_path.to_str().unwrap()
        );
        let content = read_file_content(absolute_path).expect(&panic);
        let line_str: &'static str = Box::leak(content.into_boxed_str());
        let mut fields = Vec::new();
        for field in line_str.split(',') {
            fields.push(field.trim());
        }

        fields
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::io::Write;
    use std::path::PathBuf;

    use serial_test::serial;
    use tempfile::NamedTempFile;

    use crate::constants::SDKMAN_DIR_ENV_VAR;
    use crate::helpers::infer_sdkman_dir;
    use crate::helpers::read_file_content;

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
