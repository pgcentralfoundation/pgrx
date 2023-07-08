#! /bin/sh

DIR=`pwd`
set -x

cd $DIR/pgrx-pg-config && cargo publish
cd $DIR/pgrx-sql-entity-graph && cargo publish
cd $DIR/pgrx-macros && cargo publish
cd $DIR/pgrx-pg-sys && cargo publish --no-verify
cd $DIR/pgrx && cargo publish --no-verify
cd $DIR/pgrx-tests && cargo publish --no-verify
cd $DIR/cargo-pgrx && cargo publish # cargo-pgrx last so the templates are correct
