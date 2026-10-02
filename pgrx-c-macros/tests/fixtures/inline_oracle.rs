//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

mod ffi {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    pub static INSIDE: AtomicBool = AtomicBool::new(false);
    pub static CALLS: AtomicUsize = AtomicUsize::new(0);
    pub unsafe fn pg_guard_ffi_boundary<T, F: FnOnce() -> T>(call: F) -> T {
        assert!(!INSIDE.swap(true, Ordering::SeqCst));
        CALLS.fetch_add(1, Ordering::SeqCst);
        let result = call();
        assert!(INSIDE.swap(false, Ordering::SeqCst));
        result
    }
}

struct Input(i32);
impl __pgrx_c_macros::sealed::Sealed for Input {}
impl __pgrx_c_macros::expression::IntoExpression for Input {
    type Value = __pgrx_c_macros::CValue<__pgrx_c_macros::CInt>;
    fn into_expression(self) -> Self::Value {
        assert!(!ffi::INSIDE.load(std::sync::atomic::Ordering::SeqCst));
        __pgrx_c_macros::CValue::new(self.0)
    }
}

fn unreachable_argument() -> u32 {
    panic!("lazy inline branch evaluated");
}

fn main() {
    // SAFETY: The original fixture functions only touch counters and live local
    // integers. All supplied pointers are aligned, initialized, and in bounds.
    unsafe {
        let mut value = 9_i32;
        inline_oracle_reset();
        println!("byte\t{}\t{}", INLINE_BYTE!(Input(-1)).get(), inline_oracle_count());
        println!("signed\t{}\t{}", INLINE_SIGNED!(12_i16).get(), inline_oracle_count());
        println!("bool\t{}\t{}", u8::from(INLINE_BOOL!(7_i32).get()), inline_oracle_count());
        println!(
            "double\t{:016x}\t{}",
            INLINE_DOUBLE!(1.5_f32).get().to_bits(),
            inline_oracle_count()
        );
        println!(
            "write\t{}\t{}",
            INLINE_WRITE!(&raw mut value, 65537_u32).get(),
            inline_oracle_count()
        );
        INLINE_VOID!(&raw mut value);
        println!("void\t{value}\t{}", inline_oracle_count());
        println!(
            "pointer\t{}\t{}",
            u8::from(INLINE_POINTER!(&raw mut value).get() == &raw mut value),
            inline_oracle_count()
        );
        println!(
            "const_pointer\t{}\t{}",
            u8::from(INLINE_CONST_POINTER!(&raw mut value).get() == &raw const value),
            inline_oracle_count()
        );
        println!(
            "void_pointer\t{}\t{}",
            u8::from(
                INLINE_VOID_POINTER!((&raw mut value).cast::<core::ffi::c_void>()).get()
                    == (&raw mut value).cast::<core::ffi::c_void>()
            ),
            inline_oracle_count()
        );
        println!("nested\t{}\t{}", INLINE_NESTED!(65537_u32).get(), inline_oracle_count());
        let mut evaluations = 0_u32;
        let repeated = INLINE_REPEAT!({
            evaluations += 1;
            evaluations
        });
        println!("repeat\t{}\t{evaluations}\t{}", repeated.get(), inline_oracle_count());
        println!(
            "lazy\t{}\t{}",
            INLINE_LAZY!(false, unreachable_argument()).get(),
            inline_oracle_count()
        );
        let address = &raw mut value;
        let returned = INLINE_WORD!(NativeWord(address.cast())).get();
        println!(
            "word\t{}\t{}",
            u8::from(returned == address.expose_provenance() as u64),
            inline_oracle_count()
        );
        let restored = core::ptr::with_exposed_provenance_mut::<i32>(returned as usize);
        assert_eq!(*restored, value);
        assert_eq!(ffi::CALLS.load(std::sync::atomic::Ordering::SeqCst), 13);
        assert!(!ffi::INSIDE.load(std::sync::atomic::Ordering::SeqCst));
        println!("existing\t{}\t{}", INLINE_EXISTING!(9_i32).get(), inline_oracle_count());
        // This fixture's existing binding calls C directly. Generated record
        // primitives each enter the boundary once.
        assert_eq!(ffi::CALLS.load(std::sync::atomic::Ordering::SeqCst), 13);
        let partial = INLINE_RECORD!(23).get();
        // SAFETY: C initializes member, but deliberately leaves validity
        // uninitialized. Project only member through the live raw storage.
        let member = core::ptr::addr_of!((*partial.as_ptr()).member).read();
        println!("partial_record\t{member}\t{}", inline_oracle_count());
        println!("partial_member\t{}\t{}", INLINE_RECORD_MEMBER!(31).get(), inline_oracle_count());
        assert_eq!(ffi::CALLS.load(std::sync::atomic::Ordering::SeqCst), 15);
    }
}
