extern crate clap;

use clap::{App, SubCommand};

fn main() {
    let args = App::new("help")
        .help(MAIN_HELP)
        .subcommand(SubCommand::with_name("install").alias("i"))
        .subcommand(SubCommand::with_name("uninstall").alias("rm"))
        .subcommand(SubCommand::with_name("list").alias("ls"))
        .subcommand(SubCommand::with_name("use").alias("u"))
        .get_matches();

    let help = match args.subcommand_name() {
        Some("install") => INSTALL_HELP,
        Some("uninstall") => UNINSTALL_HELP,
        Some("list") => LIST_HELP,
        Some("use") => USE_HELP,
        _ => MAIN_HELP,
    };

    println!("{}\n", help);
}

const MAIN_HELP: &str = "\
sdk - The command line interface (CLI) for SDKMAN!

USAGE:
    sdk <SUBCOMMAND> [CANDIDATE] [VERSION]

SDKMAN! is a tool for managing parallel versions of multiple Software
Development Kits on most Unix based systems. It provides a convenient Command
Line Interface (CLI) and API for installing, switching, removing and listing
Candidates.

SUBCOMMANDS:
    help              [SUBCOMMAND]
    install   or i    <CANDIDATE> [VERSION] [LOCAL_PATH]
    uninstall or rm   <CANDIDATE> <VERSION>
    list      or ls   [CANDIDATE]
    use       or u    <CANDIDATE> <VERSION>
    config
    default   or d    <CANDIDATE> [VERSION]
    home      or h    <CANDIDATE> <VERSION>
    env       or e    [INIT|INSTALL|CLEAR]
    current   or c    [CANDIDATE]
    upgrade   or ug   [CANDIDATE]
    version   or v
    broadcast or b
    offline           [ENABLE|DISABLE]
    selfupdate        [FORCE]
    update
    flush             [ARCHIVE|TMP|BROADCAST|METADATA|VERSION]

EXAMPLES:
    $ sdk install java 17.0.0-tem
    $ sdk help install
";

const INSTALL_HELP: &str = "\
sdk install

The sdk subcommand to install a candidate version.

USAGE:
    sdk install <CANDIDATE> [VERSION] [LOCAL_PATH]

Invoking this subcommand with only the candidate as a parameter will install the
currently known default version for that candidate.

Provide a subsequent qualifier to install a specific non-default version. This
subcommand will return a non-zero exit code for unfound versions.

Provide another qualifier to add an already installed local version. This
qualifier is the absolute local path to the base directory of the SDK to be
added. The local version will appear as an installed version of the candidate.
The version may not conflict with an existing version, installed or not. The
subcommand will return a non-zero return code if the directory does not exist.

The shorthand mnemonic 'i' is provided in the place of the install subcommand.

EXAMPLES:
    $ sdk install java
    $ sdk install java 17.0.0-tem
    $ sdk install java 11-local /usr/lib/jvm/java-11-openjdk
";

const UNINSTALL_HELP: &str = "\
sdk uninstall

The sdk subcommand to uninstall a candidate version.

USAGE:
    sdk uninstall <CANDIDATE> <VERSION>

Always follow the subcommand with two qualifiers, the candidate and version to
be uninstalled.

The specified version will be removed from the candidate directory in
$SDKMAN_DIR/candidates and will no longer be available for use on the system.
The binary archive for the SDK will be cached in the archives folder if
re-installation is required later.

An invalid candidate or version supplied to the subcommand will result in a
non-zero return code.

The alias 'rm' is provided as a shorthand alternative to uninstall.

EXAMPLE:
    $ sdk uninstall java 17.0.0-tem
";

const LIST_HELP: &str = "\
sdk list

The sdk subcommand to list all candidates or candidate versions.

USAGE:
    sdk list [CANDIDATE]

Invoke the subcommand without a candidate to see a comprehensive list of all
candidates with name, URL, detailed description and an installation command.

If the candidate qualifier is specified, the subcommand will display a list of
all available and local versions for that candidate. In addition, the version
list view marks all versions that are local, installed or currently in use.
They appear as follows:

+ - local version
* - installed
> - currently in use

Java has a custom list view with vendor-specific details.

The alias 'ls' is provided as a shorthand alternative to list.

EXAMPLE:
    $ sdk list
    $ sdk list java
    $ sdk list groovy
";

const USE_HELP: &str = "\
sdk use

An sdk subcommand to use a specific version in the current shell.

USAGE:
    sdk use <CANDIDATE> <VERSION>

The mandatory candidate and version follow the subcommand to specify what to
use in the current shell.

This subcommand only operates on the current shell. It does not affect other
shells running different versions of the same candidate. It also does not change
the default version set for all subsequent new shells.

The subcommand will return a non-zero return code if the candidate or version
does not exist.

The shorthand mnemonic 'u' is provided in the place of the use subcommand.

EXAMPLE:
    $ sdk use java 17.0.0-tem
";
