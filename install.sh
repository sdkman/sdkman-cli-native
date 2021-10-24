#!/usr/bin/env bash

cargo build

if [[ ! -d "$SDKMAN_DIR" ]]; then
  export SDKMAN_DIR="$HOME/.sdkman"
  echo "SDKMAN_DIR environment variable not defined, set to $SDKMAN_DIR"
fi

find target/debug -maxdepth 1 -executable -type f -exec cp -v {} "$SDKMAN_DIR/libexec" \;
