//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise the documented input vocabulary and availability wrapper against
//! the selected installation's fresh output, including unavailable generation.
//! Missing macro bodies must disappear before Rust resolves their contents.

#![cfg(not(docsrs))]

use pgrx_pg_sys as pg;

pg::if_c_macro! { __PGRX_REVIEW_MACRO_THAT_DOES_NOT_EXIST {
    compile_error!("an absent macro body must not be resolved");
}}

pg::if_c_macro! { Min {
    /// A bindgen constant's Rust unsigned storage must be explicitly labeled as
    /// its original C int identity before generic arithmetic sees it as an operand.
    #[test]
    fn public_c_input_identity_preserves_signed_constant_comparison() {
        use pg::cmacros::c::{CInt, CValue};
        let block_size = CValue::<CInt>::new(i32::try_from(pg::BLCKSZ).unwrap());
        assert_eq!(pg::Min!(block_size, -1_i32).get(), -1);
    }
}}

// Bindgen spells C `char *`, including `Page` and C strings, as `*mut c_char`.
// Rust cannot tell that apart from `signed char *` or `unsigned char *`
// storage, so callers tag it with the public pointer marker. These functions
// call guarded native functions on PG16 and later, so they are type-checked
// here and never executed outside a backend.

pg::if_c_macro! { PageGetItem {
    /// Pass a binding `Page` to a plain-char pointer parameter of the static
    /// inline function in PG16 and later, while PG15 casts it inside a C macro.
    /// The result is `Item` before PG19 and `void *` from PG19 on.
    ///
    /// # Safety
    /// page must be a live, pinned, initialized page containing item_id, and the
    /// caller must run on the PostgreSQL backend thread.
    unsafe fn tagged_page_item(page: pg::Page, item_id: pg::ItemId) -> *mut core::ffi::c_void {
        use pg::cmacros::c::{CChar, Pointer};
        // SAFETY: The caller establishes the page, item and backend-thread contract.
        unsafe { pg::PageGetItem!(Pointer::<CChar>::new(page), item_id).get().cast() }
    }
}}

pg::if_c_macro! { CStringGetDatum {
    /// Pass a read-only binding C string to a `const char *` parameter.
    ///
    /// # Safety
    /// The caller must run on the PostgreSQL backend thread.
    #[allow(unused_unsafe, reason = "this conversion is pure where it is a macro or translated")]
    unsafe fn tagged_const_cstring(value: *const core::ffi::c_char) {
        use pg::cmacros::c::{CChar, Pointer, ReadOnly};
        // SAFETY: CStringGetDatum converts the address without reading it; the
        // caller supplies any guarded call's backend-thread requirement.
        let _datum = unsafe { pg::CStringGetDatum!(Pointer::<CChar, ReadOnly>::new(value)) };
    }
}}

pg::if_c_macro! { PG_GETARG_CSTRING {
pg::if_c_macro! { PG_RETURN_CSTRING {
    /// Return a C string argument after extracting it at the Rust boundary, as
    /// `PG_RETURN_CSTRING(PG_GETARG_CSTRING(0))` does in C.
    ///
    /// # Safety
    /// fcinfo must describe a live call whose first argument is a C string Datum,
    /// and the caller must run on the PostgreSQL backend thread.
    unsafe fn tagged_cstring_round_trip(fcinfo: pg::FunctionCallInfo) -> pg::Datum {
        use pg::cmacros::c::{CChar, Pointer};
        // SAFETY: The caller supplies a live fcinfo with one C string argument
        // and the backend-thread contract of both guarded conversions.
        unsafe {
            let value = pg::PG_GETARG_CSTRING!(0, fcinfo).get();
            pg::PG_RETURN_CSTRING!(Pointer::<CChar>::new(value));
        }
    }
}}
}}
