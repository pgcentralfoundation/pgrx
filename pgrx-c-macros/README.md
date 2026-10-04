# pgrx-c-macros

A library and command-line tool for discovering C macros in the headers used by
pgrx. The CLI lists PostgreSQL function-like macros; the library retains other
definitions separately as conversion context. Analysis and translation into Rust
`macro_rules!` cover supported C expressions, with generated type and storage
adapters derived from Clang declarations and the corresponding Rust bindings.
The binding build also selects PostgreSQL-owned object expressions as
zero-argument macros and supplements missing integer constants with independent
compiler proofs. These paths preserve the original C definition and type.

See [ARCHITECTURE.md](ARCHITECTURE.md) for the end-to-end implementation, phase
contracts, C semantics, generated native adapters, and validation strategy.

## Command-line use

The tool requires libclang, as pgrx's bindgen generator does. Set
`LIBCLANG_PATH` if the shared library is outside its usual installation paths.
On supported Linux/macOS hosts, building the assembly-backed SHA-256 dependency
also requires a host C compiler; other hosts use the portable Rust backend.

Select a configured PostgreSQL major version as the first positional argument
after `list`, following cargo-pgrx's CLI convention. Both `18` and `pg18` are accepted.
The tool uses cargo-pgrx's `pgrx-pg-config` lookup, including
`$PGRX_HOME/config.toml` and the `PGRX_PG_CONFIG_PATH` override. The selected
installation supplies its server include directory and preprocessor flags.

List function-like macros across the headers pgrx uses for that version:

```sh
cargo run -p pgrx-c-macros -- list pg18
```

The crate embeds the matching pgrx wrapper for each supported version and
materializes it in a content-addressed directory under `$PGRX_HOME/c-macros/headers/`.
It contains the same header list as pgrx's binding generator. Installed CLI tools
work without a source checkout. A caller can also supply its own wrapper header.

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
unselected object macros remain context. CLI discovery still lists function-like
definitions. Library callers can select object expressions with
`AnalysisSession::prepare_objects` or mix both root kinds with
`prepare_with_objects`. The binding build selects owned nonempty objects whose
dependency closure refers to a declared variable, exposing them as `NAME!()`;
pure integer objects belong in the constant bindings instead.

Compiler-proven PostgreSQL static inline functions also expose `NAME!(arguments)`.
The binding build and CLI analysis/emission select every eligible definition
physically owned by the server headers. Library generators select these separately
with `postgres_inline_function_names` and pass them to
`AnalysisSession::prepare_with_inline_functions` beside their macro and object
lists. Active C macros win any name collision; no function adapter is inserted into
the actual macro inventory or preprocessing environment, and `list` still shows
only original macro definitions.

An inline adapter calls the original function through its verified prototype,
evaluates each argument once, and preserves C conversions, pointer qualifiers and
the actual return type. `.get()` extracts native storage, including `()` for a void
return. When a macro becomes an inline function, its return rank or evaluation
rules can therefore change with PostgreSQL. Predicate callers can use `.is_true()`
for either C `int` or C `bool`; this explicitly converts truth without changing
`.get()` or the original C identity. Native calls retain generated FFI guards and
require the original function's unsafe caller contract.

`emit` writes Rust source to stdout and skip reasons to stderr. JSON output contains
the compilation profile, analysis, source, and structured skips. Emitted source
uses `$crate::__pgrx_c_macros`, provided by `pgrx-pg-sys`, and includes a target guard.
Each generated macro's `///` doc comments include its source location and full unexpanded
C macro definition in a fenced `text` block. As in `list` output, whitespace is normalized
and line comments are omitted. Inline call adapters instead document the actual
physical function definition in a fenced `c` block, retaining its original source
and formals; their documentation never presents an invented `#define`.
The bindgen build pipeline generates these definitions alongside the bindings.
Each selected version gets a `cmacros/pgN/` module tree and
`pgN_macro_report.json` in `OUT_DIR`, plus native support source and an archive
for the active version. Bindgen emits native static-inline wrappers with either
setting of `cshim`; the archive also supplies original immutable PostgreSQL
metadata and any required macro primitives. Header paths determine the Rust files:
`utils/acl.h`, for example, becomes `cmacros/pgN/utils/acl.rs`. Shared Rust
support is split into fragments under `__pgrx_c_support/`, included in the same
module scope to preserve private adapter access. Macro definitions receive
multiline layout before rustfmt, including repetitions and internal token syntax
that rustfmt leaves untouched. This preserves literal spellings and source comments
in both library and CLI output. When available, rustfmt formats
every generated Rust file, including the documentation snapshot.
The report records the C profile, input dependencies, emitted source and explicit
skip reasons, along with independently resolved constants and their Rust binding
paths. The active version's macros are exported from `pgrx-pg-sys` and reexported
at the `pgrx` crate root, so callers can use `pgrx::TYPEALIGN!(...)`.
Setting `PGRX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE=1` also writes one documentation
snapshot to `pgrx-pg-sys/src/include/cmacros/snapshot/`, alongside the shipped bindings
and OIDs. It covers every configured supported version: each macro keeps its
documentation and invocation forms, without its implementation or native support, and
definitions shared by several versions are written once behind `cfg(any(feature = ...))`.
Only `docsrs` builds use it; normal builds regenerate macros for their own C profile.
Generate the snapshot alongside matching PostgreSQL bindings on the selected release
target, since the emitted set and its documentation describe that target.

