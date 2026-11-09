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

use core::ffi::{CStr, c_char, c_int, c_void};
use std::any::Any;
use std::borrow::Cow;
use std::cell::Cell;
use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::hint::unreachable_unchecked;
use std::panic::{
    AssertUnwindSafe, Location, PanicHookInfo, UnwindSafe, catch_unwind, panic_any, resume_unwind,
};
use std::ptr::{self, NonNull};
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
    /// The Postgres ERROR this report was made from, if pgrx caught one
    pub(crate) caught: Option<Box<CaughtPostgresError>>,
}

/// What pgrx keeps of a Postgres ERROR it caught, so that a `#[pg_guard]` function can pass the
/// ERROR back to Postgres unchanged.
///
/// Postgres gives the entries on its error data stack no identity, so when pgrx catches an ERROR,
/// it puts a [`Mark`] at the head of `ErrorContext`'s reset callbacks.  Only two things move the
/// ERROR off the top of the stack: a reset of `ErrorContext`, which `FlushErrorState()` does and
/// which empties the list, and a newer ERROR, which pgrx marks the same way when it catches it.
/// So while the mark heads the list, the ERROR is the top of the stack, unless code that doesn't
/// mark what it catches keeps a newer ERROR without flushing it.  That code is C, an extension
/// built with an older pgrx, or another extension's pgrx that had no memory for its mark.
#[derive(Debug)]
pub(crate) struct CaughtPostgresError {
    data: ErrorDataCopy,
    /// Tells this catch from every other in this library
    token: usize,
}

/// What pgrx registers on `ErrorContext` for a caught ERROR
#[repr(C)]
struct Mark {
    callback: crate::MemoryContextCallback,
    token: usize,
}

/// Counts this library's catches.  Its address, as a mark's callback argument, tells this
/// library's marks from other callbacks, including other libraries' marks.
static TOKENS: AtomicUsize = AtomicUsize::new(0);

fn mark_owner() -> *mut c_void {
    (&raw const TOKENS).cast_mut().cast()
}

/// A mark's callback.  Its place in the list is what counts, so it has nothing to do.
unsafe extern "C-unwind" fn forget_mark(_owner: *mut c_void) {}

/// This library's mark at the head of `ErrorContext`'s reset callbacks, if it's there
///
/// # Safety
///
/// Call only on the backend thread.
unsafe fn head_mark() -> Option<*mut Mark> {
    // SAFETY: ErrorContext exists once Postgres can raise an ERROR.  The head of its callback
    // list is null or registered and allocated, and only this library registers a callback
    // whose argument is `mark_owner()`, always at the start of a `Mark`.
    unsafe {
        let head = (*crate::ErrorContext).reset_cbs;
        (!head.is_null() && (*head).arg == mark_owner()).then(|| head.cast())
    }
}

impl CaughtPostgresError {
    /// Copies and marks the ERROR on top of Postgres' error data stack, and returns the report
    /// that catch handlers see for it.
    ///
    /// # Safety
    ///
    /// Call only on the backend thread, right after pgrx catches the ERROR and restores Postgres'
    /// exception and error context stacks.  `CopyErrorData()` asserts that `CurrentMemoryContext`
    /// isn't `ErrorContext`, which it is when the ERROR is caught in an error context callback.
    pub(crate) unsafe fn catch() -> ErrorReportWithLevel {
        let token = TOKENS.fetch_add(1, Ordering::Relaxed);
        // SAFETY: the caller just caught an ERROR, which `CopyErrorData()` copies into the
        // current memory context with each string null or NUL-terminated
        let data = unsafe {
            let errdata = crate::CopyErrorData();
            let data = ErrorDataCopy::new(&*errdata);
            crate::FreeErrorData(errdata);
            data
        };

        // SAFETY: as above.  The mark is allocated in ErrorContext, and Postgres removes it from
        // the list and calls it before the reset frees it.  Its fields are written before it's
        // registered.
        unsafe {
            let mark: *mut Mark = crate::MemoryContextAllocExtended(
                crate::ErrorContext,
                size_of::<Mark>(),
                crate::MCXT_ALLOC_NO_OOM as _,
            )
            .cast();
            if mark.is_null() {
                // a mark left at the head would claim this ERROR's place on the stack
                if let Some(head) = head_mark() {
                    (*head).callback.arg = ptr::null_mut();
                }
            } else {
                let callback = crate::MemoryContextCallback {
                    func: Some(forget_mark),
                    arg: mark_owner(),
                    next: ptr::null_mut(),
                };
                mark.write(Mark { callback, token });
                crate::MemoryContextRegisterResetCallback(crate::ErrorContext, mark.cast());
            }
        }

        ErrorReportWithLevel {
            level: data.elevel.into(),
            inner: data.report(),
            caught: Some(Box::new(Self { data, token })),
        }
    }

