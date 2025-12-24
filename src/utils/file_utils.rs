//! File utilities for common path validation and file content helpers.
//!
//! This file provides small, opinionated helpers around common filesystem tasks.
//!
//! # Examples
//! ```rust
//! use std::io;
//! use tempfile::NamedTempFile;
//! use sdkman::utils::file_utils::{check_file_exists, read_file_content};
//!
//! # fn main() -> io::Result<()> {
//! let mut file = NamedTempFile::new()?;
//! std::io::Write::write_all(&mut file, b"5.9.0\n")?;
//!
//! let path = check_file_exists(file.path())?;
//! let version = read_file_content(&path)?;
//! assert_eq!(version, "5.9.0");
//! # Ok(())
//! # }
//! ```
use super::PathBuf;
use std::{
    fs,
    io::{Error, ErrorKind},
    path::Path,
};

/// Checks whether the given path exists and is a regular file.
///
/// # Examples
///
/// ```rust
/// use std::io;
/// use std::path::Path;
/// use tempfile::NamedTempFile;
/// use sdkman::utils::file_utils::check_file_exists;
///
/// # fn main() -> io::Result<()> {
/// let file = NamedTempFile::new()?;
/// let path = file.path();
///
/// // Should succeed
/// let validated = check_file_exists(path)?;
/// assert_eq!(validated, path.to_path_buf());
///
/// // Should fail for a non-existent file
/// let bad_path = Path::new("does_not_exist.txt");
/// let err = check_file_exists(bad_path).unwrap_err();
/// assert_eq!(err.kind(), io::ErrorKind::NotFound);
/// # Ok(())
/// # }
/// ```
pub fn check_file_exists<P: AsRef<Path>>(path: P) -> Result<PathBuf, Error> {
    let path_buf = path.as_ref().to_path_buf();
    if path_buf.exists() && path_buf.is_file() {
        Ok(path_buf)
    } else {
        Err(Error::new(
            ErrorKind::NotFound,
            format!("Not a valid file path: {}", path_buf.display()),
        ))
    }
}

/// Reads the contents of a file, trimming whitespace, and returns an error
/// if the file is empty.
///
/// # Examples
///
/// ```rust
/// use std::io::{self, Write};
/// use tempfile::NamedTempFile;
/// use sdkman::utils::file_utils::read_file_content;
///
/// # fn main() -> io::Result<()> {
/// // Write some data to a temp file
/// let mut file = NamedTempFile::new()?;
/// writeln!(file, "5.9.0")?;
///
/// // Read it back
/// let version = read_file_content(file.path())?;
/// assert_eq!(version, "5.9.0");
///
/// // Empty file should return InvalidData error
/// let empty_file = NamedTempFile::new()?;
/// let err = read_file_content(empty_file.path()).unwrap_err();
/// assert_eq!(err.kind(), io::ErrorKind::InvalidData);
/// # Ok(())
/// # }
/// ```
pub fn read_file_content<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let content = fs::read_to_string(path)?;
    let trimmed = content.trim().to_string();
    if trimmed.is_empty() {
        Err(Error::new(ErrorKind::InvalidData, "File is empty"))
    } else {
        Ok(trimmed)
    }
}

#[cfg(test)]
mod tests {
    use super::{check_file_exists, read_file_content};
    use rstest::rstest;
    use std::{
        io::{Error, ErrorKind, Result, Write},
        path::{Path, PathBuf},
    };
    use tempfile::{tempdir, NamedTempFile};

    #[cfg(unix)]
    use std::os::unix::fs as unix_fs;

    #[cfg(windows)]
    use std::os::windows::fs as windows_fs;

    fn assert_kind_is_one_of(err: &Error, kinds: &[ErrorKind]) {
        let got = err.kind();
        assert!(
            kinds.contains(&got),
            "expected error kind to be one of {:?}, got {:?} (err: {})",
            kinds,
            got,
            err
        );
    }

    // file existence checks
    enum PathInput<'a> {
        PathRef(&'a Path),
        PathBuf(PathBuf),
        Str(&'a str),
    }

    type MakeInput = for<'a> fn(&'a Path) -> PathInput<'a>;

