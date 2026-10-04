# Using PostgreSQL C Macros

pgrx generates Rust `macro_rules!` versions of supported PostgreSQL function-like
C macros. You can call them directly as `pgrx::MACRO_NAME!(...)` in an extension.
They follow the selected headers' C arithmetic, casts, evaluation, and storage
rules, including where those rules differ from Rust's native operators.

Start with an extension configured for your PostgreSQL version. The main examples
below use PostgreSQL 18; the Datum conversion example is specifically for
PostgreSQL 15. Unless an example defines a function, place its statements inside
one of your Rust functions or tests.

## Call an expression macro

Use the original C name, including its capitalization. For example, check whether
a transaction ID is outside PostgreSQL's special transaction ID range:

```rust
use pgrx::pg_sys;

let xid = pg_sys::TransactionId::from_inner(3);
let is_normal = pgrx::TransactionIdIsNormal!(xid).get() != 0;
assert!(is_normal);
```

`TransactionId` already has a verified C identity, so it can be passed directly.
The comparison produces C `int` zero or one. Extract that integer with `.get()`
and compare it with zero when you need a Rust `bool`.

Expression macros return an already evaluated C expression wrapper. `.get()`
extracts its native Rust storage; `.into_value()` keeps the tagged C value:

```rust
use pgrx::cmacros::c::{CUnsignedLong, CValue};

let rounded = pgrx::BUFFERALIGN!(33_u32);
let bytes: u64 = rounded.get();
assert!(bytes >= 33);

let tagged: CValue<CUnsignedLong> = rounded.into_value();
assert_eq!(tagged.get(), bytes);
```

The wrapper does not rerun the macro when you extract it. `BUFFERALIGN` uses the
selected installation's alignment, so avoid assuming one fixed alignment across
builds. A pointer result extracts a raw pointer. A raw aggregate result extracts
`MaybeUninit<R>`; extraction alone does not prove that every field is initialized
or that the bytes form a valid Rust `R`.

## Choose the C identity of an input

Fixed-width Rust inputs such as `i32` and `u32` have an established C identity in
the supported target profile. Use explicit Rust suffixes when inference would
otherwise be unclear. pgrx's `pg_sys::Oid`, `TransactionId`, and `Datum` inputs also
have generated bridges that verify their C typedefs and Rust representations.

Raw `i64`, `u64`, and `isize` are ambiguous. On the accepted LP64 profile,
`usize` represents C `size_t` (`unsigned long`). C `unsigned long` and
`unsigned long long` can both occupy a Rust `u64` while having different ranks in
C's arithmetic conversions. A Rust type alias does not recover that distinction.
For example, this fails to compile:

```rust,compile_fail
let size = 33_u64;
let _ = pgrx::BUFFERALIGN!(size);
```

Use a C marker when you know the intended original C type:

```rust
use pgrx::cmacros::c::{CUnsignedLong, CUnsignedLongLong, CValue};

let size = CValue::<CUnsignedLong>::new(33_u64);
let aligned = pgrx::BUFFERALIGN!(size).get();
assert!(aligned >= 33);

let privileges = CValue::<CUnsignedLongLong>::new(1_u64);
let grant_options = pgrx::ACL_GRANT_OPTION_FOR!(privileges).get();
assert_eq!(grant_options, 0x1_0000_0000_u64);
```

`CValue::new` labels the supplied representation; it does not infer the C type
from its magnitude. Select a marker from the C declaration or the macro's
documented invocation requirements. The public input vocabulary lives in
`pgrx::cmacros::c` (also `pg_sys::cmacros::c`).

Binding constants need the same care: bindgen can store a positive C `int`
constant in a Rust `u32`. A direct caller operand follows that Rust storage
type unless you supply the original C identity. For example, label `BLCKSZ`
as C `int` before comparing it with a negative C `int`:

```rust
use pgrx::{cmacros::c::{CInt, CValue}, pg_sys};

let block_size = CValue::<CInt>::new(i32::try_from(pg_sys::BLCKSZ).unwrap());
assert_eq!(pgrx::Min!(block_size, -1_i32).get(), -1);
```

The transpiler preserves the C identity of binding references *inside* a macro
definition. It cannot infer that an arbitrary Rust path at the call site denotes
the original C constant; paths can be renamed or shadowed.

