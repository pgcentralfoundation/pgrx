/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

use core::ffi::CStr;
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

/// Indicates that something can be reported as a Postgres ERROR, if that's what it might represent.
pub trait ErrorReportable {
    type Inner;

    /// Raise a Postgres ERROR if appropriate, otherwise return a value
    fn report(self) -> Self::Inner;
}

impl<T, E> ErrorReportable for Result<T, E>
where
    E: Any + Display,
{
    type Inner = T;

    /// If this [`Result`] represents the `Ok` variant, that value is returned.
    ///
    /// If this [`Result`] represents the `Err` variant, raise it as an error.  If it happens to
    /// be an [`ErrorReport`], then that is specifically raised.  Otherwise it's just a general
    /// [`ereport!`] as a [`PgLogLevel::ERROR`].
    fn report(self) -> Self::Inner {
        match self {
            Ok(value) => value,
            Err(e) => {
                let any: Box<&dyn Any> = Box::new(&e);
                if any.downcast_ref::<ErrorReport>().is_some() {
                    let any: Box<dyn Any> = Box::new(e);
                    any.downcast::<ErrorReport>().unwrap().report(PgLogLevel::ERROR);
                    unreachable!();
                } else {
                    ereport!(ERROR, PgSqlErrorCode::ERRCODE_DATA_EXCEPTION, &format!("{}", e));
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ErrorReportLocation {
    pub(crate) file: String,
    pub(crate) funcname: Option<String>,
    pub(crate) line: u32,
    pub(crate) col: u32,
    pub(crate) backtrace: Option<std::backtrace::Backtrace>,
}

impl Default for ErrorReportLocation {
    fn default() -> Self {
        Self {
            file: std::string::String::from("<unknown>"),
            funcname: None,
            line: 0,
            col: 0,
            backtrace: None,
        }
    }
}

impl Display for ErrorReportLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.funcname {
            Some(funcname) => {
                // mimic's Postgres' output for this, but includes a column number
                write!(f, "{}, {}:{}:{}", funcname, self.file, self.line, self.col)?;
            }

            None => {
                write!(f, "{}:{}:{}", self.file, self.line, self.col)?;
            }
        }

        if let Some(backtrace) = &self.backtrace {
            if backtrace.status() == std::backtrace::BacktraceStatus::Captured {
                write!(f, "\n{}", backtrace)?;
            }
        }

        Ok(())
    }
}

impl From<&Location<'_>> for ErrorReportLocation {
    fn from(location: &Location<'_>) -> Self {
        Self {
            file: location.file().to_string(),
            funcname: None,
            line: location.line(),
            col: location.column(),
            backtrace: None,
        }
    }
}

impl From<&PanicInfo<'_>> for ErrorReportLocation {
    fn from(pi: &PanicInfo<'_>) -> Self {
        pi.location().map(|l| l.into()).unwrap_or_default()
    }
}

/// Represents the set of information necessary for pgrx to promote a Rust `panic!()` to a Postgres
/// `ERROR` (or any [`PgLogLevel`] level)
#[derive(Debug)]
pub struct ErrorReport {
    pub(crate) sqlerrcode: PgSqlErrorCode,
    pub(crate) message: String,
    pub(crate) hint: Option<String>,
    pub(crate) detail: Option<String>,
    pub(crate) location: ErrorReportLocation,
}

impl Display for ErrorReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.sqlerrcode, self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, "\nHINT: {}", hint)?;
        }
        if let Some(detail) = &self.detail {
            write!(f, "\nDETAIL: {}", detail)?;
        }
        write!(f, "\nLOCATION: {}", self.location)
    }
}

#[derive(Debug)]
pub struct ErrorReportWithLevel {
    pub(crate) level: PgLogLevel,
    pub(crate) inner: ErrorReport,
}

