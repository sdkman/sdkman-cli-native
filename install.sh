#!/usr/bin/env bash

# To build first
cargo build

# Checking environment variables
if [[ ! -d "$SDKMAN_DIR" ]]; then
  export SDKMAN_DIR="$HOME/.sdkman"
  echo "SDKMAN_DIR environment variable not defined, set to $SDKMAN_DIR"
fi

# Check the directory
if [[ ! -d "$SDKMAN_DIR/libexec" ]]; then
  mkdir -p "$SDKMAN_DIR/libexec"
  echo "Libexec directory does not exist, created to $SDKMAN_DIR/libexec"
fi

find target/debug -maxdepth 1 ! -name ".cargo-lock" ! -name "*.d" -executable -type f -exec cp -v {} "$SDKMAN_DIR/libexec/" \;