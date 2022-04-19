pub const MAIN_HELP: &str = "\
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

pub const BROADCAST_HELP: &str = "\
sdk broadcast

The sdk subcommand to display the latest announcements on SDKMAN.

USAGE:
    sdk broadcast

This subcommand displays the latest three vendor announcements about SDK
releases on SDKMAN. Each entry shows the release date and broadcast message
issued by a vendor.

The shorthand mnemonic 'b' is provided in place of the broadcast subcommand.

EXAMPLE:
    $ sdk broadcast
";

pub const CONFIG_HELP: &str = "\
sdk config

The sdk subcommand to edit the SDKMAN configuration file.

USAGE:
    sdk config

This subcommand opens a text editor on the configuration file located at
${SDKMAN_DIR}/etc/config. The subcommand will infer the text editor from the
EDITOR environment variable. If the system does not set the EDITOR environment
variable, then vi is assumed as the default editor.

EXAMPLE:
    $ sdk config
";

pub const CURRENT_HELP: &str = "\
sdk current

An sdk command to display the current default installed versions of candidates
on the system.

USAGE:
    sdk current [CANDIDATE]

This command will display a list of candidates with their default version
installed on the system. It is also possible to qualify the candidate when
running the command to display only that candidate's default version.

EXAMPLE:
    $ sdk current
    $ sdk current java
";

pub const DEFAULT_HELP: &str = "\
sdk default

The sdk subcommand to set the local default version of the candidate.

USAGE:
    sdk default <CANDIDATE> [VERSION]

The mandatory candidate qualifier of the subcommand specifies the candidate to
default for all future shells.

The optional version qualifier set that specific version as default for all
subsequent shells on the local environment. Omitting the version will set the
global SDKMAN tracked version as the default version for that candidate.

The subcommand will return a non-zero return code if the candidate or version
does not exist.

The shorthand mnemonic 'd' is provided in the place of the default subcommand.

EXAMPLE:
    $ sdk default java 17.0.0-tem
    $ sdk default java
";

pub const ENV_HELP: &str = "\
sdk env

The sdk subcommand to control SDKs on a project level, setting up specific
versions for a directory.

USAGE:
    sdk env [init|install|clear]

Allows the developer to manage the SDK versions used in a project directory. The
subcommand uses a `.sdkmanrc` file to install or switch specific SDK versions in
a project directory.

The absence of a qualifier will switch to the versions specified in `.sdkmanrc`
and emits warnings for versions not present on the system. In addition, it has
three optional qualifiers:

`install` :     Install and switch to the SDK versions specified in `.sdkmanrc`.
                The subcommand will use this as the default if the qualifier is
                omitted.
`init`    :     Allows for the creation of a default `.sdkmanrc` file with a
                single entry for the `java` candidate. It uses the current
                system default of `java` to prime this file.
`clear`   :     Reset all SDK versions to their system defaults

The `.sdkmanrc` file contains key-value pairs for each configurable SDK for that
project environment. An initial file will content such as this:

---
# Enable auto-env through the sdkman_auto_env config
# Add key=value pairs of SDKs to use below
java=11.0.13-tem
---

You may enable a configuration option for auto-env behaviour. This setting will
automatically switch versions when stepping into a directory on the presence of
a `.sdkmanrc` descriptor. When enabled, you no longer need to issue the
`install` qualifier explicitly. This behaviour is disabled by default.

EXAMPLE:
    $ sdk env
    $ sdk env install
    $ sdk env init
    $ sdk env clear
";

pub const HOME_HELP: &str = "\
sdk home

The sdk subcommand, used in scripting to output the location of a specific
candidate version.

USAGE:
    sdk home <CANDIDATE> <VERSION>

Print the absolute home path of any candidate version installed by SDKMAN. The
candidate and version parameters are mandatory. The subcommand will emit a
non-zero exit code if a valid candidate version is not locally installed.

EXAMPLE:
    $ sdk home java 17.0.0-tem
";

pub const INSTALL_HELP: &str = "\
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
subcommand will return a non-zero exit code if the directory does not exist.

The shorthand mnemonic 'i' is provided in the place of the install subcommand.

EXAMPLES:
    $ sdk install java
    $ sdk install java 17.0.0-tem
    $ sdk install java 11-local /usr/lib/jvm/java-11-openjdk
";

pub const LIST_HELP: &str = "\
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

pub const SELFUPDATE_HELP: &str = "\
sdk selfupdate

The sdk command to upgrade the core of SDKMAN!

USAGE:
    sdk selfupdate [FORCE]

Invoke this command to upgrade the core script and native components of the
SDKMAN! command-line interface. The command will only upgrade the native
components if the detected platform is supported.

The command will refuse to upgrade the core if no new version is available. A
qualifier may be added to the selfupdate command to force an upgrade.

EXAMPLES:
    $ sdk selfupdate
    $ sdk selfupdate force
";

pub const UNINSTALL_HELP: &str = "\
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

pub const UPGRADE_HELP: &str = "\
sdk upgrade

The sdk subcommand to upgrade installed candidate versions.

USAGE:
    sdk upgrade [CANDIDATE]

The optional candidate qualifier can be applied to specify the candidate you
want to upgrade. If the candidate qualifier is omitted from the command, it will
attempt an upgrade of all outdated candidates.

Candidates that do not require an upgrade will be omitted, and a notification
will be displayed that the candidates are up to date.

The subcommand will return a non-zero return code if the candidate does not
exist.

The shorthand mnemonic 'ug' is provided in the place of the use subcommand.

EXAMPLE:
    $ sdk upgrade
    $ sdk upgrade java
";

pub const USE_HELP: &str = "\
sdk use

The sdk subcommand to use a specific version in the current shell.

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

pub const VERSION_HELP: &str = "\
sdk version

The sdk subcommand to display the installed SDKMAN version.

USAGE:
    sdk version

This subcommand displays the version of the bash and native constituents of
SDKMAN on this system. The versions of the bash and native libraries evolve
independently from each other and so will not be the same.

The alias 'v' is provided as a shorthand alternative to version.

EXAMPLE:
    $ sdk version
";
