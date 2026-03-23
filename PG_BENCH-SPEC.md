# PG Bench Specification

## Status

Draft, iteration 1.

This document captures the current working design for adding in-process, Criterion-driven
benchmarks to `pgrx`.

It records:

- decisions we have already made
- implementation direction we currently prefer
- open questions we still need to settle before or during implementation

## Goals

- Add a `#[pg_bench]` macro for authoring in-process PostgreSQL benchmarks in `pgrx`.
- Use Criterion as the statistical benchmarking engine.
- Run benchmarks against the same long-lived, pgrx-managed PostgreSQL instance used by
  `cargo pgrx run`, not the ephemeral cluster used by `cargo pgrx test`.
- Store benchmark results in a managed database named `$extname_benches` by default.
- Expose SQL tables and views for later inspection and analysis of benchmark history.
- Collect environment metadata with each benchmark invocation, including PostgreSQL settings,
  git revision information, timestamps, and build metadata.
- Treat each `cargo pgrx bench` invocation as a named benchmark group/run that can be compared
  against a specific prior group.
- Keep the timed benchmark loop inside the PostgreSQL backend process.

## Non-Goals For MVP

- Do not implement a DBI-style external SQL driver benchmark tool.
- Do not make `samply` profiling a required part of the initial implementation.
- Do not attempt to benchmark true top-level transaction commit cost in the MVP.
- Do not install benchmark SQL into ordinary extension builds unless the `pg_bench` feature is
  explicitly enabled.

## High-Level Model

The design is intentionally closer to `#[pg_test]` than to `cargo pgrx regress`, but only in the
sense that benchmarked code executes inside the backend process.

It is intentionally unlike `cargo pgrx test` in one major way:

- `cargo pgrx test` boots and manages its own private test cluster
- `cargo pgrx bench` will reuse the normal pgrx-managed PostgreSQL instance used by
  `cargo pgrx run`

`cargo pgrx regress` is relevant only as precedent that `pgrx` already manages special-purpose
databases with generated names.

## User-Facing Authoring Model

Benchmarks live in a dedicated schema module named `benches`, gated by the `pg_bench` feature.

Current preferred shape:

```rust
#[cfg(feature = "pg_bench")]
#[pg_schema]
mod benches {
    use pgrx::prelude::*;
    use pgrx_bench::{Bencher, BatchSize, black_box};

    fn setup_uuid_fixture() {
        Spi::run("TRUNCATE bench_inputs").unwrap();
        Spi::run(
            "INSERT INTO bench_inputs
             SELECT gen_random_uuid()::text
             FROM generate_series(1, 10000)"
        )
        .unwrap();
    }

    #[pg_bench(setup = setup_uuid_fixture, transaction = "shared")]
    fn parse_uuid(b: &mut Bencher) {
        b.iter(|| {
            crate::parse_uuid(black_box("550e8400-e29b-41d4-a716-446655440000"))
        });
    }

    #[pg_bench(setup = setup_uuid_fixture, transaction = "subtransaction_per_batch")]
    fn parse_uuid_from_table(b: &mut Bencher) {
        b.iter_batched(
            || {
                Spi::get_one::<String>("SELECT input FROM bench_inputs LIMIT 1")
                    .unwrap()
                    .unwrap()
            },
            |input| crate::parse_uuid(black_box(&input)),
            BatchSize::SmallInput,
        );
    }
}
```

## Feature Model

The `pg_bench` feature is required.

Reason:

- without a feature gate, benchmark SQL and helper code would be included in ordinary installed
  and packaged extension builds
- that is undesirable for normal release artifacts

Current direction:

- `cargo pgrx bench` automatically enables `pg_bench`
- users gate the entire benchmark module with `#[cfg(feature = "pg_bench")]`
- benchmark code is only expected to exist under `mod benches`
- `#[pg_bench]` is only supported inside the `benches` module

This feature should be as unobtrusive as possible for users:

- attach the feature gate to the module, not to unrelated code
- avoid requiring users to thread `cfg(feature = "pg_bench")` all over the crate

## Cargo / Dependency Model

Current preferred Cargo shape for generated projects:

