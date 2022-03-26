# Copyright 2019-2022 ZomboDB, LLC and Technology Concepts & Design, Inc.
# <support@tcdi.com>. All rights reserved.  Use of this source code is governed
# by the MIT license that can be found in the LICENSE file.
#! /bin/sh

DIR=`pwd`
set -x

cd $DIR/pgx-utils && cargo publish && sleep 30
cd $DIR/pgx-macros && cargo publish && sleep 30
cd $DIR/pgx-pg-sys && cargo publish --no-verify && sleep 30
cd $DIR/pgx && cargo publish --no-verify && sleep 30
cd $DIR/pgx-tests && cargo publish --no-verify && sleep 30
cd $DIR/cargo-pgx && cargo publish