impl ErrorReportWithLevel {
    fn report(self) {
        match self.level {
            // ERRORs get converted into panics so they can perform proper stack unwinding
            PgLogLevel::ERROR => panic_any(self),

            // FATAL and PANIC are reported directly to Postgres -- they abort the process
            PgLogLevel::FATAL | PgLogLevel::PANIC => {
                do_ereport(self);
                unreachable!()
            }

            // Everything else (INFO, WARN, LOG, DEBUG, etc) are reported to Postgres too but they only emit messages
            _ => do_ereport(self),
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

    /// Get the detail line with backtrace. If backtrace is not available, it will just return the detail.
    pub fn detail_with_backtrace(&self) -> Option<String> {
        match (self.detail(), self.backtrace()) {
            (Some(d), Some(bt)) if bt.status() == std::backtrace::BacktraceStatus::Captured => {
                Some(format!("{}\n{}", d, bt))
            }
            (Some(d), _) => Some(d.to_string()),
            (None, Some(bt)) if bt.status() == std::backtrace::BacktraceStatus::Captured => {
                Some(format!("\n{}", bt))
            }
            (None, _) => None,
        }
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

    /// Returns the backtrace when the error is reported
    pub fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        self.inner.location.backtrace.as_ref()
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

thread_local! { static PANIC_LOCATION: Cell<Option<ErrorReportLocation>> = const { Cell::new(None) }}

fn take_panic_location() -> ErrorReportLocation {
    PANIC_LOCATION.with(|p| p.take().unwrap_or_default())
}

pub fn register_pg_guard_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        PANIC_LOCATION.with(|thread_local| {
            thread_local.replace({
                let mut info: ErrorReportLocation = info.into();
                info.backtrace = Some(std::backtrace::Backtrace::capture());
                Some(info)
            })
        });
    }))
}

/// What kind of error was caught?
#[derive(Debug)]
pub enum CaughtError {
    /// An error raised from within Postgres
    PostgresError(ErrorReportWithLevel),

