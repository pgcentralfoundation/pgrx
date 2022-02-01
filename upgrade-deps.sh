#! /usr/bin/env bash

# requires:  "cargo install cargo-edit" from https://github.com/killercup/cargo-edit
cargo update
cargo upgrade --workspace
cargo generate-lockfile

# examples are their own independent crates, so we have to do them individually.
for folder in pgx-examples/*; do
    if [ -d "$folder" ]; then
        cd $folder
        cargo update
        cargo upgrade
        cargo generate-lockfile
        cd -
    fi
done