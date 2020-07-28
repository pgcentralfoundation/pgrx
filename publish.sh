#! /bin/sh

DIR=`pwd`
set -x

cd $DIR/pgx-utils && cargo publish && sleep 10
cd $DIR/pgx-macros && cargo publish && sleep 10
cd $DIR/pgx-pg-sys && cargo publish --no-verify && sleep 10
cd $DIR/pgx && cargo publish --no-verify && sleep 10
cd $DIR/pgx-tests && cargo publish --no-verify && sleep 10
cd $DIR/cargo-pgx && cargo publish && sleep 10

