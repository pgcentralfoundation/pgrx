# RFC 0001: Single-Pass Schema Generation

## Status

Draft

## Summary

Eliminate the two-pass compilation required by `cargo-pgrx` for SQL schema generation.
Today, building a pgrx extension requires: (1) `cargo build --lib` to produce the
`.so`, (2) parsing the `.so`'s symbol table with the `object` crate, (3) generating a
Rust source file, (4) `cargo rustc --bin pgrx_embed` to compile that source, and (5)
executing the resulting binary to collect entity metadata and emit SQL. This RFC
replaces all of that with proc macros that embed entity metadata as const data in a
dedicated linker section (`.pgrxsc` on ELF/PE, `__DATA,__pgrxsc` on Mach-O) during the
single `cargo build --lib`. After the build, `cargo-pgrx` reads that section from the
`.so`, builds the dependency graph, and emits SQL.

## Problem Statement

### The current flow

When a user runs `cargo pgrx install`, `cargo pgrx run`, `cargo pgrx package`,
`cargo pgrx test`, or `cargo pgrx schema`, the following happens:

1. **First build** — `cargo build --lib` compiles the extension into a shared library
   (`.so` on Linux, `.dylib` on macOS). Every pgrx proc macro (`#[pg_extern]`,
   `#[derive(PostgresType)]`, etc.) generates a `#[no_mangle] pub extern "Rust" fn
   __pgrx_internals_*() -> SqlGraphEntity` function that is compiled into this
   library.

2. **Symbol extraction** — `cargo-pgrx` reads the compiled `.so` as a byte buffer and
   parses it with the `object` crate to extract the dynamic symbol table. It filters
   for symbols starting with `__pgrx_internals_` and collects their names. On macOS,
   it must also handle fat/universal binaries by slicing to the correct architecture.

3. **Codegen** — `cargo-pgrx` generates a Rust source file (written to a temp file)
   containing a `main()` function that:
   - Declares each `__pgrx_internals_*` symbol as an `unsafe extern "Rust"` function
   - Calls each one to collect `SqlGraphEntity` values into a `Vec`
   - Passes them to `PgrxSql::build()` to construct a dependency graph
   - Calls `pgrx_sql.to_file()` to emit the final `schema.sql`

4. **Second build** — `cargo rustc --bin pgrx_embed_{name} -- --cfg pgrx_embed` with
   `PGRX_EMBED` pointing to the generated source. This compiles the generated code
   into a standalone binary that links against the extension library. The
   `pgrx_embed!()` macro in `src/bin/pgrx_embed.rs` uses `include!(env!("PGRX_EMBED"))`
   to pull in the generated code.

5. **Execution** — The `pgrx_embed` binary is executed. It calls each
   `__pgrx_internals_*` function at runtime, which constructs `SqlGraphEntity` values
   containing runtime-resolved information (notably `TypeId` and `type_name::<T>()`
   values). These entities are assembled into a dependency graph and SQL is emitted.

### Why this is painful

- **Two full compilation passes.** The second pass must recompile the embed binary and
  re-link against the extension library. Even with incremental compilation caching
  most dependencies, this adds significant wall-clock time.
- **ELF/Mach-O parsing for symbol scanning.** The `object` crate scans the entire
  dynamic symbol table looking for `__pgrx_internals_*` names, requiring special
  handling for macOS fat/universal binaries.
- **Generated code.** The codegen step is fragile — it generates Rust source as
  strings, writes it to a temp file, and invokes `cargo rustc` with special `--cfg`
  flags and environment variables.
- **User-visible artifact.** Every pgrx extension must have a `src/bin/pgrx_embed.rs`
  file containing `::pgrx::pgrx_embed!();`. Users must not delete this file or the
  second build breaks. The `[[bin]]` target must be declared in `Cargo.toml`.
- **Conceptual complexity.** The system is difficult to understand, debug, and maintain.
  When schema generation fails, the error may come from any of the five steps above,
  and diagnosing the root cause requires understanding the entire pipeline.

### The constraint that makes this hard

The `__pgrx_internals_*` functions contain calls to `core::any::TypeId::of::<T>()` and
`core::any::type_name::<T>()`, where `T` is a user-defined type like `MyType`. These
calls are resolved by `rustc` during compilation — the proc macro that generates them
only sees the token `MyType`, not its fully-qualified path or unique type identity.

The `TypeId` values are used by the `PgrxSql` graph builder to match function
argument/return types to registered `PostgresType`/`PostgresEnum` entities. For
example, if `fn foo(x: MyType)` and `MyType` derives `PostgresType`, the graph builder
uses `TypeId` to create a dependency edge ensuring `CREATE TYPE my_type` appears before
`CREATE FUNCTION foo(...)` in the schema.

This is why the current system must *execute* compiled code — `TypeId` values only
exist at runtime (or const-eval time, but there is no way to extract const-evaluated
values from a compiled artifact without parsing the binary).

## Design

### Core insight

Proc macros cannot resolve types, but they can generate code that lets the *compiler*
resolve types. Two key facts enable this:

1. **`concat!()` accepts `module_path!()`** — a proc macro can construct a string
   template with a `module_path!()` hole, and `rustc` fills in the fully-qualified
   module path at compile time. This gives us compiler-resolved module paths without
   runtime execution.

