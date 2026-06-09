//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! # `PgMemoryContext` basics: switch / reset / drop
//!
//! Postgres allocates from whichever memory context is current. Pgrx surfaces
//! that via [`PgMemoryContexts`]: a transient child context can be created under
//! a parent, switched to, written to, then **reset** (memory reclaimed, context
//! reusable) or **dropped** (context destroyed).
//!
//! The pattern below shows the safe shape:
//!
//! 1. Create a child context under `CurrentMemoryContext`.
//! 2. `switch_to(|ctx| { ... })` — closure runs with the child as current.
//! 3. After the closure returns, `CurrentMemoryContext` is restored automatically.
//! 4. Calling `.reset()` reclaims everything allocated in the child without
//!    destroying the context.
//!
//! ## Anti-pattern (DO NOT do this)
//!
//! ```ignore
//! let mut child = PgMemoryContexts::new("scratch");
//! let p: *mut u8 = child.switch_to(|ctx| ctx.palloc(64));
//! child.reset();        // <-- p is now a dangling pointer
//! unsafe { *p = 0; }    // use-after-free
//! ```
//!
//! Anything you want to *survive* a reset must live in the *parent* context.
//! That's the whole point of switching back before returning a value.

use pgrx::PgMemoryContexts;
use pgrx::prelude::*;

/// Sums an `Array<i32>` using a temporary scratch context that is reset after
/// the operation. The return value lives in the caller's context.
#[pg_extern]
fn sum_with_scratch(arr: Array<i32>) -> i64 {
    let mut scratch = PgMemoryContexts::new("sum_with_scratch");

    // The closure runs with `scratch` as CurrentMemoryContext. Any palloc done inside ends up in `scratch` and gets reclaimed on `reset`.
    //
    // SAFETY: the closure does not leak any Postgres-allocated pointers out of the scratch context; `acc` is a stack-resident `i64`.
    let total: i64 = unsafe {
        scratch.switch_to(|_ctx| {
            let mut acc: i64 = 0;
            for v in arr.iter().flatten() {
                acc += v as i64;
            }
            acc
        })
    };

    // Reclaim everything allocated inside the closure. The `total` returned above is on the stack, so this is safe.
    //
    // SAFETY: no live pointers into `scratch` exist past this point.
    unsafe { scratch.reset() };

    total
}

/// Demonstrates that a value moved *out* of the closure is fine — primitives go on the stack — but a Postgres-allocated payload would NOT survive `.reset()`.
#[pg_extern]
fn scratch_count(n: i32) -> i32 {
    let mut scratch = PgMemoryContexts::new("scratch_count");
    // SAFETY: the closure returns a primitive `i32`; no Postgres-allocated pointers escape `scratch`.
    let c = unsafe {
        scratch.switch_to(|_ctx| {
            let v: Vec<i32> = (0..n).collect(); // Rust heap, not Postgres heap
            v.len() as i32
        })
    };
    // SAFETY: no live pointers into `scratch` exist past this point.
    unsafe { scratch.reset() };
    c
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn basics_sum() {
        let total = Spi::get_one::<i64>("SELECT sum_with_scratch(ARRAY[1,2,3,4,5,6])")
            .expect("query failed");
        assert_eq!(total, Some(21));
    }

    #[pg_test]
    fn basics_count() {
        let c = Spi::get_one::<i32>("SELECT scratch_count(100)").expect("query failed");
        assert_eq!(c, Some(100));
    }
}
