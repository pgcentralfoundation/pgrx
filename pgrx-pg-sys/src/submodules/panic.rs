//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#![deny(unsafe_op_in_unsafe_fn)]
#![allow(non_snake_case)]

use core::ffi::{CStr, c_void};
use std::any::Any;
use std::borrow::Cow;
use std::cell::Cell;
use std::fmt::{Display, Formatter};
use std::hint::unreachable_unchecked;
use std::num::NonZeroUsize;
use std::panic::{
    AssertUnwindSafe, Location, PanicHookInfo, UnwindSafe, catch_unwind, panic_any, resume_unwind,
};
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::elog::PgLogLevel;
use crate::errcodes::PgSqlErrorCode;
use crate::{AsPgCStr, pfree};

/// Indicates that something can be reported as a Postgres ERROR, if that's what it might represent.
pub trait ErrorReportable {
    type Inner;

    /// Raise a Postgres ERROR if appropriate, otherwise return a value
    fn unwrap_or_report(self) -> Self::Inner;
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
    fn unwrap_or_report(self) -> Self::Inner {
        self.unwrap_or_else(|e| {
            let any: Box<&dyn Any> = Box::new(&e);
            if any.downcast_ref::<ErrorReport>().is_some() {
                let any: Box<dyn Any> = Box::new(e);
                any.downcast::<ErrorReport>().unwrap().report(PgLogLevel::ERROR);
                unreachable!();
            } else {
                ereport!(ERROR, PgSqlErrorCode::ERRCODE_DATA_EXCEPTION, format!("{e}"));
            }
        })
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

        if let Some(backtrace) = &self.backtrace
            && backtrace.status() == std::backtrace::BacktraceStatus::Captured
        {
            write!(f, "\n{backtrace}")?;
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

impl From<&PanicHookInfo<'_>> for ErrorReportLocation {
    fn from(pi: &PanicHookInfo<'_>) -> Self {
        pi.location().map(|l| l.into()).unwrap_or_default()
    }
}

/// Represents the set of information necessary for pgrx to promote a Rust `panic!()` to a Postgres
/// `ERROR` (or any [`PgLogLevel`] level)
#[derive(Debug)]
pub struct ErrorReport {
    pub(crate) sqlerrcode: PgSqlErrorCode,
    pub(crate) message: Cow<'static, str>,
    pub(crate) hint: Option<String>,
    pub(crate) detail: Option<String>,
    pub(crate) domain: Option<String>,
    pub(crate) location: ErrorReportLocation,
}

impl Display for ErrorReport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.sqlerrcode, self.message)?;
        if let Some(hint) = &self.hint {
            write!(f, "\nHINT: {hint}")?;
        }
        if let Some(detail) = &self.detail {
            write!(f, "\nDETAIL: {detail}")?;
        }
        write!(f, "\nLOCATION: {}", self.location)
    }
}

#[derive(Debug)]
pub struct ErrorReportWithLevel {
    pub(crate) level: PgLogLevel,
    pub(crate) inner: ErrorReport,
    /// Marks the stack entry of the Postgres ERROR this report copies, if pgrx caught one
    pub(crate) stack_entry: Option<ErrorStackEntry>,
}

/// Identifies the entry a caught Postgres ERROR occupies on Postgres' error data stack.
///
/// Postgres gives the entries on that stack no identity, so when pgrx catches an ERROR, it
/// registers a reset callback on `ErrorContext`, which goes to the head of the context's callback
/// list.  Only two things move the ERROR off the top of the stack: a reset of `ErrorContext`,
/// which `FlushErrorState()` does and which empties the list, and a newer ERROR, which pgrx marks
/// the same way when it catches it.  So while the callback is the head of the list, the ERROR is
/// the top of the stack, unless code that doesn't mark what it catches, such as C code or an
/// extension built with an older pgrx, keeps a newer ERROR without flushing it.
#[derive(Debug)]
pub(crate) struct ErrorStackEntry {
    token: NonZeroUsize,
}

/// The callback that marks a caught ERROR.  Its place in `ErrorContext`'s callback list is the
/// mark, so it has nothing to do when Postgres calls it during the reset.
unsafe extern "C-unwind" fn caught_error_mark(_token: *mut c_void) {}