2. **`#[link_section]` embeds const data in the binary** — const byte arrays placed in
   a named linker section are readable from the `.so` without executing code. Combined
   with (1), the proc macro can embed complete entity metadata — including
   compiler-resolved module paths — as const data in the binary.

This means the `.so` produced by a single `cargo build --lib` already contains
everything needed for schema generation. `cargo-pgrx` simply reads the section, builds
the graph, and emits SQL. No second build. No code execution. No stale data. Full
incremental compilation support — the linker always produces a complete `.so`.

### Extended trait: `SqlTranslatable`

Rather than introducing a parallel schema-only trait, we extend
`SqlTranslatable` so it can describe its SQL behavior in a const-friendly form.
The proc macros then read that compile-time metadata directly when emitting
the embedded schema section.

```rust
pub unsafe trait SqlTranslatable {
    /// A compile-time identifier for dependency matching.
    ///
    /// Derive macros set this automatically. Manual implementors should use
    /// `pgrx_resolved_type!()`.
    const SCHEMA_KEY: &'static str;

    /// Declares whether this SQL type is owned by this extension or external.
    ///
    /// Set this explicitly. Use `TypeOrigin::ThisExtension` for extension-owned types that
    /// resolve through `#[derive(PostgresType)]`, `#[derive(PostgresEnum)]`, or
    /// `extension_sql!(..., creates = [Type(T)]/[Enum(T)])`.
    const TYPE_ORIGIN: TypeOrigin;

    /// Const-friendly mirror of `argument_sql()`.
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError>;

    /// Const-friendly mirror of `return_sql()`.
    const RETURN_SQL: Result<ReturnsRef, ReturnsError>;

    // Runtime methods remain as compatibility shims over the const metadata.
    fn type_name() -> &'static str { core::any::type_name::<Self>() }
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Self::ARGUMENT_SQL.into_runtime()
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Self::RETURN_SQL.into_runtime()
    }
    fn entity() -> FunctionMetadataTypeEntity { ... }
}
```

`SqlMappingRef` and `ReturnsRef` are new internal, const-friendly mirrors of
`SqlMapping` and `Returns`. They use only `&'static str`, booleans, and small
enums, so proc macros can embed them without executing code or allocating
`String`s. This preserves the historical `SqlTranslatable` source of truth for
SQL spellings while making that information available during the single library
build.

This is a **breaking change**: existing manual `unsafe impl SqlTranslatable`
blocks must provide compile-time schema metadata, including `TYPE_ORIGIN`, not
just runtime methods. The migration is still mechanical, but it is larger than
a one-line change.

### Blanket / generic `SqlTranslatable` impls

pgrx provides blanket `SqlTranslatable` impls for wrapper types: `Option<T>`,
`Vec<T>`, `&T`, `&mut T`, `*mut T`, `Result<T, E>`, `Array<T>`,
`VariadicArray<T>`, `PgBox<T>`, `PgVarlena<T>`, and others. Under this RFC,
those impls compose the const metadata the same way they compose the runtime
methods today.

For example:

```rust
unsafe impl<T: SqlTranslatable> SqlTranslatable for Option<T> {
    const SCHEMA_KEY: &'static str = T::SCHEMA_KEY;
    const TYPE_ORIGIN: TypeOrigin = T::TYPE_ORIGIN;
    const ARGUMENT_SQL: SqlMappingRef = T::ARGUMENT_SQL;
    const RETURN_SQL: ReturnsRef = T::RETURN_SQL;
}

unsafe impl<T, E> SqlTranslatable for Result<T, E>
where
    T: SqlTranslatable,
{
    const SCHEMA_KEY: &'static str = T::SCHEMA_KEY;
    const TYPE_ORIGIN: TypeOrigin = T::TYPE_ORIGIN;
    const ARGUMENT_SQL: SqlMappingRef = T::ARGUMENT_SQL;
    const RETURN_SQL: ReturnsRef = T::RETURN_SQL;
}
```

This is the key compatibility lever: instead of teaching the proc macro a
separate built-in table, we keep `SqlTranslatable` as the single source of truth
for built-ins, wrappers, and manual/custom SQL-backed types.

Argument-list properties such as nullability and `VARIADIC` stay with pgrx's
argument-shape parsing, not `SqlTranslatable`.

### Helper macro: `pgrx_resolved_type!()`

A convenience macro is provided so users don't have to hand-write the
`concat!()` / `module_path!()` incantation:

