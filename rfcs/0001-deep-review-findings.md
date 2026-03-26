# Deep Review Findings: RFC 0001 Single-Pass Schema Generation

This document complements:

- `rfcs/0001-single-pass-schema-generation.md`
- `rfcs/0001-review-findings.md`
- `rfcs/0001-initial-deep-review.md`

It captures a deeper review of `wip-one-compile-please` relative to `develop`,
with special attention to user-facing migration behavior for existing pgrx
extensions.

The architectural direction remains good. The old `pgrx_embed` pipeline is
actually gone, the one-compile design is real, and the major code motion is
coherent.

## Status On Current Branch

This document captured the branch state at review time. On the current
`wip-one-compile-please` branch:

- missing `.pgrx_schema` now errors with a targeted message
- bad `TYPE_IDENT` fallback is fixed through explicit declared-type resolution
  while `TYPE_ORIGIN` stays explicit on `SqlTranslatable`
- `SetOfIterator` rejects argument position again
- the `HexInt` example already uses the const-based `SqlTranslatable` API
- install-time schema stripping is obsolete on this branch
- the RFC already documents the binary section format

## Finding 1: Missing `.pgrx_schema` is Treated as Success

### Problem

`cargo-pgrx/src/command/schema.rs` reads the built shared object, asks
`schema_section_data()` for the embedded schema section, and accepts `None` as
"no entities":

```rust
let section = schema_section_data(&lib_so_data)?;
let entities = if let Some(section) = section {
    decode_entities(section)?
} else {
    Vec::new()
};
```

That is not a safe fallback.

The new design depends on `.pgrx_schema` existing. If the section is missing,
something has gone wrong:

- the wrong binary was inspected
- the wrong architecture slice was selected from a fat Mach-O
- the artifact was stripped
- an older `cargo-pgrx` or extension artifact was used
- the build did not emit section data at all

Treating that case as "0 entities" violates the fail-fast goal of the RFC.

### Why This Matters

For extension authors migrating to the new system, this creates the most
confusing possible failure mode:

- schema generation may appear to succeed
- entity counts may be zero or suspiciously low
- SQL output may be empty or incomplete
- the user gets no clear message that the schema section was never found

The one-compile design is easiest to trust when missing metadata is a hard
error, not a silent empty result.

### Proposed Solution

If schema generation reaches the "load entities from section" stage and no
`.pgrx_schema` section is found, error immediately.

The error should include:

- the shared object path that was inspected
- that `.pgrx_schema` was not found
- likely causes: wrong artifact, wrong architecture slice, stripped binary, or
  stale tooling

This is especially important on macOS, where fat-binary selection can also
produce `None`.

### User Impact

This is a user-facing correctness issue, not just an implementation detail.

For existing extensions migrating to the new pgrx:

- if they accidentally use the wrong `cargo-pgrx`
- if they inspect an old build artifact
- if something strips the section too early

they should get a direct error telling them the schema metadata is missing.
They should not get an empty SQL file and have to reverse-engineer why.

---

## Finding 2: Bad `TYPE_IDENT` Values Silently Degrade to `BuiltinType`

### Problem

The graph builder matches function argument and return types to registered
types/enums by `type_ident`. If no match is found, it silently fabricates a
`BuiltinType` node:

```rust
if !found {
    mapped_builtin_types
        .entry(arg.used_ty.metadata.type_ident.to_string())
        .or_insert_with(|| {
            graph.add_node(SqlGraphEntity::BuiltinType(
                arg.used_ty.metadata.type_ident.to_string(),
            ))
        });
}
```

This is fine for real builtins like `i32` or `String`.

It is not fine for migration mistakes in manual `SqlTranslatable` impls.

Under the new design, manual/custom types now depend on a correct `TYPE_IDENT`.
If the user:

- hard-codes an old bare string
- copies a stale example
- writes the key at the wrong module path
- otherwise mismatches the type's actual identity

the current behavior is not "your type ident is wrong." It is "pretend this is
a builtin and keep going."

That makes failures indirect and harder to debug.

### Why This Matters

This is one of the core migration surfaces for existing extensions.

A user porting an old manual `SqlTranslatable` impl will often do three things:

1. update trait methods to associated consts
2. add `TYPE_IDENT`
3. keep their existing SQL spelling

If step 2 is wrong, they need a precise error. Silent fallback to
`BuiltinType` means they may instead see:

- wrong SQL ordering
- missing dependency edges
- broken `creates = [Type(T)]` behavior
- confusing follow-on errors far away from the actual cause

