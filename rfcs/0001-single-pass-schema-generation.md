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
dedicated linker section (`.pgrx_schema`) during the single `cargo build --lib`. After
the build, `cargo-pgrx` reads that section from the `.so`, builds the dependency
graph, emits SQL, and optionally strips the section from the installed artifact.

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

Rather than introducing a new trait, we extend the existing `SqlTranslatable` trait
with a single const:

```rust
pub unsafe trait SqlTranslatable {
    /// A compile-time identifier for this type used during schema generation.
    /// Derive macros set this automatically. Manual implementors should use
    /// the `pgrx_resolved_type!()` helper macro.
    ///
    /// No default is provided — this is intentionally a required const so that
    /// a missing implementation is a compile error, not a silent schema bug.
    const SCHEMA_KEY: &'static str;

    // Existing methods — all unchanged
    fn type_name() -> &'static str { core::any::type_name::<Self>() }
    fn argument_sql() -> Result<SqlMapping, ArgumentError>;
    fn return_sql() -> Result<Returns, ReturnsError>;
    fn variadic() -> bool { false }
    fn optional() -> bool { false }
    fn entity() -> FunctionMetadataTypeEntity { ... }
}
```

This is a **breaking change**: existing `unsafe impl SqlTranslatable` blocks that do
not set `SCHEMA_KEY` will fail to compile. The fix is one line — see migration guide.

### Blanket / generic `SqlTranslatable` impls

pgrx provides blanket `SqlTranslatable` impls for wrapper types: `Option<T>`, `Vec<T>`,
`&T`, `&mut T`, `*mut T`, `Result<T, E>`, `Array<T>`, `VariadicArray<T>`, `PgBox<T>`,
`PgVarlena<T>`, etc. These impls delegate SQL mapping behavior to their inner type `T`.

For `SCHEMA_KEY`, these blanket impls delegate to the inner type:

```rust
unsafe impl<T: SqlTranslatable> SqlTranslatable for Option<T> {
    const SCHEMA_KEY: &'static str = T::SCHEMA_KEY;
    fn argument_sql() -> Result<SqlMapping, ArgumentError> { T::argument_sql() }
    fn return_sql() -> Result<Returns, ReturnsError> { T::return_sql() }
    fn optional() -> bool { true }
}
```

This is correct: `Option<MyType>` should have the same schema key as `MyType`, since
the wrapper types are unwrapped to their inner type during schema generation. The graph
builder matches on the inner type's name, not the wrapper's.

### Helper macro: `pgrx_resolved_type!()`

A convenience macro is provided so users don't have to hand-write the `concat!()` /
`module_path!()` incantation:

```rust
/// Produces a compile-time string identifying this type for schema generation.
/// Use in manual `SqlTranslatable` implementations:
///
/// ```rust
/// unsafe impl SqlTranslatable for HexInt {
///     const SCHEMA_KEY: &'static str = pgrx::pgrx_resolved_type!(HexInt);
///     fn argument_sql() -> Result<SqlMapping, ArgumentError> { ... }
///     fn return_sql() -> Result<Returns, ReturnsError> { ... }
/// }
/// ```
#[macro_export]
macro_rules! pgrx_resolved_type {
    ($ty:ty) => {
        concat!(module_path!(), "::", stringify!($ty))
    };
}
```

The type name must be passed explicitly because `stringify!(Self)` inside a trait impl
produces the literal string `"Self"`, not the concrete type name.

**Why `SCHEMA_KEY` has no default:** If a trait default used `module_path!()`, it would
resolve to `pgrx_sql_entity_graph::metadata::sql_translatable` — the module where the
trait is defined, not where the user's type lives. `module_path!()` expands to the
module where it is *lexically written*, so it only produces the correct result when
emitted at the user's definition site. Omitting the default forces every impl to
provide the value explicitly, catching errors at compile time rather than producing
wrong schema output silently.

### How entity data is embedded in the `.so`

Each proc macro generates a const string containing the entity's metadata, and places
it in the `.pgrx_schema` linker section. The string is built using `concat!()` with
`module_path!()` so that `rustc` resolves the module path at compile time.

The entity data is not assembled via declarative macros. Instead, each proc macro
(which runs as a normal Rust program at compile time) constructs the entity string
directly using `format!()` with proper JSON escaping, then emits the result as
generated Rust code. The proc macro has full access to `serde_json` or manual escaping
since it's an ordinary Rust program.

**What the proc macro generates** (example for `#[pg_extern]`):