```rust
/// Produces a compile-time string identifying this type for schema generation.
/// Use in manual `SqlTranslatable` implementations:
///
/// ```rust
/// unsafe impl SqlTranslatable for HexInt {
///     const SCHEMA_KEY: &'static str = pgrx::pgrx_resolved_type!(HexInt);
///     const ARGUMENT_SQL: SqlMappingRef = SqlMappingRef::literal("hexint");
///     const RETURN_SQL: ReturnsRef = ReturnsRef::one(SqlMappingRef::literal("hexint"));
/// }
/// ```
#[macro_export]
macro_rules! pgrx_resolved_type {
    ($ty:ty) => {
        concat!(module_path!(), "::", stringify!($ty))
    };
}
```

The type name must be passed explicitly because `stringify!(Self)` inside a
trait impl produces the literal string `"Self"`, not the concrete type name.

**Why `SCHEMA_KEY` has no default:** If a trait default used `module_path!()`, it
would resolve to `pgrx_sql_entity_graph::metadata::sql_translatable` — the
module where the trait is defined, not where the user's type lives.
`module_path!()` expands to the module where it is *lexically written*, so it
only produces the correct result when emitted at the user's definition or impl
site intentionally. Omitting the default forces every manual impl to make that
choice explicitly, catching errors at compile time rather than producing wrong
schema output silently.

### Meaning of `SCHEMA_KEY`

`SCHEMA_KEY` is the compile-time dependency identity of a Rust type for schema
generation. It is not the SQL spelling of the type.

- For derive-generated types and enums, `SCHEMA_KEY` is canonical and generated
  automatically.
- For aliases and re-exports, the compiler resolves the same underlying
  `SqlTranslatable` impl, so they naturally reuse the same `SCHEMA_KEY`.
- For wrapper impls that should order against the underlying SQL type
  (`Option<T>`, `Result<T, E>`, `Vec<T>`, `Array<T>`, `PgBox<T>`, and similar),
  the wrapper delegates its `SCHEMA_KEY` to the inner type.
- For manual impls, the supported/default choice is
  `pgrx_resolved_type!(ConcreteType)`.

The RFC relies on the normal Rust type system to catch ordinary trait conflicts
and overlapping impl mistakes at compile time. It does not introduce a separate
pgrx-specific validation regime for "bad but compilable" manually invented
`SCHEMA_KEY` strings. Authors are expected to treat `SCHEMA_KEY` as the
canonical identity of the underlying Rust type they are describing.

### How entity data is embedded in the `.so`

Each proc macro generates const entity data and places its serialized bytes in the
embedded schema section. For type-bearing fields, the generated code references
the relevant `SqlTranslatable` associated consts, so the compiler resolves both the
type identity and the SQL mapping during the single library build.

The entity data is not assembled by executing user code. Instead, each proc macro
constructs an internal const-friendly entity struct, then serializes that struct into
binary section bytes through a generated helper. Type-dependent fields now come from
compiler-resolved associated consts instead of a proc-macro-maintained lookup table, and
the proc macro only needs to emit const-friendly writer expressions for the fixed and
type-derived fields that make up each entity.

**What the proc macro generates** (example for `#[pg_extern]`):

```rust
::pgrx::pgrx_sql_entity_graph::__pgrx_schema_entry!(
    __pgrx_schema_fn_my_func,
    TOTAL_LEN,
    {
        let writer =
            ::pgrx::pgrx_sql_entity_graph::section::EntryWriter::<TOTAL_LEN>::new()
                .u32(PAYLOAD_LEN as u32)
                .u8(::pgrx::pgrx_sql_entity_graph::section::ENTITY_FUNCTION)
                .str("my_func")
                .str("my_func")
                .str(core::module_path!())
                .str(concat!(core::module_path!(), "::", "my_func"))
                .u32(2)
                .str("input")
                .str("MyType")
                .str("MyType")
                .bool(false)
                .bool(false)
                .bool(false)
                .str(<MyType as SqlTranslatable>::SCHEMA_KEY)
                .argument_sql(<MyType as SqlTranslatable>::ARGUMENT_SQL)
                .return_sql(<MyType as SqlTranslatable>::RETURN_SQL);
        writer.finish()
    }
);
```

Key implementation details:

- **Const section encoding** happens in the proc macro through
  `section::EntryWriter::<N>` chains. Type-dependent pieces such as schema identity and
  SQL mappings come from generated references to
  `SqlTranslatable::{SCHEMA_KEY, TYPE_ORIGIN, ARGUMENT_SQL, RETURN_SQL}`.

- **Unique static names** are generated by the proc macro using `format_ident!()`
  (from `proc_macro2`), not by declarative macro name-concatenation. This avoids the
  nightly-only `${concat(...)}` metavar feature.

- **Platform-conditional section names.** macOS Mach-O requires `"segment,section"`
  format. The proc macro emits `cfg_attr` to use `"__DATA,__pgrxsc"` on macOS and
  `".pgrxsc"` on all other platforms. The short names keep the section within the
  8-byte PE/COFF image-name limit used on Windows.

- **Section reading in `cargo-pgrx`** uses the `object` crate on ELF and custom Mach-O
  readers for thin and fat binaries. The reader extracts the schema section bytes and
  passes them through `decode_entities()`.

After compilation, the section contains length-prefixed binary entries concatenated by
the linker. `cargo-pgrx` reads the section, decodes each entry in sequence, and stops
cleanly at any trailing zero padding introduced by the linker.

### How `cargo-pgrx` changes

#### New schema generation flow

