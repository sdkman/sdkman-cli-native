//! sdkman — small, opinionated utilities for working with SDKMAN-style directories.
//!
//! This module provides helpers to:
//! - infer the SDKMAN home directory (via `SDKMAN_DIR` or a home-based fallback)
//! - resolve the default SDKMAN directory in a cross-platform way

use super::{
    constants::{DEFAULT_SDKMAN_HOME, SDKMAN_DIR},
    PathBuf,
};
use directories::UserDirs;
use std::env;

/// Attempts to determine the SDKMAN directory.
///
/// If the environment variable [`SDKMAN_DIR`] is set (and is valid Unicode),
/// its value is used. Otherwise, falls back to [`fallback_sdkman_dir`].
///
/// # Examples
///
/// ```no_run
/// use std::env;
/// use sdkman::utils::constants::SDKMAN_DIR;
/// use sdkman::utils::directory_utils::infer_sdkman_dir;
///
/// let tmp = tempfile::TempDir::new().unwrap();
/// unsafe { env::set_var(SDKMAN_DIR, tmp.path()); }
///
/// let dir = infer_sdkman_dir().unwrap();
/// assert_eq!(dir, tmp.path().to_path_buf());
///
/// unsafe { env::remove_var(SDKMAN_DIR); }
/// ```
pub fn infer_sdkman_dir() -> Result<PathBuf, std::env::VarError> {
    env::var(SDKMAN_DIR)
        .map(PathBuf::from)
        // NotPresent / NotUnicode => fallback
        .or_else(|_| Ok(fallback_sdkman_dir()))
}

/// Returns the default SDKMAN directory.
///
/// Resolution order:
/// - Windows: `USERPROFILE`, then `HOMEDRIVE`+`HOMEPATH`, then `HOME`
/// - Unix: `HOME`
/// - Fallback: `directories::UserDirs`
///
/// This is intentionally env-first so CI and tests that override home behave
/// predictably across platforms.
///
/// # Examples
///
/// ```no_run
/// use sdkman::utils::directory_utils::fallback_sdkman_dir;
/// let dir = fallback_sdkman_dir();
/// assert!(dir.ends_with(".sdkman"));
/// ```
#[doc(hidden)]
pub fn fallback_sdkman_dir() -> PathBuf {
    if let Some(home) = env_home_dir() {
        return home.join(DEFAULT_SDKMAN_HOME);
    }

    UserDirs::new()
        .map(|dir| dir.home_dir().join(DEFAULT_SDKMAN_HOME))
        .unwrap_or_else(|| PathBuf::from(DEFAULT_SDKMAN_HOME))
}