```rust
// The proc macro builds this string at macro-expansion time using format!()/serde_json.
// The only compiler-evaluated piece is module_path!(), spliced in via concat!().

const __PGRX_ENTITY_FN_MY_FUNC_JSON: &str = concat!(
    // Proc macro emits this prefix with all values JSON-escaped at expansion time:
    "{\"kind\":\"function\",\"name\":\"my_func\",\"module_path\":\"",
    module_path!(),  // filled in by rustc
    "\",\"file\":\"src/lib.rs\",\"line\":42,\"attrs\":[\"immutable\"],",
    "\"args\":[{\"name\":\"input\",\"ty\":\"MyType\"},{\"name\":\"count\",",
    "\"ty\":\"i32\",\"sql\":\"integer\"}],\"ret\":{\"ty\":\"String\",",
    "\"sql\":\"text\"}}\n",
);

// Platform-conditional link section name
#[cfg_attr(target_os = "macos", link_section = "__DATA,.pgrx_schema")]
#[cfg_attr(not(target_os = "macos"), link_section = ".pgrx_schema")]
#[used]
static __PGRX_ENTITY_FN_MY_FUNC_BYTES: [u8; __PGRX_ENTITY_FN_MY_FUNC_JSON.len()] = const {
    let bytes = __PGRX_ENTITY_FN_MY_FUNC_JSON.as_bytes();
    let mut arr = [0u8; __PGRX_ENTITY_FN_MY_FUNC_JSON.len()];
    let mut i = 0;
    while i < bytes.len() {
        arr[i] = bytes[i];
        i += 1;
    }
    arr
};
```

Key implementation details:

- **JSON construction** happens in the proc macro (an ordinary Rust program), which
  has access to `serde_json`, `format!()`, and proper string escaping. The proc macro
  constructs all JSON fragments, escaping identifiers, file paths, and SQL content.
  The only unresolved piece is `module_path!()`, which is spliced into the const via
  `concat!()` and resolved by `rustc`.

- **Unique static names** are generated by the proc macro using `format_ident!()`
  (from `proc_macro2`), not by declarative macro name-concatenation. This avoids the
  nightly-only `${concat(...)}` metavar feature.

- **Platform-conditional section names.** macOS Mach-O requires `"segment,section"`
  format. The proc macro emits `cfg_attr` to use `"__DATA,.pgrx_schema"` on macOS and
  `".pgrx_schema"` on all other platforms.

- **Section reading in `cargo-pgrx`** uses the `object` crate. On ELF,
  `section_by_name(".pgrx_schema")`. On Mach-O, `section_by_name_bytes` matching
  `.pgrx_schema` in the `__DATA` segment. The existing `parse_object()` helper already
  handles Mach-O fat binaries.

After compilation, the section contains all entity entries concatenated, each
terminated by `\n` (0x0A). `cargo-pgrx` reads the section, splits on `\n`, skips
empty lines and any null-byte padding, and parses each JSON object.

### How `cargo-pgrx` changes

#### New schema generation flow

```rust
fn generate_schema(
    so_path: &Path,
    control_file_path: &Path,
    output: &Path,
    dot: Option<&Path>,
) -> eyre::Result<()> {
    // 1. Read the .so and extract the .pgrx_schema section
    let so_data = std::fs::read(so_path)?;
    let obj = parse_object(&so_data)?; // existing helper for fat binaries
    let section = obj.section_by_name(".pgrx_schema")
        .ok_or_else(|| eyre!("no .pgrx_schema section — is this a pgrx extension?"))?;
    let section_data = section.data()?;

    // 2. Parse entity entries (newline-delimited JSON)
    let entities: Vec<EntityData> = section_data
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line))
        .collect::<Result<_, _>>()?;

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

The existing `parse_object()` helper (which already handles macOS fat binaries) is
retained. The symbol-table scanning, codegen, second build, and binary execution are
all deleted.

#### Stripping the section from installed artifacts

After schema generation, `cargo-pgrx install/package` automatically strips the
`.pgrx_schema` section from the `.so` before installing it into the PostgreSQL
extension directory. This removes dead metadata from the runtime artifact.

- **Linux:** `objcopy --remove-section=.pgrx_schema libmy_ext.so`
- **macOS:** equivalent `strip` invocation or `install_name_tool` approach

A `--no-schema-strip` flag is provided for users who want to retain the section in the
installed artifact (e.g., for debugging or third-party tooling that reads entity data
from installed extensions).

### How proc macros change

#### `#[derive(PostgresType)]`