```rust
fn generate_schema(
    so_path: &Path,
    control_file_path: &Path,
    output: &Path,
    dot: Option<&Path>,
) -> eyre::Result<()> {
    // 1. Read the .so and extract the embedded schema section
    let so_data = std::fs::read(so_path)?;
    let section_data = schema_section_data(&so_data)?
        .ok_or_else(|| eyre!("no embedded pgrx schema section — is this a pgrx extension?"))?;

    // 2. Decode binary entity entries from the section
    let entities = decode_entities(section_data)?;

    // 3. Build the dependency graph
    let control_file = ControlFile::try_from(control_file_path)?;
    let pgrx_sql = PgrxSql::build(entities.into_iter(), lib_name, versioned_so)?;

    // 4. Write SQL
    pgrx_sql.to_file(output)?;

    // 5. Optional: write Graphviz DOT
    if let Some(dot_path) = dot {
        pgrx_sql.to_dot(dot_path)?;
    }

    Ok(())
}
```

The section-reading helpers in `cargo-pgrx` are retained in simplified form. The
symbol-table scanning, codegen, second build, and binary execution are all deleted.

#### Stripping the section from installed artifacts

This RFC does not require automatic stripping as part of `cargo pgrx install`. The
embedded schema section is inert at runtime, and install/package-time artifact trimming
can be layered on later if it proves worthwhile. The important behavior for this RFC is
that schema generation reads the section from the freshly built shared object and does
not require a second build or an executable helper binary.

### How proc macros change

#### `#[derive(PostgresType)]`

**Currently generates:**
- `__pgrx_internals_type_{name}` function containing `TypeId` mappings via
  `WithTypeIds`, `WithSizedTypeIds`, `WithArrayTypeIds`, `WithVarlenaTypeIds`
- `FromDatum`, `IntoDatum`, input/output functions, `PostgresType` marker trait

**Will now generate:**
- `unsafe impl SqlTranslatable for MyType` with `SCHEMA_KEY` and const SQL metadata
  set automatically
- All existing trait impls (`FromDatum`, `IntoDatum`, `InOutFuncs`, etc.) — unchanged
- Input/output `#[pg_extern]` functions — unchanged
- **A `#[link_section]` const** containing the type entity metadata in the embedded
  schema section

**No longer generates:**
- The `__pgrx_internals_type_{name}` function
- `WithTypeIds` / `WithSizedTypeIds` / `WithArrayTypeIds` / `WithVarlenaTypeIds`
  registration calls

#### `#[derive(PostgresEnum)]`

Same pattern. The entity metadata (enum name, variants, module path) goes into the
embedded schema section.

#### `#[pg_extern]` (also `#[pg_operator]`, `#[pg_cast]`)

**Will now generate:**
- The Postgres-callable function — unchanged
- Compile-time verification for every normalized argument type and every normalized
  return leaf type (see below)
- Function metadata equivalent to today's `FunctionMetadataEntity`, but assembled at
  compile time from `SqlTranslatable` consts instead of by executing compiled code
- **A `#[link_section]` const** containing the function entity metadata in the embedded
  schema section

**Key detail about type resolution:** The proc macro no longer owns a hardcoded
built-in table. Instead:

- Syntactic wrappers already understood by pgrx's parsing layer (`default!()`,
  `composite_type!()`, `Option<T>`, `Result<T, E>`, `Vec<T>`, `Array<T>`,
  `VariadicArray<T>`, `SetOfIterator<T>`, `TableIterator<(...)>`, etc.) are
  normalized exactly as they are today.
- For each resulting leaf type, the generated code references
  `SqlTranslatable::{SCHEMA_KEY, TYPE_ORIGIN, ARGUMENT_SQL, RETURN_SQL}`.
- pgrx's own built-ins (`PgRelation`, `TimeWithTimeZone`, `Range<T>`, `PgBox<T>`,
  `PgVarlena<T>`, and so on) continue to work by implementing those associated consts
  in pgrx itself, rather than by being duplicated in proc-macro lookup tables.

#### Compile-time verification in `#[pg_extern]`

For each normalized argument type, the proc macro generates const assertions against
the schema metadata:

```rust
const _: () = {
    let _ = <MyType as SqlTranslatable>::SCHEMA_KEY;
    let _ = <MyType as SqlTranslatable>::ARGUMENT_SQL;
};

const _: () = {
    let _ = <ReturnType as SqlTranslatable>::SCHEMA_KEY;
    let _ = <ReturnType as SqlTranslatable>::RETURN_SQL;
};
```

For `SetOfIterator<T>` and `TableIterator<(name!(col, T), ...)>`, the proc macro emits
the same checks for each leaf `T`. This restores the current args-and-returns
compile-time safety of `FunctionMetadata` without keeping a runtime
`FunctionMetadata` trait in the schema-generation path.

#### `#[derive(PostgresOrd)]`, `#[derive(PostgresHash)]`

Each writes a small entity entry to the embedded schema section with the type name and
module path.
The graph builder connects them to their type entity by `SCHEMA_KEY`.

#### `#[pg_aggregate]`, `#[pg_trigger]`, `#[pg_schema]`

Each writes its entity data to the embedded schema section.

#### `extension_sql!()` and `extension_sql_file!()`

Each writes its entity data (SQL text, `requires` list, `creates` list,
`bootstrap`/`finalize` flags) to the embedded schema section.

### Graph builder changes

The `PgrxSql` graph builder is modified to match entities by `SCHEMA_KEY` instead of
`TypeId`.