```toml
[features]
pg13 = ["pgrx/pg13", "pgrx-tests/pg13", "pgrx-bench?/pg13"]
pg14 = ["pgrx/pg14", "pgrx-tests/pg14", "pgrx-bench?/pg14"]
pg15 = ["pgrx/pg15", "pgrx-tests/pg15", "pgrx-bench?/pg15"]
pg16 = ["pgrx/pg16", "pgrx-tests/pg16", "pgrx-bench?/pg16"]
pg17 = ["pgrx/pg17", "pgrx-tests/pg17", "pgrx-bench?/pg17"]
pg18 = ["pgrx/pg18", "pgrx-tests/pg18", "pgrx-bench?/pg18"]
pg_test = []
pg_bench = ["dep:pgrx-bench"]

[dependencies]
pgrx = "=..."
pgrx-bench = { version = "=...", optional = true }

[dev-dependencies]
pgrx-tests = "=..."
```

`pgrx-bench` must be an optional normal dependency, not only a dev-dependency, because benchmark
code is compiled into the extension shared library when benching.

## CLI

## `cargo pgrx bench`

Add a new command:

```text
cargo pgrx bench
```

High-level behavior:

- auto-enable `pg_bench`
- default to `--release`
- reuse the managed PostgreSQL instance lifecycle from `cargo pgrx run`
- create or reuse `$extname_benches`
- install the bench-enabled extension build into that database
- discover and execute all available `#[pg_bench]` entrypoints
- persist results and print a final comparison report

### Default Database Name

Unless explicitly overridden:

```text
$extname_benches
```

### Suggested CLI Options

Initial expected options:

- `--group-name <name>`
- `--compare-group <name>`
- `--dbname <name>`
- `--resetdb`
- `--package <pkg>`
- `--manifest-path <path>`
- `--profile <name>`
- `--release`
- `--features ...`
- `--no-default-features`
- `--all-features`
- `--postgresql-conf key=value`
- `--list`
- `--json`
- benchmark name filter(s)

### Group Naming

Every `cargo pgrx bench` invocation is considered a run group.

Requirements:

- every group has a unique name
- user may provide `--group-name`
- otherwise one is auto-generated

Suggested autogenerated form:

```text
YYYYMMDD_HHMMSS_<short-githash>
```

### Comparison Group

Users may specify:

```text
--compare-group <name>
```

Semantics:

- if provided, use that exact prior run group as the baseline
- if not provided, compare against the most recent prior completed group by default
- if no prior group exists, run without comparison
- if an explicit comparison group cannot be found, error

The chosen baseline should be stored on the new run group row, so both the CLI output and SQL
views use the same comparison target later.

## Warnings In Other Commands

If users explicitly enable `pg_bench` during ordinary workflows such as:

- `cargo pgrx run`
- `cargo pgrx install`
- `cargo pgrx package`

then `cargo-pgrx` should warn loudly, in red UI output, but should not error.

Reason:

- shipping or interactively testing a build with benchmarks enabled may be a reasonable user
  decision

Suggested warning text:

```text
WARNING: building with feature `pg_bench`
benchmark functions and helper dependencies will be included in this build
this is usually not intended for packaged releases
```

## Macro Design

Add a new macro:

```rust
#[pg_bench(...)]
```

Current expected supported arguments:

- `setup = path`
- `transaction = "shared" | "subtransaction_per_batch" | "subtransaction_per_iteration"`
- Criterion tuning knobs such as:
  - `sample_size = N`
  - `measurement_time_ms = N`
  - `warm_up_time_ms = N`
  - `nresamples = N`
  - `noise_threshold = F`
  - `significance_level = F`

### Setup Function

We do not currently want a global crate-level setup hook.

Instead:

- each benchmark may specify its own setup function
- setup is referenced as a Rust path, not as a string
- setup functions are plain Rust functions, not specially annotated in MVP

Example:

```rust
#[pg_bench(setup = setup_uuid_fixture, transaction = "shared")]
fn parse_uuid(b: &mut Bencher) { ... }
```

### Setup Semantics

- setup runs once per benchmark invocation
- setup runs before Criterion warmup
- setup is not timed
- setup is distinct from Criterion per-batch or per-iteration input preparation

If a benchmark needs fresh input for each batch or iteration, that belongs in the Criterion-style
batching API, not the bench-level `setup`.

## Runtime / Execution Model

Benchmarks must be in-process.

That means:

- the host runner may invoke a SQL entrypoint once per benchmark
- the timed loop itself must execute inside the PostgreSQL backend process

The expected implementation shape is:

- `#[pg_bench]` generates a hidden SQL-callable wrapper entrypoint
- the host runner discovers these entrypoints
- the host runner invokes each benchmark once
- the benchmark wrapper executes the timed loop inside the backend
- the wrapper returns a structured result payload to the host runner