**Currently generates:**
- `__pgrx_internals_type_{name}` function containing `TypeId` mappings via
  `WithTypeIds`, `WithSizedTypeIds`, `WithArrayTypeIds`, `WithVarlenaTypeIds`
- `FromDatum`, `IntoDatum`, input/output functions, `PostgresType` marker trait

**Will now generate:**
- `unsafe impl SqlTranslatable for MyType` with `SCHEMA_KEY` set (the derive already
  generates this impl; it now also sets the new const)
- All existing trait impls (`FromDatum`, `IntoDatum`, `InOutFuncs`, etc.) — unchanged
- Input/output `#[pg_extern]` functions — unchanged
- **A `#[link_section = ".pgrx_schema"]` const** containing the type entity metadata

**No longer generates:**
- The `__pgrx_internals_type_{name}` function
- `WithTypeIds` / `WithSizedTypeIds` / `WithArrayTypeIds` / `WithVarlenaTypeIds`
  registration calls

#### `#[derive(PostgresEnum)]`

Same pattern. The entity metadata (enum name, variants, module path) goes into the
`.pgrx_schema` section.

#### `#[pg_extern]` (also `#[pg_operator]`, `#[pg_cast]`)

**Will now generate:**
- The Postgres-callable function — unchanged
- Compile-time verification for each argument type (see below)
- **A `#[link_section = ".pgrx_schema"]` const** containing the function entity metadata

**Key detail about argument types:** For built-in types (`i32`, `i64`, `bool`, `f32`,
`f64`, `String`, `&str`, `&CStr`, `Option<T>`, `Vec<T>`, `pg_sys::Oid`,
`AnyNumeric`, `Date`, `Time`, `Timestamp`, `TimestampWithTimeZone`, etc.), the proc
macro resolves the SQL mapping directly — it has a hardcoded table of token → SQL type.
These appear as `"sql":"integer"` in the entity data. For custom types, the `sql` field
is omitted, and the graph builder resolves it by matching the type token to a registered
type/enum entity by name.

#### Compile-time verification in `#[pg_extern]`

For each argument whose type token does not match a known built-in type, the proc
macro generates a const assertion using `SqlTranslatable`:

```rust
const _: () = {
    // If MyType doesn't implement SqlTranslatable, this is a compile error.
    // The compiler resolves MyType through its normal name resolution — the proc
    // macro only sees the token "MyType", but rustc knows exactly what it is.
    let _ = <MyType as SqlTranslatable>::SCHEMA_KEY;
};
```

This provides the same compile-time safety that `FunctionMetadata` provides today.

#### `#[derive(PostgresOrd)]`, `#[derive(PostgresHash)]`

Each writes a small entity entry to `.pgrx_schema` with the type name and module path.
The graph builder connects them to their type entity by name match.

#### `#[pg_aggregate]`, `#[pg_trigger]`, `#[pg_schema]`

Each writes its entity data to `.pgrx_schema`.

#### `extension_sql!()` and `extension_sql_file!()`

Each writes its entity data (SQL text, `requires` list, `creates` list,
`bootstrap`/`finalize` flags) to `.pgrx_schema`.

### Graph builder changes

The `PgrxSql` graph builder is modified to match entities by name instead of `TypeId`:

**Type matching algorithm:**
1. For path-qualified tokens like `other_mod::MyType`, match on the last path segment
2. Search all registered type and enum entities for a name match
3. If exactly one match: create the dependency edge
4. If no match: create a `BuiltinType` node (type defined elsewhere — same as today)
5. If multiple matches: disambiguate using `module_path` (prefer same-module), then
   error if still ambiguous

