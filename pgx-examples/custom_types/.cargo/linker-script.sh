#! /usr/bin/env bash

if [[ $CARGO_BIN_NAME == "sql-generator" ]]; then
    x86_64-linux-gnu-gcc -Wl,-undefined,dynamic_lookup,-dynamic-list=$CARGO_MANIFEST_DIR/.cargo/pgx-dynamic-list.txt $@
else 
    x86_64-linux-gnu-gcc -Wl,-undefined,dynamic_lookup $@
fi