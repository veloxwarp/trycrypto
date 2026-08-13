#!/usr/bin/env bash
set -euo pipefail

RUST_TOOLCHAIN="1.97.1"
TRUNK_VERSION="0.21.14"
TRUNK_TARGET="x86_64-unknown-linux-gnu"

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
  if [[ "$(uname -m)" != "x86_64" ]] || [[ "$(uname -s)" != "Linux" ]]; then
    echo "Unsupported platform for prebuilt Trunk: $(uname -s) $(uname -m)" >&2
    exit 1
  fi

  tmpdir="$(mktemp -d)"
  trap 'rm -rf "$tmpdir"' EXIT
  asset="trunk-${TRUNK_TARGET}.tar.gz"
  base_url="https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}"

  curl --proto '=https' --tlsv1.2 -fsSL "$base_url/$asset" -o "$tmpdir/$asset"
  curl --proto '=https' --tlsv1.2 -fsSL "$base_url/$asset.sha256" -o "$tmpdir/$asset.sha256"

  expected_sha256="$(tr -d '[:space:]' < "$tmpdir/$asset.sha256")"
  actual_sha256="$(sha256sum "$tmpdir/$asset" | awk '{print $1}')"
  if [[ "$actual_sha256" != "$expected_sha256" ]]; then
    echo "Trunk checksum mismatch: expected $expected_sha256, got $actual_sha256" >&2
    exit 1
  fi

  tar -xzf "$tmpdir/$asset" -C "$tmpdir"
  install -m 0755 "$tmpdir/trunk" "$HOME/.cargo/bin/trunk"
  trap - EXIT
  rm -rf "$tmpdir"
fi

trunk build --release

if [[ -d assets ]]; then
  rm -rf dist/assets
  cp -R assets dist/assets
fi
