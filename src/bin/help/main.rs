extern crate clap;

use clap::Command;
use colored::Colorize;
use textwrap::{fill, indent};

fn main() {
    let default_error = format!(
        "error: no subcommand specified (use {} for help)",
        "sdk help".italic()
    );
    let args = Command::new("help")
        .override_help(default_error.as_str())
        .subcommand(Command::new("broadcast").alias("b"))
        .subcommand(Command::new("config"))
        .subcommand(Command::new("current").alias("c"))
        .subcommand(Command::new("default").alias("d"))
        .subcommand(Command::new("env").alias("e"))
        .subcommand(Command::new("flush"))
        .subcommand(Command::new("home").alias("h"))
        .subcommand(Command::new("install").alias("i"))
        .subcommand(Command::new("list").alias("ls"))
        .subcommand(Command::new("selfupdate"))
        .subcommand(Command::new("uninstall").alias("rm"))
        .subcommand(Command::new("update"))
        .subcommand(Command::new("upgrade").alias("rm"))
        .subcommand(Command::new("use").alias("u"))
        .subcommand(Command::new("version").alias("v"))
        .get_matches();

    let help = match args.subcommand_name() {
        Some("broadcast") => broadcast_help(),
        Some("config") => config_help(),
        Some("current") => current_help(),
        Some("default") => default_help(),
        Some("env") => env_help(),
        Some("flush") => flush_help(),
        Some("home") => home_help(),
        Some("install") => install_help(),
        Some("list") => list_help(),
        Some("selfupdate") => selfupdate_help(),
        Some("uninstall") => uninstall_help(),
        Some("update") => update_help(),
        Some("upgrade") => upgrade_help(),
        Some("use") => use_help(),
        Some("version") => version_help(),
        _ => main_help(),
    };

    println!("{}", &render(help));
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
    subcommands: Option<String>,
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
    let nameline = format!("{} - {}", help.cmd.italic(), help.tagline);
    let wrapped_nameline = fill(&nameline, TEXT_WIDTH);
    let name = format!("\n{}\n{}\n\n", "NAME".bold(), indent(&wrapped_nameline, indentation));

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

    let subcommands = help
        .subcommands
        .map(|sc| format!("{}\n{}\n\n", "SUBCOMMANDS".bold(), indent(sc.as_str(), indentation)))
        .unwrap_or_else(|| String::new());

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
        .unwrap_or_else(|| String::new());

    let exit_code = help
        .exit_code
        .map(|m| format!("{}\n{}\n\n", "EXIT CODE".bold(), indent(&fill(&m, TEXT_WIDTH), indentation)))
        .unwrap_or_else(|| String::new());

    let examples = format!(
        "{}\n{}\n\n",
        "EXAMPLES".bold(),
        indent(&format!("{}", help.examples.italic()), indentation)
    );

    format!(
        "{}{}{}{}{}{}{}",
        name, synopsis, description, subcommands, exit_code, mnemonic, examples
    )
}

