#!/usr/bin/env sh

echo "---- setup rustc ----"
rustup update "${RUST_TOOLCHAIN:-stable}"
rustup default "${RUST_TOOLCHAIN:-stable}"
# only needed for cross-compile tests but we want consistent rust configuration
rustup target add aarch64-unknown-linux-gnu

# output full rustc version data
rustc --version --verbose
# this determines what our cargo version is, so don't ask
