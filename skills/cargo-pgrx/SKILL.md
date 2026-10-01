---
name: cargo-pgrx
description: "Choose cargo-pgrx commands for extension development, tests, SQL regressions, benchmarks, packaging, and isolated worktree environments."
user-invocable: false
---

# cargo pgrx

Use this skill to choose commands and understand their effects on an extension,
its build artifacts, and PostgreSQL. Follow the user's execution limits and the
repository's instructions. A request to edit documentation does not require
running the commands it describes.

Use a cargo-pgrx version compatible with the extension's pgrx dependencies. When
working on pgrx itself, use the CLI from the same checkout; see the repository's
[development instructions](../../README.md#hacking). Read the relevant command
reference before running it. The command implementations in
[`cargo-pgrx/src/command`](../../cargo-pgrx/src/command) and the
[`pgrx-tests` harness](../../pgrx-tests/src/framework.rs) define current behavior.

## Rust tests and backend tests

The extension's shared library runs inside PostgreSQL. A Rust test executable
runs outside the backend and cannot safely call PostgreSQL functions or access
its runtime state. Such calls can fail to link or crash; successfully linking
does not make them valid.

Use `#[test]` for pure Rust operations, including operations on bindings that
do not call the backend. Constants, enums, and simple value types such as
`pg_sys::Oid` can be used in ordinary unit tests. Check the entire call path,
including destructors, before classifying a test as independent of PostgreSQL.

Use `#[pg_test]` for SPI, PostgreSQL allocation, memory contexts, relations,
backend error reporting, and other code requiring a running backend. Follow the
extension's existing `tests` schema and feature setup. When dependencies are
unclear, inspect them before choosing a test tier.

Both `cargo pgrx test` and correctly configured `cargo test` can drive the
pgrx-tests harness. The CLI selects a PostgreSQL version and adds `pg_test`;
plain Cargo requires the caller to select compatible features. For example,
with PG18 configured:

```bash
cargo pgrx test pg18
cargo test --no-default-features --features "pg18 pg_test"
```

## Command routing

| Intent | Command |
|--------|---------|
| Check compilation | `cargo check --no-default-features --features pg18` |
| Run Rust and backend tests | `cargo pgrx test pg18` |
| Filter tests by one or more substrings | `cargo pgrx test pg18 spi memory` |
| Build, install, and open psql | `cargo pgrx run pg18` |
| Install files without starting PostgreSQL or opening psql | `cargo pgrx install --pg-config /path/to/pg_config` |
| Run SQL regressions | `cargo pgrx regress pg18` |
| Bootstrap a regression expectation | `cargo pgrx regress pg18 --add test_name` |
| Promote reviewed regression output | `cargo pgrx regress pg18 --auto` |
| Run backend benchmarks | `cargo pgrx bench pg18` |
| Generate all SQL or selected items | `cargo pgrx schema pg18 [ITEM]...` |
| Create an installation package | `cargo pgrx package` |

`cargo check` skips the final link but still runs build scripts and may require
configured PostgreSQL headers and build dependencies. It does not validate
runtime behavior or FFI safety. `cargo build` produces artifacts without
installing them or starting PostgreSQL.

## Versions, features, and Cargo flags

Supported major-version labels are `pg15` through `pg19`. Where a command accepts
a version selector, selection generally uses the explicit argument, then a
PostgreSQL feature supplied with `--features`, then the manifest's default
PostgreSQL feature unless defaults are disabled. Supply an explicit selector
when the target version matters. `install` and `package` select PostgreSQL
through `--pg-config`, falling back to `pg_config` on `PATH`.

`test`, `start`, and `stop` accept `all` for versions present in both the
extension's features and the active pgrx configuration. `status` defaults to
all configured instances. Other commands do not have that `all` behavior.
Avoid `--all-features` for extensions whose PostgreSQL features are mutually
exclusive. Use `--package` or `--manifest-path` to select a workspace extension.

`bench`, `install`, `package`, `regress`, `run`, `schema`, and `test` accept
repeatable `--cargo <FLAG>` values. These reach the command's Cargo invocations
beginning with `cargo metadata`, so use flags valid for every invoked Cargo
subcommand. See [test.md](test.md) for the backend harness's separate install step.

```bash
cargo pgrx test pg18 --cargo=--config=./cargo-local.toml
cargo pgrx package --cargo "--offline --frozen"
```

Each value is split on ASCII whitespace, without shell quote parsing. Paths or
values containing spaces cannot be preserved by quoting inside `--cargo`.
`PGRX_BUILD_FLAGS` applies to build commands and does not configure the initial
metadata call. Artifact lookup uses Cargo metadata's resolved target directory,
including Cargo configuration and `CARGO_TARGET_DIR`; do not assume `./target`.

## Agent worktrees

Before starting PostgreSQL for worktree work, read
[the private PGRX_HOME procedure](init.md#private-pgrx_home-for-agent-worktrees).
Use a private home and distinct managed ports, reuse an existing PostgreSQL
installation when appropriate, and keep track of servers created by the task.
Sharing binaries still shares extension installation directories, so account
for that before installing a worktree's extension.

`run`, `connect`, `regress`, and `bench` can start a managed server and leave it
running. Stop temporary instances using the same private `PGRX_HOME` before
removing their data or abandoning the worktree. The backend test harness uses
separate clusters and dynamic ports; see [test.md](test.md) for its cleanup.

## References

- [init.md](init.md): initialization, port selection, private worktree homes
- [test.md](test.md): Rust and backend tests, filters, test cluster lifecycle
- [run.md](run.md): build, install, restart, and open a client
- [regress.md](regress.md): SQL regressions and expected-output management
- [bench.md](bench.md): backend benchmarks and stored comparisons
- [install.md](install.md): installation into a chosen PostgreSQL prefix
- [schema.md](schema.md): full schemas and selected SQL items
- [package.md](package.md): installation packages and artifact placement
- [new.md](new.md): extension scaffolding
- [instance-management.md](instance-management.md): start, stop, status, connect
- [utilities.md](utilities.md): info, get, upgrade, cross
