//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc.
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

/// Standalone Rust consumer for the callback oracle C comparison.
///
/// The harness appends this consumer after generated bindings, semantic support,
/// and macro definitions. Its observations preserve types and operand effects
/// for comparison with the original C header; helpers instrument those effects
/// without replacing any C macro definition.
///
/// Expose the fixture guard observer under the path emitted native adapters expect.
mod ffi {
    //! Expose the fixture guard observer under the path emitted native adapters expect.
    //!
    //! The enclosing selector or oracle owns this scope; generated paths must retain that
    //! ownership when expanded from a downstream consumer.

    use std::sync::atomic::{AtomicUsize, Ordering};
    /// Count fixture guard entries to compare callback boundary behavior with native
    /// observations.
    pub static GUARDS: AtomicUsize = AtomicUsize::new(0);
    /// Record calls crossing the fixture guard boundary while executing the supplied operation.
    pub unsafe fn boundary<R, F: FnOnce() -> R>(call: F) -> R {
        GUARDS.fetch_add(1, Ordering::Relaxed);
        call()
    }
}
/// Construct the fixture's registered enum identity for callback conversion observations.
fn nominal_enum<I: __pgrx_c_macros::expression::EnumIdentity, K: __pgrx_c_macros::CInteger>(
    value: __pgrx_c_macros::CValue<__pgrx_c_macros::expression::CEnum<I, K>>,
) -> K::Repr {
    value.get()
}
/// Exercise the generated definitions and print observations for the paired original-C oracle;
/// assertions cover cases with no scalar output.
fn main() {
    use __pgrx_c_macros::expression::*;
    use __pgrx_c_macros::{CLong, CLongLong, CValue};
    use std::sync::atomic::Ordering;
    // SAFETY: Every callback comes from the original live C table; pointers refer
    // to initialized local storage, and this fixture has no PostgreSQL backend.
    unsafe {
        let p = callback_table();
        callback_reset();
        for i in (-2..260).step_by(13) {
            let byte = CALLBACK_BYTE!(p, i).get();
            let lng: CValue<CLong> = CALLBACK_LONG!(p, i).into_value();
            let wide: CValue<CLongLong> = CALLBACK_WIDE!(p, i).into_value();
            let integer = CALLBACK_INT!(p, i).get();
            println!("values {i} {byte} {} {} {integer}", lng.get(), wide.get());
        }
        let mut slot = 17_i32;
        let read = CALLBACK_READ!(p, &raw const slot).get();
        let written = CALLBACK_WRITE!(p, &raw mut slot, 41).get();
        CALLBACK_CLEAR!(p, &raw mut slot);
        println!("place {read} {written} {slot}");
        let direct = CALLBACK_DIRECT!(CALLBACK_MEMBER!(p), 9).get();
        let factory = CALLBACK_FACTORY!(p, 1, 9).get();
        let global = CALLBACK_GLOBAL!(9).get();
        let noargs = CALLBACK_NOARGS!(p).get();
        let returned = CALLBACK_GET_CALL!(1, 9).get();
        println!("calls {direct} {factory} {global} {noargs} {returned}");
        let generic = CALLBACK_TWO!(CALLBACK_WRITE_MEMBER!(p), &raw mut slot, 7).get();
        println!("generic {generic} {slot}");
        let dereferenced = CALLBACK_DEREF!(CALLBACK_MEMBER!(p), 11).get();
        let addressed = CALLBACK_DIRECT!(CALLBACK_ADDRESS!(CALLBACK_MEMBER!(p)), 11).get();
        let original = CALLBACK_DIRECT!(CALLBACK_ORIGINAL_ADDRESS!(), 11).get();
        let designator = CALLBACK_DIRECT!(CALLBACK_ORIGINAL_VALUE!(), 11).get();
        println!(
            "designators {dereferenced} {addressed} {original} {designator} {}",
            i32::from(CALLBACK_ADDRESS!(CALLBACK_GET!(0)).get().is_none())
        );
        let full = CALLBACK_RECORD!(p, CallbackRecord { value: 13 }).get();
        let full_value = CALLBACK_RECORD_VALUE!(full).get();
        let again = CALLBACK_RECORD!(p, full).get();
        println!("record {full_value} {}", CALLBACK_RECORD_VALUE!(again).get());
        // The C callbacks initialize only value. Neither boolean nor enum bytes
        // may become a Rust value while passing this non-Copy record by value.
        let partial = CALLBACK_PARTIAL!(p, 17).get();
        let partial_value = CALLBACK_RECORD_VALUE!(partial).get();
        let copied = CALLBACK_PARTIAL_IDENTITY!(p, partial).get();
        let taken = CALLBACK_PARTIAL_TAKE!(p, copied).get();
        let member = CALLBACK_PARTIAL_MEMBER!(p, 17).get();
        println!("partial {partial_value} {taken} {member}");
        for value in [0_u32, 1, 2, 77, 0xFFFFFFFF] {
            let returned = nominal_enum(CALLBACK_STATE!(p, value).into_value());
            let cast = CALLBACK_STATE_VALUE!(p, value).get();
            println!("enum {value} {returned} {cast}");
        }
        let named = CALLBACK_STATE!(p, CallbackState::CallbackOne).get();
        println!("enum_named {named}");
        let even = CALLBACK_NEEDED!(CallbackOid(14)).get();
        let odd = CALLBACK_NEEDED!(15_u32).get();
        callback_set_predicate(0);
        let mut skipped = 0;
        let absent = CALLBACK_NEEDED!({
            skipped += 1;
            14_u32
        })
        .get();
        println!("predicate {even} {odd} {absent} {skipped}");
        let cancelled_void: Pointer<CVoid> =
            CALLBACK_ADDRESS!(::core::ptr::null_mut::<::core::ffi::c_void>()).into_value();
        println!("void_address {}", i32::from(cancelled_void.get().is_null()));
        let empty = CallbackTable {
            byte: None,
            lng: None,
            wide: None,
            integer: None,
            read: None,
            write: None,
            clear: None,
            factory: None,
            noargs: None,
            variadic: None,
            unprototyped: None,
            record: None,
            partial: None,
            partial_identity: None,
            partial_take: None,
            state: None,
        };
        let mut untouched = 0;
        let lazy = CALLBACK_LAZY!(&raw const empty, 0, {
            untouched += 1;
            untouched
        })
        .get();
        println!(
            "lazy {lazy} {untouched} {} {}",
            i32::from(CALLBACK_GET!(0).get().is_none()),
            callback_count()
        );
        let before = ffi::GUARDS.load(Ordering::Relaxed);
        assert!(std::panic::catch_unwind(|| CALLBACK_INT!(&raw const empty, 1)).is_err());
        assert_eq!(
            ffi::GUARDS.load(Ordering::Relaxed),
            before,
            "null rejection must precede the guard"
        );
        assert!(std::panic::catch_unwind(|| CALLBACK_BYTE!(p, 1.0e100_f64)).is_err());
        assert_eq!(
            ffi::GUARDS.load(Ordering::Relaxed),
            before,
            "conversion failure must precede the guard"
        );
        let value = CALLBACK_MEMBER!(p).into_value();
        assert!(truth(value));
    }
}
