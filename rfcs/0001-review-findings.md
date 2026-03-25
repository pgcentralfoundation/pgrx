# Review Findings: RFC 0001 Single-Pass Schema Generation

PR #2264 — `wip-one-compile-please` into `pg_bench`

## Overview

Five commits, 155 files changed (+5107/-2630). The implementation is thorough and the
old two-stage pipeline removal is complete with zero dangling references. This document
catalogs the issues found during review, proposes solutions, and analyzes user impact.

## Status On Current Branch

This is a review artifact, not the current branch status.

On `wip-one-compile-please` as it exists now:

- the RFC has been updated to the binary section format
- `pgrx_resolved_type!()` includes `module_path!()`
- missing `.pgrx_schema` is a hard error
- `SetOfIterator` rejects argument position again
- the section result tags use named constants
- the install-time schema-stripping dependency is obsolete on this branch
- unresolved `SCHEMA_KEY` fallback is now handled through explicit declared-type
  resolution and `TYPE_ORIGIN`

---

## Finding 1: RFC Describes NDJSON but Implementation Uses Binary Encoding

### Problem

The RFC (lines 719-731) specifies the `.pgrx_schema` section format as newline-delimited
JSON:

> The `.pgrx_schema` section contains entity entries as newline-delimited JSON (NDJSON).
> Each entry is a UTF-8 JSON object followed by `\n` (0x0A).

The actual implementation in `pgrx-sql-entity-graph/src/section.rs` uses a compact
length-prefixed binary encoding: each entry is a `[u32 LE payload length][tagged binary
fields]` sequence. `EntryWriter` builds these at compile time as const byte arrays.
`EntryReader` deserializes them at schema-generation time. No JSON is involved anywhere in
the pipeline.

The binary format is the better choice — it is more compact, fully const-evaluable
without dragging in a JSON serializer at compile time, and avoids JSON-escaping headaches
for embedded SQL strings. But the RFC is now a misleading guide for anyone trying to
understand the section format.

### Proposed Solution

Update the RFC's "Section Format" section to describe the actual binary encoding:

- Length-prefixed entries: `[u32 LE payload length][payload bytes]`
- First byte of each payload is an entity type tag (`ENTITY_SCHEMA = 1` through
  `ENTITY_TRIGGER = 9`)
- Fields are encoded as: `u8` raw, `bool` as 0/1, `u32` as 4 bytes LE, strings as
  `[u32 LE length][UTF-8 bytes]`, optionals as `[bool present][conditional content]`,
  lists as `[u32 count][items...]`
- Trailing zero padding from linker alignment is tolerated and terminates parsing

Replace the NDJSON-specific reading pseudocode with pseudocode matching the actual
`entry_payloads` + `decode_entity` flow.

Also update the `concat!()` / `module_path!()` JSON construction example in the "How
entity data is embedded" section (RFC lines 266-296) to reflect that the proc macros
generate `EntryWriter` chains, not JSON string concatenation.

### User Impact

None. The RFC is an internal design document. Extension authors never interact with the
section format directly.

---

## Finding 2: `pgrx_resolved_type!` Omits `module_path!()`, Diverging From RFC

### Problem

The RFC specifies `pgrx_resolved_type!` as:

```rust
// rfcs/0001-single-pass-schema-generation.md:211-215
macro_rules! pgrx_resolved_type {
    ($ty:ty) => {
        concat!(module_path!(), "::", stringify!($ty))
    };
}
```

But the implementation only stringifies the type name:

```rust
// pgrx/src/lib.rs:383-387
macro_rules! pgrx_resolved_type {
    ($ty:ty) => {
        stringify!($ty)
    };
}
```

This means `SCHEMA_KEY` for a type `Foo` in module `my_ext::types` is just `"Foo"` rather
than the RFC-specified `"my_ext::types::Foo"`. The graph builder matches function arguments
to type entities by string equality on this key:

```rust
// pgrx-sql-entity-graph/src/pgrx_sql.rs:803-819
let found = mapped_types
    .keys()
    .any(|ty_item| ty_item.matches_schema(arg.used_ty.metadata.schema_key))
    || mapped_enums
        .keys()
        .any(|ty_item| ty_item.matches_schema(arg.used_ty.metadata.schema_key));

if !found {
    mapped_builtin_types
        .entry(arg.used_ty.metadata.schema_key.to_string())
        .or_insert_with(|| {
            graph.add_node(SqlGraphEntity::BuiltinType(
                arg.used_ty.metadata.schema_key.to_string(),
            ))
        });
}
```

