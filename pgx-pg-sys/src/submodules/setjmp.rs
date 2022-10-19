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
Currently, this function is only used by PGX's generated Postgres bindings.
It is not (yet) intended (or even necessary) for normal user code.

# Safety

This function should not be called from any thread but the main thread if such ever may throw an exception,
on account of the postmaster ultimately being a single-threaded runtime.

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

The main implementation uses`sigsetjmp`, [`pg_sys::error_context_stack`], and [`pg_sys::PG_exception_stack`].
which, when Postgres enters its exception handling in `elog.c`, will prompt a `siglongjmp` back to it.

This caught error is then converted into a Rust `panic!()` and propagated up the stack, ultimately
being converted into a transaction-aborting Postgres `ERROR` by PGX.

**/
#[inline(always)]
pub(crate) unsafe fn pg_guard_ffi_boundary<T, F: FnOnce() -> T>(f: F) -> T {
    // SAFETY: Caller promises not to call us from anything but the main thread.
    unsafe { pg_guard_ffi_boundary_impl(f) }
}

#[cfg(not(feature = "postgrestd"))]
#[inline(always)]
unsafe fn pg_guard_ffi_boundary_impl<T, F: FnOnce() -> T>(f: F) -> T {
    //! This is the version that uses sigsetjmp and all that, for "normal" Rust/PGX interfaces.
    use crate as pg_sys;

    // SAFETY: This should really, really not be done in a multithreaded context as it
    // accesses multiple `static mut`. The ultimate caller asserts this is the main thread.
    unsafe {
        let prev_exception_stack = pg_sys::PG_exception_stack;
        let prev_error_context_stack = pg_sys::error_context_stack;
        let mut jump_buffer = std::mem::MaybeUninit::uninit();
        let jump_value = crate::sigsetjmp(jump_buffer.as_mut_ptr(), 0);

        let result = if jump_value == 0 {
            // first time through, not as the result of a longjmp
            pg_sys::PG_exception_stack = jump_buffer.as_mut_ptr();

            // execute the closure, which will be a wrapped internal Postgres function
            f()
        } else {
            // we're back here b/c of a longjmp originating in Postgres
            // as such, we need to put Postgres' understanding of its exception/error state back together
            pg_sys::PG_exception_stack = prev_exception_stack;
            pg_sys::error_context_stack = prev_error_context_stack;

            // and ultimately we panic
            std::panic::panic_any(pg_sys::JumpContext {});
        };

        pg_sys::PG_exception_stack = prev_exception_stack;
        pg_sys::error_context_stack = prev_error_context_stack;

        result
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
