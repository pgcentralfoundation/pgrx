/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

use std::any::Any;
use std::cell::Cell;
use std::fmt::{Display, Formatter};
use std::hint::unreachable_unchecked;
use std::panic::{
    catch_unwind, panic_any, resume_unwind, Location, PanicInfo, RefUnwindSafe, UnwindSafe,
};

use crate::elog::PgLogLevel;
use crate::errcodes::PgSqlErrorCode;
use crate::{pfree, AsPgCStr, MemoryContextSwitchTo};

#[derive(Clone, Debug)]
pub struct ErrorReportLocation {
    file: String,
    line: u32,
    col: u32,
}

impl Default for ErrorReportLocation {
    fn default() -> Self {
        Self { file: std::string::String::from("<unknown>"), line: 0, col: 0 }
    }
}

impl Display for ErrorReportLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.col)
    }
}

impl From<&Location<'_>> for ErrorReportLocation {
    fn from(location: &Location<'_>) -> Self {
        Self { file: location.file().to_string(), line: location.line(), col: location.column() }
    }
}

impl From<Option<&Location<'_>>> for ErrorReportLocation {
    fn from(location: Option<&Location<'_>>) -> Self {
        location.into()
    }
}

impl From<&PanicInfo<'_>> for ErrorReportLocation {
    fn from(pi: &PanicInfo<'_>) -> Self {
        pi.location().into()
    }
}

/// Represents the details of a Postgres `ERROR` that we caught via our use of `sigsetjmp()`
#[derive(Clone, Debug)]
pub struct ErrorData {
    pub(crate) sqlerrcode: PgSqlErrorCode,
    pub(crate) location: ErrorReportLocation,
    pub(crate) stack: Vec<ErrorReportLocation>,
}

/// Represents the set of information necessary for pgx to promote a Rust `panic!()` to a Postgres
/// `ERROR` (or any [`PgLogLevel`] level)
#[derive(Clone, Debug)]
pub struct ErrorReport {
    pub(crate) errcode: PgSqlErrorCode,
    message: String,
    detail: Option<String>,
    funcname: Option<String>,
    location: ErrorReportLocation,
}

#[derive(Clone, Debug)]
pub struct ErrorReportWithLevel {
    level: PgLogLevel,
    pub(crate) ereport: ErrorReport,
    stack: Vec<ErrorReportLocation>,
}

impl ErrorReportWithLevel {
    #[track_caller]
    fn report(self) {
        // ONLY if the log level is >=ERROR, we convert ourselves into a Rust panic and ask
        // rust to raise us as a `panic!()`
        //
        // Lesser levels (INFO, WARNING, LOG, etc) will just emit a message which isn't a panic condition
        if crate::ERROR <= self.level as _ {
            panic_any(self)
        } else {
            do_ereport(self)
        }
    }
}

impl ErrorReport {
    /// Create a [PgErrorReport] which can be raised via Rust's [std::panic::panic_any()] or as
    /// a specific Postgres "ereport()` level via [PgErrorReport::report(self, PgLogLevel)]
    ///
    /// Embedded "file:line:col" location information is taken from the caller's location
    #[track_caller]
    pub fn new<S: Into<String>>(errcode: PgSqlErrorCode, message: S) -> Self {
        Self {
            errcode,
            message: message.into(),
            detail: None,
            funcname: None,
            location: Location::caller().into(),
        }
    }

    /// Create a [PgErrorReport] which can be raised via Rust's [std::panic::panic_any()] or as
    /// a specific Postgres "ereport()` level via [PgErrorReport::report(self, PgLogLevel)].
    ///
    /// For internal use only
    fn with_location<S: Into<String>>(
        errcode: PgSqlErrorCode,
        message: S,
        location: ErrorReportLocation,
    ) -> Self {
        Self { errcode, message: message.into(), detail: None, funcname: None, location }
    }

    /// Set the `detail` property, whose default is `None`
    pub fn detail<S: Into<String>>(mut self, detail: S) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Set the `funcname` property, whose default is `None`
    pub fn funcname<S: Into<String>>(mut self, funcname: S) -> Self {
        self.funcname = Some(funcname.into());
        self
    }

    /// Report this [PgErrorReport], which will ultimately be reported by Postgres at the specified [PgLogLevel]
    ///
    /// If the provided `level` is >= [`PgLogLevel::ERROR`] this function will not return.
    #[track_caller]
    pub fn report(self, level: PgLogLevel) {
        ErrorReportWithLevel { level, ereport: self, stack: Default::default() }.report()
    }
}

thread_local! { static PANIC_LOCATION: Cell<Option<ErrorReportLocation >> = const { Cell::new(None) }}

fn take_panic_location() -> ErrorReportLocation {
    PANIC_LOCATION.with(|p| p.take().unwrap_or_default())
}

pub fn register_pg_guard_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        PANIC_LOCATION.with(|thread_local| {
            let existing = thread_local.take();

            thread_local.replace(if existing.is_none() {
                info.location().map(|l| l.into())
            } else {
                existing
            })
        });
    }))
}

