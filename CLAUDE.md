# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is sdkman-cli-native, a Rust project containing native CLI subcommands for SDKMAN!. The project builds multiple binary executables that replace bash functions in the main SDKMAN! shell wrapper for performance optimization.

## Architecture

### Binary Structure
The project follows a multi-binary Cargo workspace pattern:
- `src/lib.rs` - Contains shared constants and helper functions used across all binaries
- `src/bin/*/main.rs` - Individual subcommand implementations:
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

The project uses extensive integration testing with:
- `assert_cmd` for CLI testing
- `serial_test` for tests requiring sequential execution
- `insta` for snapshot testing (help text formatting)
- Custom test harness in `tests/support/` that creates virtual SDKMAN environments

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

## Release Process

The project uses JReleaser for automated releases across multiple platforms:
- Linux: x86_64, i686, aarch64
- macOS: x86_64, aarch64  
- Windows: x86_64

Release configuration is in `jreleaser.yml` with conventional commits changelog generation.