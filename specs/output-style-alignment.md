# Output Style Alignment Specification

## Overview

[`docs/STYLE.md`](../docs/STYLE.md), [`CONTEXT.md`](../CONTEXT.md) and ADRs [0001](../docs/adr/0001-sdk-in-text-candidate-in-code.md)–[0005](../docs/adr/0005-clap-style-help.md) now define how SDKMAN! talks to users: one warm, plain voice, a fixed set of message types, semantic styling, per-stream colour and clap-style help pages. None of the code follows them yet. Today:

1. Every binary formats its own output with ad-hoc `colored` calls. Casing, punctuation, tense and vocabulary differ from line to line (`setting java …` / `removed java …` / `No current version of java configured.`).
2. Success messages go to stdout, mixed with data. `uninstall` splits one error across stderr and stdout.
3. Unexpected failures reach users as Rust panics (`panic! could not delete directory.`, exit code 101).
4. Colour is decided by `colored`, which only checks whether stdout is a terminal and ignores `sdkman_colour_enable`.
5. Help text lives in two places that disagree: clap `about` strings in each binary, and a hand-rendered man-page layout in `src/bin/help.rs` full of "candidate", "qualifier" and "sdk subcommand to".

This spec brings the whole codebase in line with the guide. It is a user-visible change to wording, streams, exit codes and help, but **not** to what any command does.

**Key Properties:**
- A single `ui` module in the library is the only code that writes messages or applies styles. Binaries never call `println!`, `eprintln!` or a styling API directly.
- Styling uses `anstream` and `anstyle`, the stack clap already uses, so our messages and clap's help and errors share one colour decision.
- Every fallible operation returns an error to `main`. Nothing a user can trigger panics or calls `process::exit` outside `main`.
- Each command's clap definition lives in the library and is the single source of its help. `sdk help <command>` and `sdk <command> --help` print identical text.
- Every user-facing string uses the CONTEXT.md vocabulary and the STYLE.md shapes.

## Context

The bash core dispatches to the native commands directly (`"$native_command" "${@:2}"` in `sdkman-main.sh`). It does not capture their output, so moving conversational output to stderr is safe for the core. The core *sources* `$SDKMAN_DIR/etc/config` but does not export its variables, so the native commands must read `sdkman_colour_enable` from the file themselves.

