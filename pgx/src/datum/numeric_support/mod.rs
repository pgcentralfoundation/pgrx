use crate::{direct_function_call_as_datum, pg_sys, AnyNumeric};

pub mod cmp;
pub mod convert;
pub(super) mod convert_anynumeric;
pub(super) mod convert_numeric;
pub(super) mod convert_primitive;
pub mod datum;
pub mod error;
pub mod hash;
pub mod ops;
pub mod serde;
pub mod sql;

#[inline]
pub(super) fn call_numeric_func(
    func: unsafe fn(pg_sys::FunctionCallInfo) -> pg_sys::Datum,
    args: Vec<Option<pg_sys::Datum>>,
) -> AnyNumeric {
    unsafe {
        // SAFETY: this call to direct_function_call_as_datum will never return None
        let numeric_datum = direct_function_call_as_datum(func, args).unwrap_unchecked();
        // we asked Postgres to create this numeric so it'll need to be freed eventually
        AnyNumeric { inner: numeric_datum.cast_mut_ptr(), need_pfree: true }
    }
}