### Proposed Solution

Keep builtin fallback only for true builtins. Do not use it as a generic
"unmatched type" escape hatch.

For user-defined or derived types, an unmatched `type_ident` should become a
schema-generation error that clearly says:

- which type key failed to resolve
- which function argument/return referenced it
- that manual `SqlTranslatable` implementors should use
  `pgrx::pgrx_resolved_type!(T)` for `TYPE_IDENT`

### User Impact

For migrating users, the practical guidance should be:

- replace hand-written string keys with `pgrx::pgrx_resolved_type!(MyType)`
- define `TYPE_IDENT` at the type's definition site or equivalent canonical impl
  site
- expect pgrx to error if the key does not resolve

That is a much better migration story than silently treating a broken custom
type like a builtin.

---

## Finding 3: `SetOfIterator` Still Claims It Is Valid as an Argument

### Problem

`SetOfIterator<'_, T>` is return-only. It should not be usable as a function
argument.

But its `SqlTranslatable` impl currently says:

```rust
const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = T::ARGUMENT_SQL;
```

That advertises a valid SQL argument mapping whenever `T` itself is valid.

The function still cannot really work in argument position because the
call-convention layer does not support that shape. But under the new const
metadata design, the SQL metadata layer is supposed to reject invalid positions
early and explicitly.

### Why This Matters

The RFC's stated intent is that the new compile-time path should handle both
arguments and returns.

Right now, an invalid signature like:

```rust
#[pg_extern]
fn bad(arg: SetOfIterator<'static, i32>) -> i32 { ... }
```

does not fail because the SQL metadata says "no." It fails later for less
obvious reasons.

That is a regression in diagnostics quality for users, especially those
discovering unsupported signatures while migrating.

### Proposed Solution

Restore the old semantic rule at the const layer:

```rust
const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> =
    Err(ArgumentError::SetOf);
```

This makes the new metadata model accurately describe the type's legal SQL
positions and gives users the expected compile-time failure.

### User Impact

This mainly affects users with invalid function signatures.

The important point is not that these signatures should become valid. They
should not. The important point is that they should fail for the right reason,
with a migration error the user can understand immediately.

---

## Finding 4: The Main Manual `SqlTranslatable` Example Is Stale

### Problem

`pgrx-examples/custom_types/src/hexint.rs` still shows the pre-refactor API:

```rust
unsafe impl SqlTranslatable for HexInt {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> { ... }
    fn return_sql() -> Result<Returns, ReturnsError> { ... }
}
```

But the trait now requires associated consts:

- `TYPE_IDENT`
- `ARGUMENT_SQL`
- `RETURN_SQL`

This is the example most likely to be copied by users migrating manual/custom
types. Right now it teaches an API that no longer exists.

### Why This Matters

The biggest user-facing change in the refactor is the new const-based
`SqlTranslatable` contract for manual implementations.

If the example is stale, users will:

- copy old methods that no longer satisfy the trait
- fail to define `TYPE_IDENT`
- miss the intended use of `pgrx::pgrx_resolved_type!(T)`
- potentially hard-code an incorrect SQL identity

This is exactly the kind of migration paper cut that turns a sound refactor into
an unpleasant upgrade.

### Proposed Solution

Update the example to the new contract, including:

- `const TYPE_IDENT: &'static str = pgrx::pgrx_resolved_type!(HexInt);`
- `const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = ...`
- `const RETURN_SQL: Result<ReturnsRef, ReturnsError> = ...`

The example should continue showing the "Rust type name differs from SQL type
name" case, because that is one of the main reasons users need manual
`SqlTranslatable` at all.

### User Impact

This is a pure migration/documentation issue, but it is an important one.

For existing extension authors with manual SQL-backed types, the example should
make the migration path obvious:

1. move runtime methods to associated consts
2. define `TYPE_IDENT` with `pgrx_resolved_type!`
3. keep SQL spelling in `ARGUMENT_SQL` / `RETURN_SQL`

---

## Finding 5: Install-Time Schema Stripping Adds New Operational Requirements

### Problem

`cargo pgrx install` now strips `.pgrx_schema` from the installed shared object
by default.

That introduces platform-specific operational dependencies:

- on macOS, the rewritten binary must be re-signed with `codesign`
- on ELF targets, `llvm-objcopy` or `objcopy` must exist

If those tools are missing, install fails and the user is told to rerun with
`--no-schema-strip`.

### Why This Matters

This is a real user-facing change in day-to-day workflow.