For `PostgresOrd`/`PostgresHash`/aggregate connections, match by `name` and
`module_path` fields.

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
| `SqlTranslatable` trait | Extended with required `SCHEMA_KEY` const (no default — breaking change) |
| `ArgAbi` / `BoxRet` / `UnboxDatum` | Calling convention, not schema |
| `InOutFuncs` / `PgVarlenaInOutFuncs` | I/O format, not schema |
| `PgrxSql` graph structure | Same graph, same Tarjan SCC ordering |
| `ToSql` trait on entities | Same SQL emission logic |
| `PositioningRef` / `requires` system | Same explicit ordering mechanism |
| `SqlDeclaredEntity` / `creates` system | Same declaration mechanism |
| `extension_sql!()` / `extension_sql_file!()` semantics | Same, section-based now |
| `object` crate dependency in `cargo-pgrx` | Retained — reads section instead of symbol table |
| `parse_object()` fat binary helper | Retained |
| User-facing function/type APIs | Completely unchanged |

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

**One-line migration required.** `SCHEMA_KEY` has no default, so existing impls need
one additional line:

```rust
unsafe impl SqlTranslatable for HexInt {
    const SCHEMA_KEY: &'static str = pgrx::pgrx_resolved_type!(HexInt); // ADD THIS
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::As("hexint".into()))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::As("hexint".into())))
    }
}
```

Without this line, the code fails to compile with a clear error pointing at the missing
const. This is intentional — a missing `SCHEMA_KEY` would cause silent schema
generation bugs, so we make it a hard compile error.

### Manual `extension_sql!()` type creation

**Users who create types via raw SQL** (e.g., the `HexInt` pattern with
`extension_sql!("CREATE TYPE hexint ...", creates = [Type(HexInt)])`) — **no changes
needed.** The `extension_sql!()` macro embeds its entity data (including the `creates`
declarations) in `.pgrx_schema`. The graph builder uses the `SqlDeclaredEntity` system
(unchanged) to resolve `requires` references.

### Types from dependency crates

Types from dependency crates are not registered as entities in the extension's schema.
They appear as function argument/return types and are matched as built-in types (i.e.,
the graph builder creates a `BuiltinType` node for them). Same behavior as today.

### `#[bikeshed_postgres_type_manually_impl_from_into_datum]`

**No changes needed.** The derive macro still embeds the type entity data in
`.pgrx_schema`. The attribute only affects which datum-conversion traits are
generated — orthogonal to schema generation.

### `PostgresEq`, `PostgresOrd`, `PostgresHash` derives

**No changes needed** from the user's perspective. `PostgresOrd` and `PostgresHash`
embed their entity data in `.pgrx_schema` and the graph builder connects them to their
type entity by name and module path.

### `#[pg_aggregate]` implementations

**No changes needed.** The proc macro embeds aggregate entity data in `.pgrx_schema`.

### Custom `#[pg_extern]` argument types via `SqlTranslatable`

**Same one-line migration.** Users who implement `SqlTranslatable` for wrapper/newtype
types used in function signatures add `const SCHEMA_KEY: &'static str =
pgrx::pgrx_resolved_type!(Wrapper);` to their impl. The type continues to be matched
as a built-in in the graph.

## Type Matching Strategy: Detailed Rules

### 1. Built-in SQL type resolution (in the proc macro)

The `#[pg_extern]` proc macro maintains a table of known Rust-to-SQL type mappings:

| Rust token(s) | SQL type |
|-------------|----------|
| `i8` | `"char"` |
| `i16` | `smallint` |
| `i32` | `integer` |
| `i64` | `bigint` |
| `f32` | `real` |
| `f64` | `double precision` |
| `bool` | `bool` |
| `String`, `&str` | `text` |
| `&CStr`, `&std::ffi::CStr` | `cstring` |
| `()` | `void` |
| `pg_sys::Oid` | `oid` |
| `AnyNumeric` | `numeric` |
| `Date` | `date` |
| `Time` | `time` |
| `Timestamp` | `timestamp` |
| `TimestampWithTimeZone` | `timestamp with time zone` |
| `Interval` | `interval` |
| `Json` | `json` |
| `JsonB` | `jsonb` |
| `Uuid` | `uuid` |
| `Inet` | `inet` |
| `AnyElement` | `anyelement` |
| `AnyArray` | `anyarray` |
| `Internal` | `internal` |

Wrapper types (`Option<T>`, `Vec<T>`, `Array<T>`, `VariadicArray<T>`, `PgBox<T>`,
`default!(T, expr)`, `&T`, `&mut T`) are unwrapped to their inner type before
matching.

### 2. Custom type resolution (in the graph builder)