**Type matching algorithm:**
1. For each type-bearing edge source (function arg, function return, aggregate state
   type, `creates = [Type(T)]`, etc.), read the embedded `SCHEMA_KEY`
2. Search registered type, enum, and `extension_sql!()` `creates` declarations for a
   matching `SCHEMA_KEY`
3. In a valid extension, this yields at most one schema-emitting dependency target
4. If no schema entity matches and `TYPE_ORIGIN == TypeOrigin::External`, treat the
   type as an external SQL type and render its SQL spelling from the embedded
   `ARGUMENT_SQL` / `RETURN_SQL` metadata
5. Otherwise, fail with an unresolved-schema-key error

For `PostgresOrd`/`PostgresHash`/aggregate connections, the same `SCHEMA_KEY` matching
logic applies instead of `TypeId`.

### What gets deleted

| Component | Location |
|-----------|----------|
| `second_build()` | `cargo-pgrx/src/command/schema.rs` |
| `compute_codegen()` | `cargo-pgrx/src/command/schema.rs` |
| `compute_sql()` | `cargo-pgrx/src/command/schema.rs` |
| `compute_symbols()` / `find_and_compute_symbols()` | `cargo-pgrx/src/command/schema.rs` |
| Symbol table scanning logic | `cargo-pgrx/src/command/schema.rs` |
| `pgrx_embed!()` macro | `pgrx/src/lib.rs` |
| `pgrx_embed_rs` template | `cargo-pgrx/src/templates/` |
| `__pgrx_internals_*` function generation | all entity modules in `pgrx-sql-entity-graph/src/` |
| `TypeId` fields | `RustSqlMapping`, `UsedTypeEntity`, `PostgresHashEntity`, `PostgresOrdEntity`, `PgAggregateEntity` |
| `TypeMatch` / `id_matches()` trait | `pgrx-sql-entity-graph/src/lib.rs` |
| `WithTypeIds` trait and helpers | `pgrx/src/datum/with_typeid.rs` |
| `WithSizedTypeIds` struct | `pgrx/src/datum/with_typeid.rs` |
| `WithArrayTypeIds` struct | `pgrx/src/datum/with_typeid.rs` |
| `WithVarlenaTypeIds` struct | `pgrx/src/datum/with_typeid.rs` |
| `nonstatic_typeid()` | `pgrx/src/datum/with_typeid.rs` |
| `FunctionMetadata` trait | `pgrx-sql-entity-graph/src/metadata/function_metadata.rs` |

### What stays the same

| Component | Why |
|-----------|-----|
| `FromDatum` / `IntoDatum` traits | Not related to schema generation |
| `SqlTranslatable` trait | Still the source of truth for SQL translation; extended with required const schema metadata |
| `ArgAbi` / `BoxRet` / `UnboxDatum` | Calling convention, not schema |
| `InOutFuncs` / `PgVarlenaInOutFuncs` | I/O format, not schema |
| `PgrxSql` graph structure | Same graph, same Tarjan SCC ordering |
| `ToSql` trait on entities | Same SQL emission logic |
| `PositioningRef` / `requires` system | Same explicit ordering mechanism |
| `SqlDeclaredEntity` / `creates` system | Same declaration mechanism, but enriched with `SCHEMA_KEY`/SQL metadata for type-bearing declarations |
| `extension_sql!()` / `extension_sql_file!()` semantics | Same, section-based now |
| `object` crate dependency in `cargo-pgrx` | Retained — reads section instead of symbol table |
| Mach-O and fat-binary section readers | Retained in simplified form |
| Most derive-based user APIs | Unchanged; the notable breaking changes are manual `SqlTranslatable` implementations |

### Why the staleness problem doesn't exist

With the file-based approach, a deleted function's entity file would persist across
builds, producing ghost entities in the schema. The link section approach eliminates
this entirely:

- **The `.so` is always the single source of truth.** The linker produces a complete
  `.so` on every build, even incremental builds. If a function is deleted, its const
  static is removed from the `.so` at the next link.
- **Incremental compilation works perfectly.** Changed files recompile, their proc
  macros re-run, and the linker produces an updated `.so`. Unchanged files retain
  their existing object code (including link section data). The result is always
  a complete, current `.so`.
- **No `cargo clean` needed.** No stale files, no generation counters, no cleanup.
- **No `CARGO_INCREMENTAL=0` needed.** Full incremental compilation support.

## Impact on Extension Authors

### Common case: derive macros only

**Users who use `#[derive(PostgresType)]`, `#[derive(PostgresEnum)]`, `#[pg_extern]`,
etc. exclusively** — the vast majority of pgrx users:

**No code changes required.**

All derive macros continue to work identically. The only user-visible change is:
- `src/bin/pgrx_embed.rs` is no longer needed and can be deleted
- The `[[bin]]` target for `pgrx_embed` can be removed from `Cargo.toml`
- Builds are faster (single pass)

`cargo pgrx new` templates will be updated to omit these files. Existing projects can
remove them at their leisure — their presence does not break anything.

### Manual `SqlTranslatable` implementations

**Users who implement `unsafe impl SqlTranslatable for MyType`** — this is common for
types that bypass `#[derive(PostgresType)]` (for example, the `HexInt` type in
`pgrx-examples/custom_types`):

