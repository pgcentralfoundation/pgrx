//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#![cfg(all(pgrx_c_macros, not(docsrs)))]
#![deny(unsafe_op_in_unsafe_fn)]

use core::mem::MaybeUninit;
use core::ptr::{addr_of, addr_of_mut};
use pgrx_pg_sys as pg;

fn return_datum(value: pg::Datum) -> pg::Datum {
    pg::PG_RETURN_DATUM!(value);
}

fn return_void() -> pg::Datum {
    pg::PG_RETURN_VOID!();
}

/// # Safety
/// `fcinfo` must point to aligned, live storage whose isnull field is exclusively
/// writable. Other fields need not be initialized because this macro never reads them.
unsafe fn return_null(fcinfo: pg::FunctionCallInfo) -> pg::Datum {
    // SAFETY: The caller supplies the live, aligned, exclusively writable field.
    unsafe { pg::PG_RETURN_NULL!(fcinfo) }
}

/// # Safety
/// `context.call_cntr` must be initialized and exclusively writable. `fcinfo.resultinfo`
/// must point to aligned, live ReturnSetInfo storage with an exclusively writable
/// isDone field. No other fields are read by SRF_RETURN_NEXT.
unsafe fn return_next(
    context: *mut pg::FuncCallContext,
    value: pg::Datum,
    fcinfo: pg::FunctionCallInfo,
) -> pg::Datum {
    // SAFETY: The caller establishes the initialized counter, result pointer and
    // writable state field; this macro has no backend function call.
    unsafe { pg::SRF_RETURN_NEXT!(context, value, fcinfo) }
}

/// # Safety
/// The same counter/resultinfo obligations as return_next apply. Additionally,
/// fcinfo.isnull must be exclusively writable.
unsafe fn return_next_null(
    context: *mut pg::FuncCallContext,
    fcinfo: pg::FunctionCallInfo,
) -> pg::Datum {
    // SAFETY: The caller establishes every accessed field and allocation. The
    // macro writes isnull and does not call backend functions.
    unsafe { pg::SRF_RETURN_NEXT_NULL!(context, fcinfo) }
}

// These actual generated macros call pgrx-guarded native functions. Type-check
// their public expansions here; executing them requires the backend test harness.

/// # Safety
/// Must run on the backend thread under the generated native-call contract.
unsafe fn return_int32(value: i32) -> pg::Datum {
    // SAFETY: The caller meets the generated guard's backend-thread obligation;
    // Int32GetDatum accepts every int32 and only converts its value.
    unsafe { pg::PG_RETURN_INT32!(value) }
}

/// # Safety
/// Must run on the backend thread under the generated native-call contract.
unsafe fn return_bool(value: bool) -> pg::Datum {
    // SAFETY: BoolGetDatum accepts both bool values; the caller supplies the thread.
    unsafe { pg::PG_RETURN_BOOL!(value) }
}

/// # Safety
/// Must run on the backend thread. Any subsequent pointer use must separately
/// establish the pointee's allocation, lifetime, initialization and aliasing.
unsafe fn return_pointer(value: *mut i32) -> pg::Datum {
    // SAFETY: PointerGetDatum converts this pointer without dereferencing it;
    // the caller establishes the guard's backend-thread requirement.
    unsafe { pg::PG_RETURN_POINTER!(value) }
}

/// # Safety
/// Must run on the backend thread under the generated native-call contract.
unsafe fn return_float4(value: f32) -> pg::Datum {
    // SAFETY: Float4GetDatum accepts every float4 representation; the caller
    // establishes the native guard's backend-thread requirement.
    unsafe { pg::PG_RETURN_FLOAT4!(value) }
}

/// # Safety
/// Must run on the backend thread with a fully initialized, valid active SRF
/// call context and fcinfo, as required by end_MultiFuncCall and its guards.
unsafe fn return_done(
    context: *mut pg::FuncCallContext,
    fcinfo: pg::FunctionCallInfo,
) -> pg::Datum {
    // SAFETY: The caller establishes the backend/SRF resource contract. This
    // wrapper is type-checked but never executed by these backend-free tests.
    unsafe { pg::SRF_RETURN_DONE!(context, fcinfo) }
}

#[test]
fn actual_generated_datum_returns_preserve_values_and_pointer_provenance() {
    for bits in [0_usize, 1, 0xFFFFFFFF, usize::MAX] {
        assert_eq!(return_datum(pg::Datum::from(bits)).value(), bits);
    }
    assert_eq!(return_void().value(), 0);
    let mut value = 37_i32;
    let pointer = addr_of_mut!(value);
    let returned = return_datum(pg::Datum::from(pointer)).cast_mut_ptr::<i32>();
    assert_eq!(returned, pointer);
    // SAFETY: The returned pointer retains the provenance of the live initialized
    // local, is aligned and in bounds, and no references alias this write or read.
    unsafe {
        returned.write(91);
        assert_eq!(returned.read(), 91);
    }
    assert_eq!(value, 91);
}

#[test]
fn actual_generated_null_and_srf_returns_access_only_the_required_fields() {
    let mut call_storage = MaybeUninit::<pg::FunctionCallInfoBaseData>::uninit();
    let mut result_storage = MaybeUninit::<pg::ReturnSetInfo>::uninit();
    let mut context_storage = MaybeUninit::<pg::FuncCallContext>::uninit();
    let fcinfo = addr_of_mut!(call_storage).cast::<pg::FunctionCallInfoBaseData>();
    let result_info = addr_of_mut!(result_storage).cast::<pg::ReturnSetInfo>();
    let context = addr_of_mut!(context_storage).cast::<pg::FuncCallContext>();
    // SAFETY: Each pointer addresses one suitably aligned live MaybeUninit slot
    // with the corresponding C layout. Only the selected scalar/pointer fields
    // are initialized, read, or written. No whole record is materialized or
    // borrowed, and no references or other threads alias these allocations.
    // The tested macros perform no backend call or resource-management operation.
    unsafe {
        addr_of_mut!((*fcinfo).isnull).write(false);
        assert_eq!(return_null(fcinfo).value(), 0);
        assert!(addr_of!((*fcinfo).isnull).read());

        addr_of_mut!((*fcinfo).isnull).write(false);
        addr_of_mut!((*fcinfo).resultinfo).write(result_info.cast::<pg::Node>());
        addr_of_mut!((*result_info).isDone).write(pg::ExprDoneCond::ExprSingleResult);
        addr_of_mut!((*context).call_cntr).write(0);
        let value = return_next(context, pg::Datum::from(17_usize), fcinfo);
        assert_eq!(value.value(), 17);
        assert_eq!(addr_of!((*context).call_cntr).read(), 1);
        assert_eq!(addr_of!((*result_info).isDone).read(), pg::ExprDoneCond::ExprMultipleResult);
        assert!(!addr_of!((*fcinfo).isnull).read());

        assert_eq!(return_next_null(context, fcinfo).value(), 0);
        assert_eq!(addr_of!((*context).call_cntr).read(), 2);
        assert_eq!(addr_of!((*result_info).isDone).read(), pg::ExprDoneCond::ExprMultipleResult);
        assert!(addr_of!((*fcinfo).isnull).read());

        addr_of_mut!((*context).call_cntr).write(u64::MAX);
        addr_of_mut!((*fcinfo).isnull).write(false);
        assert_eq!(return_next(context, pg::Datum::from(23_usize), fcinfo).value(), 23);
        assert_eq!(addr_of!((*context).call_cntr).read(), 0);
        assert!(!addr_of!((*fcinfo).isnull).read());
    }
}
