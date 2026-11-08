# Packaging a PGRX Extension

`cargo pgrx install` puts an extension into the Postgres on your own machine. To ship it to other
machines, use `cargo pgrx package`. It builds the extension and stages its files in a directory
without touching any Postgres installation or starting a server. On Linux, it can also turn those
files into RPM and DEB packages.

A compiled extension belongs to one Postgres major version, so you build one package per major
version you support.

## Building a Package Directory

Run this from your extension's directory:

```console
$ cargo pgrx package
```

This builds in release mode. The `pg_config` on your `$PATH` selects the Postgres version and the
install paths, and `--pg-config` selects a different one. The files land in
`target/release/<extname>-pg<major>/`. That directory mirrors the paths `pg_config` reports, so it
starts at the root of the filesystem. Here is `my_extension` built against Debian's Postgres 17:

```text
target/release/my_extension-pg17/
└── usr
    ├── lib/postgresql/17/lib/
    │   └── my_extension.so
    └── share/postgresql/17/extension/
        ├── my_extension--1.0.0.sql
        └── my_extension.control
```

Because the layout starts at the root, a tarball of it unpacks directly onto a matching server:

```console
$ tar -C target/release/my_extension-pg17 -czf my_extension-pg17.tar.gz usr
$ sudo tar -C / -xzf my_extension-pg17.tar.gz
```

These options change what gets built and where it goes:

| Option                                 | Effect                                                     |
|----------------------------------------|------------------------------------------------------------|
| `-c`, `--pg-config <PATH>`             | Build for the Postgres that this `pg_config` belongs to    |
| `--out-dir <DIR>`                      | Stage the files in this directory                          |
| `--prefix-dir <DIR>`                   | Put all the files together under this path in the output   |
| `-d`, `--debug`                        | Build with the development profile                         |
| `--profile <NAME>`                     | Build with this Cargo profile                              |
| `-F`, `--features <FEATURES>`          | Enable additional Cargo features                           |
| `--target <TRIPLE>`                    | Compile for this target                                    |
| `-p`, `--package`, `--manifest-path`   | Select an extension in a workspace                         |

Run `cargo pgrx package --help` for the full list.

## Building for Several Postgres Versions

Register each Postgres with `cargo pgrx init`, then run `cargo pgrx package` once per version. Each
run has its own output directory:

```console
$ cargo pgrx init --pg16 /usr/lib/postgresql/16/bin/pg_config \
                  --pg17 /usr/lib/postgresql/17/bin/pg_config
$ cargo pgrx package --pg-config /usr/lib/postgresql/16/bin/pg_config
$ cargo pgrx package --pg-config /usr/lib/postgresql/17/bin/pg_config
```

## Choosing Where to Build

A package only runs where its library can load. The library links against the glibc of the machine
that built it, and it is compiled against the headers of the Postgres that `pg_config` belongs to.
Build on the oldest distribution release you support, or on each one, against the same Postgres
packages your users run. A container for each distribution works well for this. To build for
another architecture, see [Cross Compiling](./build/cross-compile.md).

## Building RPM and DEB Packages

To have `cargo pgrx package` build a `.rpm` or `.deb` as well, add the matching section to your
extension's `Cargo.toml`. An empty section uses the defaults:

```toml
[package]
name = "my_extension"
version = "1.0.0"
description = "My Great Extension"
license = "MIT"
authors = ["Jane Doe <jane@example.com>"]

[package.metadata.rpm]

[package.metadata.deb]
```

The defaults take the package's description, license, and maintainer from `[package]`, so
`cargo pgrx package` reports an error if one of those fields is missing and your section doesn't set
it. Package files are written to the output directory, next to the staged files. You only need the
sections for the formats you want.

The defaults follow the PGDG packages for RPM, and the Debian and PGDG packages for DEB. For the
example above and Postgres 17 on x86-64, you get:

|                    | RPM                                  | DEB                                          |
|--------------------|--------------------------------------|----------------------------------------------|
| Package file       | `my_extension_17-1.0.0-1.x86_64.rpm` | `postgresql-17-my-extension_1.0.0_amd64.deb` |
| Package name       | `my_extension_17`                    | `postgresql-17-my-extension`                 |
| Depends on         | `postgresql17-server`                | `postgresql-17`                              |
| Shared library in  | `/usr/pgsql-17/lib/`                 | `/usr/lib/postgresql/17/lib/`                |
| SQL and control in | `/usr/pgsql-17/share/extension/`     | `/usr/share/postgresql/17/extension/`        |

The install location in the package comes from these defaults, not from the `pg_config` you built
with. Install the package with your distribution's tools, then create the extension as usual:

```console
$ sudo dnf install ./target/release/my_extension-pg17/my_extension_17-1.0.0-1.x86_64.rpm
$ sudo apt install ./target/release/my_extension-pg17/postgresql-17-my-extension_1.0.0_amd64.deb
```

```sql
CREATE EXTENSION my_extension;
```

