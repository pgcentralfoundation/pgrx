use std::marker::PhantomData;
//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx::prelude::*;

#[pg_extern]
fn extern_func() -> bool {
    extern_func_impl::<u8>()
}

// This ensures that parameterized function compiles when it has `pg_guard` attached to it
#[pg_guard]
// Uncommenting the line below will make it fail to compile
// #[unsafe(no_mangle)]
extern "C-unwind" fn extern_func_impl<T>() -> bool {
    let _ = PhantomData::<T>;
    true
}

// This ensures that non-parameterized function compiles when it has `pg_guard` attached to it
// and [no_mangle]
#[pg_guard]
#[unsafe(no_mangle)]
extern "C-unwind" fn extern_func_impl_1() -> bool {
    true
}

// This ensures that lifetime-parameterized function compiles when it has `pg_guard` attached to it
// and [no_mangle]
#[pg_guard]
#[unsafe(no_mangle)]
#[allow(unused_lifetimes, clippy::extra_unused_lifetimes)]
extern "C-unwind" fn extern_func_impl_2<'a>() -> bool {
    let _ = PhantomData::<&'a ()>;
    true
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use pgrx::prelude::*;

    #[pg_test]
    fn check_for_interrupts_returns_without_a_pending_interrupt() {
        check_for_interrupts();
    }

    #[pg_test]
    fn check_for_interrupts_panics_off_the_backend_thread() {
        // Record this thread as the backend thread before another one asks.
        check_for_interrupts();
        let result = std::thread::spawn(check_for_interrupts).join();
        assert!(
            result.is_err(),
            "check_for_interrupts must refuse a thread other than the backend's"
        );
    }
}