## Managed Instance Model

`cargo pgrx bench` must reuse the same pgrx-managed PostgreSQL instance used by
`cargo pgrx run`.

This means:

- use the normal managed data directory and server lifecycle
- do not boot a private temporary cluster
- create or reuse the benchmark database on that managed instance

## Extension Refresh Model

`cargo pgrx bench` must refresh the extension definition inside `$extname_benches` so that
SQL-visible changes are available for the benchmark run.

Current required behavior:

1. install the newly built extension artifacts into the managed PostgreSQL instance
2. connect to `$extname_benches`
3. refresh the extension with a:
   - `DROP EXTENSION ...`
   - `CREATE EXTENSION ...`
   dance

Reason:

- benchmarked SQL entrypoints and any other SQL-facing extension changes must match the current
  Rust build
- simply reusing an already-installed extension in the benchmark database is not sufficient

### Critical Constraint

The benchmark runner's own historical metadata objects must survive this refresh dance.

Therefore:

- benchmark history tables must not be owned by the extension
- benchmark history views must not be owned by the extension
- benchmark history objects must not be created by the extension's generated SQL
- benchmark history objects must instead be created and managed directly by `cargo pgrx bench`

### Ownership Boundary

There should be a strict ownership split:

- extension-owned objects
  - normal extension SQL objects
  - benchmark wrapper functions generated from `#[pg_bench]`
  - anything inside the user-authored `benches` schema/module

- runner-owned objects
  - persistent benchmark metadata tables
  - persistent benchmark analysis views
  - any helper objects needed solely to store benchmark history

Current preferred runner-owned schema:

```text
pgrx_bench
```

This schema must remain outside extension ownership.

### Dependency Rule

Runner-owned persistent objects should avoid dependencies on extension-owned objects.

That means:

- do not store extension object OIDs as durable identifiers unless clearly treated as ephemeral
- prefer durable textual identifiers such as schema name, function name, benchmark name, setup
  function name, git hash, and run group name
- do not create persistent views that depend on extension-owned SQL functions or types

This reduces the risk that `DROP EXTENSION` will fail or unexpectedly cascade into historical
benchmark metadata.

## Build / Profile Expectations

Benchmark builds should default to release mode.

Current expected behavior:

- `cargo pgrx bench` defaults to `--release`
- custom profiles may be allowed with `--profile`

For future profiling support, we likely also need a profile that keeps debug symbols, for example:

- release-like optimization
- `debug = true`

## Transaction Semantics

This is one of the most important design areas.

The benchmark code runs inside a PostgreSQL backend call, so Criterion warmup and measurement
occur inside the backend process. We need explicit transaction semantics for mutating workloads.

### Top-Level Benchmark Invocation

Each benchmark invocation is expected to run inside one top-level transaction started by the host
runner.

Within that invocation, `#[pg_bench]` chooses how work is isolated during timing.

### Modes

#### `shared`

- setup, warmup, and measured work share one ambient transaction
- lowest overhead
- best for pure Rust or read-mostly workloads

#### `subtransaction_per_batch`

- each measured batch runs in its own internal subtransaction
- good default for mutating benchmarks
- lower overhead than per-iteration subtransactions

#### `subtransaction_per_iteration`

- each iteration runs in its own internal subtransaction
- cleanest isolation
- highest overhead

### Post-Run Rollback Model

Current preferred persistence model:

1. host opens a transaction
2. host invokes the benchmark wrapper
3. benchmark setup + warmup + measurement run inside that transaction
4. wrapper returns a structured result payload
5. host rolls back the benchmark transaction
6. host opens a new transaction
7. host persists the returned results into benchmark metadata tables
8. host commits the persistence transaction

This gives us:

- reproducible benchmark isolation
- no accumulation of benchmark fixture junk in `$extname_benches`
- persistent benchmark history

This persistence transaction is separate from the extension refresh lifecycle described above.
The extension may be dropped and recreated at the start of a `cargo pgrx bench` invocation, but
the persistent metadata schema must remain intact across invocations.

Important semantic note:

- `shared` means shared during timing
- it does not mean benchmark side effects persist after the run

### MVP Limitation

The MVP does not target true top-level `COMMIT` cost benchmarking.

Reason:

- internal subtransactions are feasible
- top-level transaction commit measurement is a separate design problem

## Criterion Integration Strategy

The benchmark engine should be Criterion-driven, but the API exposed to benchmark authors should
be a small `pgrx`-aware wrapper.

