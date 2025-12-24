//! SDKMAN helper utilities.
//!
//! These helpers are primarily intended for CLI flows:
//! - read the known candidates list from `$SDKMAN_DIR/var/candidates`
//! - validate a candidate name and installed version paths
//!
//! ## Error handling
//! These functions print a user-friendly error message and terminate the process
//! with exit code `1` when validation fails. If you need library-style error
//! handling, prefer implementing `try_*` variants that return `Result`.
use super::{
    constants::{CANDIDATES_DIR, CANDIDATES_FILE, VAR_DIR},
    file_utils::{check_file_exists, read_file_content},
    PathBuf,
};
use colored::Colorize;
use std::{path::Path, process};

/// Reads and parses the known SDKMAN candidates from `$SDKMAN_DIR/var/candidates`.
///
/// The candidates file is expected to be a comma-separated list (e.g. `"java, maven, gradle"`).
/// Whitespace around each entry is trimmed and empty entries are ignored.
///
/// ## Error handling
/// If the candidates file is missing or unreadable, this function prints a message to stderr
/// and terminates the process with exit code `1`.
///
/// # Examples
///
/// ```rust
/// use std::io::Write;
/// use tempfile::TempDir;
///
/// use sdkman::utils::constants::{VAR_DIR, CANDIDATES_FILE};
/// use sdkman::utils::helpers::known_candidates;
///
/// let sdkman_dir = TempDir::new().unwrap();
/// let var_dir = sdkman_dir.path().join(VAR_DIR);
/// std::fs::create_dir_all(&var_dir).unwrap();
///
/// let candidates_path = var_dir.join(CANDIDATES_FILE);
/// let mut f = std::fs::File::create(&candidates_path).unwrap();
/// writeln!(f, " java, maven,  ,gradle ").unwrap();
///
/// let candidates = known_candidates(sdkman_dir.path());
/// assert_eq!(candidates, vec!["java", "maven", "gradle"]);
/// ```
pub fn known_candidates(sdkman_dir: impl AsRef<Path>) -> Vec<String> {
    let candidates_path = sdkman_dir.as_ref().join(VAR_DIR).join(CANDIDATES_FILE);

    let verified_path = check_file_exists(&candidates_path).unwrap_or_else(|e| {
        eprintln!(
            "{} {} ({})",
            "the candidates file is missing:".red(),
            candidates_path.display(),
            e
        );
        process::exit(1);
    });

    let content = read_file_content(&verified_path).unwrap_or_else(|e| {
        eprintln!(
            "{} {} ({})",
            "failed to read candidates file:".red(),
            verified_path.display(),
            e
        );
        process::exit(1);
    });

    content
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

/// Validates that `candidate` exists in `all_candidates`.
///
/// Returns `candidate` as an owned `String` on success.
///
/// ## Error handling
/// If `candidate` is not present in `all_candidates`, this function prints an error to stderr
/// and terminates the process with exit code `1`.
///
/// # Examples
///
/// ```rust
/// use sdkman::utils::helpers::validate_candidate;
///
/// let all = vec!["java".to_string(), "maven".to_string()];
///
/// let ok = validate_candidate(&all, "java");
/// assert_eq!(ok, "java");
/// ```
pub fn validate_candidate(all_candidates: &[String], candidate: &str) -> String {
    let ok = all_candidates.iter().any(|c| c == candidate);
    if !ok {
        eprintln!("{} is not a valid candidate.", candidate.bold());
        process::exit(1);
    }
    candidate.to_string()
}

/// Validates that a version directory exists under
/// `$SDKMAN_DIR/candidates/<candidate>/<version>`.
///
/// Returns the resolved version path on success.
///
/// ## Error handling
/// If the version directory does not exist, this function prints an error to stderr
/// and terminates the process with exit code `1`.
///
/// # Examples
///
/// ```rust
/// use tempfile::TempDir;
///
/// use sdkman::utils::constants::CANDIDATES_DIR;
/// use sdkman::utils::helpers::validate_version_path;
///
/// let sdkman_dir = TempDir::new().unwrap();
/// let version_dir = sdkman_dir
///     .path()
///     .join(CANDIDATES_DIR)
///     .join("java")
///     .join("17.0.9-tem");
/// std::fs::create_dir_all(&version_dir).unwrap();
///
/// let got = validate_version_path(sdkman_dir.path(), "java", "17.0.9-tem");
/// assert_eq!(got, version_dir);
/// ```
pub fn validate_version_path(
    base_dir: impl AsRef<Path>,
    candidate: &str,
    version: &str,
) -> PathBuf {
    let version_path = base_dir
        .as_ref()
        .join(CANDIDATES_DIR)
        .join(candidate)
        .join(version);

    if version_path.is_dir() {
        version_path
    } else {
        eprintln!(
            "{} {} is not installed on your system",
            candidate.bold(),
            version.bold()
        );
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::constants::{CANDIDATES_DIR, CANDIDATES_FILE, VAR_DIR},
        known_candidates, validate_candidate, validate_version_path,
    };
    use rstest::rstest;
    use std::{io::Write, process::Command};
    use tempfile::TempDir;

    // --- success-path unit tests ---
    #[rstest]
    #[case("java,maven,gradle", vec!["java", "maven", "gradle"])]
    #[case(" java, maven,  ,gradle ", vec!["java", "maven", "gradle"])]
    #[case("kotlin", vec!["kotlin"])]
    #[case(",, ,", Vec::<&str>::new())]
    fn known_candidates_parses_csv(#[case] input: &str, #[case] expected: Vec<&str>) {
        let sdkman_dir = TempDir::new().unwrap();
        let var_dir = sdkman_dir.path().join(VAR_DIR);
        std::fs::create_dir_all(&var_dir).unwrap();

        let candidates_path = var_dir.join(CANDIDATES_FILE);
        let mut f = std::fs::File::create(&candidates_path).unwrap();
        writeln!(f, "{input}").unwrap();

        let got = known_candidates(sdkman_dir.path());
        let expected: Vec<String> = expected.into_iter().map(str::to_string).collect();
        assert_eq!(got, expected);
    }

    #[test]
    fn validate_candidate_returns_owned_string_when_valid() {
        let all = vec!["java".to_string(), "maven".to_string()];
        let got = validate_candidate(&all, "java");
        assert_eq!(got, "java");
    }

    #[test]
    fn validate_version_path_returns_path_when_installed() {
        let sdkman_dir = TempDir::new().unwrap();
        let expected = sdkman_dir
            .path()
            .join(CANDIDATES_DIR)
            .join("java")
            .join("17.0.9-tem");

        std::fs::create_dir_all(&expected).unwrap();

        let got = validate_version_path(sdkman_dir.path(), "java", "17.0.9-tem");
        assert_eq!(got, expected);
    }

    // --- exit-path tests (exit cases) ---
    //
    // `exit(1)` paths are tested by spawning this test binary as a child process.
    fn run_child(filter: &str) -> std::process::ExitStatus {
        let exe = std::env::current_exe().unwrap();
        Command::new(exe)
            .env("SDKRAN_HELPERS_EXIT_TESTS", "1")
            // Run only tests matching this substring filter
            .arg(filter)
            // Stable output, avoid capturing
            .arg("--nocapture")
            .status()
            .unwrap()
    }

    // == parent assertions ==
    #[test]
    fn exits_when_candidates_file_missing() {
        let status = run_child("helpers_exit_child_candidates_file_missing");
        assert_eq!(status.code(), Some(1));
    }

    #[test]
    fn exits_when_candidate_invalid() {
        let status = run_child("helpers_exit_child_candidate_invalid");
        assert_eq!(status.code(), Some(1));
    }

    #[test]
    fn exits_when_version_not_installed() {
        let status = run_child("helpers_exit_child_version_not_installed");
        assert_eq!(status.code(), Some(1));
    }

    // == child tests ==
    #[test]
    fn helpers_exit_child_candidates_file_missing() {
        if std::env::var_os("SDKRAN_HELPERS_EXIT_TESTS").is_none() {
            return;
        }
        let sdkman_dir = TempDir::new().unwrap();
        // no var/candidates file created -> should exit(1)
        let _ = known_candidates(sdkman_dir.path());
    }

    #[test]
    fn helpers_exit_child_candidate_invalid() {
        if std::env::var_os("SDKRAN_HELPERS_EXIT_TESTS").is_none() {
            return;
        }
        let all = vec!["java".to_string(), "maven".to_string()];
        // not in list -> should exit(1)
        let _ = validate_candidate(&all, "not-a-candidate");
    }

    #[test]
    fn helpers_exit_child_version_not_installed() {
        if std::env::var_os("SDKRAN_HELPERS_EXIT_TESTS").is_none() {
            return;
        }
        let sdkman_dir = TempDir::new().unwrap();
        // path doesn't exist -> should exit(1)
        let _ = validate_version_path(sdkman_dir.path(), "java", "0.0.0-not-installed");
    }
}
