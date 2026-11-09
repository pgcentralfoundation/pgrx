#!/usr/bin/env bash
#LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
#LICENSE
#LICENSE All rights reserved.
#LICENSE
#LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#
# Prepare a CI runner: the pinned Rust toolchain, PostgreSQL, libclang, and PGRX_HOME.
#
# Environment:
#   PG_VER           PostgreSQL major version to install and configure; empty installs none
#   CLANG_VER        Linux only: the Clang whose libclang the build loads. Empty installs the
#                    distribution's Clang when PG_VER is set, and none otherwise.
#   RUST_COMPONENTS  extra rustup components, separated by spaces
#   PG19_CACHE       macOS only: directory that holds, or receives, a PostgreSQL 19 build

set -euo pipefail

case "$(uname -s)" in
    Linux) platform=linux ;;
    Darwin) platform=macos ;;
    MINGW* | MSYS*) platform=windows ;;
    *) echo "unsupported platform $(uname -s)" >&2; exit 1 ;;
esac

github_env() {
    echo "$1" >> "$GITHUB_ENV"
    export "${1?}"
}

# Only the pinned toolchain: rust-toolchain.toml overrides `rustup default`, so installing
# other toolchains would only cost time. The bindings build script formats with rustfmt.
toolchain="$(tr -d '\r' < rust-toolchain.toml | sed -n 's/^channel *= *"\(.*\)"/\1/p')"
components=(--component rustfmt)
for component in ${RUST_COMPONENTS:-}; do
    components+=(--component "$component")
done
rustup toolchain install "$toolchain" --profile minimal --no-self-update "${components[@]}"

# Dependencies build without debuginfo, which would only enlarge the cache and slow linking.
# Workspace crates keep the workspace profile's line tables for backtraces, except on Windows.
cargo_home="${CARGO_HOME:-$HOME/.cargo}"
if [[ "$platform" == windows ]]; then
    cargo_home="$(cygpath -u "$cargo_home")"
fi
mkdir -p "$cargo_home"
cat >> "$cargo_home/config.toml" <<'EOF'
[profile.dev.package."*"]
debug = false
EOF
if [[ "$platform" == windows ]]; then
    # MSVC writes a PDB for every test binary and extension it links, which dominates
    # Windows links. Panic locations don't come from debuginfo, so workspace crates go
    # without it there too.
    cat >> "$cargo_home/config.toml" <<'EOF'
[profile.dev]
debug = false
EOF
fi

pg_config=""

