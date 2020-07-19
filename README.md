[![Actions Status](https://github.com/zombodb/pgx/workflows/cargo%20test%20--all/badge.svg)](https://github.com/zombodb/pgx/actions)
[![crates.io badge](https://img.shields.io/crates/v/pgx.svg)](https://crates.io/crates/pgx)
[![docs.rs badge](https://docs.rs/pgx/badge.svg)](https://docs.rs/pgx)
[![Twitter Follow](https://img.shields.io/twitter/follow/zombodb.svg?style=flat)](https://twitter.com/zombodb)
---

[![Logo](logo.png)](https://twitter.com/zombodb)

# pgx

###### Build Postgres Extensions with Rust!

`pgx` is a framework for developing PostgreSQL extensions in Rust and wants to make that process as idiomatic and safe
as possible.  Currently, `pgx` supports Postgres v10, v11, and v12.

## Key Features

 - A cargo sub-command (`pgx`) for creating, compiling/installing, and testing extensions
 - Self manages each Postgres version installation for you (or you can bring your own)
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


### Install `cargo pgx`
First you'll want to install the `pgx` cargo sub-command from crates.io.  You'll use it almost exclusively during
your development and testing workflow.

```shell script
$ cargo install cargo-pgx
```

### Initialize it

Next, `pgx` needs to be initialized.  You only need to do this once.

```shell script
$ cargo pgx init
```

The `init` command downloads Postgres versions 10, 11, 12, and compiles them to `~/.pgx/`.  These installations
are needed by `pgx` not only for auto-generating Rust bindings from each version's header files, but also for `pgx`'s 
test framework.


### Create a new extension

```shell script
$ cargo pgx new my_extension
``` 

### Run your extension

```shell script
$ cd my_extension
$ cargo pgx run pg12  # or pg10 or pg11
```

`cargo pgx run` compiles the extension to a shard library, copies it to the specified Postgres installation (in `~/.pgx/`),
starts that Postgres instance and connects you, via `psql` to a database named for the extension.
 
The first time, compilation takes a few minutes as `pgx` needs to generate almost 200k lines of Rust "bindings" from
Postgres' header files.

Once compiled you'll be placed in a `psql` shell, for, in this case, Postgres 12.

```shell script
$ createdb test
$ psql test
> CREATE EXTENSION my_extension;
> SELECT hello_my_extension();
```

## Digging Deeper

 - [cargo-pgx sub-command](cargo-pgx/)
 - [various examples](pgx-examples/)


## Contributing

We are most definitely open to contributions of any kind.  Bug Reports, Feature Requests, Documentation,
and even [sponsorships](https://github.com/sponsors/eeeebbbbrrrr).

Providing wrappers for Postgres' internals is not a straightforward task, and completely wrapping it is going
to take quite a bit of time.  `pgx` is generally ready for use now, and it will continue to be developed as
time goes on.  Your feedback about what you'd like to be able to do with `pgx` is greatly appreciated.

## License

```
Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. 
Use of this source code is governed by the MIT license that can be found in the LICENSE file.
```
