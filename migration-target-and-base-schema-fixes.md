# Migration Target Resolution and Base Schema Installation Fixes

## Context & Motivation

During integration testing of SQL migration fragments in ParadeDB (`pg_search`), two critical issues were uncovered when installing extensions into Postgres and inspecting migration plans:

1. **`CREATE EXTENSION` Object Collisions When `target_ver != manifest_version`**:
   - In repositories where `Cargo.toml`'s package version is not bumped post-release (e.g. `Cargo.toml` is `0.25.6` while released migrations exist up to `0.25.9`), `cargo pgrx install` compiles the Rust code and generates base schema `pg_search--0.25.6.sql`.
   - `handle_unreleased_fragments` then derives an ephemeral target version (`0.25.10`), writes upgrade script `pg_search--0.25.9--0.25.10.sql`, and updates `pg_search.control` (`default_version = '0.25.10'`).
   - However, `pg_search--0.25.10.sql` was never copied or installed into `$SHAREDIR/extension`.
   - When a user ran `CREATE EXTENSION pg_search CASCADE;`, PostgreSQL searched for `pg_search--0.25.10.sql`. Finding none, Postgres searched for an upgrade path, found `pg_search--0.25.6.sql`, and executed:
     `0.25.6.sql -> 0.25.6--0.25.7.sql -> ... -> 0.25.9--0.25.10.sql`.
   - Because `pg_search--0.25.6.sql` was generated from the current compiled Rust code, it already contained all current functions (e.g. `ctid_is_valid`). Executing the upgrade script `0.25.9--0.25.10.sql` on top of it failed with:
     `ERROR: function "ctid_is_valid" already exists with same argument types`.

2. **`cargo pgrx migrate assemble` / `info` Default Target Version Resolution**:
   - When `TARGET_VERSION` was omitted on the CLI, `assemble` and `info` previously defaulted to `manifest_pkg_ver` from `Cargo.toml`.
   - If `Cargo.toml` has `0.25.6` but `sql/` already has `0.25.9`, `resolve_prev_version` filtered for existing targets `< 0.25.6`, producing `0.25.5 -> 0.25.6`. This proposed assembling an upgrade script for a version that was already released in the past.

---

## Changes Made

### 1. Base Schema Installation in `handle_unreleased_fragments`
**File**: `cargo-pgrx/src/command/migrate/mod.rs`

When unreleased fragments are assembled during `cargo pgrx install` (or `package`), if `pkg_ver != target_ver_str`:
- The generated base schema `extdir.join(format!("{extname}--{pkg_ver}.sql"))` is copied to `extdir.join(format!("{extname}--{target_ver_str}.sql"))`.
- The new base schema file is tracked in `output_tracking` for reporting and clean uninstall.
- This ensures PostgreSQL's extension loader finds the exact base schema matching `default_version` in `.control`. `CREATE EXTENSION` executes in a single pass without searching for obsolete upgrade paths.

### 2. Centralized Target Version Resolution (`resolve_target_version`)
**Files**:
- `cargo-pgrx/src/command/migrate/version.rs`
- `cargo-pgrx/src/command/migrate/assemble.rs`
- `cargo-pgrx/src/command/migrate/info.rs`

Added `pub fn resolve_target_version`:
- If an explicit target version was passed on the CLI (`TARGET_VERSION`), it is cleaned and used directly.
- If `TARGET_VERSION` is omitted:
  - It inspects `sql/` for existing release targets and parses `manifest_version` from `Cargo.toml`.
  - If `manifest_version > latest_existing_sql_target`: uses `manifest_version` (normal active development flow).
  - If `manifest_version <= latest_existing_sql_target`: derives the next ephemeral target version (`latest_existing + 1`, e.g. `0.25.10`) using `derive_ephemeral_target_version`.
  - If `sql/` has no existing upgrade scripts: falls back to `manifest_version`.

Both `assemble` and `info` now share this centralized logic.

---

## Verification

1. **Unit Tests**:
   - `cargo test --bin cargo-pgrx command::migrate`: All 28 tests pass.
   - Added `resolves_target_version_correctly` in `version.rs`.
   - Added `handle_unreleased_fragments_copies_base_schema_when_present` in `mod.rs`.
   - Added `build_migrate_plan_defaults_target_version_when_manifest_is_older` in `info.rs`.

2. **Integration Verification**:
   - Tested against PostgreSQL 18.6 with `pg_search`.
   - `cargo pgrx install --package pg_search` successfully installs:
     - `pg_search--0.25.9--0.25.10.sql` (assembled upgrade script)
     - `pg_search--0.25.10.sql` (installed base schema)
     - `pg_search.control` (`default_version = '0.25.10'`)
   - `CREATE EXTENSION pg_search CASCADE;` in PostgreSQL succeeds cleanly without duplicate object errors.
