# cargo pgrx bench

Runs `#[pg_bench]` functions inside PostgreSQL using pgrx-bench and Criterion.
The CLI enables `pg_bench`, builds in release mode by default, installs the
extension, and stores measurements and comparison history in the benchmark
database. See [the benchmark example](../../pgrx-examples/benching) for the
optional `pgrx-bench` dependency, feature, and `benches` schema setup.

## Usage and options

```text
cargo pgrx bench [OPTIONS] [PG_VERSION] [BENCHNAME]
```

The optional version is `pg13` through `pg19`; the benchmark selector filters
names. `PG_VERSION` can supply the positional default. The default database is
`<extname>_benches` in the ordinary managed instance. Use a
[private home](init.md#private-pgrx_home-for-agent-worktrees) for agent work and
stop its server afterward.

| Flag | Meaning |
|------|---------|
| `--group-name <NAME>` | Name this run group; otherwise a name is generated |
| `--compare-group <NAME>` | Compare with an existing named group |
| `--resetdb` | Recreate the benchmark database, deleting its history |
| `--cascade` | Use CASCADE when dropping the extension during refresh |
| `--list` | Build, install, refresh the extension, then list benchmarks |
| `--report` | Read stored history without building or refreshing the extension |
| `--json` | Emit the final benchmark summary as JSON |
| `--wait <SECONDS>` | Pause after printing the backend PID for profiler attachment |
| `--debug` | Development profile instead of release |
| `--profile <P>` | Custom profile; overrides `--debug` |
| `--postgresql-conf <K=V>` | Repeatable server settings |
| `--dbname <DB>` | Override the benchmark database name |
| `-F, --features <F>` | Additional Cargo features |
| `--no-default-features`, `--all-features` | Cargo feature selection |
| `-p, --package <PKG>`, `--manifest-path <PATH>` | Select the extension |
| `--target <TARGET>` | Cargo compilation target |
| `--cargo <FLAG>` | Repeatable Cargo flags, including for metadata |

## Runs and comparisons

```bash
cargo pgrx bench pg18
cargo pgrx bench pg18 index_build --group-name before
cargo pgrx bench pg18 index_build --group-name after --compare-group before
cargo pgrx bench pg18 --list
cargo pgrx bench pg18 --report
cargo pgrx bench pg18 --json
cargo pgrx bench pg18 --wait 5
```

Without `--compare-group`, the CLI chooses the most recent completed or partial
group with the same Cargo profile, when available. Record named baselines when
the comparison matters. Keep the database between runs to retain history;
`--resetdb` discards it.

Normal runs restart the managed server and refresh the installed extension.
`--list` also performs that setup. `--report` reads existing history and may
start a stopped server, but requires the benchmark database to exist. It cannot
be combined with `--group-name`, `--compare-group`, `--resetdb`, `--cascade`,
`--list`, `--json`, a nonzero `--wait`, or `--postgresql-conf`.

Keep `pg_bench` disabled for ordinary deployment artifacts. For pure Rust
benchmarks that need no PostgreSQL state, use the project's ordinary Rust
benchmark setup instead.