setup_linux() {
    local codename packages=()
    codename="$(. /etc/os-release && echo "$VERSION_CODENAME")"

    if [[ -n "${CLANG_VER:-}" ]]; then
        packages+=("clang-$CLANG_VER" "libclang-$CLANG_VER-dev" "llvm-$CLANG_VER-dev")
        if ! apt-cache policy "clang-$CLANG_VER" 2> /dev/null | grep -q 'Candidate: [0-9]'; then
            curl -fsSL https://apt.llvm.org/llvm-snapshot.gpg.key \
                | sudo tee /etc/apt/trusted.gpg.d/apt.llvm.org.asc > /dev/null
            echo "deb http://apt.llvm.org/$codename/ llvm-toolchain-$codename-$CLANG_VER main" \
                | sudo tee /etc/apt/sources.list.d/llvm.list > /dev/null
        fi
        # The build loads the newest libclang it finds unless told which one to use.
        github_env "LIBCLANG_PATH=/usr/lib/llvm-$CLANG_VER/lib"
        github_env "CLANG_PATH=/usr/bin/clang-$CLANG_VER"
    elif [[ -n "${PG_VER:-}" ]]; then
        packages+=(clang libclang-dev llvm-dev)
    fi

    if [[ -n "${PG_VER:-}" ]]; then
        local repo="https://apt.postgresql.org/pub/repos/apt"
        local key="/usr/share/postgresql-common/pgdg/apt.postgresql.org.asc"
        sudo install -d "$(dirname "$key")" /etc/postgresql-common/createcluster.d
        sudo curl -fsSL -o "$key" https://www.postgresql.org/media/keys/ACCC4CF8.asc
        # Tests start their own servers, so the package's default cluster would only cost time.
        echo "create_main_cluster = false" \
            | sudo tee /etc/postgresql-common/createcluster.d/ci.conf > /dev/null
        {
            echo "deb [signed-by=$key] $repo $codename-pgdg main $PG_VER"
            # Beta and release-candidate packages live in the snapshot repository.
            echo "deb [signed-by=$key] $repo $codename-pgdg-snapshot main $PG_VER"
        } | sudo tee /etc/apt/sources.list.d/pgdg.list > /dev/null
        packages+=("postgresql-$PG_VER" "postgresql-server-dev-$PG_VER")
    fi

    if (( ${#packages[@]} )); then
        sudo apt-get update -qq
        sudo apt-get install -y -qq --no-install-recommends "${packages[@]}" > /dev/null
    fi

    if [[ -n "${PG_VER:-}" ]]; then
        pg_config="/usr/lib/postgresql/$PG_VER/bin/pg_config"
        sudo chmod a+rwx "$("$pg_config" --pkglibdir)" "$("$pg_config" --sharedir)/extension" \
            /var/run/postgresql
    fi
}

setup_macos() {
    [[ -n "${PG_VER:-}" ]] || return 0
    export HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_NO_INSTALL_CLEANUP=1 HOMEBREW_NO_INSTALLED_DEPENDENTS_CHECK=1
    if [[ "$PG_VER" == 19 ]]; then
        # Homebrew has no PostgreSQL 19 yet, so build it once with cargo-pgrx and cache it.
        local build_home="${PG19_CACHE:?PG19_CACHE must name the PostgreSQL 19 cache directory}"
        if [[ ! -f "$build_home/config.toml" ]]; then
            brew install --quiet icu4c pkgconf
            cargo build --locked -p cargo-pgrx
            # Homebrew's ICU is keg-only, so configure finds it only through its pkg-config files.
            PKG_CONFIG_PATH="$(brew --prefix icu4c)/lib/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}" \
                PGRX_HOME="$build_home" target/debug/cargo-pgrx pgrx init --pg19 download
        fi
        pg_config="$(sed -n "s/^pg19 = [\"']\(.*\)[\"']\$/\1/p" "$build_home/config.toml")"
    else
        brew install --quiet "postgresql@$PG_VER"
        pg_config="$(brew --prefix "postgresql@$PG_VER")/bin/pg_config"
    fi
    sudo chmod a+rwx "$("$pg_config" --pkglibdir)" "$("$pg_config" --sharedir)/extension"
}

setup_windows() {
    [[ -n "${PG_VER:-}" ]] || return 0
    # EnterpriseDB publishes binaries a few days after a release, so fall back a minor
    # version when the newest one is missing.
    local latest minor url archive="$RUNNER_TEMP/postgresql.zip"
    latest="$(curl -fsSL --retry 3 https://www.postgresql.org/versions.rss \
        | grep -o "[0-9.]* is the latest release in the $PG_VER series" | head -1 | cut -d' ' -f1)"
    if [[ -z "$latest" ]]; then
        echo "versions.rss names no PostgreSQL $PG_VER release" >&2
        exit 1
    fi
    minor="${latest#*.}"
    while (( minor >= 0 )); do
        url="https://get.enterprisedb.com/postgresql/postgresql-$PG_VER.$minor-1-windows-x64-binaries.zip"
        if curl -fsSL --retry 3 -o "$archive" "$url"; then
            break
        fi
        echo "no $url; trying the previous minor version"
        minor=$((minor - 1))
    done
    if (( minor < 0 )); then
        echo "found no EnterpriseDB binaries for PostgreSQL $PG_VER" >&2
        exit 1
    fi
    7z x -y -bd -o"$RUNNER_TEMP/postgresql-$PG_VER" "$archive" > /dev/null
    pg_config="$(cygpath -m "$RUNNER_TEMP/postgresql-$PG_VER/pgsql/bin/pg_config.exe")"
}

"setup_$platform"

if [[ -n "$pg_config" ]]; then
    "$pg_config" --version
    pgrx_home="$RUNNER_TEMP/pgrx-home"
    if [[ "$platform" == windows ]]; then
        pgrx_home="$(cygpath -m "$pgrx_home")"
    fi
    mkdir -p "$pgrx_home"
    github_env "PGRX_HOME=$pgrx_home"
    github_env "PG_CONFIG=$pg_config"
    # A TOML literal string keeps a Windows path as it is.
    printf "[configs]\npg%s = '%s'\n" "$PG_VER" "$pg_config" > "$pgrx_home/config.toml"
    cat "$pgrx_home/config.toml"
fi
