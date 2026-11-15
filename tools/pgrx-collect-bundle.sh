#!/bin/sh
#LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
#LICENSE
#LICENSE All rights reserved.
#LICENSE
#LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#
# Collect a partial pgrx bundle: the parts of a PostgreSQL installation that a build for this
# platform needs and that only this platform has. `cargo pgrx package --bundle` on another
# machine generates the rest.
#
# It needs tar and a POSIX shell, and no Rust or C compiler. On Windows, run it in Git Bash.
#
# This script copies what it is given. It stops only when it lacks what it needs to collect:
# the version that names the file, and the directories it copies. `cargo pgrx package --bundle`
# checks the bundle when it uses it.

set -eu

usage() {
    cat >&2 <<'EOF'
usage: pgrx-collect-bundle.sh --target=RUST_TARGET [options]

  --target=RUST_TARGET     the Rust target triple of the platform, such as
                           aarch64-unknown-linux-gnu (required)
  --out-dir=DIR            where to write the bundle (default: this directory)

What pg_config reports comes from these, and a later one overrides an earlier one:

  --pg-config=PATH         a pg_config program to run. Without --pg-config-file or a
                           property, the first pg_config on PATH is run.
  --pg-config-file=FILE    a file holding what pg_config prints: NAME = value lines
  --PROPERTY=VALUE         one property, named as pg_config's option is: --version=,
                           --includedir-server=, --pkglibdir=, --sharedir=, --cppflags=,
                           --cflags=, and for MSVC targets --pkgincludedir= and --libdir=

The server headers are copied from the directory that INCLUDEDIR-SERVER names, so pass
--includedir-server=DIR when they are somewhere else on this machine.

Writes pgrx-pg<major>-<target>.partial.tar.gz and prints its path.
EOF
    exit 2
}

fail() {
    echo "pgrx-collect-bundle.sh: $*" >&2
    exit 1
}

staging="$(mktemp -d)"
trap 'rm -rf "$staging"' EXIT
trap 'exit 1' HUP INT TERM

target=
out_dir=.
pg_config=
pg_config_file=
: > "$staging/properties"
for argument in "$@"; do
    case "$argument" in
        --target=*) target="${argument#*=}" ;;
        --out-dir=*) out_dir="${argument#*=}" ;;
        --pg-config=*) pg_config="${argument#*=}" ;;
        --pg-config-file=*) pg_config_file="${argument#*=}" ;;
        -h | --help) usage ;;
        --*=*)
            # Any other option is a property, written the way pg_config prints one.
            name="${argument%%=*}"
            name="$(printf '%s' "${name#--}" | tr '[:lower:]' '[:upper:]')"
            printf '%s = %s\n' "$name" "${argument#*=}" >> "$staging/properties"
            ;;
        *) echo "unknown argument: $argument" >&2; usage ;;
    esac
done
[ -n "$target" ] || { echo "--target is required" >&2; usage; }

# What marks the archive as a partial bundle, and the version of this layout.
echo 1 > "$staging/partial-bundle"
printf '%s\n' "$target" > "$staging/target"

# Everything known about the PostgreSQL, as pg_config prints it.
{
    if [ -n "$pg_config" ] || { [ -z "$pg_config_file" ] && [ ! -s "$staging/properties" ]; }; then
        "${pg_config:-pg_config}"
    fi
    if [ -n "$pg_config_file" ]; then
        cat "$pg_config_file"
        # The file may not end its last line, and the next line must not join it.
        echo
    fi
    cat "$staging/properties"
} > "$staging/reported"
# A Windows pg_config ends its lines with a carriage return.
tr -d '\r' < "$staging/reported" > "$staging/pg_config.txt"
rm "$staging/properties" "$staging/reported"

# A value this script itself needs from pg_config.txt. The last line for a property is the
# one that counts, here and in cargo-pgrx.
reported() {
    value="$(sed -n "s/^$1 = //p" "$staging/pg_config.txt" | tail -n 1)"
    [ -n "$value" ] || fail "nothing reports $1; pass --$(echo "$1" | tr '[:upper:]' '[:lower:]')="
    printf '%s\n' "$value"
}

# The major version names the bundle.
version="$(reported VERSION)"
major="$(echo "$version" | sed -n 's/^PostgreSQL \([0-9][0-9]*\).*/\1/p')"
[ -n "$major" ] || fail "VERSION is '$version', not 'PostgreSQL <version>'"

# Copy a directory of headers. -L: an archive of plain files and directories, whatever the
# installation links to. A copy keeps the installation's modes, and one from a read-only tree,
# such as a Nix store, could not be pruned or removed here, so make it writable.
collect() {
    cp -RL "$1" "$2"
    chmod -R u+w "$2"
}

mkdir "$staging/include" "$staging/lib"
headers="$(reported INCLUDEDIR-SERVER)"
collect "$headers" "$staging/include/server"
case "$target" in
    *-msvc)
        # MSVC builds also read the public headers, and extensions link against postgres.lib.
        public_headers="$(reported PKGINCLUDEDIR)"
        collect "$public_headers" "$staging/include/pkg"
        case "$headers" in
            "$public_headers"/*)
                # The server headers were inside it, and are already collected.
                rm -rf "$staging/include/pkg/${headers#"$public_headers"/}"
                ;;
        esac
        libraries="$(reported LIBDIR)"
        cp "$libraries/postgres.lib" "$staging/lib/postgres.lib"
        ;;
esac

mkdir -p "$out_dir"
bundle="$out_dir/pgrx-pg$major-$target.partial.tar.gz"
# COPYFILE_DISABLE keeps macOS's tar from adding a metadata file beside each header. The
# archive goes to standard output because GNU tar reads a Windows path such as D:/out as a
# remote host. It is moved into place only once it is complete.
COPYFILE_DISABLE=1 tar -czf - -C "$staging" partial-bundle target pg_config.txt include lib \
    > "$staging/bundle.tar.gz"
mv "$staging/bundle.tar.gz" "$bundle"
echo "$bundle"
