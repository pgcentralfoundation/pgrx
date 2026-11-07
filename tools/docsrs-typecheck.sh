#!/usr/bin/env bash
#LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
#LICENSE
#LICENSE All rights reserved.
#LICENSE
#LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

# Type-check pgrx-pg-sys and pgrx as docs.rs builds them, against the committed
# bindings and C macro documentation snapshot, for each PostgreSQL version. Then check
# that the packaged pgrx-pg-sys stays within its size budget. This needs no PostgreSQL
# or libclang, but it builds for the docs.rs target: on another host, that target's
# Rust standard library and a C compiler for it are required (see
# tools/docsrs-target.sh).

set -euo pipefail
source tools/docsrs-target.sh

# Every PostgreSQL version that pgrx-pg-sys builds has a `pgNN` feature.
VERSIONS="${DOCSRS_PG_VERSIONS:-$(awk '
    /^\[/ { section = $0; next }
    section == "[features]" && $1 ~ /^pg[0-9]+$/ { print $1 }
' pgrx-pg-sys/Cargo.toml)}"
if [[ -z "${VERSIONS//[[:space:]]/}" ]]; then
    echo "pgrx-pg-sys/Cargo.toml declares no pgNN feature" >&2
    exit 1
fi
MAX_CRATE_BYTES=$((5 * 1024 * 1024))
PACKAGE_DIR="${CARGO_TARGET_DIR:-target}/package"

# The single docs.rs target that release generation documents.
target="$(documented_target)"
require_target_toolchain "" "${target}"

for version in ${VERSIONS}; do
    major="${version#pg}"
    echo "Type-checking the docs.rs builds of pgrx-pg-sys and pgrx for pg${major} on ${target}"
    DOCS_RS=1 CLANG_PATH=/nonexistent LIBCLANG_PATH=/nonexistent cargo check \
        -p pgrx-pg-sys \
        --lib \
        --test generated_c_macros \
        --no-default-features \
        --features "pg${major} cshim" \
        --target "${target}"
    # Extensions documented for this version compile pgrx against its shells.
    DOCS_RS=1 CLANG_PATH=/nonexistent LIBCLANG_PATH=/nonexistent cargo check \
        -p pgrx \
        --lib \
        --no-default-features \
        --features "pg${major} cshim" \
        --target "${target}"
done

echo "Checking the packaged size of pgrx-pg-sys"
# Package the internal dependencies too, so a release's new, unpublished versions of
# them are not resolved from crates.io.
cargo package --no-verify --allow-dirty \
    -p pgrx-pg-config \
    -p pgrx-c-macros \
    -p pgrx-bindgen \
    -p pgrx-sql-entity-graph \
    -p pgrx-macros \
    -p pgrx-pg-sys
shopt -s nullglob
crates=("${PACKAGE_DIR}"/pgrx-pg-sys-*.crate)
if (( ${#crates[@]} == 0 )); then
    echo "Could not find packaged pgrx-pg-sys crate in ${PACKAGE_DIR}" >&2
    exit 1
fi
crate="$(ls -t "${crates[@]}" | head -n 1)"
bytes="$(wc -c < "${crate}" | tr -d ' ')"
echo "${crate}: ${bytes} bytes (budget ${MAX_CRATE_BYTES})"
if (( bytes > MAX_CRATE_BYTES )); then
    echo "the packaged pgrx-pg-sys exceeds its ${MAX_CRATE_BYTES}-byte budget" >&2
    exit 1
fi
