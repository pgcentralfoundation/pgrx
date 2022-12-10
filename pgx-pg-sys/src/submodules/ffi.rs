#![deny(unsafe_op_in_unsafe_fn)]

/**
Given a closure that is assumed to be a wrapped Postgres `extern "C"` function, [pg_guard_ffi_boundary]
works with the Postgres and C runtimes to create a "barrier" that allows Rust to catch Postgres errors
(`elog(ERROR)`) while running the supplied closure. This is done for the sake of allowing Rust to run
destructors before Postgres destroys the memory contexts that Rust-in-Postgres code may be enmeshed in.

Wrapping the FFI into Postgres enables
- memory safety
- improving error logging
- minimizing resource leaks

But only the first of these is considered paramount.

At all times PGX reserves the right to choose an implementation that achieves memory safety.
Currently, this function is used to protect **every** bindgen-generated Postgres `extern "C"` function.

Generally, the only time *you'll* need to use this function is when calling a Postgres-provided
function pointer.

# Safety

Postgres is a single-threaded runtime.  As such, [`pg_guard_ffi_boundary`] should **only** be called
from the main thread.  In fact, [`pg_guard_ffi_boundary`] will detect this and immediately panic.

More generally, Rust cannot guarantee destructors are always run, PGX is written in Rust code, and
the implementation of `pg_guard_ffi_boundary` relies on help from Postgres, the OS, and the C runtime;
thus, relying on the FFI boundary catching an error and propagating it back into Rust to guarantee
Rust's language-level memory safety when calling Postgres is unsound (i.e. there are no promises).
Postgres can and does opt to erase exception and error context stacks in some situations.
The C runtime is beholden to the operating system, which may do as it likes with a thread.
PGX has many magical powers, some of considerable size, but they are not infinite cosmic power.

Thus, if Postgres gives you a pointer into the database's memory, and you corrupt that memory
in a way technically permitted by Rust, intending to fix it before Postgres or Rust notices,
then you may not call Postgres and expect Postgres to not notice the code crimes in progress.
Postgres and Rust will see you. Whether they choose to ignore such misbehavior is up to them, not PGX.
If you are manipulating transient "pure Rust" data, however, it is unlikely this is of consequence.

# Implementation Note

The main implementation uses `sigsetjmp`, [`pg_sys::error_context_stack`], and [`pg_sys::PG_exception_stack`].
which, when Postgres enters its exception handling in `elog.c`, will prompt a `siglongjmp` back to it.

This caught error is then converted into a Rust `panic!()` and propagated up the stack, ultimately
being converted into a transaction-aborting Postgres `ERROR` by PGX.

**/
#[inline(always)]
#[track_caller]
pub unsafe fn pg_guard_ffi_boundary<T, F: FnOnce() -> T>(f: F) -> T {
    // SAFETY: Caller promises not to call us from anything but the main thread.
    unsafe { pg_guard_ffi_boundary_impl(f) }
}