/// What kind of error was caught?
#[derive(Debug)]
pub enum CaughtError {
    PostgresError(ErrorData),
    ErrorReport(ErrorReportWithLevel),
    RustPanic { ereport: ErrorReportWithLevel, payload: Box<dyn Any + Send> },
}

impl CaughtError {
    /// Rethrow this [CaughtError].  
    ///
    /// This is the same as [std::panic::resume_unwind()] and has the same semantics.
    #[track_caller]
    pub fn rethrow(mut self) -> ! {
        let location = Location::caller().into();
        match &mut self {
            CaughtError::PostgresError(errdata) => errdata.stack.push(location),
            CaughtError::ErrorReport(ereport) => ereport.stack.push(location),
            CaughtError::RustPanic { ereport, .. } => ereport.stack.push(location),
        }

        self.rethrow_ignore_location()
    }

    #[inline]
    pub(crate) fn rethrow_ignore_location(self) -> ! {
        // we resume_unwind here as [CaughtError] represents a previously caught panic, not a new
        // one to be thrown
        resume_unwind(Box::new(self))
    }
}

#[derive(Debug, Clone)]
enum GuardAction<R> {
    Return(R),
    ReThrow,
    Report(ErrorReportWithLevel),
}

/// Guard a closure such that Rust Panics are properly converted into Postgres ERRORs.
///
/// Note that any Postgres ERRORs raised within the supplied closure are transparently converted
/// to Rust panics.
///
/// Generally, this function won't need to be used directly, as it's also the implementation
/// behind the `#[pg_guard]` and `#[pg_extern]` macros.  Which means the function you'd like to guard
/// is likely already guarded.
///
/// Where it does need to be used is as a wrapper around Rust `extern "C"` function pointers given
/// to Postgres, and the `#[pg_guard]` macro takes care of this for you.
///
/// In other words, this isn't the function you're looking for.
///
/// You're probably looking for the `#[pg_guard]` macro.
///
/// Alternatively, if you're trying to mimic Postgres' C `PG_TRY/PG_CATCH` API, then you instead
/// want [`crate::pg_try::PgTryBuilder`].
///
/// # Safety
///
/// The function needs to only have [trivially-deallocated stack frames]
/// above it (well, in practice it does, and once that stabilizes it will be even more of a requirement).
/// In the short term this probably looks like just making it `unsafe`.
///
/// I think we also will need to implement it a lot more carefully to ensure it's safe to longjmp out of,
/// and probably do something about `Func` too...
///
/// [trivially-deallocated stack frames](https://github.com/rust-lang/rfcs/blob/master/text/2945-c-unwind-abi.md#plain-old-frames)
#[doc(hidden)]
pub unsafe fn pgx_extern_c_guard<Func, R: Copy>(f: Func) -> R
where
    Func: FnOnce() -> R + UnwindSafe + RefUnwindSafe,
{
    match run_guarded(f) {
        GuardAction::Return(r) => r,
        GuardAction::ReThrow => {
            extern "C" /* "C-unwind" */ {
                fn pg_re_throw() -> !;
            }
            unsafe { pg_re_throw() }
        }
        GuardAction::Report(ereport) => {
            do_ereport(ereport);
            unreachable!("pgx reported a CaughtError that wasn't raised at ERROR or above");
        }
    }
}

#[inline(never)]
fn run_guarded<F, R: Copy>(f: F) -> GuardAction<R>
where
    F: FnOnce() -> R + UnwindSafe + RefUnwindSafe,
{
    match catch_unwind(f) {
        Ok(v) => GuardAction::Return(v),
        Err(e) => match downcast_err(e) {
            CaughtError::PostgresError(errdata) => {
                extern "C" {
                    fn pgx_errcontext_msg(message: *mut std::os::raw::c_char);
                }
                unsafe {
                    pgx_errcontext_msg(errdata.location.to_string().as_pg_cstr());
                    pgx_errcontext_msg(contexts_as_pg_cstr(&errdata.stack));
                }

                // Return to the caller to rethrow -- we can't do it here
                // since we this function's has non-POF frames.
                GuardAction::ReThrow
            }
            CaughtError::ErrorReport(ereport) | CaughtError::RustPanic { ereport, .. } => {
                GuardAction::Report(ereport)
            }
        },
    }
}

