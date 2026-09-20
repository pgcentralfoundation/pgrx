//! Set-returning functions under pgrust: materialize the whole iterator into
//! the executor's tuplestore through pgrust's `InitMaterializedSRF`, instead
//! of pgrx's value-per-call protocol (which needs `fn_extra` and
//! `FuncCallContext` in their C shapes).

use ::pgr_datum::Datum as NDatum;

use super::error::{raise, unwrap_pg};
use super::fmgr::{FunctionCallInfoBaseData, native_fcinfo, native_flinfo};
use crate::Datum;

/// Materialize `rows` (one `Option<Datum>` per output column) as the result
/// of the call `fcinfo` views. Returns the datum the wrapper should return.
///
/// # Safety
/// `fcinfo` must be a frame built by `call_v1` for a call whose
/// `resultinfo` is a `ReturnSetInfo` allowing materialize mode.
pub unsafe fn materialize(
    fcinfo: *mut FunctionCallInfoBaseData,
    rows: impl Iterator<Item = Vec<Option<Datum>>>,
) -> Datum {
    let Some(native) = native_fcinfo(fcinfo) else {
        raise(Box::new(::pgr_types_error::PgError::error(
            "pgrx: set-returning function called without a native frame",
        )));
    };
    let Some(flinfo) = native_flinfo(fcinfo) else {
        raise(Box::new(::pgr_types_error::PgError::error(
            "pgrx: set-returning function called without an FmgrInfo",
        )));
    };
    // SAFETY: the executor arms es_query_cxt as the result context for SRF
    // calls; it outlives this call.
    let mcx = native.result_mcx_detached();
    let mut srf = unwrap_pg(::pgr_funcapi::InitMaterializedSRF(
        mcx,
        flinfo,
        native,
        ::pgr_funcapi::MAT_SRF_USE_EXPECTED_DESC,
    ));
    let natts = srf.tupdesc.natts as usize;
    let mut values: Vec<NDatum> = Vec::with_capacity(natts);
    let mut nulls: Vec<bool> = Vec::with_capacity(natts);
    for row in rows {
        values.clear();
        nulls.clear();
        for col in row {
            match col {
                Some(d) => {
                    values.push(NDatum::from_usize(d.value()));
                    nulls.push(false);
                }
                None => {
                    values.push(NDatum::null());
                    nulls.push(true);
                }
            }
        }
        if values.len() != natts {
            raise(Box::new(::pgr_types_error::PgError::error(format!(
                "pgrx: set-returning function produced {} columns, expected {natts}",
                values.len()
            ))));
        }
        unwrap_pg(srf.putvalues(&values, &nulls));
    }
    let d = srf.finish(native);
    (*fcinfo).isnull = native.isnull;
    Datum::from(d.as_usize())
}
