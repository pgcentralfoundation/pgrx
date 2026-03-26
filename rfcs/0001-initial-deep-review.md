# RFC 0001 Initial Deep Review

## Scope

- Branch reviewed: `wip-one-compile-please`
- Compared against: `origin/develop..HEAD` at `393d4cda`
- Primary design documents:
  - `rfcs/0001-single-pass-schema-generation.md`
  - `rfcs/0001-review-findings.md`
- Changeset size at review time: 155 files changed, 5691 insertions, 2629 deletions

This review is focused on implementation readiness and semantic fidelity to the RFCs, not
on style. The branch is a substantial and coherent end-to-end rework of schema generation.
The old `pgrx_embed` and `__pgrx_internals_*` pipeline is genuinely gone from live code,
and the remaining references are historical documentation only. That part of the branch is
in good shape.

The remaining concerns are concentrated in failure handling and validation boundaries.
The biggest live issue is that `cargo pgrx schema` still silently succeeds with an empty
schema when `.pgrx_schema` cannot be found. That violates the RFC's fail-fast story and is
the only P0 I found.

## Status On Current Branch

This document is a historical snapshot. On the current `wip-one-compile-please`
branch:

- missing `.pgrx_schema` is fixed
- `SetOfIterator` argument rejection is fixed
- the named result tags are in place
- schema stripping is no longer part of the install path
- unresolved `SCHEMA_KEY` fallback is fixed through explicit declared-type
  resolution, while `TYPE_ORIGIN` stays explicit on `SqlTranslatable`

## Executive Summary

- The branch successfully implements the main architectural move from runtime symbol
  execution to embedded linker-section metadata.
- The previously reviewed `SCHEMA_KEY` bug in `pgrx_resolved_type!` is fixed and covered by
  a regression test.
- The old pipeline removal looks complete in production code.
- Several findings from `rfcs/0001-review-findings.md` are still live:
  - missing-section handling is still silent
  - `SetOfIterator` still lies about argument validity
  - `generate_schema_implicit()` still carries dead parameters
  - the section encoding still uses unnamed `1` / `2` result tags
  - schema stripping still introduces undocumented runtime tool dependencies
- One prior finding now looks intentional rather than mistaken:
  - `Vec<T>` / `Array<T>` now reject `Skip`-mapped element types. I would treat this as an
    accepted compatibility break, but the RFC should say so explicitly.

## Review-Findings Status Matrix

| Prior finding | Status | Notes |
|---|---|---|
| 1. RFC says NDJSON, impl uses binary | Open | Still a doc/impl mismatch. The implementation is binary `EntryWriter` / `EntryReader`, not NDJSON. |
| 2. `pgrx_resolved_type!` missing `module_path!()` | Fixed | `pgrx/src/lib.rs` now expands to `concat!(module_path!(), "::", stringify!($ty))`, and `pgrx-tests/src/tests/complex.rs` has a regression test. |
| 3. `Vec<T>` / `Array<T>` reject `Skip`-mapped types | Open, but likely intentional | This is still a behavior change. I do not view it as a blocker if it is documented as a deliberate tightening. |
| 4. `SetOfIterator::ARGUMENT_SQL` lost error guard | Open | Still present and user-visible. |
| 5. Dead parameters in `generate_schema_implicit()` | Open | `_cargo` and `_features_arg` still survive from the deleted pipeline. |
| 6. Missing `.pgrx_schema` yields silent empty schema | Open | Still the highest-severity issue. |
| 7. Magic number discriminants in section encoding | Open | Still present. |
| 8. New `objcopy` / `codesign` runtime dependencies | Open | Still present and still undocumented. |

## Changeset Scope

Top-level areas touched by the branch:

- `pgrx-sql-entity-graph`
  - new section encoder/decoder
  - `SqlTranslatable` const metadata conversion
  - graph matching moved from `TypeId` to `SCHEMA_KEY`
- `cargo-pgrx`
  - schema generation now reads `.pgrx_schema` from the compiled shared object
  - install path strips the embedded schema section
  - new object parsing helper for Mach-O and ELF section access
- `pgrx`
  - built-in `SqlTranslatable` impls converted to const metadata
  - `pgrx_resolved_type!` introduced as the canonical manual identity helper
  - old embed / type-id infrastructure removed
- `pgrx-tests` and `pgrx-examples`
  - template and fixture updates to remove `pgrx_embed`
  - regression coverage for the schema-key fix

Production hotspots reviewed in detail:

