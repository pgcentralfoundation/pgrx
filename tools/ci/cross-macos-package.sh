#!/usr/bin/env bash
#LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
#LICENSE
#LICENSE All rights reserved.
#LICENSE
#LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#
# Package a PG18 extension for macOS on Linux. Inspect the package without running target code.

set -euo pipefail

toolchain="${RUNNER_TEMP:?}/osxcross"
# This image contains just the SDK and toolchain, built on Ubuntu 24.04. Pin both the SDK
# release and the image digest; its wrappers use the host's Clang and relocate with this tree.
if [[ ! -x "$toolchain/bin/oa64-clang" ]]; then
    image=ghcr.io/crazy-max/osxcross:15.5-r0-ubuntu@sha256:ab9cb3c178279fb1c1d94b87d2ec860969f07ba154466aacc593ece6dcafd62f
    tools/ci/retry.sh docker pull "$image"
    container="$(docker create "$image" unused)"
    trap 'docker rm "$container" > /dev/null' EXIT
    docker cp "$container:/osxcross" "$toolchain"
    docker rm "$container" > /dev/null
    trap - EXIT
fi

# OSXCross invokes clang by name; use the same LLVM installation as bindgen.
clang_bin="$(dirname "$(readlink -f "${CLANG_PATH:?}")")"
export PATH="$clang_bin:$toolchain/bin:$PATH"
export LD_LIBRARY_PATH="$toolchain/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
export OSXCROSS_SDKROOT="$toolchain/SDK/MacOSX15.5.sdk"
export MACOSX_DEPLOYMENT_TARGET=11.0
export CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="$toolchain/bin/oa64-clang"
export CC_aarch64_apple_darwin="$toolchain/bin/oa64-clang"
export CXX_aarch64_apple_darwin="$toolchain/bin/oa64-clang++"
export AR_aarch64_apple_darwin=llvm-ar
export BINDGEN_EXTRA_CLANG_ARGS_aarch64_apple_darwin="-isysroot $OSXCROSS_SDKROOT -mmacosx-version-min=11.0"

scratch="$(mktemp -d "$RUNNER_TEMP/cross-package.XXXXXX")"
curl -fSL --retry 3 -o "$scratch/postgresql.tar.bz2" \
    https://ftp.postgresql.org/pub/source/v18.7/postgresql-18.7.tar.bz2
echo "441dfae4280cae3e0425e5f876f80559fb0731c140f1aae22b3e5a54d0411bfd  $scratch/postgresql.tar.bz2" \
    | sha256sum --check
tar -xjf "$scratch/postgresql.tar.bz2" -C "$scratch"
pg_source="$scratch/postgresql-18.7"

# Configure for Darwin and generate only headers. No target executable or server is run.
(
    cd "$pg_source"
    CC=oa64-clang CXX=oa64-clang++ PG_SYSROOT="$OSXCROSS_SDKROOT" \
        ./configure --build="$(sh ./config/config.guess)" --host=aarch64-apple-darwin \
        --prefix=/usr/local/pgsql --without-readline --without-zlib --without-icu \
        --with-system-tzdata=/usr/share/zoneinfo
    make -C src/backend generated-headers
    make -C src/include DESTDIR="$scratch/pg18" install
    make -C src/pl/plpgsql/src DESTDIR="$scratch/pg18" install-headers
)

# The target pg_config executable cannot run here. Describe the configured installation
# with a host script, relocating only its headers; package destinations stay target paths.
pg_config="$scratch/pg_config"
pg_cflags="$(sed -n 's/^CFLAGS = //p' "$pg_source/src/Makefile.global")"
cat > "$pg_config" <<EOF
#!/usr/bin/env bash
set -eu
case "\$1" in
    --version) echo 'PostgreSQL 18.7' ;;
    --includedir-server) echo '$scratch/pg18/usr/local/pgsql/include/server' ;;
    --libdir | --pkglibdir) echo '/usr/local/pgsql/lib' ;;
    --sharedir) echo '/usr/local/pgsql/share' ;;
    --cppflags) echo '-isysroot $OSXCROSS_SDKROOT' ;;
    --cflags) echo '$pg_cflags' ;;
    --configure) echo '--without-readline --without-zlib --without-icu' ;;
    *) echo "unsupported pg_config option: \$1" >&2; exit 1 ;;
esac
EOF
chmod +x "$pg_config"
export PGRX_HOME="$scratch/pgrx-home"
mkdir -p "$PGRX_HOME"
printf "[configs]\npg18 = '%s'\n" "$pg_config" > "$PGRX_HOME/config.toml"

cargo build --locked --package cargo-pgrx
"${CARGO_TARGET_DIR:-target}/debug/cargo-pgrx" pgrx package --package versioned_so --no-default-features \
    --features pg18 --target aarch64-apple-darwin --pg-config "$pg_config" \
    --out-dir "$scratch/package" --debug --no-distrib-packages --cargo=--locked

python3 - "$scratch/package" <<'PY'
from pathlib import Path
import re
import subprocess
import sys

package = Path(sys.argv[1])
library = Path('usr/local/pgsql/lib/versioned_so-0.0.0.dylib')
control = Path('usr/local/pgsql/share/extension/versioned_so.control')
sql = Path('usr/local/pgsql/share/extension/versioned_so--0.0.0.sql')
actual = {path.relative_to(package) for path in package.rglob('*') if path.is_file()}
assert actual == {library, control, sql}, f'unexpected package contents: {sorted(actual)}'
assert "default_version = '0.0.0'" in (package / control).read_text()
schema = (package / sql).read_text()
assert re.search(r'CREATE\s+FUNCTION\s+"hello_versioned_so"\(\)', schema), 'missing function SQL'
assert "AS 'versioned_so-0.0.0', 'hello_versioned_so_wrapper'" in schema, \
    'SQL does not name the packaged library and function'

# Parse the actual binary, including its platform: an iOS ARM64 dylib must fail too.
info = subprocess.check_output(
    ['llvm-readobj', '--file-headers', '--macho-version-min', str(package / library)], text=True
)
for required in ['Format: Mach-O arm64', 'FileType: DynamicLibrary (0x6)', 'Platform: macos']:
    assert required in info, f'missing {required!r}:\n{info}'
print(info)
print(f'Validated PG18 macOS package: {sorted(map(str, actual))}')
PY
