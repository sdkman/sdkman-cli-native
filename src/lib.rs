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
    use directories::UserDirs;
    use std::path::PathBuf;
    use std::{env, fs, io};

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
        fs::read_to_string(path)
            .ok()
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
            let hint = match closest_candidate(all_candidates, candidate) {
                Some(suggestion) => format!("did you mean {}?", ui::sdk(&suggestion)),
                None => format!("run {} to see all SDKs", ui::cmd("sdk list")),
            };
            Err(CliError {
                message: format!("unknown SDK {}", ui::sdk(candidate)),
                hints: vec![hint],
            })
        }
    }

    pub fn closest_candidate(all_candidates: &[String], candidate: &str) -> Option<String> {
        all_candidates
            .iter()
            .map(|known| (known, strsim::jaro(candidate, known)))
            .filter(|(_, score)| *score > 0.7)
            .fold(None, |best, (known, score)| match best {
                Some((_, best_score)) if best_score >= score => best,
                _ => Some((known, score)),
            })
            .map(|(known, _)| known.clone())
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

    pub fn os_reason(error: &io::Error) -> String {
        let full = error.to_string();
        let reason = full.split(" (os error ").next().unwrap_or(&full);
        let mut chars = reason.chars();
        match chars.next() {
            Some(first) => first.to_lowercase().chain(chars).collect(),
            None => String::new(),
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

    pub fn brand(text: &str) -> impl Display {
        styled(
            Style::new().bold().fg_color(Some(AnsiColor::Yellow.into())),
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

pub mod cli {
    use clap::builder::styling::{AnsiColor, Style, Styles};
    use clap::error::{ContextKind, ContextValue, ErrorKind};
    use clap::{Arg, ArgAction, Command};
    use std::process::ExitCode;

    use crate::ui::{self, CliError};

    const HELP_TEMPLATE: &str = "\
{about-with-newline}
{usage-heading} {usage}

{before-help}{all-args}{after-help}";

    fn styles() -> Styles {
        let bold = Style::new().bold();
        let cyan = Style::new().fg_color(Some(AnsiColor::Cyan.into()));
        Styles::styled()
            .header(bold)
            .usage(bold)
            .literal(cyan)
            .placeholder(Style::new())
            .error(Style::new().bold().fg_color(Some(AnsiColor::Red.into())))
            .valid(Style::new().fg_color(Some(AnsiColor::Green.into())))
            .invalid(Style::new().fg_color(Some(AnsiColor::Yellow.into())))
    }

    fn configure(command: Command) -> Command {
        command
            .styles(styles())
            .max_term_width(80)
            .disable_version_flag(true)
            .disable_help_flag(true)
            .disable_help_subcommand(true)
            .help_template(HELP_TEMPLATE)
            .arg(
                Arg::new("help")
                    .short('h')
                    .long("help")
                    .help("Print help")
                    .long_help("Print help")
                    .action(ArgAction::Help),
            )
    }

    fn sdk_arg() -> Arg {
        Arg::new("candidate").value_name("SDK")
    }

    pub fn current() -> Command {
        configure(
            Command::new("current")
                .bin_name("sdk current")
                .visible_alias("c")
                .about("Show the default version of one or all SDKs")
                .before_long_help(
                    "Without an SDK, lists every SDK that has a default version. With an SDK, shows\n\
                     the default version of that SDK.",
                )
                .after_help("Aliases: c")
                .after_long_help(
                    "Aliases: c\n\
                     \n\
                     Exit status:\n\
                     \x20\x20Exits with a non-zero code if the SDK does not exist or has no default\n\
                     \x20\x20version.\n\
                     \n\
                     Examples:\n\
                     \x20\x20sdk current\n\
                     \x20\x20sdk current java",
                )
                .arg(sdk_arg().help("The SDK whose default version to show")),
        )
    }

    pub fn default() -> Command {
        configure(
            Command::new("default")
                .bin_name("sdk default")
                .visible_alias("d")
                .about("Set the default version of an SDK")
                .before_long_help(
                    "Makes an installed version the default version of its SDK, so every new shell\n\
                     uses it.",
                )
                .after_help("Aliases: d")
                .after_long_help(
                    "Aliases: d\n\
                     \n\
                     Exit status:\n\
                     \x20\x20Exits with a non-zero code if the SDK or version is not installed.\n\
                     \n\
                     Examples:\n\
                     \x20\x20sdk default java 21.0.2-tem",
                )
                .arg(sdk_arg().required(true).help("The SDK, such as java"))
                .arg(
                    Arg::new("version")
                        .value_name("VERSION")
                        .required(true)
                        .help("The installed version to make the default"),
                ),
        )
    }

    pub fn home() -> Command {
        configure(
            Command::new("home")
                .bin_name("sdk home")
                .visible_alias("h")
                .about("Print the directory of an installed version")
                .before_long_help(
                    "Prints the absolute path of an installed version and nothing else, so that\n\
                     scripts can use it.",
                )
                .after_help("Aliases: h")
                .after_long_help(
                    "Aliases: h\n\
                     \n\
                     Exit status:\n\
                     \x20\x20Exits with a non-zero code if the SDK or version is not installed.\n\
                     \n\
                     Examples:\n\
                     \x20\x20sdk home java 21.0.2-tem\n\
                     \x20\x20export JAVA_HOME=$(sdk home java 21.0.2-tem)",
                )
                .arg(sdk_arg().required(true).help("The SDK, such as java"))
                .arg(
                    Arg::new("version")
                        .value_name("VERSION")
                        .required(true)
                        .help("The installed version"),
                ),
        )
    }

    pub fn uninstall() -> Command {
        configure(
            Command::new("uninstall")
                .bin_name("sdk uninstall")
                .visible_alias("rm")
                .about("Uninstall a version of an SDK")
                .before_long_help(
                    "Uninstalls a version of an SDK from this machine. You cannot uninstall the\n\
                     default version unless you add --force.",
                )
                .after_help("Aliases: rm")
                .after_long_help(
                    "Aliases: rm\n\
                     \n\
                     Exit status:\n\
                     \x20\x20Exits with a non-zero code if the SDK or version is not installed, or if the\n\
                     \x20\x20version is the default version and --force is not given.\n\
                     \n\
                     Examples:\n\
                     \x20\x20sdk uninstall java 17.0.0-tem\n\
                     \x20\x20sdk rm --force java 17.0.0-tem",
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .action(ArgAction::SetTrue)
                        .help("Uninstall even if it is the default version"),
                )
                .arg(sdk_arg().required(true).help("The SDK, such as java"))
                .arg(
                    Arg::new("version")
                        .value_name("VERSION")
                        .required(true)
                        .help("The installed version to uninstall"),
                ),
        )
    }

    pub fn version() -> Command {
        configure(
            Command::new("version")
                .bin_name("sdk version")
                .visible_alias("v")
                .about("Show the SDKMAN! version")
                .before_long_help(
                    "Shows the versions of the core and of the native commands. The two are\n\
                     released separately, so their versions differ.",
                )
                .after_help("Aliases: v")
                .after_long_help(
                    "Aliases: v\n\
                     \n\
                     Examples:\n\
                     \x20\x20sdk version",
                ),
        )
    }

    pub fn install() -> Command {
        configure(
            Command::new("install")
                .bin_name("sdk install")
                .visible_alias("i")
                .about("Install a version of an SDK")
                .before_long_help(
                    "Installs a version of an SDK on this machine. Without a version, installs the\n\
                     recommended version. For java, this is the latest LTS version.\n\
                     \n\
                     Add a path to make a local version: SDKMAN! points to the SDK in that\n\
                     directory instead of downloading it. The version must not clash with any other\n\
                     version of the SDK.",
                )
                .after_help("Aliases: i")
                .after_long_help(
                    "Aliases: i\n\
                     \n\
                     Exit status:\n\
                     \x20\x20Exits with a non-zero code if the version does not exist or the path is not\n\
                     \x20\x20a directory.\n\
                     \n\
                     Examples:\n\
                     \x20\x20sdk install java\n\
                     \x20\x20sdk install java 21.0.2-tem\n\
                     \x20\x20sdk install java 11-local /usr/lib/jvm/java-11-openjdk",
                )
                .arg(sdk_arg().required(true).help("The SDK, such as java"))
                .arg(
                    Arg::new("version")
                        .value_name("VERSION")
                        .help("The version to install"),
                )
                .arg(
                    Arg::new("path")
                        .value_name("PATH")
                        .help("The directory of an SDK already on this machine"),
                ),
        )
    }

    pub fn list() -> Command {
        configure(
            Command::new("list")
                .bin_name("sdk list")
                .visible_alias("ls")
                .about("List SDKs or the versions of an SDK")
                .before_long_help(
                    "Without an SDK, lists every SDK with its website, a description and the\n\
                     command to install it.\n\
                     \n\
                     With an SDK, lists its versions and marks the ones on this machine:\n\
                     \n\
                     \x20\x20\x20\x20+  local version\n\
                     \x20\x20\x20\x20*  installed\n\
                     \x20\x20\x20\x20>  in use\n\
                     \n\
                     For java, the list also shows the distribution of each version.",
                )
                .after_help("Aliases: ls")
                .after_long_help(
                    "Aliases: ls\n\
                     \n\
                     Examples:\n\
                     \x20\x20sdk list\n\
                     \x20\x20sdk list java",
                )
                .arg(sdk_arg().help("The SDK whose versions to list")),
        )
    }

    pub fn r#use() -> Command {
        configure(
            Command::new("use")
                .bin_name("sdk use")
                .visible_alias("u")
                .about("Use a version of an SDK in this shell")
                .before_long_help(
                    "Puts a version in use in this shell only. Other shells and the default version\n\
                     stay as they are.",
                )
                .after_help("Aliases: u")
                .after_long_help(
                    "Aliases: u\n\
                     \n\
                     Exit status:\n\
                     \x20\x20Exits with a non-zero code if the SDK or version is not installed.\n\
                     \n\
                     Examples:\n\
                     \x20\x20sdk use java 17.0.0-tem",
                )
                .arg(sdk_arg().required(true).help("The SDK, such as java"))
                .arg(
                    Arg::new("version")
                        .value_name("VERSION")
                        .required(true)
                        .help("The installed version to use"),
                ),
        )
    }

    pub fn env() -> Command {
        configure(
            Command::new("env")
                .bin_name("sdk env")
                .visible_alias("e")
                .about("Use the versions in a project config")
                .before_long_help(
                    "Without a command, puts the versions in the project config (.sdkmanrc) of this\n\
                     directory in use in this shell, and warns about any that are not installed.",
                )
                .after_help("Aliases: e")
                .after_long_help(
                    "Aliases: e\n\
                     \n\
                     Configuration:\n\
                     \x20\x20Set sdkman_auto_env=true in the SDKMAN! config to use a project config\n\
                     \x20\x20automatically whenever you change into its directory. A project config looks\n\
                     \x20\x20like this:\n\
                     \n\
                     \x20\x20\x20\x20# Add key=value pairs of SDKs to use below\n\
                     \x20\x20\x20\x20java=21.0.2-tem\n\
                     \n\
                     Examples:\n\
                     \x20\x20sdk env\n\
                     \x20\x20sdk env init\n\
                     \x20\x20sdk env install\n\
                     \x20\x20sdk env clear",
                )
                .subcommand(
                    Command::new("init")
                        .about("Create a project config with the default version of java"),
                )
                .subcommand(
                    Command::new("install")
                        .about("Install the versions in the project config and use them"),
                )
                .subcommand(Command::new("clear").about("Go back to the default versions")),
        )
    }

    pub fn upgrade() -> Command {
        configure(
            Command::new("upgrade")
                .bin_name("sdk upgrade")
                .visible_alias("ug")
                .about("Upgrade SDKs to their recommended version")
                .before_long_help(
                    "Installs the recommended version of an SDK and makes it the default version,\n\
                     if it is newer than the default version you have. Without an SDK, upgrades\n\
                     every SDK that has a newer recommended version.",
                )
                .after_help("Aliases: ug")
                .after_long_help(
                    "Aliases: ug\n\
                     \n\
                     Exit status:\n\
                     \x20\x20Exits with a non-zero code if the SDK does not exist.\n\
                     \n\
                     Examples:\n\
                     \x20\x20sdk upgrade\n\
                     \x20\x20sdk upgrade java",
                )
                .arg(sdk_arg().help("The SDK to upgrade")),
        )
    }

    pub fn update() -> Command {
        configure(
            Command::new("update")
                .bin_name("sdk update")
                .about("Update the list of SDKs and versions")
                .before_long_help(
                    "Downloads the newest list of SDKs and versions from the SDKMAN! servers. Other\n\
                     commands use this list to install, upgrade and list versions. Run it often to\n\
                     see new versions.",
                )
                .after_long_help(
                    "Examples:\n\
                     \x20\x20sdk update",
                ),
        )
    }

    pub fn selfupdate() -> Command {
        configure(
            Command::new("selfupdate")
                .bin_name("sdk selfupdate")
                .about("Replace SDKMAN! with a newer version")
                .before_long_help(
                    "Replaces the core and the native commands with the newest SDKMAN! version. The\n\
                     native commands are only replaced on supported platforms. If there is no newer\n\
                     version, nothing changes unless you add force.",
                )
                .after_long_help(
                    "Examples:\n\
                     \x20\x20sdk selfupdate\n\
                     \x20\x20sdk selfupdate force",
                )
                .subcommand(
                    Command::new("force").about("Reinstall even if there is no newer version"),
                ),
        )
    }

    pub fn flush() -> Command {
        configure(
            Command::new("flush")
                .bin_name("sdk flush")
                .about("Flush temporary files and cached data")
                .before_long_help(
                    "Discards temporary files and cached data that SDKMAN! keeps for itself.\n\
                     Without a command, flushes everything.",
                )
                .after_long_help(
                    "Examples:\n\
                     \x20\x20sdk flush\n\
                     \x20\x20sdk flush tmp\n\
                     \x20\x20sdk flush metadata\n\
                     \x20\x20sdk flush version",
                )
                .subcommand(
                    Command::new("tmp")
                        .about("Flush downloaded archives and leftover hook scripts"),
                )
                .subcommand(Command::new("metadata").about("Flush cached metadata about SDKs"))
                .subcommand(Command::new("version").about("Flush the cached SDKMAN! versions")),
        )
    }

    pub fn config() -> Command {
        configure(
            Command::new("config")
                .bin_name("sdk config")
                .about("Edit the SDKMAN! config")
                .before_long_help(
                    "Opens the SDKMAN! config ($SDKMAN_DIR/etc/config) in the editor named by the\n\
                     EDITOR environment variable, or in vi if EDITOR is not set. Open a new shell\n\
                     for your changes to take effect.",
                )
                .after_long_help(
                    "Configuration:\n\
                     \x20\x20The SDKMAN! config contains settings such as these:\n\
                     \n\
                     \x20\x20\x20\x20sdkman_auto_answer=false\n\
                     \x20\x20\x20\x20sdkman_auto_complete=true\n\
                     \x20\x20\x20\x20sdkman_auto_env=false\n\
                     \x20\x20\x20\x20sdkman_auto_update=true\n\
                     \x20\x20\x20\x20sdkman_beta_channel=false\n\
                     \x20\x20\x20\x20sdkman_checksum_enable=true\n\
                     \x20\x20\x20\x20sdkman_colour_enable=true\n\
                     \x20\x20\x20\x20sdkman_curl_connect_timeout=7\n\
                     \x20\x20\x20\x20sdkman_curl_max_time=10\n\
                     \x20\x20\x20\x20sdkman_debug_mode=false\n\
                     \x20\x20\x20\x20sdkman_healthcheck_enable=true\n\
                     \x20\x20\x20\x20sdkman_insecure_ssl=false\n\
                     \x20\x20\x20\x20sdkman_native_enable=true\n\
                     \x20\x20\x20\x20sdkman_selfupdate_feature=true\n\
                     \n\
                     Examples:\n\
                     \x20\x20sdk config",
                ),
        )
    }

    pub fn help() -> Command {
        configure(
            Command::new("help")
                .bin_name("sdk help")
                .about("Show help for a command")
                .before_long_help(
                    "Without a command, lists all commands. With a command, shows its full help.",
                )
                .after_long_help(
                    "Examples:\n\
                     \x20\x20sdk help\n\
                     \x20\x20sdk help install",
                )
                .arg(
                    Arg::new("command")
                        .value_name("COMMAND")
                        .help("The command to show help for"),
                ),
        )
    }

    pub fn sdk() -> Command {
        configure(
            Command::new("sdk")
                .override_usage("sdk <COMMAND> [ARGUMENTS]")
                .about("The command line interface for SDKMAN!")
                .before_long_help(
                    "SDKMAN! installs and manages versions of SDKs, such as java, maven and gradle,\n\
                     and sets which version each shell uses.",
                )
                .after_long_help(
                    "Examples:\n\
                     \x20\x20sdk install java\n\
                     \x20\x20sdk help install",
                )
                .subcommand(install())
                .subcommand(uninstall())
                .subcommand(list())
                .subcommand(r#use())
                .subcommand(default())
                .subcommand(current())
                .subcommand(home())
                .subcommand(env())
                .subcommand(upgrade())
                .subcommand(update())
                .subcommand(selfupdate())
                .subcommand(flush())
                .subcommand(config())
                .subcommand(version())
                .subcommand(help()),
        )
    }

    pub fn report_parse_error(command: &str, error: clap::Error) -> ExitCode {
        if matches!(
            error.kind(),
            ErrorKind::DisplayHelp | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        ) {
            let _ = error.print();
            return ExitCode::SUCCESS;
        }

        let help_hint = format!("run {}", ui::cmd(&format!("sdk {command} --help")));
        let cli_error = match error.kind() {
            ErrorKind::UnknownArgument => {
                let invalid = context_string(&error, ContextKind::InvalidArg).unwrap_or_default();
                if invalid.starts_with('-') {
                    let mut hints = Vec::new();
                    if let Some(suggested) = context_string(&error, ContextKind::SuggestedArg) {
                        hints.push(format!("did you mean {}?", ui::cmd(&suggested)));
                    }
                    hints.push(help_hint);
                    CliError {
                        message: format!("unknown option {invalid}"),
                        hints,
                    }
                } else {
                    CliError {
                        message: format!("unexpected argument {invalid}"),
                        hints: vec![help_hint],
                    }
                }
            }
            ErrorKind::MissingRequiredArgument => {
                let missing = context_strings(&error, ContextKind::InvalidArg);
                let message = if missing.len() == 1 {
                    format!("missing argument {}", missing[0])
                } else {
                    format!("missing arguments {}", missing.join(" "))
                };
                CliError {
                    message,
                    hints: vec![help_hint],
                }
            }
            _ => fallback_error(&error, help_hint),
        };
        cli_error.report();
        ExitCode::from(2)
    }

    fn context_string(error: &clap::Error, kind: ContextKind) -> Option<String> {
        match error.get(kind) {
            Some(ContextValue::String(value)) => Some(value.clone()),
            _ => None,
        }
    }

    fn context_strings(error: &clap::Error, kind: ContextKind) -> Vec<String> {
        match error.get(kind) {
            Some(ContextValue::Strings(values)) => values.clone(),
            Some(ContextValue::String(value)) => vec![value.clone()],
            _ => Vec::new(),
        }
    }

    fn fallback_error(error: &clap::Error, help_hint: String) -> CliError {
        let rendered = error.render().to_string();
        let mut message = String::new();
        let mut hints = Vec::new();
        for line in rendered.lines() {
            if let Some(rest) = line.strip_prefix("error: ") {
                message = clean_clap_text(rest);
            } else if let Some(rest) = line.trim_start().strip_prefix("tip: ") {
                hints.push(clean_clap_text(rest));
            }
        }
        hints.push(help_hint);
        CliError { message, hints }
    }

    fn clean_clap_text(text: &str) -> String {
        let without_quotes: String = text
            .chars()
            .filter(|&character| character != '\'')
            .collect();
        let trimmed = without_quotes.trim().trim_end_matches('.');
        let mut chars = trimmed.chars();
        match chars.next() {
            Some(first) => first.to_lowercase().chain(chars).collect(),
            None => String::new(),
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
        env::set_var(SDKMAN_DIR_ENV_VAR, &sdkman_dir);
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
        file.write_all(expected_version.as_bytes()).unwrap();
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
