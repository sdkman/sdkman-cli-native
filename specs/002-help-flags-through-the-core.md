# Help Flags Through the Core Specification

## Overview

[`output-style-alignment.md`](output-style-alignment.md) gave every command a clap definition and made `<command> --help` and `sdk help <command>` print the same page. That holds for the five native commands, but only when the native binary is the one that sees the flag. Run through the bash core (`sdkman-cli`), help flags are broken everywhere else:

| Typed | Today |
|---|---|
| `sdk update --help` | runs a real update |
| `sdk flush --help`, `sdk flush -h` | flushes `tmp` and `metadata` |
| `sdk config --help` | opens the editor |
| `sdk list --help` | asks the server for an SDK called `--help` and prints its error page |
| `sdk install -h` (and every other bash command) | prints `grep`'s usage |
| `sdk -h`, `sdk --help` | `Invalid command: -h`, then the main page, exit `0` |
| `sdk help -h`, `sdk help --help` | `error: unknown command -h`, exit `2` |

The fix is split across the two repositories. The bash core sends every help flag it does not hand to a native command to the `help` binary (a small bug fix in `sdkman-cli`, outside this spec). This spec makes the `help` binary able to answer those calls, so that, with native commands enabled, **all help is rendered by native code**, `-h` always gives the short page and `--help` always gives the full page.

It also fixes one defect in the pages themselves: the `flush` page describes what bash `flush` does not do. The full form keeps clap's two-line layout of arguments and options (R4).

**Key Properties:**
- The `help` binary's command line is a contract with the bash core, stated in [The `help` contract](#the-help-contract).
- For every command in the Help Catalogue, `-h` prints the short page and `--help` prints the full page, whichever binary prints it.
- With native commands enabled, `sdk -h`, `sdk --help`, `sdk help -h` and `sdk help --help` all print the main `sdk` page.
- With `sdkman_native_enable=false`, no native code runs, including `help`. Every help request gets the bash help.
- This spec overrides `output-style-alignment.md` where they differ.

## Context

The bash core dispatches with `"$native_command" "${@:2}"` when a native binary exists for the command and `sdkman_native_enable=true`. Otherwise it runs the bash function, which does not know about help flags.

After the companion fix, with `sdkman_native_enable=true`, the core does this for help:

| Typed | The core runs |
|---|---|
| `sdk` | `help` (unchanged) |
| `sdk -h` / `sdk --help` | `help -h` / `help --help` |
| `sdk help [args…]` | `help [args…]` (unchanged) |
| `sdk <cmd> -h` / `--help`, `<cmd>` not dispatched to a native binary | `help <cmd> -h` / `help <cmd> --help` |
| `sdk <cmd> -h` / `--help`, `<cmd>` dispatched to a native binary | `<cmd> -h` / `<cmd> --help` (unchanged) |
| `sdk <unknown>` | `help <unknown>` |

The core resolves its own aliases (`l`, `ls`, `rm` and so on) before any of this, so `<cmd>` is the full command name. It uses the `help` binary only when `sdkman_native_enable=true` and `$SDKMAN_DIR/libexec/help` exists. Otherwise every help request, including `-h` and `--help`, gets the bash help, and no native code runs.

Because `sdk -h` and `sdk help -h` both arrive as `help -h`, they must print the same page. The main page was chosen. The page for the `help` command itself stays available as `sdk help help`.

## Non-Goals

**Out of scope for this spec:**
- **The bash core change.** It lands in `sdkman-cli` as a bug fix with its own Cucumber scenarios.
- **Help wording**, other than the `flush` description in R5.
- **Colour in the bash core.** It still ignores `NO_COLOR` and `CLICOLOR`.
- **Native command behaviour.** `current`, `default`, `home`, `uninstall` and `version` already send `-h` and `--help` to the right form, and their pages keep clap's layout (R4).

## Requirements

### The `help` contract

- **R1. Command line.** The `help` binary accepts `help [COMMAND] [-h | --help]`. The flag may come before or after the command name. Every case exits `0` and prints to stdout, except an unknown command:

  | Call | Prints |
  |---|---|
  | `help` | the main page, full form |
  | `help --help` | the main page, full form |
  | `help -h` | the main page, short form |
  | `help <command>` | that command's page, full form |
  | `help <command> --help` | that command's page, full form |
  | `help <command> -h` | that command's page, short form |
  | `help <unknown>`, with or without a flag | the existing `InvalidSubcommand` error on stderr, exit `2` |

  `<command>` resolves aliases, visible and hidden, as it does today. `-h` and `--help` are flags wherever they appear. Every other argument after the command name is ignored, as it is today, so `help env init -h` prints the short `env` page. If both `-h` and `--help` are given, the last one wins.
