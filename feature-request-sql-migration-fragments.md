# Feature Request: Fragment-based extension migration & upgrade management for development and release workflows

### Summary

We propose adding first-class support in `cargo-pgrx` for managing unreleased SQL migration fragments.

In active extension repositories with stable release branches (e.g. maintaining `main` for upcoming minor releases while backporting bugfixes to `0.25.x`), managing SQL upgrade scripts in monolithic files (`extension--<prev>--<target>.sql`) presents a few practical challenges:
1. Branch decoupling & cherry-picking: When authoring a change on `main`, it is not always clear which release version will ship first. Monolithic scripts couple SQL changes to specific version-range filenames, which means cherry-picking a change to a stable release branch typically requires manually adjusting or splitting migration scripts across branches.
2. Local dev iteration with fragments: While `cargo pgrx install` copies pre-existing version-to-version scripts (`sql/extension--<old>--<new>.sql`), it does not assemble unreleased fragment files. Without an ephemeral upgrade script in Postgres's extension directory during development, developers testing in-flight changes against existing databases often resort to dropping and recreating the extension, which resets test data and requires rebuilding dependent indexes.

Taking inspiration from ORM migration frameworks (such as Django, Alembic, or Flyway), where schema modifications are authored as discrete migration units declaring prerequisite dependencies, a fragment-based workflow addresses these challenges:
- Dedicated, version-agnostic fragments: Each change introduces a standalone fragment file (e.g. `sql/unreleased/<PR>.<description>.sql`). The fragment cherry-picks cleanly between branches as an isolated unit and is assembled into the correct versioned upgrade script when that specific branch cuts a release.
- Ephemeral dev assembly: `cargo pgrx install` automatically assembles unreleased fragments directly into Postgres's `$SHAREDIR/extension/` and updates `default_version` in the installed `.control` file. Developers can run `ALTER EXTENSION ... UPDATE;` immediately without modifying the git working tree or resetting existing test databases.
- Release assembly: A dedicated migration subcommand (e.g. `cargo pgrx migrate assemble`) topologically sorts fragments based on explicit dependencies (`-- depends-on:`), concatenates them into the final versioned upgrade script, and removes the consumed fragments.

---

### Motivation & Problem Description

