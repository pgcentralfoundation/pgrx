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
use std::ffi::CStr;
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
    pub(crate) file: String,
    pub(crate) funcname: Option<String>,
    pub(crate) line: u32,
    pub(crate) col: u32,
}

impl Default for ErrorReportLocation {
    fn default() -> Self {
        Self { file: std::string::String::from("<unknown>"), funcname: None, line: 0, col: 0 }
    }
}

impl Display for ErrorReportLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.funcname {
            Some(funcname) => {
                // mimic's Postgres' output for this, but includes a column number
                write!(f, "{}, {}:{}:{}", funcname, self.file, self.line, self.col)
            }

            None => {
                write!(f, "{}:{}:{}", self.file, self.line, self.col)
            }
        }
    }
}

impl From<&Location<'_>> for ErrorReportLocation {
    fn from(location: &Location<'_>) -> Self {
        Self {
            file: location.file().to_string(),
            funcname: None,
            line: location.line(),
            col: location.column(),
        }
    }
}

impl From<&PanicInfo<'_>> for ErrorReportLocation {
    fn from(pi: &PanicInfo<'_>) -> Self {
        pi.location().map(|l| l.into()).unwrap_or_default()
    }
}

/// Represents the set of information necessary for pgx to promote a Rust `panic!()` to a Postgres
/// `ERROR` (or any [`PgLogLevel`] level)
#[derive(Clone, Debug)]
pub struct ErrorReport {
    pub(crate) sqlerrcode: PgSqlErrorCode,
    pub(crate) message: String,
    pub(crate) hint: Option<String>,
    pub(crate) detail: Option<String>,
    pub(crate) location: ErrorReportLocation,
}

#[derive(Clone, Debug)]
pub struct ErrorReportWithLevel {
    pub(crate) level: PgLogLevel,
    pub(crate) inner: ErrorReport,
}

impl ErrorReportWithLevel {
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

    /// Returns the logging level of this error report
    pub fn level(&self) -> PgLogLevel {
        self.level
    }

    /// Returns the sql error code of this error report
    pub fn sql_error_code(&self) -> PgSqlErrorCode {
        self.inner.sqlerrcode
    }

    /// Returns the error message of this error report
    pub fn message(&self) -> &str {
        self.inner.message()
    }

    /// Returns the detail line of this error report, if there is one
    pub fn detail(&self) -> Option<&str> {
        self.inner.detail()
    }

    /// Returns the hint line of this error report, if there is one
    pub fn hint(&self) -> Option<&str> {
        self.inner.hint()
    }

    /// Returns the name of the source file that generated this error report
    pub fn file(&self) -> &str {
        &self.inner.location.file
    }

    /// Returns the line number of the source file that generated this error report
    pub fn line_number(&self) -> u32 {
        self.inner.location.line
    }

    /// Returns the name of the function that generated this error report, if we were able to figure it out
    pub fn function_name(&self) -> Option<&str> {
        self.inner.location.funcname.as_ref().map(|s| s.as_str())
    }

    /// Returns the context message of this error report, if any
    fn context_message(&self) -> Option<String> {
        // NB:  holding this here for future use
        None
    }
}

impl ErrorReport {
    /// Create a [PgErrorReport] which can be raised via Rust's [std::panic::panic_any()] or as
    /// a specific Postgres "ereport()` level via [PgErrorReport::report(self, PgLogLevel)]
    ///
    /// Embedded "file:line:col" location information is taken from the caller's location
    #[track_caller]
    pub fn new<S: Into<String>>(
        sqlerrcode: PgSqlErrorCode,
        message: S,
        funcname: &'static str,
    ) -> Self {
        let mut location: ErrorReportLocation = Location::caller().into();
        location.funcname = Some(funcname.to_string());

        Self { sqlerrcode, message: message.into(), hint: None, detail: None, location }
    }

    /// Create a [PgErrorReport] which can be raised via Rust's [std::panic::panic_any()] or as
    /// a specific Postgres "ereport()` level via [PgErrorReport::report(self, PgLogLevel)].
    ///
    /// For internal use only
    fn with_location<S: Into<String>>(
        sqlerrcode: PgSqlErrorCode,
        message: S,
        location: ErrorReportLocation,
    ) -> Self {
        Self { sqlerrcode, message: message.into(), hint: None, detail: None, location }
    }

    /// Set the `detail` property, whose default is `None`
    pub fn set_detail<S: Into<String>>(mut self, detail: S) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Set the `hint` property, whose default is `None`
    pub fn set_hint<S: Into<String>>(mut self, hint: S) -> Self {
        self.hint = Some(hint.into());
        self
    }