    fn as_path_ref<'a>(p: &'a Path) -> PathInput<'a> {
        PathInput::PathRef(p)
    }
    fn as_path_buf<'a>(p: &'a Path) -> PathInput<'a> {
        PathInput::PathBuf(p.to_path_buf())
    }
    fn as_str<'a>(p: &'a Path) -> PathInput<'a> {
        PathInput::Str(p.to_str().expect("temp path should be valid UTF-8"))
    }

    #[rstest]
    #[case::path_ref(as_path_ref)]
    #[case::path_buf(as_path_buf)]
    #[case::str_path(as_str)]
    fn check_file_exists_ok_param(#[case] make_input: MakeInput) -> Result<()> {
        let file = NamedTempFile::new()?;
        let expected = file.path().to_path_buf();

        let input = make_input(file.path());
        let validated = match input {
            PathInput::PathRef(p) => check_file_exists(p)?,
            PathInput::PathBuf(pb) => check_file_exists(pb)?,
            PathInput::Str(s) => check_file_exists(s)?,
        };

        assert_eq!(validated, expected);
        Ok(())
    }

    #[derive(Clone, Copy, Debug)]
    enum InvalidCheckPath {
        MissingFile,
        Directory,
    }

    #[rstest]
    #[case::missing_file(InvalidCheckPath::MissingFile)]
    #[case::directory(InvalidCheckPath::Directory)]
    fn check_file_exists_err_cases(#[case] kind: InvalidCheckPath) -> Result<()> {
        let dir = tempdir()?;
        let path = match kind {
            InvalidCheckPath::MissingFile => dir.path().join("nope_this_file_should_not_exist"),
            InvalidCheckPath::Directory => dir.path().to_path_buf(),
        };

        let err = check_file_exists(&path).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert!(err.to_string().contains("Not a valid file path"));
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn check_file_exists_ok_with_symlink_to_file() -> Result<()> {
        let file = NamedTempFile::new()?;
        let dir = tempdir()?;
        let link_path = dir.path().join("link_to_file");

        unix_fs::symlink(file.path(), &link_path)?;

        let validated = check_file_exists(&link_path)?;
        assert_eq!(validated, link_path);
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn check_file_exists_err_with_broken_symlink() -> Result<()> {
        let dir = tempdir()?;
        let target = dir.path().join("target_that_does_not_exist");
        let link = dir.path().join("broken_link");

        unix_fs::symlink(&target, &link)?;

        let err = check_file_exists(&link).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
        Ok(())
    }

    // On Windows, creating symlinks can require admin privileges or Developer Mode.
    // So: attempt to create the symlink, and if it fails, skip the test.
    #[cfg(windows)]
    #[test]
    fn check_file_exists_ok_with_symlink_to_file() -> Result<()> {
        let file = NamedTempFile::new()?;
        let dir = tempdir()?;
        let link_path = dir.path().join("link_to_file");

        if windows_fs::symlink_file(file.path(), &link_path).is_err() {
            eprintln!("skipping symlink test (no permission / developer mode not enabled)");
            return Ok(());
        }

        let validated = check_file_exists(&link_path)?;
        assert_eq!(validated, link_path);
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn check_file_exists_err_with_broken_symlink() -> Result<()> {
        let dir = tempdir()?;
        let target = dir.path().join("target_that_does_not_exist");
        let link = dir.path().join("broken_link");

        if windows_fs::symlink_file(&target, &link).is_err() {
            eprintln!("skipping symlink test (no permission / developer mode not enabled)");
            return Ok(());
        }

        let err = check_file_exists(&link).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
        Ok(())
    }

    // read_file_content tests
    #[rstest]
    #[case::trims_newline("5.9.0\n", "5.9.0")]
    #[case::trims_surrounding_ws("   hello world \n\n\t", "hello world")]
    #[case::preserves_internal_ws("  a   b   c  ", "a   b   c")]
    #[case::unicode(" こんにちは世界 \n", "こんにちは世界")]
    fn read_file_content_ok_cases(#[case] input: &str, #[case] expected: &str) -> Result<()> {
        let mut file = NamedTempFile::new()?;
        write!(file, "{input}")?;

        let content = read_file_content(file.path())?;
        assert_eq!(content, expected);
        Ok(())
    }

    #[rstest]
    #[case::empty(None)]
    #[case::whitespace_only(Some(" \n\t  \r\n"))]
    fn read_file_content_err_emptyish(#[case] contents: Option<&str>) -> Result<()> {
        let mut file = NamedTempFile::new()?;
        if let Some(s) = contents {
            write!(file, "{s}")?;
        }

        let err = read_file_content(file.path()).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert_eq!(err.to_string(), "File is empty");
        Ok(())
    }

    #[test]
    fn read_file_content_err_missing_file() -> Result<()> {
        let dir = tempdir()?;
        let missing = dir.path().join("definitely_missing.txt");

        let err = read_file_content(&missing).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::NotFound);
        Ok(())
    }

    #[test]
    fn read_file_content_err_when_path_is_directory() -> Result<()> {
        let dir = tempdir()?;
        let err = read_file_content(dir.path()).unwrap_err();

        // OS/filesystem dependent.
        // Windows can report PermissionDenied/Other for "is a directory" attempts.
        assert_kind_is_one_of(
            &err,
            &[
                ErrorKind::IsADirectory,
                ErrorKind::PermissionDenied,
                ErrorKind::Other,
            ],
        );

        Ok(())
    }
}