- `cargo-pgrx/src/command/schema.rs`
- `cargo-pgrx/src/object_utils.rs`
- `cargo-pgrx/src/command/install.rs`
- `pgrx-sql-entity-graph/src/section.rs`
- `pgrx-sql-entity-graph/src/metadata/sql_translatable.rs`
- `pgrx-sql-entity-graph/src/pgrx_sql.rs`
- `pgrx-sql-entity-graph/src/pg_extern/*`
- `pgrx-sql-entity-graph/src/extension_sql/*`
- `pgrx/src/iter.rs`
- `pgrx/src/lib.rs`

## Technique Selection

| Technique | Used | Why |
|---|---|---|
| T1 Call Graph Slice | Yes | The branch replaces the entire schema-generation pipeline, so boundary tracing matters more than isolated hunks. |
| T2 Interaction Flow Traces | Yes | Needed to reason about happy-path behavior, missing-section handling, and invalid signature handling. |
| T3 Diff-Structural Analysis | Yes | The branch changes signatures, control flow, data representation, and artifact format. |
| T4 Invariant Analysis | Yes | The RFC makes explicit promises about fail-fast behavior and compile-time validation. |
| T5 Ownership / Responsibility | Yes | The change crosses proc-macro emission, CLI schema generation, object parsing, and install-time artifact mutation. |
| T6 Failure Mode Enumeration | Yes | The biggest remaining risks are failure-path problems, not steady-state logic. |
| T7 Diff Minimality | Yes | This is a sweeping removal/refactor; it is worth checking what cleanup is still incomplete. |

## T3: Diff-Structural Analysis

### Signature delta

Key signature and interface changes:

- Added `SqlTranslatable::{SCHEMA_KEY, TYPE_ORIGIN, ARGUMENT_SQL, RETURN_SQL}` as the
  new source of truth for schema metadata.
- Added const-friendly metadata types:
  - `SqlMappingRef`
  - `ReturnsRef`
- Added section encoding/decoding surface in `pgrx-sql-entity-graph/src/section.rs`.
- Added `cargo-pgrx/src/object_utils.rs` for object-file section access and Mach-O stripping.
- Removed the live dependency of schema generation on:
  - `pgrx_embed`
  - symbol scanning for `__pgrx_internals_*`
  - runtime `TypeId` matching in the SQL graph builder

### Control-flow delta

`cargo pgrx schema` now flows:

1. build the extension shared object once
2. read the produced shared object
3. extract `.pgrx_schema`
4. decode entities
5. build `PgrxSql`
6. write SQL

That is the intended architecture from the RFC.

The key control-flow regression is the missing-section branch in
`cargo-pgrx/src/command/schema.rs:237-244`: it falls back to `Vec::new()` instead of
erroring, even though the RFC pseudocode explicitly errors on missing `.pgrx_schema`.

### Type-flow delta

Type identity is now carried by `SCHEMA_KEY` instead of `TypeId`.

This change is structurally sound. The important data path is:

- derive/manual `SqlTranslatable` impl
- `UsedType::section_writer_tokens()`
- embedded `FunctionMetadataTypeEntity`
- `decode_entities()`
- `PgrxSql` matching by `schema_key`

The branch mostly preserves that flow correctly. The notable regression is that
`SetOfIterator<'_, T>` now forwards `ARGUMENT_SQL` from `T`, which widens the accepted
type surface relative to the old implementation even though `SetOfIterator` is still only
semantically valid in return position.

## T1: Call Graph Slice

### Schema generation path

```text
Schema::execute() [fan-in: CLI]
  -> generate_schema_for_cli()              domain object / config
     -> first_build()                       external I/O: cargo build
     -> generate_schema_implicit()          domain object
        -> load_section_entities()          external I/O: filesystem
           -> schema_section_data()         parser boundary
              -> slice_arch32/64()          parser boundary
              -> macho_schema_section_data()
              -> schema_section_data_from_object()
           -> decode_entities()             parser boundary
        -> PgrxSql::build()                 domain object
        -> to_file() / write()              external I/O
```

Semantically important edges:

- `schema_section_data()` decides whether the branch has any schema entities at all.
- `decode_entities()` is the only place that validates the embedded format.
- `PgrxSql::build()` assumes the entity set is complete and coherent.

The bad edge is:

- `schema_section_data() -> None`
  - currently mapped to `Vec::new()`
  - should be an error boundary, not a normal control-flow edge

### Invalid `SetOfIterator` argument path