Current preferred direction:

- create a `pgrx-bench` crate
- expose `Bencher`, `BatchSize`, and `black_box`
- implement transaction-aware wrappers over Criterion timing loops
- use lower-level Criterion/custom-timing integration where necessary to place setup and
  subtransaction boundaries correctly

We do not currently intend to expose raw `Criterion` directly as the main benchmark authoring API.

Reason:

- we need control over transaction semantics
- we want a stable `pgrx`-friendly benchmark surface

## Reserved Schemas

Current expected schema/module conventions:

- user-authored benchmarks live in `#[pg_schema] mod benches`
- persistent benchmark metadata tables live in a harness schema such as `pgrx_bench`

This keeps:

- benchmark definitions separate from
- persistent benchmark history and analysis tables

## Persistent Storage Design

Persistent benchmark metadata should live in a dedicated schema, currently expected to be:

```text
pgrx_bench
```

This schema is runner-owned and must survive extension drop/recreate cycles.

### Table: `pgrx_bench.run_group`

One row per `cargo pgrx bench` invocation.

Columns:

- `id uuid primary key`
- `group_name text not null unique`
- `created_at timestamptz not null`
- `completed_at timestamptz`
- `status text not null`
- `compare_group_id uuid null references pgrx_bench.run_group(id)`
- `extname text not null`
- `extversion text`
- `pg_version_major int not null`
- `profile_name text not null`
- `cargo_features text[] not null`
- `command_line text not null`
- `os text`
- `arch text`
- `rustc_version text`
- `cargo_version text`
- `pgrx_version text`
- `cargo_pgrx_version text`
- `git_commit text`
- `git_branch text`
- `git_dirty boolean not null default false`
- `git_describe text`
- `extra_metadata jsonb not null default '{}'::jsonb`

Status values are expected to include:

- `running`
- `completed`
- `partial`
- `failed`

### Table: `pgrx_bench.run_group_pg_setting`

Snapshot of PostgreSQL settings for a run group.

Columns:

- `group_id uuid not null references pgrx_bench.run_group(id)`
- `name text not null`
- `setting text`
- `unit text`
- `source text`
- `sourcefile text`
- `sourceline int`
- `boot_val text`
- `reset_val text`
- `pending_restart boolean`

Primary key:

- `(group_id, name)`

This should snapshot `pg_settings`, not only CLI overrides.

### Table: `pgrx_bench.benchmark_case`

Stable benchmark definition metadata.

Columns:

- `id bigserial primary key`
- `schema_name text not null`
- `bench_name text not null`
- `function_name text not null`
- `setup_function text`
- `transaction_mode text not null`
- `source_file text`
- `source_line int`

Unique key:

- `(schema_name, bench_name)`

### Table: `pgrx_bench.benchmark_run`

One row per benchmark case executed within a run group.

Columns:

- `id bigserial primary key`
- `group_id uuid not null references pgrx_bench.run_group(id)`
- `case_id bigint not null references pgrx_bench.benchmark_case(id)`
- `status text not null`
- `error_text text`
- `started_at timestamptz not null`
- `finished_at timestamptz`
- `criterion_config jsonb not null`
- `raw_result jsonb not null`

Unique key:

- `(group_id, case_id)`

Expected status values:

- `ok`
- `failed`
- `skipped`

### Table: `pgrx_bench.benchmark_estimate`

Normalized Criterion estimates.

Columns:

- `benchmark_run_id bigint not null references pgrx_bench.benchmark_run(id)`
- `estimate_kind text not null`
- `point_estimate_ns double precision not null`
- `standard_error_ns double precision`
- `confidence_level double precision`
- `ci_lower_bound_ns double precision`
- `ci_upper_bound_ns double precision`

Primary key:

- `(benchmark_run_id, estimate_kind)`

Expected estimate kinds:

- `mean`
- `median`
- `median_abs_dev`
- `slope`
- `std_dev`

### Table: `pgrx_bench.benchmark_sample`

Raw sample timing data.

Columns:

- `benchmark_run_id bigint not null references pgrx_bench.benchmark_run(id)`
- `sample_index int not null`
- `iteration_count bigint not null`
- `elapsed_ns double precision not null`

Primary key:

- `(benchmark_run_id, sample_index)`

### Table: `pgrx_bench.benchmark_throughput`

Optional throughput metadata.

Columns:

