#! /bin/sh

DIR=`pwd`
set -x

cd $DIR/pgx-utils && cargo publish && sleep 30
cd $DIR/pgx-macros && cargo publish && sleep 30
cd $DIR/pgx-pg-sys && cargo publish --no-verify && sleep 30
cd $DIR/pgx && cargo publish --no-verify && sleep 30
cd $DIR/pgx-tests && cargo publish --no-verify && sleep 30
cd $DIR/cargo-pgx && cargo publish