Currently, `cargo-pgrx` handles extension installation in [cargo-pgrx/src/command/install.rs](https://github.com/pgcentralfoundation/pgrx/blob/70383e8840428d0859a85011709422a57864aa9a/cargo-pgrx/src/command/install.rs#L418-L439) by:
1. Generating the full base schema `extension--<version>.sql` and placing it in `$SHAREDIR/extension/`.
2. Copying pre-existing version upgrade files strictly matching `^extension--.+--.+\.sql$` from `<crate>/sql/` into `$SHAREDIR/extension/`.

In projects with concurrent pull requests or multiple release branches, this model has a few areas that can be improved:

#### 1. Decoupling Changes for Stable Branch Backports & Cherry-Picking

In extensions with active stable branches (e.g. fixing a bug on `main` that must also be backported to `0.25.x`):
* On `main`, the next release might be `0.26.0` (target file: `extension--0.25.0--0.26.0.sql`).
* On the stable branch, the next release might be `0.25.5` (target file: `extension--0.25.4--0.25.5.sql`).
* When authoring a commit on `main`, developers cannot predict with certainty which patch release will receive the backport, or whether the change will only ever ship in the next minor release.
* Committing SQL directly to a monolithic version-range file can complicate `git cherry-pick` operations or require manual version adjustments across branches.

With standalone fragments, each change lives in its own file (`sql/unreleased/<PR>.<description>.sql`). The fragment can be cherry-picked cleanly between branches without file conflicts, and each branch independently resolves and assembles the fragments present on that branch at release time.

#### 2. Local Development Iteration with Unreleased Fragments

When developers work with monolithic files in `sql/`, `cargo pgrx install` copies those files over, and `ALTER EXTENSION ... UPDATE;` works as expected. However:
* Testing changes locally against an existing database requires adding SQL to a version-coupled script in `sql/` (e.g. `extension--0.25.0--0.26.0.sql`), introducing version assumptions in the working copy that may need to be adjusted later when cherry-picking or rebasing.
* If an extension instead organizes pending changes into unreleased fragments (`sql/unreleased/*.sql`), `cargo pgrx install` ignores them because they do not match `^extension--.+--.+\.sql$`.
* Without an assembled upgrade script in `$SHAREDIR/extension/`, `ALTER EXTENSION ... UPDATE;` is a no-op because Postgres does not find an upgrade path between the installed version and the target version.
* The common fallback is to run `DROP EXTENSION ... CASCADE; CREATE EXTENSION ...;`. For extensions backed by substantial datasets or complex indexes (such as BM25 search indexes, vector indices, or benchmark suites like ParadeDB or TxPipe's Mumak in [#1960](https://github.com/pgcentralfoundation/pgrx/issues/1960)), recreating the extension drops dependent objects and requires re-indexing from scratch, which slows down local iteration.

#### 3. Merge Conflicts on Monolithic Scripts

When multiple contributors work on separate features concurrently, having every PR edit the same monolithic upgrade script (`extension--0.25.0--0.26.0.sql` or `unreleased.sql`) can cause merge conflicts upon rebase or merge.

---

### Detailed Design

#### 1. Fragment Directory & File Convention

PRs that introduce DDL or schema adjustments place standalone migration fragments in:
```
<crate>/sql/unreleased/<ID>.<description>.sql
```
For example:
* `sql/unreleased/6221.inline_row_evaluation.sql`
* `sql/unreleased/6245.add_bm25_similarity_operator.sql`

The `<ID>` typically corresponds to a PR or issue number (numeric) or a unique slug. 

The unreleased directory can be configured in `Cargo.toml` under `[package.metadata.pgrx]`:
```toml
[package.metadata.pgrx]
unreleased-sql-dir = "sql/unreleased" # default
```

#### 2. Explicit Dependency Tracking & Topological Sorting

When one migration fragment depends on objects introduced by another in-flight fragment, it declares dependencies via a standardized comment header:
```sql
-- depends-on: 6099, 6120
```
or
```sql
/* depends-on: 6099, 6120 */
```

`cargo-pgrx` parses these declarations and builds a dependency graph:
* Topological Sort: Fragments are ordered so dependencies run before dependents.
* Deterministic Tie-Breaking: Independent fragments are sorted deterministically (e.g. by `(ID, filename)`).
* Cycle & Missing Dependency Detection: Cycles or missing dependencies are reported with clear diagnostics. On release branches, this also allows CI to verify that any prerequisite fragments were backported along with the dependent fragment.

This mechanism takes direct inspiration from ORM migration systems (such as Django migrations or Alembic), where migration files form a directed acyclic graph (DAG) via explicit dependency declarations rather than relying on brittle global sequence numbers. In the context of PostgreSQL extensions, `cargo-pgrx` adapts this model by resolving the DAG and compiling it into the versioned `<ext>--<prev>--<target>.sql` file expected by Postgres.

#### 3. Development Workflow: Ephemeral Assembly during `cargo pgrx install`

When running `cargo pgrx install` (or `cargo pgrx run`):
1. Normal compilation and installation proceed (building the shared library into `$PKGLIBDIR`, generating the base schema, and copying pre-existing released upgrade scripts).
2. If `sql/unreleased/` contains `.sql` fragments:
   - Resolve Previous Version: Detect the highest target version from existing `sql/<ext>--*--*.sql` files (or fallback to the base version in `Cargo.toml`).
   - Compute Ephemeral Target Version: Derive an ephemeral target version (e.g. bumping the minor/patch version, such as `0.25.9` -> `0.26.0-dev` or `0.26.0`).
   - Assemble Ephemeral Script: Topologically sort the unreleased fragments and assemble them directly into `$SHAREDIR/extension/<ext>--<prev>--<target>.sql`.
     - Includes standard safety header:
       ```sql
       \echo Use "ALTER EXTENSION <ext> UPDATE TO '<target>'" to load this file. \quit
       ```
     - Includes fragment delimiter banners for clear SQL debugging.
   - Update Installed Control File: Update `default_version` in `$SHAREDIR/extension/<ext>.control` to match the target version.

Crucially, this happens entirely within Postgres's `$SHAREDIR/extension/`:
* The repository working copy and git status remain completely clean (no dirty untracked files).
* `cargo pgrx install` requires no live database connection.
* Developers can run `ALTER EXTENSION <ext> UPDATE;` in their active test database to apply in-flight changes without dropping tables or rebuilding indexes.

CLI Flags:
* `--assemble-unreleased`: Defaults to `true` when unreleased fragments exist.
* `--no-assemble-unreleased`: Disables ephemeral script assembly.

#### 4. Release Workflow: `cargo pgrx migrate assemble`

At release time, maintainers freeze the unreleased fragments into a permanent versioned upgrade script:

```bash
cargo pgrx migrate assemble [TARGET_VERSION] [OPTIONS]
```

Options for `assemble`:
* `TARGET_VERSION`: The new release version (e.g. `0.26.0`). If omitted, reads `package.version` from `Cargo.toml`.
* `--prev-version <VERSION>`: The previous release version (e.g. `0.25.9`). If omitted, automatically resolves the latest target version from existing upgrade scripts in `sql/`.
* `--consume`: (Default for releases) Deletes the assembled fragments from `sql/unreleased/` once concatenated into `sql/<ext>--<prev>--<target>.sql`.
* `--preserve-fragments`: Preserves fragments in `sql/unreleased/` (useful for release candidate / beta builds).
* `--output-dir <DIR>`: Output directory for the assembled script (defaults to `<crate>/sql/`).

Command Placement:
We propose a top-level `migrate` subcommand rather than nesting under `schema` or `package`:
* `cargo pgrx schema` currently accepts positional arguments (`cargo pgrx schema [PG_VERSION] [ENTITY_NAME...]`) for AST schema generation. Nesting migration assembly here (`cargo pgrx schema assemble`) creates clap argument ambiguity with entity names and conflates AST code generation with version upgrade lifecycle management.
* `cargo pgrx package` is focused on compiling and staging binary distribution packages in `target/`. Mutating repository sources (deleting fragments and writing release SQL) within `package` would break its idempotency and cause conflicts during multi-version matrix builds (`pg13`..`pg17`).
* A dedicated `migrate` command creates a clean, discoverable home for upgrade workflows, leaving room for future subcommands like `cargo pgrx migrate lint`. (If maintainers prefer grouping under an existing command, `cargo pgrx schema assemble` is an alternative).

#### 5. Role of `cargo pgrx package` as Downstream Consumer

`cargo pgrx package` delegates directly to `install_extension(...)` to copy extension artifacts into the packaging staging directory (`target/release/extname-pgXX/share/extension/`). It should consume this workflow in two ways:

1. Release Validation (Default):
   When running `cargo pgrx package` to build distribution artifacts, `package` checks whether `sql/unreleased/` contains unreleased fragments. If found, it warns or errors:
   `error: found unreleased SQL fragments in sql/unreleased/. Run cargo pgrx migrate assemble before packaging for release.`
   This prevents accidental releases where a version bump was committed but in-flight migration fragments were forgotten.

2. Hermetic Package Assembly:
   For CI or nightly pipelines that build release tarballs/packages without committing the assembled script back to git first, `cargo pgrx package --assemble-unreleased` can assemble the unreleased fragments directly into the staging package directory (`out_dir`), matching what `install` does for `$SHAREDIR`.

---

### Alternatives Considered

#### 1. Pure Schema-Diffing (Issue [#2375](https://github.com/pgcentralfoundation/pgrx/issues/2375))

Issue [#2375](https://github.com/pgcentralfoundation/pgrx/issues/2375) discusses integrating schema-diffing tools (such as `pg-schema-diff`) directly into `cargo-pgrx` to automatically generate `extension--<old>--<new>.sql` upgrade scripts.

In fact, ParadeDB already uses `pg-schema-diff` in CI today: CI diffs the base branch schema against the pull request schema to generate a suggested SQL diff, and validates that the pull request contains those changes.

However, relying purely on release-time schema diffing without a fragment workflow has a few limitations:

1. Destination for in-flight changes and cherry-picking:
   Even when a tool like `pg-schema-diff` generates the SQL diff between a pull request and its base branch, that SQL needs a place to live in the repository. If it is committed directly to a version-named monolithic file (e.g. `extension--0.25.0--0.26.0.sql`), cherry-picking that commit to a stable release branch (targeting `0.25.5`) requires manually moving the SQL into a different version-range file. Fragments provide a version-agnostic container for the diff.
2. Local development iteration without branch comparisons:
   Generating a schema diff requires a clean baseline schema from a base branch (which CI can generate by checking out the base branch). During offline or iterative local development, checking out separate branches to diff against is less convenient than having `cargo pgrx install` assemble unreleased fragments directly into `$SHAREDIR/extension/`.
3. Destructive ambiguity and ordering:
   Declarative diffs cannot always deduce developer intent for renames or signature changes. For example, renaming a function or changing an argument type can cause a declarative diff to emit `DROP FUNCTION old(...)` followed by `CREATE FUNCTION new(...)`. In Postgres, if user views, indexes, or privileges depend on `old(...)`, dropping the function can fail or unexpectedly drop dependent user objects. Explicit fragments allow developers to review the diff generated by tooling and adjust it (e.g., adding backward-compatible wrapper functions or ordering dependencies).
4. Non-DDL and extension-specific commands:
   While most Postgres extension upgrades consist of function and type DDL, extensions that maintain internal tables (such as metadata catalogs or sequence caches) sometimes require custom commands (e.g., `SELECT pg_catalog.pg_extension_config_dump(...)`, custom `GRANT`s, or metadata backfills) that declarative catalog diffs do not generate.

How Fragments and Schema-Diffing Work Together:
Rather than competing alternatives, fragments and schema-diffing complement each other naturally:
- Tooling like `pg-schema-diff` can generate the suggested SQL diff for a pull request.
- The fragment file provides the version-agnostic container that can be reviewed, committed, and cherry-picked cleanly between branches.
- `cargo pgrx migrate assemble` bundles those fragments into the versioned script at release time.

#### 2. Monolithic `sql/unreleased.sql`

Another alternative is maintaining a single `sql/unreleased.sql` file on `main`.
* Drawback: This reintroduces cherry-picking friction. When backporting a bugfix to a stable release branch, cherry-picking edits to a single shared `unreleased.sql` file causes merge conflicts across branches and between concurrent pull requests on `main`.
* Standalone fragments allow changes to be cherry-picked cleanly as discrete units.

#### 3. Ad-hoc Entity Extraction (`cargo pgrx schema <entity>`, PR [#2293](https://github.com/pgcentralfoundation/pgrx/pull/2293))

PR [#2293](https://github.com/pgcentralfoundation/pgrx/pull/2293) introduced `cargo pgrx schema <entity_name>` to emit SQL with `ALTER EXTENSION ... ADD ...`.
* Drawback: This was designed as an ad-hoc debugging tool for individual functions. It requires the developer to manually determine every modified entity, pipe SQL into `psql`, and does not handle ordered multi-object migrations, data adjustments, or automated release script generation.

#### 4. AST / Proc-Macro Snapshot Diffing (PR [#2197](https://github.com/pgcentralfoundation/pgrx/pull/2197))

PR [#2197](https://github.com/pgcentralfoundation/pgrx/pull/2197) explored generating upgrade scripts by comparing serialized snapshot graphs in `pgrx-sql-entity-graph`.
* Drawback: This approach involved maintaining snapshot serialization types and diffing logic within the proc-macro and SQL graph infrastructure, which proved challenging to maintain alongside evolving AST features. Managing migrations at the SQL fragment level keeps `cargo-pgrx` simpler and decoupled from compiler AST internals.

---

### Reference Implementation

This workflow is currently used in production in ParadeDB (for `pg_search`):
* Migration fragments, topological sorting, dependency validation, and branch linting are run across hundreds of PRs.
* The ephemeral `$SHAREDIR` installation wrapper allows developers to pull branches, run `cargo pgrx install`, and test changes against existing databases using `ALTER EXTENSION pg_search UPDATE;` without recreating the extension or rebuilding indexes.
