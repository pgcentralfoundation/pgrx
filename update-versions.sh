#! /usr/bin/env bash
# Portions Copyright 2019-2021 ZomboDB, LLC.
# Portions Copyright 2021-2022 Technology Concepts & Design, Inc.
# <support@tcdi.com>
#
# All rights reserved.
#
# Use of this source code is governed by the MIT license that can be found in
# the LICENSE file.

# requires:
# * ripgrep

if [ "x$1" == "x" ]; then
    echo "usage:  ./update-verions.sh <VERSION>"
    exit 1
fi

set -ex

HEAD=$(git rev-parse HEAD)
VERSION=$1

# ordered in a topological fashion starting from the workspace root Cargo.toml
# this isn't always necessary, but it's nice to use a mostly-consistent ordering
CARGO_TOMLS_TO_BUMP=(
    ./Cargo.toml
    ./pgx-pg-config/Cargo.toml
    ./pgx-utils/Cargo.toml
    ./cargo-pgx/Cargo.toml
    ./pgx-macros/Cargo.toml
    ./pgx-pg-sys/Cargo.toml
    ./pgx/Cargo.toml
    ./pgx-tests/Cargo.toml
)

CARGO_TOMLS_TO_SED=(
    ./Cargo.toml
    ./pgx-pg-config/Cargo.toml
    ./pgx-utils/Cargo.toml
    ./cargo-pgx/Cargo.toml
    ./pgx-macros/Cargo.toml
    ./pgx-pg-sys/Cargo.toml
    ./pgx/Cargo.toml
    ./pgx-tests/Cargo.toml
    ./pgx-examples/*/Cargo.toml
    ./cargo-pgx/src/templates/cargo_toml
    ./nix/templates/default/Cargo.toml
)

DEPENDENCIES_TO_UPDATE=(
    "pgx-pg-config"
    "pgx-utils"
    "cargo-pgx"
    "pgx-macros"
    "pgx-pgx-sys"
    "pgx"
    "pgx-tests"
)

SEMVER_REGEX="(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?"

for cargo_toml in ${CARGO_TOMLS_TO_SED[@]}; do
    for dependency in ${DEPENDENCIES_TO_UPDATE[@]}; do
        rg --passthru --no-line-number \
        "(?P<prefix>^${dependency}.*\")(?P<pin>=?)${SEMVER_REGEX}(?P<postfix>\".*$)" \
        -r "\${prefix}=${VERSION}\${postfix}" \
        ${cargo_toml} > ${cargo_toml}.tmp || true
        mv ${cargo_toml}.tmp ${cargo_toml}
    done
done

for cargo_toml in ${CARGO_TOMLS_TO_BUMP[@]}; do
    rg --passthru --no-line-number "(?P<prefix>^version = \")${SEMVER_REGEX}(?P<postfix>\"$)" -r "\${prefix}${VERSION}\${postfix}" ${cargo_toml} > ${cargo_toml}.tmp || true
    mv ${cargo_toml}.tmp ${cargo_toml}
done

cargo generate-lockfile

PGX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE=1 cargo test --no-run --workspace --no-default-features --features "pg14"
