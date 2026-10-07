# SDKMAN! CLI

The vocabulary SDKMAN! uses when it talks to users on the command line: in responses, errors and help pages. Every user-facing string should use these terms and nothing else.

## Language

### Things

**SDK**:
A named piece of software that SDKMAN! can install and manage, such as `java`, `maven` or `gradle`.
_Avoid_: candidate, tool, package

**Version**:
One numbered edition of an SDK, such as `21.0.2`. Written together with its SDK, and its distribution where there is one, as "java 21.0.2 (Temurin)", without a separate noun for the combination. In commands the user types, the distribution's short name is still joined to the version: `21.0.2-tem`. This short form is a legacy of the core and goes away when the new native commands take version and distribution separately.
_Avoid_: release, build, identifier

**Distribution**:
A named line of versions of an SDK published by one provider, such as Temurin or Zulu for java. Shown by its full name in messages and by its short name, such as `tem`, in commands.
_Avoid_: vendor, flavour

**Recommended version**:
The version SDKMAN! installs when the user names no version. For java, help may call it the latest LTS version. This is the only allowed use of "latest".
_Avoid_: default version, latest version, stable version

**Local version**:
A version that SDKMAN! points to in an existing directory on the user's machine instead of downloading it.
_Avoid_: linked version, external version, custom version

**Project config**:
The `.sdkmanrc` file in a project directory that lists which version of each SDK the project needs. Name the file in brackets when the user needs to find it: "project config (.sdkmanrc)".
_Avoid_: descriptor, environment, rc file

### SDKMAN! itself

**SDKMAN! directory**:
The directory where SDKMAN! keeps everything it installs and knows, usually `~/.sdkman`, and named by `$SDKMAN_DIR`.
_Avoid_: SDKMAN! home, install dir

**SDKMAN! config**:
The user's settings file for SDKMAN!, at `$SDKMAN_DIR/etc/config`.
_Avoid_: settings, preferences, properties

**SDKMAN! version**:
The version of SDKMAN! itself, as opposed to the version of an SDK.
_Avoid_: CLI version, script version

**Core**:
The part of SDKMAN! that runs inside the user's shell, provides the `sdk` command and calls the native commands.
_Avoid_: script, wrapper, bash CLI

**Native**:
The part of SDKMAN! made of compiled commands that the core calls.
_Avoid_: binaries, Rust CLI

### The command line

**Command**:
A word after `sdk` that selects what SDKMAN! does, such as `install` or `current`, or a word after another command that selects what it does, such as `init` in `sdk env init`.
_Avoid_: subcommand

**Alias**:
A short form of a command, such as `rm` for `uninstall`.
_Avoid_: mnemonic, shorthand

**Argument**:
A value given to a command by position, such as the SDK or version.
_Avoid_: qualifier, parameter

**Option**:
A named setting, written with leading dashes, that changes how a command behaves, such as `--force`.
_Avoid_: flag, qualifier

**Shell**:
One running shell, such as bash or zsh, that the user types commands into. Settings made with **Use** last only for that shell.
_Avoid_: session, terminal, window

### States

**Default version**:
The version of an SDK that every new shell gets unless something overrides it.
_Avoid_: current version, global version

**In use**:
Describes the version of an SDK that is active in the shell the user is typing in, which may differ from the default version.
_Avoid_: current, active, selected

### Actions

Each action is named after its command. Command names and aliases, such as `sdk current` and `rm`, are exempt from the _Avoid_ lists.

**Install**:
Download a version of an SDK and make it available on this machine.
_Avoid_: add, get, fetch

**Uninstall**:
Take an installed version of an SDK off this machine.
_Avoid_: remove, delete

**Set as default**:
Make a version the default version of its SDK.
_Avoid_: switch, select, make current

**Use**:
Put a version in use in this shell only, without changing the default version.
_Avoid_: switch, activate, select

**Update**:
Download the newest list of SDKs and versions from the SDKMAN! servers.
_Avoid_: refresh, sync

**Upgrade**:
Install a newer version of an SDK and make it the default version.
_Avoid_: update

**Selfupdate**:
Replace SDKMAN! with a newer SDKMAN! version.
_Avoid_: upgrade SDKMAN!

**Flush**:
Discard temporary files and cached data that SDKMAN! keeps for itself.
_Avoid_: clean, clear, purge
