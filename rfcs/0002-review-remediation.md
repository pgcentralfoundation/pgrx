# RFC 0002: Review Remediation for Single-Pass Schema Generation

## Summary

This document records the follow-up work for the branch review captured in:

- `rfcs/0001-initial-deep-review.md`
- `rfcs/0001-deep-review-findings.md`

The two blocking questions from that review are resolved on this branch:

1. `extension_sql!(creates = [Type(T)]/[Enum(T)])` should not flatten extension-owned
   types into `BuiltinType`.
   Resolution: declared types now resolve to the declaring `extension_sql!()` graph node.
   That preserves schema identity and lets SQL rendering use the graph instead of a
   builtin placeholder.
2. `TYPE_ORIGIN` should be encoded for declared type and enum entries.
   Resolution: declared type metadata now stores `TYPE_ORIGIN` in memory and in the
   binary schema section format.

The result is closer to the RFC model from
`rfcs/0001-single-pass-schema-generation.md`: dependency resolution now has a single
schema-emitting target for declared types, and the metadata needed for future
resolution decisions is carried forward instead of being recomputed.

## Decisions

### Preserve declared-type identity in the graph

The review raised two possible directions:

- keep flattening declared types into `BuiltinType` and teach more SQL renderers to
  compensate
- preserve declared-type identity in the graph

This branch chooses the second path.

Why:

- it matches the RFC's type-matching algorithm
- it avoids a second class of "sort of schema-owned, sort of builtin" nodes
- it keeps schema prefix lookup on the same graph path used for derived types and enums
- it avoids repeating special-case resolution logic in aggregate rendering

Implementation effect:

- `initialize_resolved_type()` now treats matching `extension_sql!()` declarations as
  schema-emitting graph targets, not as a reason to create a builtin placeholder
- `connect_resolved_type()` now connects directly to that `extension_sql!()` node
- `SqlGraphEntity::schema_matches()` recognizes declared types carried by a
  `CustomSql` node, so generic graph lookups can find them

### Store `TYPE_ORIGIN` in declared type metadata

The review correctly called out that the RFC model assumes `TYPE_ORIGIN` is available
everywhere type resolution decisions may need it, even if the current matching logic
does not consume it.

This branch now stores `TYPE_ORIGIN` in:

- `SqlDeclaredEntityData`
- the generated `.pgrx_schema` payload for declared type and enum entries
- the corresponding section decoder path

That keeps declared type metadata structurally aligned with the rest of the
`SqlTranslatable`-derived metadata.

## Findings Status

### P1

#### 1. `extension_sql`-declared types lose graph identity

Status: fixed

What changed:

- declared types no longer become `BuiltinType`
- graph edges for functions and aggregates now point at the declaring
  `extension_sql!()` node
- graph-based type matching can now see declared types on `CustomSql` nodes

User-visible effect:

- custom-schema declared types are now schema-qualified correctly in aggregate state
  type SQL
- function and aggregate rendering both use the same graph identity

Regression coverage:

- `extension_sql_declared_type_orders_before_function_and_aggregate`
- `extension_sql_declared_type_in_custom_schema_prefixes_aggregate_state_type`

#### 2. `TYPE_ORIGIN` not recorded in `SqlDeclaredEntity`

Status: fixed

What changed:

- `SqlDeclaredEntityData` now stores `type_origin: Option<TypeOrigin>`
- declared type and enum section entries now encode and decode `TYPE_ORIGIN`

Regression coverage:

- `round_trip_sql_declared_type_preserves_type_origin`

#### 3. No duplicate `SCHEMA_KEY` detection

Status: fixed

What changed:

- `PgrxSql::build()` now calls `ensure_unique_type_targets()` after collecting type,
  enum, and `extension_sql!()` declarations
- duplicate schema keys now fail fast with a message that lists the conflicting SQL
  entities

Scope:

- derived types
- derived enums
- declared types and enums from `extension_sql!()`

Regression coverage:

- `duplicate_schema_key_errors`

### P2

#### 4. Magic number discriminants in `Result` encoding

Status: already resolved on this branch, re-verified

Current state:

- `section.rs` uses `RESULT_OK` and `RESULT_ERR`

No code change was needed in this remediation pass.

#### 5. Aggregate `to_sql()` duplicates type resolution logic

Status: fixed

What changed:

- aggregate rendering now uses `PgrxSql::find_type_dependency()` for `STYPE`, regular
  args, and direct args
- the old aggregate-only scan over `types` and `enums` is gone

This removes the divergence that caused the custom-schema declared-type bug.

#### 6. Dead parameters in `generate_schema_implicit()`

Status: already resolved on this branch, re-verified

Current state:

- `cargo-pgrx/src/command/schema.rs` no longer carries the old `_cargo` and
  `_features_arg` parameters in `generate_schema_implicit()`

No code change was needed in this remediation pass.

#### 7. RFC section format description

Status: already resolved before this remediation

Current state:

- `rfcs/0001-single-pass-schema-generation.md` documents the binary section format

No code change was needed in this remediation pass.

## Verification

Targeted verification completed:

- `cargo test -p pgrx-sql-entity-graph`
- `cargo test -p pgrx-unit-tests --features pg17 schema_key_tests --lib`
- `cargo fmt --all --check`

New or strengthened evidence:

- declared types no longer create builtin placeholders in the graph
- aggregate `STYPE` picks up the schema prefix from the declaring `extension_sql!()`
  node
- declared type metadata round-trips `TYPE_ORIGIN`
- duplicate `SCHEMA_KEY` values fail during graph build

## Notes

This remediation intentionally did not broaden the patch into unrelated SQL rendering
work. The goal here was to close the reviewed correctness and metadata gaps while
keeping the fix on the graph-resolution path the RFC already described.
