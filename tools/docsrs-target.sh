#LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
#LICENSE
#LICENSE All rights reserved.
#LICENSE
#LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

# Shared by tools/docsrs-check.sh and tools/docsrs-typecheck.sh, which source it from
# the repository root.
#
# docs.rs documents pgrx-pg-sys for the single target in its
# [package.metadata.docs.rs] targets, and the committed C macro snapshot rejects every
# other target. Checking from another host cross-compiles for that target, which
# needs its Rust standard library and a C compiler for it: cee-scape compiles C in
# its build script.

# Print the single docs.rs target of pgrx-pg-sys.
documented_target() {
    local target
    target="$(awk '
        /^\[/ { section = $0; next }
        section == "[package.metadata.docs.rs]" && $1 == "targets" { print }
    ' pgrx-pg-sys/Cargo.toml | sed -n 's/^targets *= *\["\([^"]*\)"\] *$/\1/p')"
    if [[ -z "${target}" ]]; then
        echo "pgrx-pg-sys/Cargo.toml must name exactly one [package.metadata.docs.rs] target" >&2
        return 1
    fi
    echo "${target}"
}

# Fail early when this host cannot build for docs.rs target $2 with Rust toolchain $1,
# which may be empty for the repository's default toolchain.
require_target_toolchain() {
    local toolchain="${1:++$1}" target="$2"
    local host
    host="$(rustc ${toolchain} -vV | awk '/^host:/ { print $2 }')"
    if [[ "${host}" == "${target}" ]]; then
        return 0
    fi
    if [[ ! -d "$(rustc ${toolchain} --print target-libdir --target "${target}")" ]]; then
        echo "docs.rs builds ${target}; install its standard library with" \
            "\`rustup${1:+ +$1} target add ${target}\`" >&2
        return 1
    fi
    local underscored="${target//-/_}"
    if [[ -z "$(printenv "CC_${target}")$(printenv "CC_${underscored}")${TARGET_CC:-}${CC:-}" ]] &&
        ! command -v "${target/-unknown-/-}-gcc" > /dev/null; then
        echo "docs.rs builds ${target}, and cee-scape compiles C for it. Install" \
            "${target/-unknown-/-}-gcc, or set CC_${underscored} to a cross C compiler and" \
            "CFLAGS_${underscored} to its flags, such as --sysroot=<${target} sysroot>." >&2
        return 1
    fi
}
