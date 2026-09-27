# cargo pgrx run

Builds and installs an extension, starts its managed PostgreSQL instance,
creates the named database if missing, and opens `psql` or `pgcli`. It stops the
managed server before installing and restarts it afterward. Run this only
against an authorized instance, using a [private home](init.md#private-pgrx_home-for-agent-worktrees)
for agent worktree experiments.

Installing files does not execute `CREATE EXTENSION` in the database. Create or
update the extension there as the task requires. Exiting the client leaves the
server running; shut down task-owned instances when finished.

## Usage and options

```text
cargo pgrx run [OPTIONS] [PG_VERSION] [DBNAME]
```

`PG_VERSION` accepts `pg13` through `pg19`, with `PG_VERSION` as an environment
fallback. Otherwise the CLI uses a PostgreSQL feature supplied explicitly or in
the manifest's defaults. `DBNAME` defaults to the extension name. If the first
positional argument is not a recognized version and no second argument is given,
it is treated as the database name.

| Flag | Meaning |
|------|---------|
| `-r, --release` | Release profile; default is development |
| `--profile <P>` | Custom profile; overrides `--release` |
| `--pgcli` | Use pgcli on PATH; also accepts `PGRX_PGCLI` |
| `--valgrind` | Start PostgreSQL under Valgrind |
| `--install-only` | Skip restarting the server and creating the database; see the limitation below |
| `-F, --features <F>` | Additional Cargo features |
| `--no-default-features`, `--all-features` | Cargo feature selection |
| `-p, --package <PKG>` | Select a workspace extension |
| `--manifest-path <PATH>` | Select its manifest |
| `--target <TARGET>` | Cargo compilation target; the installed artifact must run on this server |
| `--cargo <FLAG>` | Repeatable Cargo flags, including for metadata |

In the current implementation, `--install-only` skips server startup inside the
installation step, but command dispatch still attempts to open the client.
Use [cargo pgrx install](install.md) with an explicit `--pg-config` for a command
that installs files without stopping or starting PostgreSQL or opening a client.

```bash
cargo pgrx run pg18
cargo pgrx run pg18 scratch_db
cargo pgrx run pg18 scratch_db --pgcli
cargo pgrx run pg18 --release --cargo=--config=./cargo-local.toml
```

Use `cargo check` for compilation and `cargo pgrx test` or `cargo pgrx regress`
for automated coverage. `connect` can open a client without rebuilding, but may
also start the server and create a database.