An existing extension author upgrading to the new pgrx may reasonably expect:

- `cargo pgrx install`
- `cargo pgrx run`
- `cargo pgrx package`

to keep working if their Rust/Postgres setup is sound.

With stripping enabled by default, the operational contract changes:

- local developer machines may now need extra toolchain components
- CI images may now need extra packages
- macOS users may encounter re-signing failures unrelated to extension code

That may be acceptable, but it should be treated as a product decision, not an
incidental implementation detail.

### Proposed Solution

At minimum:

- document the new dependencies prominently in migration and install docs
- explain when `--no-schema-strip` is appropriate

Potentially better:

- make stripping opt-in rather than default
- or restrict stripping to package/distribution workflows instead of routine
  install/run paths

### User Impact

For migrating users, the practical guidance is:

- if install suddenly fails on `codesign`, `llvm-objcopy`, or `objcopy`, the new
  schema-stripping step is the likely cause
- `--no-schema-strip` is the short-term escape hatch

The bigger decision is whether pgrx wants this extra operational burden on the
default local workflow at all.

---

## Finding 6: The RFC Still Describes NDJSON, but the Implementation Is Binary

### Problem

The RFC still describes `.pgrx_schema` as newline-delimited JSON. The
implementation now uses a compact binary format with length-prefixed entries.

This is not directly user-facing for extension authors, but it does matter for:

- maintainers
- reviewers
- people debugging the new pipeline
- future contributors reading the RFC as the design source of truth

### Why This Matters

The refactor is large enough that maintainability depends on the RFC matching
the code reasonably well.

When the document still describes NDJSON but the code uses binary encoding,
someone trying to debug schema extraction from the RFC will head in the wrong
direction immediately.

### Proposed Solution

Update the RFC's section-format discussion to describe the actual wire format:

- length-prefixed entries
- tagged payloads
- binary field encoding
- tolerated trailing zero padding

Also update any pseudocode that still implies JSON construction or parsing.

### User Impact

None directly for extension authors.

This is a maintainer-facing documentation drift issue.

---

## Migration Notes for Existing Extension Authors

These are the practical changes most likely to matter during upgrade.

### 1. Manual `SqlTranslatable` impls must move to associated consts

Old-style implementations using:

- `fn argument_sql()`
- `fn return_sql()`

must be rewritten to:

- `const TYPE_IDENT`
- `const ARGUMENT_SQL`
- `const RETURN_SQL`

The intended pattern is:

```rust
unsafe impl SqlTranslatable for MyType {
    const TYPE_IDENT: &'static str = pgrx::pgrx_resolved_type!(MyType);
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = ...;
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> = ...;
}
```

### 2. `TYPE_IDENT` should come from `pgrx_resolved_type!`

Users should not hand-write ad hoc string keys unless they have a very specific
reason and fully understand the consequences.

The normal migration is:

```rust
const TYPE_IDENT: &'static str = pgrx::pgrx_resolved_type!(MyType);
```

If pgrx accepts a bad key silently, that is an implementation problem. From the
user's point of view, the correct migration path is still to use the macro.

### 3. Arrays of non-SQL types are now more explicitly rejected

This was already noted in `rfcs/0001-review-findings.md`, but it is worth
repeating because it is user-facing.

Wrappers like `Vec<T>` and array-style SQL positions no longer quietly propagate
`Skip` for non-SQL leaf types. They error instead.

That is probably the right semantic rule, but it is a real compatibility change
for extensions with unusual signatures.

### 4. The old `pgrx_embed` bin is gone

This is a positive migration change.

Extensions no longer need:

- `src/bin/pgrx_embed.rs`
- a `[[bin]]` target for `pgrx_embed`
- a second schema-generation build

### 5. Tooling failures around section stripping may be new

If a previously healthy extension starts failing on:

- `codesign`
- `llvm-objcopy`
- `objcopy`

the new stripping step is the likely reason.

---

## Overall Assessment

The refactor is on the right path and substantially improves the architecture.
The remaining work is less about changing the design and more about making the
new system trustworthy under failure and humane to migrate to.

Before calling the branch migration-ready, the implementation should:

1. error when `.pgrx_schema` is missing
2. error on unresolved non-builtin `TYPE_IDENT` references
3. reject `SetOfIterator` in argument position at the metadata layer
4. update the manual custom-type example to the new const-based API
5. decide whether schema stripping belongs in the default install workflow

Once those are addressed, the one-compile design should be much easier for
existing extension authors to adopt with confidence.
