# pgrx-c-macros

A library and command-line tool for discovering C macros in the headers used by
pgrx. This first version inventories definitions; translation into Rust
`macro_rules!` and support for C arithmetic semantics will follow separately.

## Command-line use

The tool requires libclang, as pgrx's bindgen generator does. Set
`LIBCLANG_PATH` if the shared library is outside its usual installation paths.

Select a configured PostgreSQL major version as the first positional argument
after `list`, following cargo-pgrx's CLI convention. Both `18` and `pg18` are accepted.
The tool uses cargo-pgrx's `pgrx-pg-config` lookup, including
`$PGRX_HOME/config.toml` and the `PGRX_PG_CONFIG_PATH` override. The selected
installation supplies its server include directory and preprocessor flags.

List macros from that installation's `postgres.h` and its included files:

```sh
cargo run -p pgrx-c-macros -- list pg18
```

Supply a wrapper header to discover a broader set of PostgreSQL headers.
Additional include directories, preprocessor definitions, and target options
follow `--`. Clang processes the input as C by default:

```sh
cargo run -p pgrx-c-macros -- list pg18 path/to/header.h -- -DFEATURE=1
```

The `list` command supports these output formats:

- `--format definitions` (the default) prints normalized C `#define` directives.
- `--format names` prints one macro name per line.
- `--format json` preserves macro kinds, token spellings and kinds, source
  locations, and Clang diagnostics in a structured inventory.

Use `--function-like` or `--object-like` to select a macro kind,
`--name TYPEALIGN` to select an exact name, and `--main-file-only` to exclude
included files. Repeat `--name` to select several names. Compiler-provided
definitions are excluded by default; `--include-builtins` includes them.

To inventory the headers in pgrx's binding generator's wrapper:

```sh
cargo run -p pgrx-c-macros -- list pg18 pgrx-pg-sys/include/pg18.h \
  --function-like --format json
```

## Discovery contract

The inventory contains definitions encountered by Clang for the supplied
configuration, including definitions from included files. Inactive conditional
branches are excluded. A definition remains in the inventory if a later
directive redefines or undefines it: this is a record of definitions, rather
than the final preprocessor environment.

The library preserves preprocessing order. The CLI sorts by name and retains
the relative order of repeated definitions with the same name. Text definitions
normalize whitespace and are intended for inspection; JSON retains individual
tokens for later analysis. Neither output expands replacement tokens or infers
their types.

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
let inventory = postgres.scan(&scanner, None, &[])?;
for definition in &inventory.macros {
    println!("{definition}");
}
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
