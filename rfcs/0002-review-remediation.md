# RFC 0002: Review Remediation for Single-Pass Schema Generation

## Summary

This document records the follow-up work for the branch review captured in:

- `rfcs/0001-initial-deep-review.md`
- `rfcs/0001-deep-review-findings.md`

The two blocking questions from that review are resolved on this branch:

1. `extension_sql!(creates = [Type(T)]/[Enum(T)])` should not flatten extension-owned
   types into `BuiltinType`.
   Resolution: declared types now resolve to the declaring `extension_sql!()` graph node.
   That preserves type identity and lets SQL rendering use the graph instead of a
   builtin placeholder.
2. Whether `TYPE_ORIGIN` needs to be encoded for declared type and enum entries.
   Resolution: no. The current implementation only resolves declared entries by
   `TYPE_IDENT`, so declared type metadata carries just `TYPE_IDENT` plus SQL mapping.
   `TYPE_ORIGIN` stays explicit on `SqlTranslatable` / `UsedType` metadata, where
   unresolved-type decisions are actually made.

The result is closer to the RFC model from
`rfcs/0001-single-pass-schema-generation.md`: dependency resolution now has a single
schema-emitting target for declared types, and ownership metadata stays on the
`SqlTranslatable` path that the resolver actually consumes.

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
- `SqlGraphEntity::type_ident_matches()` recognizes declared types carried by a
  `CustomSql` node, so generic graph lookups can find them

### Keep declared type metadata lean

The review correctly called out that `TYPE_ORIGIN` matters for unresolved-type
decisions. The follow-up work on this branch showed those decisions only happen on the
`SqlTranslatable` / `UsedType` path, not on declared type entries.

This branch keeps declared type metadata to the fields the current implementation
actually consumes:

- `TYPE_IDENT`
- SQL mapping
- the corresponding section decoder path for those values

Declared type and enum entries do not duplicate `TYPE_ORIGIN`.

`TYPE_ORIGIN` still lives on `SqlTranslatable`-derived metadata for function args,
returns, aggregates, and other used-type positions, which is where the resolver
needs it.

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

#### 2. Declared entries do not store `TYPE_ORIGIN`

Status: closed with a narrower design

What changed:

- declared type and enum entries store `TYPE_IDENT` plus SQL mapping
- declared type and enum section entries encode and decode those values
- `TYPE_ORIGIN` stays on `SqlTranslatable` / `UsedType` metadata instead of being
  duplicated on declared entries

Regression coverage:

- `round_trip_sql_declared_type_preserves_type_ident_and_sql`

#### 3. No duplicate `TYPE_IDENT` detection

Status: fixed

What changed:

- `PgrxSql::build()` now calls `ensure_unique_type_targets()` after collecting type,
  enum, and `extension_sql!()` declarations
- duplicate type idents now fail fast with a message that lists the conflicting SQL
  entities

Scope:

- derived types
- derived enums
- declared types and enums from `extension_sql!()`

Regression coverage:

- `duplicate_type_ident_errors`

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
- `cargo test -p pgrx-unit-tests --features pg17 type_ident_tests --lib`
- `cargo fmt --all --check`

New or strengthened evidence:

- declared types no longer create builtin placeholders in the graph
- aggregate `STYPE` picks up the schema prefix from the declaring `extension_sql!()`
  node
- declared type metadata round-trips `TYPE_IDENT` and SQL mapping
- duplicate `TYPE_IDENT` values fail during graph build

## Notes

This remediation intentionally did not broaden the patch into unrelated SQL rendering
work. The goal here was to close the reviewed correctness and metadata gaps while
keeping the fix on the graph-resolution path the RFC already described.