**Small but mechanical migration required.** Manual impls now provide compile-time
schema metadata in addition to (or instead of) runtime methods:

```rust
unsafe impl SqlTranslatable for HexInt {
    const SCHEMA_KEY: &'static str = pgrx::pgrx_resolved_type!(HexInt);
    const TYPE_ORIGIN: TypeOrigin = TypeOrigin::ThisExtension;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> =
        Ok(SqlMappingRef::literal("hexint"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("hexint")));
}
```

If an existing impl's SQL behavior can be expressed with static strings, wrappers, and
the structured `SqlMappingRef` / `ReturnsRef` forms, the migration is straightforward.
If an impl depends on runtime-only computation, that capability is intentionally no
longer part of the automatic schema path; users must rewrite it as static metadata.

Manual impls now have two supported modes:

- extension-owned SQL types: use `pgrx::pgrx_resolved_type!(T)` and set
  `TYPE_ORIGIN = TypeOrigin::ThisExtension`
- mappings to an existing SQL type such as `TEXT` or `uuid`: set
  `TYPE_ORIGIN = TypeOrigin::External`

### Manual `extension_sql!()` type creation

**Users who create types via raw SQL** (e.g., the `HexInt` pattern with
`extension_sql!("CREATE TYPE hexint ...", creates = [Type(HexInt)])`) keep the same
surface syntax. Internally, `creates = [Type(HexInt)]` now records `HexInt`'s
`SCHEMA_KEY` alongside the declaration.

Declared type and enum entries stay lean: they carry `SCHEMA_KEY` plus the SQL
mapping needed for rendering, but they do not duplicate `TYPE_ORIGIN`.

This is important for manual/custom SQL-backed types:

- dependency edges come from `SCHEMA_KEY` matching
- actual SQL spellings for function args/returns come from `HexInt`'s
  `ARGUMENT_SQL` / `RETURN_SQL`, not from the Rust identifier `HexInt`

That preserves historically supported patterns where the Rust type name and the SQL
type name differ.

`creates = [Type(T)]/[Enum(T)]` is only valid for extension-owned types. If
`T::TYPE_ORIGIN == TypeOrigin::External`, the declaration should fail instead of
letting raw SQL claim ownership of an external type.

### Types from dependency crates

Types from dependency crates can continue to appear in function signatures as long as
they implement the const-friendly `SqlTranslatable` metadata required by this RFC.
If they are not registered schema entities, the graph builder treats them as
external SQL types and renders their SQL spelling from the embedded
metadata. If they cannot provide const metadata, they are outside the automatic
single-pass schema model described by this RFC.

### `#[bikeshed_postgres_type_manually_impl_from_into_datum]`

**No changes needed.** The derive macro still embeds the type entity data in
the embedded schema section. The attribute only affects which datum-conversion traits are
generated — orthogonal to schema generation.

### `PostgresEq`, `PostgresOrd`, `PostgresHash` derives

**No changes needed** from the user's perspective. `PostgresOrd` and `PostgresHash`
embed their entity data in the embedded schema section and the graph builder connects them to their
type entity by `SCHEMA_KEY`.

### `#[pg_aggregate]` implementations

**No changes needed.** The proc macro embeds aggregate entity data in the embedded schema section.

### Custom `#[pg_extern]` argument types via `SqlTranslatable`

**Same mechanical migration.** Users who implement `SqlTranslatable` for wrapper/newtype
types used in function signatures add `SCHEMA_KEY` plus const SQL metadata. Once they
do, those types work for both arguments and return positions under the single-pass
scheme.

## Type Matching Strategy: Detailed Rules

### 1. Type identity comes from `SCHEMA_KEY`

For every normalized leaf type appearing in function args, function returns, aggregate
state, `creates = [Type(T)]`, and similar positions, the proc macro records
`<T as SqlTranslatable>::SCHEMA_KEY`.

`creates = [Type(T)]/[Enum(T)]` is only valid when `T::TYPE_ORIGIN ==
TypeOrigin::ThisExtension`.

That becomes the primary dependency-matching key across the graph:

1. Search registered type and enum entities for a matching `SCHEMA_KEY`
2. Search `extension_sql!()` `creates` declarations for a matching `SCHEMA_KEY`
3. If a match is found, create the dependency edge
4. If no schema entity matches and `TYPE_ORIGIN == TypeOrigin::External`, treat the
   type as an external SQL type
5. Otherwise, fail

### 2. SQL rendering comes from `SqlTranslatable` const metadata

The SQL spelling of a type is not derived from the Rust token and is not guessed from
`creates = [Type(T)]`. Instead:

- function argument SQL comes from `ARGUMENT_SQL`
- function return SQL comes from `RETURN_SQL`
- wrapper behavior (`Result<T, E>`, `Option<T>`, `Vec<T>`, arrays, etc.) comes from the
  blanket impls' composed const metadata

This preserves cases like a Rust type `HexInt` that must render as SQL `hexint`.

### 3. Local SQL type overrides are deferred

This RFC does not rely on `#[pgrx(sql_type = "...")]` or define new local
type-spelling override behavior. The single-pass design here is intentionally
specified in terms of `SqlTranslatable` metadata plus existing entity-level SQL
customization mechanisms.