fn env_home_dir() -> Option<PathBuf> {
    #[cfg(windows)]
    {
        if let Ok(p) = env::var("USERPROFILE") {
            if !p.is_empty() {
                return Some(PathBuf::from(p));
            }
        }

        if let (Ok(drive), Ok(path)) = (env::var("HOMEDRIVE"), env::var("HOMEPATH")) {
            if !drive.is_empty() && !path.is_empty() {
                return Some(PathBuf::from(format!("{drive}{path}")));
            }
        }

        // MSYS/Git-Bash sometimes sets HOME on Windows
        if let Ok(p) = env::var("HOME") {
            if !p.is_empty() {
                return Some(PathBuf::from(p));
            }
        }

        None
    }

    #[cfg(not(windows))]
    {
        if let Ok(p) = env::var("HOME") {
            if !p.is_empty() {
                return Some(PathBuf::from(p));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{fallback_sdkman_dir, infer_sdkman_dir, DEFAULT_SDKMAN_HOME, SDKMAN_DIR};
    use rstest::rstest;
    use std::{
        env::{remove_var, set_var, var_os},
        ffi::{OsStr, OsString},
        sync::{Mutex, MutexGuard, OnceLock},
    };
    use tempfile::TempDir;

    // --- global env lock ---
    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn lock_env() -> MutexGuard<'static, ()> {
        match ENV_LOCK.get_or_init(|| Mutex::new(())).lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(), // recover after a panic
        }
    }

    // --- RAII guard to set/remove env vars ---
    struct ScopedEnvVar {
        key: &'static str,
        old: Option<OsString>,
    }

    impl ScopedEnvVar {
        fn set(key: &'static str, value: impl AsRef<OsStr>) -> Self {
            let old = var_os(key);
            unsafe { set_var(key, value) };
            Self { key, old }
        }

        fn remove(key: &'static str) -> Self {
            let old = var_os(key);
            unsafe { remove_var(key) };
            Self { key, old }
        }
    }

    impl Drop for ScopedEnvVar {
        fn drop(&mut self) {
            unsafe {
                match self.old.take() {
                    Some(v) => set_var(self.key, v),
                    None => remove_var(self.key),
                }
            }
        }
    }

    // helper: what fallback expects as "home" given our precedence
    fn expected_env_home() -> Option<std::path::PathBuf> {
        #[cfg(windows)]
        {
            if let Some(v) = var_os("USERPROFILE") {
                if !v.is_empty() {
                    return Some(v.into());
                }
            }
            let drive = var_os("HOMEDRIVE");
            let path = var_os("HOMEPATH");
            if let (Some(d), Some(p)) = (drive, path) {
                if !d.is_empty() && !p.is_empty() {
                    return Some(std::path::PathBuf::from(format!(
                        "{}{}",
                        d.to_string_lossy(),
                        p.to_string_lossy()
                    )));
                }
            }
            if let Some(v) = var_os("HOME") {
                if !v.is_empty() {
                    return Some(v.into());
                }
            }
            None
        }

        #[cfg(not(windows))]
        {
            var_os("HOME").filter(|v| !v.is_empty()).map(Into::into)
        }
    }

    #[test]
    fn fallback_is_home_join_default() {
        let _guard = lock_env();

        // This test matches the implementation: env-home (if present) wins.
        if let Some(home) = expected_env_home() {
            assert_eq!(fallback_sdkman_dir(), home.join(DEFAULT_SDKMAN_HOME));
        } else {
            // no env home: last component should still be DEFAULT_SDKMAN_HOME
            assert!(fallback_sdkman_dir().ends_with(DEFAULT_SDKMAN_HOME));
        }
    }

    #[test]
    fn fallback_respects_overridden_home_dir() {
        let _guard = lock_env();

        #[cfg(windows)]
        const HOME_KEY: &str = "USERPROFILE";
        #[cfg(not(windows))]
        const HOME_KEY: &str = "HOME";

        let temp_home = TempDir::new().unwrap();
        let _home = ScopedEnvVar::set(HOME_KEY, temp_home.path());

        let expected = temp_home.path().join(DEFAULT_SDKMAN_HOME);
        assert_eq!(fallback_sdkman_dir(), expected);
    }

    #[rstest]
    #[case::env_set(true)]
    #[case::env_unset(false)]
    fn infer_sdkman_dir_env_cases(#[case] env_set: bool) {
        let _guard = lock_env();

        if env_set {
            let temp_dir = TempDir::new().unwrap();
            let expected = temp_dir.path().to_path_buf();
            let _sdkman = ScopedEnvVar::set(SDKMAN_DIR, temp_dir.path());

            let got = infer_sdkman_dir().unwrap();
            assert_eq!(got, expected);
        } else {
            let _sdkman = ScopedEnvVar::remove(SDKMAN_DIR);

            let got = infer_sdkman_dir().unwrap();
            assert_eq!(got, fallback_sdkman_dir());
        }
    }

    // Non-unicode env var is only meaningful on Unix.
    #[cfg(unix)]
    mod unix_only {
        use super::*;
        use std::os::unix::ffi::OsStringExt;

        #[test]
        fn infer_falls_back_when_env_is_not_unicode() {
            let _guard = lock_env();

            let invalid = OsString::from_vec(vec![0xFF, 0xFE, 0xFD]);
            let _sdkman = ScopedEnvVar::set(SDKMAN_DIR, invalid);

            let got = infer_sdkman_dir().unwrap();
            assert_eq!(got, fallback_sdkman_dir());
        }
    }
}