If you already have a `Datum`, pass it as a `Datum`. For PostgreSQL 15, where
`DatumGetInt32` is a C macro, a known pass-by-value int32 Datum can be decoded as
follows:

```rust
use pgrx::pg_sys;

/// Decode a Datum known by the caller to contain a pass-by-value int32.
#[cfg(feature = "pg15")]
fn read_pass_by_value_int32(datum: pg_sys::Datum) -> i32 {
    pgrx::DatumGetInt32!(datum).get()
}
```

The caller must know what the Datum contains. A Datum that carries a pointer is
not an int32 just because its address can be converted to integer bits.
PostgreSQL 18 defines `DatumGetInt32` as a function, so there is no generated
`DatumGetInt32!` macro for that version. Check the selected version's function
bindings when a header uses a function instead of a macro.

## Compose macros before extracting Rust values

Pass generated macro invocations directly to other generated macros:

```rust
let privileges =
    pgrx::ACL_OPTION_TO_PRIVS!(pgrx::ACL_GRANT_OPTION_FOR!(1_u32)).get();
assert_eq!(privileges, 1_u64);
```

The outer macro can request the inner macro's value, addressable storage, or
unevaluated size context as required by the original C expression. The internal
entry arms you may see in generated source implement those contexts; they do not
add ordinary C arguments.

For an intermediate value, keep its C identity with `.into_value()`:

```rust
let grant_options = pgrx::ACL_GRANT_OPTION_FOR!(1_u32).into_value();
let privileges = pgrx::ACL_OPTION_TO_PRIVS!(grant_options).get();
assert_eq!(privileges, 1_u64);
```

Extracting `grant_options` with `.get()` would produce a raw `u64`, which needs a
new explicit C tag before being passed back to a generated macro. The evaluated
wrapper or tagged value retains a value's C type, but does not retain its place
or all source-only properties. In particular, an integer constant expression
equal to zero can be a C null-pointer constant; storing an evaluated zero in a
variable does not give that variable the same capability. Preserve the generated
macro invocation syntax when that source distinction matters, or supply a typed
Rust null pointer where appropriate.

C macro substitution also controls evaluation. Repeated occurrences can evaluate
an argument repeatedly; unused arguments do not evaluate; logical and conditional
branches remain lazy. Do not assume function-style evaluate-once behavior merely
because the invocation looks like a function call.

## Mutate storage with an explicit safety argument

Macros that load or mutate raw storage require an `unsafe` context. This also
applies to a generated assignment to an ordinary Rust local, because the C place
operations have explicit unsafe access contracts:

```rust
use pgrx::pg_sys;

let mut xid = pg_sys::TransactionId::from_inner(u32::MAX);
// SAFETY: xid is initialized, aligned, owned, and exclusively writable.
// This macro performs scalar operations and does not access backend state.
unsafe { pgrx::TransactionIdAdvance!(xid) };
assert_eq!(xid.into_inner(), 3);
```

`TransactionIdAdvance` is a statement macro: it executes its statements in order
and yields `()`. Do not append `.get()` or pass it as an expression operand to
another generated macro.

A field operation need not initialize an entire struct. Keep partial storage as
`MaybeUninit` and use raw pointers until you can establish Rust validity:

```rust
use pgrx::pg_sys;

let mut storage = core::mem::MaybeUninit::<pg_sys::Node>::uninit();
let node = storage.as_mut_ptr();
// SAFETY: node points to live, aligned, exclusively writable storage.
// NodeSetTag initializes type_; nodeTag reads that initialized field.
// Neither operation creates a reference or materializes a complete Node.
unsafe {
    pgrx::NodeSetTag!(node, pg_sys::NodeTag::T_Invalid);
    assert_eq!(pgrx::nodeTag!(node).get(), 0_u32);
}
```

For real PostgreSQL objects, establish the allocation's lifetime, bounds,
alignment, initialization of every field read, and aliasing rules. Account for
memory-context resets, tuple ownership, buffer pins, or whichever resource keeps
the object alive. A non-null pointer alone proves none of those conditions.

## Use return macros in the caller

A macro whose original C body contains `return` returns from the enclosing Rust
function or closure. Call it directly:

