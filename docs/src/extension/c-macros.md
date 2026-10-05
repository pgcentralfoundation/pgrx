# Using PostgreSQL C Macros

pgrx generates Rust `macro_rules!` versions of supported PostgreSQL function-like
C macros and adapters for supported PostgreSQL static inline functions. You can
call both directly as `pgrx::NAME!(...)` in an extension.
They follow the selected headers' C arithmetic, casts, evaluation, and storage
rules, including where those rules differ from Rust's native operators.

Start with an extension configured for your PostgreSQL version. The main examples
below use PostgreSQL 18. Unless an example defines a function, place its statements inside
one of your Rust functions or tests.

## Migrate handwritten helper calls

To migrate a handwritten helper call, use the generated macro with its original
PostgreSQL name and extract the result at the Rust boundary:

| Previous call | Generated call |
| --- | --- |
| `pg_sys::TransactionIdIsNormal(xid)` | `pgrx::TransactionIdIsNormal!(xid).get() != 0` |
| `pgrx::varsize_any(ptr)` | `pgrx::VARSIZE_ANY!(ptr).get() as usize` |
| `pgrx::vardata_any(ptr)` | `pgrx::VARDATA_ANY!(ptr).get()` |
| `pgrx::set_varsize(ptr, len)` | `pgrx::SET_VARSIZE!(ptr, len)` |
| `check_for_interrupts!()` | `pgrx::CHECK_FOR_INTERRUPTS!()` |

Keep the caller's length checks, resource lifetimes, backend-thread restrictions,
and unsafe blocks. For example, `VARSIZE_ANY!` reads raw storage, and
`CHECK_FOR_INTERRUPTS!` accesses backend state and can raise a PostgreSQL error.
Statement macros yield `()`, so they do not need `.get()`. Return macros return
from the enclosing Rust function or closure; replace the complete return site.

When replacing a `pgrx::is_a` call, preserve its NULL short-circuit before
loading a tag with `nodeTag!`; a generated C field access requires valid storage.
Application-level ownership and error adapters retain their Rust contracts.

PostgreSQL sometimes changes a macro into a static inline function. The
generator keeps the `NAME!(...)` interface available in both cases, so callers
do not need version checks or an extra pointer cast merely because the C name
became a function. For example, `VARDATA_ANY!(ptr).get()` works across PG15–19.
When a value-returning function's body uses no C macros and only constructs the
generator supports, the generated macro translates that body to Rust, as it
would a macro's. Otherwise it calls the original C function through the checked
native binding. Native wrappers are built even when the public `cshim` feature
is disabled.

The selected header still determines argument and result types. A size result
can change from C `int` to `size_t`; `.get() as usize` is an explicit Rust size
conversion in either case. For predicates that change from C `int` to `bool`,
use `.is_true()` to obtain a Rust condition with C's zero/null semantics. A
function adapter evaluates each argument once, while a C macro keeps its
original argument repetition. Generated docs distinguish the two and show the
original definition. Genuine signature changes can still require caller changes.

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
and compare it with zero, or use `.is_true()` when you need a Rust `bool`:

```rust
let is_normal = pgrx::TransactionIdIsNormal!(xid).is_true();
```

Expression macros return an already evaluated C expression wrapper. `.get()`
extracts its native Rust storage; `.into_value()` keeps the tagged C value:

```rust
use pgrx::{cmacros::c::CValue, pg_sys};

let rounded = pgrx::BUFFERALIGN!(33_u32);
let bytes = rounded.get();
assert!(bytes >= 33);

let tagged: CValue<pg_sys::__pgrx_c_types::uintptr_t> = rounded.into_value();
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

Raw `i64`, `u64`, and `isize` are ambiguous. `usize` represents C `size_t`, whose
rank is verified for the selected target. On LP64 it is usually `unsigned long`.
C `unsigned long` and
`unsigned long long` can both occupy a Rust `u64` while having different ranks in
C's arithmetic conversions. A Rust type alias does not recover that distinction.
For a C typedef, use its generated identity from `pg_sys::__pgrx_c_types` rather
than assuming that one primitive width always corresponds to `long`:

```rust
use pgrx::{cmacros::c::CValue, pg_sys};

let input_len = 1024_usize;
let capacity = pgrx::PGLZ_MAX_OUTPUT!(
    CValue::<pg_sys::__pgrx_c_types::size_t>::new(input_len as _)
).get() as usize;
assert_eq!(capacity, 1028);
```

This supplies C `size_t` identity, including its rank on the selected target.
The macro follows unsigned C overflow rules; it does not saturate. Validate
application limits before using its result as an allocation size.

Plain C `char` also needs an explicit `CValue<CChar>` tag. The inspected compiler
selects its signedness and promotion, independently of Rust's native
`core::ffi::c_char` alias. Generated bridges verify compatible one-byte storage.
Raw `i8` and `u8` inputs retain the distinct C `signed char` and `unsigned char`
identities even when one shares plain `char`'s storage.

For example, this fails to compile:

```rust,compile_fail
let size = 33_u64;
let _ = pgrx::BUFFERALIGN!(size);
```

Use a C marker when you know the intended original C type:

```rust
use pgrx::cmacros::c::{CUnsignedLong, CUnsignedLongLong, CValue};

