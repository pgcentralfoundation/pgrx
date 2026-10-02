# pgrx-c-macros

A library and command-line tool for discovering C macros in the headers used by
pgrx. The CLI lists PostgreSQL function-like macros; the library retains other
definitions separately as conversion context. Analysis and translation into Rust
`macro_rules!` cover supported C expressions, with generated type and storage
adapters derived from Clang declarations and the corresponding Rust bindings.

## Command-line use

The tool requires libclang, as pgrx's bindgen generator does. Set
`LIBCLANG_PATH` if the shared library is outside its usual installation paths.

Select a configured PostgreSQL major version as the first positional argument
after `list`, following cargo-pgrx's CLI convention. Both `18` and `pg18` are accepted.
The tool uses cargo-pgrx's `pgrx-pg-config` lookup, including
`$PGRX_HOME/config.toml` and the `PGRX_PG_CONFIG_PATH` override. The selected
installation supplies its server include directory and preprocessor flags.

List function-like macros across the headers pgrx uses for that version:

```sh
cargo run -p pgrx-c-macros -- list pg18
```

The default input is this checkout's `pgrx-pg-sys/include/pg18.h` for PG18,
or the matching wrapper for another selected version. This is the same header
list used by pgrx's binding generator. The default wrapper comes from the pgrx
source checkout used to build this tool; keep that checkout available when
running it. A caller can also supply its own wrapper header.

Only function-like definitions physically inside the selected installation's
`pg_config --includedir-server` tree are candidates. This is the installed copy
of PostgreSQL's server headers. System and third-party headers are excluded
from every `list` format. Resolving each file's path before checking this
boundary handles symlinks and `..` components; a similarly named sibling
directory is outside the tree. Extra include paths and custom wrappers do not
expand this boundary.

Supply a header or source file to override that wrapper. Additional include
directories, preprocessor definitions, and target options follow `--`.
Clang processes the input as C by default:

```sh
cargo run -p pgrx-c-macros -- list pg18 path/to/header.h -- -DFEATURE=1
```

The `list` command supports these output formats:

- `--format definitions` (the default) prints normalized C `#define` directives.
- `--format names` prints one macro name per line.
- `--format json` preserves macro kinds, token spellings and kinds, source
  locations, provenance, and Clang diagnostics in a structured inventory.

Use `--name TYPEALIGN` to select an exact name and `--main-file-only` to exclude
included files. Repeat `--name` to select several names. Compiler-provided and
command-line definitions are conversion context and are never listed.

For a JSON inventory of the default wrapper:

```sh
cargo run -p pgrx-c-macros -- list pg18 --format json
```

## Discovery contract

`PostgresConfig::scan` returns a `PostgresInventory`. Its `inventory.macros`
contains only PostgreSQL function-like definitions. Its `context` contains
all other definitions: PostgreSQL object-like macros, external header macros,
wrapper definitions outside the server tree, and compiler-provided and
command-line macros. Context is available to library callers, but is omitted
from CLI output, including JSON. `MacroScanner::scan` is the lower-level raw
scan and retains all definitions without PostgreSQL ownership filtering.

Clang processes the whole include graph using the supplied configuration,
including external definitions needed for conditional compilation. Inactive
conditional branches are excluded. A definition remains recorded if a later
directive redefines or undefines it: this is a record of definitions, rather
than the final preprocessor environment.

The library preserves preprocessing order separately in the inventory and
context collections. The CLI sorts by name and retains the relative order of
repeated definitions with the same name. Text definitions
normalize whitespace and are intended for inspection; JSON retains individual
tokens for later analysis. Neither output expands replacement tokens or infers
their types.

Every definition with a physical source file has `provenance`: an absolute
filename and inclusive, one-based `start_line` and `end_line`. These span the
macro's name through its last replacement token, including physical line
continuations and interior comments. Clang excludes trailing comments and
empty trailing continuations from this token span. `#line` directives do not
change its physical filename or lines. Each redefinition has its own span.
The existing `location` still points precisely at the macro name, with its
column and byte offset. Compiler-provided and command-line definitions have
no physical provenance and serialize it as `null`.
Clang's `-working-directory` option is rejected; use the process working
directory so relative Clang filenames can be resolved for provenance.

