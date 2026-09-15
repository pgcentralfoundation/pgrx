# Extension Upgrades & Migrations

PostgreSQL extensions support in-place version upgrades via upgrade scripts: files named `<extname>--<oldver>--<newver>.sql` located in the extension's SQL directory. When a user runs `ALTER EXTENSION my_ext UPDATE TO '1.1.0';`, Postgres executes the chain of upgrade scripts required to move from the installed version to the target version.

Because Postgres extensions often evolve over time, developers must write explicit SQL DDL statements (such as `ALTER TABLE`, `CREATE OR REPLACE FUNCTION`, custom type alterations) to upgrade their schema between releases. `pgrx` does not automatically generate arbitrary SQL schema migration statements from Rust code diffs.

However, writing monolithic, hand-edited upgrade scripts directly in source control across parallel feature branches frequently creates merge conflicts. To make team development and releases seamless, `pgrx` provides a modular migration fragments architecture and the `cargo pgrx migrate` toolchain.

---

## The Migration Fragments Workflow

Instead of modifying a shared upgrade script, each developer or PR adds individual SQL snippets to `sql/unreleased/`:

```shell
$ tree
.
├── Cargo.toml
├── my_ext.control
├── sql
│   ├── my_ext--0.24.0--0.25.0.sql
│   └── unreleased
│       ├── 101.add_custom_index.sql
│       ├── 102.new_vector_proc.sql
│       └── 105.alter_vector_proc.sql
└── src
    └── lib.rs
```

### Fragment Naming

Fragments are named with a PR or issue number prefix and a descriptive slug:
* `<PR_NUMBER>.<slug>.sql` (e.g. `101.add_custom_index.sql` -> ID `101`)
* `<slug>.sql` (e.g. `add_custom_index.sql` -> ID `add_custom_index`)

### Declaring Prerequisites (`-- depends-on:`)

When a fragment depends on database objects created or modified in another unreleased fragment, it declares dependencies in a header comment:

```sql
-- depends-on: 101, 102
/* depends-on: 103 */

ALTER FUNCTION my_vector_proc(integer) ...;
```

`pgrx` topologically sorts fragments according to their dependency graph. Independent fragments are ordered deterministically by ID (numerically, then alphabetically by filename). Missing prerequisites not found in `sql/unreleased/` are assumed to have been applied in a predecessor release.

---

## Development Workflows

During development (`cargo pgrx install`, `cargo pgrx run`, `cargo pgrx test`), `pgrx` automatically compiles and installs unreleased migration fragments ephemerally:

1. Resolves the latest released target version in `sql/` (or package version in `Cargo.toml`).
2. Derives an ephemeral target version bump (e.g. `0.25.0` -> `0.25.1`).
3. Assembles unreleased fragments topologically directly into the destination extension directory as `my_ext--0.25.0--0.25.1.sql`.
4. Copies the compiled base schema to `my_ext--0.25.1.sql` so `CREATE EXTENSION` installs directly at the new version without traversing upgrade paths.
5. Updates `default_version` in the installed `my_ext.control` file.

Your Git working tree remains completely clean while testing migrations locally. To disable ephemeral assembly during install or run, pass `--no-assemble-unreleased`.

### Packaging Protection

To prevent unreleased development fragments from leaking into release tarballs, `cargo pgrx package` disallows unreleased fragments by default and exits with an error unless `--assemble-unreleased` is explicitly passed.

---

## Assembling for Release (`cargo pgrx migrate assemble`)

When cutting a release, assemble unreleased fragments into a permanent release upgrade script:

```console
$ cargo pgrx migrate assemble 0.26.0 --update-control
  Assembling 3 SQL migration fragment(s) from sql/unreleased
   - 101.add_custom_index.sql
   - 102.new_vector_proc.sql
   - 105.alter_vector_proc.sql
       Saved assembled upgrade script to sql/my_ext--0.25.0--0.26.0.sql
     Updated default_version = '0.26.0' in my_ext.control
     Removed consumed fragment 101.add_custom_index.sql
     Removed consumed fragment 102.new_vector_proc.sql
     Removed consumed fragment 105.alter_vector_proc.sql
```

Key arguments & flags:
* `[TARGET_VERSION]`: Target release version. Defaults to `package.version` in `Cargo.toml` if greater than existing `sql/` releases, or derives the next patch version from the latest existing release script.
* `--update-control`: Automatically updates `default_version = '<target>'` in `<extname>.control`.
* `--prev-version <VERSION>`: Explicit predecessor version override.
* `--allow-empty`: Generates a valid stub upgrade script when no schema changes occurred.
* `--preserve-fragments`: Keeps fragments in `sql/unreleased/` instead of consuming (deleting) them.
* `--dry-run`: Previews the migration plan without mutating disk.
* `--json`: Outputs the migration plan in structured JSON format.

---

## CI & Pre-Commit Linting (`cargo pgrx migrate check` / `lint`)

To catch invalid directives, circular dependency loops, or missing dependency declarations before merging PRs, run `cargo pgrx migrate check` in CI:

```console
$ cargo pgrx migrate check --format github
::error file=sql/unreleased/105.alter_vector_proc.sql::Fragment touches unreleased object(s) from fragment `102` but does not declare '-- depends-on: 102'
❌ 105.alter_vector_proc.sql: Fragment touches unreleased object(s) from fragment `102` but does not declare '-- depends-on: 102'

❌ Migration fragment lint failed with 1 error(s).
```

### Lint Checks

- DAG cycles and syntax: Verifies `-- depends-on:` syntax and ensures no circular dependency cycles exist.
- Undeclared dependencies: Extracts top-level SQL statements (`CREATE`, `ALTER`, `DROP` for `FUNCTION`, `PROCEDURE`, `AGGREGATE`, `TABLE`, `VIEW`, `TYPE`, `OPERATOR`). If fragment `B` alters or replaces an object introduced in unreleased fragment `A`, `B` must declare `-- depends-on: <A>`.
- Mixed object prevention (`--deny-mixed-objects`): Errors if a fragment touches both already-released objects and unreleased objects, safeguarding cherry-picks to stable maintenance branches.
- Git diff scoping (`--base <REF>`): Scopes checks only to fragments added or modified relative to `<REF>` (e.g. `main` or `origin/main`).
- Diagnostics formatting (`--format <text|json|github>`): Outputs diagnostics for terminal, JSON, or GitHub Actions annotations.

---

## Inspecting Migration Plans (`cargo pgrx migrate info`)

Automated release workflows can inspect predecessor versions, target versions, and the ordered fragment plan without touching disk:

```console
$ cargo pgrx migrate info 0.26.0 --json
```

---

## Customizing Fragment Directory

By default, fragments are loaded from `<crate>/sql/unreleased/`. You can specify a custom directory in `Cargo.toml`:

```toml
[package.metadata.pgrx]
unreleased-sql-dir = "migrations/unreleased"
```
