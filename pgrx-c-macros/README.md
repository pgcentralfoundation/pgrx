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

Unlike the `list` filter, `analyze` and `emit` fail when a `--name` selects no
PostgreSQL-owned function-like macro or inline function.

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

An inline adapter evaluates each argument once and preserves C conversions,
pointer qualifiers and the actual return type. When the analyzer supports every
statement of the definition's body, and every macro the body reaches has at the end
of the headers the definition it had in the body, the adapter translates that body;
otherwise it calls the original function through its verified prototype. A macro
defined again after the function keeps the body native, unless a header entered
again repeats the one definition, as Clang's `<stddef.h>` repeats `NULL` for each
glibc header that asks for it. When the
preprocessor enters a header more than once, as for a template included twice,
only functions its first entry defines are translated. A translation evaluates
every argument before its
body runs, and the arguments' temporaries live until it finishes, as they do for a
call. Generated calls to a translated function expand its translation. `.get()`
extracts native storage, including `()` for a void return. When a macro becomes
an inline function, its return rank or evaluation rules can therefore change with
PostgreSQL, and so can whether a call needs `unsafe`. Whether a body is translated
also depends on the libclang that inspects it: some libclang releases locate a
statement that starts with a macro at the macro's definition, which keeps the body
native. It depends on the target's C library and architecture too, where a body
reaches their macros or is itself conditional: glibc defines `INT64_MIN` with a
token paste, which keeps `pg_abs_s64` native on Linux, and macOS defines `memcpy`
as a macro over a compiler builtin, which keeps `pq_writeint32` native there. An
assert-enabled build and a release build translate the same bodies; see
[Assertions and build flavors](#assertions-and-build-flavors). Predicate
callers can use `.is_true()` for either C `int` or C `bool`; this explicitly
converts truth without changing `.get()` or the original C identity. Native calls
retain generated FFI guards and require the original function's unsafe caller
contract.

`emit` writes Rust source to stdout and skip reasons to stderr. JSON output contains
the compilation profile, analysis, source, and structured skips. An inline function
whose translation could not be rendered, for example because bindgen and Clang
disagree about a constant it uses, records that reason as `fallback`. Emitted source
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
in both library and CLI output. When available, rustfmt also formats the
documentation snapshot; build output keeps the generator's layout.
The report records the C profile, input dependencies, emitted source and explicit
skip reasons, along with independently resolved constants and their Rust binding
paths. The active version's macros are exported from `pgrx-pg-sys` and reexported
at the `pgrx` crate root, so callers can use `pgrx::TYPEALIGN!(...)`.
Setting `PGRX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE=1` also writes one documentation
snapshot to `pgrx-pg-sys/src/include/cmacros/snapshot/`, alongside the shipped bindings
and OIDs. It covers every configured supported version. Each macro becomes a typed
shell: it keeps its documentation and invocation forms, and its arms have the real
macro's result type without its body. The snapshot also ships each version's generated
Rust support, without C source or native archives, so crates compiled against it under
docs.rs type-check `.get()`, `.into_value()`, markers and metadata getters as in ordinary
builds. Shells panic if run. Shells and support items shared by several versions are
written once behind `cfg(any(feature = ...))`. Only `docsrs` builds use the snapshot;
normal builds regenerate macros for their own C profile.
Release generation must run on the single docs.rs target in pgrx-pg-sys's
`[package.metadata.docs.rs] targets`, and it verifies and formats the merged snapshot
and bindings before writing anything to `src/include`. Each `pgNN.rs` records a
generation id that the snapshot asserts, so commit the bindings and the snapshot from
the same run, with every supported version configured;
`PGRX_PG_SYS_RELEASE_REQUIRE_ALL_VERSIONS=1` makes a run that misses one fail.
`tools/docsrs-typecheck.sh` type-checks the docs builds of pgrx-pg-sys and pgrx
against the committed tree for each version and checks the packaged crate size.

Ordinary bindgen retains its established target, CPPFLAGS and server include
directories. Its system include directories come from the Clang driver that
macro inspection selects: `PostgresConfig::system_includes` asks that driver for
its `#include <...>` search list under the profile's arguments without include
directories, so a recorded `-isysroot` or `--sysroot` selects the list. Bindgen
and inspection both receive the result as system directories (`-isystem` and
`-iframework`), searched after every `-I` directory as the driver searches them.
Macro inspection additionally observes recorded CFLAGS before CPPFLAGS, with
bindgen's environment overrides at the end, and reconciles those C facts with
the fresh bindings' actual storage. The native support archive, including the C
shim when the `cshim` feature is enabled, is compiled with that inspected profile.
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
options fail rather than disappearing. Inspection or native-generation failures
stop the binding build; unsupported individual macros retain structured skips.
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

The docs.rs path uses shipped bindings, typed macro shells and generated adapter
types without requiring PostgreSQL or Clang. The shipped support keeps its target
guard, so docs builds work only for the target release generation inspected.
Native interfaces can be type-checked, but calling their C symbols still requires
the matching native archive. Documentation snapshots and handwritten ports are not
semantic fallbacks for ordinary builds.

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
compiler facts regardless of the switch. Binding, inspection, configuration
and native-generation errors fail the build without publishing guessed code or
linkage, and a failed generation writes no new report. This switch
controls the binding build's diagnostics; explicit CLI `analyze` and `emit`
commands continue to print their requested output and skip reasons.

### Translation behavior

The runtime models C integer promotions, conversions, casts, comparisons, shifts,
and lazy logical and conditional operators on compiler-verified LP64, LLP64, and ILP32 targets.
Emitted literals keep their radix and digit spelling. C suffixes, octal notation
and character escapes are adjusted to Rust syntax without changing the inferred
C type or value. Microsoft sized suffixes such as `i32` and `ui64`, which MSVC's
`<stdint.h>` uses, are translated when the compiler accepts them, as it does for
MSVC targets. They take the type that both the compiler and libclang report, and
a value that the compiler would truncate to the suffix's width is refused.

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
A shared dependency graph records conservative dependencies among all final active
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
tokens without evaluating them. A type operand may be stringified too, and its
hole spells the Rust type. Non-ASCII token spelling, returned strings,
unknown array extents, address-taking, pointer arithmetic, and concatenation of
such holes are refused. A root's own `#` stringification is admitted on the same
terms, as `Assert`'s condition text is. A translated inline body may reach a macro
that stringifies only the condition it passes to `ExceptionalCondition`; that text
can differ from C's in spacing, because the body's tokens are replayed with spaces
between them. `__FILE__` and `__LINE__` use the Rust invocation's `file!()` and
`line!()`, in macros and translated inline bodies alike, rather than freezing the
generator's or the header's source location. Stringification of those location
markers and other dynamic builtins remain unsupported. Literal writes acquire
no mutation capability; wide, UTF-prefixed and unproved encodings are skipped.
The typed expression support also covers:

- Qualified pointers, array decay, indexing, pointer arithmetic and casts, with
  C pointee identity retained across compositions.
- Record and union fields, packed fields, compiler-described bitfields, volatile
  accesses, assignment, compound assignment, and prefix and postfix mutation.
- Unevaluated `sizeof` and alignment expressions, comma expressions, type and
  field-name parameters, unused arguments, and empty replacements. An unused
  argument must be a Rust expression, and is checked in dead code like any operand
  the expansion does not evaluate.
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
  A C library `offsetof` spelled as `((size_t)&(((s*)0)->m))`, as in Microsoft's
  C runtime headers without `_CRT_USE_BUILTIN_OFFSETOF`, is refused. So is every
  other evaluated member access, subscript or dereference through a null pointer
  constant, which designates no object; only `sizeof` operands and `&*` remain
  allowed.
- Direct `__builtin_expect` calls with a compiler-verified `long(long, long)`
  prototype and result identity. Both operands are evaluated and converted as
  in C, and the generated expression returns the first unchanged. Fixed expected
  values use stable Rust's `core::hint::cold_path()` to preserve branch direction;
  enclosing expectations take precedence over nested hints through groups or casts.
  Dynamic expectations remain unhinted, with a generated comment explaining why.
- `ExceptionalCondition`, `abort()`, `__builtin_unreachable()` and MSVC's
  `__assume`, which stop the process; see
  [Assertions and build flavors](#assertions-and-build-flavors).
- Closed token pastes resolved by Clang, including literal suffixes and fixed
  type, field, function, enum and helper names. Separate compiler probes reject
  pastes that depend on caller operands and discover synthesized dependencies
  before rescanning can hide them. Empty-side pastes that preserve an operand
  unchanged are supported. The validation currently recognizes upstream Clang
  versions 6 through 23 and Apple clang versions 11 through 21, and fails closed
  on unfamiliar diagnostics. Of the Apple releases, only Apple clang 21's
  builtins were enumerated. With any other Clang version, every macro that
  reaches a paste is skipped.
- A fixed identifier pasted onto the name of a type operand, where the result
  names an enumerator, as `T_##_type_` names a node tag in PostgreSQL's `IsA`,
  `makeNode` and `castNode`. The macro takes that operand as a Rust type; see
  [Type operands that name an enumerator](#type-operands-that-name-an-enumerator).
  The same compiler probes show that the macro's only operand pastes have this
  form and that each pasted name survives in the expansion.
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
  declaration forms remain skips. An integer, `_Bool`, floating or pointer local
  that is declared without an initializer and whose address is taken counts as
  initialized: C leaves it indeterminate until it is written, as a callee writes
  an out-parameter, and the translation declares it holding zero.
- GNU statement expressions, `({ statements; value; })`, wherever an expression
  may stand: in a macro, in a statement block and in a translated inline body.
  The statements are those of a statement block, in a scope of their own, and
  the last one must be the expression that gives the value. The value is never
  a place. A statement expression without a final expression, which C gives the
  type `void`, and one that contains a `return` are skipped. They nest at most
  8 deep.

Signed overflow follows the inspected undefined or wrapping policy. Arithmetic
outside C's defined domain is rejected by the safe scalar support. Pointer and
storage operations require the caller's `unsafe` context; the generated macro does
not discharge allocation bounds, initialization, aliasing, provenance or backend
thread requirements. PostgreSQL calls retain pgrx's FFI error and panic guard.

Reading a global is the exception when the binding catalog names a thread check, as
pgrx-pg-sys's does. A macro reads an integer, `_Bool`, enum or data pointer global
after pgrx's active-thread check, so the read needs no `unsafe`. That check is the
one pgrx relies on for every call into PostgreSQL, which reads and writes its
globals on the thread that passes it; it panics on another thread. A global that
PostgreSQL also sets asynchronously is `volatile` in C and is read as `volatile`.
An object of one of those types loads a valid Rust value from anything C stores in
it through that type. Enum globals load their compatible integer, never the Rust
enum. Writes, dereferences, and reads of floating, aggregate or function pointer
globals still need the caller's `unsafe`. The check runs in release builds too. A
macro that reads several such globals runs it once, before the first read it
evaluates, and not at all when it evaluates none. A macro that does so is a block,
so the temporaries of its operands end with it; do not keep a pointer into one.
Native adapters convert arguments and check nullable function pointers before
entering that guard, and decode results after leaving it.

Other statements and initialization constructs, variadic arguments,
stringification outside the diagnostic contract, any other operand-dependent token
pasting, unsupported compiler constructs and unmodeled literals remain explicit skips.
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

### Assertions and build flavors

A program that uses the generated macros compiles against a PostgreSQL configured
with `--enable-cassert` and against one configured without, when both are the same
version. A macro has the same name, operands, result and need for `unsafe` against
both, and the same Rust source apart from layout facts. No macro leaves out a check
that the assert-enabled headers make. `USE_ASSERT_CHECKING` otherwise changes which
names the headers declare, how `Assert` and its relatives are defined, and what some
static inline functions contain.

Inspection therefore parses the headers a second time, in memory, as the other build
flavor: with `USE_ASSERT_CHECKING` defined on the command line, or with its
definitions blanked. Both flavors then hold the same two views and decide the same
things from them:

- A function-like macro that the flavors define differently, such as `Assert`,
  `AssertMacro` and `pg_unreachable`, is translated from its assert-enabled
  definition in both. Macros and static inline functions that use it follow. The
  generated documentation shows that definition and its location.
- Whether an assertion runs is a constant of the installation,
  `__pgrx_c_assert_checking!()`, which the generated support defines. Against a
  release build an `if` statement that only reports a failed assertion, and the
  `holds || (report, 0)` of `AssertMacro`, are compiled and never evaluated, as in
  release C. Their operands are still type-checked and still need `unsafe` where
  they read through a pointer, so `PageValidateSpecialPointer` needs `unsafe`
  against both builds.
- A function that only an assert-enabled build declares, where a release build
  defines a macro that does nothing, becomes a macro that calls the function:
  `AssertTransactionIdInAllowableRange`, `AssertPendingSyncs_RelationCache` and
  `SlruPagePrecedesUnitTests`. Against a release build the call is not made and its
  arguments are not evaluated, so the release macro must do nothing: it may hold
  only grouping, a cast to `void`, integer constants and an empty `do`/`while`. A
  release macro that evaluates an operand or does any other work is generated by
  neither flavor. The binding generator leaves the function out, and
  the macro reaches it through a generated adapter, which does nothing in a release
  build. So does a function that one flavor defines `static inline` and the other
  only declares, such as `AssertCouldGetRelation`, which both builds bind.
- Any other external function that only an assert-enabled build declares, and that
  returns nothing, can be called from a merged inline body, described next, by the
  statements that only an assert-enabled build has:
  `AssertBufferLocksPermitCatalogRead` and `MemoryContextCheck`, for example.
  Against a release build the call is not made. A macro whose assert-enabled
  definition calls one is still generated by neither flavor: its release definition
  is other text, and nothing proves that it does the same work without the call. A
  function that returns a value has nothing a release build could return, so code
  that calls one is not shared either.
- A static inline function whose definition differs between the flavors, such as
  `TupleDescCompactAttr`, is translated from one body that both flavors build from
  the two definitions. The statements that only one flavor has run when
  `__pgrx_c_assert_checking!()` says the installation is that flavor, and the other
  flavor still compiles them. The function is called, not translated, in both when
  the difference is anything but whole statements of a block: tokens inside an
  expression, braces that do not pair, or the branch of an `if`. So it is when
  those statements use a name the flavors do not share, or declare a local that
  later statements use.
- A name that only one flavor declares is generated and bound by neither:
  `castNodeImpl`, `MemoryContextCheck` and `USE_ASSERT_CHECKING` itself, for example.
  So is a function whose prototype differs. `FrontendOutput::flavor_specific_names`
  lists them for the binding generator.
- A static inline function that only an assert-enabled build defines, such as
  `castNodeImpl`, is still called by the macros that use it, through a generated
  adapter. A release build compiles the function's own definition into its native
  support, where its assertions do nothing. So `castNode`, the list macros built
  on it such as `lfirst_node`, and `intVal` and its relatives check the node's tag
  against an assert-enabled build and are the same macros against a release
  build.
- A macro whose assert-enabled definition needs any other name that a release build
  lacks is generated by neither flavor, and neither is a macro that expands it.
  Their skip code is `build_flavor`. Emitting the release definition instead would
  leave out a check.

Calls that stop the process lower to safe Rust in pgrx-pg-sys's support, without a
native call. `ExceptionalCondition`, which `Assert` and its relatives call, writes
PostgreSQL's `TRAP:` line and a backtrace to stderr and aborts. Its condition text
is the C text, or the Rust invocation's tokens for a macro operand, and its file and
line are those of the Rust code that used the macro. `abort()` aborts. Reaching
`__builtin_unreachable()`, or a false MSVC `__assume`, is undefined in C, so it
writes a `TRAP:` line and aborts. A failed check never unwinds, so it cannot become
a recoverable PostgreSQL `ERROR`. Only a report through `ExceptionalCondition`
depends on the installation; any other stop, and the condition that guards it, runs
in every build.

A macro that calls an assertion function needs the caller's `unsafe`, as any native
call does, unless the function is one that generated code may call from safe code.
Those are listed by name in `emit/assertions.rs`, each with the reason: they take no
pointer and need only the backend state that pgrx's safe wrappers already assume,
and the guarded call checks pgrx's active thread. `SlruPagePrecedesUnitTests` takes a
pointer and needs `unsafe` against both builds.

A macro operand that the expansion never names as a value, because the argument is
unused or appears only in a stop's text, is checked in dead code, so an operand that
needs `unsafe` needs it although no build's C evaluates it.

Differences that are not names stay: a structure member or enumerator that only one
flavor has, such as `JumbleState.total_jumble_len`, and the value of an object-like
macro. The emission report records `caller_unsafe` for each emitted macro, and
pgrx-pg-sys's macro report records these decisions under `build_flavors`, so that
builds can be compared. Its `merged` list names the inline functions whose two
definitions became one body, and `native_only` those that both builds call.

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

Macros introducing C locals, in a statement block or a statement expression,
reject arguments mentioning those local names:
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

A macro that applies `.` to a parameter, such as `XidFromFullTransactionId`,
reads that member through the argument's place, so it never copies the rest of
the record. The argument must therefore be a Rust place expression. C also
accepts a function result or another macro's record value there, but those
arguments fail to compile here; bind the record to a local first.

A Rust array argument is evaluated as an array value and does not decay to a
pointer to the caller's array. A macro that subscripts its parameter, such as
`lengthof`, therefore does not compile with an array argument. Pass a pointer
where C would decay the array. `sizeof` then measures that pointer, as it would
in C, so `lengthof!` has no array form; use the array's `len()` instead.

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

### Type operands that name an enumerator

A macro that pastes a fixed prefix onto a type operand takes a Rust type there.
PostgreSQL's node macros paste `T_` to name the type's node tag:

```rust,ignore
use pgrx_pg_sys as pg_sys;

// SAFETY: a backend allocates the node, and `node` points to it afterwards.
unsafe {
    let query: *mut pg_sys::Query = pg_sys::makeNode!(pg_sys::Query).get();
    let node = query.cast::<pg_sys::Node>();
    assert!(pg_sys::IsA!(node, pg_sys::Query).is_true());
    let same: *mut pg_sys::Query = pg_sys::castNode!(pg_sys::Query, node).get();
}
```

The generated support holds one sealed trait for each such prefix, and the macro
reads the enumerator as that trait's constant for the operand's type. A type
implements the trait when all of these hold:

- Its C name is a typedef of a structure or union, and the prefix followed by
  that name is an enumerator. All such enumerators of one prefix must belong to
  one enum and have one C type.
- No macro of either name is active, which C would expand instead, and both build
  flavors declare both names.
- The bindings hold the structure and the enumerator, with the value Clang reports.
- No other C name of the same structure has an enumerator of its own, even one
  that a macro hides or the bindings lack. PostgreSQL names one structure `OpExpr`, `DistinctExpr` and `NullIfExpr`, with a node tag
  for each. Rust sees one type, so none of the three names selects a tag; compare
  `nodeTag!` with the enumerator instead.

Any other type fails to compile with an error that names the prefix and the enum,
as the pasted name fails to resolve in C. Any Rust path or alias of a bound type
names that type. C also accepts a name that is no type, as in
`IsA(list, IntList)`; a Rust type operand cannot say that, so compare `nodeTag!`
with `T_IntList`.

The lookup itself is a constant. `IsA!` reads the node through the caller's
pointer, `castNode!` and the list macros call `castNodeImpl`, and `makeNode!` calls
`palloc0` through `newNode` and stores the tag in the result, so each needs
`unsafe` against both build flavors. PostgreSQL 15 and 16 define `newNode` as a
macro: a GNU statement expression for GCC and Clang, and for other compilers a
comma expression that keeps the node in the global `newNodeMacroHolder`, a write
that needs the caller's `unsafe` like any other. PostgreSQL 17 made `newNode` an
inline function.
Each form gives the same `makeNode!`. The size check of the two macro forms
stringifies the type operand, so a failed check names the Rust type.

A macro is still skipped when a paste puts the operand first or between two
pasted parts, when the expansion erases, stringifies or pastes the result again,
when the operand is also used as a value, or when the pasted name is used as
anything but an expression, such as a member name.

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
generated bindings. The catalog describes only those bindings: it cannot claim a
generated callback signature, function address getter, or field or offset
adapter. `emit_with_bindings` generates none of these, so it skips a macro that
needs one. Nor can the catalog mark an inline function as translated, so a call
from `emit_with_bindings` output never expands a translated body. It also skips
a call whose C prototype passes or returns plain `char`, an enum or a record by
value. The catalog's Rust storage for such a function cannot prove its call ABI,
and only a generated C adapter can make that call.
Output from `emit_with_bindings` can still name a C enum's generated identity,
which only the support from `generate_with_bindings` defines.
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
cargo test -p pgrx-c-macros --test postgres_emission --test postgres_handports --test character_literals --test oracle --test plain_char --test string_oracle
cargo test -p pgrx-bindgen --test macro_build -- --ignored
cargo test -p pgrx-bindgen --test shipped_macros -- --ignored
```

Which macros a build generates depends on its libclang, and a test cannot see a
macro that was skipped. On Linux x86_64 and aarch64, CI therefore runs this
crate's tests and builds `pgrx-pg-sys` for every supported PostgreSQL version
with each Clang major from 14 through 23. `pgrx-ci macro-compare` then fails
when, for one architecture and PostgreSQL version, a macro is emitted under one
Clang major and skipped under another, or is emitted with a different
`caller_unsafe`, invocation contract or fallback to the original C function.
The comparison's list of accepted differences is `EXPECTED` in
`tools/ci/src/macros.rs`.

The `macro_build` harness also cross-compiles for a target chosen from the
host: `x86_64-apple-darwin` on Apple silicon, `aarch64-apple-darwin` on x86_64
macOS, `aarch64-unknown-linux-gnu` on x86_64 Linux and
`x86_64-unknown-linux-gnu` on aarch64 Linux. Install that target's standard
library with `rustup target add`; the test fails and names the command when it
is missing.

Tests wrapped in `if_c_macro!` or the classifier's availability arm disappear
when their macro is not emitted. The `pgrx-pg-sys` `macro_availability` test
therefore requires each name gated in `pgrx-pg-sys/tests` and
`pgrx-unit-tests/src/tests` to be emitted, apart from the version differences it
lists. It skips a gate that the build compiles
out through a file-level `#![cfg]` or a single-line `#[cfg]` written directly
on the gate, so put a platform or version cfg there rather than inside the
gate. The guard evaluates only `target_os = "..."`, `feature = "pgNN"`, `docsrs`
and `all`, `any` or `not` of them: write `target_os` rather than `unix` or
`windows`, because any other predicate on a gate, or on a file that has gates,
fails the test. Its report must also record a completed generation.

## Generation in extension builds

`pgrx-pg-sys` cannot compile without its generated macros, so every build of the
active `pgNN` feature generates them. A missing matching Clang driver, server
headers configured for another machine, a refused profile or a failed native
support build stops the build. The error names the PostgreSQL version, the
Cargo target, the server headers, the source of the PostgreSQL metadata (a
`pg_config` path or `PGRX_PG_CONFIG_AS_ENV`) and the cause. Unsupported
individual macros remain structured skips in the report. `PGRX_MACRO_DEBUG=1`
prints the generation summary and each skip; normal builds stay quiet. docs.rs
builds skip generation and use the shipped snapshots, and a complete target
bundle imported through `PGRX_TARGET_INFO_PATH_PGNN` supplies its verified
generated files instead.

`PostgresConfig::inspect` and `inspect_with_arguments` check that the server
headers describe the inspected target, so the binding build and the CLI refuse
the same headers. When the headers define `PG_VERSION_NUM`, the machine named in
`PG_VERSION_STR` must match Clang's architecture and OS, and its C library and
ARM float ABI when it names them, as autoconf's GNU and musl triples do and
meson's `<cpu>-<system>`, MinGW and MSVC strings do not. meson's `netbsd` and `linux` also
match DragonFly and Android, which meson renames to them. For an installation's
own macOS headers, the `-arch` values in its recorded CFLAGS replace the
recorded architecture, so a universal build such as Postgres.app describes each
slice. An unrecognized machine is accepted only when the target has the
generator's own architecture and OS. One libclang parse then compares the `pg_config.h` sizes,
member-offset alignments, `MAXIMUM_ALIGNOF`, byte order, 64-bit integer flags
and major version with the target compiler, and reports every disagreement. A
refusal is `PostgresError::TargetConfiguration`, which wraps a
`ConfigurationError`. The binding build adds a remedy naming
`PGRX_INCLUDEDIR_SERVER_PGNN_<target>` in both target spellings,
`PGRX_PG_CONFIG_AS_ENV` and `PGRX_TARGET_INFO_PATH_PGNN`.

The binding build takes the server header root from `PGRX_INCLUDEDIR_SERVER`
and its version- and target-specific variants when one is set, and otherwise
from `pg_config --includedir-server`. That root selects bindgen's allowlist and,
through `PostgresConfig::with_server_include_dir`, owns the generated macros.

A `pg_config` program describes the machine it runs on. When Cargo's `HOST` and
`TARGET` differ, the binding build therefore applies only PostgreSQL's
functional CFLAGS from it: `-fno-strict-aliasing`, `-fwrapv` and
`-fexcess-precision=standard`, in recorded order. The report lists the other
recorded flags under `cflags_dropped`, and the `PGRX_MACRO_DEBUG=1` summary
prints them. Native builds and `PGRX_PG_CONFIG_AS_ENV` metadata apply every
recorded flag, as does the CLI, except Darwin `-arch` selections: Clang cannot
preprocess for two slices at once, and the target selects the slice, so they
are listed under `cflags_dropped` too.

The runtime checks the selected compiler's LP64, LLP64, or ILP32 representation,
including plain-char signedness, C long width, and the distinct ranks of `size_t`
and pointer differences. This includes unsigned-char Linux AArch64 and Windows
LLP64; both byte orders are admitted when the C and Rust target agree. See the
[cross-compilation guide](../docs/src/extension/build/cross-compile.md#postgresql-headers-and-metadata-for-the-target)
for supplying target headers and metadata.

Some installations do not record historical CFLAGS, and `PGRX_PG_CONFIG_AS_ENV`
metadata may omit `PGRX_PG_CONFIG_CFLAGS`. In that case, the selected headers,
target, and current compiler invocation define the macro profile. The build
report records `cflags_recorded: false`; it does not infer the server's
unrecorded build options, including `-fwrapv`. The binding build splits
recorded flags with the Cargo target's quoting rules and translates reviewed
MSVC options to equivalent Clang options for MSVC targets.
`PostgresConfig::inspect`, and so the CLI, use the host platform's rules.