With `sdkman_native_enable=true` (the installer's default), `sdk help <command>` runs the native `help` binary. With no command or an unknown command, the core runs it with no arguments. It must therefore carry help for every command, including the ones still implemented in bash (`install`, `list`, `use`, `env`, `flush`, `config`, `selfupdate`, `update`, `upgrade`).

## Non-Goals

**Out of scope for this spec:**
- **What `sdk current` reports.** It keeps reading the `current` symlink, which is the default version. Whether it should report the version in use (as the bash version does) is a separate decision. This spec only makes its wording honest: it talks about default versions.
- **Distribution display.** STYLE.md's target form `java 21.0.2 (Temurin)` needs distribution names that are not yet available locally. They arrive with the native `sdk update`, the first command to be ported after this spec, which stores the state from sdkman-state in a local SQLite database with version and distribution in separate fields. Until then, messages keep the legacy form `java 21.0.2-tem` (see R2). A later spec switches over.
- **Prompts.** No native command prompts today. The prompt shape in STYLE.md is implemented when the first prompting command moves to native code.
- **Behaviour changes.** No command gains or loses functionality. In particular, `sdk default` keeps requiring a version, even though the bash version makes it optional. The one accepted exception is `sdk version`, which today ignores its arguments: once it parses with its `cli` definition, extra arguments are a clap error (exit `2`) and `--help` prints its help page.
- **The bash core.** No change to `sdkman-cli`.
- **Man pages** and terminal hyperlinks.

## Requirements

### Foundation

- **R1. One output module.** Add a `ui` module to `src/lib.rs` that provides every message type in STYLE.md (success, info, warning, error, hint) plus data output (single values and titled tables). Binaries write output only through it. No `println!`, `eprintln!`, `print!` or styling calls remain in `src/bin/`.
- **R2. Interim version form.** Messages write an SDK and version as `java 21.0.2-tem`, with the SDK and version bold. STYLE.md gets a short, dated note that records this as an interim exception until the native `sdk update` stores distribution names locally.
- **R3. Streams.** Success lines, info lines, warnings, errors and hints go to stderr. Only data goes to stdout: the path from `sdk home`, the output of `sdk current` and `sdk version`, and help pages.
- **R4. Colour decision.** Styling is decided once at startup and applied to each stream separately:
  1. `CLICOLOR_FORCE` set to a non-zero value: styling on.
  2. `NO_COLOR` set to any non-empty value, `CLICOLOR=0`, or `sdkman_colour_enable=false` in `$SDKMAN_DIR/etc/config`: styling off.
  3. Otherwise, each stream is styled only if it is a terminal.

  A missing or unreadable config file counts as "not set". clap's help and error output follow the same decision.
- **R5. Unstyled values.** The single value printed by `sdk home` or `sdk current <sdk>` is never styled, not even under `CLICOLOR_FORCE`.
- **R6. Errors return to `main`.** Every binary has the shape `fn main() -> ExitCode { … run() … }`. Library helpers return `Result` instead of calling `process::exit`, `panic!`, `expect` or `unwrap` on anything a user can trigger, such as a missing file, a bad argument or an I/O failure.
- **R7. Exit codes.** `0` for success, including success with warnings and empty results. `1` for errors from SDKMAN!. `2` for invalid command lines reported by clap. Exit code `101` (panic) no longer occurs.
- **R8. Paths in messages.** Paths in messages abbreviate the user's home directory to `~` (for example `~/.sdkman/var/candidates`). Data output (`sdk home`) keeps the absolute path. Paths are written with `Path::display`, never `to_str().unwrap()`, so a path that is not valid UTF-8 never fails.

### Messages

- **R9.** Every existing message is replaced as listed in the [Message Catalogue](#message-catalogue). No other wording is introduced without updating that catalogue in this spec.
- **R10. Unknown SDK suggestions.** When an SDK name is unknown, the error hint suggests the closest known SDK, using the same rule clap uses for its own suggestions: `strsim::jaro` (already in `Cargo.lock` via clap), with a score above `0.7`. On a tie, the SDK listed first in `var/candidates` wins. If no SDK scores above `0.7`, the hint points to `sdk list`. For example, `jav` suggests `java`, and `xyz` points to `sdk list`.

### Help

- **R11. Shared definitions.** Each command's clap definition moves to a `cli` module in the library, one function or derive struct per command. The native command binaries parse with it, and the `help` binary renders help from it. Commands still implemented in bash get a definition used only for help.
- **R12. Layout.** Help follows STYLE.md: tagline, `Usage:`, description, `Commands:`, `Arguments:`, `Options:`, `Aliases:`, `Configuration:`, `Exit status:`, `Examples:`. `-h` shows the short form (tagline, usage, commands, arguments, options and aliases), while `--help` and `sdk help <command>` show everything. Content is listed in the [Help Catalogue](#help-catalogue).
- **R13. Help styling.** Headings bold, not underlined. Usage lines, example lines, commands and options cyan. Placeholders plain. Help wraps to the terminal width, up to 80 columns. The help option reads `-h, --help  Print help` in both forms, with no `(see a summary with '-h')`. The native commands do not offer `-V`/`--version`.
- **R14. clap errors.** clap's parse errors are restyled to match STYLE.md (see [clap error mapping](#clap-error-mapping)), go to stderr and exit with code `2`. Help requested through `-h` or `--help` goes to stdout and exits with code `0`.

### Cleanup

- **R15.** `colored` is removed from `Cargo.toml`. `textwrap` is removed if nothing still uses it once clap does the wrapping.
- **R16.** The project description in `CLAUDE.md` mentions the `ui` and `cli` modules next to `helpers` and `constants`.

## Rules

**Before implementing, read and internalise:**
- [`docs/STYLE.md`](../docs/STYLE.md): every message, help page and styling choice.
- [`CONTEXT.md`](../CONTEXT.md): the only vocabulary allowed in user-facing text.
- [`docs/adr/`](../docs/adr/): the reasons behind the less obvious rules.
- The project and workspace `CLAUDE.md`: testing approach, small commits, `cargo fmt` and `cargo clippy` before each commit.

**If this spec conflicts with STYLE.md or CONTEXT.md, this spec wins.**

## Design

### The `ui` module

An illustrative sketch. Names may change, but the responsibilities may not:

```rust
pub mod ui {
    /// Decides styling for stdout and stderr (R4). Called first in every main.
    pub fn init(sdkman_dir: &Path);

    // Messages, all to stderr
    pub fn success(text: impl Display);   // "✓ Installed java 21.0.2-tem"
    pub fn info(text: impl Display);      // "No SDK has a default version"
    pub fn warning(text: impl Display);   // "warning: …"
    pub fn hint(text: impl Display);      // "  hint: …", always after another message

    // Data, to stdout
    pub fn value(text: impl Display);                   // never styled (R5)
    pub fn table(title: &str, rows: &[(String, String)]); // bold title, aligned rows

    // Semantic tokens, used to build message text
    pub fn sdk(name: &str) -> impl Display;                   // bold
    pub fn sdk_version(sdk: &str, version: &str) -> impl Display; // "java 21.0.2-tem", bold
    pub fn cmd(text: &str) -> impl Display;                   // cyan
    pub fn path(path: &Path) -> impl Display;                 // plain, "~" abbreviated (R8)
}

pub struct CliError {
    pub message: String,    // rendered after "error: "
    pub hints: Vec<String>, // at most two, rendered as hint lines
}
```

`main` renders a `CliError` as `error:` plus its hints on stderr and returns exit code `1`.

### Colour with `anstream` and `anstyle`

- Styles are `anstyle::Style` values, such as bold, `AnsiColor::Green`, `AnsiColor::Cyan` and dimmed.
- Messages are written through `anstream::stderr()` and data through `anstream::stdout()`.
- `ui::init` makes the decision itself, because `anstream` checks `NO_COLOR` before `CLICOLOR_FORCE`, the reverse of R4. A pure function `colour_choice(clicolor_force, no_color, clicolor, colour_enable) -> ColorChoice` applies R4 rules 1 and 2 in order and returns `Always`, `Never`, or `Auto` when neither applies. `ui::init` reads the environment and the config, calls it, and sets the result globally with `ColorChoice::write_global`. clap reads the same global choice, so help and errors follow it.
- With `Auto`, `AutoStream` does only the per-stream terminal check (R4 rule 3).
- Reading `etc/config` is a small `key=value` line parser. It only needs `sdkman_colour_enable`, ignores comments and blank lines, and treats a missing file as empty.

### clap integration

- `cli` holds one builder function or derive struct per command, plus a `configure(Command) -> Command` helper that applies the shared settings:
  - `Styles`: header bold (no underline), usage bold, literal cyan, placeholder plain, error bold red, valid green, invalid yellow.
  - `max_term_width(80)` with clap's `wrap_help` feature.
  - `disable_help_flag(true)`, plus one explicit `help` argument with `-h` and `--help` (`ArgAction::Help`), with `help` and `long_help` both set to `Print help`. clap shows the short form for `-h` and the long form for `--help`, and lists the argument as one `-h, --help  Print help` row.
  - `disable_version_flag(true)`.
  - `disable_help_subcommand(true)`, so clap adds no `help` row to `Commands:`. The `help` row on the `sdk` page comes from the `cli` definition of `help`.
  - A `help_template` that gives the section order in R12. The description goes in `before_long_help` and is placed after `{usage}`. Aliases go in `after_help`. Configuration, exit status and examples go in `after_long_help` with the aliases repeated first, so that `-h` shows aliases but not examples.
- Each `Commands:` entry, such as `env init`, is a clap subcommand used for help. For bash commands this is display only.
- The `help` binary builds `Command::new("sdk")` from every `cli` definition, with `override_usage("sdk <COMMAND> [ARGUMENTS]")`. This is the only usage line not generated by clap. `sdk help` prints its long help. `sdk help <command>` does not parse `<command>` against its definition, because required arguments would fail. Instead it looks the name up with `Command::find_subcommand`, which also resolves aliases (`sdk help rm`), and prints that command's long help. If nothing matches, it picks the closest command name with the R10 rule and prints the `InvalidSubcommand` row of the clap error mapping itself.

### clap error mapping

Binaries call `try_parse()`. On an error:

| clap `ErrorKind` | Output (stderr, exit 2) |
|---|---|
| `DisplayHelp` | the help, on **stdout**, exit 0 |
| `UnknownArgument`, value starts with `-` | `error: unknown option --forse` / `  hint: did you mean --force?` (when clap has a suggestion) / `  hint: run sdk uninstall --help` |
| `UnknownArgument`, any other value | `error: unexpected argument extra` / `  hint: run sdk <command> --help` |
| `MissingRequiredArgument`, one argument | `error: missing argument <VERSION>` / `  hint: run sdk uninstall --help` |
| `MissingRequiredArgument`, several arguments | `error: missing arguments <SDK> <VERSION>` / `  hint: run sdk uninstall --help` |
| `InvalidSubcommand` (help binary, built by the binary, not from clap's error) | `error: unknown command instal` / `  hint: did you mean install?` or `  hint: run sdk help to see all commands` |
| anything else | clap's message, with the first letter lowercased, quotes removed, no trailing full stop, `tip:` rendered as `hint:`, and the closing "For more information…" replaced by `hint: run sdk <command> --help` |

Values clap reports, such as argument names, are read from the error's context (`ContextKind::InvalidArg`, `SuggestedArg`, `SuggestedSubcommand` and so on), not parsed from its text.

## Message Catalogue

Short form until R2's interim ends: `java 21.0.2-tem`. **Bold** marks the SDK and version, and `cmd` marks cyan commands. "Shared" rows come from library helpers used by several commands.

### Shared (library)

| Situation | Today | After |
|---|---|---|
| Unknown SDK | stderr `jav is not a valid candidate.` exit 1 | `error: unknown SDK` **jav** / `  hint: did you mean` **java**`?`, or `  hint: run` `sdk list` `to see all SDKs` |
| Version not installed (`default`, `home`) | stderr `java 17.0.0-tem is not installed on your system` | `error:` **java 17.0.0-tem** `is not installed` / `  hint: run` `sdk install java 17.0.0-tem` |
| Version not installed (`uninstall`) | as above | `error:` **java 17.0.0-tem** `is not installed` / `  hint: run` `sdk list java` `to see installed versions` |
| SDK list file missing | panic `not a valid path: …/var/candidates`, exit 101 | `error: cannot read ~/.sdkman/var/candidates: no such file or directory` / `  hint: run` `sdk update` |
| SDK list file empty | panic `the candidates file is missing: …`, exit 101 | `error: no SDKs found in ~/.sdkman/var/candidates` / `  hint: run` `sdk update` |
| No `SDKMAN_DIR` and no home directory | panic, exit 101 | `error: cannot find your home directory`, no hint, exit 1 |

### `sdk current`

| Situation | Today | After |
|---|---|---|
| One SDK, has a default | stdout `Current default java version 21.0.2-tem` | stdout `21.0.2-tem` only, unstyled like `sdk home` (R5), so `$(sdk current java)` works in scripts |
| One SDK, no default | stderr `No current version of java configured.` exit 1 | `error:` **java** `has no default version` / `  hint: run` `sdk default java <version>`, exit 1 |
| All SDKs | stdout bold `Current default versions:` then unaligned `java 21.0.2-tem` rows | stdout table: bold `Default versions`, then rows indented two spaces with the version column aligned |
| No SDK has a default | stderr `No candidates are in use.` exit 0 | info `No SDK has a default version` / `  hint: run` `sdk install <sdk>`, exit 0 |

### `sdk default`

| Situation | Today | After |
|---|---|---|
| Success | stdout, printed **before** the change: `setting java 21.0.2-tem as the default version for all shells.` | `✓ Set` **java 21.0.2-tem** `as the default version`, printed **after** the change succeeds |
| Symlink not possible, copied instead | stdout bold `cannot create current symlink, fall back to copy!` | the success line, then `warning: cannot create a symlink here, so SDKMAN! copied` **java 21.0.2-tem** `instead` |
| Cannot remove old default | panic `cannot remove current directory for java.` | `error: cannot remove ~/.sdkman/candidates/java/current: <os reason>` |
| Copy or rename fails | panic `cannot copy to tmp folder.` / `cannot rename copied folder.` | `error: cannot copy <source> to <target>: <os reason>` / `error: cannot move <source> to <target>: <os reason>` |

### `sdk home`

| Situation | Today | After |
|---|---|---|
| Success | stdout absolute path | unchanged (unstyled, R5) |
| Not installed | stderr `java 17.0.0-tem is not installed on your system.` | shared "Version not installed (`default`, `home`)" row |

### `sdk uninstall`

| Situation | Today | After |
|---|---|---|
| Success | stdout `removed java 17.0.0-tem.` | `✓ Uninstalled` **java 17.0.0-tem** |
| Default version, no `--force` | stderr `\njava 17.0.0-tem is the current version and should not be removed.` + stdout `\n\nOverride with --force, but leaves the candidate unusable!` | `error:` **java 17.0.0-tem** `is the default version` / `  hint: run` `sdk default java <version>` `first` / `  hint: or run` `sdk uninstall --force java 17.0.0-tem`, exit 1, all on stderr with no blank lines |
| Default version, `--force` | stdout `removed java 17.0.0-tem.` | `✓ Uninstalled` **java 17.0.0-tem** / `warning:` **java** `has no default version now` |
| Default link cannot be read (behaviour unchanged: uninstall continues) | stderr `current link broken, stepping over: <io error>` | `warning: cannot tell which version is the default version of` **java**`: <os reason>` |
| Delete fails | panic `panic! could not delete directory.` | `error: cannot remove ~/.sdkman/candidates/java/17.0.0-tem: <os reason>` |
| Cannot remove default link (forced) | panic `cannot remove current directory for java.` | `error: cannot remove ~/.sdkman/candidates/java/current: <os reason>` |

### `sdk version`

| Situation | Today | After |
|---|---|---|
| Success | stdout `\nSDKMAN!\nscript: 5.19.0\nnative: 0.7.35 (linux x86_64)\n\n` | stdout `\nSDKMAN!\ncore:   5.19.0\nnative: 0.7.35 (linux x86_64)\n\n`, with `SDKMAN!` bold yellow on a terminal and values aligned |
| Version file missing or empty | panic, exit 101 | `error: cannot read ~/.sdkman/var/version: no such file or directory` (or `: file is empty`), no hint, exit 1 |

### `sdk help`

| Situation | Today | After |
|---|---|---|
| Unknown command | clap's default error | clap error mapping, `InvalidSubcommand` row |

## Help Catalogue

Every page below is the `--help` form. The `-h` form drops the description, configuration, exit status and examples. Arguments and options are generated by clap from the definitions, and their descriptions are listed here.

### `sdk` (main help, `sdk help`)

```
The command line interface for SDKMAN!

Usage: sdk <COMMAND> [ARGUMENTS]

SDKMAN! installs and manages versions of SDKs, such as java, maven and gradle,
and sets which version each shell uses.

Commands:
  install     Install a version of an SDK [aliases: i]
  uninstall   Uninstall a version of an SDK [aliases: rm]
  list        List SDKs or the versions of an SDK [aliases: ls]
  use         Use a version of an SDK in this shell [aliases: u]
  default     Set the default version of an SDK [aliases: d]
  current     Show the default version of one or all SDKs [aliases: c]
  home        Print the directory of an installed version [aliases: h]
  env         Use the versions in a project config [aliases: e]
  upgrade     Upgrade SDKs to their recommended version [aliases: ug]
  update      Update the list of SDKs and versions
  selfupdate  Replace SDKMAN! with a newer version
  flush       Flush temporary files and cached data
  config      Edit the SDKMAN! config
  version     Show the SDKMAN! version [aliases: v]
  help        Show help for a command

Options:
  -h, --help  Print help

Examples:
  sdk install java
  sdk help install
```

### `sdk install`

```
Install a version of an SDK

Usage: sdk install <SDK> [VERSION] [PATH]

Installs a version of an SDK on this machine. Without a version, installs the
recommended version. For java, this is the latest LTS version.

Add a path to make a local version: SDKMAN! points to the SDK in that
directory instead of downloading it. The version must not clash with any other
version of the SDK.

Arguments:
  <SDK>      The SDK, such as java
  [VERSION]  The version to install
  [PATH]     The directory of an SDK already on this machine

Options:
  -h, --help  Print help

Aliases: i

Exit status:
  Exits with a non-zero code if the version does not exist or the path is not
  a directory.

Examples:
  sdk install java
  sdk install java 21.0.2-tem
  sdk install java 11-local /usr/lib/jvm/java-11-openjdk
```

### `sdk uninstall`

```
Uninstall a version of an SDK

Usage: sdk uninstall [OPTIONS] <SDK> <VERSION>

Uninstalls a version of an SDK from this machine. You cannot uninstall the
default version unless you add --force.

Arguments:
  <SDK>      The SDK, such as java
  <VERSION>  The installed version to uninstall

Options:
  -f, --force  Uninstall even if it is the default version
  -h, --help   Print help

Aliases: rm

Exit status:
  Exits with a non-zero code if the SDK or version is not installed, or if the
  version is the default version and --force is not given.

Examples:
  sdk uninstall java 17.0.0-tem
  sdk rm --force java 17.0.0-tem
```

### `sdk list`

```
List SDKs or the versions of an SDK

Usage: sdk list [SDK]

Without an SDK, lists every SDK with its website, a description and the
command to install it.

With an SDK, lists its versions and marks the ones on this machine:

    +  local version
    *  installed
    >  in use

For java, the list also shows the distribution of each version.

Arguments:
  [SDK]  The SDK whose versions to list

Options:
  -h, --help  Print help

Aliases: ls

Examples:
  sdk list
  sdk list java
```

### `sdk use`

```
Use a version of an SDK in this shell

Usage: sdk use <SDK> <VERSION>

Puts a version in use in this shell only. Other shells and the default version
stay as they are.

Arguments:
  <SDK>      The SDK, such as java
  <VERSION>  The installed version to use

Options:
  -h, --help  Print help

Aliases: u

Exit status:
  Exits with a non-zero code if the SDK or version is not installed.

Examples:
  sdk use java 17.0.0-tem
```

### `sdk default`

```
Set the default version of an SDK

Usage: sdk default <SDK> <VERSION>

Makes an installed version the default version of its SDK, so every new shell
uses it.

Arguments:
  <SDK>      The SDK, such as java
  <VERSION>  The installed version to make the default

Options:
  -h, --help  Print help

Aliases: d

Exit status:
  Exits with a non-zero code if the SDK or version is not installed.

Examples:
  sdk default java 21.0.2-tem
```

### `sdk current`

```
Show the default version of one or all SDKs

Usage: sdk current [SDK]

Without an SDK, lists every SDK that has a default version. With an SDK, shows
the default version of that SDK.

Arguments:
  [SDK]  The SDK whose default version to show

Options:
  -h, --help  Print help

Aliases: c

Exit status:
  Exits with a non-zero code if the SDK does not exist or has no default
  version.

Examples:
  sdk current
  sdk current java
```

### `sdk home`

```
Print the directory of an installed version

Usage: sdk home <SDK> <VERSION>

Prints the absolute path of an installed version and nothing else, so that
scripts can use it.

Arguments:
  <SDK>      The SDK, such as java
  <VERSION>  The installed version

Options:
  -h, --help  Print help

Aliases: h

Exit status:
  Exits with a non-zero code if the SDK or version is not installed.

Examples:
  sdk home java 21.0.2-tem
  export JAVA_HOME=$(sdk home java 21.0.2-tem)
```

### `sdk env`

```
Use the versions in a project config

Usage: sdk env [COMMAND]

Without a command, puts the versions in the project config (.sdkmanrc) of this
directory in use in this shell, and warns about any that are not installed.

Commands:
  init     Create a project config with the default version of java
  install  Install the versions in the project config and use them
  clear    Go back to the default versions

Options:
  -h, --help  Print help

Aliases: e

Configuration:
  Set sdkman_auto_env=true in the SDKMAN! config to use a project config
  automatically whenever you change into its directory. A project config looks
  like this:

    # Add key=value pairs of SDKs to use below
    java=21.0.2-tem

Examples:
  sdk env
  sdk env init
  sdk env install
  sdk env clear
```

### `sdk upgrade`

```
Upgrade SDKs to their recommended version

Usage: sdk upgrade [SDK]

Installs the recommended version of an SDK and makes it the default version,
if it is newer than the default version you have. Without an SDK, upgrades
every SDK that has a newer recommended version.

Arguments:
  [SDK]  The SDK to upgrade

Options:
  -h, --help  Print help

Aliases: ug

Exit status:
  Exits with a non-zero code if the SDK does not exist.

Examples:
  sdk upgrade
  sdk upgrade java
```

### `sdk update`

```
Update the list of SDKs and versions

Usage: sdk update

Downloads the newest list of SDKs and versions from the SDKMAN! servers. Other
commands use this list to install, upgrade and list versions. Run it often to
see new versions.

Options:
  -h, --help  Print help

Examples:
  sdk update
```

### `sdk selfupdate`

```
Replace SDKMAN! with a newer version

Usage: sdk selfupdate [COMMAND]

Replaces the core and the native commands with the newest SDKMAN! version. The
native commands are only replaced on supported platforms. If there is no newer
version, nothing changes unless you add force.

Commands:
  force  Reinstall even if there is no newer version

Options:
  -h, --help  Print help

Examples:
  sdk selfupdate
  sdk selfupdate force
```

### `sdk flush`

```
Flush temporary files and cached data

Usage: sdk flush [COMMAND]

Discards temporary files and cached data that SDKMAN! keeps for itself.
Without a command, flushes everything.

Commands:
  tmp       Flush downloaded archives and leftover hook scripts
  metadata  Flush cached metadata about SDKs
  version   Flush the cached SDKMAN! versions

Options:
  -h, --help  Print help

Examples:
  sdk flush
  sdk flush tmp
  sdk flush metadata
  sdk flush version
```

### `sdk config`

```
Edit the SDKMAN! config

Usage: sdk config

Opens the SDKMAN! config ($SDKMAN_DIR/etc/config) in the editor named by the
EDITOR environment variable, or in vi if EDITOR is not set. Open a new shell
for your changes to take effect.

Options:
  -h, --help  Print help

Configuration:
  The SDKMAN! config contains settings such as these:

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
    sdkman_healthcheck_enable=true
    sdkman_insecure_ssl=false
    sdkman_native_enable=true
    sdkman_selfupdate_feature=true

Examples:
  sdk config
```

The settings come from the installer template, `sdkman-hooks/app/views/install_stable.scala.txt`. `sdkman_auto_update` is added later by `sdk selfupdate` (`selfupdate_stable.scala.txt`).

### `sdk version`

```
Show the SDKMAN! version

Usage: sdk version

Shows the versions of the core and of the native commands. The two are
released separately, so their versions differ.

Options:
  -h, --help  Print help

Aliases: v

Examples:
  sdk version
```

### `sdk help`

```
Show help for a command

Usage: sdk help [COMMAND]

Without a command, lists all commands. With a command, shows its full help.

Arguments:
  [COMMAND]  The command to show help for

Options:
  -h, --help  Print help

Examples:
  sdk help
  sdk help install
```

## Extra Considerations

- **Ordering in `sdk default`.** Today the success line prints before the symlink is created. The new success line must print only after the change has fully succeeded, including the copy fallback.
- **Alias list.** The current `help` binary does not accept `sdk help ug`, although its upgrade page lists the alias. The bash core also maps `l` to `list`. Help lists only the aliases in the catalogue above.
- **`sdk home` newline.** The old help claimed that `sdk home` prints no trailing newline, but the code does print one. Shell `$(…)` strips it, so the newline stays and the help no longer makes the claim.
- **`colored` in tests.** `src/bin/help.rs` tests call `colored::control::set_override`. These tests move to `tests/help.rs` (see Testing) and use `CLICOLOR_FORCE=1` instead.
- **`known_candidates` and `Box::leak`.** It currently leaks the file content to return `&'static str`. Changing it to return `Result<Vec<String>, CliError>` (R6) removes the leak as a side effect. Its two unit tests in `tests/helpers.rs` change accordingly: the `#[should_panic]` test becomes an assertion on the error.
- **Avoided words in file errors.** Errors about a path describe a file operation, not an SDKMAN! action, so `cannot remove <path>` is correct even though "remove" is avoided for the **Uninstall** concept. Do not use file-operation verbs to describe SDKMAN! actions.
- **Clippy and fmt.** Run `cargo fmt` and `cargo clippy` before every commit, as `CLAUDE.md` requires.
- **Windows.** `anstream` enables ANSI support on Windows consoles. The `✓` glyph needs a UTF-8 console. No fallback is planned, but note it for the release notes.

## Testing Considerations

**Framework:** the existing `assert_cmd` integration tests in `tests/` are the acceptance tests. Each runs a real binary against a virtual SDKMAN! directory from `tests/support`. Tests stay `#[serial]`.

- **Test-first, happy path first.** Each command's migration starts with its success path, followed by one commit per unhappy path, each with its test.
- **Plain text by default.** `assert_cmd` does not run in a terminal, so output is unstyled. Assertions match exact plain text on the right stream, for example `stderr(eq("✓ Uninstalled java 17.0.0-tem\n"))` and `stdout(is_empty())`. Tests should assert full lines, not `contains` fragments, because the wording is the feature.
- **Styling tests.** A small number of tests set `CLICOLOR_FORCE=1` and assert the ANSI sequences for one example of each message type and token. Another test sets `NO_COLOR=1` together with `CLICOLOR_FORCE=0`. Another writes `sdkman_colour_enable=false` to `etc/config`, which needs `VirtualEnv` in `tests/support` to support an optional config.
- **Unit tests only for permutations.** The colour decision (R4) is the pure function `colour_choice`, taking three environment variables and the config value. Test it in a table-driven unit test, including `CLICOLOR_FORCE=1` with `NO_COLOR=1` giving `Always`. Everything else is tested through binaries.
- **Help snapshots.** Replace the unit snapshot tests in `src/bin/help.rs` with `insta` snapshots in `tests/help.rs`, taken from running the binaries without colour, one per page in the Help Catalogue. Add one test per native command asserting that `sdk help <command>` and `<command> --help` print byte-identical stdout.
- **Exit codes.** `tests/version.rs` currently asserts exit code `101` for a missing or empty version file. These become exit code `1` with the catalogue's error text.
- **Stream moves.** The success-path tests in `tests/default.rs` and `tests/uninstall.rs` currently assert on stdout. They move to stderr with the new wording.

## Suggested Slice Breakdown

Each slice is one or more Conventional Commits. Each commit holds one test path and its implementation, passes `cargo test`, and is formatted and linted. The order follows the dependencies.

1. **`build: add anstream and anstyle as direct dependencies`.** No behaviour change.
2. **`feat(ui): decide styling per stream`.** `ui::init` with the R4 rules, the config reader and the table-driven unit test.
3. **`feat(ui): add message and data primitives`.** success, info, warning, hint, value, table, the tokens and `CliError` rendering. Tested through the first command migrated in slice 4.
4. **Migrate `sdk home`** (smallest command): R6 `run()` shape, shared not-installed error, unknown-SDK error. One commit per test path.
5. **Migrate `sdk uninstall`**: success, default guard, forced warning, unreadable default warning, delete failure.
6. **Migrate `sdk default`**: success after the change, copy-fallback warning, failures.
7. **Migrate `sdk current`**: bare value, table, no-default error, empty info.
8. **Migrate `sdk version`**: `core:` label, alignment, errors instead of panics.
9. **`feat: suggest the closest SDK for unknown names`** (R10).
10. **`feat(cli): share clap definitions and help styling`.** The `cli` module, `configure()`, `Print help`, the template and wrapping. Native commands switch to it, with `--help` snapshots.
11. **`feat(cli): restyle clap errors`** (R14). One commit per mapped `ErrorKind`. Include `sdk version extra`, the exception in Non-Goals.
12. **Rebuild the `help` binary on `cli`.** One commit per help page, each with its snapshot. Native pages first, then bash ones.
13. **`build: remove colored`** (and `textwrap` if unused). R15.
14. **`docs: amend STYLE.md and CLAUDE.md`.** The R2 interim note, a note that the `sdk` page overrides its usage line, and R16.

## Verification

After all slices are merged:

- [ ] `cargo test`, `cargo clippy -- -D warnings` and `cargo fmt --check` pass.
- [ ] `grep -rn "println!\|eprintln!\|print!" src/bin` returns nothing.
- [ ] `grep -rn "process::exit\|panic!\|\.expect(\|\.unwrap()" src` returns nothing outside `main` and tests.
- [ ] `grep -rni "candidate" src/bin src/lib.rs` finds only identifiers and literal paths, never output text.
- [ ] `colored` no longer appears in `Cargo.toml`.
- [ ] No test asserts exit code `101`.
- [ ] For every native command, `sdk help <command>` and `<command> --help` give identical stdout.
- [ ] Manual check in a real terminal: each message type and table renders as in STYLE.md, and is plain with `NO_COLOR=1`, with `sdkman_colour_enable=false`, and when piped (`sdk current | cat`, `sdk home java x 2>&1 | cat`).
- [ ] Manual check: `$(sdk home java <version>)` contains only the path, both with and without `CLICOLOR_FORCE=1`.
- [ ] Manual check through the bash core (`sdk uninstall …`, `sdk help`) with `sdkman_native_enable=true`.
- [ ] Every string in the Message Catalogue and Help Catalogue appears verbatim in a test or snapshot.