    /// How a `#[pg_guard]` function passes this ERROR back to Postgres.  `pg_re_throw()` raises
    /// whatever is on top of the error data stack, so it's right only while the original is
    /// there, and otherwise the copy is raised.  Returns `None` when `ErrorContext` has no memory
    /// left for the copy.
    ///
    /// # Safety
    ///
    /// Call only on the backend thread.
    unsafe fn guard_action<R>(&self) -> Option<GuardAction<R>> {
        // SAFETY: the caller is on the backend thread
        unsafe {
            if head_mark().is_some_and(|mark| (*mark).token == self.token) {
                Some(GuardAction::ReThrow)
            } else {
                self.data.in_error_context().map(GuardAction::ReThrowCopy)
            }
        }
    }
}

/// A Rust-owned copy of every field of a caught ERROR's `ErrorData`, which outlives any memory
/// context reset, so pgrx can raise the ERROR as it was after Postgres has discarded it.
#[derive(Debug)]
struct ErrorDataCopy {
    elevel: c_int,
    output_to_server: bool,
    output_to_client: bool,
    hide_stmt: bool,
    hide_ctx: bool,
    lineno: c_int,
    sqlerrcode: c_int,
    cursorpos: c_int,
    internalpos: c_int,
    saved_errno: c_int,
    strings: ErrorDataStrings,
}

impl ErrorDataCopy {
    /// # Safety
    ///
    /// Each of `edata`'s strings is null or NUL-terminated.
    unsafe fn new(edata: &crate::ErrorData) -> Self {
        Self {
            elevel: edata.elevel,
            output_to_server: edata.output_to_server,
            output_to_client: edata.output_to_client,
            hide_stmt: edata.hide_stmt,
            hide_ctx: edata.hide_ctx,
            lineno: edata.lineno,
            sqlerrcode: edata.sqlerrcode,
            cursorpos: edata.cursorpos,
            internalpos: edata.internalpos,
            saved_errno: edata.saved_errno,
            // SAFETY: the caller vouches for the strings
            strings: unsafe { ErrorDataStrings::new(edata) },
        }
    }

    /// The report that catch handlers see, with its strings decoded as UTF-8
    fn report(&self) -> ErrorReport {
        let lossy = |s: &Option<CString>| s.as_deref().map(|s| s.to_string_lossy().into_owned());
        let strings = &self.strings;
        ErrorReport {
            sqlerrcode: self.sqlerrcode.into(),
            message: lossy(&strings.message)
                .unwrap_or_else(|| "<null error message>".into())
                .into(),
            hint: lossy(&strings.hint),
            detail: lossy(&strings.detail),
            domain: lossy(&strings.domain),
            location: ErrorReportLocation {
                file: lossy(&strings.filename).unwrap_or_else(|| "<null filename>".into()),
                funcname: lossy(&strings.funcname),
                line: self.lineno as _,
                col: 0,
                backtrace: None,
            },
        }
    }

    /// Lays the copy out in one `ErrorContext` allocation for `ReThrowError()`.  That copies the
    /// strings Postgres frees with an error, but keeps the others, such as `filename`, as they
    /// are, so they need memory that lasts until Postgres discards the error, which it does by
    /// resetting `ErrorContext`.  Returns `None` when `ErrorContext` has no memory left.
    ///
    /// # Safety
    ///
    /// Call only on the backend thread.
    unsafe fn in_error_context(&self) -> Option<NonNull<crate::ErrorData>> {
        let size = size_of::<crate::ErrorData>() + self.strings.len();
        // SAFETY: ErrorContext exists once Postgres can raise an ERROR.  The allocation is
        // MAXALIGNed, which suits ErrorData, and has room for it and then for the strings.  It
        // can't raise, which a guard that has already caught its panic relies on: NO_OOM returns
        // null instead, and HUGE accepts any size these strings can add up to.
        unsafe {
            let edata = NonNull::new(
                crate::MemoryContextAllocExtended(
                    crate::ErrorContext,
                    size,
                    (crate::MCXT_ALLOC_HUGE | crate::MCXT_ALLOC_NO_OOM) as _,
                )
                .cast::<crate::ErrorData>(),
            )?;
            let mut copy = crate::ErrorData {
                elevel: self.elevel,
                output_to_server: self.output_to_server,
                output_to_client: self.output_to_client,
                hide_stmt: self.hide_stmt,
                hide_ctx: self.hide_ctx,
                lineno: self.lineno,
                sqlerrcode: self.sqlerrcode,
                cursorpos: self.cursorpos,
                internalpos: self.internalpos,
                saved_errno: self.saved_errno,
                assoc_context: crate::ErrorContext,
                ..Default::default()
            };
            self.strings.write(&mut copy, edata.as_ptr().add(1).cast());
            edata.write(copy);
            Some(edata)
        }
    }
}