fn main_help() -> Help {
    Help {
        cmd: "sdk".to_string(),
        tagline: "The command line interface (CLI) for SDKMAN!".to_string(),
        synopsis: "sdk <subcommand> [candidate] [version]".to_string(),
        description: "SDKMAN! is a tool for managing parallel versions of multiple JVM related Software Development Kits on most Unix based systems. It provides a convenient Command Line Interface (CLI) and API for installing, switching, removing and listing Candidates.".to_string(),
        subcommands: Some("\
help              [subcommand]
install   or i    <candidate> [version] [path]
uninstall or rm   <candidate> <version>
list      or ls   [candidate]
use       or u    <candidate> <version>
config
default   or d    <candidate> [version]
home      or h    <candidate> <version>
env       or e    [init|install|clear]
current   or c    [candidate]
upgrade   or ug   [candidate]
version   or v
broadcast or b
offline           [enable|disable]
selfupdate        [force]
update
flush             [tmp|broadcast|metadata|version]".to_string()),
        examples: "sdk install java 17.0.0-tem\nsdk help install".to_string(),
        ..Default::default()
    }
}

fn broadcast_help() -> Help {
    Help {
        cmd: "sdk broadcast".to_string(),
        tagline: "sdk subcommand to display the latest announcements".to_string(),
        synopsis: "sdk broadcast".to_string(),
        description: "This subcommand displays the latest three vendor announcements about SDK releases on SDKMAN. Each entry shows the release date and broadcast message issued by a vendor.".to_string(),
        mnemonic: Some(Mnemonic { shorthand: "b".to_string(), command: "broadcast".to_string() }),
        examples: "sdk broadcast".to_string(),
        ..Default::default()
    }
}

fn config_help() -> Help {
    Help {
        cmd: "sdk config".to_string(),
        tagline: "sdk subcommand to edit the SDKMAN configuration file".to_string(),
        synopsis: "sdk config".to_string(),
        description: "This subcommand opens a text editor on the configuration file located at ${SDKMAN_DIR}/etc/config. The subcommand will infer the text editor from the EDITOR environment variable. If the system does not set the EDITOR environment variable, then vi is assumed as the default editor.".to_string(),
        examples: "sdk config".to_string(),
        ..Default::default()
    }
}

fn current_help() -> Help {
    Help {
        cmd: "sdk current".to_string(),
        tagline: "sdk subcommand to display the current default installed versions".to_string(),
        synopsis: "sdk current [candidate]".to_string(),
        description: "This subcommand will display a list of candidates with their default version installed on the system. It is also possible to qualify the candidate when running the subcommand to display only that candidate's default version.".to_string(),
        mnemonic: Some(Mnemonic { shorthand: "c".to_string(), command: "current".to_string() }),
        examples: "sdk current\nsdk current java".to_string(),
        ..Default::default()
    }
}

fn default_help() -> Help {
    Help {
        cmd: "sdk default".to_string(),
        tagline: "sdk subcommand to set the local default version of the candidate".to_string(),
        synopsis: "sdk default <candidate> [version]".to_string(),
        description: "\
The mandatory candidate qualifier of the subcommand specifies the candidate to default for all future shells.\n
The optional version qualifier sets that specific version as default for all subsequent shells on the local environment. Omitting the version will set the global SDKMAN tracked version as the default version for that candidate.".to_string(),
        mnemonic: Some(Mnemonic { shorthand: "d".to_string(), command: "default".to_string() }),
        exit_code: Some("The subcommand will return a non-zero return code if the candidate or version does not exist.".to_string()),
        examples: "sdk default java 17.0.0-tem\nsdk default java".to_string(),
        ..Default::default()
    }
}

fn env_help() -> Help {
    Help {
        cmd: "sdk env".to_string(),
        tagline: "sdk subcommand to control SDKs on a project level, setting up specific versions for a directory".to_string(),
        synopsis: "sdk env [init|install|clear]".to_string(),
        description: "\
Allows the developer to manage the SDK versions used in a project directory. The subcommand uses a `.sdkmanrc` file to install or switch specific SDK versions in a project directory.\n
The absence of a qualifier will switch to the versions specified in `.sdkmanrc` and emits warnings for versions not present on the system. In addition, it has three optional qualifiers:

install  :  install and switch to the SDK versions specified in `.sdkmanrc`
            (used as default if the qualifier is omitted)
init     :  allows for the creation of a default `.sdkmanrc` file with a
            single entry for the `java` candidate, set to the current default
            value
clear    :  reset all SDK versions to their system defaults

The `.sdkmanrc` file contains key-value pairs for each configurable SDK for that project environment. An initial file will content such as this:

---
# Enable auto-env through the sdkman_auto_env config
# Add key=value pairs of SDKs to use below
java=11.0.13-tem
---

You may enable a configuration option for auto-env behaviour. This setting will automatically switch versions when stepping into a directory on the presence of a `.sdkmanrc` descriptor. When enabled, you no longer need to issue the `install` qualifier explicitly. This behaviour is disabled by default.".to_string(),
        examples: "sdk env\nsdk env install\nsdk env init\nsdk env clear".to_string(),
        ..Default::default()
    }
}

fn flush_help() -> Help {
    Help {
        cmd: "sdk flush".to_string(),
        tagline: "sdk subcommand used for flushing local temporal state of SDKMAN".to_string(),
        synopsis: "sdk flush [tmp|broadcast|metadata|version]".to_string(),
        description: "This command cleans temporary storage under the `tmp` and `var` folders, removing broadcast, metadata, and version caches. It also removes any residual download artifacts. It is possible to flush specific targets by providing a qualifier. Omission of the qualifier results in a full flush of all targets.".to_string(),
        subcommands: Some("\
The following qualifiers apply to this command:

tmp         :  cleans out pre/post hooks and residual archives from`.sdkman/tmp`
broadcast   :  wipes cached broadcast messages
metadata    :  removes any header metadata
version     :  flushes the SDKMAN version file".to_string()),
        examples: "sdk flush\nsdk flush tmp\nsdk flush broadcast\nsdk flush metadata\nsdk flush version".to_string(),
        ..Default::default()
    }
}

fn home_help() -> Help {
    Help {
        cmd: "sdk home".to_string(),
        tagline: "sdk subcommand to output the path of a specific candidate version".to_string(),
        synopsis: "sdk home <candidate> <version>".to_string(),
        description: "Print the absolute home path of any candidate version installed by SDKMAN. The candidate and version parameters are mandatory. Often used for scripting, so does not append a newline character.".to_string(),
        exit_code: Some("The subcommand will emit a non-zero exit code if a valid candidate version is not locally installed.".to_string()),
        examples: "sdk home java 17.0.0-tem".to_string(),
        ..Default::default()
    }
}

fn install_help() -> Help {
    Help {
        cmd: "sdk install".to_string(),
        tagline: "sdk subcommand to install a candidate version".to_string(),
        synopsis: "sdk install <candidate> [version] [path]".to_string(),
        description: "\
Invoking this subcommand with only the candidate as a parameter will install the currently known default version for that candidate.\n
Provide a subsequent qualifier to install a specific non-default version.\n
Provide another qualifier to add an already installed local version. This qualifier is the absolute local path to the base directory of the SDK to be added. The local version will appear as an installed version of the candidate. The version may not conflict with an existing version, installed or not.".to_string(),
        mnemonic: Some(Mnemonic { shorthand: "i".to_string(), command: "install".to_string() }),
        exit_code: Some("The subcommand will return a non-zero exit code for unfound versions or if the path does not exist.".to_string()),
        examples: "sdk install java\nsdk install java 17.0.0-tem\nsdk install java 11-local /usr/lib/jvm/java-11-openjdk".to_string(),
        ..Default::default()
    }
}

fn list_help() -> Help {
    Help {
        cmd: "sdk list".to_string(),
        tagline: "sdk subcommand to list all candidates or candidate versions".to_string(),
        synopsis: "sdk list [candidate]".to_string(),
        description: "\
Invoke the subcommand without a candidate to see a comprehensive list of all candidates with name, URL, detailed description and an installation command.\n
If the candidate qualifier is specified, the subcommand will display a list of all available and local versions for that candidate. In addition, the version list view marks all versions that are local, installed or currently in use.
They appear as follows:\n
+ - local version
* - installed
> - currently in use

Java has a custom list view with vendor-specific details.".to_string(),
        mnemonic: Some(Mnemonic { shorthand: "ls".to_string(), command: "list".to_string() }),
        examples: "sdk list\nsdk list java\nsdk list groovy".to_string(),
        ..Default::default()
    }
}

fn selfupdate_help() -> Help {
    Help {
        cmd: "sdk selfupdate".to_string(),
        tagline: "sdk subcommand to upgrade the SDKMAN core".to_string(),
        synopsis: "sdk selfupdate [force]".to_string(),
        description: "\
Invoke this command to upgrade the core script and native components of the SDKMAN command-line interface. The command will only upgrade the native components if the detected platform is supported.\n
The command will refuse to upgrade the core if no new version is available. A qualifier may be added to the selfupdate command to force an upgrade.".to_string(),
        examples: "sdk selfupdate\nsdk selfupdate force".to_string(),
        ..Default::default()
    }
}

fn uninstall_help() -> Help {
    Help {
        cmd: "sdk uninstall".to_string(),
        tagline: "sdk subcommand to uninstall a candidate version".to_string(),
        synopsis: "sdk uninstall <candidate> <version>".to_string(),
        description: "\
Always follow the subcommand with two qualifiers, the candidate and version to be uninstalled.\n
The specified version will be removed from the candidate directory in $SDKMAN_DIR/candidates and will no longer be available for use on the system.".to_string(),
        mnemonic: Some(Mnemonic { shorthand: "rm".to_string(), command: "uninstall".to_string() }),
        exit_code: Some("An invalid candidate or version supplied to the subcommand will result in a non-zero return code.".to_string()),
        examples: "sdk uninstall java 17.0.0-tem".to_string(),
        ..Default::default()
    }
}

fn update_help() -> Help {
    Help {
        cmd: "sdk update".to_string(),
        tagline: "sdk subcommand to update the local state of SDKMAN".to_string(),
        synopsis: "sdk update".to_string(),
        description: "\
This command is used to download information about all candidates and versions. Other commands operate on this data to perform version installations and upgrades or search and display details about all packages available for installation.\n
Run this command often to ensure that all candidates are up to date and that the latest versions will be visible and installed.".to_string(),
        examples: "sdk update".to_string(),
        ..Default::default()
    }
}

fn upgrade_help() -> Help {
    Help {
        cmd: "sdk upgrade".to_string(),
        tagline: "sdk subcommand to upgrade installed candidate versions".to_string(),
        synopsis: "sdk upgrade [candidate]".to_string(),
        description: "\
The optional candidate qualifier can be applied to specify the candidate you want to upgrade. If the candidate qualifier is omitted from the command, it will attempt an upgrade of all outdated candidates.\n
Candidates that do not require an upgrade will be omitted, and a notification will be displayed that the candidates are up to date.".to_string(),
        mnemonic: Some(Mnemonic { shorthand: "ug".to_string(), command: "upgrade".to_string() }),
        exit_code: Some("The subcommand will return a non-zero return code if the candidate does not exist.".to_string()),
        examples: "sdk upgrade\nsdk upgrade java".to_string(),
        ..Default::default()
    }
}

fn use_help() -> Help {
    Help {
        cmd: "sdk use".to_string(),
        tagline: "sdk subcommand to use a specific version in the current shell".to_string(),
        synopsis: "sdk use <candidate> <version>".to_string(),
        description: "\
The mandatory candidate and version follow the subcommand to specify what to use in the current shell.\n
This subcommand only operates on the current shell. It does not affect other shells running different versions of the same candidate. It also does not change the default version set for all subsequent new shells.".to_string(),
        mnemonic: Some(Mnemonic { shorthand: "u".to_string(), command: "use".to_string() }),
        exit_code: Some("The subcommand will return a non-zero return code if the candidate or version does not exist.".to_string()),
        examples: "sdk use java 17.0.0-tem".to_string(),
        ..Default::default()
    }
}

fn version_help() -> Help {
    Help {
        cmd: "sdk version".to_string(),
        tagline: "sdk subcommand to display the installed SDKMAN version".to_string(),
        synopsis: "sdk version".to_string(),
        description: "This subcommand displays the version of the bash and native constituents of SDKMAN on this system. The versions of the bash and native libraries evolve independently from each other and so will not be the same.".to_string(),
        mnemonic: Some(Mnemonic { shorthand: "v".to_string(), command: "version".to_string() }),
        examples: "sdk version".to_string(),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::{broadcast_help, config_help, current_help, default_help, main_help, render};

    #[test]
    fn render_main_help() {
        let help_text = "
NAME
    sdk - The command line interface (CLI) for SDKMAN!

SYNOPSIS
    sdk <subcommand> [candidate] [version]

DESCRIPTION
    SDKMAN! is a tool for managing parallel versions of multiple JVM related
    Software Development Kits on most Unix based systems. It provides a
    convenient Command Line Interface (CLI) and API for installing, switching,
    removing and listing Candidates.

SUBCOMMANDS
    help              [subcommand]
    install   or i    <candidate> [version] [path]
    uninstall or rm   <candidate> <version>
    list      or ls   [candidate]
    use       or u    <candidate> <version>
    config
    default   or d    <candidate> [version]
    home      or h    <candidate> <version>
    env       or e    [init|install|clear]
    current   or c    [candidate]
    upgrade   or ug   [candidate]
    version   or v
    broadcast or b
    offline           [enable|disable]
    selfupdate        [force]
    update
    flush             [tmp|broadcast|metadata|version]

EXAMPLES
    sdk install java 17.0.0-tem
    sdk help install

";
        colored::control::set_override(false);
        assert_eq!(help_text, render(main_help()));
    }

    #[test]
    fn render_broadcast_help() {
        let broadcast_text = "
NAME
    sdk broadcast - sdk subcommand to display the latest announcements

SYNOPSIS
    sdk broadcast

DESCRIPTION
    This subcommand displays the latest three vendor announcements about SDK
    releases on SDKMAN. Each entry shows the release date and broadcast message
    issued by a vendor.

MNEMONIC
    b - may be used in place of the broadcast subcommand.

EXAMPLES
    sdk broadcast

";
        colored::control::set_override(false);
        assert_eq!(broadcast_text, render(broadcast_help()));
    }

    #[test]
    fn render_config_help() {
        let config_text = "
NAME
    sdk config - sdk subcommand to edit the SDKMAN configuration file

SYNOPSIS
    sdk config

DESCRIPTION
    This subcommand opens a text editor on the configuration file located at
    ${SDKMAN_DIR}/etc/config. The subcommand will infer the text editor from
    the EDITOR environment variable. If the system does not set the EDITOR
    environment variable, then vi is assumed as the default editor.

EXAMPLES
    sdk config

";
        colored::control::set_override(false);
        assert_eq!(config_text, render(config_help()));
    }

    #[test]
    fn render_current_help() {
        let current_text = "
NAME
    sdk current - sdk subcommand to display the current default installed
    versions

SYNOPSIS
    sdk current [candidate]

DESCRIPTION
    This subcommand will display a list of candidates with their default version
    installed on the system. It is also possible to qualify the candidate when
    running the subcommand to display only that candidate's default version.

MNEMONIC
    c - may be used in place of the current subcommand.

EXAMPLES
    sdk current
    sdk current java

";
        colored::control::set_override(false);
        assert_eq!(current_text, render(current_help()));
    }

    #[test]
    fn render_default_help() {
        let default_text = "
NAME
    sdk default - sdk subcommand to set the local default version of the
    candidate

SYNOPSIS
    sdk default <candidate> [version]

DESCRIPTION
    The mandatory candidate qualifier of the subcommand specifies the candidate
    to default for all future shells.

    The optional version qualifier sets that specific version as default for all
    subsequent shells on the local environment. Omitting the version will set
    the global SDKMAN tracked version as the default version for that candidate.

EXIT CODE
    The subcommand will return a non-zero return code if the candidate or
    version does not exist.

MNEMONIC
    d - may be used in place of the default subcommand.

EXAMPLES
    sdk default java 17.0.0-tem
    sdk default java

";
        colored::control::set_override(false);
        assert_eq!(default_text, render(default_help()));
    }
}