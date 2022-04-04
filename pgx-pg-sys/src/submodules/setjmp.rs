/// Given a closure that is assumed to be a wrapped Postgres `extern "C"` function, [pg_guard_ffi_boundary]
/// will setup intermediate `sigsetjmp` barrier to enable Rust to catch any `elog(ERROR)` Postgres
/// might raise while running the supplied closure.  
///
/// This caught error is then converted into a Rust `panic!()` and propagated up the stack, ultimately
/// being converted into a transaction-aborting Postgres `ERROR` by pgx.
///
/// Currently, this function is only used by pgx' generated Postgres bindings.  It is not (yet)
/// intended (or even necessary) for normal user code.
#[inline(always)]
pub(crate) unsafe fn pg_guard_ffi_boundary<T, F: FnOnce() -> T>(f: F) -> T {
    use crate as pg_sys;

    // as the panic message says, we can't call Postgres functions from threads
    // the value of IS_MAIN_THREAD gets set through the pg_module_magic!() macro
    #[cfg(debug_assertions)]
    if crate::submodules::guard::IS_MAIN_THREAD.with(|v| v.get().is_none()) {
        panic!("functions under #[pg_guard] cannot be called from threads");
    };

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