```text
#[pg_extern] parse
  -> PgExternArgument::build_from_pat_type()        accepts any parsed Rust type
     -> UsedType::new()                             normalizes wrappers
     -> section_writer_tokens()                     embeds <T as SqlTranslatable>::ARGUMENT_SQL
        -> SetOfIterator<'_, T>::ARGUMENT_SQL       currently delegates to T
SQL rendering
  -> PgExternEntity::to_sql()
     -> match arg.used_ty.metadata.argument_sql
        -> Err(err) only here, during SQL generation
```

This is a real validation-boundary regression. The proc macro path does not reject
`SetOfIterator` in argument position, so the trait metadata needs to.

### Install-time strip path

```text
install_extension()
  -> copy_file(shared object)
  -> strip_schema_section()
     -> schema_section_present()
        -> schema_section_data()
     -> strip_macho_schema_section() or strip_non_macho_schema_section()
        -> codesign                      external I/O / tool boundary
        -> llvm-objcopy or objcopy      external I/O / tool boundary
```

The responsibility split is reasonable, but the operational cost changed: install now
depends on host tooling that was not previously required.

## T2: Interaction Flow Traces

### Trace: Happy-path single-pass schema generation

1. `generate_schema_for_cli()` builds the library once via `first_build()`.
   - PRE: manifest and feature resolution are already complete.
   - POST: shared object exists in the target directory.
2. `generate_schema_implicit()` finds the control file and shared object name.
3. `load_section_entities()` reads the shared object and extracts `.pgrx_schema`.
4. `decode_entities()` reconstructs typed entities.
5. `PgrxSql::build()` assembles the dependency graph from:
   - `ExtensionRoot`
   - decoded linker-section entities
6. `to_file()` or `write()` emits SQL.

Result: this is structurally aligned with RFC 0001. The one-pass design itself is working
the way the RFC intends.

### Trace: Missing `.pgrx_schema` section

1. `generate_schema_implicit()` calls `load_section_entities()`.
2. `load_section_entities()` reads the shared object and calls `schema_section_data()`.
3. `schema_section_data()` returns `Ok(None)` if:
   - the section is absent
   - fat-binary slicing cannot find the current host architecture
4. `load_section_entities()` takes the `else` branch and returns `Vec::new()`.
5. `generate_schema_implicit()` still pushes `ExtensionRoot`.
6. `PgrxSql::build()` succeeds with a graph containing only the control file.
7. SQL is written successfully, but it is effectively empty.

This is the clearest fail-fast violation in the branch. The RFC pseudocode at
`rfcs/0001-single-pass-schema-generation.md:333-349` says missing `.pgrx_schema` should
be an error. The implementation normalizes it into success.

### Trace: `SetOfIterator<i32>` as a function argument

1. `PgExternArgument::build_from_pat_type()` accepts the parsed argument type and calls
   `UsedType::new()`.
2. `UsedType::section_writer_tokens()` embeds
   `<SetOfIterator<'_, i32> as SqlTranslatable>::ARGUMENT_SQL`.
3. In `pgrx/src/iter.rs:83-89`, that value is `T::ARGUMENT_SQL`, so it becomes
   `Ok(SqlMappingRef::As("INT"))`.
4. The entity is emitted and later decoded successfully.
5. SQL generation reaches `PgExternEntity::to_sql()`, which only errors if the embedded
   metadata is already an `Err`.

Because the metadata says the argument is valid, the invalidity is not rejected at the
type-metadata boundary. This directly contradicts the intent recorded in
`rfcs/0001-review-findings.md:273-325`.

## T4: Invariant Analysis

### [PRESERVED] The old runtime symbol-execution pipeline is removed from production code

Evidence:

- no live references to `pgrx_embed`
- no live `find_and_compute_symbols()` / `second_build()` path
- no live `__pgrx_internals_*` production path

The branch does achieve the main cleanup objective of RFC 0001.

### [PRESERVED] Type identity now flows through `SCHEMA_KEY`

Evidence:

- `pgrx/src/lib.rs:382-386` defines `pgrx_resolved_type!()` with `module_path!()`
- `pgrx-tests/src/tests/complex.rs:37-50` uses it in a manual `SqlTranslatable` impl
- `pgrx-tests/src/tests/complex.rs:94-97` asserts the resulting key

Finding 2 from `rfcs/0001-review-findings.md` is fixed.

### [BROKEN] Missing section should fail fast

RFC evidence:

- `rfcs/0001-single-pass-schema-generation.md:336-337` explicitly errors if
  `.pgrx_schema` is missing

Implementation evidence:

- `cargo-pgrx/src/command/schema.rs:237-244` silently converts `None` to `Vec::new()`