Without `module_path!()`, two types with the same name in different modules would collide
on the same `SCHEMA_KEY`. More importantly, the bare name is fragile for manual
`SqlTranslatable` impls: the author must know to use the exact stringified form of the
type name as it appears at the derive site, rather than a module-qualified path.

When a `schema_key` does not match any registered type or enum, the code silently
fabricates a `BuiltinType` node. No warning, no error. This is correct for actual builtins
like `i32` or `String`, but it is also the silent failure mode for mismatched keys.

### Proposed Solution

Implement the macro as the RFC specifies:

```rust
#[macro_export]
macro_rules! pgrx_resolved_type {
    ($ty:ty) => {
        concat!(module_path!(), "::", stringify!($ty))
    };
}
```

This produces keys like `"my_ext::types::Foo"` instead of `"Foo"`, which:

1. **Disambiguates same-named types in different modules.** Two types named `Status` in
   `my_ext::orders` and `my_ext::users` get distinct keys.

2. **Makes manual `SqlTranslatable` impls self-locating.** When a user writes
   `pgrx_resolved_type!(HexInt)` inside their impl block, `module_path!()` expands to
   the module where the impl is written. As long as the derive and the manual impl are
   in the same module (or the manual impl is at the type's definition site), the keys
   match. This is the natural, correct placement.

3. **Matches the `#[derive(PostgresType)]` and `#[derive(PostgresEnum)]` code paths.**
   The derive macros in `postgres_type/mod.rs` and `postgres_enum/mod.rs` emit
   `::pgrx::pgrx_resolved_type!(#name)` inside the generated `SqlTranslatable` impl.
   Since `module_path!()` expands at the *expansion site* (the user's crate, where the
   derive is applied), it will produce the user's module path, not pgrx's internal path.

The `#[pg_extern]` side already reads `SCHEMA_KEY` through the trait
(`<T as SqlTranslatable>::SCHEMA_KEY`), so the function argument's key automatically
matches the type's key as long as they resolve to the same `SqlTranslatable` impl.
Re-exports and type aliases also work because the compiler resolves to the same impl.

The `simple_sql_type!` invocations for pgrx builtins (in
`pgrx-pg-sys/src/submodules/sql_translatable.rs`) pass a literal `$schema_key` string
rather than using the macro, so they are unaffected. These should continue to use short
literal keys (`"i32"`, `"String"`, etc.) since their `SCHEMA_KEY` values are matched by
`SqlTranslatable` trait resolution, not by string comparison against a derive-generated
key.

**One edge case to handle:** `extension_sql!` with `creates = [Type(Foo)]` records both
`concat!(module_path!(), "::", "Foo")` as `data.name` and `<Foo as SqlTranslatable>::SCHEMA_KEY`
as `data.schema_key`. With the fix, both sides now include `module_path!()`, so the
comparison `ident_name == &data.schema_key` should still hold. Verify that the
`section_identifier_tokens` code path in `extension_sql/mod.rs` produces the same
`module_path!()` prefix as the type's own `SCHEMA_KEY`.

### User Impact

**Derive-only users**: No impact. The derive macros call `pgrx_resolved_type!` in
generated code at the user's definition site, so both the type entity and any function
referencing it resolve to the same module-qualified key automatically.

**Manual `SqlTranslatable` implementors**: The `SCHEMA_KEY` value changes from
`"HexInt"` to `"my_ext::HexInt"` (or whatever `module_path!()` produces at the impl
site). This is a **breaking change** for anyone who hard-codes `SCHEMA_KEY` as a bare
string rather than using `pgrx_resolved_type!()`. The migration is to replace:
```rust
const SCHEMA_KEY: &'static str = "HexInt";
```
with:
```rust
const SCHEMA_KEY: &'static str = pgrx::pgrx_resolved_type!(HexInt);
```
which is what the RFC's migration guide already recommends. Anyone following the
documented migration path is unaffected.

**`extension_sql!` with `creates = [Type(T)]`**: Works as before, since both the
`creates` declaration and the type entity derive their keys from the same
`SqlTranslatable` impl.

---

## Finding 3: `Vec<T>` / `Array<T>` Behavior Change for `Skip`-Mapped Types

### Problem

The old runtime `Vec<T>::argument_sql()` propagated `SqlMapping::Skip` through unchanged:

```rust
// old behavior (removed in this PR):
Ok(SqlMapping::Skip) => Ok(SqlMapping::Skip),
```

The new const version converts `Skip` to an error:

```rust
// pgrx-sql-entity-graph/src/metadata/sql_translatable.rs:136
Ok(SqlMappingRef::Skip) => Err(ArgumentError::SkipInArray),
```

And identically for `array_return_sql`:

```rust
// pgrx-sql-entity-graph/src/metadata/sql_translatable.rs:155
Ok(ReturnsRef::One(SqlMappingRef::Skip)) => Err(ReturnsError::SkipInArray),
```

`Skip` is used for types like `FunctionCallInfoBaseData` and `pg_sys::PlannerInfo` that
have no SQL representation and should not appear in function signatures. `Vec<T>` where
`T` is `Skip`-mapped was silently accepted before and would now be a compile error (the
`Err` propagates through to the entity data, and the graph builder will reject it or
produce broken SQL).

### Proposed Solution

Two options:

**Option A — Preserve old behavior (propagate Skip):**

```rust
pub const fn array_argument_sql(
    mapping: Result<SqlMappingRef, ArgumentError>,
) -> Result<SqlMappingRef, ArgumentError> {
    match mapping {
        // ...existing arms...
        Ok(SqlMappingRef::Skip) => Ok(SqlMappingRef::Skip),
        Err(err) => Err(err),
    }
}
```

And similarly for `array_return_sql`.

**Option B — Keep the error but document it as intentional (recommended):**

`Vec<FunctionCallInfoBaseData>` in a `#[pg_extern]` signature is almost certainly a bug —
these types have no SQL representation and wrapping them in a `Vec` does not create one.
The old behavior silently produced a function with a missing argument in the SQL, which is
worse than a clear error. Keep `SkipInArray` as an error, but add a clear error message
that explains what happened:

```rust
ArgumentError::SkipInArray => "Array/Vec of a type with no SQL representation \
    (e.g., FunctionCallInfoBaseData) cannot appear in a #[pg_extern] signature. \
    If you need this type, implement SqlTranslatable with an explicit ARGUMENT_SQL.",
```

### User Impact

**Option A**: No breaking change. Preserves the old (arguably wrong) behavior.

**Option B**: Breaking change for any extension that has `Vec<T>` or `Array<T>` in a
function signature where `T` maps to `Skip`. This is expected to be extremely rare — the
`Skip` types are Postgres internal structs that should not appear as array elements in
user-facing SQL functions. Any affected user was already producing broken SQL silently.

Recommendation: Option B. This is a correctness improvement disguised as a breaking
change. The old behavior was a latent bug.

---

## Finding 4: `SetOfIterator::ARGUMENT_SQL` Lost Its Error Guard

### Problem

The old runtime implementation returned `Err(ArgumentError::SetOf)` when `SetOfIterator`
appeared as a function argument (set-returning functions can only appear in return
position). The new const impl delegates to `T`:

```rust
// pgrx/src/iter.rs:83-90
unsafe impl<T> SqlTranslatable for SetOfIterator<'_, T>
where
    T: SqlTranslatable,
{
    const SCHEMA_KEY: &'static str = T::SCHEMA_KEY;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = T::ARGUMENT_SQL;
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> = setof_return_sql(T::RETURN_SQL);
}
```

If `T` is `i32`, then `SetOfIterator<i32>::ARGUMENT_SQL` is `Ok(As("INT"))`, which would
allow it to pass validation as a function argument. The `ArgumentError::SetOf` variant
still exists but is no longer set anywhere.

Whether this matters depends on whether the `#[pg_extern]` proc macro independently
rejects `SetOfIterator` in argument position during its own syntactic analysis (before
`SqlTranslatable` is consulted). If it does, this is a defense-in-depth loss but not a
bug. If it does not, this is a silent acceptance of an invalid signature.

### Proposed Solution

Restore the error in the trait impl:

```rust
unsafe impl<T> SqlTranslatable for SetOfIterator<'_, T>
where
    T: SqlTranslatable,
{
    const SCHEMA_KEY: &'static str = T::SCHEMA_KEY;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Err(ArgumentError::SetOf);
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> = setof_return_sql(T::RETURN_SQL);
}
```

This makes the trait impl self-documenting: `SetOfIterator` is not valid as an argument
regardless of what `T` says. Defense in depth — even if the proc macro also catches this,
the trait should not lie about its capabilities.

### User Impact

None for correct code. `SetOfIterator` in argument position was never valid. Any
extension that somehow had this would have produced broken SQL before and will now get a
clear error.

---

## Finding 5: Dead Parameters in `generate_schema_implicit`

### Problem

```rust
// cargo-pgrx/src/command/schema.rs:171-181
pub(crate) fn generate_schema_implicit(
    _cargo: Cargo,
    package_manifest_path: &Path,
    profile: &CargoProfile,
    _features_arg: String,
    target: Option<&str>,
    path: Option<&Path>,
    dot: Option<&Path>,
    output_tracking: &mut Vec<PathBuf>,
    manifest: cargo_toml::Manifest,
) -> eyre::Result<()> {
```

`_cargo` and `_features_arg` are unused. They were needed by the old pipeline (to invoke a
second `cargo rustc` build). The new pipeline reads the `.pgrx_schema` section from the
already-built `.so` and does not invoke cargo at all during schema generation.

Callers must still construct and pass these values even though they are ignored. This is
API pollution and a maintenance trap — a future reader will assume these parameters affect
behavior.

### Proposed Solution

Remove both parameters from `generate_schema_implicit` and update all call sites:

- `cargo-pgrx/src/command/schema.rs` — the `generate_schema_for_cli` caller
- `cargo-pgrx/src/command/install.rs` — the `copy_sql_files` caller (via the
  `generate_schema` alias)
- Any other call sites found by the compiler after removal

Also consider removing the `generate_schema_for_cli` / `generate_schema` alias — the two
names for the same function add confusion without adding value.

### User Impact

None. These are `pub(crate)` functions internal to `cargo-pgrx`. No external API change.

---

## Finding 6: Silent Empty Schema When `.pgrx_schema` Section Is Missing

### Problem

```rust
// cargo-pgrx/src/command/schema.rs:238-242
let entities = if let Some(section) = section {
    decode_entities(section).wrap_err("couldn't decode pgrx schema section")?
} else {
    Vec::new()
};
```

When the `.pgrx_schema` section is absent from the compiled `.so`, `load_section_entities`
returns an empty `Vec`. The final SQL output will contain only the `ExtensionRoot` (from
the control file) — a valid but empty schema with no functions, types, or other entities.

This could happen if:
- The extension was compiled with an older pgrx that does not emit section data
- A build misconfiguration caused the section to be stripped or not linked
- The wrong `.so` was picked up from the target directory
- Cross-compilation produced a binary for a different architecture and the fat binary
  slicer returned `None`

In all these cases, the user gets a valid `.sql` file that silently omits their entire
extension. This is the worst kind of failure — it looks like success.

### Proposed Solution

Emit a loud warning when the section is missing, and consider making it an error by
default with an opt-out flag:

```rust
let entities = if let Some(section) = section {
    decode_entities(section).wrap_err("couldn't decode pgrx schema section")?
} else {
    eprintln!(
        "WARNING: no .pgrx_schema section found in `{}`. \
         The generated SQL will contain no entities. \
         This usually means the extension was not compiled with a pgrx version \
         that supports single-pass schema generation.",
        lib_so.display(),
    );
    Vec::new()
};
```

Or, more aggressively:

```rust
} else {
    bail!(
        "no .pgrx_schema section found in `{}` — cannot generate schema. \
         Was the extension compiled with the matching pgrx version?",
        lib_so.display(),
    );
};
```

The aggressive version is probably correct for this PR — if the extension was built by
this version of `cargo-pgrx`, it should always have the section. The only case where it
would not is a genuine error.

### User Impact

**New pgrx users**: No impact — their extensions will always have the section.

**Users upgrading from old pgrx**: If they run new `cargo-pgrx schema` against an old
build artifact, they get a clear error instead of a silently empty schema. This is
strictly better.

**Cross-compilation users**: If the fat binary slicer picks the wrong arch, they get an
error instead of empty SQL. Also strictly better.

Recommendation: Use `bail!` (hard error). An empty schema is never what the user wants.

---

## Finding 7: Section Encoding Uses Magic Number Discriminants

### Problem

The `argument_sql` and `return_sql` fields encode `Result` variants as raw byte
discriminants `1` (Ok) and `2` (Err) without named constants:

```rust
// Writer — pgrx-sql-entity-graph/src/section.rs:318-323
pub const fn argument_sql(self, value: Result<SqlMappingRef, ArgumentError>) -> Self {
    match value {
        Ok(mapping) => self.u8(1).sql_mapping(mapping),
        Err(err) => self.u8(2).argument_error(err),
    }
}

// Reader — pgrx-sql-entity-graph/src/section.rs:481-486
pub fn read_argument_sql(&mut self) -> Result<Result<SqlMappingRef, ArgumentError>> {
    match self.read_u8()? {
        1 => Ok(Ok(self.read_sql_mapping()?)),
        2 => Ok(Err(self.read_argument_error()?)),
        other => Err(eyre!("invalid argument sql tag in schema entry: {other}")),
    }
}
```

Every other discriminant in the file has a named constant (`ENTITY_SCHEMA = 1`,
`ENTITY_CUSTOM_SQL = 2`, etc., and the `SqlMapping` / `Returns` / error variant tags).
These two are the exception, and the asymmetry is a maintenance risk — a future editor
might change one side without the other.

### Proposed Solution

Add named constants alongside the entity tags:

```rust
pub(crate) const RESULT_OK: u8 = 1;
pub(crate) const RESULT_ERR: u8 = 2;
```

Then use them in both writer and reader:

```rust
// Writer
Ok(mapping) => self.u8(RESULT_OK).sql_mapping(mapping),
Err(err) => self.u8(RESULT_ERR).argument_error(err),

// Reader
RESULT_OK => Ok(Ok(self.read_sql_mapping()?)),
RESULT_ERR => Ok(Err(self.read_argument_error()?)),
```

### User Impact

None. Internal encoding detail.

---

## Finding 8: New Runtime Dependencies for Section Stripping

### Problem

After schema generation, `cargo pgrx install` strips the `.pgrx_schema` section from the
installed `.so` to remove dead metadata from the runtime artifact.

- **Linux**: Requires `llvm-objcopy` or `objcopy` in `$PATH`. If neither is found,
  install fails unless `--no-schema-strip` is passed.
- **macOS**: The strip is done by zeroing section bytes in-place (no external tool), but
  then `codesign --force --sign -` (ad-hoc re-signing) is run to fix the code signature
  invalidated by the byte modification.

`llvm-objcopy` / `objcopy` was not previously required by `cargo pgrx install`. This is a
new hard dependency on the default install path that could surprise users, particularly in
minimal Docker images or CI environments.

The error message does mention `--no-schema-strip` as an escape hatch, which is good.

### Proposed Solution

Three layers:

1. **Document the new dependency** in the cargo-pgrx README (and UPGRADING notes if
   they exist). Call out that `llvm-objcopy` or `objcopy` is needed on Linux, and that
   `--no-schema-strip` skips the requirement.

2. **Degrade gracefully on Linux**: If neither tool is found, emit a warning and skip
   stripping rather than failing the install. The `.pgrx_schema` section in the installed
   `.so` is inert — Postgres will not load or execute it. It wastes a small amount of
   disk and memory (mapped but never touched), but does not affect correctness. Make
   `--no-schema-strip` the implicit fallback when the tools are missing, and print a note
   telling the user how to install the tool for a cleaner artifact.

3. **Consider the macOS codesign interaction**: Ad-hoc signing (`codesign --sign -`) is
   correct for local development but will break any binary that was previously signed with
   a real Developer ID. This is probably fine for pgrx's use case (extensions are loaded
   by Postgres, not distributed as standalone macOS apps), but document that
   `--no-schema-strip` preserves the original signature.