```rust
use pgrx::pg_sys;

/// Return an existing Datum without decoding its contents.
fn return_existing_datum(value: pg_sys::Datum) -> pg_sys::Datum {
    pgrx::PG_RETURN_DATUM!(value);
}
```

Do not add an outer `return` or `.get()`: the generated macro performs the return
and applies C assignment conversion to the function's result. Ordinary Rust
cleanup runs on that return. If the enclosing native Rust result has an ambiguous
C identity, the documented explicit form is:

```text
MACRO!(@__pgrx_c_return_as [CMarker]; arguments...)
```

Some C macros reference identifiers supplied implicitly by their C callers.
Rust callers supply those operands explicitly after the original formals. For
example, the generated `PG_RETURN_NULL!` takes `fcinfo`, even though its original
C signature has no arguments. Its documentation lists the required captures;
its write to `fcinfo` still requires the caller's pointer safety argument.

## Follow explicit invocation boundaries

Some C replacements are unparenthesized and can interact with surrounding C
operators. Their generated documentation requires `@__pgrx_c_expression;`, which
chooses the meaning of a parenthesized C invocation. For example, assigning a WAL
segment number from a position immediately after the previous segment:

```rust
use pgrx::cmacros::c::{CUnsignedLongLong, CValue};

let position = CValue::<CUnsignedLongLong>::new(32_u64 * 1024 * 1024);
let mut segment = CValue::<CUnsignedLongLong>::new(0_u64);
// SAFETY: segment is initialized, aligned, owned, and exclusively writable.
// The segment size is positive; this invocation uses only unsigned arithmetic.
unsafe {
    let result = pgrx::XLByteToPrevSeg!(
        @__pgrx_c_expression;
        position, segment, 16_i32 * 1024 * 1024
    );
    assert_eq!(result.get(), 1_u64);
}
assert_eq!(segment.get(), 1_u64);
```

Unbraced conditional statement replacements can similarly require:

```text
MACRO!(@__pgrx_c_statement; arguments...)
```

That form chooses a braced C invocation and prevents a caller's `else` from
changing the macro's control flow. Put an explicit return marker after the
statement boundary if both are required. An ungrouped C formal may also require
one atomic token tree, such as a variable name or a parenthesized expression.
Read each generated macro's invocation documentation before adapting its call.

## Keep PostgreSQL calls on the backend thread

A generated macro can read backend globals or call PostgreSQL functions. Its
FFI adapters preserve pgrx's PostgreSQL error guard, but the caller must still
satisfy the original function's backend-thread, resource, pointer, and callback
requirements. For example, an interrupt checkpoint belongs in a backend call:

```rust
/// Check interrupts during a SQL call on the PostgreSQL backend thread.
///
/// # Safety
/// Rust callers must run in the initialized backend on its permitted thread,
/// with invariants valid if PostgreSQL raises ERROR.
#[pgrx::pg_extern]
unsafe fn interrupt_checkpoint() {
    // SAFETY: PostgreSQL invokes this function in the initialized backend.
    // No transient invariant needs repair if an interrupt raises ERROR.
    unsafe { pgrx::CHECK_FOR_INTERRUPTS!() }
}
```

Do not execute this example in a standalone Rust unit test or on a Rust worker
thread. Use pgrx's `#[pg_test]` harness for tests that need backend state. Keep
invariants valid before a call that can raise `ERROR`; do not rely on a destructor
restoring them. Rust callbacks invoked by PostgreSQL also need the appropriate
pgrx callback guard. See [FFI Error Handling](../ffi-error-handling.md).

## Find macros and inspect the selected build

From a pgrx source checkout, the library's CLI can list the PostgreSQL
function-like macros included by pgrx's version-specific wrapper header:

```sh
cargo run -p pgrx-c-macros -- list pg18
cargo run -p pgrx-c-macros -- list pg18 --name ACL_GRANT_OPTION_FOR
cargo run -p pgrx-c-macros -- analyze pg18 --name BUFFERALIGN --format json
```

The version is the first positional argument after the command, using the same
installation resolution as cargo-pgrx. `list` is a discovery inventory and can
contain repeated definitions. Analysis uses the final active definitions; a
candidate analysis is not yet proof that emission will succeed with your Rust
bindings. Standalone CLI emission also lacks the build's complete binding
catalog, so the normal extension build is the authority for available exports.