Ordinary bindgen and the fallback C shim retain their established target,
CPPFLAGS and include settings. Optional macro inspection additionally observes
recorded CFLAGS before CPPFLAGS, with bindgen's environment overrides at the
end, and reconciles those C facts with the fresh bindings' actual storage.
Successful macros require unchanged compiler/header inputs around binding
and native generation. Cargo tracks consumed headers, search directories,
configuration, compiler/library files and relevant environment values. Compiler
lookup tracks only attempted executable paths and PATH when it was actually
searched. Earlier absent candidates require safe parent-directory watches so
creating a preferred compiler invalidates selection; absolute successful
selection does not add PATH dependencies. Same-content symlink retargets also
invalidate the recorded input identity.
Custom compiler or `pg_config` wrappers must declare hidden inputs in their build
script so changes to those inputs can trigger generation.

The runtime admits compiler-verified LP64, LLP64, and ILP32 profiles with either
byte order, subject to a recognized Rust architecture, OS, and ABI guard. This
model does not establish backend test coverage for every admitted target.
Plain char follows the inspected compiler's signedness and promotion,
independently of Rust's native `core::ffi::c_char` alias. A verified one-byte
storage bridge preserves its bytes. Raw `i8` and `u8` inputs retain the separate
signed-char and unsigned-char identities, so a plain-char operand requires an
explicit `CValue<CChar>` or `Pointer<CChar, _>` tag.
Recorded MSVC flags use Windows quoting and explicit Clang translations; unknown
options fail rather than disappearing. Optional inspection or native-generation
failures record unavailable macros; unsupported individual macros retain
structured skips.
The decoder retains runtime choices until `lower_msvc_runtime_flags` sees the
combined flags: the last `/MD[d]` or `/MT[d]` choice determines the runtime
predefines and native COFF library directives. User define/undefine overrides
remain effective, including on older Clang versions without `-fms-runtime-lib`.
Build integration refuses runtime selectors in `BINDGEN_EXTRA_CLANG_ARGS`, whose
unchanged tail bindgen appends internally; recorded flags carry those selections.

Precomputed imports require a complete target bundle containing raw bindings,
macro/support source, the compiler report, native wrapper source, and the native
archive. The importer checks member hashes and lengths, target, major, generator
version, and `cshim` before linkage. Raw-only imports cannot supply the migrated
callers. Format 2 includes the inline invocation interface; older bundles require
fresh generation into a new directory. See the [cross-compilation guide](../docs/src/extension/build/cross-compile.md)
for `PGRX_PG_SYS_EXTRA_TARGET_INFO_PATH` export and
`PGRX_TARGET_INFO_PATH_PGnn` import. Bundle integrity does not authenticate its
producer; use trusted artifacts from the same pgrx release and revision.

The docs.rs path uses shipped bindings, macro definitions and generated adapter
types without requiring PostgreSQL or Clang. Only installation-specific target
error guards are disabled there. Native interfaces can be type-checked, but
calling their C symbols still requires the matching native archive. Documentation
snapshots and handwritten ports are not semantic fallbacks for ordinary builds.

### Build diagnostics

Macro-generation Cargo warnings are quiet by default. Set `PGRX_MACRO_DEBUG=1`
to print the generation summary and a warning for
each skipped macro, including bindgen/Clang value disagreements and their
dependent skips:

