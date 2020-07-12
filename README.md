[![Actions Status](https://github.com/zombodb/pgx/workflows/test/badge.svg)](https://github.com/zombodb/pgx/actions)
[![crates.io badge](https://img.shields.io/crates/v/pgx.svg)](https://crates.io/crates/pgx)
[![docs.rs badge](https://docs.rs/pgx/badge.svg)](https://docs.rs/pgx)

[![Twitter Follow](https://img.shields.io/twitter/follow/zombodb.svg?style=social)](https://twitter.com/zombodb)

# pgx

`pgx` is a framework for developing PostgreSQL extensions in Rust and wants to make that process as idiomatic and safe
as possible.  Currently, `pgx` supports Postgres v10, v11, and v12.

## Key Features

 - A cargo sub-command (`pgx`) for creating, compiling/installing, and testing extensions
 - Postgres `Datum`<-->Rust type conversion via `pgx::IntoDatum` and `pgx::FromDatum`
 - Safe handling of `NULL` Datums -- Datums are simply `Option<T>`
 - Translation of Rust `panic!()`s into Postgres `ERROR`s, which abort the current transaction instead of the Postgres cluster
 - `#[derive(PostgresType)]` macro for automatically generating Postgres types based on Rust structs
 - `#[derive(PostgresEnum)]` macro for automatically generating Postgres enums based on Rust enums
 - `extension_sql!()` macro for providing custom extension schema DDL
 - `#[pg_extern]` proc-macro for automatically creating UDFs
 - Automatic extension schema generation
 - Transparent support for generating Set Returning Functions (SRFs) by returning a `std::iter::Iterator<Item = T>`
 - `#[pg_test]` proc-macro for unit tests that run **in-proccess** in Postgres
 - `PgMemoryContexts` wrapper around Postgres' "MemoryContext" system
 - Executor/planner/transaction/subtransaction hook support
 - `#[pg_guard]` proc-macro for guarding `extern "C"` Rust functions that need to be passed into Postgres
 - Basic SPI support
 - Direct `unsafe` access to large parts of Postgres internals via the `pgx::pg_sys` module
 - Separation of Postgres symbols (types, functions, etc) by what's common across all supported versions, and then
 version-specific modules
 - lots more!

## Getting Started

First you'll want to install the `pgx` cargo sub-command from crates.io.  You'll use it almost exclusively during
your development and testing workflow.

```shell script
$ cargo install pgx
```

It has a number of sub-commands.  For example, to create a new extension project, simply run:

```shell script
$ cargo pgx new my_extension
``` 

Then `cd my_extension` and run:

```shell script
$ cargo pgx install
```

The first time, this will take awhile.  Behind the scenes, `pgx` is downloading, configuring, compiling and installing
(within `target/`) Postgres v10, v11, and v12.  All of this happens in the `target/` directory and the artifacts
will remain until a `cargo clean`.  This is necessary in order to generate the proper Rust bindings for Postgres internals.

Note that `cargo pgx install` will compile your extension and then install it to your locally installed Postgres instance
as identified by `pg_config`, so make sure that `pg_config` is in your `$PATH`.

From here, you can create a Postgres database and create your extension in it:

```shell script
$ createdb test
$ psql test
> CREATE EXTENSION my_extension;
> SELECT hello_my_extension();
```

## Digging Deeper

 - [cargo-pgx sub-command](cargo-pgx/)
 - [various examples](pgx-examples/)


 