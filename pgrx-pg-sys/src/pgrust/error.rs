//! Errors across the pgrust boundary.
//!
//! pgrx already models a Postgres error as a Rust panic whose payload is
//! caught at the function wrapper (`CaughtError`). Under pgrust the backend
//! reports errors as `PgResult` values instead of longjmp, so the two
//! directions are:
//!
//! * pgrust → pgrx: a shim that receives `Err(PgError)` calls [`raise`], which
//!   panics with a [`PgrustError`] payload. `downcast_panic_payload` turns
//!   that into `CaughtError::PostgresError` carrying the original error, so
//!   `PgTryBuilder` handlers see the right SQLSTATE.
//! * pgrx → pgrust: the wrapper (`call_v1`) catches any panic and converts
//!   the payload back into a `PgError` with [`to_pg_error`].

use std::panic::panic_any;

use ::pgr_types_error::{ErrorLevel, PgError, PgResult, SqlState};

use crate::elog::PgLogLevel;
use crate::errcodes::PgSqlErrorCode;
use crate::panic::{CaughtError, ErrorReport, ErrorReportLocation, ErrorReportWithLevel};

/// The panic payload a pgrust-backed shim raises for `Err(PgError)`.
pub struct PgrustError(pub Box<PgError>);

/// Raise a pgrust error through pgrx's panic-based propagation.
#[cold]
#[inline(never)]
pub fn raise(err: Box<PgError>) -> ! {
    panic_any(PgrustError(err))
}

/// `?` for shims: unwrap a `PgResult` or raise its error.
#[inline]
pub fn unwrap_pg<T>(r: PgResult<T>) -> T {
    match r {
        Ok(v) => v,
        Err(e) => raise(e),
    }
}

pub fn level_to_pg(level: PgLogLevel) -> ErrorLevel {
    ErrorLevel(level as i32)
}

pub fn level_from_pg(level: ErrorLevel) -> PgLogLevel {
    // PgLogLevel's discriminants are the C elevel values, as are pgrust's.
    match level.0 {
        10 => PgLogLevel::DEBUG5,
        11 => PgLogLevel::DEBUG4,
        12 => PgLogLevel::DEBUG3,
        13 => PgLogLevel::DEBUG2,
        14 => PgLogLevel::DEBUG1,
        15 => PgLogLevel::LOG,
        16 => PgLogLevel::LOG_SERVER_ONLY,
        17 => PgLogLevel::INFO,
        18 => PgLogLevel::NOTICE,
        19 | 20 => PgLogLevel::WARNING,
        21 => PgLogLevel::ERROR,
        22 => PgLogLevel::FATAL,
        _ => PgLogLevel::PANIC,
    }
}

/// A pgrust error as pgrx's report type, keeping the original alongside.
pub fn report_from_pg_error(err: Box<PgError>) -> ErrorReportWithLevel {
    let sqlerrcode = PgSqlErrorCode::from(err.sqlstate().0);
    let inner = ErrorReport {
        sqlerrcode,
        message: std::borrow::Cow::Owned(err.message().to_owned()),
        hint: err.hint().map(str::to_owned),
        detail: err.detail().map(str::to_owned),
        domain: err.domain().map(str::to_owned),
        location: ErrorReportLocation::default(),
        pgrust: Some(err.clone()),
    };
    ErrorReportWithLevel {
        level: level_from_pg(err.level()),
        inner,
    }
}

/// pgrx's report type as a pgrust error. The original error is returned
/// untouched when the report came from pgrust in the first place.
pub fn to_pg_error(report: ErrorReportWithLevel) -> Box<PgError> {
    if let Some(orig) = report.inner.pgrust {
        return orig;
    }
    let level = level_to_pg(report.level);
    let mut err = PgError::new(level, report.inner.message.into_owned())
        .with_sqlstate(SqlState(report.inner.sqlerrcode as i32));
    if let Some(d) = report.inner.detail {
        err = err.with_detail(d);
    }
    if let Some(h) = report.inner.hint {
        err = err.with_hint(h);
    }
    if let Some(d) = report.inner.domain {
        err = err.with_domain(d);
    }
    if !report.inner.location.file.is_empty() {
        err = err.with_location(
            report.inner.location.file.clone(),
            report.inner.location.line as i32,
            report.inner.location.funcname.clone().unwrap_or_default(),
        );
    }
    Box::new(err)
}

/// A caught panic as a pgrust error (the wrapper's Err).
pub fn caught_to_pg_error(caught: CaughtError) -> Box<PgError> {
    match caught {
        CaughtError::PostgresError(r) | CaughtError::ErrorReport(r) => to_pg_error(r),
        CaughtError::RustPanic { ereport, .. } => to_pg_error(ereport),
    }
}

/// The pgrust arm of pgrx's `do_ereport`: report through pgrust's elog. For
/// ERROR and above this never returns.
pub fn do_ereport(report: ErrorReportWithLevel) {
    let level = report.level;
    let err = to_pg_error(report);
    match ::pgr_elog::ThrowErrorData(*err) {
        Ok(()) => {
            if level >= PgLogLevel::ERROR {
                // ThrowErrorData only returns Ok for suppressed reports, which
                // an ERROR never is; keep the contract visible.
                raise(Box::new(PgError::error("pgrx: error report did not raise")));
            }
        }
        Err(e) => raise(e),
    }
}
