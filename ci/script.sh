#!/usr/bin/env bash
set -euxo pipefail

main() {
    cargo check --target $TARGET

    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        cargo test --target $TARGET
        cargo test --target $TARGET --release
    fi
}

main