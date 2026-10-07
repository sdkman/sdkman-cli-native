pub mod constants {
    pub const CANDIDATES_DIR: &str = "candidates";
    pub const CANDIDATES_FILE: &str = "candidates";
    pub const CONFIG_FILE: &str = "config";
    pub const CURRENT_DIR: &str = "current";
    pub const DEFAULT_SDKMAN_HOME: &str = ".sdkman";
    pub const ETC_DIR: &str = "etc";
    pub const SDKMAN_DIR_ENV_VAR: &str = "SDKMAN_DIR";
    pub const TMP_DIR: &str = "tmp";
    pub const VAR_DIR: &str = "var";
}

pub mod helpers {
    use colored::Colorize;
    use directories::UserDirs;
    use std::path::PathBuf;
    use std::{env, fs, io, process};

    use crate::constants::{
        CANDIDATES_DIR, CANDIDATES_FILE, DEFAULT_SDKMAN_HOME, SDKMAN_DIR_ENV_VAR, VAR_DIR,
    };
    use crate::ui::{self, CliError};

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

    pub fn known_candidates(sdkman_dir: PathBuf) -> Result<Vec<String>, CliError> {
        let absolute_path = sdkman_dir.join(VAR_DIR).join(CANDIDATES_FILE);
        let content = fs::read_to_string(&absolute_path).map_err(|error| CliError {
            message: format!(
                "cannot read {}: {}",
                ui::path(&absolute_path),
                os_reason(&error)
            ),
            hints: vec![format!("run {}", ui::cmd("sdk update"))],
        })?;
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Err(CliError {
                message: format!("no SDKs found in {}", ui::path(&absolute_path)),
                hints: vec![format!("run {}", ui::cmd("sdk update"))],
            });
        }
        Ok(trimmed
            .split(',')
            .map(|field| field.trim().to_string())
            .collect())
    }

    pub fn require_candidate(
        all_candidates: &[String],
        candidate: &str,
    ) -> Result<String, CliError> {
        if all_candidates.iter().any(|known| known == candidate) {
            Ok(candidate.to_string())
        } else {
            Err(CliError {
                message: format!("unknown SDK {}", ui::sdk(candidate)),
                hints: vec![format!("run {} to see all SDKs", ui::cmd("sdk list"))],
            })
        }
    }

    pub fn require_version_path(
        base_dir: PathBuf,
        candidate: &str,
        version: &str,
    ) -> Result<PathBuf, CliError> {
        let version_path = base_dir.join(CANDIDATES_DIR).join(candidate).join(version);
        if version_path.exists() && version_path.is_dir() {
            Ok(version_path)
        } else {
            Err(CliError {
                message: format!("{} is not installed", ui::sdk_version(candidate, version)),
                hints: vec![format!(
                    "run {}",
                    ui::cmd(&format!("sdk install {candidate} {version}"))
                )],
            })
        }
    }

    fn os_reason(error: &io::Error) -> String {
        let full = error.to_string();
        let reason = full.split(" (os error ").next().unwrap_or(&full);
        let mut chars = reason.chars();
        match chars.next() {
            Some(first) => first.to_lowercase().chain(chars).collect(),
            None => String::new(),
        }
    }

    pub fn validate_candidate(all_candidates: Vec<String>, candidate: &str) -> String {
        if !all_candidates.iter().any(|known| known == candidate) {
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

pub mod ui {
    use anstream::ColorChoice;
    use anstyle::{AnsiColor, Style};
    use std::env;
    use std::fmt::{self, Display};
    use std::fs;
    use std::io::Write;
    use std::path::Path;

    use crate::constants::{CONFIG_FILE, ETC_DIR};

    const CLICOLOR_FORCE_ENV_VAR: &str = "CLICOLOR_FORCE";
    const NO_COLOR_ENV_VAR: &str = "NO_COLOR";
    const CLICOLOR_ENV_VAR: &str = "CLICOLOR";
    const COLOUR_ENABLE_KEY: &str = "sdkman_colour_enable";

    pub fn colour_choice(
        clicolor_force: Option<&str>,
        no_color: Option<&str>,
        clicolor: Option<&str>,
        colour_enable: Option<bool>,
    ) -> ColorChoice {
        if clicolor_force.is_some_and(|value| value != "0") {
            return ColorChoice::Always;
        }
        let no_color_set = no_color.is_some_and(|value| !value.is_empty());
        let clicolor_off = clicolor == Some("0");
        let colour_disabled = colour_enable == Some(false);
        if no_color_set || clicolor_off || colour_disabled {
            return ColorChoice::Never;
        }
        ColorChoice::Auto
    }

    pub fn init(sdkman_dir: &Path) {
        let clicolor_force = env::var(CLICOLOR_FORCE_ENV_VAR).ok();
        let no_color = env::var(NO_COLOR_ENV_VAR).ok();
        let clicolor = env::var(CLICOLOR_ENV_VAR).ok();
        let colour_enable = read_colour_enable(sdkman_dir);
        colour_choice(
            clicolor_force.as_deref(),
            no_color.as_deref(),
            clicolor.as_deref(),
            colour_enable,
        )
        .write_global();
    }

    fn read_colour_enable(sdkman_dir: &Path) -> Option<bool> {
        let config_path = sdkman_dir.join(ETC_DIR).join(CONFIG_FILE);
        let content = fs::read_to_string(config_path).ok()?;
        content.lines().find_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (key, value) = line.split_once('=')?;
            (key.trim() == COLOUR_ENABLE_KEY).then(|| value.trim() != "false")
        })
    }

    pub fn success(text: impl Display) {
        let check = Style::new().fg_color(Some(AnsiColor::Green.into()));
        let mut out = anstream::stderr();
        let _ = writeln!(out, "{}✓{} {text}", check.render(), check.render_reset());
    }

    pub fn info(text: impl Display) {
        let mut out = anstream::stderr();
        let _ = writeln!(out, "{text}");
    }

    pub fn warning(text: impl Display) {
        let label = Style::new().fg_color(Some(AnsiColor::Yellow.into()));
        let mut out = anstream::stderr();
        let _ = writeln!(
            out,
            "{}warning:{} {text}",
            label.render(),
            label.render_reset()
        );
    }

    pub fn error(text: impl Display) {
        let label = Style::new().bold().fg_color(Some(AnsiColor::Red.into()));
        let mut out = anstream::stderr();
        let _ = writeln!(
            out,
            "{}error:{} {text}",
            label.render(),
            label.render_reset()
        );
    }

    pub fn hint(text: impl Display) {
        let label = Style::new().dimmed();
        let mut out = anstream::stderr();
        let _ = writeln!(
            out,
            "  {}hint:{} {text}",
            label.render(),
            label.render_reset()
        );
    }

    pub fn value(text: impl Display) {
        let mut out = anstream::stdout();
        let _ = writeln!(out, "{text}");
    }

    pub fn table(title: &str, rows: &[(String, String)]) {
        let bold = Style::new().bold();
        let mut out = anstream::stdout();
        let _ = writeln!(out, "{}{title}{}", bold.render(), bold.render_reset());
        let width = rows.iter().map(|(name, _)| name.len()).max().unwrap_or(0);
        for (name, version) in rows {
            let _ = writeln!(out, "  {name:<width$}  {version}");
        }
    }

    pub fn sdk(name: &str) -> impl Display {
        styled(Style::new().bold(), name.to_string())
    }

    pub fn sdk_version(sdk: &str, version: &str) -> impl Display {
        styled(Style::new().bold(), format!("{sdk} {version}"))
    }

    pub fn cmd(text: &str) -> impl Display {
        styled(
            Style::new().fg_color(Some(AnsiColor::Cyan.into())),
            text.to_string(),
        )
    }

    pub fn path(path: &Path) -> impl Display {
        match dirs::home_dir().and_then(|home| path.strip_prefix(home).ok()) {
            Some(rest) if rest.as_os_str().is_empty() => "~".to_string(),
            Some(rest) => format!("~/{}", rest.display()),
            None => path.display().to_string(),
        }
    }

    fn styled(style: Style, inner: String) -> impl Display {
        Styled { style, inner }
    }

    struct Styled {
        style: Style,
        inner: String,
    }

    impl Display for Styled {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}{}{}",
                self.style.render(),
                self.inner,
                self.style.render_reset()
            )
        }
    }

    #[derive(Debug)]
    pub struct CliError {
        pub message: String,
        pub hints: Vec<String>,
    }

    impl CliError {
        pub fn report(&self) {
            error(&self.message);
            for line in &self.hints {
                hint(line);
            }
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

    #[test]
    fn colour_choice_follows_r4_order() {
        use crate::ui::colour_choice;
        use anstream::ColorChoice;

        let cases = [
            (Some("1"), Some("1"), None, None, ColorChoice::Always),
            (Some("1"), None, None, Some(false), ColorChoice::Always),
            (Some("0"), None, None, None, ColorChoice::Auto),
            (Some("0"), Some("1"), None, None, ColorChoice::Never),
            (None, Some("1"), None, None, ColorChoice::Never),
            (None, Some(""), None, None, ColorChoice::Auto),
            (None, None, Some("0"), None, ColorChoice::Never),
            (None, None, Some("1"), None, ColorChoice::Auto),
            (None, None, None, Some(false), ColorChoice::Never),
            (None, None, None, Some(true), ColorChoice::Auto),
            (None, None, None, None, ColorChoice::Auto),
        ];

        for (clicolor_force, no_color, clicolor, colour_enable, expected) in cases {
            assert_eq!(
                colour_choice(clicolor_force, no_color, clicolor, colour_enable),
                expected,
                "force={clicolor_force:?} no_color={no_color:?} clicolor={clicolor:?} enable={colour_enable:?}"
            );
        }
    }
}
