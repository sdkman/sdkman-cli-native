//! sdkman — small, opinionated utilities for working with SDKMAN-style directories.
//!
//! The file provides helpers to:
//! - infer the SDKMAN home directory (via `SDKMAN_DIR` or a home-based fallback)
//! - work with well-known SDKMAN layout constants
//!
//! # Examples
//! ```rust
//! use std::io;
//! use tempfile::TempDir;
//! use sdkman::utils::constants::SDKMAN_DIR;
//! use sdkman::utils::directory_utils::infer_sdkman_dir;
//!
//! # fn main() -> io::Result<()> {
//! // Prefer `SDKMAN_DIR` when present.
//! let tmp = TempDir::new()?;
//! unsafe { std::env::set_var(SDKMAN_DIR, tmp.path()); }
//!
//! let dir = infer_sdkman_dir().unwrap();
//! assert_eq!(dir, tmp.path().to_path_buf());
//!
//! // Cleanup to avoid leaking state across doctests.
//! unsafe { std::env::remove_var(SDKMAN_DIR); }
//! # Ok(())
//! # }
//! ```

use super::{
    constants::{DEFAULT_SDKMAN_HOME, SDKMAN_DIR},
    PathBuf,
};
use directories::UserDirs;
use std::env;

/// Attempts to determine the SDKMAN directory.
///
/// If the environment variable [`SDKMAN_DIR`] is set, its value is used.
/// Otherwise, falls back to [`fallback_sdkman_dir`].
///
/// # Examples
///
/// Set the environment variable and retrieve it:
/// ```
/// use std::{env, path::PathBuf};
/// use sdkman::utils::directory_utils::infer_sdkman_dir;
/// use sdkman::utils::constants::SDKMAN_DIR;
///
/// let temp_dir = tempfile::TempDir::new().unwrap();
/// unsafe {
///     env::set_var(SDKMAN_DIR, temp_dir.path());
/// }
///
/// let dir = infer_sdkman_dir().unwrap();
/// assert_eq!(dir, temp_dir.path().to_path_buf());
/// ```
///
/// Unset the variable to fall back:
/// ```
/// use std::env;
/// use sdkman::utils::directory_utils::{infer_sdkman_dir, fallback_sdkman_dir};
/// use sdkman::utils::constants::SDKMAN_DIR;
///
/// unsafe {
///     env::remove_var(SDKMAN_DIR);
/// }
/// let dir = infer_sdkman_dir().unwrap();
/// assert_eq!(dir, fallback_sdkman_dir());
/// ```
pub fn infer_sdkman_dir() -> Result<PathBuf, std::env::VarError> {
    env::var(SDKMAN_DIR)
        .map(PathBuf::from)
        .or_else(|_| Ok(fallback_sdkman_dir()))
}

/// Returns the default SDKMAN directory, based on the user's home directory.
///
/// # Examples
///
/// ```
/// use sdkman::utils::directory_utils::fallback_sdkman_dir;
/// use sdkman::utils::constants::DEFAULT_SDKMAN_HOME;
/// use directories::UserDirs;
///
/// let expected = UserDirs::new()
///     .unwrap()
///     .home_dir()
///     .join(DEFAULT_SDKMAN_HOME);
/// assert_eq!(fallback_sdkman_dir(), expected);
/// ```
#[doc(hidden)]
pub fn fallback_sdkman_dir() -> PathBuf {
    UserDirs::new()
        .map(|dir| dir.home_dir().join(DEFAULT_SDKMAN_HOME))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{fallback_sdkman_dir, infer_sdkman_dir, UserDirs, DEFAULT_SDKMAN_HOME, SDKMAN_DIR};
    use rstest::rstest;
    use std::{
        env::{remove_var, set_var, var_os},
        ffi::{OsStr, OsString},
        os::unix::ffi::OsStringExt,
        sync::{Mutex, MutexGuard, OnceLock},
    };
    use tempfile::TempDir;

    // --- global env lock ---
    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn lock_env() -> MutexGuard<'static, ()> {
        ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    // --- RAII guard to set/remove env vars ---
    struct ScopedEnvVar {
        key: &'static str,
        old: Option<OsString>,
    }

    impl ScopedEnvVar {
        fn set(key: &'static str, value: impl AsRef<OsStr>) -> Self {
            let old = std::env::var_os(key);
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

    #[test]
    fn fallback_is_home_join_default() {
        let _guard = lock_env();
        let expected = UserDirs::new()
            .unwrap()
            .home_dir()
            .join(DEFAULT_SDKMAN_HOME);
        assert_eq!(fallback_sdkman_dir(), expected);
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

    // non-unicode is considered as fallback rn
    #[cfg(unix)]
    #[test]
    fn infer_falls_back_when_env_is_not_unicode() {
        let _guard = lock_env();

        let invalid = OsString::from_vec(vec![0xFF, 0xFE, 0xFD]);
        let _sdkman = ScopedEnvVar::set(SDKMAN_DIR, invalid);

        let got = infer_sdkman_dir().unwrap();
        assert_eq!(got, fallback_sdkman_dir());
    }
}
