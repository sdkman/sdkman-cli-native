# Output style guide

How SDKMAN! talks to users on the command line: responses, errors, warnings, hints and help pages. All commands follow this guide so they read in one voice.

Use only the terms defined in [CONTEXT.md](../CONTEXT.md). Use British spelling: colour, behaviour, licence. The reasons behind the less obvious rules are recorded in [docs/adr](adr/).

## Voice

Warm and plain. SDKMAN! is friendly through clarity and helpful hints, not through jokes.

- Speak to the user as "you".
- Do not use contractions. Write "is not" and "cannot", which are clearer for readers whose first language is not English.
- SDKMAN! rarely refers to itself. When it must, it uses its name, never "we" or "I".
- Use present or past tense and active voice: "Set java 21.0.2 (Temurin) as the default version", not "java 21.0.2 (Temurin) will be set as default".
- Lead with the outcome. For errors, say what is wrong, then give the exact command that fixes it in a hint.
- Never blame the user. Describe the state, not their mistake: "java 17.0.0 (Temurin) is not installed", not "you entered an invalid version".
- Keep it short. No filler such as "please", "successfully", "simply" or "just".
- No drama: no "panic", "oops", emoji or exclamation marks (except in the name SDKMAN!).

## Casing and punctuation

Labels follow the Rust and clap convention, because clap prints its own errors next to ours. Clap's errors are restyled to match: `hint:` instead of `tip:`, and the closing sentence "For more information, try '--help'." becomes `hint: run sdk <command> --help`.