#[cfg(not(feature = "postgrestd"))]
#[inline(always)]
#[track_caller]
unsafe fn pg_guard_ffi_boundary_impl<T, F: FnOnce() -> T>(f: F) -> T {
    //! This is the version that uses sigsetjmp and all that, for "normal" Rust/PGX interfaces.
    use crate as pg_sys;

    // just use these here to avoid compilation warnings when #[cfg(feature = "postgrestd")] is on
    use crate::panic::{CaughtError, ErrorReport, ErrorReportLocation, ErrorReportWithLevel};
    use std::ffi::CStr;

    // The next code is definitely thread-unsafe (it manipulates statics in an
    // unsynchronized manner), so we may as well check here.
    super::thread_check::check_active_thread();

    // SAFETY: This should really, really not be done in a multithreaded context as it
    // accesses multiple `static mut`. The ultimate caller asserts this is the main thread.
    unsafe {
        let caller_memxct = pg_sys::CurrentMemoryContext;
        let prev_exception_stack = pg_sys::PG_exception_stack;
        let prev_error_context_stack = pg_sys::error_context_stack;
        let mut jump_buffer = std::mem::MaybeUninit::uninit();
        let jump_value = crate::sigsetjmp(jump_buffer.as_mut_ptr(), 0);

        if jump_value == 0 {
            // first time through, not as the result of a longjmp
            pg_sys::PG_exception_stack = jump_buffer.as_mut_ptr();

            // execute the closure, which will be a wrapped internal Postgres function
            let result = f();

            // restore Postgres' understanding of where its next longjmp should go
            pg_sys::PG_exception_stack = prev_exception_stack;
            pg_sys::error_context_stack = prev_error_context_stack;

            return result;
        } else {
            // we're back here b/c of a longjmp originating in Postgres

            // the overhead to get the current [ErrorData] from Postgres and convert
            // it into our [ErrorReportWithLevel] seems worth the user benefit
            //
            // Note that this only happens in the case of us trapping an error

            // At this point, we're running within `pg_sys::ErrorContext`, but should be in the
            // memory context the caller was in before we call [CopyErrorData()] and start using it
            pg_sys::CurrentMemoryContext = caller_memxct;

            // SAFETY: `pg_sys::CopyErrorData()` will always give us a valid pointer, so just assume so
            let errdata_ptr = pg_sys::CopyErrorData();
            let errdata = errdata_ptr.as_ref().unwrap_unchecked();

            // copy out the fields we need to support pgx' error handling
            let level = errdata.elevel.into();
            let sqlerrcode = errdata.sqlerrcode.into();
            let message = errdata
                .message
                .is_null()
                .then(|| String::from("<null error message>"))
                .unwrap_or_else(|| CStr::from_ptr(errdata.message).to_string_lossy().to_string());
            let detail = errdata.detail.is_null().then(|| None).unwrap_or_else(|| {
                Some(CStr::from_ptr(errdata.detail).to_string_lossy().to_string())
            });
            let hint = errdata.hint.is_null().then(|| None).unwrap_or_else(|| {
                Some(CStr::from_ptr(errdata.hint).to_string_lossy().to_string())
            });
            let funcname = errdata.funcname.is_null().then(|| None).unwrap_or_else(|| {
                Some(CStr::from_ptr(errdata.funcname).to_string_lossy().to_string())
            });
            let file =
                errdata.filename.is_null().then(|| String::from("<null filename>")).unwrap_or_else(
                    || CStr::from_ptr(errdata.filename).to_string_lossy().to_string(),
                );
            let line = errdata.lineno as _;

            // clean up after ourselves by freeing the result of [CopyErrorData] and restoring
            // Postgres' understanding of where its next longjmp should go
            pg_sys::FreeErrorData(errdata_ptr);
            pg_sys::PG_exception_stack = prev_exception_stack;
            pg_sys::error_context_stack = prev_error_context_stack;

            // finally, turn this Postgres error into a Rust panic so that we can ensure proper
            // Rust stack unwinding and also defer handling until later
            std::panic::panic_any(CaughtError::PostgresError(ErrorReportWithLevel {
                level,
                inner: ErrorReport {
                    sqlerrcode,
                    message,
                    detail,
                    hint,
                    location: ErrorReportLocation { file, funcname, line, col: 0 },
                },
            }))
        }
    }
}

#[cfg(feature = "postgrestd")]
#[inline(always)]
unsafe fn pg_guard_ffi_boundary_impl<T, F: FnOnce() -> T>(f: F) -> T {
    /*  As "postgrestd", we don't have to do anything because we are simply assuming that it is okay
        to allow Postgres to deinitialize everything the way that Postgres likes it, because Rust
        assumes the operating system deciding to clean up Rust threads is acceptable behavior.

        In this context, the "operating system" is "Postgres as supervising runtime".
    */
    f()
}
