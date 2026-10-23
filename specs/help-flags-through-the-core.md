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

The fix is split across the two repositories. The bash core sends every help flag it does not hand to a native command to the `help` binary (a small bug fix in `sdkman-cli`, outside this spec). This spec makes the `help` binary able to answer those calls, so that **all help is rendered by native code**, `-h` always gives the short page and `--help` always gives the full page.

It also fixes two defects in the pages themselves: the full form lays out arguments and options on two lines each, against R13 of the earlier spec, and the `flush` page describes what bash `flush` does not do.

**Key Properties:**
- The `help` binary's command line is a contract with the bash core, stated in [The `help` contract](#the-help-contract).
- For every command in the Help Catalogue, `-h` prints the short page and `--help` prints the full page, whichever binary prints it.
- `sdk -h`, `sdk --help`, `sdk help -h` and `sdk help --help` all print the main `sdk` page.
- This spec overrides `output-style-alignment.md` where they differ.

## Context

The bash core dispatches with `"$native_command" "${@:2}"` when a native binary exists for the command and `sdkman_native_enable=true`. Otherwise it runs the bash function, which does not know about help flags.

After the companion fix, the core does this for help:

| Typed | The core runs |
|---|---|
| `sdk` | `help` (unchanged) |
| `sdk -h` / `sdk --help` | `help -h` / `help --help` |
| `sdk help [args…]` | `help [args…]` (unchanged) |
| `sdk <cmd> -h` / `--help`, `<cmd>` not dispatched to a native binary | `help <cmd> -h` / `help <cmd> --help` |
| `sdk <cmd> -h` / `--help`, `<cmd>` dispatched to a native binary | `<cmd> -h` / `<cmd> --help` (unchanged) |
| `sdk <unknown>` | `help <unknown>` |

The core resolves its own aliases (`l`, `ls`, `rm` and so on) before any of this, so `<cmd>` is the full command name. It uses the `help` binary whenever `$SDKMAN_DIR/libexec/help` exists, and its own bash help only when it does not.

Because `sdk -h` and `sdk help -h` both arrive as `help -h`, they must print the same page. The main page was chosen. The page for the `help` command itself stays available as `sdk help help`.

## Non-Goals

**Out of scope for this spec:**
- **The bash core change.** It lands in `sdkman-cli` as a bug fix with its own Cucumber scenarios.
- **Help wording**, other than the `flush` description in R5.
- **Colour in the bash core.** It still ignores `NO_COLOR` and `CLICOLOR`.
- **Native command behaviour.** `current`, `default`, `home`, `uninstall` and `version` already answer `-h` and `--help` correctly and keep doing so.

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

  `<command>` resolves aliases, visible and hidden, as it does today. Further arguments after the command name are ignored, as they are today. If both `-h` and `--help` are given, the last one wins.
- **R2. Short main page.** The short form of the main page is tagline, `Usage:`, `Commands:` and `Options:`. Like the full form, it lists every alias as `[aliases: X]`.
- **R3. Equivalence.** For every native command, `<command> -h` and `help <command> -h` print byte-identical stdout, and `<command> --help`, `help <command> --help` and `help <command>` print byte-identical stdout.

### Help pages

- **R4. One row per argument and option.** In both forms, each argument and option is one row: the name, then its description aligned in a column, as the Help Catalogue in `output-style-alignment.md` shows (`-h, --help  Print help`). A description too long for the line wraps under its own column. No blank lines separate the rows.
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
- **Snapshots change on purpose.** R4 changes every full-form page. Accept the new snapshots with `cargo insta accept` in the same commit as the layout fix. The R5 page change is a second, separate snapshot change.
- **`[aliases: X]` rewrite.** The main page's plural-alias rewrite must apply to the short form too, or `-h` and `--help` disagree on the `Commands:` rows.
- **Unknown flags.** `help --foo` is still reported as an unknown command `--foo`. Only `-h` and `--help` are flags.

## Testing Considerations

**Framework:** the `assert_cmd` integration tests and `insta` snapshots in `tests/help.rs`. Tests stay `#[serial]` and assert plain text on the right stream.

- **R1, one test per row** of the contract table, asserting stdout, an empty stderr and the exit code. The unknown-command row is tested with and without `-h`.
- **R1, flag position:** `help -h install` and `help install -h` print the same page.
- **R2:** an `insta` snapshot of `help -h`. A test that `help`, `help --help` and the existing main-page snapshot agree.
- **R3:** for each native command, one test for `-h` and one for `--help`, comparing the native binary with the `help` binary byte for byte. The existing `--help` comparisons stay.
- **Short form for bash commands:** one `insta` snapshot of `help <command> -h` for each command still in bash (`install`, `list`, `use`, `env`, `upgrade`, `update`, `selfupdate`, `flush`, `config`) and for `help`.
- **R4:** covered by the re-accepted full-form snapshots. Add one assertion that the uninstall page contains the line `  -h, --help   Print help`, so the layout cannot drift back unnoticed by a blanket accept.
- **R6:** `help l` prints the same stdout as `help list`. The main-page snapshot does not change because of it.

## Suggested Slice Breakdown

Each slice is one or more Conventional Commits. Each commit holds one test path and its implementation, and passes the verification gate in `CLAUDE.md`.

1. **`fix(help): lay out arguments and options on one row`** (R4). Re-accept the full-form snapshots.
2. **`feat(help): accept -h and --help`** (R1, R2). One commit per contract row, short main page first.
3. **`test(help): assert -h matches the native commands`** (R3).
4. **`feat(help): short pages for bash commands`.** The snapshots from Testing Considerations.
5. **`fix(help): describe what flush does without a command`** (R5).
6. **`feat(cli): accept l as a hidden alias of list`** (R6).

## Verification

After all slices are merged:

- [ ] `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` pass.
- [ ] Every row of the R1 table has a test.
- [ ] No full-form snapshot has an argument or option description on its own line.
- [ ] For every native command, `-h` and `--help` match the `help` binary byte for byte (R3).
- [ ] No `.snap.new` file remains.
