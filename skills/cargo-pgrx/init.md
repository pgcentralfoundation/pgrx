# cargo pgrx init

Registers PostgreSQL installations in the active `PGRX_HOME`, which defaults to
`~/.pgrx`. An explicit `pg_config` path reuses an existing installation without
downloading or recompiling PostgreSQL. `download` fetches and installs the
selected version; with no version flags or corresponding environment variables,
init downloads all supported defaults.

On Unix, downloaded installations live under
`$PGRX_HOME/<full-version>/pgrx-install`. Windows uses downloaded binaries under
the version directory. The current PG19 download is PostgreSQL 19beta4.

## Effects

Init stops the selected managed instances in the active home, validates each
installation, and creates missing `data-<major>` clusters unless `--no-run` is
set or the caller is root. It writes `config.toml` with the selected `pg_config`
paths and port bases. Existing version entries are retained, but omitted port
bases revert to defaults; repeat custom port flags when reinitializing a home.

## Usage and options

```text
cargo pgrx init [OPTIONS]
```

| Flag | Environment variable | Meaning |
|------|----------------------|---------|
| `--pg13 <PATH\|download>` | `PG13_PG_CONFIG` | PostgreSQL 13 |
| `--pg14 <PATH\|download>` | `PG14_PG_CONFIG` | PostgreSQL 14 |
| `--pg15 <PATH\|download>` | `PG15_PG_CONFIG` | PostgreSQL 15 |
| `--pg16 <PATH\|download>` | `PG16_PG_CONFIG` | PostgreSQL 16 |
| `--pg17 <PATH\|download>` | `PG17_PG_CONFIG` | PostgreSQL 17 |
| `--pg18 <PATH\|download>` | `PG18_PG_CONFIG` | PostgreSQL 18 |
| `--pg19 <PATH\|download>` | `PG19_PG_CONFIG` | PostgreSQL 19 |
| `--base-port <PORT>` | | Managed-instance base port; default 28800 |
| `--base-testing-port <PORT>` | | Configured test base port; default 32200 |
| `--configure-flag <FLAG>` | | Repeatable PostgreSQL configure arguments |
| `--no-run` | | Skip cluster initialization, useful for cross-compilation; configuration still queries `pg_config` |
| `--valgrind` | | Build downloaded PostgreSQL with Valgrind support |
| `-j, --jobs <N>` | | Parallel make jobs; defaults to available host parallelism |

Managed ports are `base_port + major`: a base of 39000 gives PG18 port 39018.
The configured test port is `base_testing_port + major`, but the current
pgrx-tests harness reserves a dynamic port for each test executable instead.
Choose non-overlapping ranges across concurrent worktrees and existing servers;
keep every resulting port within 1 through 65535. Init records these settings
without reserving the managed ports.

```bash
cargo pgrx init --pg18=download
cargo pgrx init --pg18=/usr/local/pgsql/bin/pg_config
cargo pgrx init --pg18=download --pg19=download --jobs 8
```

## Private PGRX_HOME for agent worktrees

A worktree can have its own `config.toml`, managed data directories, and logs
without rebuilding PostgreSQL. Save the source home before setting a private
`PGRX_HOME`, then register an existing
`$source_pgrx_home/<full-version>/pgrx-install/bin/pg_config`. The directory uses
the full version, such as `18.4` or `19beta4`, rather than the feature label
`pg18` or `pg19`. Read the source home's `config.toml` or use
`cargo pgrx info pg-config 18` to find its actual path.

Registering an existing installation isolates managed clusters, not extension
installation files. `install`, `run`, `test`, `regress`, and `bench` still write
to the `pkglibdir` and extension directory reported by that `pg_config`, which
may be inside the user's default home. Avoid concurrent installs of conflicting
builds into that prefix. When the user's installed extensions must remain
untouched, use a separate installation prefix, potentially a private copy of
the existing installation, and verify that its `pg_config` reports private paths
before installing. Merely copying `config.toml` or symlinking `pg_config` does
not isolate those files.

This Bash example is for an extension crate in an agent worktree. Adjust the
installed version, manifest, and both port bases before use. The sample ports
are examples, not a reservation; choose distinct available ranges for each
worktree.

```bash
source_pgrx_home="${PGRX_HOME:-$HOME/.pgrx}"
source_pg_config="$source_pgrx_home/18.4/pgrx-install/bin/pg_config"
worktree_manifest="$PWD/Cargo.toml"
worktree_pgrx_home="$(mktemp -d "${TMPDIR:-/tmp}/pgrx-worktree.XXXXXX")"

(
    set -e
    export PGRX_HOME="$worktree_pgrx_home"
    unset PGRX_PG_CONFIG_PATH PGRX_PG_CONFIG_AS_ENV

    cargo pgrx init --pg18="$source_pg_config" \
        --base-port 39000 --base-testing-port 40000

    trap 'cargo pgrx stop pg18 --manifest-path "$worktree_manifest"' EXIT

    cargo pgrx test pg18 --manifest-path "$worktree_manifest"
    # Keep any further worktree commands inside this subshell.
)
```

The subshell preserves the caller's original `PGRX_HOME`. Keep the private path
available for later commands and cleanup, and pass it explicitly in new shells.
`PGRX_PG_CONFIG_PATH` bypasses `config.toml` and its custom port bases;
`PGRX_PG_CONFIG_AS_ENV` also overrides configuration selection. Clear inherited
overrides that defeat the private configuration. Inherited `PG13_PG_CONFIG`
through `PG19_PG_CONFIG` values can register additional versions during init.

## Shut down temporary PostgreSQL

Stop every managed version started for the task with the same private home:

```bash
(
    export PGRX_HOME="$worktree_pgrx_home"
    unset PGRX_PG_CONFIG_PATH PGRX_PG_CONFIG_AS_ENV
    cargo pgrx stop pg18 --manifest-path "$worktree_manifest"
    cargo pgrx status pg18
)
```

Repeat for each version the task started. `stop all` is limited to versions shared by
the selected manifest and configuration; it is not a machine-wide cleanup tool.
If Cargo metadata prevents shutdown, use the registered installation's `pg_ctl`
with `stop -D "$worktree_pgrx_home/data-18" -m fast` to stop that owned cluster
directly.

The `test` harness normally shuts down its separate cluster and removes its
temporary data and socket directories. After an interrupted test, follow
[test.md](test.md) to identify any survivor; `cargo pgrx stop` does not manage
those test clusters. Confirm shutdown before deleting temporary directories.
If shutdown fails, retain the directory and report the failure. Remove only
task-owned resources, and keep the reused PostgreSQL installation intact.
