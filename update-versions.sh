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
# * Cargo extension 'cargo-edit'

if [ "$1" == "" ]; then
    echo "usage:  ./update-versions.sh <VERSION>"
    exit 1
fi

set -ex

if ! which rg &> /dev/null; then
    echo "Command \`rg\` (ripgrep) was not found. Please install it and try again."
    exit 1
fi

if ! cargo set-version --help &> /dev/null; then
    echo "Cargo extension \`cargo-edit\` is not installed. Please install it by running: cargo install cargo-edit"
    exit 1
fi

VERSION=$1

# Use `cargo set-version` to update all main project files
function update_main_files() {
    local version=$1

    EXCLUDE_PACKAGES=()

    # Add additional packages to ignore by using the following syntax:
    # EXCLUDE_PACKAGES+=('foo' 'bar' 'baz')

    # Ignore all packages in pgx-examples/
    for file in ./pgx-examples/**/Cargo.toml; do
        EXCLUDE_PACKAGES+=("$(rg --multiline '\[package\](.*\n)(?:[^\[]*\n)*name\s?=\s?"(?P<name>[-_a-z]*)"(.*\n)*' -r "\${name}" "$file")")
    done

    echo "Excluding the following packages:"
    echo "${EXCLUDE_PACKAGES[@]}"

    # shellcheck disable=2068 # allow the shell to split --exclude from each EXCLUDE_PACKAGES value
    cargo set-version ${EXCLUDE_PACKAGES[@]/#/--exclude } --workspace "$version"
}

# This is a legacy holdover for updating extra toml files throughout various crates
function update_extras() {
    local version=$1

    # ordered in a topological fashion starting from the workspace root Cargo.toml
    # this isn't always necessary, but it's nice to use a mostly-consistent ordering
    CARGO_TOMLS_TO_SED=(
        ./Cargo.toml
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
            -r "\${prefix}=${version}\${postfix}" \
            ${cargo_toml} > ${cargo_toml}.tmp || true
            mv ${cargo_toml}.tmp ${cargo_toml}
        done
    done

}

update_main_files $VERSION
update_extras $VERSION

cargo generate-lockfile

PGX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE=1 cargo test --no-run --workspace --no-default-features --features "pg14"