Clang warnings are reported on standard error. Errors, including missing
includes, fail the command instead of producing a partial inventory.

## Library use

Disable the default `cli` feature when using this crate as a build dependency
to avoid bringing in clap. JSON remains a library dependency for Clang's VFS overlay:

```toml
[build-dependencies]
pgrx-c-macros = { path = "../pgrx-c-macros", default-features = false }
```

```rust,no_run
use pgrx_c_macros::{MacroScanner, PostgresConfig};

let postgres = PostgresConfig::resolve("pg18")?;
let scanner = MacroScanner::new()?;
let discovery = postgres.scan(&scanner, None, &[])?;
for definition in &discovery.inventory.macros {
    println!("{definition}");
}
// discovery.context retains definitions that may be needed during conversion.
# Ok::<(), pgrx_c_macros::PostgresError>(())
```

Binding generators can use `PostgresConfig::from_pg_config` with an already
selected `PgConfig`, and pass their target and other Clang options to `scan`.
`MacroScanner::scan` also supports direct scanning with caller-provided options.

Macro records own their strings and locations and remain valid after the
scanner is dropped. The scanner retains Clang's restriction of one active
instance per process and stays on the thread that created it. Discovery reuses
an existing Clang runtime, including bindgen's, and keeps it loaded on the
thread after the scanner is dropped.

## Analysis and Rust emission

The selected installation's original headers and recorded compiler flags determine
the C semantics. Handwritten pgrx macro ports are not a reference. A compatible
Clang executable is required alongside libclang. Use `--clang` to select one
explicitly, or set `CLANG_PATH`. Explicit selections take precedence over the
installation's configured compiler; an incompatible recorded preference falls back
to a compiler matching libclang. Place additional compiler flags after `--` to
override the installation's flags. The version remains the first positional argument:

```sh
cargo run -p pgrx-c-macros -- analyze pg18 --format json
cargo run -p pgrx-c-macros -- emit pg18 --name TYPEALIGN
cargo run -p pgrx-c-macros -- emit pg18 --format json
```

`analyze` and `emit` use the final active macro map, declaration catalog and target
facts. They expand nested macros with Clang while retaining the original main-file
preprocessing context. PostgreSQL owns the primary macros; external headers and
object macros remain context.

`emit` writes Rust source to stdout and skip reasons to stderr. JSON output contains
the compilation profile, analysis, source, and structured skips. Emitted source
uses `$crate::__pgrx_c_macros`, provided by `pgrx-pg-sys`, and includes a target guard.
Each generated macro's documentation includes its source location and full unexpanded
C definition in a fenced `text` block. As in `list` output, whitespace is normalized
and line comments are omitted.
The bindgen build pipeline generates these definitions alongside the bindings.
Each selected version gets `pgN_macros.rs` and `pgN_macro_report.json` in `OUT_DIR`,
plus native support source and an archive when the generated adapters require them.
The report records the C profile, input dependencies, emitted source and explicit
skip reasons, along with independently resolved constants and their Rust binding
paths. The active version's macros are exported from `pgrx-pg-sys`.
Setting `PGRX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE=1` also writes `pgN_macros.rs`
to `pgrx-pg-sys/src/include`, alongside the shipped bindings and OIDs. These
documentation snapshots include the generated adapters. Their target guards
remain active outside `docsrs`; normal builds regenerate macros for their own C profile.
Generate release snapshots on Linux with the matching PostgreSQL bindings. The
snapshot values and layouts describe that release target, not the build host of
a later user.

Binding and macro inspection share the complete flags, target, include directories
and compiler resource directory. Recorded CFLAGS precede CPPFLAGS, and bindgen's
environment overrides apply once at the end. Native C shims also receive recorded
CFLAGS before CPPFLAGS. Cargo tracks consumed headers, header
search directories, configuration, compiler/library files and relevant environment
values. Creating optional or shadowing headers also triggers regeneration. Compiler
lookup tracks rejected candidates and searched PATH roots. Set an absolute `CLANG_PATH`
when those roots include Cargo's generated output; recursive tracking would cause
repeated generation, while omitting them could miss a compiler selection change.
Custom compiler or `pg_config` wrappers must declare hidden inputs in their build
script so changes to those inputs can trigger generation.

