use colored::Colorize;
use textwrap::{fill, indent};

#[derive(clap::Args, Debug)]
#[command(about = "Show detailed help for a subcommand")]
pub struct Args {
    /// optional subcommand name (e.g. `install`), Aliases like `i`, `ls`, etc. also work
    pub subcommand: Option<String>,
}

pub fn run(args: Args) -> Result<(), i32> {
    let help = match args
        .subcommand
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        None => main_help(),

        // qualifiers + aliases
        Some("config") => config_help(),
        Some("current") | Some("c") => current_help(),
        Some("default") | Some("d") => default_help(),
        Some("env") | Some("e") => env_help(),
        Some("flush") => flush_help(),
        Some("home") | Some("h") => home_help(),
        Some("install") | Some("i") => install_help(),
        Some("list") | Some("ls") => list_help(),
        Some("selfupdate") => selfupdate_help(),
        Some("uninstall") | Some("rm") => uninstall_help(),
        Some("update") => update_help(),
        Some("upgrade") | Some("ug") => upgrade_help(),
        Some("use") | Some("u") => use_help(),
        Some("version") | Some("v") => version_help(),

        Some(other) => {
            eprintln!(
                "error: unknown help topic '{}'\n(use {} to see available subcommands)",
                other,
                "sdk help".italic()
            );
            return Err(1);
        }
    };

    println!("{}", render(help));
    Ok(())
}

struct Subcommand {
    command: String,
    description: String,
}

struct Configuration {
    content: String,
    snippet: String,
}

struct Mnemonic {
    shorthand: String,
    command: String,
}

#[derive(Default)]
struct Help {
    cmd: String,
    tagline: String,
    synopsis: String,
    description: String,
    subcommands: Option<Vec<Subcommand>>,
    configuration: Option<Configuration>,
    mnemonic: Option<Mnemonic>,
    exit_code: Option<String>,
    examples: String,
}

const INDENTATION_WIDTH: usize = 4;
const TERMINAL_WIDTH: usize = 80;
const TEXT_WIDTH: usize = TERMINAL_WIDTH - INDENTATION_WIDTH;

fn render(help: Help) -> String {
    let spaced_tab = format!("{:width$}", " ", width = INDENTATION_WIDTH);
    let indentation = spaced_tab.as_str();

    let sep = if help.cmd.trim() == "sdk" { " - " } else { " " };
    let nameline = format!("{}{}{}", help.cmd.italic(), sep, help.tagline);

    let wrapped_nameline = fill(&nameline, TEXT_WIDTH);
    let name = format!(
        "\n{}\n{}\n\n",
        "NAME".bold(),
        indent(&wrapped_nameline, indentation)
    );

    let synopsis = format!(
        "{}\n{}\n\n",
        "SYNOPSIS".bold(),
        indent(&format!("{}", help.synopsis.italic()), indentation)
    );

    let description = format!(
        "{}\n{}\n\n",
        "DESCRIPTION".bold(),
        indent(&fill(help.description.as_str(), TEXT_WIDTH), indentation)
    );

    let subcommands: String = help
        .subcommands
        .iter()
        .map(|subs| {
            let lines: String = subs
                .iter()
                .map(|sub| {
                    let desc_depth = 17;
                    let desc_indent = format!("{:width$}", " ", width = desc_depth);

                    let command = indent(&fill(&sub.command, desc_depth), indentation);
                    let description = &indent(
                        &fill(&sub.description, TEXT_WIDTH - desc_depth),
                        &desc_indent,
                    )[command.len()..];

                    format!("{}{}\n", command, description)
                })
                .collect();

            format!("{}\n{}\n", "SUBCOMMANDS & QUALIFIERS".bold(), lines)
        })
        .collect();

    let configuration = help
        .configuration
        .map(|config| {
            format!(
                "{}\n{}\n\n{}\n\n",
                "CONFIGURATION".bold(),
                indent(&fill(&config.content, TEXT_WIDTH), indentation),
                indent(&config.snippet, indentation)
            )
        })
        .unwrap_or_default();

    let mnemonic = help
        .mnemonic
        .map(|mnemonic| {
            let text = format!(
                "{} - may be used in place of the {} subcommand.",
                &mnemonic.shorthand.bold(),
                &mnemonic.command.bold()
            );
            format!("{}\n{}\n\n", "MNEMONIC".bold(), indent(&text, indentation))
        })
        .unwrap_or_default();

    let exit_code = help
        .exit_code
        .map(|m| {
            format!(
                "{}\n{}\n\n",
                "EXIT CODE".bold(),
                indent(&fill(&m, TEXT_WIDTH), indentation)
            )
        })
        .unwrap_or_default();

    let examples = format!(
        "{}\n{}\n\n",
        "EXAMPLES".bold(),
        indent(&format!("{}", help.examples.italic()), indentation)
    );

    format!(
        "{}{}{}{}{}{}{}{}",
        name, synopsis, description, subcommands, configuration, exit_code, mnemonic, examples
    )
}

