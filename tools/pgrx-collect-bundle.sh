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
# Run this where PostgreSQL's server headers are installed. It needs pg_config, tar and a
# POSIX shell, and no Rust or C compiler. On Windows, run it in Git Bash.

set -eu

usage() {
    cat >&2 <<'EOF'
usage: pgrx-collect-bundle.sh --target RUST_TARGET [options]

  --target RUST_TARGET        the Rust target triple of this platform, such as
                              aarch64-unknown-linux-gnu (required)
  --pg-config PATH            the pg_config of the PostgreSQL to collect
                              (default: the first pg_config on PATH)
  --includedir-server DIR     that PostgreSQL's server headers, when they are not
                              where pg_config reports them
  --out-dir DIR               where to write the bundle (default: this directory)

Writes pgrx-pg<major>-<target>.partial.tar.gz and prints its path.
EOF
    exit 2
}

fail() {
    echo "pgrx-collect-bundle.sh: $*" >&2
    exit 1
}

target=
pg_config=pg_config
includedir_server=
out_dir=.
while [ $# -gt 0 ]; do
    case "$1" in
        --target | --pg-config | --includedir-server | --out-dir)
            [ $# -ge 2 ] || { echo "$1 needs a value" >&2; usage; }
            case "$1" in
                --target) target="$2" ;;
                --pg-config) pg_config="$2" ;;
                --includedir-server) includedir_server="$2" ;;
                --out-dir) out_dir="$2" ;;
            esac
            shift 2
            ;;
        -h | --help) usage ;;
        *) echo "unknown argument: $1" >&2; usage ;;
    esac
done

[ -n "$target" ] || { echo "--target is required" >&2; usage; }
case "$target" in
    *[!A-Za-z0-9_.-]* | -* | *-) fail "$target is not a Rust target triple" ;;
    *-*-*) ;;
    *) fail "$target is not a Rust target triple" ;;
esac

# A Windows pg_config ends its lines with a carriage return.
ask() {
    "$pg_config" "$1" | tr -d '\r'
}

version="$(ask --version)" || fail "couldn't run $pg_config"
major="$(printf '%s\n' "$version" | sed -n 's/^PostgreSQL \([0-9][0-9]*\).*/\1/p')"
[ -n "$major" ] || fail "$pg_config reported the version as \"$version\""

[ -n "$includedir_server" ] || includedir_server="$(ask --includedir-server)"
[ -f "$includedir_server/pg_config.h" ] \
    || fail "$includedir_server has no pg_config.h; install PostgreSQL's server development package, or pass --includedir-server"

staging="$(mktemp -d)"
trap 'rm -rf "$staging"' EXIT
trap 'exit 1' HUP INT TERM

# What marks the archive as a partial bundle, and the version of this layout.
echo 1 > "$staging/partial-bundle"
printf '%s\n' "$target" > "$staging/target"
# Everything pg_config reports, exactly as it prints it.
"$pg_config" | tr -d '\r' > "$staging/pg_config.txt"

# -L: an archive of plain files and directories, whatever the installation links to.
mkdir "$staging/include" "$staging/lib"
cp -RL "$includedir_server" "$staging/include/server"
case "$target" in
    *-msvc)
        # MSVC builds also read the public headers, and extensions link against postgres.lib.
        pkgincludedir="$(ask --pkgincludedir)"
        cp -RL "$pkgincludedir" "$staging/include/pkg"
        case "$includedir_server" in
            "$pkgincludedir"/*)
                # The server headers were inside it, and are already collected.
                rm -rf "$staging/include/pkg/${includedir_server#"$pkgincludedir"/}"
                ;;
        esac
        libdir="$(ask --libdir)"
        cp "$libdir/postgres.lib" "$staging/lib/postgres.lib" \
            || fail "$libdir has no postgres.lib"
        ;;
esac

mkdir -p "$out_dir"
bundle="$out_dir/pgrx-pg$major-$target.partial.tar.gz"
# COPYFILE_DISABLE keeps macOS's tar from adding a metadata file beside each header.
COPYFILE_DISABLE=1 tar -czf "$bundle" -C "$staging" partial-bundle target pg_config.txt include lib
echo "$bundle"
