# migration_fragments

An extension whose upgrade SQL is kept as [SQL fragments](../../docs/src/upgrades/fragments.md):
one small file per change, assembled into an upgrade script when a release is cut.

```console
.
├── Cargo.toml                           # opts in with [package.metadata.pgrx]
├── migration_fragments.control          # default_version = '@CARGO_VERSION@'
├── sql
│   ├── migration_fragments--1.0.0--1.1.0.sql
│   └── unreleased
│       ├── 98.index_shouted_greetings.sql
│       ├── 101.add_shout.sql
│       └── 102.add_greeting_log.sql
├── src/lib.rs
└── tests/pg_regress/sql/upgrade.sql
```

Version 1.1.0 is the latest release, and its upgrade script is in `sql/`. Three pull requests have
merged since then, each with a fragment. Fragment 98 is the oldest pull request, but it indexes a
table that fragment 102 creates, so it declares `-- depends-on: 102`.

Every object appears twice: once in `src/lib.rs`, which `CREATE EXTENSION` installs from, and once
as SQL under `sql/`, which `ALTER EXTENSION ... UPDATE` runs.

## Try It

Run these from this directory.

See what releasing would do, without changing anything:

```console
$ cargo pgrx migrate info
Extension: migration_fragments (1.1.0 -> 1.1.1)
Output: .../sql/migration_fragments--1.1.0--1.1.1.sql
Fragments: (3):
  - 101.add_shout.sql (id: 101)
  - 102.add_greeting_log.sql (id: 102)
  - 98.index_shouted_greetings.sql (id: 98, depends on: [102])
```

Check that the fragments declare their dependencies:

```console
$ cargo pgrx migrate check --deny-mixed-objects
✅ All migration fragments passed lint checks.
```

Run the tests. `cargo pgrx test` assembles the fragments into a temporary 1.1.1 upgrade, and the
tests check that the update path from 1.0.0 reaches it:

```console
$ cargo pgrx test
```

Run the upgrade itself. `upgrade.sql` takes a database from 1.1.0 to 1.1.1 with
`ALTER EXTENSION ... UPDATE`, and checks that it ends up with the same objects as a new install:

```console
$ cargo pgrx regress
```

Preview the release, then cut it if you like. `git restore .` undoes it.

```console
$ cargo pgrx migrate assemble --dry-run
$ cargo pgrx migrate assemble
```