- Labels are lowercase: `error:`, `warning:`, `hint:`.
- Text after a label starts lowercase and has no trailing full stop.
- Success and info lines use sentence case and have no trailing full stop.
- In help pages, the description, configuration and exit status text use full sentences with full stops. The other sections follow [Help pages](#help-pages).

```
✓ Installed java 21.0.2 (Temurin)

error: java 17.0.0 (Temurin) is not installed
  hint: run sdk install java 17.0.0-tem

warning: java 17.0.0 (Temurin) is not in your project config (.sdkmanrc)
```

## Streams

- **stdout** carries only data a script might capture: paths, versions and tables. It never contains success symbols, labels or hints.
- **stderr** carries everything conversational: success and info lines, warnings, errors, hints and prompts.

This keeps commands like `$(sdk home java 21.0.2-tem)` clean.

## Message types

Each message type has one fixed shape.

| Type | Shape | Example |
|---|---|---|
| Success | Green `✓`, then the command's verb in the past tense | `✓ Uninstalled java 17.0.0 (Temurin)` |
| Info | A plain sentence-case line with no marker | `Using java 21.0.2 (Temurin) in this shell` |
| Warning | Yellow `warning:` label | `warning: java 11.0.22 (Temurin) is not installed` |
| Error | Bold red `error:` label, then exit with a non-zero code | `error: unknown SDK jav` |
| Hint | Dim `hint:`, indented two spaces under the message it belongs to | `  hint: did you mean java?` |
| Prompt | Bold `?`, a short question, then `[Y/n]` with the default answer in capitals | `? Set java 21.0.2 (Temurin) as the default version? [Y/n]` |

Each command has one fixed success line:

| Command | Success line |
|---|---|
| `sdk install` | `✓ Installed java 21.0.2 (Temurin)` |
| `sdk uninstall` | `✓ Uninstalled java 17.0.0 (Temurin)` |
| `sdk default` | `✓ Set java 21.0.2 (Temurin) as the default version` |
| `sdk flush` | `✓ Flushed temporary files` |
| `sdk use` | `Using java 21.0.2 (Temurin) in this shell` (an info line, because nothing changes beyond this shell) |

- Use a warning when the command still does what was asked, and an error when it does not.
- A hint never stands alone. It always follows another message.
- A hint that continues onto a second line aligns with the text after `hint:`.
- An error that the user can fix always has a hint. If there are two ways to fix it, give one hint line for each, and never more than two:

  ```
  error: java 17.0.0 (Temurin) is the default version
    hint: run sdk default java <version> first
    hint: or run sdk uninstall --force java 17.0.0-tem
  ```
- After a forced or partial action, add a warning that states the resulting state: `warning: java has no default version now`.
- When a command succeeds by another route than usual, print the success line first, then a warning that says what happened instead. Never write "fall back".
- An empty result, such as no SDKs with a default version, is an info line, may carry a hint, and exits with code 0.
- An error message continues onto further lines with the same two-space indent as hints.
- When an SDK or version is unknown, the hint suggests the closest match ("did you mean java?"). If there is no close match, it points to the command that lists what is available, such as `sdk list`.
- Write `[Y/n]` when the default answer is yes and `[y/N]` when it is no. Prompts go to stderr. When stdin is not a terminal, or `sdkman_auto_answer=true`, SDKMAN! takes the default answer without asking.

## Exit codes

- `0`: the command did what was asked, including when it printed warnings or found nothing.
- `1`: an error from SDKMAN!.
- `2`: an invalid command line, as reported by clap.

## Unexpected failures

Failures the user did not cause, such as a missing metadata file or a disk error, are errors like any other. They never appear as a Rust panic.

```
error: cannot read ~/.sdkman/var/candidates: no such file or directory
  hint: run sdk update
```

Name the file or directory, then give the reason from the operating system in lowercase after a colon. Add a hint when a command can repair the problem. When none can, leave the hint out.

## Styling

- In messages, write an SDK, version and distribution as `java 21.0.2 (Temurin)`: the SDK and version in **bold** with no quotes, then the distribution's full name in plain text in brackets. SDKs without distributions leave the brackets out: **maven 3.9.6**.
- The short form `21.0.2-tem` appears only inside commands the user can type. It is a legacy form that goes away when the new native commands take version and distribution separately. Do not build new output around it.
- Commands the user can run are cyan, with no quotes or backticks: `hint: run sdk env install`, where `sdk env install` is cyan. The surrounding words ("run …", "with …") mark them out when colour is off. A command is cyan as a whole, including any SDK name or version inside it. Placeholders such as `<version>` inside a command stay plain.
- Italic and underline never carry meaning. Do not use underline at all, because it looks like a link.
- Paths and URLs are plain text. Do not emit terminal hyperlinks for now.
- Literal paths and file names are quoted exactly, even when they contain avoided words: `~/.sdkman/candidates/java`. Write the SDKMAN! directory as `$SDKMAN_DIR` or `~/.sdkman`, never `${SDKMAN_DIR}`.

## Data output

- Label and value pairs and table columns are aligned, so values start in one column.
- A table starts with a bold title line in sentence case, followed by its rows indented two spaces. There are no column headers.

  ```
  Default versions
    java    21.0.2 (Temurin)
    maven   3.9.6
    gradle  8.7
  ```
- `sdk version` keeps its own layout, with blank lines around it. The bold yellow `SDKMAN!` brand is the only yellow that does not mean a warning.

```

SDKMAN!
core:   5.19.0
native: 0.7.35 (linux x86_64)

```

## When to use colour

Colour and other styling are decided for each stream separately, in this order:

1. `CLICOLOR_FORCE` set to a non-zero value turns styling on.
2. `NO_COLOR` set to any non-empty value, `CLICOLOR=0`, or `sdkman_colour_enable=false` in the SDKMAN! config, turns styling off.
3. Otherwise, a stream is styled only when it is connected to a terminal.

Tables and lists on stdout follow the same rules, so they get bold headings on a terminal and become plain text when piped. A single value meant for scripts, such as the path from `sdk home`, is never styled, not even with `CLICOLOR_FORCE`.

Every message must still make sense with styling off: symbols, labels and wording carry the meaning, and colour only reinforces it.

## Help pages

Help uses clap's own layout, so help pages and clap's error messages look the same. Each command's help is defined once, in its clap definition, and both `sdk help <command>` and `sdk <command> --help` show it. Commands still written in bash get a clap definition in the `help` binary that is used only for help. See [ADR 0005](adr/0005-clap-style-help.md).

Sections appear in this order. Optional sections appear only when they apply.

| Section | Required | Content |
|---|---|---|
| Tagline | yes | One line describing the command, starting with a verb: `Uninstall a version of an SDK`. No full stop. Never write "sdk subcommand to". |
| `Usage:` | yes | Generated by clap from the command's definition |
| Description | no | Short paragraphs of full sentences in the voice above, shown with `--help` only |
| `Commands:` | no | Fixed words the command takes, such as `init`, `install` and `clear` for `sdk env` |
| `Arguments:` | no | Generated by clap, one line for each argument |
| `Options:` | no | Generated by clap, one line for each option, short form first: `-f, --force` |
| `Aliases:` | no | Short forms of the command, such as `rm` |
| `Configuration:` | no | Related SDKMAN! config keys or files, shown with `--help` only |
| `Exit status:` | no | When the command exits with a non-zero code, shown with `--help` only |
| `Examples:` | yes | One command per line, with no `$` prompt |

- `-h` shows the short form: tagline, usage, commands, arguments, options and aliases. `--help` and `sdk help <command>` show everything.
- Section headings are bold, in sentence case, and end with a colon. They are not underlined, which overrides clap's default style.
- Usage and example lines are cyan, like commands in hints. Placeholders such as `<VERSION>` stay plain. Commands and options mentioned in prose, such as `--force`, are cyan too.
- The built-in `-h, --help` option always reads `Print help`. Override clap's long-help wording, "Print help (see a summary with '-h')".
- Argument, option and command descriptions are capitalised fragments with no full stop, matching clap's built-in `Print help`: `Uninstall even if it is the default version`.
- SDK names in help prose are not bold. Help is reference text, not a result.
- Embedded files, such as a sample project config, are indented two more spaces than the text around them and left unstyled, with no `---` fences.
- Help wraps to the terminal width, up to a maximum of 80 columns.
- Describe what the command does, not what it "will" do: "Uninstalls a version of an SDK", not "This subcommand will remove…".
- Use the terms in CONTEXT.md. Write "argument" and "option", never "qualifier" or "parameter".

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
  Exits with a non-zero code if the SDK or version is not installed.

Examples:
  sdk uninstall java 17.0.0-tem
  sdk rm --force java 17.0.0-tem
```
