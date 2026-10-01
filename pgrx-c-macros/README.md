# pgrx-c-macros

A library and command-line tool for discovering C macros in the headers used by
pgrx. The CLI lists PostgreSQL function-like macros; the library retains other
definitions separately as context for future conversion. This version inventories
definitions; translation into Rust `macro_rules!` and support for C arithmetic
semantics will follow separately.

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
to avoid bringing in clap and the JSON writer:

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
