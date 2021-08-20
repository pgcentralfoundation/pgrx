#! /usr/bin/env bash

if [[ $CARGO_BIN_NAME == "sql-generator" ]]; then
    gcc -Wl,-undefined,dynamic_lookup,-dynamic-list=$CARGO_MANIFEST_DIR/.cargo/pgx-dynamic-list.txt $@
else 
    gcc -Wl,-undefined,dynamic_lookup $@
fi