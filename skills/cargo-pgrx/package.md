# cargo pgrx package

Builds an extension and stages its control file, SQL, and shared library in a
directory for distribution. It defaults to a release build and does not start
PostgreSQL or install files into the server's live installation. On Linux, it
also builds an RPM or DEB for each matching section in `Cargo.toml`.

## Usage and options

```text
cargo pgrx package [OPTIONS]
```

| Flag | Meaning |
|------|---------|
| `-c, --pg-config <PATH>` | Target installation and PostgreSQL version; default is pg_config on PATH |
| `--out-dir <DIR>` | Package staging directory |
| `--prefix-dir <DIR>` | Override artifact placement inside the staging directory on Unix |
| `-d, --debug` | Development profile instead of release |
| `--profile <P>` | Custom profile; overrides `--debug` |
| `--test` | Build with test support |
| `-F, --features <F>` | Additional Cargo features |
| `--no-default-features`, `--all-features` | Cargo feature selection |
| `-p, --package <PKG>` | Select a workspace extension |
| `--manifest-path <PATH>` | Select its manifest |
| `--target <TARGET>` | Cargo compilation target |
| `--cargo <FLAG>` | Repeatable Cargo flags, including for metadata |

Without `--out-dir`, output goes under Cargo's resolved target directory as
`[<target>/]<profile>/<extname>-pg<major>/`, with the development profile using
`debug`. This respects Cargo configuration and `CARGO_TARGET_DIR`.

## Extension artifact placement

The default Unix package layout mirrors `pg_config`'s installation paths beneath
the staging directory. `--prefix-dir` puts the control file, SQL files, and shared
library together under the supplied path inside that directory. A leading slash
is removed when composing the staging path:

```bash
cargo pgrx package --pg-config /usr/local/pgsql/bin/pg_config
cargo pgrx package --out-dir ./dist --prefix-dir /opt/my_extension
cargo pgrx package --debug --cargo=--config=./cargo-local.toml
```

The second command places the files in `./dist/opt/my_extension/`. The option
controls staging layout, not PostgreSQL's runtime search paths. On Windows,
the package uses `lib` and `share/extension` regardless of `--prefix-dir`.
Avoid enabling `pg_bench` in distribution builds unless benchmark support is
intended; cargo-pgrx warns when it is enabled for packaging.

## RPM and DEB packages

`[package.metadata.rpm]` and `[package.metadata.deb]` each request one package
for the PostgreSQL major version selected by `pg_config`. Empty sections use
defaults that follow the PGDG and Debian layouts, and `[package]` must then set
`description`, `license`, and `authors`. Package files are written beside the
staged files in the output directory.

Each key in a section replaces that default as a whole. The accepted keys are
those of [`arx_pack::Manifest`](https://docs.rs/arx-pack/latest/arx_pack/struct.Manifest.html).
String values may use the `${VAR}` variables listed in
[cargo-pgrx's README](../../cargo-pgrx/README.md#building-rpm-and-deb-packages);
an unknown variable is an error.

Check these before relying on the output:

- Use `${LIB_FILE}` for the library. `${NAME}` is the crate name, which differs
  from the installed file for hyphenated crates and versioned shared libraries.
- Non-Linux platforms print a warning and skip these packages.
  `cargo pgrx install --sudo` also stages through this command and never builds them.
- `--prefix-dir` moves the staged files, so override `files` and `dirs` when
  combining it with these sections.
- The output directory is not cleared between runs, and `dirs` packages everything
  in it. Delete it before a release build so stale SQL files do not ship.
- `${VERSION}` replaces `-` with `~`, since RPM rejects `-` and Debian would sort
  a pre-release after the final release.
- `${ARCH}` is the first part of `--target`, or the host's architecture. Set `arch`
  when the package format rejects that name.
- Packages are unsigned, and the RPM release number is always `1`.
- `cargo deb` reads `[package.metadata.deb]` too, with different rules. A section
  written for it can fail here; `depends = "$auto"` is rejected, for example.

To check a package without installing it, use `rpm -qpl FILE` or
`dpkg-deb --contents FILE`. The [packaging example](../../pgrx-examples/packaging)
is a complete extension, and [the book](../../docs/src/extension/package.md) has
a walkthrough.