Windows/MSVC and precomputed target-binding imports currently report macro generation
unavailable. Other unsupported inspected profiles retain per-macro skips. The docs.rs
path uses shipped bindings, macro definitions and input adapters without requiring
PostgreSQL or Clang. Native adapter implementations are excluded from documentation
builds because they belong to the inspected installation's platform and layouts.
Existing handwritten ports remain available while migration proceeds.

The runtime models C integer promotions, conversions, casts, comparisons, shifts,
and lazy logical and conditional operators on checked signed-char LP64 targets.
Emitted literals keep their radix and digit spelling. C suffixes, octal notation
and character escapes are adjusted to Rust syntax without changing the inferred
C type or value.

Generated matchers and operands retain the original C formal parameter names.
Verified casts name the corresponding Rust binding, such as `AclMode`, while
the accompanying C type marker preserves integer rank, signedness and pointer
qualifications. Unavailable or incompatible binding aliases retain the canonical
C cast with a `/* PGRX: ... */` explanation.

Binding-aware emission preserves verified names such as `$crate::MaxAllocHugeSize`
and enum constants in their generated modules. The Rust binding supplies the
referenced value; Clang supplies its original C integer type and independently
verifies its value. Bindgen's values
and types remain unchanged. A disagreement skips the entire macro and produces
a build warning naming the constant, both values and the skipped macro.
A shared petgraph graph records conservative dependencies among all final active
macros, including object macros and external context. Reverse graph traversal
also skips dependent macros and explains their skipped dependencies in build
warnings. Historical definitions and shadowed formal parameter names do not
create active dependency edges. A second preprocessing pass proves that preserving
an object name reproduces the original expansion, and requires atomic or fully
parenthesized object expressions. Otherwise expansion remains, with a
`/* PGRX: ... */` explanation. Literal fallbacks retain their original spelling.
Uncertain constant arithmetic also stays expanded so folding cannot bypass the
support's checks for undefined operations, even when Clang can evaluate it.

Direct wrappers such as `BUFFERALIGN` call `$crate::TYPEALIGN!` when that callee
is emitted. Matching the two compiler-expanded expression trees recovers the
argument expressions while preserving grouping, C type identity, repeated
evaluation and lazy branches. Calls inside larger expressions, unsupported
callees and unrecoverable arguments still expand with explanatory comments.
The CLI preserves calls between its emitted macros; without a Rust binding
catalog, named constants use documented fallbacks.
Ordinary character literals support a bounded ASCII subset after compiler and
libclang probes verify the execution character set; wide, multi-character and
unmodeled encoding forms remain explicit skips.
The typed expression support also covers:

- Qualified pointers, array decay, indexing, pointer arithmetic and casts, with
  C pointee identity retained across compositions.
- Record and union fields, packed fields, compiler-described bitfields, volatile
  accesses, assignment, compound assignment, and prefix and postfix mutation.
- Unevaluated `sizeof` and alignment expressions, comma expressions, type and
  field-name parameters, unused arguments, and empty replacements.
- Calls through complete C prototypes, including static inline functions and
  nullable function pointers, and addresses of original C functions.
- Compiler byte-swap intrinsics whose direct-call prototypes and LLVM value
  flow match the selected profile. Input conversion and result rank follow C;
  the lowered operation uses Rust's byte swap. Intrinsic addresses and callbacks
  remain unsupported.
- C enums whose compatible integer representation is established by Clang,
  including values without a named Rust enum variant.
- `float` and `double` where the compiler profile establishes the modeled IEEE
  representations and evaluation modes. Floating arithmetic additionally requires
  contraction disabled; excess precision, fast-math modes and `long double`
  remain outside the modeled family.
- Statement blocks, including conditional and terminal `return`, with
  nested lexical blocks and literal-zero `do`/`while` wrappers, ordered expression
  statements, `if`/`else`, and flat typed local declarations.
  Local reads require initialization on every path that can reach them;
  unsupported control flow and
  declaration forms remain skips.

Signed overflow follows the inspected undefined or wrapping policy. Arithmetic
outside C's defined domain is rejected by the safe scalar support. Pointer and
storage operations require the caller's `unsafe` context; the generated macro does
not discharge allocation bounds, initialization, aliasing, provenance or backend
thread requirements. PostgreSQL calls retain pgrx's FFI error and panic guard.
Native adapters convert arguments and check nullable function pointers before
entering that guard, and decode results after leaving it.

