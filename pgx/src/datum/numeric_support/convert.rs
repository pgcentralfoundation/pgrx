use pgx_pg_sys::errcodes::PgSqlErrorCode;
use pgx_pg_sys::panic::CaughtError;
use pgx_pg_sys::PgTryBuilder;

use crate::numeric::make_typmod;
use crate::numeric_support::error::Error;
use crate::{
    direct_function_call, direct_function_call_as_datum, pg_sys, AnyNumeric, IntoDatum, Numeric,
};

pub use super::convert_anynumeric::*;
pub use super::convert_numeric::*;

pub(crate) fn from_primitive_helper<I: IntoDatum, const P: u32, const S: u32>(
    value: I,
    func: unsafe fn(pg_sys::FunctionCallInfo) -> pg_sys::Datum,
) -> Result<Numeric<P, S>, Error> {
    let datum = value.into_datum();
    let materialize_numeric_datum = move || unsafe {
        if func == pg_sys::numeric_in {
            debug_assert_eq!(I::type_oid(), pg_sys::CSTRINGOID);
            direct_function_call(
                pg_sys::numeric_in,
                vec![datum, pg_sys::InvalidOid.into_datum(), make_typmod(P, S).into_datum()],
            )
        } else if func == pg_sys::numeric {
            debug_assert_eq!(I::type_oid(), pg_sys::NUMERICOID);
            direct_function_call(pg_sys::numeric, vec![datum, make_typmod(P, S).into_datum()])
        } else {
            debug_assert!(
                func == pg_sys::float4_numeric
                    || func == pg_sys::float8_numeric
                    || func == pg_sys::int2_numeric
                    || func == pg_sys::int4_numeric
                    || func == pg_sys::int8_numeric
            );
            // use the user-provided `func` to make a Numeric from some primitive type
            let numeric_datum = direct_function_call_as_datum(func, vec![datum]);

            if P != 0 || S != 0 {
                // and if it has a precision or a scale, try to coerce it into those constraints
                direct_function_call(
                    pg_sys::numeric,
                    vec![numeric_datum, make_typmod(P, S).into_datum()],
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
