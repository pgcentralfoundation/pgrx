# cargo pgrx package

Builds an extension and stages its control file, SQL, and shared library in a
directory for distribution. It defaults to a release build and does not start
PostgreSQL or install files into the server's live installation.

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