Other statements and initialization constructs, variadic arguments, stringification,
token pasting, unsupported compiler constructs and unmodeled literals remain
explicit skips. Parser and type limitations are reported as skips. Referenced
declarations and the final macro environment must match the inspection.
Compatible function pointer types
with distinct generated signature markers are conservatively rejected in
operations that would require a proof relating those signatures.

## Calling generated macros

Public expression results use `CExpression<V>`. Its `.into_value()` retains the
semantic value, such as `CValue<K>` or a typed pointer, and `.get()` extracts the
native Rust storage. Integer markers keep `long` and `long long` distinct even
when both use `i64` storage. The wrapper is an already evaluated value, not a
deferred load or an ABI type. Empty replacements expand to an empty Rust expression.

Return macros perform the return in the enclosing Rust function and apply C
assignment conversion to that function's result type. Call them directly:

```rust,ignore
fn return_int32(value: i32) -> pgrx_pg_sys::Datum {
    pgrx_pg_sys::PG_RETURN_INT32!(value);
}
```

Like a Rust `return`, this exits the nearest function or closure and runs
the caller's normal Rust cleanup.

`Datum` has a verified C identity. Ambiguous native result types require an
explicit marker, such as `RETURN!(@__pgrx_c_return_as [CUnsignedLong]; value)`
for an original function returning C `unsigned long`. Return macros cannot
serve as expression operands. Forms that capture caller context take explicit
extra arguments: `PG_RETURN_NULL!(fcinfo)` and
`SRF_RETURN_NEXT!(context, result, fcinfo)`. Their pointer operations and backend
calls still require the caller's established safety contract.

Statement macros without a return execute their full expressions in order,
preserve nested local scopes, and yield `()`. They discard each expression's
result without dropping its evaluation. They cannot be used as C expression
operands.

Conditional statements evaluate their C scalar condition once and execute only
the selected branch. A return in a branch exits the caller; other branches can
continue through the rest of the macro. Unbraced conditional replacements require
`MACRO!(@__pgrx_c_statement; arguments...)`, which corresponds to the braced C
invocation `{ MACRO(arguments...); }`. This boundary prevents a caller's `else`
from changing the macro's control flow. If a return marker is needed, put
`@__pgrx_c_return_as [CMarker];` after the statement boundary.

Macros introducing C locals reject arguments mentioning those local names:
C substitution can capture a local that Rust macro hygiene would resolve
differently. A compile-time check inspects stringified arguments, including
forwarded expression fragments. Matches in fields, paths or strings are also
conservatively rejected. The check admits at most 4096 stringified argument
bytes and 64 local declarations.

Raw `bool`, 8-, 16-, 32-, and 128-bit Rust integers have established C input
identities. Raw `i64`, `u64`, `isize` and `usize` require an explicit tag, such as
`CValue::<CUnsignedLong>::new(value)`, because their C identity is ambiguous.
Raw pointers likewise require an established pointee identity; equal Rust widths
do not establish equal C types. Supply explicit tags or literal suffixes when
Rust cannot infer the intended type. Some generic conditional expressions still
require a suffix even where the corresponding C literal defaults to `int`.
The sealed input traits accept only the modeled C value families; Rust operator
implementations do not change the C arithmetic rules.

Generated adapters accept `pgrx-pg-sys::Oid` and `TransactionId` after checking
their original C typedef identities and storage. The verified `Datum` adapter
preserves exposed pointer provenance when converting its pointer-backed Rust
storage to and from the C pointer-width integer representation. C comparisons
return a tagged C `int`, so extract the value and compare it to zero when a Rust
`bool` is needed:

```rust,ignore
use pgrx_pg_sys as pg_sys;
use pg_sys::__pgrx_c_macros::{CUnsignedLong, CValue};

let aligned = pg_sys::TYPEALIGN!(8_i32, CValue::<CUnsignedLong>::new(13_u64)).get();
let normal = pg_sys::TransactionIdIsNormal!(pg_sys::TransactionId::from_inner(3)).get() != 0;
```

