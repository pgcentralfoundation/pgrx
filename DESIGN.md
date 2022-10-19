🚨 DRAFT OUTLINE 🚨

This document attempts to describe the general design decisions `pgx` has made as it relates to providing a Rust-y API
around Postgres internals.
 
## Rust Bindings Generation

- Generated from compiled-Postgres header files (ie, from "-server" or "-server-dev" distro package)
- "OID" type value definitions converted to Rust enum (`PgOid`)
- Various trait impls for certain Postgres structs (`impl Display for Node`, `impl PgNode for Node`)
- Separate bindings for each supported Postgres version, behind feature flags

## Memory Allocation

- Rust allocates as it normally does
  - No custom Rust GlobalAllocator
  - `impl Drop` happens when expected
- `PgBox` exists for managing Postgres-allocated pointers
  - Knows (via a ZST) who allocated the backing pointer (via `palloc()`), and if Rust did so follows Rust's drop semantics, otherwise the backing pointer is freed by Postgres when it decides
- Any Rust-allocated object can be purposely copied into a Postgres MemoryContext and have its `drop()` implementation called when Postgres frees that MemoryContext
  - `PgMemoryContexts::leak_and_drop_on_delete()` is responsible for this
- `PgMemoryContexts` enum for easily executing code within a specific Postgres memory context
  - In general one doesn't worry since Rust is generally managing allocations 
- Direct access to `palloc/pfree` (and friends)

## Error Handling

- `sigsetjmp` boundaries around **every** Postgres-internal FFI call
  - Allows pgx to catch Postgres `ERROR`s and convert to Rust `panic!()`s to ensure proper stack unwinding and that **Rust destructors are called**
- `catch_unwind` boundaries for `#[pg_extern]`-style functions for converting Rust `panic!()`s to Postgres `ERROR`s
  - Rust `panic!()`s are ultimately passed to Postgres' `ereport()`
- No user-discernible difference between a Rust `panic!()` and a Postgres `error!()`
- `#[pg_guard]` procmacro for wrapping Rust `extern "C"` functions that need to be passed to Postgres as function pointers
  - Also ensures proper stack unwinding across FFI boundaries
- `fn pg_try(||) -> PgTryResult` for executing code in a recoverable manner in the face of Postgres ERRORs
  - Makes use of `sigsetjmp`
  - Akin to Postgres' `PG_TRY/PG_CATCH/PG_FINALLY/PG_RE_THROW` C macros
  
## Thead Safety

- As it relates to any Postgres-thing (calling a function, allocated memory, anything at all from `pgx_pg_sys::`), there is none
- We (will) detect FFI calls into Postgres in non-the-main-thread and immediately panic
- pgx extensions can use threads so long as they **don't use any** Postgres-thing within them

## Type Conversions

- Postgres Datums are converted to equivalent Rust types via `pgx::FromDatum`
  - Sometimes this is a built-in from Rust's standard library (ie, `text/varchar` -> `String`)
  - Sometimes this is a pgx-specific wrapper type (ie, `timestamp with time zone` -> `pgx::TimestampWithTimeZone`)
  - Some types are converted zero-copy (deTOASTING may occur, of course)
- Rust type instances are converted into Postgres-allocated memory (for pass-by-reference Datums) via `pgx::IntoDatum`
  - This is not a zero-copy conversion
- Any supported type instance can be `NULL` if it's written as `Option<T>`

## `postgrestd` Interactions

- TODO:  What are they, why are they, and are we sure they're good decisions?