/// Declares the strings of an `ErrorData` that `ErrorDataCopy` keeps, so that copying them out
/// of Postgres and back in covers the same fields.
macro_rules! error_data_strings {
    ($($field:ident),* $(,)?) => {
        /// The strings of a caught ERROR's `ErrorData`, byte for byte, since they're in the
        /// server's encoding
        #[derive(Debug)]
        struct ErrorDataStrings {
            $($field: Option<CString>,)*
        }

        impl ErrorDataStrings {
            /// # Safety
            ///
            /// Each of `edata`'s strings is null or NUL-terminated.
            unsafe fn new(edata: &crate::ErrorData) -> Self {
                // SAFETY: the caller vouches for the strings
                unsafe { Self { $($field: owned_c_str(edata.$field),)* } }
            }

            /// The bytes the strings take with their NULs
            fn len(&self) -> usize {
                [$(&self.$field),*].into_iter().flatten().map(|s| s.as_bytes_with_nul().len()).sum()
            }

            /// Points `edata`'s strings at copies written from `buf` on.
            ///
            /// # Safety
            ///
            /// `buf` is valid for writes of `self.len()` bytes.
            unsafe fn write(&self, edata: &mut crate::ErrorData, mut buf: *mut c_char) {
                // SAFETY: the caller vouches for `buf`
                unsafe { $(edata.$field = write_c_str(&self.$field, &mut buf);)* }
            }
        }
    };
}

error_data_strings!(
    filename,
    funcname,
    domain,
    context_domain,
    message,
    detail,
    detail_log,
    hint,
    context,
    backtrace,
    message_id,
    schema_name,
    table_name,
    column_name,
    datatype_name,
    constraint_name,
    internalquery,
);

/// # Safety
///
/// `s` is null or NUL-terminated.
unsafe fn owned_c_str(s: *const c_char) -> Option<CString> {
    // SAFETY: the caller vouches for `s`
    (!s.is_null()).then(|| unsafe { CStr::from_ptr(s) }.to_owned())
}

/// Writes `s` and its NUL at `*buf` and moves `*buf` past them.  Returns where `s` was written,
/// or null for `None`.
///
/// # Safety
///
/// `*buf` is valid for writes of `s` and its NUL.
unsafe fn write_c_str(s: &Option<CString>, buf: &mut *mut c_char) -> *mut c_char {
    let Some(s) = s else { return ptr::null_mut() };
    let bytes = s.as_bytes_with_nul();
    let start = *buf;
    // SAFETY: the caller vouches for the room, and Rust's copy can't overlap Postgres' memory
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr().cast::<c_char>(), start, bytes.len());
        *buf = start.add(bytes.len());
    }
    start
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
        ErrorReportWithLevel { level, inner: self, caught: None }.report()
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
/// that when a catch handler returns from a [`CaughtError::PostgresError`], and Postgres does when
/// it recovers from an ERROR that no one caught, such as one that fails a statement.
///
/// [`CaughtError::rethrow()`] passes Postgres back its original ERROR while that ERROR is still on
/// top of the stack.  Otherwise pgrx raises the copy of every field that it made when it caught
/// the ERROR, so Postgres' error handlers, such as PL/pgSQL's `GET STACKED DIAGNOSTICS`, see the
/// same ERROR either way.
///
/// A [`CaughtError::PostgresError`] that is swallowed instead of rethrown, for example with
/// [`std::panic::catch_unwind()`], keeps its stack entry, and so does one that a Rust panic
/// replaced, for example when a catch handler panicked and a [`PgTryBuilder`] handled that panic.
/// Postgres has room for five entries and PANICs when an error needs a sixth, so call
/// [`crate::FlushErrorState()`] once you're done with such an error.  A flush empties the whole
/// stack, including an ERROR that C code which called into Rust is still handling, such as one
/// that a `PG_FINALLY()` block will rethrow.
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
    /// Rethrow the caught ERROR on top of Postgres' error data stack
    ReThrow,
    /// Rethrow this copy of a caught ERROR that's no longer on top of the stack
    ReThrowCopy(NonNull<crate::ErrorData>),
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
        GuardAction::ReThrowCopy(edata) => {
            // The bindings guard ReThrowError(), which would catch the ERROR it raises.  They
            // also disagree on its signature: bindgen sees `pg_attribute_noreturn()` through
            // PG17, but not PG18's `pg_noreturn`.
            #[allow(clashing_extern_declarations)]
            #[cfg_attr(target_os = "windows", link(name = "postgres"))]
            unsafe extern "C-unwind" {
                fn ReThrowError(edata: *mut crate::ErrorData) -> !;
            }
            // SAFETY: `edata` is a complete ERROR in ErrorContext, which keeps it until Postgres
            // discards the error, and this frame holds nothing that needs dropping
            unsafe { ReThrowError(edata.as_ptr()) }
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
            CaughtError::PostgresError(mut ereport) => {
                // Return to the caller to rethrow the Postgres ERROR unchanged.  We can't do it
                // here because this function has non-POF frames.
                //
                // SAFETY: Postgres called the guarded function on the backend thread
                let action =
                    ereport.caught.take().and_then(|caught| unsafe { caught.guard_action() });
                action.unwrap_or_else(|| {
                    // with no memory for a copy of every field, report the main ones.  Like a
                    // rethrow, the report gets no backtrace
                    ereport.inner.location.backtrace = None;
                    GuardAction::Report(ereport)
                })
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
            caught: None,
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
                caught: None,
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
                caught: None,
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
                caught: None,
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
