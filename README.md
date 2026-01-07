# SDKMAN! Native CLI components

[![release](https://github.com/sdkman/sdkman-cli-native/actions/workflows/release.yml/badge.svg)](https://github.com/sdkman/sdkman-cli-native/actions/workflows/release.yml)

Native CLI subcommand components for SDKMAN! written in [Rust](https://www.rust-lang.org/). Use the binaries generated
by this project in the `sdk` wrapper shell function from the [sdkman-cli](https://github.com/sdkman/sdkman-cli) project.

## Getting this on your machine

Nothing needs to be done if you are using one of the following supported architectures:

* aarch64-unknown-linux-gnu
* x86_64-unknown-linux-gnu
* i686-unknown-linux-gnu
* aarch64-apple-darwin
* x86_64-apple-darwin
* x86_64-pc-windows-msvc

The default installer will automatically detect your architecture and install the latest release version on your machine.

## Local development setup:

1. [Install the Rust toolchain](https://www.rust-lang.org/tools/install) with `rustup`.
2. Build the project with Cargo using `cargo build`
3. Install the native binaries into the `libexec` folder of your local SDKMAN installation with `./install.sh`
4. Try it out!

```bash
$ sdk help
$ sdk help [subcommand]
```
## Hosting

We're proud to host our backend services on DigitalOcean as a sponsored partner.

[![DigitalOcean Referral Badge](https://web-platforms.sfo2.cdn.digitaloceanspaces.com/WWW/Badge%203.svg)](https://www.digitalocean.com/?refcode=d99e5747251d&utm_campaign=Referral_Invite&utm_medium=Referral_Program&utm_source=badge)
