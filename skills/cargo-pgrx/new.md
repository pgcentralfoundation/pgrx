# cargo pgrx new

Creates a pgrx extension directory with a standard template or a background
worker template. Choose a new directory name; this command writes template
files and is not a migration tool for an existing extension.

```text
cargo pgrx new [OPTIONS] <NAME>
```

`-b, --bgworker` selects the background worker template.

## Generated files

- `Cargo.toml` with a cdylib target, matching pgrx and pgrx-tests dependencies,
  PostgreSQL features, `pg_test`, and optional `pg_bench` support
- `src/lib.rs` with the selected extension template and backend test setup
- `.cargo/config.toml` with macOS linker settings
- `<name>.control` and `.gitignore`
- `tests/pg_regress/sql/setup.sql` and
  `tests/pg_regress/expected/setup.out`
- An initially empty `sql/` directory for extension SQL files

The current template supports `pg13` through `pg19` and defaults to `pg13`.
Select another configured version explicitly or adjust the default feature.

```bash
cargo pgrx new my_extension
cd my_extension
cargo pgrx test pg18
```

For a background worker:

```bash
cargo pgrx new my_worker --bgworker
```

Initialize the needed PostgreSQL version first, using
[a private PGRX_HOME](init.md#private-pgrx_home-for-agent-worktrees) for agent
worktrees. Interactive use with `cargo pgrx run pg18` starts a persistent managed
server; stop the task's server afterward.
