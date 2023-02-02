#! /usr/bin/env bash
# Portions Copyright 2019-2021 ZomboDB, LLC.
# Portions Copyright 2021-2022 Technology Concepts & Design, Inc.
# <support@tcdi.com>
#
# All rights reserved.
#
# Use of this source code is governed by the MIT license that can be found in
# the LICENSE file.

# requires:  "cargo install cargo-edit" from https://github.com/killercup/cargo-edit
cargo update
cargo upgrade
cargo generate-lockfile

# examples are their own independent crates, so we have to do them individually.
for folder in pgx-examples/*; do
    if [ -d "$folder" ]; then
        cd $folder
        cargo update
        cargo upgrade
        cargo generate-lockfile
        cargo check
        cd -
    fi
done
