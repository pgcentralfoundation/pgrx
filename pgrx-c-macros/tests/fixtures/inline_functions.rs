//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

/// Run native calls without a PostgreSQL backend.
mod ffi {
    /// Execute only the fixture ABI call.
    ///
    /// # Safety
    /// The closure must uphold its original C function's pointer and value requirements.
    pub unsafe fn pg_guard_ffi_boundary<T, F: FnOnce() -> T>(call: F) -> T {
        call()
    }
}

/// Times a generated function or global read checked its thread. Only functions that touch
/// a global check, and each read of the global checks.
static CHECKS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Count thread checks instead of comparing threads.
pub fn thread_check() {
    CHECKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

/// The bindings with the generated functions shadowing those of the same name.
mod wrapped {
    pub use super::*;
    crate::__pgrx_c_inline_functions!();
}

/// Print each generated function's result and the number of thread checks.
fn main() {
    let mut values = [0_i32; 2];
    // SAFETY: the functions add, multiply, write through a pointer to `values`, and update
    // the fixture's global on this thread.
    unsafe {
        println!("add\t{}", wrapped::WrapAdd(2, 3));
        let next: *mut i32 = wrapped::WrapNext(values.as_mut_ptr());
        println!("next\t{}", next.offset_from(values.as_ptr()));
        wrapped::WrapStore(next, 7);
        println!("store\t{}", values[1]);
        let doubled: u32 = wrapped::WrapKeyword(21);
        println!("keyword\t{doubled}");
        println!("bump\t{}", wrapped::WrapBump(2));
        println!("bump twice\t{}", wrapped::WrapBumpTwice(1));
    }
    println!("checks\t{}", CHECKS.load(std::sync::atomic::Ordering::Relaxed));
}