If your extension needs [`shared_preload_libraries`](./install.md#when-shared_preload_libraries-is-required),
the package does not set it. Whoever installs the package still has to configure it.

### Changing the Defaults

Any key you set in a section replaces that default as a whole, including lists such as `depends` and
`files`. This RPM adds a dependency and installs into Fedora's Postgres layout instead of PGDG's:

```toml
[package.metadata.rpm]
depends = ["postgresql-server", "postgis"]
files = [
  { source = "${BUILD_BASE_PATH}/${PG_PKGLIBDIR}/${LIB_FILE}", dest = "/usr/lib64/pgsql/${LIB_FILE}", mode = "0755" },
]
dirs = [
  { source = "${BUILD_BASE_PATH}/${PG_SHAREDIR}/extension/", dest = "/usr/share/pgsql/extension/" },
]
```

`files` lists single files, and each one needs a `source`, a `dest`, and a `mode`. `dirs` copies a
directory's contents. Packages are built with the [`arx-pack`] crate, and its [`Manifest`] documents
every key a section accepts, including `conflicts`, `provides`, `replaces`, and maintainer scripts.

A relative `source`, or a relative path to a maintainer script, is read from the crate's directory,
the one that holds its `Cargo.toml`, wherever you run `cargo pgrx package`. This DEB also ships the
workspace's license:

```toml
[package.metadata.deb]
files = [
  { source = "${BUILD_BASE_PATH}/${PG_PKGLIBDIR}/${LIB_FILE}", dest = "/usr/lib/postgresql/${PG_MAJOR_VERSION}/lib/${LIB_FILE}", mode = "0644" },
  { source = "${WORKSPACE_ROOT}/LICENSE", dest = "/usr/share/doc/postgresql-${PG_MAJOR_VERSION}-${DEB_NAME}/LICENSE", mode = "0644" },
]
```

[`arx-pack`]: https://crates.io/crates/arx-pack
[`Manifest`]: https://docs.rs/arx-pack/latest/arx_pack/struct.Manifest.html

String values can use these variables. They are not environment variables, and an unknown name is
an error.

| Variable              | Value                                                          |
|-----------------------|----------------------------------------------------------------|
| `${NAME}`             | The crate name                                                 |
| `${DEB_NAME}`         | The crate name in lowercase, with `_` replaced by `-`          |
| `${LIB_FILE}`         | The file name of the extension's shared library                |
| `${VERSION}`          | The crate version, with each `-` replaced by `~`               |
| `${DESCRIPTION}`      | The crate description                                          |
| `${LICENSE}`          | The crate license                                              |
| `${MAINTAINER}`       | The first entry of the crate's `authors`                       |
| `${ARCH}`             | The `--target` architecture, or the host's if there isn't one  |
| `${PG_MAJOR_VERSION}` | The Postgres major version, such as `17`                       |
| `${PG_PKGLIBDIR}`     | `pg_config --pkglibdir`                                        |
| `${PG_SHAREDIR}`      | `pg_config --sharedir`                                         |
| `${BUILD_BASE_PATH}`  | The output directory that holds the staged files               |
| `${MANIFEST_DIR}`     | The directory that holds the crate's `Cargo.toml`              |
| `${WORKSPACE_ROOT}`   | The root directory of the crate's Cargo workspace              |

`${BUILD_BASE_PATH}`, `${MANIFEST_DIR}` and `${WORKSPACE_ROOT}` are absolute paths.

Use `${LIB_FILE}` for the library rather than building its name from `${NAME}`. The two differ for a
crate with a hyphenated name or for a versioned shared library.

`${VERSION}` replaces `-` so that pre-releases work. RPM rejects `-` in a version, and Debian would
treat `1.0.0-rc.1` as newer than `1.0.0`. Both sort `1.0.0~rc.1` before `1.0.0`.

### Things to Know

- RPM and DEB packages can only be built on Linux. On other platforms, `cargo pgrx package` prints a
  warning, skips them, and still produces the package directory.
- `cargo pgrx install --sudo` stages its files the same way but never builds these packages.
- `--no-distrib-packages` skips these packages for one build, such as a container build that only
  copies the staged files.
- `cargo pgrx package` does not clear the output directory between runs, and `dirs` packages every
  file it finds. Delete the directory before a release so SQL files from older versions don't ship.
- The default `files` and `dirs` expect the layout from `pg_config`. If you pass `--prefix-dir`,
  override both keys with paths under `${BUILD_BASE_PATH}`.
- `${ARCH}` is the first part of a `--target` triple. If the package format doesn't recognize it, as
  with `powerpc64le`, set `arch` in your section.
- Packages are unsigned, and the RPM release number is always `1`.
- `cargo deb` reads `[package.metadata.deb]` too, with different rules, so a section written for it
  can stop `cargo pgrx package` with an error. For example, `depends = "$auto"` is rejected here.

For a complete extension, see the [packaging example].

[packaging example]: https://github.com/pgcentralfoundation/pgrx/tree/develop/pgrx-examples/packaging