Severity: P0. This is a fail-fast violation that can produce a silently empty extension
schema.

### [BROKEN] `SetOfIterator` must not be valid in argument position

RFC evidence:

- `rfcs/0001-single-pass-schema-generation.md:431-448` promises args-and-returns checks
  across normalized leaf types
- `rfcs/0001-review-findings.md:273-325` identifies the lost guard and recommends
  restoring `Err(ArgumentError::SetOf)`

Implementation evidence:

- `pgrx/src/iter.rs:83-89` still delegates `ARGUMENT_SQL` to `T`
- `pgrx-sql-entity-graph/src/pg_extern/argument.rs:44-61` does not reject the shape
  syntactically

Severity: P1. Invalid signatures are accepted deeper into the pipeline than intended.

### [IMPLICIT] Host architecture is assumed to be the correct schema-reader architecture

Evidence:

- `cargo-pgrx/src/object_utils.rs:21-35`
- `cargo-pgrx/src/object_utils.rs:408-434`

This is what makes cross-arch or mismatched universal-binary cases fall into the silent
empty-schema path instead of a targeted error.

Risk: Medium. It is mostly a cross-compilation / universal-binary problem, but it is
part of the missing-section failure mode.

## T5: Ownership and Responsibility Analysis

### Layer map

- Proc-macro emission layer:
  - `pgrx-sql-entity-graph`
- Shared metadata / identity layer:
  - `SqlTranslatable`, `UsedType`, section encoding
- CLI schema-generation layer:
  - `cargo-pgrx/src/command/schema.rs`
- Artifact inspection / mutation layer:
  - `cargo-pgrx/src/object_utils.rs`
  - `cargo-pgrx/src/command/install.rs`

### Responsibility shifts

1. `cargo-pgrx` now owns binary section extraction instead of symbol discovery and embed
   execution.
   - This shift is good and matches the RFC.

2. `cargo-pgrx install` now owns artifact schema stripping.
   - This is acceptable, but it introduces host-tool dependencies that need to be
     documented or degraded gracefully.

3. `generate_schema_implicit()` still carries `_cargo` and `_features_arg`.
   - This is a leftover ownership leak from the deleted second-build pipeline.
   - The function signature suggests responsibility it no longer has.

## T6: Failure Mode Enumeration

### 1. Missing `.pgrx_schema`

- Trigger:
  - old artifact
  - mis-linked artifact
  - wrong fat-binary slice
  - stripped artifact
- Current behavior:
  - silent empty SQL
- Correct behavior:
  - hard error

### 2. Invalid `SetOfIterator` argument

- Trigger:
  - user writes `#[pg_extern] fn f(arg: SetOfIterator<'_, i32>)`
- Current behavior:
  - metadata says argument is valid
  - failure occurs later during SQL generation, if at all
- Correct behavior:
  - `SqlTranslatable` should encode `Err(ArgumentError::SetOf)` immediately

### 3. Missing strip tools on Linux

- Trigger:
  - `llvm-objcopy` and `objcopy` both unavailable
- Current behavior:
  - install fails with guidance to use `--no-schema-strip`
- Risk:
  - operational friction in CI, Docker, and minimal environments

### 4. `codesign` failure on macOS

- Trigger:
  - stripped Mach-O cannot be re-signed ad hoc
- Current behavior:
  - install fails
- Risk:
  - local dev disruption, especially on locked-down systems

### 5. Build-graph failure in `PgrxSql::build()`

- Trigger:
  - malformed entity graph
- Current behavior:
  - `cargo-pgrx/src/command/schema.rs:202-203` panics via `expect("SQL generation error")`
- Risk:
  - degraded diagnostics relative to the RFC pseudocode, which used `?`

## T7: Diff Minimality

### Necessity

The major structural edits all belong:

- section encoding/decoding is necessary for one-pass schema generation
- `SqlTranslatable` const metadata is necessary for compile-time embedding
- `cargo-pgrx` object reading is necessary to consume the new format
- install-time stripping is a sensible cleanup step once metadata is embedded in runtime
  artifacts

### Completeness

The main implementation gap is not missing functionality. It is incomplete cleanup and
failure-policy tightening:

- missing-section branch still normalizes an impossible state into success
- dead parameters remain in `generate_schema_implicit()`
- docs do not explain the new strip-tool dependency
- the RFC still describes NDJSON instead of the binary format that shipped

### Coherence

The changeset is large but coherent. I do not think it is over-bundled. Most of the edits
are coupled by design. The branch would be harder to reason about if the pipeline removal,
const metadata transition, and schema reader were split apart without intermediate shims.

