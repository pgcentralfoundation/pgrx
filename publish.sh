#! /bin/sh
# Portions Copyright 2019-2021 ZomboDB, LLC.
# Portions Copyright 2021-2022 Technology Concepts & Design, Inc.
# <support@tcdi.com>
#
# All rights reserved.
#
# Use of this source code is governed by the MIT license that can be found in
# the LICENSE file.

DIR=`pwd`
set -x

cd $DIR/pgx-pg-config && cargo publish
cd $DIR/pgx-sql-entity-graph && cargo publish
cd $DIR/pgx-macros && cargo publish
cd $DIR/pgx-pg-sys && cargo publish --no-verify
cd $DIR/pgx && cargo publish --no-verify
cd $DIR/pgx-tests && cargo publish --no-verify
cd $DIR/cargo-pgx && cargo publish # cargo-pgx last so the templates are correct
