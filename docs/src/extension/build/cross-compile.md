# Cross compiling `pgrx` (on Linux)

*Warning: this guide is still a work in progress!*

## Caveats

Note that guide is fairly preliminary and does not cover many cases, most notably:

1. This does not (yet) cover cross compiling with `cargo pgrx` (planned). Note that this means this documentation may only be useful to a small set of users.

2. Generating bindings on the build host needs a Clang driver and `llvm-ar` matching `libclang`, the target's C library headers, and PostgreSQL server headers configured for the target. A complete target bundle needs none of these: it includes its native inline and macro adapters, plus the C shim when the producer enabled the `cshim` feature. Producer and consumer must select the same `cshim` feature.

3. This guide is assuming that you are cross compiling between `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu` (either direction works).

    - Compiling between other architectures (but still Linux on both ends) will likely be similar, but are left as an exercise for the reader.
    - Compiling between other OSes is... considerably more difficult, and not officially supported at the moment (although it is possible with quite a bit of pain).
    - Non-`gnu` linux targets have not been investigated at all (this mostly applies to `-musl` targets like Alpine, as I doubt PG will work well under uclibc).

    Other cases are encouraged to investigate Docker, QEMU, and so on -- it's honestly probably the easier path for a lot of cases.

## PostgreSQL headers and metadata for the target

`pgrx-pg-sys` generates its Rust bindings and C macros from PostgreSQL's server
headers, and compiles a small native support archive for the target. Every
build does this, and a failure stops the build. A cross build therefore needs:

- server headers configured for the target, from the same PostgreSQL build as
  the server that will load the extension;
- PostgreSQL metadata for those headers: the version, CPPFLAGS, CFLAGS and the
  library directory;
