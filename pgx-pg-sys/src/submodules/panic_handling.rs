/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

use crate::elog::PgLogLevel;
use crate::errcodes::PgSqlErrorCode;
use crate::{
    pfree, AsPgCStr, CopyErrorData, CurrentMemoryContext, FlushErrorState, FreeErrorData,
    MemoryContextSwitchTo, TopTransactionContext,
};
use std::any::Any;
use std::cell::Cell;
use std::hint::unreachable_unchecked;
use std::panic::{catch_unwind, Location, PanicInfo, RefUnwindSafe, UnwindSafe};

/// Represents an `ERROR` that we caught from Postgres, via our use of `sigsetjmp()`
#[derive(Clone, Debug)]
pub(crate) struct PostgresError {}

/// Represents the set of information necessary for pgx to promote a Rust `panic!()` to a Postgres
/// `ERROR` (or any [`PgLogLevel`] level)
#[derive(Clone, Debug)]
pub struct PgErrorReport {
    errcode: PgSqlErrorCode,
    message: std::string::String,
    detail: Option<std::string::String>,
    funcname: Option<std::string::String>,
    location: PgErrorReportLocation,
}

#[derive(Clone, Debug)]
struct PgErrorReportLocation {
    file: std::string::String,
    line: u32,
    col: u32,
}

impl Default for PgErrorReportLocation {
    /// `#[track_caller]` is applied here so we can construct one with with
    /// the correct code location information
    #[track_caller]
    fn default() -> Self {
        Location::caller().into()
    }
}

impl From<&Location<'_>> for PgErrorReportLocation {
    fn from(location: &Location<'_>) -> Self {
        Self { file: location.file().to_string(), line: location.line(), col: location.column() }
    }
}

impl From<Option<&Location<'_>>> for PgErrorReportLocation {
    fn from(location: Option<&Location<'_>>) -> Self {
        location.into()
    }
}

impl From<Option<PgErrorReportLocation>> for PgErrorReportLocation {
    fn from(location: Option<PgErrorReportLocation>) -> Self {
        location.unwrap_or_else(|| PgErrorReportLocation {
            file: std::string::String::from("<unknown>"),
            line: 0,
            col: 0,
        })
    }
}

impl From<&PanicInfo<'_>> for PgErrorReportLocation {
    fn from(pi: &PanicInfo<'_>) -> Self {
        pi.location().into()
    }
}

#[derive(Clone, Debug)]
struct PgErrorReportWithLevel {
    level: PgLogLevel,
    panic: PgErrorReport,
}

impl PgErrorReportWithLevel {
    fn report(self) {
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
                colno: i32,
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
            // from rust code
            let old_cxt = MemoryContextSwitchTo(crate::ErrorContext);
            let funcname = self.panic.funcname.as_pg_cstr();
            let file = self.panic.location.file.as_pg_cstr();
            MemoryContextSwitchTo(old_cxt);

            pgx_ereport(
                self.level as _,
                self.panic.errcode as _,
                self.panic.message.as_pg_cstr(),
                self.panic.detail.as_pg_cstr(),
                funcname,
                file,
                self.panic.location.line as _,
                self.panic.location.col as _,
            );

            if crate::ERROR <= self.level as _ {
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
}

impl PgErrorReport {
    /// Create a [PgErrorReport] which can be raised via Rust's [std::panic::panic_any()] or as
    /// a specific Postgres "ereport()` level via [PgErrorReport::report(self, PgLogLevel)]
    ///
    /// Embedded "file:line:col" location information is taken from the caller's location
    #[track_caller]
    pub fn new<S: Into<std::string::String>>(errcode: PgSqlErrorCode, message: S) -> Self {
        Self {
            errcode,
            message: message.into(),
            detail: None,
            funcname: None,
            location: PgErrorReportLocation::default(),
        }
    }

    /// Create a [PgErrorReport] which can be raised via Rust's [std::panic::panic_any()] or as
    /// a specific Postgres "ereport()` level via [PgErrorReport::report(self, PgLogLevel)].
    ///
    /// For internal use only
    fn with_location<S: Into<std::string::String>>(
        errcode: PgSqlErrorCode,
        message: S,
        location: PgErrorReportLocation,
    ) -> Self {
        Self { errcode, message: message.into(), detail: None, funcname: None, location }
    }

    /// Set the `detail` property, whose default is `None`
    pub fn detail<S: Into<std::string::String>>(mut self, detail: S) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Set the `funcname` property, whose default is `None`
    pub fn funcname<S: Into<std::string::String>>(mut self, funcname: S) -> Self {
        self.funcname = Some(funcname.into());
        self
    }

    /// Report this [PgErrorReport], which will ultimately be reported by Postgres at the specified [PgLogLevel]
    #[inline]
    pub fn report(self, level: PgLogLevel) {
        PgErrorReportWithLevel { level, panic: self }.report()
    }
}

thread_local! { static PANIC_LOCATION: Cell<Option<PgErrorReportLocation >> = Cell::new(None) }

fn take_panic_location() -> PgErrorReportLocation {
    PANIC_LOCATION.with(|p| p.take().into())
}

pub fn register_pg_guard_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        PANIC_LOCATION.with(|p| {
            let existing = p.take();

            p.replace(if existing.is_none() { info.location().map(|l| l.into()) } else { existing })
        });
    }))
}