impl ErrorStackEntry {
    /// Marks the entry at the top of Postgres' error data stack.  Returns `None` when
    /// `ErrorContext` has no memory left for the mark.
    ///
    /// # Safety
    ///
    /// Call only on the backend thread, while the ERROR that pgrx just caught is on top of the
    /// stack.
    pub(crate) unsafe fn mark_top() -> Option<Self> {
        static LAST_TOKEN: AtomicUsize = AtomicUsize::new(0);
        // a token that wraps to zero leaves its ERROR unmarked, which only costs fields
        let token = NonZeroUsize::new(LAST_TOKEN.fetch_add(1, Ordering::Relaxed).wrapping_add(1))?;

        // SAFETY: ErrorContext exists once Postgres can raise an ERROR.  The callback is
        // allocated in ErrorContext, and Postgres removes it from the list and calls it before
        // the reset frees it.  Its fields are written before it's registered.
        unsafe {
            let cb: *mut crate::MemoryContextCallback = crate::MemoryContextAllocExtended(
                crate::ErrorContext,
                size_of::<crate::MemoryContextCallback>(),
                crate::MCXT_ALLOC_NO_OOM as _,
            )
            .cast();
            if cb.is_null() {
                return None;
            }
            cb.write(crate::MemoryContextCallback {
                func: Some(caught_error_mark),
                arg: ptr::without_provenance_mut(token.get()),
                next: ptr::null_mut(),
            });
            crate::MemoryContextRegisterResetCallback(crate::ErrorContext, cb);
        }
        Some(Self { token })
    }

    /// Whether the marked ERROR is still the top of Postgres' error data stack.
    ///
    /// # Safety
    ///
    /// Call only on the backend thread.
    unsafe fn is_top(&self) -> bool {
        // SAFETY: ErrorContext exists once Postgres can raise an ERROR, and the head of its
        // callback list is either null or a callback that is still registered and allocated.
        unsafe {
            let head = (*crate::ErrorContext).reset_cbs;
            if head.is_null() {
                return false;
            }
            let mark: unsafe extern "C-unwind" fn(*mut c_void) = caught_error_mark;
            (*head).func.is_some_and(|func| ptr::fn_addr_eq(func, mark))
                && (*head).arg.addr() == self.token.get()
        }
    }
}

impl ErrorReportWithLevel {
    fn report(self) {
        match self.level {
            // ERRORs get converted into panics so they can perform proper stack unwinding
            PgLogLevel::ERROR => panic_any(self),

            // FATAL and PANIC are reported directly to Postgres -- they abort the process
            PgLogLevel::FATAL | PgLogLevel::PANIC => unsafe {
                crate::InterruptHoldoffCount += 1;
                do_ereport(self);
                unreachable!()
            },

            // Everything else (INFO, WARN, LOG, DEBUG, etc) are reported to Postgres too but they only emit messages
            _ => unsafe {
                crate::InterruptHoldoffCount += 1;
                do_ereport(self);
                assert!(crate::InterruptHoldoffCount > 0);
                crate::InterruptHoldoffCount -= 1;
            },
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
            (Some(detail), Some(bt))
                if bt.status() == std::backtrace::BacktraceStatus::Captured =>
            {
                Some(format!("{detail}\n{bt}"))
            }
            (Some(d), _) => Some(d.to_string()),
            (None, Some(bt)) if bt.status() == std::backtrace::BacktraceStatus::Captured => {
                Some(format!("\n{bt}"))
            }
            (None, _) => None,
        }
    }

    /// Returns the hint line of this error report, if there is one
    pub fn hint(&self) -> Option<&str> {
        self.inner.hint()
    }

    /// Returns the domain of this error report, if set
    pub fn domain(&self) -> Option<&str> {
        self.inner.domain()
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
        self.inner.location.funcname.as_deref()
    }

    /// Returns the context message of this error report, if any
    fn context_message(&self) -> Option<String> {
        // NB:  holding this here for future use
        None
    }
}