- `benchmark_run_id bigint primary key references pgrx_bench.benchmark_run(id)`
- `kind text not null`
- `value double precision not null`

### Table: `pgrx_bench.artifact`

Store artifacts associated with a benchmark run.

Columns:

- `id bigserial primary key`
- `benchmark_run_id bigint not null references pgrx_bench.benchmark_run(id)`
- `artifact_kind text not null`
- `media_type text not null`
- `payload bytea`
- `payload_json jsonb`
- `metadata jsonb not null default '{}'::jsonb`

This is the expected landing place for future profiling artifacts such as `samply`.

## Environmental Metadata

The benchmark system should capture, at minimum:

- timestamp
- extname
- extension version
- PostgreSQL major version
- selected build profile
- enabled Cargo features
- full command line
- OS
- architecture
- `rustc` version
- `cargo` version
- `pgrx` version
- `cargo-pgrx` version
- current git commit hash
- current git branch
- whether the git tree is dirty
- `git describe` if available
- full `pg_settings` snapshot

The desired behavior is to make historical benchmark rows self-describing enough for later
analysis.

## Views

The following views are currently desired:

### `pgrx_bench.v_run_group_summary`

One row per run group with high-level metadata and chosen comparison target.

### `pgrx_bench.v_run_group_nondefault_settings`

Filter settings down to the interesting subset, such as non-default or file-backed overrides.

### `pgrx_bench.v_primary_estimate`

One row per benchmark run selecting the estimate used by default for reporting.

Current preference:

- use `slope` if present
- otherwise use `mean`

### `pgrx_bench.v_group_results`

Join run group, benchmark case, and primary estimate for easy reporting.

### `pgrx_bench.v_default_comparison`

Compare each benchmark result against the baseline stored in `compare_group_id`.

It should classify results into categories such as:

- `new`
- `missing`
- `faster`
- `slower`
- `unchanged`

and report a percentage delta.

## Final CLI Report

At the end of `cargo pgrx bench`, the final report should compare the current run group against:

- the explicitly chosen `--compare-group`, if provided
- otherwise the default prior group

This report should not always compare only to "the previous benchmark run" if the user chose a
different baseline.

## Packaging / Release Expectations

Ordinary packaged extensions should not include benchmark SQL by default.

This is the main reason the `pg_bench` feature exists.

However:

- enabling `pg_bench` in `run/install/package` is allowed
- `cargo-pgrx` should warn, not block

## Profiling And `samply`

Future goal:

- optional `samply` support at runtime
- store resulting profiling artifacts in the `_benches` database
- provide initial SQL analysis over those artifacts

Current status:

- deferred until after MVP

Expected implications:

- likely need a profile with optimization plus debug symbols
- likely need additional artifact metadata tables or analysis views
- Linux and macOS support are both desired

## Likely Implementation Pieces

The current implementation plan is expected to involve:

1. new `cargo pgrx bench` command in `cargo-pgrx`
2. new `pgrx-bench` crate
3. new `#[pg_bench]` macro in `pgrx-macros`
4. template updates for the `pg_bench` feature and optional dependency
5. benchmark discovery and execution machinery
6. persistent metadata schema in `$extname_benches`
7. final CLI comparison reporting
8. later profiling integration

## Open Questions

The following questions are still open enough that implementation may refine them:

- exact `#[pg_bench(...)]` grammar for all Criterion tuning knobs
- exact wire format of `raw_result jsonb`
- whether benchmark entrypoints should live in a reserved hidden SQL schema or directly in
  `benches`
- the final naming scheme for autogenerated run groups
- exact baseline selection heuristics when `--compare-group` is omitted
- how much git/environment probing should be best-effort vs required
- whether some PostgreSQL settings should be stored in both normalized and raw JSON form
- exact format and ingestion path for future `samply` artifacts
- whether we eventually want a separate mode for true top-level commit-cost benchmarks

## Summary Of Current Decisions

The most important settled decisions so far are:

- benchmarks are in-process
- Criterion is the intended engine
- `cargo pgrx bench` reuses the managed `cargo pgrx run` PostgreSQL instance
- benchmark code is gated by a `pg_bench` feature
- the feature gate should be attached to `mod benches`
- setup is per-benchmark via `setup = path`
- setup runs once and is untimed
- benchmark side effects should be rolled back after each run
- benchmark results persist in `$extname_benches`
- each CLI invocation is a named run group
- users can choose a specific comparison group
- ordinary `run/install/package` with `pg_bench` should warn, not error
