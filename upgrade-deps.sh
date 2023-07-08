#! /usr/bin/env bash

# requires:  "cargo install cargo-edit" from https://github.com/killercup/cargo-edit
cargo update
cargo upgrade --incompatible --exclude syn
cargo generate-lockfile

# examples are their own independent crates, so we have to do them individually.
for folder in pgrx-examples/*; do
    if [ -d "$folder" ]; then
        cd $folder
        cargo update
        cargo upgrade --incompatible --exclude syn
        cargo generate-lockfile
        cargo check || exit $?
        cd -
    fi
done