    /// Returns the error message of this error report
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the detail message of this error report
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_ref().map(|s| s.as_str())
    }

    /// Returns the hint message of this error report
    pub fn hint(&self) -> Option<&str> {
        self.hint.as_ref().map(|s| s.as_str())
    }

    /// Report this [PgErrorReport], which will ultimately be reported by Postgres at the specified [PgLogLevel]
    ///
    /// If the provided `level` is >= [`PgLogLevel::ERROR`] this function will not return.
    pub fn report(self, level: PgLogLevel) {
        ErrorReportWithLevel { level, inner: self }.report()
    }
}

thread_local! { static PANIC_LOCATION: Cell<Option<ErrorReportLocation >> = const { Cell::new(None) }}

fn take_panic_location() -> ErrorReportLocation {
    PANIC_LOCATION.with(|p| p.take().unwrap_or_default())
}

pub fn register_pg_guard_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        PANIC_LOCATION.with(|thread_local| thread_local.replace(Some(info.into())));
    }))
}

/// What kind of error was caught?
#[derive(Debug)]
pub enum CaughtError {
    /// An error raised from within Postgres
    PostgresError(ErrorReportWithLevel),

    /// A `pgx::error!()` or `pgx::ereport!(ERROR, ...)` raised from within Rust
    ErrorReport(ErrorReportWithLevel),

    /// A Rust `panic!()` or `std::panic::panic_any()`
    RustPanic { ereport: ErrorReportWithLevel, payload: Box<dyn Any + Send> },
}

