# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is sdkman-cli-native, a Rust project containing native CLI subcommands for SDKMAN!. The project builds multiple binary executables that replace bash functions in the main SDKMAN! shell wrapper for performance optimization.

## Architecture

### Binary Structure
The project is a single Cargo package (not a workspace) that builds one binary per file in `src/bin/`:
- `src/lib.rs` - Contains shared constants and helper functions used across all binaries
- `src/bin/*.rs` - Individual subcommand implementations:
  - `current` - Shows current versions of SDK candidates
  - `default` - Manages default SDK versions
  - `help` - Provides contextual help for all subcommands
  - `home` - Shows SDK installation directories
  - `uninstall` - Removes SDK installations
  - `version` - Shows SDKMAN CLI and native component versions

### Core Components
- `helpers` module - Provides shared utilities for SDKMAN directory inference, candidate validation, and file operations
- `constants` module - Defines SDKMAN directory structure constants
- All binaries use `clap` for command-line argument parsing with consistent patterns

### SDKMAN Integration
The binaries are designed to be installed into `$SDKMAN_DIR/libexec/` and called by the main `sdk` shell wrapper function. They expect the standard SDKMAN directory structure:
```
$SDKMAN_DIR/
├── candidates/           # SDK installations
├── var/                 # Metadata files
│   ├── candidates       # List of available candidates
│   └── version         # CLI version
└── libexec/            # Native binaries location
```

## Development Commands

### Building
```bash
cargo build                    # Build all binaries in debug mode
cargo build --release         # Build optimized release binaries
```

### Testing
```bash
cargo test                     # Run all unit and integration tests
cargo test --test current     # Run specific integration test file
```

Test layout:
- `tests/*.rs` - Integration tests, one file per binary, plus `tests/helpers.rs` for `src/lib.rs`
- `tests/support/` - Custom test harness that creates virtual SDKMAN environments
- `src/lib.rs` - Unit tests for the helpers in a `#[cfg(test)]` module
- `tests/help.rs` - `insta` snapshot tests for help text formatting, with snapshots in `tests/snapshots/`

The tests use:
- `assert_cmd` and `predicates` for CLI testing
- `serial_test` for tests requiring sequential execution
- `insta` for snapshot testing (`cargo insta review` to accept changes)

### Installing for Local Development
```bash
./install.sh              # Build and install binaries to $SDKMAN_DIR/libexec/
```

### Code Quality
```bash
cargo fmt                  # Format code
cargo clippy               # Lint code
```

## Testing Patterns

Tests use a virtual environment pattern where temporary SDKMAN directories are created with:
- Mock candidate installations
- Symlinked "current" versions
- Realistic directory structures

Tests are marked `#[serial]` because they manipulate global environment variables and must run sequentially.

## Verification Gate

Run these commands in this order. Work is done only when every command exits with status 0:
```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Snapshot rules:
- When a change alters help text on purpose, accept the new snapshots with `cargo insta accept` and commit the `.snap` files with the change.
- In every other case, a `.snap.new` file is a failure. Fix the code, not the snapshot.
- Never run `cargo insta review`. It is interactive.

## Boundaries

- Never run `./install.sh`. It overwrites the binaries in the real `$SDKMAN_DIR/libexec/`.
- Never create, change or delete files outside this repository.
- Never push, switch branches, rebase or rewrite Git history.
- Never start Docker containers. This project does not need a database.
- Never edit `~/.sdkman/etc/config` or any other SDKMAN configuration.

## Release Process

The project uses JReleaser for automated releases across multiple platforms:
- Linux: x86_64, i686, aarch64
- macOS: x86_64, aarch64
- Windows: x86_64

Release configuration is in `jreleaser.yml` with conventional commits changelog generation. After a release, `bin/release-binary.sh` records the new version as the beta or stable native CLI version in the SDKMAN! MongoDB.

## Documentation

- `CONTEXT.md` - Glossary of user-facing terms
- `docs/STYLE.md` - Output style guide
- `docs/adr/` - Architecture decision records
- `specs/` - Feature specifications

## User-Facing Text

Every message, prompt and help page must follow `docs/STYLE.md` and use only the terms in `CONTEXT.md`. Check new or changed text against both when writing or reviewing it. The key rules are:
- Say "SDK", never "candidate", in output. Code keeps `candidate` (ADR 0001).
- Warm and plain voice: speak to "you", use active voice, no contractions, no filler, no exclamation marks (except in the name SDKMAN!).
- Success messages reuse the command's verb: `✓ Uninstalled java 17.0.0 (Temurin)`.
- Labels follow Rust and clap: `error:`, `warning:` and `hint:` are lowercase, and the text after them starts lowercase with no trailing full stop.
- stdout is for data only. Everything conversational goes to stderr (ADR 0003).