- **R2. Short main page.** The short form of the main page is tagline, `Usage:`, `Commands:` and `Options:`. Like the full form, it lists every alias as `[aliases: X]`.
- **R3. Equivalence.** For every native command, `<command> -h` and `help <command> -h` print byte-identical stdout, and `<command> --help`, `help <command> --help` and `help <command>` print byte-identical stdout.

### Help pages

- **R4. clap's layout stands.** The short form lists each argument and option on one row. The full form keeps clap's long layout: the name on one line, its description indented on the next, with a blank line between rows. clap cannot render the full form any other way, and ADR 0005 keeps clap's layout. This overrides the one-row full pages in the Help Catalogue of `output-style-alignment.md`.
- **R5. `flush` description.** The `flush` page's description reads:

  ```
  Discards temporary files and cached data that SDKMAN! keeps for itself.
  Without a command, flushes tmp and metadata.
  ```

  The bash `flush` with no command does not flush `version`, so the page no longer says it flushes everything.
- **R6. Hidden alias `l`.** `list` gets `l` as a hidden alias, matching the bash core. `help l` prints the `list` page. `l` appears on no page.

## Rules

**Before implementing, read and internalise:**
- [`output-style-alignment.md`](output-style-alignment.md): the Help Catalogue, R12 to R14 and the clap integration design.
- [`docs/STYLE.md`](../docs/STYLE.md) and [`CONTEXT.md`](../CONTEXT.md) for any text touched.
- The project `CLAUDE.md`: verification gate, snapshot rules and boundaries.

## Extra Considerations

- **`-h` on the main page and on `help`.** clap shows the short form only when the page has something to drop. The main page drops its description and examples, so the short form differs from the full form. Assert both.
- **Snapshots change on purpose.** R5 changes the `flush` page. Accept its new snapshot with `cargo insta accept` in the same commit. No other existing page changes.
- **`[aliases: X]` rewrite.** The main page's plural-alias rewrite must apply to the short form too, or `-h` and `--help` disagree on the `Commands:` rows.
- **Unknown flags.** `help --foo` is still reported as an unknown command `--foo`. Only `-h` and `--help` are flags.

## Testing Considerations

**Framework:** the `insta` snapshots and binary runs in `tests/help.rs`. Tests run the binaries with `std::process::Command` and set colour variables per process, as the file does today, so they need no `#[serial]`. They assert plain text on the right stream.

- **R1, one test per row** of the contract table, asserting stdout, stderr and the exit code. stderr is empty for every row except the unknown-command row, which asserts an empty stdout and the exact error and hint on stderr. The unknown-command row is tested with and without `-h`.
- **R1, flag position:** `help -h install` and `help install -h` print the same page.
- **R2:** an `insta` snapshot of `help -h`. A test that `help`, `help --help` and the existing main-page snapshot agree.
- **R3:** for each native command, one test for `-h` and one for `--help`, comparing the native binary with the `help` binary byte for byte. The existing `--help` comparisons stay.
- **Short form for bash commands:** one `insta` snapshot of `help <command> -h` for each command still in bash (`install`, `list`, `use`, `env`, `upgrade`, `update`, `selfupdate`, `flush`, `config`) and for `help`.
- **R4:** the existing full-form snapshots stay unchanged.
- **R6:** `help l` prints the same stdout as `help list`. The main-page snapshot does not change because of it.

## Suggested Slice Breakdown

Each slice is one or more Conventional Commits. Each commit holds one test path and its implementation, and passes the verification gate in `CLAUDE.md`.

1. **`feat(help): accept -h and --help`** (R1, R2). One commit per contract row, short main page first.
2. **`test(help): assert -h matches the native commands`** (R3).
3. **`feat(help): short pages for bash commands`.** The snapshots from Testing Considerations.
4. **`fix(help): describe what flush does without a command`** (R5).
5. **`feat(cli): accept l as a hidden alias of list`** (R6).

## Verification

After all slices are merged:

- [ ] `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` pass.
- [ ] Every row of the R1 table has a test.
- [ ] For every native command, `-h` and `--help` match the `help` binary byte for byte (R3).
- [ ] No `.snap.new` file remains.
