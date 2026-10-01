# pgrx-c-macros

A library and command-line tool for discovering C macros in the headers used by
pgrx. The CLI lists PostgreSQL function-like macros; the library retains other
definitions separately as conversion context. Analysis and translation into Rust
`macro_rules!` currently cover a bounded C integer expression family.

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
The bindgen build pipeline generates these definitions alongside the bindings.
Each selected version gets `pgN_macros.rs` and `pgN_macro_report.json` in `OUT_DIR`.
The report records the C profile, input dependencies, emitted source and explicit
skip reasons. The active version's macros are exported from `pgrx-pg-sys`.

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
unavailable. Other unsupported inspected profiles retain per-macro skips. The current
docs.rs path uses shipped bindings and does not expose generated macros or input
adapters. Existing handwritten ports remain available while migration proceeds.

The first supported family covers integer expressions, casts, comparisons, shifts,
and lazy logical and conditional operators on checked signed-char LP64 targets.
Ordinary character literals support a bounded ASCII subset after compiler and
libclang probes verify the execution character set; wide, multi-character and
unmodeled encoding forms remain explicit skips.
Signed overflow follows the inspected undefined or wrapping policy. Pointer access,
mutation, statements, arbitrary calls, variadic arguments, stringification and
token pasting are skipped. Arithmetic outside C's defined domain is rejected by
the safe Rust support. Referenced typedefs, constants and macro definitions must
match the inspected environment; caller-local C shadowing is outside this family.

Results retain C type identity in `CValue<K>`. This keeps `long` and `long long`
distinct through composition even though both use `i64` storage. `.get()` extracts
the Rust representation. Raw `bool`, 8-, 16-, 32-, and 128-bit Rust integers have
established C input identities. Raw `i64`, `u64`, `isize` and `usize` require an
explicit tag, such as `CValue::<CUnsignedLong>::new(value)`, because their C identity
is otherwise ambiguous. The input traits are sealed; user-defined arithmetic
cannot substitute for C operations. Generated macros currently support runtime
expressions and report `RuntimeOnly` rather than const compatibility.

Generated adapters accept `pgrx-pg-sys::Oid` and `TransactionId` only after their
original C typedef identities and 32-bit storage are checked. Pointer-backed `Datum`
requires separate provenance support. C comparisons return a tagged C `int`, so
extract the value and compare it to zero when a Rust `bool` is needed:

```rust,ignore
use pgrx_pg_sys as pg_sys;
use pg_sys::__pgrx_c_macros::{CUnsignedLong, CValue};

let aligned = pg_sys::TYPEALIGN!(8_i32, CValue::<CUnsignedLong>::new(13_u64)).get();
let normal = pg_sys::TransactionIdIsNormal!(pg_sys::TransactionId::from_inner(3)).get() != 0;
```

`PGSIXBIT` and `MAKE_SQLSTATE` now emit at runtime, but existing enum discriminants
still need const-capable lowering before they can migrate.

Library callers inspect once with `PostgresConfig::inspect` or `inspect`, then
prepare an `AnalysisSession` for a batch of names. The session couples immutable
type/profile facts to compiler expansion. Input content, environment, working
directory and include resolution are checked during preparation; changed inputs
require another inspection. `session.analyze(name)` and `emit(&session, name)`
return owned reports. No macro signature list is required.

The tests compile original C and emitted Rust separately, comparing integer type
identities, values and evaluation counts. Downstream tests also cover `$crate`
hygiene and rejection of unsupported inputs. Handwritten ports identify test targets;
all expected types, values and evaluation counts come from C. Installation-dependent
comparisons and the isolated successive-build test run explicitly with:

```text
cargo test -p pgrx-c-macros --test postgres_emission -- --ignored
cargo test -p pgrx-c-macros --test postgres_handports -- --ignored
cargo test -p pgrx-c-macros --test character_literals -- --include-ignored
cargo test -p pgrx-bindgen --test macro_build -- --ignored
```
