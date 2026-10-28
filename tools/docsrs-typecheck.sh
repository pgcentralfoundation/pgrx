#!/usr/bin/env bash
#LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
#LICENSE
#LICENSE All rights reserved.
#LICENSE
#LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

# Type-check pgrx-pg-sys as docs.rs builds it, against the committed bindings and
# C macro documentation snapshot, for each PostgreSQL version. Then check that the
# packaged crate stays within its size budget. No PostgreSQL or Clang is needed.

set -euo pipefail

VERSIONS="${DOCSRS_PG_VERSIONS:-15 16 17 18 19}"
MAX_CRATE_BYTES=$((5 * 1024 * 1024))
PACKAGE_DIR="${CARGO_TARGET_DIR:-target}/package"

# The single docs.rs target that release generation documents.
target="$(awk '
    /^\[/ { section = $0; next }
    section == "[package.metadata.docs.rs]" && $1 == "targets" { print }
' pgrx-pg-sys/Cargo.toml | sed -n 's/^targets *= *\["\([^"]*\)"\] *$/\1/p')"
if [[ -z "${target}" ]]; then
    echo "pgrx-pg-sys/Cargo.toml must name exactly one [package.metadata.docs.rs] target" >&2
    exit 1
fi

for version in ${VERSIONS}; do
    major="${version#pg}"
    echo "Type-checking the docs.rs build of pgrx-pg-sys for pg${major} on ${target}"
    DOCS_RS=1 CLANG_PATH=/nonexistent LIBCLANG_PATH=/nonexistent cargo check \
        -p pgrx-pg-sys \
        --lib \
        --test generated_c_macros \
        --no-default-features \
        --features "pg${major} cshim" \
        --target "${target}"
done

echo "Checking the packaged size of pgrx-pg-sys"
cargo package -p pgrx-pg-sys --no-verify --allow-dirty
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
