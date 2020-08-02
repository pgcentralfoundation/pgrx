# pgx-examples

This directory contains examples of how to work with various aspects of `pgx`.

- [arrays/](arrays/):  Working with Arrays
- [bad_ideas/](bad_ideas/):  Some "bad ideas" to do in Postgres extensions
- [bytea/](bytea/):  Working with Postgres' `bytea` type as `Vec<u8>` and `&[u8]` in Rust
- [custom_types/](custom_types/): Create your own custom Postgres types backed by Rust structs/enums
- [errors/](errors/):  Error handling using Postgres or Rust errors/panics
- [operators/](operators/):  Creating operator functions and associated `CREATE OPERATOR` ddl
- [spi/](spi/):  Using Postgres' Server Programming Interface (SPI)
- [srf/](srf/):  Set-Returning-Functions
- [strings/](strings/):  Using Postgres `text`/`varlena` types as Rust `String`s and `&str`s