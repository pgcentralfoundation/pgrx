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