impl ErrorReport {
    /// Create an [ErrorReport] which can be raised via Rust's [std::panic::panic_any()] or as
    /// a specific Postgres "ereport()` level via [ErrorReport::unwrap_or_report(self, PgLogLevel)]
    ///
    /// Embedded "file:line:col" location information is taken from the caller's location
    #[track_caller]
    pub fn new<S: Into<Cow<'static, str>>>(
        sqlerrcode: PgSqlErrorCode,
        message: S,
        funcname: &'static str,
    ) -> Self {
        let mut location: ErrorReportLocation = Location::caller().into();
        location.funcname = Some(funcname.to_string());

        Self {
            sqlerrcode,
            message: message.into(),
            hint: None,
            detail: None,
            domain: None,
            location,
        }
    }

    /// Create an [ErrorReport] which can be raised via Rust's [std::panic::panic_any()] or as
    /// a specific Postgres "ereport()` level via [ErrorReport::unwrap_or_report(self, PgLogLevel)].
    ///
    /// For internal use only
    fn with_location<S: Into<Cow<'static, str>>>(
        sqlerrcode: PgSqlErrorCode,
        message: S,
        location: ErrorReportLocation,
    ) -> Self {
        Self {
            sqlerrcode,
            message: message.into(),
            hint: None,
            detail: None,
            domain: None,
            location,
        }
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

    /// Set the `domain` property for message translation/internationalization, whose default is `None`.
    ///
    /// This corresponds to the `domain` argument in Postgres' `ereport_domain` C macro.
    /// When set, the domain is passed to `errstart()` to indicate the gettext text domain
    /// for message translation.
    pub fn set_domain<S: Into<String>>(mut self, domain: S) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// Returns the error message of this error report
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the detail message of this error report
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    /// Returns the hint message of this error report
    pub fn hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }

    /// Returns the domain of this error report, if set
    pub fn domain(&self) -> Option<&str> {
        self.domain.as_deref()
    }

    /// Report this [ErrorReport], which will ultimately be reported by Postgres at the specified [PgLogLevel]
    ///
    /// If the provided `level` is >= [`PgLogLevel::ERROR`] this function will not return.
    pub fn report(self, level: PgLogLevel) {
        ErrorReportWithLevel { level, inner: self, stack_entry: None }.report()
    }
}

thread_local! { static PANIC_LOCATION: Cell<Option<ErrorReportLocation>> = const { Cell::new(None) }}

fn take_panic_location() -> ErrorReportLocation {
    PANIC_LOCATION.with(|p| p.take().unwrap_or_default())
}

pub fn register_pg_guard_panic_hook() {
    use super::thread_check::is_os_main_thread;

    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info: _| {
        // Always capture the backtrace into PANIC_LOCATION so that
        // downcast_panic_payload can attach it to CaughtError::PostgresError.
        // This is a thread-local, so it's harmless on non-main threads.
        PANIC_LOCATION.with(|thread_local| {
            thread_local.replace({
                let mut info: ErrorReportLocation = info.into();
                info.backtrace = Some(std::backtrace::Backtrace::capture());
                Some(info)
            })
        });

        if is_os_main_thread() == Some(false) {
            // definitely not the main thread -- we don't know which connection
            // to associate the panic with, so also call the default hook
            default_hook(info)
        }
    }))
}

/// What kind of error was caught?
///
/// # Postgres errors
///
/// When pgrx catches a Postgres ERROR, the ERROR stays on Postgres' error data stack, as it does
/// for a C `PG_CATCH()` block, until something calls `FlushErrorState()`.  [`PgTryBuilder`] does
/// that when a catch handler returns, and Postgres does when it aborts the transaction.
///
/// [`CaughtError::rethrow()`] passes Postgres back its original ERROR, with every field, while
/// that ERROR is still on top of the stack.  Otherwise pgrx raises a new ERROR from its copy,
/// which has the same SQLSTATE, message, DETAIL, HINT, and location, takes its CONTEXT from where
/// it is raised, and lacks Postgres' other fields, such as the constraint name.
///
/// A [`CaughtError::PostgresError`] that is neither rethrown nor handled by [`PgTryBuilder`],
/// for example one caught by [`std::panic::catch_unwind()`] and dropped, keeps its stack entry.
/// Postgres has room for five entries and PANICs when an error needs a sixth, so call
/// [`crate::FlushErrorState()`] after swallowing one.  That and [`PgTryBuilder`] both empty the
/// whole stack, including an ERROR that C code which called into Rust is still handling, such as
/// one that a `PG_FINALLY()` block will rethrow.
///
/// [`PgTryBuilder`]: crate::PgTryBuilder
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
    /// This is the same as [std::panic::resume_unwind()] and has the same semantics.  See
    /// [Postgres errors](CaughtError#postgres-errors) for what Postgres receives when the unwind
    /// reaches a `#[pg_guard]` function.
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
/// Where it does need to be used is as a wrapper around Rust `extern "C-unwind"` function pointers
/// given to Postgres, and the `#[pg_guard]` macro takes care of this for you.
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
/// `extern "C-unwind" fn` implemented in Rust.
///
/// [trivially-deallocated stack frames](https://github.com/rust-lang/rfcs/blob/master/text/2945-c-unwind-abi.md#plain-old-frames)
#[doc(hidden)]
// FIXME: previously, R was bounded on Copy, but this prevents using move-only POD types
// what we really want is a bound of R: !Drop, but negative bounds don't exist yet
pub unsafe fn pgrx_extern_c_guard<Func, R>(f: Func) -> R
where
    Func: FnOnce() -> R,
{
    match unsafe { run_guarded(AssertUnwindSafe(f)) } {
        GuardAction::Return(r) => r,
        GuardAction::ReThrow => {
            #[cfg_attr(target_os = "windows", link(name = "postgres"))]
            unsafe extern "C-unwind" {
                fn pg_re_throw() -> !;
            }
            unsafe {
                crate::CurrentMemoryContext = crate::ErrorContext;
                pg_re_throw()
            }
        }
        GuardAction::Report(ereport) => {
            unsafe {
                do_ereport(ereport);
            }
            unreachable!("pgrx reported a CaughtError that wasn't raised at ERROR or above");
        }
    }
}

