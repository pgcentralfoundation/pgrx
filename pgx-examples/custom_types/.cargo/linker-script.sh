#! /usr/bin/env bash
x86_64-linux-gnu-gcc -Wl,-undefined,dynamic_lookup,-dynamic-list=$CARGO_MANIFEST_DIR/.cargo/test.txt $@