fn main_help() -> Help {
    Help {
        cmd: "sdk".to_string(),
        tagline: "The command line interface (CLI) for SDKMAN!".to_string(),
        synopsis: "sdk <subcommand> [candidate] [version]".to_string(),
        description: "SDKMAN! is a tool for managing parallel versions of multiple JVM related Software Development \
        Kits on most Unix based systems. It provides a convenient Command Line Interface (CLI) and API for installing, \
        switching, removing and listing Candidates."
            .to_string(),
        subcommands: Some(vec![
            Subcommand {
                command: "help".to_string(),
                description: "[subcommand]".italic().to_string(),
            },
            Subcommand {
                command: "install".to_string(),
                description: "<candidate> [version] [path]".italic().to_string(),
            },
            Subcommand {
                command: "uninstall".to_string(),
                description: "<candidate> <version>".italic().to_string(),
            },
            Subcommand {
                command: "list".to_string(),
                description: "[candidate]".italic().to_string(),
            },
            Subcommand {
                command: "use".to_string(),
                description: "<candidate> <version>".italic().to_string(),
            },
            Subcommand {
                command: "config".to_string(),
                description: "no qualifier".to_string(),
            },
            Subcommand {
                command: "default".to_string(),
                description: "<candidate> [version]".italic().to_string(),
            },
            Subcommand {
                command: "home".to_string(),
                description: "<candidate> <version>".italic().to_string(),
            },
            Subcommand {
                command: "env".to_string(),
                description: "[init|install|clear]".italic().to_string(),
            },
            Subcommand {
                command: "current".to_string(),
                description: "[candidate]".italic().to_string(),
            },
            Subcommand {
                command: "upgrade".to_string(),
                description: "[candidate]".italic().to_string(),
            },
            Subcommand {
                command: "version".to_string(),
                description: "no qualifier".to_string(),
            },
            Subcommand {
                command: "selfupdate".to_string(),
                description: "[force]".italic().to_string(),
            },
            Subcommand {
                command: "update".to_string(),
                description: "no qualifier".to_string(),
            },
            Subcommand {
                command: "flush".to_string(),
                description: "[tmp|metadata|version]".italic().to_string(),
            },
        ]),
        examples: "sdk install java 17.0.0-tem\nsdk help install".to_string(),
        ..Default::default()
    }
}

