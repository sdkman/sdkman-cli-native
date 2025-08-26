#!/usr/bin/env bash

project_root="$(dirname -- "$0")"
sdkman_dir="${SDKMAN_DIR:-$HOME/.sdkman}"

# TODO: use --artifact-dir when it stable
# https://github.com/rust-lang/cargo/issues/6790
cargo build --manifest-path "$project_root/Cargo.toml"
for file in target/debug/*; do
    if [ -f "$file" ] && [ -x "$file" ]; then
        cp "$file" "$sdkman_dir/libexec/"
    fi
done
