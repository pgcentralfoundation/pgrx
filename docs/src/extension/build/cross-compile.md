# Cross compiling `pgrx` extensions

Cross compiling builds an extension on one machine, the build host, for another
platform, the target. pgrx does this from a *target bundle*: one file, generated
on the target platform, that holds everything `cargo pgrx package` needs to
build for that platform's PostgreSQL from anywhere else.

1. On the target platform, in your extension's directory:

    ```sh
    cargo pgrx package --bundle-generate
    ```

    This writes `pgrx-pg18-aarch64-unknown-linux-gnu.tar.gz`, named for the
    PostgreSQL major version and the platform, and packages nothing.

2. Copy that file to the build host.

3. On the build host, with a Rust toolchain that can link for the target:

    ```sh
    cargo pgrx package --bundle pgrx-pg18-aarch64-unknown-linux-gnu.tar.gz
    ```

`cargo pgrx package` is the only command that builds for another platform, and a
bundle is the only way to tell it which one.

If the target platform has PostgreSQL's server headers but no Rust or Clang, or
its headers exist only in a sysroot, a shell script collects a
[partial bundle](#collect-a-partial-bundle) from them instead, and the build
host does the generating.

| | Made with | The build host also needs |
| --- | --- | --- |
| Full bundle | Rust, Clang and PostgreSQL's server headers, on the target | a linker for the target |
| Partial bundle | `tar`, the target's server headers, and what its `pg_config` reports | a linker, Clang, and the target's system headers |

## Why the bundle comes from the target

`pgrx-pg-sys` generates its Rust bindings and C macros from PostgreSQL's server
headers, and compiles a small native support archive. All three depend on what
PostgreSQL's own build recorded for one machine: `pg_config.h` holds configure's
results, and other headers choose declaration shapes from them. PostgreSQL 15 to
18, built for x86_64 with a run-time check for the POPCNT instruction, declare
`pg_popcount32` as a pointer to a function, while an aarch64 server exports a
function of that name. Bindings generated from the wrong headers would load that
function's first instructions as a pointer and call the result.

Generating on the target uses that PostgreSQL's real `pg_config`, its real
headers, the platform's own system headers and a native compile, so nothing is
copied by hand or guessed. What remains for you to ensure is that the
PostgreSQL there is the same build as the server that will load the extension:
the same packages from the same distribution, or the same source build. pgrx
cannot detect a different minor version or different configure options.

## Generate a bundle

The target platform needs what any pgrx build needs: Rust, Clang and `libclang`,
and a PostgreSQL installation with its server headers.

```sh
cargo pgrx package --bundle-generate --pg-config /usr/lib/postgresql/18/bin/pg_config
```

- **PostgreSQL.** `--pg-config` selects it; without it, the first `pg_config` on
  `PATH`. Neither `cargo pgrx init` nor a pgrx configuration file is needed.
- **Output.** The file goes in Cargo's target directory, or in `--out-dir`. The
  command builds your extension to generate from its exact pgrx version and
  features, but stages no package.
- **Where to run it.** The target platform does not have to be hardware. A
  container of the target architecture with the distribution's PostgreSQL
  packages installed, or a CI runner of that platform, generates the same
  bundle. macOS and Windows targets need a machine or runner of that kind.

A bundle holds:

- `bundle.json`: the Rust target, and the version, `pkglibdir` and `sharedir`
  that `pg_config` printed;
- `target-info/`: the raw bindings, the generated C macro tree, the compiler
  profile report, and the native support archive with its sources, listed in a
  manifest with a SHA-256 and length for every file;
- `lib/postgres.lib`, for Windows MSVC targets, whose extensions link against it.

A bundle does not depend on your extension. It is valid for one Rust target,
one PostgreSQL major version, one pgrx version and one setting of the `cshim`
feature, so generate it again when any of those change. Development builds of
pgrx must also be the same source revision. Anyone packaging for the same
PostgreSQL can share one, but it contains compiled code that is linked into the
extension, so take it only from a build you trust.

## Collect a partial bundle

A target with no Rust toolchain or no Clang cannot generate a full bundle, but it
still has the only things that must come from it: PostgreSQL's server headers and
what `pg_config` reports. `tools/pgrx-collect-bundle.sh`, in the pgrx repository,
collects those into a partial bundle. Take the script from the tag of the pgrx
release your extension uses, and copy it to the target.

```sh
sh pgrx-collect-bundle.sh --target=aarch64-unknown-linux-gnu
```

The script needs `tar` and a POSIX shell; on Windows, run it in Git Bash. It is
short enough to read before you run it. Every option is written `--name=value`.

- **`--target=`.** The Rust target triple of that platform. A machine without Rust
  cannot name its own, so you state it; the build that uses the bundle checks the
  headers against it, as described under [What pgrx verifies](#what-pgrx-verifies).
- **`--out-dir=`.** Where to write the file; without it, the current directory.

The other options say what `pg_config` reports. They combine, and each one in
this list overrides the ones before it:

- **`--pg-config=`.** A `pg_config` program to run. Given neither of the next
  two, the script runs the first `pg_config` on `PATH`.
- **`--pg-config-file=`.** A file holding what `pg_config` prints when run with
  no arguments: a `NAME = value` line for each property.
- **`--<property>=`.** One property, named as `pg_config`'s own option is, such
  as `--pkglibdir=/usr/lib/postgresql/18/lib`.

The script copies the server headers from the directory that `INCLUDEDIR-SERVER`
names. When they are somewhere else on the machine running the script, pass
`--includedir-server=` with that directory.

It writes `pgrx-pg18-aarch64-unknown-linux-gnu.partial.tar.gz` and prints the
path. The file holds the Rust target, the `pg_config` lines, the server headers,
and for Windows MSVC targets the public headers and `postgres.lib`.

The script copies what it is given and checks none of it.
`cargo pgrx package --bundle` checks the bundle when it uses it, and names a
value that is missing. It cannot tell that a value is wrong for the target: a
mistaken `--pkglibdir=` produces a package laid out for a directory the server
does not load from.

### When no `pg_config` can run

A sysroot from Yocto or Buildroot holds the target's server headers, but its
`pg_config` is built for the target and does not run on the machine that holds
the sysroot. Run the script there and state the properties:

```sh
sh pgrx-collect-bundle.sh \
    --target=aarch64-unknown-linux-gnu \
    --version='PostgreSQL 18.1' \
    --includedir-server="$SYSROOT/usr/include/postgresql/server" \
    --pkglibdir=/usr/lib/postgresql \
    --sharedir=/usr/share/postgresql \
    --cppflags=-D_GNU_SOURCE \
    --cflags='-O2 -fwrapv -fno-strict-aliasing'
```

| Property | What to give |
| --- | --- |
| `--version=` | `PostgreSQL` and the version, as `pg_config --version` prints it |
| `--includedir-server=` | the directory holding `pg_config.h`, on the machine running the script |
| `--pkglibdir=` | where the target's server loads extension libraries from, as a path on the target |
| `--sharedir=` | the directory on the target whose `extension` directory holds control and SQL files |
| `--cppflags=` | the preprocessor flags PostgreSQL was built with; give `--cppflags=` if there were none |
| `--cflags=` | the compiler flags PostgreSQL was built with; optional |
| `--pkgincludedir=`, `--libdir=` | for Windows MSVC targets, the public headers and the directory holding `postgres.lib`, on the machine running the script |

If the target can run `pg_config` but the script has to run elsewhere, save
`pg_config`'s output on the target and pass the file, with the headers'
directory on this machine:

```sh
sh pgrx-collect-bundle.sh \
    --target=aarch64-unknown-linux-gnu \
    --pg-config-file=pg_config.txt \
    --includedir-server="$SYSROOT/usr/include/postgresql/server"
```

### Use a partial bundle

Nothing in a partial bundle is generated, so it does not depend on the pgrx
version or the `cshim` feature. Collect it once for a PostgreSQL installation and
keep using it across pgrx upgrades.

The build host generates what a full bundle would have carried, so it needs the
Clang toolchain described under
[Clang, LLVM and system headers](#clang-llvm-and-system-headers) and the target's
system headers, which usually come with the cross toolchain's sysroot. Point
Clang at them with `BINDGEN_EXTRA_CLANG_ARGS_<target>`, for example
`--sysroot=/path/to/sysroot`. Those system headers are the one input that does
not come from the target: they must be for the same C library and architecture.
pgrx refuses headers whose recorded sizes and alignments disagree with the
compiler, but it cannot prove the two C libraries are the same.

## Package from a bundle

The build host needs the Rust standard library for the target and a linker for
it, as described under [Cross toolchains](#cross-toolchains). It needs no
PostgreSQL, no `pg_config`, and no `cargo pgrx init`. With a full bundle it
needs no server headers and no Clang either.

```sh
rustup target add aarch64-unknown-linux-gnu
cargo pgrx package --bundle pgrx-pg18-aarch64-unknown-linux-gnu.tar.gz
```

`--bundle` takes either kind of bundle. The bundle supplies the target and its PostgreSQL, so `--bundle` cannot be
combined with `--pg-config`. The extension is staged for the directories the
target's `pg_config` reported. Windows packages always stage into `lib` and
`share/extension`, and Linux targets can also produce
[RPM and DEB packages](../../../../cargo-pgrx/README.md#building-rpm-and-deb-packages).
cargo-pgrx reads the generated SQL out of the built library without loading it,
so no target code runs on the build host.

The importer validates all members and the compiler target before publishing
Rust source or linking the bundled native archive. It rejects missing manifests,
raw-only imports, changed bytes, extra files, mismatched targets/versions/features,
and symlinks at the bundle root or within the bundle. Configured ancestor path
aliases are resolved normally. Members are limited to 128 MiB each, 512 MiB in
total and 8192 files/directory entries; manifests are limited to 4 MiB. Export
replaces only an intact bundle of the same domain or an empty directory, preserving
unrelated files on refusal.
A bundle from another pgrx version, PostgreSQL major or `cshim` setting is
refused the same way.

## What pgrx verifies

Every build that generates from PostgreSQL's headers, including the one that
makes a bundle, first compares the server headers with the Clang target and
refuses headers that do not describe it:

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
header directory and the metadata source, and says what to do:

```text
these server headers do not describe aarch64-unknown-linux-gnu; use the `pg_config` of a PostgreSQL 18 built for aarch64-unknown-linux-gnu, or to package for another platform, generate a bundle there with `cargo pgrx package --bundle-generate` and pass it to `cargo pgrx package --bundle`
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
- build options such as `BLCKSZ`, `NAMEDATALEN` and `USE_ASSERT_CHECKING`.

Generate the bundle from the same PostgreSQL build as the server that will load
the extension.

## Clang, LLVM and system headers

This applies wherever `pgrx-pg-sys` generates from headers. For a cross build
that is the machine that makes a full bundle, or the build host that uses a
partial one. A build from a full bundle runs no Clang.

The `pgrx-pg-sys` build script runs Clang itself to find system headers, for
header inspection and for its native support archive, rather than through the
`cc` crate.

- **Driver.** Its major version must match `libclang`. pgrx uses `CLANG_PATH`
  when it is set; otherwise it uses the first match among the `CLANG` recorded
  by a `pg_config` program's `--configure` output, the `clang` installed with
  `libclang`, and `clang-<major>` or `clang` on `PATH`.
- **Configuration files.** Clang 16 and later load default configuration
  files: `<triple>-clang.cfg` for the target or, without one, `<triple>.cfg`
  together with `clang.cfg`, which applies to every target. Some distributions
  install them; Gentoo, for example, installs `clang.cfg`. Their
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

# Cross toolchains

Rust needs the target's standard library and a linker that can produce the
target's libraries. This is ordinary Rust cross compiling; pgrx adds nothing to
it. Other crates in your build that compile C or C++ may need more.

## Debian

Debian-family systems make Linux-to-Linux cross builds easy: the cross tools
install from one package, and the cross sysroot is the normal root.

1. Install a Rust toolchain for the target:
    - *`target=aarch64`*: `rustup target add aarch64-unknown-linux-gnu`.
    - *`target=x86_64`*: `rustup target add x86_64-unknown-linux-gnu`.

2. Install the `crossbuild-essential-<arch>` package for the architecture you are targeting:
    - *`target=aarch64`*: `sudo apt install crossbuild-essential-arm64`.
    - *`target=x86_64`*: `sudo apt install crossbuild-essential-amd64`.

3. Tell Cargo which linker to use:
    - *`target=aarch64`*: `export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc`.
    - *`target=x86_64 `*: `export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-linux-gnu-gcc`.

    It's also possible to set these in your `.cargo/config.toml`, but they're distribution-specific (and on other distros, potentially machine-specific), so I would recommend against checking them into version control.
    ```toml
    # Replace `<arch>` with the target arch.
    [target.<arch>-unknown-linux-gnu]
    linker = "<arch>-linux-gnu-gcc"
    ```

4. Package your extension with the bundle for that target:

    ```sh
    cargo pgrx package --bundle pgrx-pg18-aarch64-unknown-linux-gnu.tar.gz
    ```

This stages the extension's library, control file and SQL under `./target/<target>/release/<extension>-pg18/`, laid out for the target's directories.

With a partial bundle, also install Clang and point it at the target's C library headers before step 4. `-isystem` adds the headers that `crossbuild-essential-<arch>` installs, and `-ccc-gcc-name` names the cross GCC for the Clang driver:

- `sudo apt install clang llvm libclang-dev`
- *`target=aarch64`*: `export BINDGEN_EXTRA_CLANG_ARGS_aarch64_unknown_linux_gnu="-isystem /usr/aarch64-linux-gnu/include/ -ccc-gcc-name aarch64-linux-gnu-gcc"`.
- *`target=x86_64`*: `export BINDGEN_EXTRA_CLANG_ARGS_x86_64_unknown_linux_gnu="-isystem /usr/x86_64-linux-gnu/include/ -ccc-gcc-name x86_64-linux-gnu-gcc"`.

## Other distributions

*Note: these steps are still somewhat experimental, and may be missing some pieces.*

Install a Rust toolchain for the target, as under Debian. After this you need a cross compilation toolchain, which can be challenging.

### Get a cross-compilation toolchain

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

### Use the cross compilation toolchain

Continuing from above, I will assume (without loss of generality) that you're targeting aarch64 and have a toolchain directory at `$toolchain_dir`. Set

- `CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=$toolchain_dir/bin/aarch64-linux-gnu-gcc`

and package with the bundle for that target, as in step 4 above.

Other crates in your build that compile C or C++ with the `cc` crate may also need these; `pgrx-pg-sys` does not use them when it builds from a bundle:
- `CC_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-gcc`
- `AR_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-ar`
- `CXX_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-g++`
- `LD_aarch64_unknown_linux_gnu=$toolchain_dir/bin/aarch64-linux-gnu-ld`

For those crates, adding `$toolchain_dir/bin` to `PATH` and setting `CROSS_COMPILE=aarch64-linux-gnu-` can sometimes help. Sadly, this can break things depending on what's in your toolchain's path, or if you have a tool which doesn't check if it's actually a cross-compile before looking at the `CROSS_COMPILE` var.