    /// A `pgrx::error!()` or `pgrx::ereport!(ERROR, ...)` raised from within Rust
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

#[derive(Debug)]
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
pub unsafe fn pgrx_extern_c_guard<Func, R: Copy>(f: Func) -> R
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
            unreachable!("pgrx reported a CaughtError that wasn't raised at ERROR or above");
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
/// Different implementations are provided for different postgres version ranges to ensure
/// best performance. Care is taken to avoid work if `errstart` signals we can finish early.
///
/// We localize the definition of the various `err*()` functions involved in reporting a Postgres
/// error (and purposely exclude them from `build.rs`) to ensure users can't get into trouble
/// trying to roll their own error handling.
fn do_ereport(ereport: ErrorReportWithLevel) {
    // SAFETY:  we are providing a null-terminated byte string
    const PERCENT_S: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"%s\0") };
    const DOMAIN: *const ::std::os::raw::c_char = std::ptr::null_mut();

    // the following code is definitely thread-unsafe -- not-the-main-thread can't be creating Postgres
    // ereports.  Our secret `extern "C"` definitions aren't wrapped by #[pg_guard] so we need to
    // manually do the active thread check
    crate::thread_check::check_active_thread();

    //
    // only declare these functions here.  They're explicitly excluded from bindings generation in
    // `build.rs` and we'd prefer pgrx users not have access to them at all
    //

    extern "C" {
        fn errcode(sqlerrcode: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
        fn errmsg(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
        fn errdetail(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
        fn errhint(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
        fn errcontext_msg(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    }

    /// do_ereport impl for postgres 13 and later
    /// In this case, we only allocate file, lineno and funcname if `errstart` returns true
    #[inline(always)]
    #[rustfmt::skip]    // my opinion wins
    #[cfg(any(feature = "pg13", feature = "pg14", feature = "pg15", feature = "pg16"))]
    fn do_ereport_impl(ereport: ErrorReportWithLevel) {

        extern "C" {
            fn errstart(elevel: ::std::os::raw::c_int, domain: *const ::std::os::raw::c_char) -> bool;
            fn errfinish(filename: *const ::std::os::raw::c_char, lineno: ::std::os::raw::c_int, funcname: *const ::std::os::raw::c_char);
        }

        let level = ereport.level();
        unsafe {
            if errstart(level as _, DOMAIN) {

                let sqlerrcode = ereport.sql_error_code();
                let message = ereport.message().as_pg_cstr();
                let detail = ereport.detail_with_backtrace().as_pg_cstr();
                let hint = ereport.hint().as_pg_cstr();
                let context = ereport.context_message().as_pg_cstr();
                let lineno = ereport.line_number();

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
                errcode(sqlerrcode as _);
                if !message.is_null() { errmsg(PERCENT_S.as_ptr(), message);         pfree(message.cast()); }
                if !detail.is_null()  { errdetail(PERCENT_S.as_ptr(), detail);       pfree(detail.cast());  }
                if !hint.is_null()    { errhint(PERCENT_S.as_ptr(), hint);           pfree(hint.cast());    }
                if !context.is_null() { errcontext_msg(PERCENT_S.as_ptr(), context); pfree(context.cast()); }

                errfinish(file, lineno as _, funcname);

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
    }

    /// do_ereport impl for postgres up to 12
    /// In this case, `errstart` takes file, lineno and funcname, which need special handling
    /// to be freed in case level < ERROR
    #[inline(always)]
    #[rustfmt::skip]    // my opinion wins
    #[cfg(any(feature = "pg11", feature = "pg12"))]
    fn do_ereport_impl(ereport: ErrorReportWithLevel) {

        extern "C" {
            fn errstart(elevel: ::std::os::raw::c_int, filename: *const ::std::os::raw::c_char, lineno: ::std::os::raw::c_int, funcname: *const ::std::os::raw::c_char, domain: *const ::std::os::raw::c_char) -> bool;
            fn errfinish(dummy: ::std::os::raw::c_int, ...);
        }

        unsafe {
            // SAFETY:  We know that `crate::ErrorContext` is a valid memory context pointer and one
            // that Postgres will clean up for us in the event of an ERROR, and we know it'll live long
            // enough for Postgres to use `file` and `funcname`, which it expects to be `const char *`s

            let prev_cxt = MemoryContextSwitchTo(crate::ErrorContext);
            let file = ereport.file().as_pg_cstr();
            let lineno = ereport.line_number();
            let funcname = ereport.function_name().as_pg_cstr();
            MemoryContextSwitchTo(prev_cxt);

            let level = ereport.level();
            if errstart(level as _, file, lineno as _, funcname, DOMAIN) {

                let sqlerrcode = ereport.sql_error_code();
                let message = ereport.message().as_pg_cstr();
                let detail = ereport.detail_with_backtrace().as_pg_cstr();
                let hint = ereport.hint().as_pg_cstr();
                let context = ereport.context_message().as_pg_cstr();


                // do not leak the Rust `ErrorReportWithLocation` instance
                drop(ereport);

                // SAFETY
                //
                // The following functions are all FFI into Postgres, so they're inherently unsafe.
                //
                // The various pointers used as arguments to these functions might have been allocated above
                // or they might be the null pointer, so we guard against that possibility for each usage.
                errcode(sqlerrcode as _);
                if !message.is_null() { errmsg(PERCENT_S.as_ptr(), message);         pfree(message.cast()); }
                if !detail.is_null()  { errdetail(PERCENT_S.as_ptr(), detail);       pfree(detail.cast());  }
                if !hint.is_null()    { errhint(PERCENT_S.as_ptr(), hint);           pfree(hint.cast());    }
                if !context.is_null() { errcontext_msg(PERCENT_S.as_ptr(), context); pfree(context.cast()); }

                errfinish(0);
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

    do_ereport_impl(ereport)
}
