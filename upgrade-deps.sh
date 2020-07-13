#! /bin/bash

# requires:  "cargo install cargo-edit" from https://github.com/killercup/cargo-edit

DIR=`pwd`
set -x

cd $DIR/pgx && cargo upgrade
cd $DIR/pgx-utils && cargo upgrade
cd $DIR/pgx-macros && cargo upgrade
cd $DIR/pgx-tests && cargo upgrade
cd $DIR/pgx-pg-sys && cargo upgrade
cd $DIR/cargo-pgx && cargo upgrade

cd $DIR/pgx-examples/arrays && cargo upgrade
cd $DIR/pgx-examples/errors && cargo upgrade
cd $DIR/pgx-examples/srf && cargo upgrade
cd $DIR/pgx-examples/strings && cargo upgrade



