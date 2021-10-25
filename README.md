# SDKMAN! Native CLI components

Native CLI subcommand components for SDKMAN! written in [Rust](https://www.rust-lang.org/). Use the binaries generated
by this project in the `sdk` wrapper shell function from the [sdkman-cli](https://github.com/sdkman/sdkman-cli) project.

Initial setup:

1. [Install the Rust toolchain](https://www.rust-lang.org/tools/install) with `rustup`.
2. Build the project with Cargo using `cargo build`
3. Install the native binaries into the `libexec` folder of your local SDKMAN installation with `./install.sh`
4. Try it out!

```bash
$ sdk help
$ sdk help install
$ sdk help uninstall
$ sdk help list
$ sdk help use
```

This is an initial installation process which will be automated sometime in the future.