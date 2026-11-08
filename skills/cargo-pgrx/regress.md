# cargo pgrx regress

Builds and installs the extension, restarts its managed PostgreSQL instance,
and compares SQL results with expected output using `pg_regress`. It uses the
ordinary managed instance, not the separate backend-test cluster. Use a
[private home](init.md#private-pgrx_home-for-agent-worktrees) for agent experiments
and stop the temporary server when finished.

## Files and database lifecycle

Paths are relative to the selected extension manifest:

| Path | Purpose |
|------|---------|
| `tests/pg_regress/sql/*.sql` | SQL inputs |
| `tests/pg_regress/expected/*.out` | Committed expectations |
| `tests/pg_regress/results/*.out` | Actual results |
| `tests/pg_regress/regression.diffs` | Differences; repeated runs preserve numbered diff files |

The database defaults to `<extname>_regress`. It is reused unless `--resetdb`
is given or `setup.sql` is newer than its expected output, in which case it is
recreated. `setup.sql` is included when the database is created, including
filtered runs. Check ownership before using any option that recreates a database.

## Usage and options

```text
cargo pgrx regress [OPTIONS] [PG_VERSION] [TESTNAME]...
```

The optional version is `pg15` through `pg19` and must come first; a supported
version label in a later position is an error, not a filter. Each `TESTNAME` is
a substring filter, and a test runs when its name contains any of them:
`cargo pgrx regress pg18 arrays spi` runs every test whose name contains
`arrays` or `spi`. Each filter must match at least one test, so a typo fails the
run instead of silently selecting nothing. Selected tests run in command-line
order: `cargo pgrx regress pg18 spi arrays` runs the `spi` matches before the
`arrays` matches. Tests matching the same filter run in sorted file order, and
a test matching several filters runs once, at its first match. `setup.sql`, when
included, runs first. All selected tests share one database, so each sees what
`setup.sql` and earlier selected tests left behind. `PG_VERSION` can supply the
positional default.

| Flag | Meaning |
|------|---------|
| `-a, --auto` | Promote changed output for tests with existing expectations |
| `--add <TEST>` | Bootstrap one test's expectation; implies `--resetdb` |
| `--dry-run` | Preview selection without building, installing, starting PostgreSQL, or running SQL |
| `--resetdb` | Recreate the regression database |
| `--repeat <N>` | Repeat the suite; default 1 |
| `--dbname <DB>` | Override the regression database name |
| `-r, --release` | Release profile; default is development |
| `--profile <P>` | Custom profile; overrides `--release` |
| `--psql-verbosity <V>` | `default`, `verbose`, `terse`, or `sqlstate`; default is terse |
| `--postgresql-conf <K=V>` | Repeatable server settings; the runner appends `client_min_messages=warning` |
| `--valgrind` | Start PostgreSQL under Valgrind |
| `--runas <USER>` | Used for database creation/deletion through sudo; not for managed server startup |
| `--pgdata <DIR>`, `-n, --no-schema` | Accepted but currently not applied to the regression install/start path |
| `-F, --features <F>` | Additional Cargo features |
| `--no-default-features`, `--all-features` | Cargo feature selection |
| `-p, --package <PKG>`, `--manifest-path <PATH>` | Select the extension |
| `--cargo <FLAG>` | Repeatable Cargo flags, including for metadata |
| `-v` | Print diff contents as well as their location |

`--dry-run` still resolves Cargo metadata and may create missing regression
directories. It prints an error for a filter that matches no test but still
exits successfully. It is not a substitute for inspecting the selected manifest
and database before an actual run. `--pgdata` does not isolate this command;
choose the private `PGRX_HOME` explicitly.

## Expected-output workflow

```bash
cargo pgrx regress pg18 -v
cargo pgrx regress pg18 search_basic search_phrase
cargo pgrx regress pg18 --add my_new_test
cargo pgrx regress pg18 --auto
cargo pgrx regress pg18 --resetdb --repeat 5
cargo pgrx regress pg18 --dry-run
```

Run without `--auto` first and inspect failures. Promote only intentional
changes, then rerun without promotion to verify the new expectations. A run
that encountered failures still exits unsuccessfully even if `--auto` copied
its results into expected files.

A test containing a `-- pgrx-regress-skip pg15 pg16` or
`-- pgrx-regress-only pg17` line is reported as `SKIP` on other versions, even
when selected explicitly. It needs no expected output there and cannot be
bootstrapped with `--add` on such a version. Malformed labels are an error.

Unfiltered runs skip tests without expected output; explicitly selecting one
reports an error directing you to `--add`. Bootstrap new tests with `--add`,
which runs setup, generates output, and stages the new expected files when in
a Git repository. Review both working-tree and staged diffs. Do not hand-edit
expected output to hide failures.