/// convert types of `e` that we understand/expect into the representative [CaughtError]
pub(crate) fn downcast_err(e: Box<dyn Any + Send>) -> CaughtError {
    if e.downcast_ref::<CaughtError>().is_some() {
        // caught a previously caught CaughtError that is being rethrown
        *e.downcast().unwrap()
    } else if e.downcast_ref::<ErrorData>().is_some() {
        // caught an error via our `sigsetjmp` handling at FFI boundaries
        CaughtError::PostgresError(*e.downcast().unwrap())
    } else if e.downcast_ref::<ErrorReportWithLevel>().is_some() {
        // someone called `panic_any(PgErrorReportWithLevel)`
        CaughtError::ErrorReport(*e.downcast().unwrap())
    } else if e.downcast_ref::<ErrorReport>().is_some() {
        // someone called `panic_any(PgErrorReport)` so we convert it to be PgLogLevel::ERROR
        CaughtError::ErrorReport(ErrorReportWithLevel {
            level: PgLogLevel::ERROR,
            ereport: *e.downcast().unwrap(),
            stack: Default::default(),
        })
    } else if let Some(message) = e.downcast_ref::<&str>() {
        // something panic'd with a &str, so it gets raised as an INTERNAL_ERROR at the ERROR level
        CaughtError::RustPanic {
            ereport: ErrorReportWithLevel {
                level: PgLogLevel::ERROR,
                ereport: ErrorReport::with_location(
                    PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                    *message,
                    take_panic_location(),
                ),
                stack: Default::default(),
            },
            payload: e,
        }
    } else if let Some(message) = e.downcast_ref::<String>() {
        // something panic'd with a String, so it gets raised as an INTERNAL_ERROR at the ERROR level
        CaughtError::RustPanic {
            ereport: ErrorReportWithLevel {
                level: PgLogLevel::ERROR,
                ereport: ErrorReport::with_location(
                    PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                    message,
                    take_panic_location(),
                ),
                stack: Default::default(),
            },
            payload: e,
        }
    } else {
        // not a type we understand, so it gets raised as an INTERNAL_ERROR at the ERROR level
        CaughtError::RustPanic {
            ereport: ErrorReportWithLevel {
                level: PgLogLevel::ERROR,
                ereport: ErrorReport::with_location(
                    PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                    "Box<Any>",
                    take_panic_location(),
                ),
                stack: Default::default(),
            },
            payload: e,
        }
    }
}

fn do_ereport(ereport: ErrorReportWithLevel) {
    // we define this here to make it difficult for not only pgx, but pgx users
    // to find and directly call this function.  They'd have to do the same as
    // this, and that seems like more work than a normal programmer would want to do
    extern "C" {
        fn pgx_ereport(
            level: i32,
            code: i32,
            message: *const std::os::raw::c_char,
            detail: *const std::os::raw::c_char,
            funcname: *const std::os::raw::c_char,
            file: *const std::os::raw::c_char,
            lineno: i32,
            contexts: *const std::os::raw::c_char,
        );
    }

    unsafe {
        // because of the calls to `.as_pg_cstr()`, which allocate using `palloc0()`,
        // we need to be in the `ErrorContext` when we allocate those
        //
        // specifically, the problem here is `self.panic.location.file & .funcname`.  At the C level,
        // Postgres expects these to be static strings, created at compile time, rather
        // than something allocated from a MemoryContext.  Our version of ereport (pgx_ereport)
        // accepts a user-provided strings for them, so we can report function/file/line information
        // from rust code.
        //
        // We just go ahead and allocate all the strings we need in the `ErrorContext` for convenience
        let old_cxt = MemoryContextSwitchTo(crate::ErrorContext);
        let level = ereport.level as i32;
        let errocode = ereport.ereport.errcode as i32;
        let contexts = contexts_as_pg_cstr(&ereport.stack);
        let funcname = ereport.ereport.funcname.as_ref().as_pg_cstr();
        let file = ereport.ereport.location.file.as_str().as_pg_cstr();
        let message = ereport.ereport.message.as_str().as_pg_cstr();
        let detail = ereport.ereport.detail.as_ref().as_pg_cstr();
        let line = ereport.ereport.location.line as i32;
        MemoryContextSwitchTo(old_cxt);

        // before calling `pgx_ereport` it's imperative we drop everything Rust-allocated we possibly can.
        // `pgx_ereport` very well might `longjmp` to somewhere else, either in pgx or Postgres, and
        // we'd rather not be leaking memory during error handling
        //
        // the few `.as_pg_cstr()`s do their allocation in Postgres' `CurrentMemoryContext`, so they'll
        // be cleaned up by Postgres at the right time
        drop(ereport);

        // there's a good chance this will `longjump` us out of here
        pgx_ereport(level, errocode, message, detail, funcname, file, line, contexts);

        if crate::ERROR <= level as _ {
            // SAFETY:  this is true because if we're being reported as an ERROR or greater,
            // we'll never return from the above call to `pgx_ereport()`
            unreachable_unchecked()
        }

        // if pgx_ereport() returned control (user didn't report a message at a level >=ERROR)
        // then lets not leak our fucname & file pointers
        if !file.is_null() {
            pfree(file.cast())
        }
        if !funcname.is_null() {
            pfree(funcname.cast())
        }
    }
}

fn contexts_as_pg_cstr(stack: &Vec<ErrorReportLocation>) -> *mut core::ffi::c_char {
    let contexts = if stack.is_empty() {
        None
    } else {
        Some(stack.iter().map(|s| s.to_string()).collect::<Vec<_>>().join("\n"))
    };
    contexts.as_pg_cstr()
}
