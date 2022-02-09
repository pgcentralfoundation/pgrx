#! /usr/bin/env bash

# requires: https://github.com/sunng87/cargo-release

if [ "x$1" == "x" ]; then
    echo "usage:  ./update-verions.sh <VERSION>"
    exit 1
fi

set -ex

HEAD=$(git rev-parse HEAD)
VERSION=$1

CARGO_TOMLS_TO_SED=(
    ./cargo-pgx/src/templates/cargo_toml
    ./nix/templates/default/Cargo.toml
    ./pgx/Cargo.toml
    ./pgx-utils/Cargo.toml
    ./pgx-macros/Cargo.toml
    ./pgx-tests/Cargo.toml
    ./cargo-pgx/Cargo.toml
    ./pgx-pg-sys/Cargo.toml
    ./pgx-examples/*/Cargo.toml
)

DEPENDENCIES_TO_UPDATE=(
    "pgx"
    "pgx-tests"
    "pgx-macros"
    "pgx-pgx-sys"
    "pgx-utils"
)

for cargo_toml in ${CARGO_TOMLS_TO_SED[@]}; do
    for dependency in ${DEPENDENCIES_TO_UPDATE[@]}; do
        sed -i'' -E "s/(^${dependency}.*\")[0-9]+\.[0-9]+\.[0-9]+(\".*$)/\1${VERSION}\2/" ${cargo_toml}
    done
done