If pgrx later adds or retains per-argument/per-return SQL spelling overrides,
they should be specified as a follow-up design, not as a hidden dependency of
this RFC.

## Known Limitations and Edge Cases

### Type aliases

For types that implement `SqlTranslatable`, aliases now work naturally: the compiler
resolves `<MyAlias as SqlTranslatable>::SCHEMA_KEY` and the associated SQL metadata
through the underlying impl, so `type MyAlias = MyType;` behaves the same as `MyType`
for schema generation.

### Re-exported types

Re-exports are no longer special. As long as the re-exported type resolves to the same
`SqlTranslatable` impl, it carries the same `SCHEMA_KEY` and the same SQL metadata.

### Generic types

pgrx still does not support generic type parameters in `#[pg_extern]` function
signatures (e.g., `fn foo<T: PostgresType>(x: T)` is not valid pgrx). Concrete generic
instantiations can work if their `SqlTranslatable` impl exposes a stable `SCHEMA_KEY`
and const SQL metadata, but the automatic path is intentionally limited to cases that
can be described without runtime computation.

### `#[cfg]`-gated entities

Entities behind `#[cfg(feature = "foo")]` are only compiled when the feature is active.
Since the const static in the embedded schema section is part of the generated code, it is subject
to the same `#[cfg]` gating. This means the section only contains entities for the
active configuration — same behavior as today with `__pgrx_internals_*` symbols.

### `extension_sql_file!()` with large SQL content

`extension_sql_file!()` uses `include_str!()` to read SQL files at compile time. The
SQL content is embedded in the schema section as a length-prefixed UTF-8 string
field in the binary section format. For extensions with large bootstrap SQL, this
increases section size proportionally. This is acceptable - even 100KB of SQL produces
a manageable section size.

The proc macro emits the SQL body bytes as a normal string field in the generated
`EntryWriter` chain. No JSON escaping layer is involved.

### LTO and static deduplication

Link-Time Optimization may merge identical statics. If two entities produced
byte-identical section entries (unlikely in practice due to differing names and line
numbers), LTO could merge them, losing one entity. The generated statics use unique
names and are marked `#[used]`, which preserves them in practice.

### Multi-crate workspace extensions

When types are defined in a library crate and functions in the extension crate, both
produce schema-section data in their respective object files. The linker concatenates
sections from all linked object files (including from `.rlib` archives) into the final
`.so`. This is standard linker behavior for named sections — verified on both ELF and
Mach-O.

## Section Format

The embedded schema section (`.pgrxsc` on ELF/PE, `__DATA,__pgrxsc` on Mach-O)
contains a binary stream of concatenated entity entries. Each
entry is encoded as:

```text
[u32 little-endian payload length][payload bytes]
```

The first byte of each payload is an entity tag identifying which entity decoder to run
(`ENTITY_SCHEMA`, `ENTITY_CUSTOM_SQL`, `ENTITY_FUNCTION`, and so on). The remaining
fields are encoded in a compact const-friendly format:

- `u8`: one byte
- `bool`: `0` or `1`
- `u32`: 4 bytes, little-endian
- strings: `[u32 little-endian byte length][UTF-8 bytes]`
- optionals: `[bool present][value if present]`
- lists: `[u32 count][items...]`
- `Result<T, E>` fields such as `ARGUMENT_SQL` and `RETURN_SQL`: one-byte tag followed
  by the encoded `Ok` or `Err` payload

The decoder reads entries sequentially until the section is exhausted. Trailing zero
padding is tolerated, since some linkers may insert alignment bytes between or after
section fragments.

Example logical contents (3 entities):

- type entity for `MyType`
- function entity for `my_func(MyType, i32) -> text`
- schema entity for `my_schema`

## Future Work

### `type_name::<T>()` becoming `const fn`

