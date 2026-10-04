//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise generated prediction-hint macros through pgrx-pg-sys exports.
//!
//! Runtime checks ensure C result identity and operand effects survive the public
//! entry point. The standalone compiler witnesses separately establish branch
//! weights; these integration tests check usable exported behavior. Macros with
//! backend function or global dependencies belong in pgrx-unit-tests instead.

#![cfg(all(pgrx_c_macros, not(docsrs)))]
#![deny(unsafe_op_in_unsafe_fn)]

use core::cell::Cell;
use pgrx_pg_sys as pg;

/// Expose the semantic C rank for assertions that branch-hint truth results remain C long.
fn rank<K: pg::__pgrx_c_macros::CInteger>(_: pg::__pgrx_c_macros::CValue<K>) -> u8 {
    K::RANK
}

pg::__pgrx_c_classify! { @if_available likely {
pg::__pgrx_c_classify! { @if_available unlikely {
/// Checks that generated expectation wrappers preserve C long truth and single evaluation.
#[test]
fn generated_expectation_wrappers_preserve_c_long_truth_and_single_evaluation() {
    // The installed c.h passes (x != 0) to __builtin_expect, whose compiler
    // prototype returns long. The independent C oracle verifies that rank and
    // width even though the payload has only the values zero and one.
    for value in [-2147483647_i32,-1,0,1,17,i32::MAX] {
        let calls=Cell::new(0_u32);
        let likely=pg::likely!({calls.set(calls.get()+1);value});
        let unlikely=pg::unlikely!({calls.set(calls.get()+1);value});
        assert_eq!(likely.get(),i64::from(value!=0));
        assert_eq!(unlikely.get(),i64::from(value!=0));
        assert_eq!(rank(likely.into_value()),4);
        assert_eq!(rank(unlikely.into_value()),4);
        assert_eq!(core::mem::size_of_val(&likely.get()),core::mem::size_of::<core::ffi::c_long>());
        assert_eq!(calls.get(),2);
    }
    for value in [false,true] {
        assert_eq!(pg::likely!(value).get(),i64::from(value));
        assert_eq!(pg::unlikely!(value).get(),i64::from(value));
    }
    for value in [-3.75_f64,-0.0,0.0,0.75,f64::NAN,f64::INFINITY] {
        assert_eq!(pg::likely!(value).get(),i64::from(value!=0.0));
        assert_eq!(pg::unlikely!(value).get(),i64::from(value!=0.0));
    }
    let mut object=31_i32;
    for pointer in [core::ptr::null_mut(),&raw mut object] {
        assert_eq!(pg::likely!(pointer).get(),i64::from(!pointer.is_null()));
        assert_eq!(pg::unlikely!(pointer).get(),i64::from(!pointer.is_null()));
    }
}
} }
} }