fn config_help() -> Help {
    let config_file = "${SDKMAN_DIR}/etc/config";
    let default_config = "\
---
sdkman_auto_answer=false
sdkman_auto_complete=true
sdkman_auto_env=false
sdkman_auto_update=true
sdkman_beta_channel=false
sdkman_checksum_enable=true
sdkman_colour_enable=true
sdkman_curl_connect_timeout=7
sdkman_curl_max_time=10
sdkman_debug_mode=false
sdkman_insecure_ssl=false
sdkman_selfupdate_feature=true
---";

    Help {
        cmd: "sdk config".to_string(),
        tagline: "sdk subcommand to edit the SDKMAN configuration file".to_string(),
        synopsis: "sdk config".to_string(),
        description: format!(
            "This subcommand opens a text editor on the configuration file located at {}. \
             The subcommand will infer the text editor from the {} environment variable. If the system does \
             not set the {} environment variable, then vi is assumed as the default editor.",
            config_file.underline(),
            "EDITOR".italic(),
            "EDITOR".italic()
        ),
        configuration: Some(Configuration {
            content: format!(
                "The {} file contains the following default configuration. A new shell should be \
                 opened for any configuration changes to take effect.",
                config_file.underline()
            ),
            snippet: default_config.italic().to_string(),
        }),
        examples: "sdk config".to_string(),
        ..Default::default()
    }
}

fn current_help() -> Help {
    Help {
        cmd: "sdk current".to_string(),
        tagline: "sdk subcommand to display the current default installed versions".to_string(),
        synopsis: "sdk current [candidate]".to_string(),
        description: "This subcommand will display a list of candidates with their default version installed on the \
        system. It is also possible to qualify the candidate when running the subcommand to display only that \
        candidate's default version."
            .to_string(),
        mnemonic: Some(Mnemonic {
            shorthand: "c".to_string(),
            command: "current".to_string(),
        }),
        examples: "sdk current\nsdk current java".to_string(),
        ..Default::default()
    }
}

fn default_help() -> Help {
    Help {
        cmd: "sdk default".to_string(),
        tagline: "sdk subcommand to set the local default version of the candidate".to_string(),
        synopsis: "sdk default <candidate> [version]".to_string(),
        description: "The mandatory candidate qualifier of the subcommand specifies the candidate to default for all \
        future shells.\n\nThe optional version qualifier sets that specific version as default for all subsequent \
        shells on the local environment. Omitting the version will set the global SDKMAN tracked version as the \
        default version for that candidate."
            .to_string(),
        mnemonic: Some(Mnemonic {
            shorthand: "d".to_string(),
            command: "default".to_string(),
        }),
        exit_code: Some(
            "The subcommand will return a non-zero return code if the candidate or version does not exist."
                .to_string(),
        ),
        examples: "sdk default java 17.0.0-tem\nsdk default java".to_string(),
        ..Default::default()
    }
}

fn env_help() -> Help {
    let config_file_content = "\
---
# Enable auto-env through the sdkman_auto_env config
# Add key=value pairs of SDKs to use below
java=11.0.13-tem
---"
    .italic();

    Help {
        cmd: "sdk env".to_string(),
        tagline:
            "sdk subcommand to control SDKs on a project level, setting up specific versions for a directory"
                .to_string(),
        synopsis: "sdk env [init|install|clear]".to_string(),
        description: format!(
            "Allows the developer to manage the SDK versions used in a project directory. The \
             subcommand uses an {} file to install or switch specific SDK versions in a project directory.\n\nWhen \
             issuing the subcommand without a qualifier, it will switch to the versions specified in {} and emit \
             warnings for versions not present on the system. In addition, the subcommand has three optional qualifiers.",
            ".sdkmanrc".underline(),
            ".sdkmanrc".underline()
        ),
        subcommands: Some(vec![
            Subcommand {
                command: "install".to_string(),
                description: format!(
                    "install and switch to the SDK versions specified in {}",
                    ".sdkmanrc".underline()
                ),
            },
            Subcommand {
                command: "init".to_string(),
                description: format!(
                    "allows for the creation of a default {} file with a single entry for the {} \
                     candidate, set to the current default value)",
                    ".sdkmanrc".underline(),
                    "java".italic()
                ),
            },
            Subcommand {
                command: "clear".to_string(),
                description: "reset all SDK versions to their system defaults".to_string(),
            },
        ]),
        configuration: Some(Configuration {
            content: format!(
                "The {} file contains key-value pairs for each configurable SDK for that project \
                 environment. You may enable a configuration option for auto-env behaviour by setting {} in the {} \
                 file. This setting will automatically switch versions when stepping into a directory on the presence \
                 of a {} descriptor. When enabled, you no longer need to issue the {} qualifier explicitly. This \
                 behaviour is disabled by default. An initial file will have content such as this:",
                ".sdkmanrc".underline(),
                "sdkman_auto_env=true".italic(),
                "$SDKMAN_DIR/etc/config".underline(),
                ".sdkmanrc".underline(),
                "install".italic()
            ),
            snippet: config_file_content.to_string(),
        }),
        examples: "sdk env\nsdk env install\nsdk env init\nsdk env clear".to_string(),
        ..Default::default()
    }
}