- a Clang driver and `llvm-ar` that match the `libclang` bindgen loads, as
  described in [Clang, LLVM and system headers](#clang-llvm-and-system-headers).

There are three supported ways to provide the headers and metadata. Raw Rust
bindings alone and documentation snapshots do not supply the target's macro or
native semantics.

### Host `pg_config` with target headers

Keep the host installation's `pg_config`, from `cargo pgrx init` or
`PGRX_PG_CONFIG_PATH`, and name the target's server headers in a variable
suffixed with Cargo's target triple:

```sh
export PGRX_INCLUDEDIR_SERVER_PG18_aarch64_unknown_linux_gnu="$HOME/pg-arm64/usr/include/postgresql/18/server"
cargo build --target aarch64-unknown-linux-gnu --release
```

The suffix can be spelled as Cargo spells the target,
`aarch64-unknown-linux-gnu`, or with underscores, which shells can export.
`env` and Cargo's `[env]` table accept either spelling. pgrx uses the first of
these variables that is set, trying the hyphenated suffix before the underscored
one, and otherwise `pg_config --includedir-server`:

1. `PGRX_INCLUDEDIR_SERVER_PG18_<target>`
2. `PGRX_INCLUDEDIR_SERVER_PG18`
3. `PGRX_INCLUDEDIR_SERVER_<target>`
4. `PGRX_INCLUDEDIR_SERVER`

The target-suffixed names apply only to builds for that target, so a host build
in the same Cargo invocation keeps the host's headers. The selected directory
also owns the generated macros: pgrx translates the macros defined in that tree.

The host `pg_config` still supplies the version, CPPFLAGS, the `--configure`
output and the library directory. Its CFLAGS describe the host's server, so when
Cargo's host and target differ pgrx applies only PostgreSQL's functional flags:
`-fno-strict-aliasing`, `-fwrapv` and `-fexcess-precision=standard`. The other
flags, such as `-O2`, `-fcf-protection` or `-mno-omit-leaf-frame-pointer`, are
host code generation and instruction set options. The build lists them under
`cflags_dropped` in `OUT_DIR/pg18_macro_report.json`, and `PGRX_MACRO_DEBUG=1`
prints them. CPPFLAGS still apply in full. If the host and target run different
distributions, check that host values such as `-D_GNU_SOURCE` or
`-I/usr/include/libxml2` also suit the target.

pgrx cannot tell a `pg_config` program for the target, run under emulation or
replicated by a shell script, from a host one. It applies only the functional
CFLAGS from such a program too. Use the next path to apply a target's full CFLAGS.

### PostgreSQL metadata in the environment

Set `PGRX_PG_CONFIG_AS_ENV=true` and describe the target's installation with
`PGRX_PG_CONFIG_*` variables. Take the values from `pg_config` on a machine that
runs the deployed server, except the header directory, which must exist on the
build host:

| Variable | Value |
| --- | --- |
| `PGRX_PG_CONFIG_VERSION` | `pg_config --version`, for example `PostgreSQL 18.1` |
| `PGRX_PG_CONFIG_INCLUDEDIR-SERVER` | where the target's server headers are on the build host |
| `PGRX_PG_CONFIG_CPPFLAGS` | `pg_config --cppflags` |
| `PGRX_PG_CONFIG_CFLAGS` | `pg_config --cflags` |
| `PGRX_PG_CONFIG_LIBDIR` | `pg_config --libdir` |

pgrx then runs no `pg_config` program and applies every CFLAGS value. The
`--configure` output is not needed: only a `pg_config` program's `CLANG` entry
is used, as a hint for the Clang driver. Shells
cannot export `PGRX_PG_CONFIG_INCLUDEDIR-SERVER` because of its hyphen, so set
the variables with `env`:

```sh
env PGRX_PG_CONFIG_AS_ENV=true \
    PGRX_PG_CONFIG_VERSION="PostgreSQL 18.1" \
    "PGRX_PG_CONFIG_INCLUDEDIR-SERVER=$HOME/pg-arm64/usr/include/postgresql/18/server" \
    PGRX_PG_CONFIG_CPPFLAGS="$target_cppflags" \
    PGRX_PG_CONFIG_CFLAGS="$target_cflags" \
    PGRX_PG_CONFIG_LIBDIR=/usr/lib/aarch64-linux-gnu \
    cargo build --target aarch64-unknown-linux-gnu --release
```

These variables are not target-scoped. Every build of `pgrx-pg-sys` in that
Cargo invocation uses them, and the version must match the selected `pgNN`
feature. Without `PGRX_PG_CONFIG_CFLAGS`, the report records
`cflags_recorded: false` and the profile lacks PostgreSQL's `-fwrapv`. Generated
macros then treat signed overflow as undefined, as C does by default, instead of
wrapping as PostgreSQL's own build does, and can differ from a native build.
Supply the target's CFLAGS.

### A complete target bundle

A bundle produced on the target machine, or by a cross build that used one of
the paths above, contains the bindings, the generated macros and the native
archive. Importing it needs no Clang and no server headers, but still needs the
target's PostgreSQL configuration. See
[Import a complete target bundle](#import-a-complete-target-bundle).

### What pgrx verifies

Before generating anything, pgrx compares the server headers with the Clang
target and refuses headers that do not describe it:

- **Machine.** `PG_VERSION_STR` in `pg_config.h` names the machine configure or
  meson built for, such as `on x86_64-pc-linux-gnu` or `on aarch64-darwin`. Its
  architecture and operating system must match the Clang target. Its C library
  and ARM float ABI must match too when it names them, as autoconf's GNU and
  musl triples such as `x86_64-pc-linux-gnu` and `arm-unknown-linux-gnueabihf`
  do; meson's `<cpu>-<system>` spelling and Windows builds, MinGW's
  `x86_64-w64-mingw32` or MSVC, name neither. meson records DragonFly
  as `netbsd` and Android as `linux`, so those spellings also match those
  systems. A machine that pgrx does not recognize is accepted only when the
  target has the build host's architecture and operating system.
- **macOS slices.** A universal macOS installation such as Postgres.app is
  configured once with `-arch arm64 -arch x86_64` in its CFLAGS and compiles
  every slice from the same headers, while `PG_VERSION_STR` names only the
  machine configure ran on. For an installation's own macOS headers, the
  `-arch` values decide the architecture: the target's must be one of them.
  The `-arch` flags themselves stay out of the profile, because Clang cannot
  preprocess for two slices at once and Cargo's target selects the slice; the
  report lists them under `cflags_dropped`.
- **Layout.** One parse with the target compiler compares the sizes, alignments,
  `MAXIMUM_ALIGNOF`, byte order and 64-bit integer flags recorded in
  `pg_config.h` with the target, and `PG_MAJORVERSION_NUM` with the selected
  `pgNN` feature. The error lists every disagreement.

Both checks are needed. x86_64 and aarch64 Linux share every layout fact, so
x86_64 headers pass the layout check for an aarch64 target and fail the machine
check. x86_64 headers pass the machine check for the x32 target
`x86_64-unknown-linux-gnux32` and fail the layout check.

Headers configured for another machine are refused even when every layout fact
agrees. `pg_config.h` records configure's results for one machine, and other
headers choose declaration shapes from them. PostgreSQL 15 to 18, built for
x86_64 with a run-time check for the POPCNT instruction, declare `pg_popcount32`
in `port/pg_bitutils.h` as a pointer to a function. An aarch64 server exports a
function of that name instead. Rust bindings generated from the x86_64 headers
would load that function's first instructions as a pointer and call the result.
The layout check cannot see this kind of difference, so the machine check is the
only evidence against it. A refused build reports, for example:

```text
PostgreSQL headers do not describe the inspected C target: pg_config.h was configured for x86_64-pc-linux-gnu (PG_VERSION_STR), but Clang targets aarch64-unknown-linux-gnu
```

The error chain also names the PostgreSQL version, the Cargo target, the server
header directory and the metadata source, and gives the remedies:

```text
these server headers do not describe aarch64-unknown-linux-gnu; set PGRX_INCLUDEDIR_SERVER_PG18_aarch64-unknown-linux-gnu (or PGRX_INCLUDEDIR_SERVER_PG18_aarch64_unknown_linux_gnu) to PostgreSQL 18 server headers configured for aarch64-unknown-linux-gnu, describe the target with PGRX_PG_CONFIG_AS_ENV, or import a target bundle with PGRX_TARGET_INFO_PATH_PG18
```

The checks do not prove that the headers came from the deployed server. These
remain trusted:

- the minor version and the packaging of the headers;
- instruction set options the server's build chose within one architecture,
  such as `USE_SVE_POPCNT_WITH_RUNTIME_CHECK`, which changes how PostgreSQL 18
  and later declare `pg_popcount_optimized` on aarch64, or the `USE_AVX512_*`
  options on x86_64;
- the C library and ARM float ABI, when `PG_VERSION_STR` does not name them,
  as with meson and Windows builds;
- C library feature macros (`HAVE_*`) within one C library family;
- build options such as `BLCKSZ`, `NAMEDATALEN` and `USE_ASSERT_CHECKING`;
- the host's CPPFLAGS, when a host `pg_config` supplies the metadata.

Dropping host CFLAGS also means the native support archive is compiled without
the server's optimization and hardening flags. Take the headers from the same
PostgreSQL build as the server that will load the extension.

### Clang, LLVM and system headers

The `pgrx-pg-sys` build script runs Clang itself to find system headers, for
header inspection and for its native support archive, rather than through the
`cc` crate.

- **Driver.** Its major version must match `libclang`. pgrx uses `CLANG_PATH`
  when it is set; otherwise it uses the first match among the `CLANG` recorded
  by a `pg_config` program's `--configure` output, the `clang` installed with
  `libclang`, and `clang-<major>` or `clang` on `PATH`.
- **Configuration files.** Clang 16 and later load default configuration
  files named for the target, and some distributions install them, such as
  Gentoo's `/etc/clang/<major>/x86_64-unknown-linux-gnu-clang.cfg`. Their
  options would apply to the driver but appear in no recorded profile, so pgrx
  refuses a driver that reports `Configuration file:`. Set
  `CLANG_NO_DEFAULT_CONFIG=1`, or add `--no-default-config` to
  `BINDGEN_EXTRA_CLANG_ARGS`, to build without them.
- **Target arguments.** pgrx passes `--target=<Cargo target>` unless the
  arguments already name a target, followed by
  `BINDGEN_EXTRA_CLANG_ARGS_<target>` or, if that is unset,
  `BINDGEN_EXTRA_CLANG_ARGS`. The target suffix can be hyphenated or
  underscored. The same arguments reach bindgen, header inspection, include
  path detection and native compilation. Use them to name the target's sysroot
  (`--sysroot=...`), or its headers and GCC installation (`-isystem` and
  `-ccc-gcc-name`), when Clang cannot find them.
- **System headers.** pgrx asks the same Clang driver for its system include
  directories with the arguments of the inspected profile without their `-I`
  options: the target arguments above, the applied CFLAGS and PostgreSQL's
  CPPFLAGS. A `--sysroot` or `-isysroot` in any of them, such as the SDK that
  PostgreSQL's macOS build records in CPPFLAGS, selects the search list, so
  system headers come from the target's sysroot rather than the host's or the
  driver's default SDK. Bindgen and header inspection both receive those
  directories as system directories, searched after every `-I` directory as
  the driver searches them, and the build fails if inspection then selects a
  different driver. The report records the query under `include_detection`.
- **Archiver.** pgrx uses `llvm-ar-<major>` beside a `clang-<major>` driver,
  then `llvm-ar` beside the driver, then `llvm-ar` beside the driver's resolved
  path, such as Debian's `/usr/lib/llvm-<major>/bin`. On Debian, install the
  `llvm-<major>` package that matches `clang-<major>`; `/usr/bin/llvm-ar`
  belongs to the distribution's default LLVM release. Without an LLVM archiver
  pgrx uses the system `ar`, which may write an archive without a symbol index
  for another machine's objects. The link can then fail.
- **`cc` variables.** `CC_<target>`, `AR_<target>`, `CXX_<target>` and
  `CROSS_COMPILE` do not affect `pgrx-pg-sys`'s native support. Other crates
  that compile C with the `cc` crate still read them.

The `cshim` feature needs nothing extra. The same Clang compiles the C shim into
the native support archive with the arguments above.

## Import a complete target bundle

Build `pgrx-pg-sys` on the target machine, or with target headers and the Clang
toolchain configured as in
[PostgreSQL headers and metadata for the target](#postgresql-headers-and-metadata-for-the-target).
Use the same pgrx release as the consuming extension; development builds must
also use the same source revision. Set
`PGRX_PG_SYS_EXTRA_TARGET_INFO_PATH` to a dedicated directory for the selected
PostgreSQL major and Cargo target. For example, on an aarch64 Linux build host:

```sh
PGRX_PG_SYS_EXTRA_TARGET_INFO_PATH=/tmp/pgrx-pg18-aarch64-info \
    cargo build -p pgrx-pg-sys --target aarch64-unknown-linux-gnu \
    --no-default-features --features pg18
```

The directory contains `pg18_target_artifact.json`, the original raw bindings,
the complete `cmacros/pg18` Rust tree, the compiler/profile report, original
native wrapper/support sources and their linked target archive. Module-magic and
function-info metadata are included in the generated support and archive. The
manifest records SHA-256 and byte lengths for every member and binds the bundle
to the PostgreSQL major, exact Rust target, `cshim` feature and generator version.
These checks detect corruption or mixed builds; obtain the bundle from a trusted
build. Keep it intact when copying it to the host machine.

To import it, select the same PostgreSQL major and feature set, configure the
Rust target linker, and point `PGRX_TARGET_INFO_PATH_PG18` at the copied directory:

```sh
PGRX_TARGET_INFO_PATH_PG18=/opt/pgrx-targets/pg18-aarch64-info \
    cargo build --target aarch64-unknown-linux-gnu --release \
    --no-default-features --features pg18
```

The importer validates all members and the compiler target before publishing
Rust source or linking the bundled native archive. It rejects missing manifests,
raw-only imports, changed bytes, extra files, mismatched targets/versions/features,
and symlinks at the bundle root or within the bundle. Configured ancestor path
aliases are resolved normally. Members are limited to 128 MiB each, 512 MiB in
total and 8192 files/directory entries; manifests are limited to 4 MiB. Export
replaces only an intact bundle of the same domain or an empty directory, preserving
unrelated files on refusal.

Target PostgreSQL configuration and any system/PostgreSQL import libraries must
still describe the server where the extension will run. The bundle supplies
bindings and generated support; it does not install PostgreSQL or a Rust linker.
The existing `PGRX_PG_SYS_EXTRA_OUTPUT_PATH` remains a raw-binding file export
and cannot be used by itself as a target bundle.

An additional caveat (one that is unrelated to the document's completeness) is that the cross-compilation process is highly specific to your Linux distribution, unfortunately. This guide covers Debian (where it's easy) and Fedora (where it's hard). The Debian steps should apply to any `apt`-using distribution, and most of the Fedora steps will apply to other distributions with minimal conversion.

# System reqirements

1. Postgres headers configured for the target (which we'll talk about shortly).
2. A cross compilation toolchain (which we'll also talk about shortly).
3. Everything you'd need to build things on the host (which we won't talk about, but install it).
4. A Rust toolchain for the target (which we also won't talk about).

## Getting PostgreSQL headers for the target
[getting-headers]: #getting-postgresql-headers-for-the-target

To generate the bindings for `pgrx-pg-sys` we need the PostgreSQL headers. Unfortunately, a few of these headers get generated when Postgres is compiled and contain non-portable information, which means that we can't just use the headers for the version of postgres you have installed. pgrx refuses headers configured for another machine (see [What pgrx verifies](#what-pgrx-verifies)). You need to get the server headers from a build PostgreSQL done for the cross-compile target. The version of PostgreSQL *must* match.

*Caveat: pgrx refuses headers for another major version or another machine, but it cannot detect a different minor version or different configure options. Such a mismatch will _probably not_ be detected at build time, and can result in an extension that fails _terribly_ at runtime. Make sure the headers come from the server you deploy.*

To minimize the chance for compatibility issues: The headers should be produced by a PostgreSQL build as close as possible to the one which will actually use the cross-compiled extension. That means they should be built with the same configuration flags, and ideally for the same Linux distribution (yes, really[^distro]). You probably also need to try to ensure that glibc and linux kernel versions match what you use for the cross-compile toolchain in the next step (an exact match would be ideal, but it is difficult on anything other than Debian).

[^distro]: Comments on PostgreSQL's mailing list [indicate](https://www.postgresql.org/message-id/1556.1486012839%40sss.pgh.pa.us) that they consider it fine if there are ABI differences between extensions built on (for example) Debian versus RHEL. In reality this seems unlikely as long as things like kernel and glibc versions are compatible (but don't blame me if that isn't true), but either way, it's an indication that you should go out of your way to ensure *everything* on the build machine and cross-compilation environment is *as similar as possible* to what it will be in the location where the extension will ultimately get installed.

One additional warning is that while currently `pgrx` doesn't emit bindings for headers which include (transitively or otherwise) headers provided by third-party dependencies, those do exist (PostgreSQL has headers which transitively include in headers from ICU, and from LLVM's C API). At the moment we don't need them, but future versions of PG could change that, so be aware. If those headers become necessary, it's very likely that the failure would be at compile time rather than runtime, so this is *mostly* something that doesn't need to be handled yet (but might be necessary when dealing with patched versions of postgres).

### Where to get the headers
[get-headers-options]: #where-to-get-the-headers

Anyway, there are a few ways you can go about getting these headers. This is not exhaustive, although some alternatives have good reasons for being left out (for example, attempting to cross-compile PostgreSQL itself to get headers is only a good idea if you're actually going to use that cross-compiled PostgreSQL):

1. If you built the version of PostgreSQL you're planning on using for the cross-compile target yourself (or a coworker you have access to did it), then use the headers from that build. They should be

    While you're at it, mark down the versions of glibc and the Linux kernel that they were built against, as that will be useful for figuring out the cross-compilation toolchain.

2. A simple approach that is unlikely to go wrong is to copy them off of an actual machine of that architecture running the same Postgres configuration you care about. This is a good option for cases like using [PL/Rust](https://github.com/tcdi/plrust)'s cross-compilation and replication support simultaneously, but it may not always be possible.

    Concretely, you want to use the headers from the folder displayed when you run `pg_config --includedir-server`. Make sure to keep track of the version output by `pg_config --version` (and make sure it matches what you expect).

3. If you know that you're planning on using Postgres installed from a package repository, you can grab the headers from there. This is a good option if you're cross-compiling an extension on the same version of the same Linux distribution, and intend to use the same package repositories to install Postgres in both cases.

    For example, on Debian something like `apt download postgresql-server-dev-$VERSION:arm64` (note that Debian uses `arm64` for `aarch64` targets, and `amd64` for `x86_64` targets) will produce a `postgresql-server-dev-$VERSION_..._arm64.deb` file in the current working directory. `dpkg-deb -x` extracts it, and the headers are under `usr/include/postgresql/$VERSION/server`. This requires that your distribution package binaries your chosen target architecture (`arm64` in the example above), which is not guaranteed (while Debian itself is quite solid about providing binaries for a several architectures, this is not true for every apt-based distribution). [The Debian steps](#the-steps) fetch the package from the PostgreSQL apt repository (PGDG) instead, which needs no foreign architecture configured in apt.

    Similarly, the incantation on Fedora is something like `dnf download --forcearch aarch64 postgresql-server-devel`, which puts an a `.rpm` in the current working directory. RPM is somewhat more involved to extract (use `rpm2cpio` and `cpio` in conjunction), but entirely possible. Be very aware about the Postgres version these headers come from, since Fedora (for example) does not seem to have packages for multiple versions of PostgreSQL.

And so on, that should give you ideas.

## Getting a cross-compilation toolchain
[get-toolchain]: #getting-a-cross-compilation-toolchain

First off, if you're on Debian (and probably other `apt`-using distributions) this should be pretty easy -- just `sudo apt install crossbuild-essential-arm64` if you want to cross-compile *to* aarch64 (*from* x86_64), and `sudo apt install crossbuild-essential-amd64` if you want to cross-compile *to* x86_64 (*from* aarch64). If that's you, you're done with this step, and can move on to the next section.

Otherwise, you're going to have to do this the hard way, which means providing a cross-compile toolchain. Unfortunately, there are some extra wrinkles, as we need bindings generated to be as identical as possible to the the ones that will be generated in non-cross settings.

In particular, this means you should try to ensure that the version of glibc and the linux headers for the cross-compilation toolchain are... reasonably close to reality.

FIXME finish rewriting the rest of this

# Distributions

Unfortunately, the cross-compilation process is quite distribution specific. We'll cover two cases:

1. Debian-based distributions, where this is not that bad.
2. Distributions where userspace cross-compilation is not directly supported (such as the Fedora-family). This is much more difficult, so if you have a choice you should not go this route.

## Debian

Of the mainstream distributions (that is, excluding things like NixOS which apparently are designed to make this easy) the easiest path available is likely to be on Debian-family systems. This is for two reasons:

1. The cross compilation tools can be installed via an easy package like `crossbuild-essential-arm64` (when targeting `aarch64`) or `crossbuild-essential-amd64` (when targeting `x86_64`)

2. The cross compilation sysroot is the same as the normal sysroot -- they're both `/`.

3. Many tools in the Rust ecosystem (the `bindgen` and `cc` crates) know where many things are located, out of the box.

And a few other aspects which are less critical (if you get the tools on Debian 11, then you know they'll run on any machine has a Debian 11 install -- no need to worry about glibc versions, for example).

### The Steps

On the steps on Debian-family are as follows:

1. Set up everything you'd need to perform non-cross builds, including Clang, LLVM and `libclang` from one LLVM release: `sudo apt install clang llvm libclang-dev`.

2. Install a Rust toolchain for the target:
    - *`target=aarch64`*: `rustup target add aarch64-unknown-linux-gnu`.
    - *`target=x86_64`*: `rustup target add x86_64-unknown-linux-gnu`.

3. Install the `crossbuild-essential-<arch>` package for the architecture you are targeting
    - *`target=aarch64`*: `sudo apt install crossbuild-essential-arm64`.
    - *`target=x86_64`*: `sudo apt install crossbuild-essential-amd64`.

4. Install the host's PostgreSQL in the version your servers run, from the [PostgreSQL apt repository (PGDG)](https://wiki.postgresql.org/wiki/Apt), and select its `pg_config`:

    ```sh
    sudo apt install postgresql-18 postgresql-server-dev-18
    cargo pgrx init --pg18 /usr/lib/postgresql/18/bin/pg_config
    ```

    Setting `PGRX_PG_CONFIG_PATH=/usr/lib/postgresql/18/bin/pg_config` instead of running `cargo pgrx init` also works.

5. Fetch the target's `postgresql-server-dev` package from PGDG and extract it. This reads PGDG's package index for the target architecture directly, so apt needs no foreign architecture. Use the package version of the server that will load the extension; this example uses the version installed on the build host:

    ```sh
    pg=18
    arch=arm64    # Debian's name for aarch64; use amd64 for x86_64
    version="$(dpkg-query -W -f='${Version}' "postgresql-server-dev-$pg")"
    repo=https://apt.postgresql.org/pub/repos/apt
    index="$repo/dists/$(. /etc/os-release && echo "$VERSION_CODENAME")-pgdg/main/binary-$arch/Packages.gz"
    deb="$(curl -fsSL "$index" | gunzip | awk -v p="postgresql-server-dev-$pg" -v v="$version" '
        $1 == "Package:" { name = $2 }
        $1 == "Version:" { ver = $2 }
        !found && $1 == "Filename:" && name == p && ver == v { print $2; found = 1 }')"
    if [ -n "$deb" ]; then
        curl -fsSL -o "/tmp/postgresql-server-dev-$pg-$arch.deb" "$repo/$deb" &&
            dpkg-deb -x "/tmp/postgresql-server-dev-$pg-$arch.deb" "$HOME/pg-$arch"
    else
        echo "$index lists no postgresql-server-dev-$pg $version"
    fi
    ```

    awk reads the whole index rather than stopping at the match, so `curl` and `gunzip` finish writing and the pipeline also succeeds under `set -o pipefail`. The headers are then in `$HOME/pg-arm64/usr/include/postgresql/18/server`. The index lists only current versions; PGDG keeps older ones in its archive repository, `apt-archive.postgresql.org`. Beta versions are published in a component named after the major version, the `19` in `deb https://apt.postgresql.org/pub/repos/apt noble-pgdg main 19`, so replace `main` in the index URL with that component.

6. Set some relevant environment vars:
    - *`target=aarch64`*: `export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc`.
    - *`target=x86_64 `*: `export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc`.

    Note: It's also possible to set these in your `.cargo/config.toml`, but note that they're distribution-specific (and on other distros, potentially machine-specific), so I would recommend against checking them into version control.
    ```toml
    # Replace `<arch>` with the target arch.
    [target.<arch>-unknown-linux-gnu]
    linker = "<arch>-linux-gnu-gcc"
    ```

    Point Clang at the target's C library headers. `-isystem` adds the headers that `crossbuild-essential-<arch>` installs, and `-ccc-gcc-name` names the cross GCC for the Clang driver. pgrx passes these arguments to bindgen, header inspection, include path detection and native compilation:
    - *`target=aarch64`*: `export BINDGEN_EXTRA_CLANG_ARGS_aarch64_unknown_linux_gnu="--target=aarch64-unknown-linux-gnu -isystem /usr/aarch64-linux-gnu/include/ -ccc-gcc-name aarch64-linux-gnu-gcc"`.
    - *`target=x86_64`*: `export BINDGEN_EXTRA_CLANG_ARGS_x86_64_unknown_linux_gnu="--target=x86_64-unknown-linux-gnu -isystem /usr/x86_64-linux-gnu/include/ -ccc-gcc-name x86_64-linux-gnu-gcc"`.

    Select the target's server headers from step 5:
    - *`target=aarch64`*: `export PGRX_INCLUDEDIR_SERVER_PG18_aarch64_unknown_linux_gnu="$HOME/pg-arm64/usr/include/postgresql/18/server"`.
    - *`target=x86_64`*: `export PGRX_INCLUDEDIR_SERVER_PG18_x86_64_unknown_linux_gnu="$HOME/pg-amd64/usr/include/postgresql/18/server"`.

7. Build your extension.
    - *`target=aarch64`*: `cargo build --target=aarch64-unknown-linux-gnu --release`.
    - *`target=x86_64 `*: `cargo build --target=x86_64-unknown-linux-gnu --release`.

This will produce a `.so` in `./target/<target>/release/lib$yourext.so`, which you can use.

If the build stops because the headers do not describe the target, check that `PGRX_INCLUDEDIR_SERVER_PG18_<target>` names the extracted headers and that they came from the target architecture's package.

# Other Distributions

*Note: these steps are still somewhat experimental, and may be missing some pieces.*

The first few steps are the same as under debian.

1. Set up everything you'd need to perform non-cross builds.

2. Install a Rust toolchain for the target:
    - *`target=aarch64`*: `rustup target add aarch64-unknown-linux-gnu`.
    - *`target=x86_64`*: `rustup target add x86_64-unknown-linux-gnu`.

After this you need a cross compilation toolchain, which can be challenging.

## Get a cross-compilation toolchain

To cross compile, you need a toolchain. This is basically two parts:

- The cross-compile tools: suite of tools, scripts, libraries (and the like) which are compiled for the host architecture (so that you can run them) which are capable of building for the target. Specifically, this includes things like the compiler, linker, assembler, archiver, ... as well as any libraries they link to, and tools they invoke.

- A sysroot, which is basically an emulated unix filesystem root, with `<sysroot>/usr`, `<sysroot>/etc` and so-on. The important thing here is that it has libraries built for the target, headers configured for the target, and so on.

Pick well here, since getting a bad one may cause builds that succeed but fail at runtime.

An easy option for targeting `aarch64` (or several other architectures) from `x86_64` is to use one of the ones on <https://toolchains.bootlin.com/releases_aarch64.html> (not an endorsement: they're something I've used for development, I don't know how well-made they are, and they honestly seem kind of idiosyncratic. IOW, I'd want to do a lot more research before putting them into production).

Sadly, I don't have a good option for an easily downloaded x86_64 toolchain that has tools built for aarch64. I've been using a manually built one, which isn't covered in this guide (TODO?).

So, assuming you're getting one from the link above, you need to find a toolchain:
- Which uses glibc (not musl/uclibc), since we're compiling to a `-linux-gnu` Rust target.
- Contains a version of glibc which is:
    - Has a version of glibc which is no newer than the ones which will be used on any of the target machines.
    - But still new enough to contain any symbols you link to non-weakly (don't worry about this part unless it becomes a problem).
- Has linux headers which are similarly no newer than the version on any of the target machines. This *probably* doesn't matter for you, and it might not be a thing you have to worry about with toolchains from elsewhere (most don't contain kernel headers, since most applications don't need them to build).

If you can't find one with an old enough version of glibc, try to get as close as possible, and we'll just have to hope for the best -- there's a chance it will work still, it depends on what system functions you call in the compiled binary.

Anyway, once you have one of these you may need to put it somewhere specific -- not all of them are relocatable (the ones from `toolchains.bootlin.com` above are, however). An easy way to do this is by running `bin/aarch64-linux-gcc --print-sysroot`, and seeing if it prints the a path inside it's directory. If not, you may have to move things around so that the answer it gives is correct.

## Use the cross compilation toolchain

Continuing from above, I will assume (without loss of generality) that you're targeting aarch64, have a toolchain directory at `$toolchain_dir` and your sysroot is at `$sysroot_dir` -- try `$toolchain_dir/bin/aarch64-linux-gnu-gcc --print-sysroot`.

Anyway, set

- `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=$toolchain_dir/bin/aarch64-linux-gnu-gcc`
- `BINDGEN_EXTRA_CLANG_ARGS_aarch64-unknown-linux-gnu=--sysroot=\"$sysroot_dir\"`
- `PGRX_INCLUDEDIR_SERVER_PG18_aarch64-unknown-linux-gnu` to the target's server headers, for example ones extracted from the target's `postgresql-server-devel` package (see [Getting PostgreSQL headers for the target](#getting-postgresql-headers-for-the-target))

Shells can export these with underscores in the target name instead, such as `BINDGEN_EXTRA_CLANG_ARGS_aarch64_unknown_linux_gnu`. The header override goes with a host `pg_config` of the same PostgreSQL major, from `cargo pgrx init` or `PGRX_PG_CONFIG_PATH`; alternatively, describe the target as in [PostgreSQL metadata in the environment](#postgresql-metadata-in-the-environment).

The toolchain supplies the linker and sysroot. `pgrx-pg-sys` also needs Clang, `llvm-ar` and `libclang` from one LLVM release, installed on the build host: it inspects the headers and compiles its native support archive, including the C shim when the `cshim` feature is enabled, with that Clang. pgrx adds `--target=aarch64-unknown-linux-gnu`, and the `--sysroot` above makes Clang use the toolchain's C library headers. See [Clang, LLVM and system headers](#clang-llvm-and-system-headers).

Other crates in your build that compile C or C++ with the `cc` crate may also need these; `pgrx-pg-sys`'s native support does not use them:
- `CC_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-gcc`
- `AR_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-ar`
- `CXX_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-g++`
- `LD_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-ld`

For those crates, adding `$toolchain_dir/bin` to `PATH` and setting `CROSS_COMPILE=aarch64-linux-gnu-` can sometimes help. Sadly, this can break things depending on what's in your toolchain's path, or if you have a tool which doesn't check if it's actually a cross-compile before looking at the `CROSS_COMPILE` var.