/// A `std::result::Result`-type value returned from `pg_try()` that allows for performing cleanup
/// work after a closure raised an error and before it is possibly rethrown
#[must_use = "this `PgTryResult` may be be holding a Postgres ERROR.  It must be consumed or rethrown"]
pub struct PgTryResult<T>(std::thread::Result<T>);

impl<T> PgTryResult<T> {
    /// Returns `true` if the Result is `Ok`
    pub fn is_ok(&self) -> bool {
        self.0.is_ok()
    }

    /// If this [PgTryResult] is holding onto an Error, what is its corresponding
    /// SQL error code.  A usable version can be looked up through `pgx::PgSqlErrorCodes::from()`
    pub fn sqlerrcode(&self) -> Option<PgSqlErrorCode> {
        if self.is_ok() {
            None
        } else {
            unsafe {
                // get the sql error code that caused this PgTryResult to be in an error state

                // must allocate the ErrorData struct in a memory context that isn't
                // the current memory context as the current memory context is the "ErrorContext"
                let previous = CurrentMemoryContext;
                CurrentMemoryContext = TopTransactionContext;
                let errordata = CopyErrorData();
                let sqlerrcode = (*errordata).sqlerrcode;

                // cleanup after ourselves
                FreeErrorData(errordata);
                CurrentMemoryContext = previous;

                // and return the final error code back to the caller as something that'll
                // later be compatible with `pgx::PgSqlErrorCodes`, which is not part of this crate
                Some(PgSqlErrorCode::from(sqlerrcode as isize))
            }
        }
    }

    /// Retrieve the returned value or panic if the try block raised an error
    pub fn unwrap(self) -> T {
        self.unwrap_or_rethrow(|| {})
    }

    /// ## Safety
    ///
    /// This function is unsafe because you might be ignoring a caught Postgres ERROR (or Rust panic)
    /// and you better know what you're doing when you do that.  
    ///
    /// Doing so can potentially leave Postgres in an undefined state and ultimately cause it
    /// to crash.
    // Maybe not actually unsafe? Depends on why an error is reached.
    pub unsafe fn unwrap_or(self, value: T) -> T {
        match self.0 {
            Ok(result) => result,
            Err(_) => {
                // SAFETY: Caller asserts it is okay to avoid rethrowing an ERROR.
                unsafe { FlushErrorState() };
                value
            }
        }
    }

    /// If the result is successful, return `Ok(value)`, otherwise return `Err(error_code)` if
    /// that was the error Postgres raised during `pg_try()`.  
    ///
    /// If Postgres raised a different error, then it is immediately rethrown.
    pub fn unwrap_or_catch_error(self, error_code: PgSqlErrorCode) -> Result<T, PgSqlErrorCode> {
        match self.sqlerrcode() {
            None => Ok(self.unwrap()),
            Some(e) => {
                if e == error_code {
                    unsafe {
                        FlushErrorState();
                    }
                    Err(error_code)
                } else {
                    let _ = self.unwrap();
                    unsafe {
                        // SAFETY:  `self.unwrap()` on the above line will throw because we know
                        // we're holding onto an error
                        unreachable_unchecked()
                    }
                }
            }
        }
    }

    /// Perform some operation cleanup operation after the try block if an error was thrown.
    ///
    /// ## Safety
    ///
    /// This function does not rethrow a caught ERROR.  You better know what you're doing when you
    /// call this function.
    ///
    /// Ignoring a caught error can leave Postgres in an undefined state and ultimately cause it
    /// to crash.
    // Maybe not actually unsafe? Depends on why an error is reached.
    pub unsafe fn unwrap_or_else<F>(self, cleanup: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self.0 {
            Ok(result) => result,
            Err(_) => {
                // SAFETY: Caller asserts it is okay to avoid rethrowing an ERROR.
                unsafe { FlushErrorState() };
                cleanup()
            }
        }
    }