```sh
PGRX_MACRO_DEBUG=1 cargo build
```

Unset the variable or set `PGRX_MACRO_DEBUG=0` for normal quiet builds; only the
exact value `1` enables these warnings. Cargo tracks the variable, so changing it
reruns the binding build. Every completed generation writes
`pgN_macro_report.json` in `OUT_DIR`, retaining skip reasons and available
compiler facts regardless of the switch. Ordinary binding errors still fail
the build; optional inspection or native-generation failures record unavailable
macros without publishing guessed code or linkage. This switch
controls the binding build's diagnostics; explicit CLI `analyze` and `emit`
commands continue to print their requested output and skip reasons.

### Translation behavior

The runtime models C integer promotions, conversions, casts, comparisons, shifts,
and lazy logical and conditional operators on compiler-verified LP64, LLP64, and ILP32 targets.
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
and types remain unchanged. A disagreement skips the entire macro and records
the constant, both values, and the skipped macro in the report; with
`PGRX_MACRO_DEBUG=1`, it also emits a build warning.
A shared petgraph graph records conservative dependencies among all final active
macros, including object macros and external context. Reverse graph traversal
also skips dependent macros and records their skipped dependencies, with build
warnings enabled by `PGRX_MACRO_DEBUG=1`. Historical definitions and shadowed
formal parameter names do not create active dependency edges. A second preprocessing pass proves that preserving
an object name reproduces the original expansion, and requires atomic or fully
parenthesized object expressions. Otherwise expansion remains, with a
`/* PGRX: ... */` explanation. Literal fallbacks retain their original spelling.
Uncertain constant arithmetic also stays expanded so folding cannot bypass the
support's checks for undefined operations, even when Clang can evaluate it.

For integer object macros omitted by bindgen, `probe_integer_object_constants`
requires valid constant initialization, a concrete C integer identity and value,
and independent Clang driver witnesses. The binding build adds only missing
constants, including closed integer-suffix pastes that the compiler proves. It
never overwrites an existing disagreement. Accepted facts and refusals appear in
the report; a malformed candidate does not establish a neighboring candidate's
value. This constant path is separate from readable expression-name retention.

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
Ordinary closed strings support the verified ASCII byte subset, escapes, embedded
zeros, adjacent literal concatenation, array extent, decay and read-only access.
Clang can resolve closed stringification in a dependency to such a literal.
Operand-dependent dependency stringification is narrower: it must feed a direct
`const char *` argument of a fixed, nonvariadic, void-returning native diagnostic.
The C text template is retained, while holes stringify the outer Rust invocation
tokens without evaluating them. Non-ASCII token spelling, returned strings,
unknown array extents, address-taking, pointer arithmetic, and concatenation of
such holes are refused. Root `#` stringification remains unsupported.
`__FILE__` and `__LINE__` use the Rust invocation's `file!()` and `line!()`, rather
than freezing the generator's source location. Stringification of those location
markers and other dynamic builtins remain unsupported. Literal writes acquire
no mutation capability; wide, UTF-prefixed and unproved encodings are skipped.
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
- `offsetof` with named records, type parameters, and dotted member paths,
  including member-path parameters. Offset metadata checks Clang's layouts
  against the actual bindings without accessing memory or requiring a loadable
  field value. Bitfields, array indices, and indirect member paths are rejected.
- Direct `__builtin_expect` calls with a compiler-verified `long(long, long)`
  prototype and result identity. Both operands are evaluated and converted as
  in C, and the generated expression returns the first unchanged. Fixed expected
  values use stable Rust's `core::hint::cold_path()` to preserve branch direction;
  enclosing expectations take precedence over nested hints through groups or casts.
  Dynamic expectations remain unhinted, with a generated comment explaining why.
- Closed token pastes resolved by Clang, including literal suffixes and fixed
  type, field, function, enum and helper names. Separate compiler probes reject
  pastes that depend on caller operands and discover synthesized dependencies
  before rescanning can hide them. Empty-side pastes that preserve an operand
  unchanged are supported. The validation currently recognizes upstream Clang
  versions 6 through 21 and fails closed on unfamiliar diagnostics.
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