If `core::any::type_name::<T>()` is stabilized as `const fn`
(tracking: [rust-lang/rust#63084](https://github.com/rust-lang/rust/issues/63084)),
we can embed compiler-resolved fully-qualified type names directly into the section
data for diagnostics, consistency checks, and richer debug output.

### Automatic alias detection

The `SCHEMA_KEY` design already handles aliases and re-exports for
`SqlTranslatable` types. If `type_name::<T>()` becomes `const fn`, we can additionally
record the compiler-resolved full type name in the emitted metadata and use it as a
debugging aid or to validate that a manual `SCHEMA_KEY` matches the underlying type the
user intended.

### Local SQL type overrides

Per-position override attributes such as `#[pgrx(sql_type = "...")]` are deferred
from this RFC. If we reintroduce them later, they should be defined as a small,
local rendering feature layered on top of the `SCHEMA_KEY` model, not as a
replacement for it.

## Compatibility Notes

### MSRV

The `const { ... }` block syntax in static initializer position (used for the byte
array construction) was stabilized in Rust 1.79. pgrx's current toolchain is 1.90.0,
so this is not a concern.

### `pgrx-sql-entity-graph` semver

`pgrx-sql-entity-graph` is a published crate but its APIs are documented as internal
to pgrx. No downstream compatibility is promised or maintained. This RFC is part of a
major pgrx version bump.

## Migration Guide

### For extension authors

1. **Delete `src/bin/pgrx_embed.rs`** and remove the `[[bin]]` target from
   `Cargo.toml`.

2. **Add compile-time schema metadata to every manual `unsafe impl SqlTranslatable`
   block.** At minimum that means `SCHEMA_KEY`, `TYPE_ORIGIN`, `ARGUMENT_SQL`,
   and `RETURN_SQL`:
   ```rust
   const SCHEMA_KEY: &'static str = pgrx::pgrx_resolved_type!(YourType);
   const TYPE_ORIGIN: TypeOrigin = TypeOrigin::ThisExtension;
   const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> =
       Ok(SqlMappingRef::literal("your_sql_type"));
   const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
       Ok(ReturnsRef::One(SqlMappingRef::literal("your_sql_type")));
   ```

   If the Rust type maps to a SQL type your extension already declares, leave
   `TYPE_ORIGIN = TypeOrigin::ThisExtension` and make sure the type resolves through
   `#[derive(PostgresType)]`, `#[derive(PostgresEnum)]`, or
   `extension_sql!(..., creates = [Type(T)]/[Enum(T)])`.

   If the Rust type intentionally maps to an external SQL type such as `TEXT`,
   `uuid`, or `regclass`, add:
   ```rust
   const TYPE_ORIGIN: TypeOrigin = TypeOrigin::External;
   ```

   `creates = [Type(T)]/[Enum(T)]` is not valid for those external mappings.

3. Types using `#[derive(PostgresType)]`, `#[derive(PostgresEnum)]`, and all other
   derive macros need **no manual changes** — the derives generate the const schema
   metadata automatically.

### For pgrx contributors

1. **`pgrx-sql-entity-graph`** — Each entity module's proc macro expansion changes
   from generating `__pgrx_internals_*` functions to emitting `#[link_section]` const
   statics. The entity structs lose their `TypeId` fields. The graph builder switches
   to `SCHEMA_KEY`-based matching and consumes const-friendly SQL metadata.

2. **`pgrx-macros`** — The `pg_extern`, `pg_operator`, `pg_cast` macro implementations
   gain the `#[link_section]` entity embedding and compile-time verification logic for
   both normalized args and normalized returns.

3. **`cargo-pgrx`** — The `schema.rs` command module is substantially simplified:
   `first_build` remains; symbol scanning, codegen, second build, and binary execution
   are replaced with a single section read + binary entity decode.

4. **`pgrx`** — The `pgrx_embed!()` macro and `WithTypeIds` infrastructure are removed.
   `SqlTranslatable` gains required const schema metadata (`SCHEMA_KEY`,
   `ARGUMENT_SQL`, `RETURN_SQL`, plus const-friendly mirrors of the runtime enums).
   The `pgrx_resolved_type!()` and `__pgrx_schema_entity!()` helper macros are added.

5. **Templates** — `cargo pgrx new` templates are updated to omit `pgrx_embed.rs` and
   the `[[bin]]` target.

## Appendix A: Complete Entity Inventory

| Entity kind | Embedded by | Replaces |
|-------------|-----------|----------|
| `type` | `#[derive(PostgresType)]` | `__pgrx_internals_type_{name}` |
| `enum` | `#[derive(PostgresEnum)]` | `__pgrx_internals_enum_{name}` |
| `function` | `#[pg_extern]` / `#[pg_operator]` / `#[pg_cast]` | `__pgrx_internals_fn_{name}` |
| `schema` | `#[pg_schema]` | `__pgrx_internals_schema_{name}_{hash}` |
| `ord` | `#[derive(PostgresOrd)]` | `__pgrx_internals_ord_{name}` |
| `hash` | `#[derive(PostgresHash)]` | `__pgrx_internals_hash_{name}` |
| `aggregate` | `#[pg_aggregate]` | `__pgrx_internals_aggregate_{name}` |
| `trigger` | `#[pg_trigger]` | `__pgrx_internals_trigger_{name}` |
| `sql` | `extension_sql!()` / `extension_sql_file!()` | `__pgrx_internals_sql_{name}` |

## Appendix B: Platform Notes

### Link section naming

Mach-O requires `"segment,section"` format; ELF uses a plain section name. The proc
macros use `cfg_attr` to emit the correct attribute:

```rust
#[cfg_attr(target_os = "macos", link_section = "__DATA,__pgrxsc")]
#[cfg_attr(not(target_os = "macos"), link_section = ".pgrxsc")]
```

- **Linux (ELF):** Section name `.pgrxsc`
- **macOS (Mach-O):** Segment `__DATA`, section `__pgrxsc`
- **Windows (PE/COFF):** Section name `.pgrxsc`, which stays within the 8-byte image
  section-name limit.

### Reading the section

```rust
let data = std::fs::read(&so_path)?;
let raw = schema_section_data(&data)?
    .ok_or_else(|| eyre!("no embedded pgrx schema section — is this a pgrx extension?"))?;
```

On Mach-O, `cargo-pgrx` must handle both thin binaries and fat binaries, so the reader
selects the active architecture slice first and then scans load commands for the schema
section. On ELF, the same helper falls back to the `object` crate's normal section
lookup.
