use std::{
    fs,
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
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
    use std::{Write}
}