For argument types not resolved by the proc macro:
1. Extract the short name from the token: `other_mod::MyType` → `MyType`
2. Search registered type/enum entities for a name match
3. If no match, search `extension_sql!()` `creates` declarations
4. If still no match, create a `BuiltinType` node
5. If multiple matches, disambiguate by `module_path`, then error if ambiguous

## Known Limitations and Edge Cases

### Type aliases

If a user writes `type MyAlias = MyType;` and uses `MyAlias` in a `#[pg_extern]`
signature, the proc macro sees the token `MyAlias`, not `MyType`. There is no type
entity named `MyAlias`, so the graph builder falls through to creating a `BuiltinType`
node. The dependency edge from the function to `MyType`'s `CREATE TYPE` is lost.

This is a regression from `TypeId`-based matching, where
`TypeId::of::<MyAlias>() == TypeId::of::<MyType>()` handles aliases transparently.

**Mitigation:** The `#[pgrx(sql_type = "...")]` attribute on a function argument
provides an explicit override:

```rust
type MyAlias = MyType;

#[pg_extern]
fn foo(#[pgrx(sql_type = "my_type")] x: MyAlias) -> i32 { ... }
```

When `sql_type` is set, the proc macro writes it directly into the entity data and the
graph builder uses it for matching instead of the token name. This handles aliases,
re-exports with different names, and any other case where the token doesn't match the
registered type name.

If `type_name::<T>()` becomes `const fn` in the future, we can embed the
compiler-resolved name and handle aliases automatically without the attribute.

### Re-exported types

If `my_ext::types::Foo` is re-exported as `my_ext::Foo` and a function in `my_ext`
uses `Foo`, the proc macro sees the token `Foo`. The type entity's `module_path` is
`my_ext::types` (where the derive macro ran). The function's `module_path` is
`my_ext`. Name matching finds `Foo` by short name. The `module_path` disambiguation
does not interfere because there is only one registered type named `Foo` — it matches.
This case works correctly.

If there are *two* types named `Foo` in different modules and both are registered, the
disambiguation by `module_path` may pick the wrong one if the function uses a
re-export from a different module. This is the same ambiguity that exists in any
name-based resolution system. The graph builder errors on true ambiguity.

### Generic types

pgrx does not support generic type parameters in `#[pg_extern]` function signatures
(e.g., `fn foo<T: PostgresType>(x: T)` is not valid pgrx). Concrete generic
instantiations like `MyGenericType<i32>` are uncommon with `#[derive(PostgresType)]`
but possible.

If used, the proc macro sees `MyGenericType<i32>` as the type token. The short name
extraction gives `MyGenericType`. The type entity from the derive was registered as
`MyGenericType`. The name matches. The generic parameter is lost in the match, but
since the dependency edge only needs to order `CREATE TYPE my_generic_type` before the
function, this is correct — there is only one `CREATE TYPE` regardless of the generic
parameter.

### `#[cfg]`-gated entities

Entities behind `#[cfg(feature = "foo")]` are only compiled when the feature is active.
Since the const static in `.pgrx_schema` is part of the generated code, it is subject
to the same `#[cfg]` gating. This means the section only contains entities for the
active configuration — same behavior as today with `__pgrx_internals_*` symbols.

### `extension_sql_file!()` with large SQL content

`extension_sql_file!()` uses `include_str!()` to read SQL files at compile time. The
SQL content is embedded in the `.pgrx_schema` section as a JSON string value (with
proper JSON escaping by the proc macro). For extensions with large bootstrap SQL, this
increases section size proportionally. This is acceptable — even 100KB of SQL produces
a manageable section size, and the section can be stripped after schema generation.

