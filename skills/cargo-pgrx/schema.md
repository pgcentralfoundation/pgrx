# cargo pgrx schema

Builds the extension and reads its embedded SQL entity metadata to generate SQL.
It does not start PostgreSQL or install the extension. Normal install, run,
package, and backend-test workflows already generate the required schema.

## Usage and options

```text
cargo pgrx schema [OPTIONS] [PG_VERSION] [ITEM]...
```

If the first positional value is a supported `pg13` through `pg19` label, it
selects the PostgreSQL version. Otherwise all positional values are item names,
and version selection uses explicit Cargo features or the manifest's default.
With no items, the command emits the complete extension schema.

| Flag | Meaning |
|------|---------|
| `-o, --out <PATH>` | SQL output file; default is stdout |
| `-d, --dot <PATH>` | GraphViz dependency graph |
| `--skip-build` | Read existing artifacts for the chosen target and profile |
| `--no-alter-extension` | Omit extension membership statements in item mode |
| `--test` | Build with test support |
| `-c, --pg-config <PATH>` | Accepted by the parser but currently unused by schema execution; select a configured version instead |
| `-r, --release` | Release profile; default is development |
| `--profile <P>` | Custom profile; overrides `--release` |
| `-F, --features <F>` | Additional Cargo features |
| `--no-default-features`, `--all-features` | Cargo feature selection |
| `-p, --package <PKG>` | Select a workspace extension |
| `--manifest-path <PATH>` | Select its manifest |
| `--target <TARGET>` | Cargo compilation target |
| `--cargo <FLAG>` | Repeatable Cargo flags, including for metadata |

## Selected SQL items

Item mode emits the selected functions, types, enums, operators, aggregates,
triggers, schemas, or `extension_sql` blocks with their transitive dependencies
in installation order. Names containing `::` match Rust paths to disambiguate
items. Unqualified names must identify the intended item unambiguously.

Item output substitutes `'MODULE_PATHNAME'` with `'$libdir/<lib_name>'` and, by
default, adds `ALTER EXTENSION ... ADD ...` statements to attach emitted objects
to an existing extension. `--no-alter-extension` omits those membership statements.
Inspect generated SQL before executing it against an authorized database.

```bash
cargo pgrx schema pg18
cargo pgrx schema pg18 -o extension.sql
cargo pgrx schema pg18 --dot dependencies.dot
cargo pgrx schema pg18 my_function
cargo pgrx schema pg18 my_extension::types::MyType --no-alter-extension
cargo pgrx schema pg18 --skip-build --cargo=--config=./cargo-local.toml
```

`--skip-build` still needs a compatible artifact in Cargo's resolved target
directory. If code, features, PostgreSQL version, or build profile changed,
rebuild before trusting its SQL.
