//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

/// Observe native guard entry without depending on a running PostgreSQL backend.
mod ffi {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    /// Record whether native execution is inside its guard.
    pub static INSIDE: AtomicBool = AtomicBool::new(false);
    /// Count exactly one boundary per direct native root invocation.
    pub static CALLS: AtomicUsize = AtomicUsize::new(0);
    /// Execute only the fixture ABI call while observing the boundary.
    ///
    /// # Safety
    /// The closure must uphold its original C function's pointer and value requirements.
    pub unsafe fn pg_guard_ffi_boundary<T, F: FnOnce() -> T>(call: F) -> T {
        assert!(!INSIDE.swap(true, Ordering::SeqCst));
        CALLS.fetch_add(1, Ordering::SeqCst);
        let result = call();
        assert!(INSIDE.swap(false, Ordering::SeqCst));
        result
    }
}

/// Record native storage type independently of the emitted C marker names.
trait Kind {
    /// Tag compared with the original compiler's independent C type selection.
    const TAG: i32;
}
/// Observe native C int result storage.
impl Kind for i32 {
    /// Match the C oracle's int tag.
    const TAG: i32 = 1;
}
/// Observe native C bool result storage without normalizing it to C int.
impl Kind for bool {
    /// Match the C oracle's bool tag.
    const TAG: i32 = 2;
}
/// Observe native unsigned int result storage.
impl Kind for u32 {
    /// Match the C oracle's unsigned int tag.
    const TAG: i32 = 3;
}
/// Observe native unsigned long storage in the admitted LP64 oracle family.
impl Kind for u64 {
    /// Match the independent C size_t identity on the admitted Unix profile.
    const TAG: i32 = 4;
}
/// Select the observed storage identity after extraction from a C expression.
fn kind<T: Kind>(_: &T) -> i32 {
    T::TAG
}

/// Instrument argument conversion, which must happen outside the FFI boundary.
struct Input(
    /// Scalar payload whose conversion timing is independently observed.
    i32,
);
/// Register a fixture-only value with the same sealed input boundary as production.
impl __pgrx_c_macros::sealed::Sealed for Input {}
/// Assert native operand conversion occurs before entering the guard.
impl __pgrx_c_macros::expression::IntoExpression for Input {
    /// Keep the original C int identity throughout conversion.
    type Value = __pgrx_c_macros::CValue<__pgrx_c_macros::CInt>;
    /// Convert the payload while asserting that no native boundary is active.
    fn into_expression(self) -> Self::Value {
        assert!(!ffi::INSIDE.load(std::sync::atomic::Ordering::SeqCst));
        __pgrx_c_macros::CValue::new(self.0)
    }
}

/// Observe each operand evaluation without introducing unsequenced C mutations.
fn next_value(evaluations: &core::cell::Cell<u32>) -> u32 {
    assert!(!ffi::INSIDE.load(std::sync::atomic::Ordering::SeqCst));
    evaluations.set(evaluations.get() + 1);
    evaluations.get()
}

/// Use exactly the same invocation syntax for original macros and original inline functions.
fn main() {
    let mut value = 7_i32;
    let evaluations = core::cell::Cell::new(0_u32);
    // SAFETY: Fixture functions access only the initialized, aligned local int
    // through pointers valid for each call. Pointer results retain that local's
    // provenance. Enum results remain compatible integer storage, including 17.
    unsafe {
        println!("int\t{}", ROOT_INT!(Input(-2)).get());
        println!(
            "truth\t{}\t{}\t{}",
            u8::from(ROOT_PRED!(-2_i32).is_true()),
            u8::from(ROOT_PRED!(0_i32).is_true()),
            kind(&ROOT_PRED!(0_i32).get())
        );
        println!("word\t{}\t{}", ROOT_WORD!(7_u32).get(), kind(&ROOT_WORD!(7_u32).get()));
        println!(
            "pointer\t{}\t{}",
            u8::from(ROOT_POINTER!(&raw mut value).get() == (&raw mut value).cast()),
            u8::from(ROOT_CONST_POINTER!(&raw const value).get() == (&raw const value).cast())
        );
        println!("enum\t{}", ROOT_ENUM!(17_i32).get() as u64);
        let _: () = ROOT_VOID!(&raw mut value, 23_i32).get();
        println!("void\t{value}");
        let repeated = ROOT_REPEAT!(next_value(&evaluations)).get();
        println!("repeat\t{repeated}\t{}", evaluations.get());
        evaluations.set(0);
        let size = ROOT_SIZE!(next_value(&evaluations)).get();
        println!("size\t{size}\t{}", evaluations.get());
        evaluations.set(0);
        let lazy = ROOT_LAZY!(0_i32, next_value(&evaluations)).get();
        println!("lazy\t{lazy}\t{}", evaluations.get());
        println!(
            "capture\t{}\t{}",
            ROOT_CAPTURE!(31_i32).get(),
            __pgrx_inline_parameter_0!(37_i32).get()
        );
        println!("collision\t{}", ROOT_COLLISION!(1_i32).get());
        println!("zero\t{}", ROOT_ZERO!().get());
    }
    assert_eq!(ffi::CALLS.load(std::sync::atomic::Ordering::SeqCst), EXPECTED_GUARDS);
}
