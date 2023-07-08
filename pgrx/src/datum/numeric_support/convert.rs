use pgrx_pg_sys::errcodes::PgSqlErrorCode;
use pgrx_pg_sys::panic::CaughtError;
use pgrx_pg_sys::PgTryBuilder;

use crate::numeric::make_typmod;
use crate::numeric_support::error::Error;
use crate::{
    direct_function_call, direct_function_call_as_datum, pg_sys, AnyNumeric, IntoDatum, Numeric,
};

pub use super::convert_anynumeric::*;
pub use super::convert_numeric::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum FromPrimitiveFunc {
    NumericIn,
    Numeric,
    Float4Numeric,
    Float8Numeric,
    Int2Numeric,
    Int4Numeric,
    Int8Numeric,
}

impl From<FromPrimitiveFunc> for unsafe fn(pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    fn from(value: FromPrimitiveFunc) -> Self {
        match value {
            FromPrimitiveFunc::NumericIn => pg_sys::numeric_in,
            FromPrimitiveFunc::Numeric => pg_sys::numeric,
            FromPrimitiveFunc::Float4Numeric => pg_sys::float4_numeric,
            FromPrimitiveFunc::Float8Numeric => pg_sys::float8_numeric,
            FromPrimitiveFunc::Int2Numeric => pg_sys::int2_numeric,
            FromPrimitiveFunc::Int4Numeric => pg_sys::int4_numeric,
            FromPrimitiveFunc::Int8Numeric => pg_sys::int8_numeric,
        }
    }
}

pub(crate) fn from_primitive_helper<I: IntoDatum, const P: u32, const S: u32>(
    value: I,
    func: FromPrimitiveFunc,
) -> Result<Numeric<P, S>, Error> {
    let datum = value.into_datum();
    let materialize_numeric_datum = move || unsafe {
        if func == FromPrimitiveFunc::NumericIn {
            debug_assert_eq!(I::type_oid(), pg_sys::CSTRINGOID);
            direct_function_call(
                pg_sys::numeric_in,
                &[datum, pg_sys::InvalidOid.into_datum(), make_typmod(P, S).into_datum()],
            )
        } else if func == FromPrimitiveFunc::Numeric {
            debug_assert_eq!(I::type_oid(), pg_sys::NUMERICOID);
            direct_function_call(pg_sys::numeric, &[datum, make_typmod(P, S).into_datum()])
        } else {
            debug_assert!(matches!(
                func,
                FromPrimitiveFunc::Float4Numeric
                    | FromPrimitiveFunc::Float8Numeric
                    | FromPrimitiveFunc::Int2Numeric
                    | FromPrimitiveFunc::Int4Numeric
                    | FromPrimitiveFunc::Int8Numeric
            ));
            // use the user-provided `func` to make a Numeric from some primitive type
            let numeric_datum = direct_function_call_as_datum(func.into(), &[datum]);

            if P != 0 || S != 0 {
                // and if it has a precision or a scale, try to coerce it into those constraints
                direct_function_call(
                    pg_sys::numeric,
                    &[numeric_datum, make_typmod(P, S).into_datum()],
                )
            } else {
                numeric_datum
            }
        }
        .unwrap_unchecked()
    };

    PgTryBuilder::new(|| {
        let datum = materialize_numeric_datum();
        // we asked Postgres to create this Numeric datum for us, so it'll need to be freed at some point
        Ok(Numeric(AnyNumeric { inner: datum.cast_mut_ptr(), need_pfree: true }))
    })
    .catch_when(PgSqlErrorCode::ERRCODE_INVALID_TEXT_REPRESENTATION, |e| {
        if let CaughtError::PostgresError(ref ereport) = e {
            Err(Error::Invalid(ereport.message().to_string()))
        } else {
            e.rethrow()
        }
    })
    .catch_when(PgSqlErrorCode::ERRCODE_NUMERIC_VALUE_OUT_OF_RANGE, |e| {
        if let CaughtError::PostgresError(ref ereport) = e {
            Err(Error::OutOfRange(ereport.message().to_string()))
        } else {
            e.rethrow()
        }
    })
    .catch_when(PgSqlErrorCode::ERRCODE_FEATURE_NOT_SUPPORTED, |e| {
        if let CaughtError::PostgresError(ref ereport) = e {
            Err(Error::ConversionNotSupported(ereport.message().to_string()))
        } else {
            e.rethrow()
        }
    })
    .execute()
}
