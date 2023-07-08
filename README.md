![Logo](art/pgrx-logo-color-transparent-475x518.png)

# `pgrx`

> Build Postgres Extensions with Rust!

![cargo test --all](https://github.com/tcdi/pgrx/workflows/cargo%20test%20--all/badge.svg)
[![crates.io badge](https://img.shields.io/crates/v/pgrx.svg)](https://crates.io/crates/pgrx)
[![docs.rs badge](https://docs.rs/pgrx/badge.svg)](https://docs.rs/pgrx)
[![Twitter Follow](https://img.shields.io/twitter/follow/zombodb.svg?style=flat)](https://twitter.com/zombodb)
[![Discord Chat](https://img.shields.io/discord/561648697805504526.svg)][Discord]


`pgrx` is a framework for developing PostgreSQL extensions in Rust and strives to be as idiomatic and safe as possible.

`pgrx` supports Postgres v11-v15.

**Feel free to join our [Discord Server](https://discord.gg/hPb93Y9).**

## Key Features

- **A fully managed development environment with [`cargo-pgrx`](cargo-pgrx/README.md)**
   + `cargo pgrx new`: Create new extensions quickly
   + `cargo pgrx init`: Install new (or register existing) PostgreSQL installs
   + `cargo pgrx run`: Run your extension and interactively test it in `psql` (or `pgcli`)
   + `cargo pgrx test`: Unit-test your extension across multiple PostgreSQL versions
   + `cargo pgrx package`: Create installation packages for your extension
   + More in the [`README.md`](cargo-pgrx/README.md)!
- **Target Multiple Postgres Versions**
   + Support Postgres v11-v15 from the same codebase
   + Use Rust feature gating to use version-specific APIs
   + Seamlessly test against all versions
- **Automatic Schema Generation**
   + Implement extensions entirely in Rust
   + [Automatic mapping for many Rust types into PostgreSQL](#mapping-of-postgres-types-to-rust)
   + SQL schemas generated automatically (or manually via `cargo pgrx schema`)
   + Include custom SQL with `extension_sql!` & `extension_sql_file!`
- **Safety First**
   + Translates Rust `panic!`s into Postgres `ERROR`s that abort the transaction, not the process
   + Memory Management follows Rust's drop semantics, even in the face of `panic!` and `elog(ERROR)`
   + `#[pg_guard]` procedural macro to ensure the above
   + Postgres `Datum`s are `Option<T> where T: FromDatum`
      - `NULL` Datums are safely represented as `Option::<T>::None`
- **First-class UDF support**
   + Annotate functions with `#[pg_extern]` to expose them to Postgres
   + Return `pgrx::iter::SetOfIterator<'a, T>` for `RETURNS SETOF`
   + Return `pgrx::iter::TableIterator<'a, T>` for `RETURNS TABLE (...)`
   + Create trigger functions with `#[pg_trigger]`
- **Easy Custom Types**
   + `#[derive(PostgresType)]` to use a Rust struct as a Postgres type
      - By default, represented as a CBOR-encoded object in-memory/on-disk, and JSON as human-readable
      - Provide custom in-memory/on-disk/human-readable representations
   + `#[derive(PostgresEnum)]` to use a Rust enum as a Postgres enum
   + Composite types supported with the `pgrx::composite_type!("Sample")` macro
- **Server Programming Interface (SPI)**
   + Safe access into SPI
   + Transparently return owned Datums from an SPI context
- **Advanced Features**
   + Safe access to Postgres' `MemoryContext` system via `pgrx::PgMemoryContexts`
   + Executor/planner/transaction/subtransaction hooks
   + Safely use Postgres-provided pointers with `pgrx::PgBox<T>` (akin to `alloc::boxed::Box<T>`)
   + `#[pg_guard]` proc-macro for guarding `extern "C"` Rust functions that need to be passed into Postgres
   + Access Postgres' logging system through `eprintln!`-like macros
   + Direct `unsafe` access to large parts of Postgres internals via the `pgrx::pg_sys` module
   + New features added regularly!

## System Requirements

- A Rust toolchain: `rustc`, `cargo`, and `rustfmt`. The recommended way to get these is from https://rustup.rs †
- `git`
- `libclang` 5.0 or greater (required by bindgen)
   - Ubuntu: `apt install libclang-dev` or `apt install clang`
   - RHEL: `yum install clang`
- `tar`
- `bzip2`
- GCC 7 or newer
- [PostgreSQL's build dependencies](https://wiki.postgresql.org/wiki/Compile_and_Install_from_source_code) ‡

 † PGRX has no MSRV policy, thus may require the latest stable version of Rust, available via Rustup

 ‡ A local PostgreSQL server installation is not required. `cargo pgrx` can download and compile PostgreSQL versions on its own.

<details>
   <summary>How to: GCC 7 on CentOS 7</summary>
   
In order to use GCC 7, install [`scl`](https://wiki.centos.org/AdditionalResources/Repositories/SCL) and enter the GCC 7 development environment:

```bash
yum install centos-release-scl
yum install devtoolset-7
scl enable devtoolset-7 bash
```
</details>

## Getting Started


First install the `cargo-pgrx` sub-command and initialize the development environment:

```bash
cargo install --locked cargo-pgrx
cargo pgrx init
```

The `init` command downloads currently supported PostgreSQL versions, compiles them to `~/.pgrx/`, and runs `initdb`. It's also possible to use an existing (user-writable) PostgreSQL install, or install a subset of versions, see the [`README.md` of `cargo-pgrx` for details](cargo-pgrx/README.md#first-time-initialization).

```bash
cargo pgrx new my_extension
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
cargo pgrx run
```

This compiles the extension to a shared library, copies it to the specified Postgres installation, starts that Postgres instance and connects you to a database named the same as the extension.

Once `cargo-pgrx` drops us into `psql` we can [load the extension](https://www.postgresql.org/docs/13/sql-createextension.html) and do a SELECT on the example function.

```sql
my_extension=# CREATE EXTENSION my_extension;
CREATE EXTENSION

my_extension=# SELECT hello_my_extension();
 hello_my_extension
---------------------
 Hello, my_extension
(1 row)
```

For more details on how to manage pgrx extensions see [Managing pgrx extensions](cargo-pgrx/README.md).

## Upgrading

You can upgrade your current `cargo-pgrx` installation by passing the `--force` flag
to `cargo install`:

```bash
cargo install --force --locked cargo-pgrx
```

As new Postgres versions are supported by `pgrx`, you can re-run the `pgrx init` process to download and compile them:

```bash
cargo pgrx init
```

### Mapping of Postgres types to Rust

| Postgres Type              | Rust Type (as `Option<T>`)                            |
|----------------------------|-------------------------------------------------------|
| `bytea`                    | `Vec<u8>` or `&[u8]` (zero-copy)                      |
| `text`                     | `String` or `&str` (zero-copy)                        |
| `varchar`                  | `String` or `&str` (zero-copy) or `char`              |
| `"char"`                   | `i8`                                                  |
| `smallint`                 | `i16`                                                 |
| `integer`                  | `i32`                                                 |
| `bigint`                   | `i64`                                                 |
| `oid`                      | `u32`                                                 |
| `real`                     | `f32`                                                 |
| `double precision`         | `f64`                                                 |
| `bool`                     | `bool`                                                |
| `json`                     | `pgrx::Json(serde_json::Value)`                        |
| `jsonb`                    | `pgrx::JsonB(serde_json::Value)`                       |
| `date`                     | `pgrx::Date`                                           |
| `time`                     | `pgrx::Time`                                           |
| `timestamp`                | `pgrx::Timestamp`                                      |
| `time with time zone`      | `pgrx::TimeWithTimeZone`                               |
| `timestamp with time zone` | `pgrx::TimestampWithTimeZone`                          |
| `anyarray`                 | `pgrx::AnyArray`                                       |
| `anyelement`               | `pgrx::AnyElement`                                     |
| `box`                      | `pgrx::pg_sys::BOX`                                    |
| `point`                    | `pgrx::pgrx_sys::Point`                                 |
| `tid`                      | `pgrx::pg_sys::ItemPointerData`                        |
| `cstring`                  | `&core::ffi::CStr`                                    |
| `inet`                     | `pgrx::Inet(String)` -- TODO: needs better support     |
| `numeric`                  | `pgrx::Numeric<P, S> or pgrx::AnyNumeric`               |
| `void`                     | `()`                                                  |
| `ARRAY[]::<type>`          | `Vec<Option<T>>` or `pgrx::Array<T>` (zero-copy)       |
| `int4range`                | `pgrx::Range<i32>`                                     |
| `int8range`                | `pgrx::Range<i64>`                                     |
| `numrange`                 | `pgrx::Range<Numeric<P, S>>` or `pgrx::Range<AnyRange>` |
| `daterange`                | `pgrx::Range<pgrx::Date>`                               |
| `tsrange`                  | `pgrx::Range<pgrx::Timestamp>`                          |
| `tstzrange`                | `pgrx::Range<pgrx::TimestampWithTimeZone>`              |
| `NULL`                     | `Option::None`                                        |
| `internal`                 | `pgrx::PgBox<T>` where `T` is any Rust/Postgres struct |
| `uuid`                     | `pgrx::Uuid([u8; 16])`                                 |

There are also `IntoDatum` and `FromDatum` traits for implementing additional type conversions,
along with `#[derive(PostgresType)]` and `#[derive(PostgresEnum)]` for automatic conversion of
custom types.

Note that `text` and `varchar` are converted to `&str` or `String`, so PGX
assumes that any Postgres database you use it with has a UTF-8-compatible
encoding. Currently, PGX will panic if it detects this is incorrect, to inform
you, the programmer, that you were wrong. However, it is best to not rely on
this behavior, as UTF-8 validation can be a performance hazard, this event was
previously assumed to simply not happen, and PGX may decide to change the
details of how it does UTF-8 validation in the future, as all functions for
doing so are, in any case, fundamentally `unsafe`. For best results, always use
PGX with UTF-8, and set database encodings explicitly upon database creation,
as the default Postgres server encoding, `SQL_ASCII`, will guarantee neither
ASCII nor UTF-8 (as Postgres will then accept but ignore non-ASCII bytes).

## Digging Deeper

 - [cargo-pgrx sub-command](cargo-pgrx/)
 - [Custom Types](pgrx-examples/custom_types/)
 - [Postgres Operator Functions and Operator Classes/Families](pgrx-examples/operators/)
 - [Shared Memory Support](pgrx-examples/shmem/)
 - [various examples](pgrx-examples/)

## Caveats & Known Issues

There's probably more than are listed here, but a primary things of note are:

 - Threading is not really supported.  Postgres is strictly single-threaded.  As such, if you do venture into using threads, those threads **MUST NOT** call *any* internal Postgres function, or otherwise use any Postgres-provided pointer.  There's also a potential problem with Postgres' use of `sigprocmask`.  This was being discussed on the -hackers list, even with a patch provided, but the conversation seems to have stalled (https://www.postgresql.org/message-id/flat/5EF20168.2040508%40anastigmatix.net#4533edb74194d30adfa04a6a2ce635ba).
 - How to correctly interact with Postgres in an `async` context remains unexplored.
 - `pgrx` wraps a lot of `unsafe` code, some of which has poorly-defined safety conditions. It may be easy to induce illogical and undesirable behaviors even from safe code with `pgrx`, and some of these wrappers may be fundamentally unsound. Please report any issues that may arise.
 - Not all of Postgres' internals are included or even wrapped.  This isn't due to it not being possible, it's simply due to it being an incredibly large task.  If you identify internal Postgres APIs you need, open an issue and we'll get them exposed, at least through the `pgrx::pg_sys` module.
 - Windows is not supported.  It could be, but will require a bit of work with `cargo-pgrx` and figuring out how to compile `pgrx`'s "cshim" static library.
 - Sessions started before `ALTER EXTENSION my_extension UPDATE;` will continue to see the old version of `my_extension`. New sessions will see the updated version of the extension.
 - `pgrx` is used by many "in production", but it is not "1.0.0" or above, despite that being recommended by SemVer for production-quality software. This is because there are many unresolved soundness and ergonomics questions that will likely require breaking changes to resolve, in some cases requiring cutting-edge Rust features to be able to expose sound interfaces. While a 1.0.0 release is intended at some point, it seems prudent to wait until it seems like a 2.0.0 release would not be needed the next week and the remaining questions can be deferred.

## TODO

There's a few things on our immediate TODO list

 - Automatic extension schema upgrade scripts, based on diffs from a previous git tag and HEAD.  Likely, this
will be built into the `cargo-pgrx` subcommand and make use of https://github.com/zombodb/postgres-parser.
 - More examples -- especially around memory management and the various derive macros `#[derive(PostgresType/Enum)]`


## Feature Flags
PGRX has optional feature flags for Rust code that do not involve configuring the version of Postgres used,
but rather extend additional support for other kinds of Rust code. These are not included by default.


### "unsafe-postgres": Allow compilation for Postgres forks that have a different ABI

As of Postgres v15, forks are allowed to specify they use a different ABI than canonical Postgres.
Since pgrx makes countless assumptions about Postgres' internal ABI it is not possible for it to 
guarantee that a compiled pgrx extension will probably execute within such a Postgres fork.  You,
dear compiler runner, can make this guarantee for yourself by specifying the `unsafe-postgres` 
feature flag.  Otherwise, a pgrx extension will fail to compile with an error similar to:

```
error[E0080]: evaluation of constant value failed
   --> pgrx/src/lib.rs:151:5
    |
151 | /     assert!(
152 | |         same_slice(pg_sys::FMGR_ABI_EXTRA, b"xPostgreSQL\0"),
153 | |         "Unsupported Postgres ABI. Perhaps you need `--features unsafe-postgres`?",
154 | |     );
    | |_____^ the evaluated program panicked at 'Unsupported Postgres ABI. Perhaps you need `--features unsafe-postgres`?', pgrx/src/lib.rs:151:5
    |
```


## Contributing

We are most definitely open to contributions of any kind.  Bug Reports, Feature Requests, Documentation,
and even [sponsorships](https://github.com/sponsors/eeeebbbbrrrr).

If you'd like to contribute code via a Pull Request, please make it against our `develop` branch.  The `master` branch is meant to represent what is currently available on crates.io.

Providing wrappers for Postgres' internals is not a straightforward task, and completely wrapping it is going
to take quite a bit of time.  `pgrx` is generally ready for use now, and it will continue to be developed as
time goes on.  Your feedback about what you'd like to be able to do with `pgrx` is greatly appreciated.

## Hacking

If you're hacking on `pgrx` and want to ensure your test will run correctly, you need to have the current
implementation of `cargo-pgrx` (from the revision you're working on) in your `PATH`.

An easy way would be to install [cargo-local-install](https://github.com/MaulingMonkey/cargo-local-install):

```shell
cargo install cargo-local-install
```

and then run `cargo local-install` to install `cargo-pgrx` as specified in top-level's Cargo.toml.

Don't forget to prepend `/path/to/pgrx/bin` to your `PATH`!

This approach can also be used in extensions to ensure a matching version of `cargo-pgrx` is used.

## License

```
Portions Copyright 2019-2021 ZomboDB, LLC.  
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>. 
All rights reserved.
Use of this source code is governed by the MIT license that can be found in the LICENSE file.
```

[Discord]: https://discord.gg/hPb93Y9
[timecrate]: https://crates.io/crates/time
