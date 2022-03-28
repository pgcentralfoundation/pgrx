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

cd $DIR/pgx-utils && cargo publish && sleep 30
cd $DIR/pgx-macros && cargo publish && sleep 30
cd $DIR/pgx-pg-sys && cargo publish --no-verify && sleep 30
cd $DIR/pgx && cargo publish --no-verify && sleep 30
cd $DIR/pgx-tests && cargo publish --no-verify && sleep 30
cd $DIR/cargo-pgx && cargo publish

