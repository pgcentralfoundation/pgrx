# Instance management

Managed instances use `$PGRX_HOME/data-<major>`, `$PGRX_HOME/<major>.log`, and
`base_port + major`. These are separate from the backend test harness's clusters.
For agent worktrees, use [a private home](init.md#private-pgrx_home-for-agent-worktrees)
and stop the instances the task starts.

## start

```bash
cargo pgrx start pg18
cargo pgrx start pg18 --postgresql-conf shared_buffers=256MB
cargo pgrx start all
```

Starts a stopped instance, creating its cluster if needed. An already running
instance is left running, so new configuration settings require a restart.
With no version, selection uses the chosen extension's default PostgreSQL
feature. `all` selects versions present in both its features and the active
pgrx configuration.

Options: `--package`, `--manifest-path`, repeatable `--postgresql-conf <K=V>`,
and `--valgrind`. Package metadata is needed even with an explicit version.

## stop

```bash
cargo pgrx stop pg18
cargo pgrx stop all
```

Uses `pg_ctl stop -m fast` for the selected managed cluster. An already stopped
instance is accepted. Version and package selection follow `start`, including
the manifest/configuration intersection for `all`. Options: `--package` and
`--manifest-path`.

This does not stop arbitrary PostgreSQL processes or pgrx-tests clusters. Never
change to the user's default home as a way to clean up a private worktree.

## status

```bash
cargo pgrx status pg18
cargo pgrx status
```

Reports whether managed instances are running. With no version, it checks all
configured versions, unless `PG_VERSION` selects one. Although `--package` and
`--manifest-path` are accepted, status does not use them to choose a default
version or filter the configured instances.

## connect

```bash
cargo pgrx connect pg18
cargo pgrx connect pg18 mydb
cargo pgrx connect pg18 mydb --pgcli
```

Starts the managed server if needed, creates the database if missing, and opens
`psql` or `pgcli`. It does not rebuild or install the extension, but it is not a
read-only status command. The database defaults to the extension name; `DBNAME`
can supply it. `PG_VERSION` can supply the version, and an unrecognized first
positional value can be interpreted as a database name.

Options: `--package`, `--manifest-path`, `--pgcli` (also `PGRX_PGCLI`), and
`--valgrind`. Valgrind affects server startup, not an already running server.
Exiting the client leaves the server running; stop temporary instances when done.