## Prioritized Findings

### 1. [P0] Missing `.pgrx_schema` still silently generates empty SQL

Files:

- `cargo-pgrx/src/command/schema.rs:224-244`
- `cargo-pgrx/src/object_utils.rs:21-35`
- `cargo-pgrx/src/object_utils.rs:408-434`

Why this matters:

- The RFC explicitly says missing `.pgrx_schema` should be an error.
- The current implementation returns an empty entity list and still emits SQL.
- Cross-arch fat-binary slice mismatches also collapse into this path.

Recommended fix:

- Replace the `Vec::new()` fallback with `bail!(...)`.
- Include the shared object path and a hint that the artifact may have been built by an
  incompatible pgrx or stripped incorrectly.

### 2. [P1] `SetOfIterator` argument validation regressed and now fails too late

Files:

- `pgrx/src/iter.rs:83-89`
- `pgrx-sql-entity-graph/src/pg_extern/argument.rs:44-61`
- `pgrx-sql-entity-graph/src/pg_extern/entity/mod.rs:121-184`

Why this matters:

- `SetOfIterator` is only meaningful in return position.
- The old implementation encoded that fact in `argument_sql()`.
- The new implementation delegates `ARGUMENT_SQL` to `T`, so invalid argument types can
  make it all the way into emitted metadata.

Recommended fix:

- Change `SetOfIterator<'_, T>::ARGUMENT_SQL` to `Err(ArgumentError::SetOf)`.

### 3. [P2] `generate_schema_implicit()` still carries dead second-build parameters

Files:

- `cargo-pgrx/src/command/schema.rs:157-181`

Why this matters:

- `_cargo` and `_features_arg` are construction work that no longer affects behavior.
- They are now a false interface contract.

Recommended fix:

- Delete both parameters and simplify callers.

### 4. [P2] RFC 0001 still describes NDJSON, but the shipped format is binary

Files:

- `rfcs/0001-single-pass-schema-generation.md:719-738`
- `pgrx-sql-entity-graph/src/section.rs`

Why this matters:

- The RFC is now the wrong guide for anyone debugging section contents or extending the
  encoder.
- The implementation is length-prefixed binary with explicit tags, not NDJSON.

Recommended fix:

- Update the RFC to describe the actual binary encoding and decoding flow.

### 5. [P2] Section result tags still use unnamed `1` / `2` discriminants

Files:

- `pgrx-sql-entity-graph/src/section.rs:318-343`
- `pgrx-sql-entity-graph/src/section.rs:481-508`

Why this matters:

- This is the only part of the section encoding that does not use named constants.
- It increases the chance of reader/writer drift during future edits.

Recommended fix:

- Introduce named constants such as `RESULT_OK` and `RESULT_ERR`.

### 6. [P2] Install-time schema stripping adds undocumented host-tool dependencies

Files:

- `cargo-pgrx/src/command/install.rs:310-383`

Why this matters:

- Linux installs now require `llvm-objcopy` or `objcopy`, unless the user discovers
  `--no-schema-strip`.
- macOS installs now depend on `codesign` succeeding after in-place mutation.
- I did not find corresponding documentation in the repo.

Recommended fix:

- Document the dependency in `cargo-pgrx` user docs.
- Consider warning-and-skip behavior on Linux instead of hard failure.

### 7. [P2] `PgrxSql::build()` failures panic instead of returning a normal CLI error

Files:

- `cargo-pgrx/src/command/schema.rs:202-203`
- `rfcs/0001-single-pass-schema-generation.md:349`

Why this matters:

- The RFC pseudocode propagates `PgrxSql::build(...)?`.
- The implementation uses `expect("SQL generation error")`.
- That loses context and turns graph-construction failures into a panic boundary.

Recommended fix:

- Replace `expect(...)` with `?` and wrap with CLI-level context.

## Suggested Fix Order

1. Fix missing-section handling.
2. Restore `SetOfIterator` argument rejection.
3. Remove dead parameters and panic-based error handling in `schema.rs`.
4. Update the RFC to describe binary section encoding.
5. Clean up section discriminant constants.
6. Document or relax install-time strip dependencies.

## Bottom Line

The branch is close. The core architectural shift is sound, and the previously most
important identity bug is already fixed. I would not merge it as-is because of the silent
empty-schema path, and I would want the `SetOfIterator` validation regression addressed at
the same time. After those two, the remaining issues are cleanup and documentation, not a
fundamental problem with the design.