### User Impact

**Linux CI/Docker users**: Currently a hard failure if `objcopy` is not installed. With
the proposed graceful degradation, they get a warning but the install succeeds.

**macOS users**: No change if they are using ad-hoc or unsigned builds (the common case
for Postgres extensions). Users with real code signatures should use `--no-schema-strip`.

**All users**: The section is small (typically a few KB) and completely inert at runtime.
Leaving it in place has no functional consequence.

---

## Summary Table

| # | Finding | Severity | Proposed Action |
|---|---------|----------|-----------------|
| 1 | RFC describes NDJSON, impl uses binary | Low | Update RFC |
| 2 | `pgrx_resolved_type!` omits `module_path!()`, diverging from RFC | Medium | Add `module_path!()` per RFC spec |
| 3 | `Vec<T>` rejects `Skip` types (behavior change) | Medium | Keep as error (intentional improvement) |
| 4 | `SetOfIterator::ARGUMENT_SQL` lost error guard | Medium | Restore `Err(ArgumentError::SetOf)` |
| 5 | Dead parameters in `generate_schema_implicit` | Low | Remove them |
| 6 | Silent empty schema on missing section | High | Make it a hard error |
| 7 | Magic number discriminants in section encoding | Low | Add named constants |
| 8 | New `objcopy` / `codesign` runtime dependencies | Medium | Graceful degradation + docs |
