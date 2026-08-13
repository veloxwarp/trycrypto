#!/usr/bin/env bash
set -euo pipefail

RUST_TOOLCHAIN="1.97.1"
TRUNK_VERSION="0.21.14"

export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --profile minimal --default-toolchain none
fi

rustup toolchain install "$RUST_TOOLCHAIN" --profile minimal
rustup target add --toolchain "$RUST_TOOLCHAIN" wasm32-unknown-unknown
export RUSTUP_TOOLCHAIN="$RUST_TOOLCHAIN"

if ! command -v trunk >/dev/null 2>&1 \
  || [[ "$(trunk --version)" != "trunk $TRUNK_VERSION" ]]; then
  cargo install trunk --version "$TRUNK_VERSION" --locked
fi

trunk build --release