// SAFETY: similar constraints as pgrx_extern_c_guard
#[inline(never)]
unsafe fn run_guarded<F, R>(f: F) -> GuardAction<R>
where
    F: FnOnce() -> R + UnwindSafe,
{
    match catch_unwind(f) {
        Ok(v) => GuardAction::Return(v),
        Err(e) => match downcast_panic_payload(e) {
            // SAFETY: Postgres called the guarded function on the backend thread
            CaughtError::PostgresError(ereport)
                if ereport.stack_entry.as_ref().is_some_and(|entry| unsafe { entry.is_top() }) =>
            {
                // Return to the caller to rethrow the original Postgres ErrorData unchanged.  We
                // can't do it here because this function has non-POF frames.
                GuardAction::ReThrow
            }
            CaughtError::PostgresError(mut ereport) => {
                // `pg_re_throw()` would raise whatever is on top of the stack now, or read below
                // an empty one, so raise our copy instead.  Like a rethrow, it gets no backtrace
                ereport.inner.location.backtrace = None;
                GuardAction::Report(ereport)
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
        let mut caught = *e.downcast::<CaughtError>().unwrap();

        // For PostgresErrors (originating from a pg_sys FFI longjmp caught by
        // pg_guard_ffi_boundary), the panic hook captured a Rust backtrace into
        // PANIC_LOCATION.  Attach it now so PgTryBuilder catch handlers can inspect it without
        // changing the original Postgres error that will be rethrown.
        if let CaughtError::PostgresError(ref mut ereport) = caught {
            if ereport.inner.location.backtrace.is_none() {
                let panic_location = take_panic_location();
                ereport.inner.location.backtrace = panic_location.backtrace;
            }
        }

        caught
    } else if e.downcast_ref::<ErrorReportWithLevel>().is_some() {
        // someone called `panic_any(ErrorReportWithLevel)`
        CaughtError::ErrorReport(*e.downcast().unwrap())
    } else if e.downcast_ref::<ErrorReport>().is_some() {
        // someone called `panic_any(ErrorReport)` so we convert it to be PgLogLevel::ERROR
        CaughtError::ErrorReport(ErrorReportWithLevel {
            level: PgLogLevel::ERROR,
            inner: *e.downcast().unwrap(),
            stack_entry: None,
        })
    } else if let Some(message) = e.downcast_ref::<&str>() {
        // something panic'd with a &str, so it gets raised as an INTERNAL_ERROR at the ERROR level
        CaughtError::RustPanic {
            ereport: ErrorReportWithLevel {
                level: PgLogLevel::ERROR,
                inner: ErrorReport::with_location(
                    PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                    message.to_string(),
                    take_panic_location(),
                ),
                stack_entry: None,
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
                    message.clone(),
                    take_panic_location(),
                ),
                stack_entry: None,
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
                stack_entry: None,
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
unsafe fn do_ereport(ereport: ErrorReportWithLevel) {
    const PERCENT_S: &CStr = c"%s";
    const DEFAULT_DOMAIN: *const ::std::os::raw::c_char = std::ptr::null_mut();

    // the following code is definitely thread-unsafe -- not-the-main-thread can't be creating Postgres
    // ereports.  Our secret `extern "C"` definitions aren't wrapped by #[pg_guard] so we need to
    // manually do the active thread check
    crate::thread_check::check_active_thread();

    //
    // only declare these functions here.  They're explicitly excluded from bindings generation in
    // `build.rs` and we'd prefer pgrx users not have access to them at all
    //

    #[cfg_attr(target_os = "windows", link(name = "postgres"))]
    unsafe extern "C-unwind" {
        fn errcode(sqlerrcode: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
        fn errmsg(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
        fn errdetail(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
        fn errhint(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
        fn errcontext_msg(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    }

    // we only allocate file, lineno and funcname if `errstart` returns true
    #[cfg_attr(target_os = "windows", link(name = "postgres"))]
    unsafe extern "C-unwind" {
        fn errstart(elevel: ::std::os::raw::c_int, domain: *const ::std::os::raw::c_char) -> bool;
        fn errfinish(
            filename: *const ::std::os::raw::c_char,
            lineno: ::std::os::raw::c_int,
            funcname: *const ::std::os::raw::c_char,
        );
    }

    let level = ereport.level();

    // Use the domain from the ErrorReport if one was set, otherwise use the null default
    let domain_cstring = ereport.inner.domain.as_ref().map(|d| {
        std::ffi::CString::new(d.as_str()).expect("domain must not contain interior NUL bytes")
    });
    let domain_ptr = domain_cstring.as_ref().map_or(DEFAULT_DOMAIN, |c| c.as_ptr());

    unsafe {
        if errstart(level as _, domain_ptr) {
            let sqlerrcode = ereport.sql_error_code();
            let message = ereport.message().as_pg_cstr();
            let detail = ereport.detail_with_backtrace().as_pg_cstr();
            let hint = ereport.hint().as_pg_cstr();
            let context = ereport.context_message().as_pg_cstr();
            let lineno = ereport.line_number();

            // SAFETY:  We know that `crate::ErrorContext` is a valid memory context pointer and one
            // that Postgres will clean up for us in the event of an ERROR, and we know it'll live long
            // enough for Postgres to use `file` and `funcname`, which it expects to be `const char *`s

            let prev_cxt = crate::MemoryContextSwitchTo(crate::ErrorContext);
            let file = ereport.file().as_pg_cstr();
            let funcname = ereport.function_name().as_pg_cstr();
            crate::MemoryContextSwitchTo(prev_cxt);

            // do not leak the Rust `ErrorReportWithLocation` instance
            drop(ereport);

            // SAFETY
            //
            // The following functions are all FFI into Postgres, so they're inherently unsafe.
            //
            // The various pointers used as arguments to these functions might have been allocated above
            // or they might be the null pointer, so we guard against that possibility for each usage.
            errcode(sqlerrcode as _);
            if !message.is_null() {
                errmsg(PERCENT_S.as_ptr(), message);
                pfree(message.cast());
            }
            if !detail.is_null() {
                errdetail(PERCENT_S.as_ptr(), detail);
                pfree(detail.cast());
            }
            if !hint.is_null() {
                errhint(PERCENT_S.as_ptr(), hint);
                pfree(hint.cast());
            }
            if !context.is_null() {
                errcontext_msg(PERCENT_S.as_ptr(), context);
                pfree(context.cast());
            }

            if level >= PgLogLevel::ERROR {
                crate::submodules::thread_check::active_thread::clear();
            }
            errfinish(file, lineno as _, funcname);

            if level >= PgLogLevel::ERROR {
                // SAFETY:  `crate::errstart() is guaranteed to have returned true if >=ERROR and
                // `crate::errfinish()` is guaranteed to not have not returned at all if >= ERROR, which
                // means we won't either
                unreachable_unchecked()
            } else {
                // if it wasn't an ERROR we need to free up the things that Postgres wouldn't have
                //
                // ... except on Postgres 19, whose `errfinish()` resets `ErrorContext` (where we
                // allocated `file` and `funcname`) on its way out, making a `pfree()` here a
                // use-after-free.  In the nested-error case where pg19 doesn't reset the context,
                // these allocations just live in `ErrorContext` until it's eventually reset
                #[cfg(not(feature = "pg19"))]
                {
                    if !file.is_null() {
                        pfree(file.cast());
                    }
                    if !funcname.is_null() {
                        pfree(funcname.cast());
                    }
                }
            }
        }
    }
}
