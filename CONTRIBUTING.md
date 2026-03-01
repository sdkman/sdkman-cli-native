# Contributing to SDKMAN! Native CLI

Thank you for your interest in contributing to SDKMAN! Native CLI. We welcome contributions that help migrate SDKMAN commands from Bash to Rust using our incremental strangler pattern approach.

## Before You Start

**This is not a standard single-binary Rust project.** We build multiple independent binaries, each replacing one SDKMAN Bash function. Please read [Discussion #18](https://github.com/sdkman/sdkman-cli-native/discussions/18) to understand our architectural approach.

**Discuss significant changes first:**
- Open a discussion or issue before starting major work
- Propose new command migrations before implementing
- Seek feedback early to ensure alignment with project direction

## Getting Started

### Development Setup

See the "Local development setup" section in [README.md](README.md) for installation and build instructions.

### Project Structure

```
src/
├── lib.rs              # Shared constants and utilities
├── helpers/            # Common helper functions
└── bin/
    ├── command1/       # Each directory is a separate binary
    ├── command2/       # that implements one sdk subcommand
    └── ...
```

Each binary in `src/bin/*/main.rs` is independent and uses shared code from `lib.rs` and `helpers/`.

## Development Workflow

### Making Changes

1. **Fork and clone** the repository
2. **Create a branch** for your changes
3. **Make focused changes** - one feature/fix per PR
4. **Write/update tests** in the `tests/` directory
5. **Run tests**: `cargo test`
6. **Format code**: `cargo fmt`
7. **Check for issues**: `cargo clippy`
8. **Commit** using [Conventional Commits](https://www.conventionalcommits.org/) format

### Testing

The project uses both unit tests and integration tests:

- **Unit tests**: Located within command files (e.g., `src/bin/help/main.rs`) for testing individual functions
- **Integration tests**: Located in `tests/` directory for end-to-end command testing
- **Snapshot tests**: Help text is validated using `insta` snapshots in the help command

The `tests/support/` module provides helper functions to create virtual SDKMAN environments for integration tests.

```bash
cargo test                    # Run all tests
cargo test --test current    # Run specific integration test file
cargo insta review           # Review snapshot changes (help text)
```

Integration tests are marked `#[serial]` because they manipulate environment variables.

### Adding a New Command

To migrate a Bash command to a native binary:

1. **Open a discussion** proposing the migration
2. **Research** the existing Bash implementation in [sdkman-cli](https://github.com/sdkman/sdkman-cli)
3. **Create** `src/bin/newcommand/main.rs` following existing patterns
4. **Use** `clap` for argument parsing, import helpers from `crate::helpers`
5. **Write** integration tests in `tests/newcommand.rs`
6. **Test manually** by running `./install.sh` to inject the native binaries into your local SDKMAN installation

## Pull Request Guidelines

### What Makes a Good PR

✅ **Do:**
- Keep PRs small and focused (one command/fix/feature)
- Write clear commit messages using conventional commit format
- Add tests for new functionality
- Update help text snapshots if needed
- Respond to feedback constructively

❌ **Don't:**
- Submit large "change the world" PRs without prior discussion
- Mix unrelated changes in a single PR
- Ignore project conventions and patterns
- Skip tests or code formatting

### PR Process

1. Ensure all tests pass and code is formatted
2. Push to your fork and open a PR against `master`
3. Fill out the PR description explaining what and why
4. Wait for maintainer review
5. Address feedback and iterate as needed

## Code Style

- **Formatting**: Run `cargo fmt` before committing
- **Linting**: Address `cargo clippy` warnings
- **Naming**: Follow Rust conventions (`snake_case` for functions, `PascalCase` for types)
- **Error handling**: Use appropriate exit codes from the `exitcode` crate
- **Documentation**: Comment non-obvious logic, keep help text clear

## Communication

- **Discord**: Join our [Discord server](https://discord.gg/y9mVJYVyu4) - we value discussion here before anything else!
- **Discussions**: Open a [GitHub Discussion](https://github.com/sdkman/sdkman-cli-native/discussions) for proposals and questions
- **Bugs**: Report via [GitHub Issues](https://github.com/sdkman/sdkman-cli-native/issues)

## Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md). We are committed to providing a welcoming and inclusive environment for all contributors.

## Developer Certificate of Origin

By contributing to this project, you agree to the terms of the [Developer Certificate of Origin](dco.txt). This certifies that you have the right to submit your contribution under the project's open source license.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for helping make SDKMAN better! 🙏