Nested generated macro operands carry the C context requested by each use:
value, place, size or discard. This preserves repeated evaluation, lazy branches,
array identity in `sizeof`, and mutations through an inner macro's place. The
normalizer recognizes generated macro invocations in a bounded token grammar;
it does not parse arbitrary Rust expressions as C. To pass an unrelated Rust
macro invocation as a native value, wrap that argument as
`(@__pgrx_c_native [some_rust_macro!(argument)])`.

An ungrouped C parameter accepts only an atomic token tree or an explicitly
parenthesized argument. If the original replacement is itself unparenthesized,
the generated documentation requires
`MACRO!(@__pgrx_c_expression; arguments...)`. This requests the semantics of the
parenthesized C invocation, rather than attempting to reproduce interaction with
operators outside the invocation. Caller-scope identifiers that are absent from
the declaration catalog become additional explicit arguments, listed in each
macro's documentation after its original parameters.

Original C aggregate results travel as `RawRecordValue<R>` backed by
`MaybeUninit<R>`. Reading one initialized field does not materialize neighboring
uninitialized fields or invalid Rust bool/enum values. `.get()` returns
`MaybeUninit<R>`; `unsafe assume_initialized()` requires proof that the entire
record satisfies Rust's validity and ownership rules. C enum expression values
retain nominal enum identity but extract their compatible integer storage, so
unnamed C values never require construction of an invalid Rust enum.

Generation and helpers currently support runtime expressions and report
`RuntimeOnly`, rather than promising const evaluation. `PGSIXBIT` and
`MAKE_SQLSTATE` emit at runtime; existing enum discriminants still need
const-capable lowering before they can migrate.

## Binding generator integration

Library callers inspect once with `PostgresConfig::inspect` or `inspect`, then
prepare an `AnalysisSession` for a batch of names. The session couples immutable
type/profile facts to compiler expansion. Input content, environment, working
directory and include resolution are checked during preparation; changed inputs
require another inspection. `session.analyze(name)` and `emit(&session, name)`
return owned reports. No macro signature list is required.
`session.integer_constants()` exposes the probed C object constants.
`emit_with_bindings` accepts a `BindingCatalog` of actual Rust constant paths,
values, storage representations and declarations for functions, records, enums,
variables and aliases. It checks every referenced constant value against C before
emitting a reference; callers must supply the catalog from the defining crate's
generated bindings.
Generators should use `generate_with_bindings` to prepare the macro set and its
shared adapters together. Its `MacroGeneration` contains the per-macro emissions
and a `MacroSupportArtifact` with Rust source and optional C source. Include the
Rust support once in the defining crate. Write the C source to disk, compile and
archive it with `compile_native_support` under the inspected profile, and link
the archive into that crate. The C source must include the inspected header.
These access and ABI primitives support field operations and original functions;
they do not substitute whole C macro bodies for Rust translation.

Set `BindingCatalog::ffi_boundary` to the defining crate's PostgreSQL FFI guard
when generating native adapters that can call PostgreSQL. The pgrx bindgen pipeline
supplies this path. Standalone callers must establish an equivalent boundary if
their C functions can perform PostgreSQL nonlocal error jumps or callbacks.
`emit_support_with_bindings` is only suitable when no native C artifact is needed;
it returns an error otherwise. `emit_batch_with_bindings` remains available for
callers managing support separately, and propagates value-mismatch failures
through the shared dependency graph.
`frontend.dependencies()` exposes that graph, its direct and reverse edges,
and affected callers for auditing.

## Validation

Ordinary `cargo test -p pgrx-c-macros` runs native fixtures using Clang and libclang.
The tests compile original C and emitted Rust separately and compare types,
values, mutations and evaluation counts. They cover partial aggregate initialization,
enum values, callback ABI and guards, packed and volatile storage, unevaluated
operands, nested macro contexts, `$crate` hygiene and invalid invocations. These
checks use the original C definitions as the oracle. Handwritten pgrx ports only
identify additional test targets.

Installation-dependent comparisons and isolated build/release/docs.rs tests run
explicitly with:

```text
cargo test -p pgrx-c-macros --test postgres_emission -- --ignored
cargo test -p pgrx-c-macros --test postgres_handports -- --ignored
cargo test -p pgrx-c-macros --test character_literals -- --include-ignored
cargo test -p pgrx-bindgen --test macro_build -- --ignored
cargo test -p pgrx-bindgen --test shipped_macros -- --ignored
```