    /// Perform some operation cleanup operation after the try block if an error was thrown.
    ///
    /// In the event an error was caught, it is rethrown.
    pub fn unwrap_or_rethrow<F>(self, cleanup: F) -> T
    where
        F: FnOnce(),
    {
        match self.0 {
            Ok(result) => result,
            Err(e) => {
                catch_guard(e, cleanup);
                unreachable!("failed to rethrow ERROR during pg_try().unwrap_or_rethrow()")
            }
        }
    }

    /// Perform some operation after the try block completes, regardless of if an error was thrown.
    ///
    /// In the event an error was caught, it is rethrown.  Otherwise, the return value from the try
    /// block is returned
    pub fn finally_or_rethrow<F>(self, finally_block: F) -> T
    where
        F: FnOnce(),
    {
        match self.0 {
            Ok(result) => {
                finally_block();
                result
            }
            Err(e) => {
                catch_guard(e, finally_block);
                unreachable!("failed to rethrow ERROR during pg_try().finally_or_rethrow()")
            }
        }
    }
}

/// Guard a closure such that Rust Panics are properly converted into Postgres ERRORs
///
/// Generally, this function won't need to be used directly, as it's also the implementation
/// behind the `#[pg_guard]` and `#[pg_extern]` macros.  Which means the function you'd like to guard
/// is likely already guarded.
///
/// This function is re-entrant and will properly "bubble-up" panics or errors to the top-level
/// before they're converted into Postgres ERRORs
pub fn guard<Func, R>(f: Func) -> R
where
    Func: FnOnce() -> R + UnwindSafe + RefUnwindSafe,
{
    pg_try(f).unwrap()
}

/// Similar to `guard`, but allows the caller to unwrap the result in various ways, possibly
/// performing cleanup work before the caught error is rethrown
pub fn pg_try<Try, R>(try_func: Try) -> PgTryResult<R>
where
    Try: FnOnce() -> R + UnwindSafe + RefUnwindSafe,
{
    try_guard(try_func)
}

fn try_guard<Try, R>(try_func: Try) -> PgTryResult<R>
where
    Try: FnOnce() -> R + UnwindSafe + RefUnwindSafe,
{
    // run try_func() in a catch_unwind, as we never want a Rust panic! to leak
    // from this function.  It's imperative that we nevery try to panic! across
    // FFI (extern "C") function boundaries
    let result = catch_unwind(try_func);

    // return our result -- it could be Ok(), or it could be an Err()
    PgTryResult(result)
}

fn catch_guard<Catch>(error: Box<dyn Any + std::marker::Send>, catch_func: Catch)
where
    Catch: FnOnce(),
{
    // call our catch function to do any cleanup work that might be necessary
    // before we end up rethrowing the error
    catch_func();

    // determine how to rethrow the error
    match downcast_err(error) {
        // the error is a [PgErrorReportWithLevel], so it's an error from either a Rust `panic!()` or
        // from an error-raising [PgLogLevel] `ereport!()` call.
        Ok(error) => {
            error.report();
            unreachable!("PgErrorReportWithLevel.report() failed at depth==0");
        }

        // the error is a PostgresError, so all we can do is ask Postgres to rethrow it
        Err(_pg_error) => unsafe {
            extern "C" {
                // don't care to expose this to the rest of pgx
                fn pg_re_throw();
            }
            pg_re_throw();
            unreachable!("pg_re_throw() failed");
        },
    }
}

/// convert types of `e` that we understand/expect into either a
/// `Ok(String)` or a `Err<JumpContext>`
fn downcast_err(mut e: Box<dyn Any + Send>) -> Result<PgErrorReportWithLevel, PostgresError> {
    if e.downcast_mut::<PostgresError>().is_some() {
        Err(*e.downcast().unwrap())
    } else if e.downcast_mut::<PgErrorReportLocation>().is_some() {
        Ok(*e.downcast().unwrap())
    } else if e.downcast_mut::<PgErrorReport>().is_some() {
        Ok(PgErrorReportWithLevel { level: PgLogLevel::ERROR, panic: *e.downcast().unwrap() })
    } else if e.downcast_ref::<&str>().is_some() {
        Ok(PgErrorReportWithLevel {
            level: PgLogLevel::ERROR,
            panic: PgErrorReport::with_location::<&str>(
                PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                *e.downcast().unwrap(),
                take_panic_location(),
            ),
        })
    } else if e.downcast_ref::<std::string::String>().is_some() {
        Ok(PgErrorReportWithLevel {
            level: PgLogLevel::ERROR,
            panic: PgErrorReport::with_location::<std::string::String>(
                PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                *e.downcast().unwrap(),
                take_panic_location(),
            ),
        })
    } else {
        // not a type we understand, so use a generic string
        Ok(PgErrorReportWithLevel {
            level: PgLogLevel::ERROR,
            panic: PgErrorReport::with_location(
                PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                "Box<Any>",
                take_panic_location(),
            ),
        })
    }
}