The proc macro must JSON-escape the SQL content (handling `"`, `\`, newlines, etc.)
when constructing the entity data. Since the proc macro is an ordinary Rust program
with access to `serde_json`, this is straightforward.

### LTO and static deduplication

Link-Time Optimization may merge identical statics. If two entities produce
byte-identical JSON (unlikely in practice due to differing names and line numbers),
LTO could merge them, losing one entity. Adding `#[no_mangle]` with unique names
(which the proc macro already generates) prevents this.

### Multi-crate workspace extensions

When types are defined in a library crate and functions in the extension crate, both
produce `.pgrx_schema` data in their respective object files. The linker concatenates
sections from all linked object files (including from `.rlib` archives) into the final
`.so`. This is standard linker behavior for named sections — verified on both ELF and
Mach-O.

## Section Format

The `.pgrx_schema` section contains entity entries as newline-delimited JSON (NDJSON).
Each entry is a UTF-8 JSON object followed by `\n` (0x0A). Entries from different
compilation units are concatenated by the linker.

The section reader handles inter-entry padding robustly:
- Split section data on `\n`
- Skip any segment that does not start with `{` (catches null-byte padding, alignment
  bytes, or any other linker-inserted data between compilation units)
- Parse remaining segments as JSON objects

The statics use `[u8; N]` arrays with alignment 1 (the default for byte arrays), so
standard ELF and Mach-O linkers concatenate them without padding. The skip-non-JSON
logic is a safety net, not the primary mechanism.

Example section content (3 entities):
```
{"kind":"type","name":"MyType","module_path":"my_ext","file":"src/lib.rs","line":10,...}
{"kind":"function","name":"my_func","module_path":"my_ext","file":"src/lib.rs","line":20,...}
{"kind":"schema","name":"my_schema","module_path":"my_ext::my_schema",...}
```

## Future Work

### `type_name::<T>()` becoming `const fn`

If `core::any::type_name::<T>()` is stabilized as `const fn`
(tracking: [rust-lang/rust#63084](https://github.com/rust-lang/rust/issues/63084)),
we can embed fully-qualified type names in the section data using the same
`concat!()` technique, giving us exact type matching instead of token-name matching.

### Automatic alias detection

If `type_name::<T>()` becomes `const fn`, we could embed the compiler-resolved
fully-qualified type name alongside the token name in the section data. The graph
builder could then detect type aliases automatically without requiring
`#[pgrx(sql_type = "...")]` annotations.

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

2. **Add `SCHEMA_KEY` to every `unsafe impl SqlTranslatable` block.** The compiler
   will tell you exactly which impls need it. The fix is one line per impl:
   ```rust
   const SCHEMA_KEY: &'static str = pgrx::pgrx_resolved_type!(YourType);
   ```

3. Types using `#[derive(PostgresType)]`, `#[derive(PostgresEnum)]`, and all other
   derive macros need **no changes** — the derive generates `SCHEMA_KEY` automatically.

### For pgrx contributors

1. **`pgrx-sql-entity-graph`** — Each entity module's proc macro expansion changes
   from generating `__pgrx_internals_*` functions to emitting `#[link_section]` const
   statics. The entity structs lose their `TypeId` fields. The graph builder switches
   to name-based matching.

2. **`pgrx-macros`** — The `pg_extern`, `pg_operator`, `pg_cast` macro implementations
   gain the `concat!()` / `#[link_section]` entity embedding and compile-time
   verification logic.

3. **`cargo-pgrx`** — The `schema.rs` command module is substantially simplified:
   `first_build` remains; symbol scanning, codegen, second build, and binary execution
   are replaced with a single section read + JSON parse.

4. **`pgrx`** — The `pgrx_embed!()` macro and `WithTypeIds` infrastructure are removed.
   `SqlTranslatable` gains the required `SCHEMA_KEY` const (no default — breaking
   change). The `pgrx_resolved_type!()` and `__pgrx_schema_entity!()` helper macros
   are added.

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
#[cfg_attr(target_os = "macos", link_section = "__DATA,.pgrx_schema")]
#[cfg_attr(not(target_os = "macos"), link_section = ".pgrx_schema")]
```

- **Linux (ELF):** Section name `.pgrx_schema`
- **macOS (Mach-O):** Segment `__DATA`, section `.pgrx_schema`
- **Windows:** Not a primary target for pgrx; untested but `#[link_section]` works on
  PE/COFF in principle.

### Reading the section

```rust
let data = std::fs::read(&so_path)?;
let obj = parse_object(&data)?;  // existing helper, handles fat binaries

// section_by_name matches the section name field on both ELF and Mach-O
let section = obj.section_by_name(".pgrx_schema")
    .ok_or_else(|| eyre!("no .pgrx_schema section — is this a pgrx extension?"))?;
let raw = section.data()?;
```

On Mach-O, the `object` crate's `section_by_name()` matches against the 16-byte
section name field, so `".pgrx_schema"` matches a section declared as
`"__DATA,.pgrx_schema"`. This is the same `object` crate already used by
`cargo-pgrx` — its usage is just reduced from symbol-table scanning to a single
section lookup.
