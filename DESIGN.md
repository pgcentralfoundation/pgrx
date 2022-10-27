🚨 DRAFT OUTLINE 🚨

This document attempts to describe the general design decisions `pgx` has made as it relates to providing a Rust-y API
around Postgres internals.

## A Note from @eeeebbbbrrrr

pgx was developed out of frustrations with developing Postgres extensions in C and SQL, specifically @ZomboDB. ZomboDB 
quickly evolved into a fairly large extension.  Roughly 25k lines of C, 8k lines of hand-written SQL, and all sorts of 
bash machinery to accommodate testing, releases, and general project maintenance.  In C, it is just too much for one 
person to manage.  I needed a hand and the Rust compiler seemed like a good coworker.

Today, ZomboDB is still about 25k lines -- but of Rust!, about 1k lines of hand-written SQL, and much less bash
machinery.  And the Rust compiler really is a good coworker.

## Prior Art

- https://github.com/jeff-davis/postgres-extension.rs
- https://github.com/bluejekyll/pg-extend-rs
- https://github.com/wasmerio/wasmer-postgres

None of these appear active today.

## Goals

pgx' primary goal is to make Rust extension development as natural to Rust programmers as possible.  This generally 
means doing the best we can to avoid violating [the principle of least astonishment](https://en.wikipedia.org/wiki/Principle_of_least_astonishment) 
as it relates to error handling, memory management, and type conversion.  

As part of this goal, pgx tries to provide larger Rust APIs around internal Postgres APIs.  Once such example is its 
safe Rust API for creating Postgres aggregate functions.  The developer implements a single trait, 
`pgx::aggregate::Aggregate`, described in safe Rust, and pgx handles all the code and SQL generation to expose the 
aggregate function to the SQL-level.

pgx also provides "unsafe" access to all of Postgres' internals.  Doing so allows pgx, and especially developers, to 
access all Postgres internals -- especially those not yet wrapped in safe Rust APIs.

The secondary goal, which is not any less important, is an SQL schema generator that programmatically generates 
extension SQL from the extension Rust sources.  Other than solving the lazy programmer problem, the schema generator 
ensures certain SQL<-->Postgres<-->Rust invariants are maintained, which includes typing for function arguments and 
return values.

Finally, pgx wants to improve the day-to-day extension development experience.  `cargo-pgx` is the Cargo plugin that 
does the heavy lifting of manging Postgres instances for running and testing an extension.

## Non-Goals

pgx does not aim to present safe wrappers around Postgres internals in a one-to-one mapping.  Where it can (and so far 
has), it endeavors to wrap larger APIs that look and feel like Rust, not Postgres' version of C.  This is an 
enormous undertaking which will continue for many years.

## Rust Bindings Generation

pgx uses [`bindgen`](https://github.com/rust-lang/rust-bindgen) to generate Rust "bindings" from Postgres' C headers.
Ultimately, Rust versions of Postgres' internal structs, typedefs, functions definitions, globals, and enums are 
generated.

The generated bindings are part of the sibling `pgx-pg-sys` crate and are exported through `pgx` as the `::pg_sys` 
module.  One set of bindings are generated for each supported Postgres version, and the proper one is used based on Rust 
conditional compilation feature flags.

A bit of post-processing is done to provide some developer conveniences.  

First, a graph is built of all Postgres internal structs that appear to be a Postgres `Node`.  The resulting struct 
definitions are then augmented to `impl Display` (via `pg_sys::nodeToString()`) and to also`impl PgNode`, which is 
essentially a marker trait.

Additionally, pgx auto-generates a Rust `PgBuiltInOids` enum from the set of `#define xxxOID` defines found in the
Postgres headers.

pgx reserves the right to generate other convenience types, traits, etc in the future.  Such things may come from user
requests or from the need to ensure more robust compile-time type checking and safety guarantees.

The bindings are generated via the `build.rs` script in the `pgx-pg-sys` crate.  As some of Postgres header files are 
themselves machine-generated, pgx requires its bindings be generated from these machine-generated headers.  When pgx is 
managing Postgres instances itself (via `cargo-pgx`), it gets them from there, otherwise the "-server"/"-server-dev" 
distribution packages must be locally installed.

## Error Handling

Postgres has a few different methods of raising errors -- the `ERROR`, `FATAL`, and `PANIC` levels.  `ERROR` aborts
the current transaction, `FATAL` terminates the raising backend process, and `PANIC` restarts the entire cluster.

Rust only has one: `panic!()`, and it terminates the process.

pgx, wanting to be as least surprising as possible to Rust developers, provides mechanisms for seamlessly handling
Postgres `ERROR`s and Rust `panic!()`s.  There's two concerns here.  One is that Rust developers expect proper drop 
semantics during stack unwinding.  The other is that Postgres expects the *transaction* to abort in the face of a 
recoverable error, not the entire process.

### Protecting the Rust Stack (read: lying to Postgres)

Postgres uses the POSIX `sigsetjmp` and `longjmp` functions to implement its transaction error handling machinery.  
Essentially, at the point in the code where a new transaction begins, Postgres creates `sigsetjmp` point, and if an 
ERROR is raised during the transaction, Postgres `longjmp`s back to that point, taking the other branch to abort the 
transaction and perform necessary cleanup.  This means that Postgres is jumping to a stack frame from an earlier point 
in time.  

Jumping across Rust stack frames is unsound at best, and completely unsafe at worst.  Specifically, doing so defeats any
Rust destructor that was planned to execute during normal stack unwinding.  The ramifications of this can vary from 
leaking memory to leaving things like reference-counted objects or locks completely unaware that they've been released.  
If Rust's stack does not properly unwind, it is impossible to know the program state after `sigsetjmp`'s second return.

With this in mind, it's also important to know that any Postgres internal function is subject to raise an ERROR, causing
a `longjmp`.

To solve this, pgx creates its own `sigsetjmp` points at each Postgres FFI boundary. As part of pgx' machine-generated 
Rust bindings, it wraps each Postgres function, adding its own `sigsetjmp` handling prior to calling the internal 
Postgres function.  The wrapper lies and tells Postgres that if it raises an error it should `longjmp` to this 
`sigsetjmp` point instead of the one Postgres created at transaction start.  Regardless of if an ERROR is raised, pgx 
restores Postgres' understanding of the original `sigsetjmp` point after the internal function returns.  These wrapper 
functions are what's exposed to the user, ensuring it's not possible to bypass this protection.

(as an aside, I did some quick LLVM IR callgraph analysis a few years ago and found that, indeed, nearly every Postgres
function can raise an ERROR (either directly or indirectly through its callgraph).  I don't recall the specific numbers,
but it was such a small percentage that didn't call `ereport()` that I didn't even consider adding such smarts to pgx' 
bindings generator)

When Postgres `longjmp`s due to an ERROR during a function call made through pgx, its wrapper function's `sigsetjmp` 
point activates and the ERROR is converted into a Rust `panic!()`.  This panic is then raised using Rust's normal panic 
machinery.  As such, the Rust stack unwinds properly, ensuring all Rust destructors are called.

After Rust's stack has unwound, the `panic!()` is handed off to `ereport()`, passing final error handling control back 
to Postgres.

This process is similar to Postgres' `PG_TRY`/`PG_CATCH` C macros.  However, it happens at *every* Rust-->Postgres 
function call boundary, allowing Rust to unwind its stack and call destructors before relinquishing control to 
Postgres.

### Protecting Postgres Transactions (read: asking Rust for help)

The above is about how to handle Postgres ERRORs when an internal Postgres function is called from Rust.  This section
talks about how to handle Rust `panic!()`s generated by a Rust function called from Postgres.

A decision was made during initial pgx development that **all** Rust `panic!()`s will properly abort the active
Postgres transaction.  While Rust's preference is to abort the **process**, this makes no sense in the context of a
transaction-aware database and would be quite astonishing to even the most general Postgres user.

Postgres has many places where it wants an (in Rust-speak) `extern "C"` function pointer.  Probably the most common 
place is when it calls an extension-provided function via SQL.

When the function is written in Rust, it is important to ensure that should it raise a Rust `panic!()`, the panic is 
properly converted into a regular Postgres ERROR.  Trying to propagate a Rust panic through Postgres' C stack is 
undefined behavior and even it weren't, Rust's unwind machinery  would not know how to instead `longjmp` to Postgres' 
transaction `sigsetjmp` point.

pgx fixes this via its `#[pg_guard]` procmacro.  pgx' `#[pg_extern]`, `#[pg_operator]`, etc macros apply `#[pg_guard]` 
automatically, so in general users don't need to think about this.  It does come up when a pgx extension needs to 
provide Postgres with a function pointer.  For example, the "index access method API" requires a number of function 
pointers. In this case, each function needs a `#[pg_guard]` attached to it.

`#[pg_guard]` sets up a "panic boundary" using Rust's `std::panic::catch_unwind(|| ...)` facility.  This function takes 
a closure as its argument and returns a `Result`, where the `Error` variant contains the panic data.  The `Ok` variant, 
of course, contains the closure's return value if no panic was raised.

Everything executed within the `catch_unwind()` boundary adheres to proper Rust stack unwinding and destructor rules.  
When it does catch a `panic!()` pgx will then call Postgres' `ereport()` function with it.  How its handled from there
depends how deep in the stack we are between Rust and internal Postgres functions, but ultimately, at the end of the
original `#[pg_guard]`, the panic information is properly handed off to Postgres as an ERROR.

---

Together, these two error handling mechanisms allow a pgx-based extension to safely raise (and survive!) runtime 
"errors" from both Rust and Postgres and "bounce" back-n-forth through either type.  Assuming `#[pg_guard]` is properly 
applied a call chain like `Postgres-->Rust-->Rust-->Rust-->Postgres-->Rust(panic!)` will properly unwind the Rust stack, 
call Rust destructors, and safely abort the active Postgres transaction. So will a call chain like 
`Postgres-->Rust-->Postgres-->Rust-->Rust-->Postgres(ERROR)`.

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
 
## Thread Safety

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

## `cargo-pgx` is Your Friend

TODO

## `postgrestd` Interactions

- TODO:  What are they, why are they, and are we sure they're good decisions?