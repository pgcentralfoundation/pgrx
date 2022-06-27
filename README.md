![Logo](logo-transparent.png)

# `pgx`

> Build Postgres Extensions with Rust!

![cargo test --all](https://github.com/zombodb/pgx/workflows/cargo%20test%20--all/badge.svg)
[![crates.io badge](https://img.shields.io/crates/v/pgx.svg)](https://crates.io/crates/pgx)
[![docs.rs badge](https://docs.rs/pgx/badge.svg)](https://docs.rs/pgx)
[![Twitter Follow](https://img.shields.io/twitter/follow/zombodb.svg?style=flat)](https://twitter.com/zombodb)
[![Discord Chat](https://img.shields.io/discord/561648697805504526.svg)](https://discord.gg/hPb93Y9)


`pgx` is a framework for developing PostgreSQL extensions in Rust and strives to be as idiomatic and safe as possible.

`pgx` supports Postgres v10-v14.

**Feel free to join our [Discord Server](https://discord.gg/hPb93Y9).**

## Key Features

- **A fully managed development environment with [`cargo-pgx`](./cargo-pgx/README.md)**
   + `cargo pgx new`: Create new extensions quickly
   + `cargo pgx init`: Install new (or register existing) PostgreSQL installs
   + `cargo pgx run`: Run your extension and interactively test it in `psql` (or `pgcli`)
   + `cargo pgx test`: Unit-test your extension across multiple PostgreSQL versions
   + `cargo pgx package`: Create installation packages for your extension
   + More in the [`README.md`](./cargo-pgx/README.md)!
- **Target Multiple Postgres Versions**
   + Support Postgres v10-v14 from the same codebase
   + Use Rust feature gating to use version-specific APIs
   + Seamlessly test against all versions
- **Automatic Schema Generation**
   + Implement extensions entirely in Rust
   + [Automatic mapping for many Rust types into PostgreSQL](#mapping-of-postgres-types-to-rust)
   + SQL schemas generated automatically (or manually via `cargo pgx schema`)
   + Include custom SQL with `extension_sql!` & `extension_sql_file!`
- **Safety First**
   + Translates Rust `panic!`s into Postgres `ERROR`s that abort the transaction, not the process
   + Memory Management follows Rust's drop semantics, even in the face of `panic!` and `elog(ERROR)`
   + `#[pg_guard]` procedural macro to ensure the above
   + Postgres `Datum`s are `Option<T> where T: FromDatum`
      - `NULL` Datums are safely represented as `Option::<T>::None`
- **First-class UDF support**
   + Annotate functions with `#[pg_extern]` to expose them to Postgres
   + Return `impl std::iter::Iterator<Item = T> where T: IntoDatum` for `RETURNS SETOF` and `RETURNS TABLE (...)`
   + Create trigger functions with `#[pg_trigger]`
- **Easy Custom Types**
   + `#[derive(PostgresType)]` to use a Rust struct as a Postgres type
      - By default, represented as a CBOR-encoded object in-memory/on-disk, and JSON as human-readable
      - Provide custom in-memory/on-disk/human-readable representations
   + `#[derive(PostgresEnum)]` to use a Rust enum as a Postgres enum
   + Composite types supported with the `pgx::composite_type!("Sample")` macro
- **Server Programming Interface (SPI)**
   + Safe access into SPI
   + Transparently return owned Datums from an SPI context
- **Advanced Features**
   + Safe access to Postgres' `MemoryContext` system via `pgx::PgMemoryContexts`
   + Executor/planner/transaction/subtransaction hooks
   + Safely use Postgres-provided pointers with `pgx::PgBox<T>` (akin to `alloc::boxed::Box<T>`)
   + `#[pg_guard]` proc-macro for guarding `extern "C"` Rust functions that need to be passed into Postgres
   + Access Postgres' logging system through `eprintln!`-like macros
   + Direct `unsafe` access to large parts of Postgres internals via the `pgx::pg_sys` module
   + New features added regularly!

## System Requirements

- A `rustup` managed toolchain (>=1.58) (or `rustc`, `cargo`, and `rustfmt`)
- `git`
- `libclang`
   - Ubuntu: `libclang-dev` or `clang`
   - RHEL: `clang`
- `tar`
- `bzip2`
- GCC 7 or newer
- [Build dependencies for PostgreSQL](https://wiki.postgresql.org/wiki/Compile_and_Install_from_source_code)

<details>
   <summary>How to: GCC 7 on CentOS 7</summary>
   
In order to use GCC 7, install [`scl`](https://wiki.centos.org/AdditionalResources/Repositories/SCL) and enter the GCC 7 development environment:

```bash
yum install centos-release-scl
yum install devtoolset-7
scl enable devtoolset-7 bash
```
</details>

Note that a local PostgreSQL Server installation is not required. `pgx` typically downloads and compiles PostgreSQL versions itself.

## Getting Started


First install the `cargo-pgx` sub-command and initialize the development environment:

```bash
cargo install cargo-pgx
cargo pgx init
```

The `init` command downloads PostgreSQL versions v10 through v14 compiles them to `~/.pgx/`, and runs `initdb`. It's also possible to use an existing (user-writable) PostgreSQL install, or install a subset of versions, see the [`README.md` of `cargo-pgx` for details](cargo-pgx/README.md#first-time-initialization).

```bash
cargo pgx new my_extension
cd my_extension
```

This will create a new directory for the extension crate.

```
$ tree 
.
├── Cargo.toml
├── my_extension.control
├── sql
└── src
    └── lib.rs

2 directories, 3 files
```

The new extension includes an example, so you can go ahead and run it right away.

```bash
cargo pgx run
```

This compiles the extension to a shared library, copies it to the specified Postgres installation, starts that Postgres instance and connects you to a database named the same as the extension.

Once `cargo-pgx` drops us into `psql` we can [load the extension](https://www.postgresql.org/docs/13/sql-createextension.html) and do a SELECT on the example function.

```sql
my_extension=# CREATE EXTENSION my_extension;
CREATE EXTENSION

my_extension=# SELECT hello_my_extension();
 hello_my_extension
---------------------
 Hello, my_extension
(1 row)
```

For more details on how to manage pgx extensions see [Managing pgx extensions](./cargo-pgx/README.md).

## Upgrading

You can upgrade your current `cargo-pgx` installation by passing the `--force` flag
to `cargo install`:

```bash
cargo install --force cargo-pgx
```

As new Postgres versions are supported by `pgx`, you can re-run the `pgx init` process to download and compile them:

```bash
cargo pgx init
```

### Mapping of Postgres types to Rust

Postgres Type | Rust Type (as `Option<T>`)
--------------|-----------
`bytea` | `Vec<u8>` or `&[u8]` (zero-copy)
`text` | `String` or `&str` (zero-copy)
`varchar` | `String` or `&str` (zero-copy) or `char`
`"char"` | `i8`
`smallint` | `i16`
`integer` | `i32`
`bigint` | `i64`
`oid` | `u32`
`real` | `f32`
`double precision` | `f64`
`bool` | `bool`
`json` | `pgx::Json(serde_json::Value)`
`jsonb` | `pgx::JsonB(serde_json::Value)`
`date` | `pgx::Date`
`time` | `pgx::Time`
`timestamp` | `pgx::Timestamp`
`time with time zone` | `pgx::TimeWithTimeZone`
`timestamp with time zone` | `pgx::TimestampWithTimeZone`
`anyarray` | `pgx::AnyArray`
`anyelement` | `pgx::AnyElement`
`box` | `pgx::pg_sys::BOX`
`point` | `pgx::pgx_sys::Point`
`tid` | `pgx::pg_sys::ItemPointerData`
`cstring` | `&std::ffi::CStr`
`inet` | `pgx::Inet(String)` -- TODO: needs better support
`numeric` | `pgx::Numeric(String)` -- TODO: needs better support
`void` | `()`
`ARRAY[]::<type>` | `Vec<Option<T>>` or `pgx::Array<T>` (zero-copy)
`NULL` | `Option::None`
`internal` | `pgx::PgBox<T>` where `T` is any Rust/Postgres struct
`uuid` | `pgx::Uuid([u8; 16])`

There are also `IntoDatum` and `FromDatum` traits for implementing additional type conversions,
along with `#[derive(PostgresType)]` and `#[derive(PostgresEnum)]` for automatic conversion of
custom types.

## Digging Deeper

 - [cargo-pgx sub-command](cargo-pgx/)
 - [Custom Types](pgx-examples/custom_types/)
 - [Postgres Operator Functions and Operator Classes/Families](pgx-examples/operators/)
 - [Shared Memory Support](pgx-examples/shmem/)
 - [various examples](pgx-examples/)

## Caveats & Known Issues

There's probably more than are listed here, but a primary things of note are:

 - Threading is not really supported.  Postgres is strictly single-threaded.  As such, if you do venture into using threads, those threads **MUST NOT** call *any* internal Postgres function, or otherwise use any Postgres-provided pointer.  There's also a potential problem with Postgres' use of `sigprocmask`.  This was being discussed on the -hackers list, even with a patch provided, but the conversation seems to have stalled (https://www.postgresql.org/message-id/flat/5EF20168.2040508%40anastigmatix.net#4533edb74194d30adfa04a6a2ce635ba).
 - `async` interactions are unknown right now.
 - `pgx` uses lots of `unsafe` Rust.  That's generally the nature of the beast when doing FFI wrappers, so be aware.
 - Not all of Postgres' internals are included or even wrapped.  This isn't due to it not being possible, it's simply due to it being an incredibly large task.  If you identify internal Postgres APIs you need, open an issue and we'll get them exposed, at least through the `pgx::pg_sys` module.
 - Windows is not supported.  It could be, but will require a bit of work with `cargo-pgx` and figuring out how to compile `pgx`'s "cshim" static library.
 - Sessions started before `ALTER EXTENSION my_extension UPDATE;` will continue to see the old version of `my_extension`. New sessions will see the updated version of the extension.

## TODO

There's a few things on our immediate TODO list

 - Automatic extension schema upgrade scripts, based on diffs from a previous git tag and HEAD.  Likely, this
will be built into the `cargo-pgx` subcommand and make use of https://github.com/zombodb/postgres-parser.
 - More examples -- especially around memory management and the various derive macros `#[derive(PostgresType/Enum)]`

## Contributing

We are most definitely open to contributions of any kind.  Bug Reports, Feature Requests, Documentation,
and even [sponsorships](https://github.com/sponsors/eeeebbbbrrrr).

If you'd like to contribute code via a Pull Request, please make it against our `develop` branch.  The `master` branch is meant to represent what is currently available on crates.io.

Providing wrappers for Postgres' internals is not a straightforward task, and completely wrapping it is going
to take quite a bit of time.  `pgx` is generally ready for use now, and it will continue to be developed as
time goes on.  Your feedback about what you'd like to be able to do with `pgx` is greatly appreciated.


## License

```
Portions Copyright 2019-2021 ZomboDB, LLC.  
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>. 
All rights reserved.
Use of this source code is governed by the MIT license that can be found in the LICENSE file.
```