Other statements and initialization constructs, variadic arguments,
stringification outside the diagnostic contract, operand-dependent token pasting,
unsupported compiler constructs and unmodeled literals remain explicit skips.
Parser and type limitations are reported as skips. Referenced
declarations and the final macro environment must match the inspection.
An application such as `(T)(x)` is ambiguous when `T` is a macro parameter:
C callers can supply either a type or a function pointer. Translation requires
independent type evidence, such as `sizeof(T *)`, or an unambiguous expression
callee, such as `((callback))(x)`. Otherwise analysis reports a type-parameter
skip. Bare GNU `_Alignof(T)` does not resolve this ambiguity because Clang also
accepts expression operands there.
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
Generated `__pgrx_c_types` aliases name each inspected integer typedef's C marker.
Use these for portable `int64`, `Timestamp`, `Size`, or `size_t` operands rather
than assuming that 64-bit storage means `long`. On LLP64, `long` uses 32-bit
storage and `long long` retains 64-bit storage and its higher rank.

Generated adapters accept `pgrx-pg-sys::Oid` and `TransactionId` after checking
their original C typedef identities and storage. The verified `Datum` adapter
preserves exposed pointer provenance when converting its pointer-backed Rust
storage to and from the C pointer-width integer representation. C comparisons
return a tagged C `int`, so extract the value and compare it to zero when a Rust
`bool` is needed:

