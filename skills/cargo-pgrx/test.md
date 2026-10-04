# cargo pgrx test

Runs ordinary Rust tests and backend `#[pg_test]` functions. The CLI selects a
PostgreSQL version, adjusts features, adds `pg_test`, and invokes `cargo test`.
The pgrx-tests harness builds and installs the extension, initializes a separate
test cluster, and runs the backend tests there. Plain `cargo test` can use the
same harness when the extension's test setup and features are configured.

## Usage and options

```text
cargo pgrx test [OPTIONS] [PG_VERSION] [TESTNAME]...
```

`PG_VERSION` accepts `pg15` through `pg19`, or `all`, and can come from the
environment variable of the same name. Without a selector, version resolution
uses explicit PostgreSQL features or the manifest's defaults. A first positional
argument that is not a recognized version is treated as a test filter.

Multiple filters match test names containing any supplied substring. `all` runs
versions present in both the extension's features and the active configuration.

| Flag | Meaning |
|------|---------|
| `-r, --release` | Release profile; default is development |
| `--profile <P>` | Custom profile; overrides `--release` |
| `-n, --no-schema` | Request skipping SQL generation; currently incompatible with the harness install step described below |
| `--runas <USER>` | Use sudo for the PostgreSQL test instance; unsupported on Windows |
| `--pgdata <DIR>` | Base directory for test clusters; must be writable by the runas user |
| `-F, --features <F>` | Additional Cargo features |
| `--no-default-features`, `--all-features` | Cargo feature selection; avoid incompatible PostgreSQL features |
| `-p, --package <PKG>` | Select a workspace extension |
| `--manifest-path <PATH>` | Select its manifest |
| `--cargo <FLAG>` | Repeatable Cargo flags, including for metadata; see [SKILL.md](SKILL.md#versions-features-and-cargo-flags) |

```bash
cargo pgrx test pg18
cargo pgrx test pg18 spi memory_context
cargo pgrx test spi
cargo pgrx test all --package my-extension
cargo pgrx test pg18 --release
cargo pgrx test pg18 --cargo=--config=./cargo-local.toml
```

Use ordinary `#[test]` only for code independent of the backend, including its
cleanup paths. Calls into PostgreSQL belong in `#[pg_test]`. Use
`cargo check` for compilation, [regress](regress.md) for SQL expectations, and
[bench](bench.md) for measurements.

## Harness installation

The harness invokes cargo-pgrx separately to install the test extension. It uses
`CARGO_PGRX` when set, otherwise prefers the source checkout's CLI when available,
then falls back to PATH. Keep that executable compatible with the tested code.

The top-level `--cargo` arguments are not explicitly forwarded into this separate
install command. Configuration needed there must also be available through
Cargo's normal configuration discovery or environment. The resolved
`CARGO_TARGET_DIR` is inherited, so artifact locations remain consistent.

Currently `--no-schema` makes the harness pass `--no-schema` to `cargo pgrx install`,
whose parser does not accept that option. Avoid it for backend tests and allow
the schema to regenerate.

## Test cluster lifecycle

The current harness reserves a free TCP port for each test executable, overriding
the configured testing port. It creates a cluster under
`<resolved-target-dir>/test-pgdata/<major>-<test-process-id>`. `--pgdata` or
`CARGO_PGRX_TEST_PGDATA` changes the base directory; the invocation suffix remains.
On Unix, socket directories are separate short paths under `/tmp`.

Shutdown hooks normally stop PostgreSQL and remove the invocation's data and
socket directories. A forced termination can prevent cleanup. Use the failed
run's logs and exact data path to identify its server before using that
installation's `pg_ctl stop -D <owned-test-data-dir> -m fast`, under the same OS
user that started it. Do not delete a running cluster or use a broad process kill.
`cargo pgrx stop` only handles managed `PGRX_HOME/data-<major>` instances and will
not clean up these test clusters.

An isolated [PGRX_HOME](init.md#private-pgrx_home-for-agent-worktrees) still helps
select the intended installation. Dynamic test ports and unique data directories
do not isolate extension artifacts installed into a shared PostgreSQL prefix.

## Diagnosing failures

For missing PostgreSQL symbols, trace any backend calls made by plain unit tests.
For stale SQL, regenerate it from the current build. For hangs or crashes,
inspect the test server's logs, resource limits, and backtrace before changing
code or stopping a server; the ordinary managed instance may be unrelated.