impl CaughtError {
    /// Rethrow this [CaughtError].  
    ///
    /// This is the same as [std::panic::resume_unwind()] and has the same semantics.
    pub fn rethrow(self) -> ! {
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
/// The function needs to only have [trivially-deallocated stack frames]
/// above it. That is, the caller (and their caller, etc) cannot have
/// objects with pending destructors in their stack frames, unless those
/// objects have already been dropped.
///
/// In practice, this should only ever be called at the top level of an
/// `extern "C" fn` (ideally `extern "C-unwind"`) implemented in
/// Rust.
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
            unsafe {
                crate::CurrentMemoryContext = crate::ErrorContext;
                pg_re_throw()
            }
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
        Err(e) => match downcast_panic_payload(e) {
            CaughtError::PostgresError(_) => {
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
pub(crate) fn downcast_panic_payload(e: Box<dyn Any + Send>) -> CaughtError {
    if e.downcast_ref::<CaughtError>().is_some() {
        // caught a previously caught CaughtError that is being rethrown
        *e.downcast::<CaughtError>().unwrap()
    } else if e.downcast_ref::<ErrorReportWithLevel>().is_some() {
        // someone called `panic_any(PgErrorReportWithLevel)`
        CaughtError::ErrorReport(*e.downcast().unwrap())
    } else if e.downcast_ref::<ErrorReport>().is_some() {
        // someone called `panic_any(PgErrorReport)` so we convert it to be PgLogLevel::ERROR
        CaughtError::ErrorReport(ErrorReportWithLevel {
            level: PgLogLevel::ERROR,
            inner: *e.downcast().unwrap(),
        })
    } else if let Some(message) = e.downcast_ref::<&str>() {
        // something panic'd with a &str, so it gets raised as an INTERNAL_ERROR at the ERROR level
        CaughtError::RustPanic {
            ereport: ErrorReportWithLevel {
                level: PgLogLevel::ERROR,
                inner: ErrorReport::with_location(
                    PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                    *message,
                    take_panic_location(),
                ),
            },
            payload: e,
        }
    } else if let Some(message) = e.downcast_ref::<String>() {
        // something panic'd with a String, so it gets raised as an INTERNAL_ERROR at the ERROR level
        CaughtError::RustPanic {
            ereport: ErrorReportWithLevel {
                level: PgLogLevel::ERROR,
                inner: ErrorReport::with_location(
                    PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                    message,
                    take_panic_location(),
                ),
            },
            payload: e,
        }
    } else {
        // not a type we understand, so it gets raised as an INTERNAL_ERROR at the ERROR level
        CaughtError::RustPanic {
            ereport: ErrorReportWithLevel {
                level: PgLogLevel::ERROR,
                inner: ErrorReport::with_location(
                    PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                    "Box<Any>",
                    take_panic_location(),
                ),
            },
            payload: e,
        }
    }
}

/// This is a (as faithful as possible) Rust unrolling of Postgres' `#define ereport(...)` macro.
///
/// Bits of it are behind `#[cfg(feature)` flags for various Postgres versions and may need to be
/// updated as new versions of Postgres are released.
///
/// We localize the definition of the various `err*()` functions involved in reporting a Postgres
/// error (and purposely exclude them from `build.rs`) to ensure users can't get into trouble
/// trying to roll their own error handling.
#[rustfmt::skip]    // my opinion wins
fn do_ereport(ereport: ErrorReportWithLevel) {
    // SAFETY:  we are providing a null-terminated byte string
    const PERCENT_S: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"%s\0") };
    const DOMAIN: *const ::std::os::raw::c_char = std::ptr::null_mut();

    //
    // only declare these functions here.  They're explicitly excluded from bindings generation in 
    // `build.rs` and we'd prefer pgx users not have access to them at all
    //

    extern "C" {
        fn errcode(sqlerrcode: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
        fn errmsg(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
        fn errdetail(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
        fn errhint(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
        fn errcontext_msg(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    }

    #[cfg(any(feature = "pg11", feature = "pg12"))]
    extern "C" {
        fn errstart(elevel: ::std::os::raw::c_int, filename: *const ::std::os::raw::c_char, lineno: ::std::os::raw::c_int, funcname: *const ::std::os::raw::c_char, domain: *const ::std::os::raw::c_char) -> bool;
        fn errfinish(dummy: ::std::os::raw::c_int, ...);
    }

    #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15"))]
    extern "C" {
        fn errstart(elevel: ::std::os::raw::c_int, domain: *const ::std::os::raw::c_char) -> bool;
        fn errfinish(filename: *const ::std::os::raw::c_char, lineno: ::std::os::raw::c_int, funcname: *const ::std::os::raw::c_char);
    }

    let level = ereport.level();
    let sqlerrcode = ereport.sql_error_code();
    let message = ereport.message().as_pg_cstr();
    let detail = ereport.detail().as_pg_cstr();
    let hint = ereport.hint().as_pg_cstr();
    let context = ereport.context_message().as_pg_cstr();
    let lineno = ereport.line_number();

    unsafe {
        // SAFETY:  We know that `crate::ErrorContext` is a valid memory context pointer and one
        // that Postgres will clean up for us in the event of an ERROR, and we know it'll live long
        // enough for Postgres to use `file` and `funcname`, which it expects to be `const char *`s
        let prev_cxt = MemoryContextSwitchTo(crate::ErrorContext);
        let file = ereport.file().as_pg_cstr();
        let funcname = ereport.function_name().as_pg_cstr();
        MemoryContextSwitchTo(prev_cxt);

        // do not leak the Rust `ErrorReportWithLocation` instance
        drop(ereport);

        // SAFETY
        //
        // The following functions are all FFI into Postgres, so they're inherently unsafe.
        //
        // The various pointers used as arguments to these functions might have been allocated above
        // or they might be the null pointer, so we guard against that possibility for each usage.

        if {
            #[cfg(any(feature = "pg11", feature = "pg12"))]
            {
                errstart(level as _, file, lineno as _, funcname, DOMAIN)
            }

            #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15"))]
            {
                errstart(level as _, DOMAIN)
            }
        } {
            errcode(sqlerrcode as _);
            if !message.is_null() { errmsg(PERCENT_S.as_ptr(), message);         pfree(message.cast()); }
            if !detail.is_null()  { errdetail(PERCENT_S.as_ptr(), detail);       pfree(detail.cast());  }
            if !hint.is_null()    { errhint(PERCENT_S.as_ptr(), hint);           pfree(hint.cast());    }
            if !context.is_null() { errcontext_msg(PERCENT_S.as_ptr(), context); pfree(context.cast()); }

            #[cfg(any(feature = "pg11", feature = "pg12"))]
            errfinish(0);

            #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15"))]
            errfinish(file, lineno as _, funcname);
        }

        if level >= PgLogLevel::ERROR {
            // SAFETY:  `crate::errstart() is guaranteed to have returned true if >=ERROR and
            // `crate::errfinish()` is guaranteed to not have not returned at all if >= ERROR, which
            // means we won't either
            unreachable_unchecked()
        } else {
            // if it wasn't an ERROR we need to free up the things that Postgres wouldn't have
            if !file.is_null()     { pfree(file.cast());     }
            if !funcname.is_null() { pfree(funcname.cast()); }
        }
    }
}