fn flush_help() -> Help {
    Help {
        cmd: "sdk flush".to_string(),
        tagline: "sdk subcommand used for flushing local temporal state of SDKMAN".to_string(),
        synopsis: "sdk flush [tmp|metadata|version]".to_string(),
        description: format!(
            "This command cleans temporary storage under {} in the {} and {} directories, removing \
             metadata and version caches. It also removes any residual download artifacts. It is possible to \
             flush specific targets by providing a qualifier. Omission of the qualifier results in a full flush of all \
             targets.",
            "$SDKMAN_DIR".underline(),
            "var".underline(),
            "tmp".underline()
        ),
        subcommands: Some(vec![
            Subcommand {
                command: "tmp".to_string(),
                description: format!(
                    "cleans out pre/post hooks and residual archives from {}",
                    "$SDKMAN_DIR/tmp".underline()
                ),
            },
            Subcommand {
                command: "metadata".to_string(),
                description: "removes any header metadata".to_string(),
            },
            Subcommand {
                command: "version".to_string(),
                description: format!(
                    "flushes the {} and {} files under {}",
                    "version".underline(),
                    "version_native".underline(),
                    "$SDKMAN_DIR/var".underline()
                ),
            },
        ]),
        examples: "sdk flush\nsdk flush tmp\nsdk flush metadata\nsdk flush version".to_string(),
        ..Default::default()
    }
}

fn home_help() -> Help {
    Help {
        cmd: "sdk home".to_string(),
        tagline: "sdk subcommand to output the path of a specific candidate version".to_string(),
        synopsis: "sdk home <candidate> <version>".to_string(),
        description: "Print the absolute home path of any candidate version installed by SDKMAN. The candidate and \
        version parameters are mandatory. This subcommand is usually used for scripting, so does not append a newline \
        character."
            .to_string(),
        exit_code: Some(
            "The subcommand will emit a non-zero exit code if a valid candidate version is not locally installed."
                .to_string(),
        ),
        examples: "sdk home java 17.0.0-tem".to_string(),
        ..Default::default()
    }
}

fn install_help() -> Help {
    Help {
        cmd: "sdk install".to_string(),
        tagline: "sdk subcommand to install a candidate version".to_string(),
        synopsis: "sdk install <candidate> [version] [path]".to_string(),
        description: "Invoking this subcommand with only the candidate as parameter will install the currently \
        known default version for that candidate. Provide a second qualifier to install a specific non-default \
        version. Provide a third optional qualifier to add an already installed local version. This final qualifier is \
        the absolute local path to the base directory of the SDK to be added. The local version will appear as an \
        installed version of the candidate. The version may not conflict with an existing version, installed or not."
            .to_string(),
        mnemonic: Some(Mnemonic {
            shorthand: "i".to_string(),
            command: "install".to_string(),
        }),
        exit_code: Some(
            "The subcommand will return a non-zero exit code for versions not found or for an invalid path."
                .to_string(),
        ),
        examples: "sdk install java\nsdk install java 17.0.0-tem\nsdk install java 11-local /usr/lib/jvm/java-11-openjdk"
            .to_string(),
        ..Default::default()
    }
}