let size = CValue::<CUnsignedLong>::new(33 as _);
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

If you already have a `Datum`, pass it as a `Datum`. A known pass-by-value int32
Datum can be decoded with the same invocation on PG15–19:

```rust
use pgrx::pg_sys;

/// Decode a Datum known by the caller to contain a pass-by-value int32.
///
/// # Safety
/// Call on the PostgreSQL backend thread with a valid int4 Datum.
#[allow(unused_unsafe, reason = "this conversion is pure wherever it is translated")]
unsafe fn read_pass_by_value_int32(datum: pg_sys::Datum) -> i32 {
    // SAFETY: The caller supplies a Datum containing an initialized int32;
    // execute this within the PostgreSQL backend thread.
    unsafe { pgrx::DatumGetInt32!(datum).get() }
}
```

The caller must know what the Datum contains. A Datum that carries a pointer is
not an int32 just because its address can be converted to integer bits.
For versions that define `DatumGetInt32` as a static inline function, the
generator translates its body, so the conversion stays pure Rust. The ordinary
function binding is also available when you want its exact native Rust signature.
An inline function the generator calls natively needs the unsafe block, so the
narrow lint allowance keeps this shared call quiet where it is pure.

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

Stringified dependency diagnostics, such as an assertion inside another macro,
combine the original C diagnostic template with `stringify!` spelling of the
tokens supplied to the outer Rust invocation. The generator retains those
tokens before forwarding expression fragments. Rust suffixes, paths and casts
remain Rust spelling; this contract does not recover the corresponding C
operand text. Invocation-sensitive `__FILE__` and `__LINE__` refer to the Rust
call site. Unsupported uses that require exact C preprocessing spelling or an
unproved array extent still produce a diagnostic instead of guessed code.

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

Supply a callback's C typedef identity explicitly when a Rust function-pointer
alias could correspond to several C types. For PG16 and later, a raw parse-tree
walker uses the generated `tree_walker_callback` wrapper:

```rust
use core::ffi::c_void;
use pgrx::pg_sys;

/// Visit a raw parse-tree child without reading it or the unused context.
#[pgrx::pg_guard]
extern "C-unwind" fn visit_child(_node: *mut pg_sys::Node, _context: *mut c_void) -> bool {
    false
}

/// Walk a raw parse tree during a PostgreSQL backend call.
///
/// # Safety
/// tree must be a live initialized raw parse tree accepted by PostgreSQL, with
/// valid child-node storage for the call. The caller runs on the backend thread.
unsafe fn visit_children(tree: *mut pg_sys::Node) -> bool {
    let callback: pg_sys::tree_walker_callback = Some(visit_child);
    // SAFETY: the caller provides the live tree and backend-thread contract.
    // The guarded callback ignores its context, so a NULL context is valid.
    unsafe {
        pgrx::raw_expression_tree_walker!(
            tree,
            pg_sys::__pgrx_c_callbacks::tree_walker_callback::new(callback),
            core::ptr::null_mut::<c_void>()
        ).get()
    }
}
```

The wrapper checks the exact generated callback storage and supplies its C
identity without changing the ABI. It does not establish the callback's pointer,
thread, or PostgreSQL error-handling obligations. PG15 exposes the older native
walker declaration instead of this macro; use that version's function binding.

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

The checked-in `pgrx-pg-sys/src/include/cmacros/snapshot/` files are a documentation
snapshot made with the release-generation installations and target. They hold each
supported PostgreSQL version's macro documentation, gated by the `pgNN` features, with
arms that only show the accepted invocations. docs.rs uses them because it cannot
generate macros; ordinary builds never do. A definition that changed between versions
appears once per version range, next to its other versions. Use your build's `OUT_DIR`
bindings, macros, and report when checking actual platform values or diagnosing an
unavailable macro.

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

Direct generation needs a GNU-style Clang executable compatible with the loaded
`libclang`; the same inspected profile compiles native accessors and inline
wrappers. Windows/MSVC additionally needs LLVM's `llvm-lib` or `llvm-ar --format=coff` and the target's
headers and libraries. Unix uses an adjacent `llvm-ar` or a compatible local
`ar`.

The CLI and optional binding-build inspection decode recorded CFLAGS before
CPPFLAGS; ordinary bindgen retains its established CPPFLAGS invocation. GNU
flags retain shell quoting; MSVC flags preserve Windows quoting and
backslashes and translate only known options. Unknown semantic options fail.
Stock MSVC `pg_config` may report `not recorded`; that exact value contributes
no flags. The inspected profile then describes the installed headers and
explicit binding arguments, without recovering absent server build settings.

Cross builds can import a
[complete target bundle](build/cross-compile.md#import-a-complete-target-bundle)
through `PGRX_TARGET_INFO_PATH_PGNN`. Raw bindings alone and checked-in
documentation snapshots cannot substitute for the target's inspected macro and
native support.

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
