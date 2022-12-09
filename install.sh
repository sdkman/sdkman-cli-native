#!/usr/bin/env bash
set -e

cargo build

if [[ ! -d "$SDKMAN_DIR" ]]; then
  export SDKMAN_DIR="$HOME/.sdkman"
  echo "SDKMAN_DIR environment variable not defined, set to $SDKMAN_DIR"
fi
mkdir -p "$SDKMAN_DIR/libexec/"
find target/debug -maxdepth 1 -executable -type f -exec cp -v {} "$SDKMAN_DIR/libexec/" \;
