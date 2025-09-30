#!/usr/bin/env bash

project_root="$(dirname -- "$0")"
sdkman_dir="${SDKMAN_DIR:-$HOME/.sdkman}"
sdkman_libexec_dir="${sdkman_dir}/libexec"

# TODO: use --artifact-dir when it stable
# https://github.com/rust-lang/cargo/issues/6790
cargo build --manifest-path "$project_root/Cargo.toml"

# Create SDKMAN libexec directory if it does not exist.
if [ ! -d "${sdkman_libexec_dir}" ]; then
    mkdir -p  "${sdkman_libexec_dir}" || exit 1
fi

# Copy files.
for file in target/debug/*; do
    if [ -f "$file" ] && [ -x "$file" ]; then
        cp "$file" "${sdkman_libexec_dir}/"
    fi
done

# Report completion.
printf -- "All files installed in: %s\n"  "${sdkman_libexec_dir}"

# --