fn list_help() -> Help {
    let legend = "\
+ - local version
* - installed
> - currently in use";

    Help {
        cmd: "sdk list".to_string(),
        tagline: "sdk subcommand to list all candidates or candidate versions".to_string(),
        synopsis: "sdk list [candidate]".to_string(),
        description: format!(
            "Invoke the subcommand without a candidate to see a comprehensive list of all candidates \
             with name, URL, detailed description and an installation command.\nIf the candidate qualifier is specified, \
             the subcommand will display a list of all available and local versions for that candidate. In addition, the \
             version list view marks all versions that are local, installed or currently in use. They appear as follows:\n
{}

Java has a custom list view with vendor-specific details.",
            legend.italic()
        ),
        mnemonic: Some(Mnemonic {
            shorthand: "ls".to_string(),
            command: "list".to_string(),
        }),
        examples: "sdk list\nsdk list java\nsdk list groovy".to_string(),
        ..Default::default()
    }
}

fn selfupdate_help() -> Help {
    Help {
        cmd: "sdk selfupdate".to_string(),
        tagline: "sdk subcommand to upgrade the SDKMAN core".to_string(),
        synopsis: "sdk selfupdate [force]".to_string(),
        description: "Invoke this command to upgrade the core script and native components of the SDKMAN command-line \
        interface. The command will only upgrade the native components if the detected platform is supported. The \
        command will refuse to upgrade the core if no new version is available. A qualifier may be added to the \
        selfupdate command to force an upgrade."
            .to_string(),
        examples: "sdk selfupdate\nsdk selfupdate force".to_string(),
        ..Default::default()
    }
}

fn uninstall_help() -> Help {
    Help {
        cmd: "sdk uninstall".to_string(),
        tagline: "sdk subcommand to uninstall a candidate version".to_string(),
        synopsis: "sdk uninstall <candidate> <version>".to_string(),
        description: format!(
            "Always follow the subcommand with two qualifiers, the candidate and version to be \
             uninstalled.\n\nThe specified version will be removed from the corresponding candidate directory under {} and \
             will no longer be available for use on the system.",
            "$SDKMAN_DIR/candidates".underline()
        ),
        mnemonic: Some(Mnemonic {
            shorthand: "rm".to_string(),
            command: "uninstall".to_string(),
        }),
        exit_code: Some(
            "An invalid candidate or version supplied to the subcommand will result in a non-zero return code."
                .to_string(),
        ),
        examples: "sdk uninstall java 17.0.0-tem".to_string(),
        ..Default::default()
    }
}

fn update_help() -> Help {
    Help {
        cmd: "sdk update".to_string(),
        tagline: "sdk subcommand to update the local state of SDKMAN".to_string(),
        synopsis: "sdk update".to_string(),
        description: "This command is used to download information about all candidates and versions. Other \
        commands operate on this data to perform version installations and upgrades or search and display details \
        about all packages available for installation. Run this command often to ensure that all candidates are \
        up to date and that the latest versions will be visible and installed."
            .to_string(),
        examples: "sdk update".to_string(),
        ..Default::default()
    }
}

fn upgrade_help() -> Help {
    Help {
        cmd: "sdk upgrade".to_string(),
        tagline: "sdk subcommand to upgrade installed candidate versions".to_string(),
        synopsis: "sdk upgrade [candidate]".to_string(),
        description: "The optional candidate qualifier can be applied to specify the candidate you want to upgrade. \
        If the candidate qualifier is omitted from the command, it will attempt an upgrade of all outdated \
        candidates.\nCandidates that do not require an upgrade will be omitted, and a notification will be displayed \
        that these candidates are up to date."
            .to_string(),
        mnemonic: Some(Mnemonic {
            shorthand: "ug".to_string(),
            command: "upgrade".to_string(),
        }),
        exit_code: Some(
            "The subcommand will return a non-zero return code if the candidate does not exist.".to_string(),
        ),
        examples: "sdk upgrade\nsdk upgrade java".to_string(),
        ..Default::default()
    }
}

