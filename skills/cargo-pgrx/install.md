# cargo pgrx install

Builds the extension, generates its SQL, and installs the shared library into
`pg_config --pkglibdir` and the control and SQL files into the installation's
extension directory. It does not start PostgreSQL, create a database, or execute
`CREATE EXTENSION`.

The destination is selected by `--pg-config`, falling back to `pg_config` on
`PATH`. Supply an explicit path when the destination matters. A private
`PGRX_HOME` does not redirect files written into a reused installation prefix.

## Usage and options

```text
cargo pgrx install [OPTIONS]
```

| Flag | Meaning |
|------|---------|
| `-c, --pg-config <PATH>` | Target installation and PostgreSQL version |
| `-r, --release` | Release profile; default is development |
| `--profile <P>` | Custom profile; overrides `--release` |
| `--test` | Include test support, used by the backend test harness |
| `-s, --sudo` | Use sudo for copying extension artifacts |
| `-F, --features <F>` | Additional Cargo features |
| `--no-default-features`, `--all-features` | Cargo feature selection |
| `-p, --package <PKG>` | Select a workspace extension |
| `--manifest-path <PATH>` | Select its manifest |
| `--target <TARGET>` | Cargo compilation target |
| `--cargo <FLAG>` | Repeatable Cargo flags, including for metadata |

```bash
cargo pgrx install --pg-config /usr/local/pgsql/bin/pg_config
cargo pgrx install --pg-config /usr/local/pgsql/bin/pg_config --release
cargo pgrx install --cargo=--config=./cargo-local.toml
```

Use `--sudo` only when authorized to install into the chosen prefix. Installation
can replace files used by other databases or worktrees sharing that prefix.
For staged distribution files, use [package](package.md), which also provides
`--prefix-dir`. For interactive development, use [run](run.md).

Building with `pg_bench` includes benchmark functions and their dependencies;
cargo-pgrx warns about this during install. Keep that feature out of ordinary
deployment builds unless explicitly wanted.
