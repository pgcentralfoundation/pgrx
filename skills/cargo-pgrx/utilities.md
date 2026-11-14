# Utility commands

## cargo pgrx info

Reads the active pgrx configuration. Versions can be given as `18` or `pg18`.

```bash
cargo pgrx info path 18
cargo pgrx info pg-config 18
cargo pgrx info version 18
```

These print the installation path, `pg_config` path, and full PostgreSQL version
respectively. `info pg-config` is useful for finding an existing installation
before registering it in a [private worktree home](init.md#private-pgrx_home-for-agent-worktrees).

## cargo pgrx get

Reads a property from the extension's control file. It also provides the derived
`extname` and `git_hash` properties. `--package` and `--manifest-path` select the
extension; Cargo metadata is resolved before reading its control file.

```bash
cargo pgrx get comment
cargo pgrx get default_version
cargo pgrx get extname
```

## cargo pgrx upgrade

Updates pgrx dependency version requirements in the selected manifest. It does
not upgrade the cargo-pgrx executable or initialize PostgreSQL installations.
Review the manifest changes and keep the CLI compatible with the dependencies.

```bash
cargo pgrx upgrade --dry-run
cargo pgrx upgrade
cargo pgrx upgrade --to '=0.19.2'
cargo pgrx upgrade --include-prereleases
cargo pgrx upgrade --package my-extension
```

Options: `--to <VERSION_REQUIREMENT>`, `-m, --manifest-path`,
`-n, --dry-run`, `--include-prereleases`, and `-p, --package`.
`--dry-run` prints the proposed manifest instead of writing it.