fn use_help() -> Help {
    Help {
        cmd: "sdk use".to_string(),
        tagline: "sdk subcommand to use a specific version only in the current shell".to_string(),
        synopsis: "sdk use <candidate> <version>".to_string(),
        description: "The mandatory candidate and version follow the subcommand to specify what to use in the \
        shell. This subcommand only operates on the current shell. It does not affect other shells \
        running different versions of the same candidate. It also does not change the default version set for \
        all subsequent shells."
            .to_string(),
        mnemonic: Some(Mnemonic {
            shorthand: "u".to_string(),
            command: "use".to_string(),
        }),
        exit_code: Some(
            "The subcommand will return a non-zero return code if the candidate or version does not exist."
                .to_string(),
        ),
        examples: "sdk use java 17.0.0-tem".to_string(),
        ..Default::default()
    }
}

fn version_help() -> Help {
    Help {
        cmd: "sdk version".to_string(),
        tagline: "sdk subcommand to display the installed SDKMAN version".to_string(),
        synopsis: "sdk version".to_string(),
        description: "This subcommand displays the version of the bash and native components of SDKMAN on this \
        system. The versions of the bash and native libraries evolve independently from each other and so will not \
        be in sync."
            .to_string(),
        mnemonic: Some(Mnemonic {
            shorthand: "v".to_string(),
            command: "version".to_string(),
        }),
        examples: "sdk version".to_string(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::control;

    fn setup() {
        control::set_override(true);
        control::SHOULD_COLORIZE.set_override(true);
    }

    fn snapshot_dir() -> std::path::PathBuf {
        // point this at where your existing snapshots already live
        // (matches what you described: src/bin/help/snapshots/)
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("commands")
            .join("snapshots")
    }

    fn assert_help_snapshot(name: &str, rendered: String) {
        setup();
        insta::with_settings!({
            snapshot_path => snapshot_dir(),
            prepend_module_to_snapshot => false, // keep names stable across module moves
        }, {
            insta::assert_snapshot!(name, rendered);
        });
    }

    #[test]
    fn renders_main_help() {
        assert_help_snapshot("main_help", render(main_help()));
    }

    #[test]
    fn renders_config_help() {
        assert_help_snapshot("config_help", render(config_help()));
    }

    #[test]
    fn renders_current_help() {
        assert_help_snapshot("current_help", render(current_help()));
    }

    #[test]
    fn renders_default_help() {
        assert_help_snapshot("default_help", render(default_help()));
    }

    #[test]
    fn renders_env_help() {
        assert_help_snapshot("env_help", render(env_help()));
    }

    #[test]
    fn renders_flush_help() {
        assert_help_snapshot("flush_help", render(flush_help()));
    }

    #[test]
    fn renders_home_help() {
        assert_help_snapshot("home_help", render(home_help()));
    }

    #[test]
    fn renders_install_help() {
        assert_help_snapshot("install_help", render(install_help()));
    }

    #[test]
    fn renders_list_help() {
        assert_help_snapshot("list_help", render(list_help()));
    }

    #[test]
    fn renders_selfupdate_help() {
        assert_help_snapshot("selfupdate_help", render(selfupdate_help()));
    }

    #[test]
    fn renders_uninstall_help() {
        assert_help_snapshot("uninstall_help", render(uninstall_help()));
    }

    #[test]
    fn renders_update_help() {
        assert_help_snapshot("update_help", render(update_help()));
    }

    #[test]
    fn renders_upgrade_help() {
        assert_help_snapshot("upgrade_help", render(upgrade_help()));
    }

    #[test]
    fn renders_use_help() {
        assert_help_snapshot("use_help", render(use_help()));
    }

    #[test]
    fn renders_version_help() {
        assert_help_snapshot("version_help", render(version_help()));
    }
}
