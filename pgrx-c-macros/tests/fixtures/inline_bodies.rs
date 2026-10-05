//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

/// Count native guard entries without depending on a running PostgreSQL backend.
mod ffi {
    use std::sync::atomic::{AtomicUsize, Ordering};
    /// Count each native call that reaches the boundary.
    pub static CALLS: AtomicUsize = AtomicUsize::new(0);
    /// Execute only the fixture ABI call while counting the boundary.
    ///
    /// # Safety
    /// The closure must uphold its original C function's pointer and value requirements.
    pub unsafe fn pg_guard_ffi_boundary<T, F: FnOnce() -> T>(call: F) -> T {
        CALLS.fetch_add(1, Ordering::SeqCst);
        call()
    }
}

/// Print the same lines as the C oracle through the generated macros.
fn main() {
    let mut value = 0_i32;
    // SAFETY: The fixture functions write only through a pointer to the live, aligned local,
    // and BODY_PAIR initializes both fields of the record it returns.
    unsafe {
        println!("macro\t{}", BODY_MACRO!(3_i32).get());
        println!("dropped\t{}", BODY_DROPPED!(3_i32).get());
        println!("branch\t{}", BODY_BRANCH!(3_i32).get());
        BODY_STORE!(&raw mut value, 9_i32).get();
        BODY_STORE!(&raw mut value, -1_i32).get();
        println!("store\t{value}");
        BODY_EARLY!(&raw mut value, 11_i32).get();
        BODY_EARLY!(&raw mut value, -1_i32).get();
        println!("early\t{value}");
        println!("color\t{}\t{}", BODY_COLOR!(0_i32).get(), BODY_COLOR!(5_i32).get());
        // The native call returns the record the C function initialized.
        println!("pair\t{}", BODY_PAIR!(4_i32).get().assume_init().right);
        println!("caller\t{}", BODY_CALLER!(1_i32).get());
        println!("recursive\t{}", BODY_RECURSIVE!(3_i32).get());
        println!("late\t{}", BODY_LATE_USE!(1_i32).get());
        println!("redefined\t{}", BODY_REDEFINED_USE!(1_i32).get());
        println!("line\t{}", BODY_LINE!().get());
        println!("argument\t{}", BODY_MACRO_ARG!(13_i32).get());
    }
    assert_eq!(ffi::CALLS.load(std::sync::atomic::Ordering::SeqCst), EXPECTED_GUARDS);
}