```rust,ignore
use pgrx_pg_sys as pg_sys;
use pg_sys::cmacros::c::CValue;

let aligned = pg_sys::TYPEALIGN!(8_i32, CValue::<pg_sys::__pgrx_c_types::size_t>::new(13 as _)).get();
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
`MAKE_SQLSTATE` emit at runtime. Compiler-proved object constants, such as missing
`ERRCODE_*` bindings, supply Rust const items through the separate constant path;
they do not make arbitrary macro invocations const-capable.

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

Support selection follows the successfully emitted macros. Compiler-established
record owners restrict field and offset adapters to that record; caller-selected
offset paths retain its reachable by-value records and anonymous members.
Caller-selected fields retain type families constrained by the C expression's
operations and member paths. A finite worklist propagates value categories through
operators, and indexed declaration queries establish possible field and callback
types. Known function prototypes and assignment destinations also constrain
nominal records and pointer compatibility, including qualifiers and nested types.
Declared member, element, and call results restrict downstream owners even when
the original caller operand is generic.
Unknown declaration facts retain conservative support. This selection does not
depend on sampled invocations or a signature list.
Callback identity bridges and callable implementations are selected separately.
Nominal callback identities share null and address operations through physical
families generated only for the selected ABI and arity pairs, without an arity
limit. Each identity retains its exact binding storage and distinct C prototype;
equal Rust function-pointer types do not merge C integer ranks or calling rules.
Caller-selected native function pointers receive an input bridge only when the
complete binding catalog proves one validated C identity for their Rust storage.
Unselected, rejected and missing C witnesses participate in that proof. Ambiguous
storage retains the tagged `FunctionValue` interface.
Selected, validated typedefs also appear under `__pgrx_c_callbacks`, following
their actual binding module paths. For example,
`__pgrx_c_callbacks::MyCallback::new(pointer)` explicitly tags that typedef's
exact nullable native storage. This constructor adds no ABI cast or permission
to call the pointer. Different C prototypes remain different markers even when
their Rust function-pointer storage is equal; rejected prototypes receive no
constructor. Invocation still requires the original callback's unsafe contract.
An identity-only callback does not retain adapters for its prototype's nested
callbacks. Function address references likewise do not retain callable wrappers;
calls inside unevaluated operands still retain the code Rust needs to type-check.
Complete C/Rust ABI validation still examines every witness, including aliases
whose code is not emitted. Ordinary fields share a raw projection implementation;
each descriptor retains its exact storage, offset and qualification proofs,
including typed Rust checks for promoted fields and transparent storage wrappers.
Uncalled projection witnesses share a function-pointer type alias while retaining
each exact native field type, including wrappers and promoted intermediate
fields. Rust checks these bodies even when unused, while numeric layout checks
stay in eager top-level constants.
The shared field trait derives alignment and access metadata from those verified
storage types and qualifications, avoiding repeated constants in each adapter.
One qualifier table combines containing-place access with declared member
constness, so individual fields only record their declared access.
Complete native records share sealing, type and input conversions through a
crate-private registration marker. Actual Rust `Copy` bounds permit native record
values; non-`Copy` records still support raw pointers and field access. Incomplete
bindings retain their distinct opaque capability.
Generic scalar and pointer operands can require whole native type families;
those bridges remain available. An open native enum operand retains distinct
Rust enum storage; bindgen's integer aliases already use primitive bridges.
Named C enum casts, fields, variables, and prototypes retain their nominal enum
identities separately. Private numeric enum registration fixes each identity's
exact compatible integer kind; primitive storage conversions and enum/integer
pointer compatibility share bounded tables without widening that pairing.
Native Rust enum encoders still check every discriminant.
Equal storage layout assertions are emitted once after every enum has been
validated; conflicting compiler facts remain errors.
Unreferenced identities in the hidden generated support module can disappear
between builds. Rejected macros retain no exclusive adapter dependencies.

Set `BindingCatalog::ffi_boundary` to the defining crate's PostgreSQL FFI guard
when generating native adapters that can call PostgreSQL. The pgrx bindgen pipeline
supplies this path. Standalone callers must establish an equivalent boundary if
their C functions can perform PostgreSQL nonlocal error jumps or callbacks.
`emit_support_with_bindings` is only suitable when no native C artifact is needed;
it returns an error otherwise. `emit_batch_with_bindings` remains available for
callers managing support separately, and propagates value-mismatch failures
through the shared dependency graph. It returns an error when shared support
generation or verification fails.
`frontend.dependencies()` exposes that graph, its direct and reverse edges,
and affected callers for auditing.

The pgrx binding build handles declaration metadata through the original C macros,
outside expression translation. C initializes a static `PG_MODULE_MAGIC_DATA`
record and emits `PG_FUNCTION_INFO_V1`; generated getters return immutable data.
Recursive C/Rust storage, size, alignment and field-offset witnesses establish
record validity before Rust copies module magic or borrows function info. These
pure getters have no backend, callback or error effects. The exported Rust glue
and custom static name/version controls remain in pgrx, with module magic cached
once. No copied version arithmetic or function-info initializer supplies values.

## Validation

Ordinary `cargo test -p pgrx-c-macros` runs native fixtures using Clang and libclang.
The tests compile original C and emitted Rust separately and compare types,
values, mutations and evaluation counts. They cover partial aggregate initialization,
enum values, callback ABI and guards, packed and volatile storage, unevaluated
operands, nested macro contexts, `$crate` hygiene and invalid invocations. These
checks use the original C definitions as the oracle. The historically named
`postgres_handports` target keeps an explicit test-only corpus of original-header
primitives so deleting a Rust port does not delete its oracle coverage. This is
not a production signature list.

Installed-header comparisons also run as ordinary tests for every configured
version, or the exact `PG_VER` selection used by CI. An explicit selection without
an installation fails; without a selection, missing installations are reported
as omitted coverage. The isolated build/release/docs.rs tests remain explicit:

```text
cargo test -p pgrx-c-macros --test postgres_emission --test postgres_handports --test character_literals --test oracle
cargo test -p pgrx-bindgen --test macro_build -- --ignored
cargo test -p pgrx-bindgen --test shipped_macros -- --ignored
```

## Optional generation in extension builds

`pgrx-pg-sys` generates ordinary bindings independently of macro inspection.
A missing matching Clang driver, refused profile, or failed native support build
produces an unavailable macro report and an empty availability map, while
ordinary binding generation continues. Set `PGRX_C_MACROS=0` to disable this
optional work. `PGRX_MACRO_DEBUG=1` enables refusal warnings; normal builds stay
quiet. The CLI inspection and analysis commands still report their own errors.

The runtime checks the selected compiler's LP64, LLP64, or ILP32 representation,
including plain-char signedness, C long width, and the distinct ranks of `size_t`
and pointer differences. This includes unsigned-char Linux AArch64 and Windows
LLP64; both byte orders are admitted when the C and Rust target agree. Cross
compilation needs target PostgreSQL metadata through `PGRX_PG_CONFIG_AS_ENV`;
a host installation's CFLAGS are not target evidence.

Some installations do not record historical CFLAGS. In that case, the selected
headers, target, and current compiler invocation define the macro profile. The
build report records `cflags_recorded: false`; it does not infer the server's
unrecorded build options. Recorded flags use the host platform's quoting rules,
with reviewed MSVC options translated to equivalent Clang options on Windows.
