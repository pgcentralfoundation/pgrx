//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc.
//LICENSE Use of this source code is governed by the MIT license.
mod ffi {
    pub unsafe fn boundary<R, F: FnOnce() -> R>(call: F) -> R {
        call()
    }
}
fn main() {
    // SAFETY: The original native functions initialize exactly first and
    // inner.value. Other fields remain inside MaybeUninit and are never read.
    // Local aggregate storage is live, aligned, and exclusively mutated.
    unsafe {
        for i in (-100..100).step_by(7) {
            let value = PARTIAL_RECORD!(i).get();
            let other = PARTIAL_IDENTITY!(value).get();
            let mut assigned = core::mem::MaybeUninit::<PartialRecord>::uninit();
            let result = PARTIAL_ASSIGN!(assigned, other).get();
            let changed = PARTIAL_MUTATE!(assigned, i + 17).get();
            let first = PARTIAL_FIRST!(result).get();
            let inner = PARTIAL_INNER!(result).get();
            let taken = PARTIAL_TAKE!(PARTIAL_CHOOSE!(i & 1, result, assigned)).get();
            let mut unevaluated = i;
            let size = PARTIAL_SIZE!(PARTIAL_RECORD!({
                unevaluated += 1;
                unevaluated
            }))
            .get();
            assert_eq!(unevaluated, i, "sizeof must not evaluate the record call or argument");
            println!(
                "record {i} {first} {inner} {changed} {taken} {size} {}",
                PARTIAL_TEMP_INNER!(i).get()
            );
        }
    }
}
