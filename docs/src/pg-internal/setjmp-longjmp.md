# sigsetjmp & siglongjmp

In order to handle errors that may be distributed widely across the database and deeply nested,
Postgres uses `sigsetjmp` and `siglongjmp` in a certain "calling convention" to handle a stack
of error-handling steps. At a "try-catch" site, `sigsetjmp` is called, and at an error site,
`siglongjmp` is called, each time manipulating a global stack of error contexts to allow nested
try-catches. To address the fact that Rust code is preferably not jumped over, instead properly
handling its destructors via unwinding, pgrx guards calls into C with a function that handles the
global state and then panics. Likewise, Rust panics are hooked in ways that then propagate into
errors in Postgres.

<!--
TODO: Make the next statement slightly untrue by making it easier to call functions unsoundly so
that we can call certain functions in tight loops with only a single guard on the inner loop.
-->
The functions normally accessed via `pgrx::pg_sys` are `unsafe`, but they are less unsafe than
some C functions because they are wrapped by this guard. You do not need to worry about longjmp
when calling those. However, if you define your own `extern "C" fn` for *Postgres* to call, you
may need to apply `#[pg_guard]` in that case. If you do, try to limit the amount of code that lies
within the scope of that guard, as it is easy to make a mistake that makes this guard useless. The
code should not have any destructors. Each of the aforementioned FFI functions are a guard around
a *single* call, and that is an appropriate scope.

<!-- TODO: Provide more context on appropriate code, explain C-unwind a bit -->