`pgrx-pg-sys` generates macros alongside its fresh bindgen bindings during a
normal build. The selected installation's C flags, target ABI, active definitions,
and available Rust bindings determine the result. Generation tracks those inputs
so a change causes the affected build outputs to be regenerated. It requires a
Clang executable compatible with the libclang used for inspection.

The current build's `pgrx-pg-sys` `OUT_DIR` contains a tree such as:

```text
out/
  pg18.rs
  pg18_macro_report.json
  cmacros/
    pg18/
      mod.rs
      c.rs
      utils/acl.rs
      access/transam.rs
      __pgrx_c_support/...
```

Header paths determine the macro files. Read the generated Rustdoc for the
original C definition, physical header and line span, operand names, and any
special invocation contract. Recognizable literals such as `0xFFFFFFFF`, verified
type aliases such as `AclMode`, and available binding symbols remain visible in
the translation. For example, `ACL_GRANT_OPTION_FOR!` documents the C expression
whose cast and hexadecimal mask it implements:

```c
#define ACL_GRANT_OPTION_FOR(privs) \
    (((AclMode) (privs) & 0xFFFFFFFF) << 32)
```

`/* PGRX: ... */` comments explain cases where expansion had to remain instead of
preserving a symbol or nested macro call.

The checked-in `pgrx-pg-sys/src/include/cmacros/` files are documentation snapshots,
made with the release-generation installation and target. Their source includes
target guards; they may describe a different platform from your computer. Use
your build's `OUT_DIR` bindings, macros, and report when
checking actual platform values or diagnosing an unavailable macro.

## Understand refusals and runtime limits

Generated macro support currently reports `RuntimeOnly`: these macros are not
promised to work in `const` initializers or enum discriminants. The runtime
uses compiler-verified LP64, LLP64, and ILP32 profiles, with either plain-char
signedness and byte order. This includes unsigned-char Linux AArch64 and
Windows. The compiler proves integer ranks, alignment, and relevant native
calling conventions separately; an unmodeled ABI option still causes a refusal.

Binding generation continues if macro inspection or native compilation is
unavailable. `PGRX_C_MACROS=0` explicitly disables generation. Macro availability
also varies with version and headers; use the public availability wrapper to
include optional items before Rust resolves missing macro names:

```rust
pgrx::if_c_macro! { CHECK_FOR_INTERRUPTS {
    /// Check interrupts on the PostgreSQL backend thread.
    ///
    /// # Safety
    /// The caller must be on the permitted backend thread.
    unsafe fn check_interrupts() {
        unsafe { pgrx::CHECK_FOR_INTERRUPTS!(); }
    }
}}
```

Unsupported grammar, unproved types or access rules, preprocessing constructs,
and exhausted bounds produce skips rather than guessed code. If bindgen and
Clang disagree on a referenced constant's value, generation skips the macro and
affected dependents. The build writes `pgNN_macro_report.json` even when macro
diagnostics are quiet. To enable macro-generation Cargo warnings, rebuild your
extension with exactly `PGRX_MACRO_DEBUG=1`:

```sh
PGRX_MACRO_DEBUG=1 cargo build
```

Only exactly `1` enables these warnings; unset or `PGRX_MACRO_DEBUG=0` keeps them
quiet. This setting controls normal build diagnostics; the CLI still prints the
reports you request. Inspect the report and enabled warnings for the exact
reason. A missing `NAME!` can also mean that the selected headers no longer
define `NAME` as a function-like macro.

At a call site, Rust's trait checks reject incompatible operands. Those checks
do not prove a raw pointer safe or validate PostgreSQL resource ownership. The
supported invocation rules exclude undefined C operations, later changes to the
inspected macro environment, and caller-local shadowing of bound C symbols.
Macros introducing C locals also reject argument tokens that could capture those
local names under C substitution.

For the generation pipeline, C semantic model, native adapters, and validation
strategy, see the crate's
[architecture reference](https://github.com/pgcentralfoundation/pgrx/blob/develop/pgrx-c-macros/ARCHITECTURE.md).
