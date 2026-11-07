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

/// Print the same lines as the C oracle through the generated macros.
fn main() {
    // SAFETY: FALLBACK_RANK reads only its by-value enum argument.
    let rank = unsafe { FALLBACK_RANK!(FallbackShade_FALLBACK_DARK).get() };
    println!("rank\t{rank}");
    /* ADDRESS */ println!("address\t{}", i32::from(FALLBACK_ADDRESS!().get().is_some()));
}
