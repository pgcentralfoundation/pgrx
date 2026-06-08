#!/usr/bin/env bash
#LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
#LICENSE
#LICENSE All rights reserved.
#LICENSE
#LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

set -euo pipefail

PG_VER="${PG_VER:-14}"
PG_FEATURE="pg${PG_VER#pg}"
TOOLCHAIN="${DOCSRS_TOOLCHAIN:-nightly}"
PACKAGE_DIR="${CARGO_TARGET_DIR:-target}/package"

echo "Checking packaged pgrx-bindgen dependency metadata"
# Package the local internal dependency too so unpublished release-candidate
# workspace versions do not force pgrx-bindgen to resolve it from crates.io.
cargo "+${TOOLCHAIN}" package --allow-dirty --no-verify -p pgrx-pg-config -p pgrx-bindgen

shopt -s nullglob
crates=("${PACKAGE_DIR}"/pgrx-bindgen-*.crate)
if (( ${#crates[@]} == 0 )); then
    echo "Could not find packaged pgrx-bindgen crate in ${PACKAGE_DIR}" >&2
    exit 1
fi
crate="$(ls -t "${crates[@]}" | head -n 1)"

tmpdir="$(mktemp -d)"
trap 'rm -rf "${tmpdir}"' EXIT

manifest_path="$(tar -tf "${crate}" | grep '/Cargo.toml$' | head -n 1)"
lock_path="$(tar -tf "${crate}" | grep '/Cargo.lock$' | head -n 1 || true)"

tar -xOf "${crate}" "${manifest_path}" > "${tmpdir}/Cargo.toml"
if grep -A 8 '^\[dependencies.bindgen\]' "${tmpdir}/Cargo.toml" | grep -q '"experimental"'; then
    echo "pgrx-bindgen package enables bindgen/experimental, which breaks docs.rs via annotate-snippets" >&2
    exit 1
fi

if [[ -n "${lock_path}" ]]; then
    tar -xOf "${crate}" "${lock_path}" > "${tmpdir}/Cargo.lock"
    if grep -q 'name = "annotate-snippets"' "${tmpdir}/Cargo.lock"; then
        echo "pgrx-bindgen package lockfile contains annotate-snippets; check bindgen features" >&2
        exit 1
    fi
fi

echo "Running docs.rs-style rustdoc check for pgrx with ${PG_FEATURE} cshim"
host="$(rustc "+${TOOLCHAIN}" -vV | awk '/^host:/ { print $2 }')"
rustdoc_cmd=(
    cargo "+${TOOLCHAIN}" rustdoc
    -p pgrx
    --lib
    --no-default-features
    --features "${PG_FEATURE} cshim"
    -Zrustdoc-map
    -Zhost-config
    -Ztarget-applies-to-host
)

docsrs_target="${DOCSRS_TARGET:-}"
if [[ -z "${docsrs_target}" && "${host}" == "x86_64-unknown-linux-gnu" ]]; then
    docsrs_target="x86_64-unknown-linux-gnu"
fi

if [[ -n "${docsrs_target}" ]]; then
    rustdoc_cmd+=(--target "${docsrs_target}")
fi

docsrs_rustdocflags="${RUSTDOCFLAGS:-}"
docsrs_rustdocflags="${docsrs_rustdocflags:+${docsrs_rustdocflags} }--cfg docsrs -Z unstable-options --emit=invocation-specific --cap-lints warn"

DOCS_RS=1 RUSTDOCFLAGS="${docsrs_rustdocflags}" "${rustdoc_cmd[@]}"
