#![allow(non_snake_case)]

use crate::pg_sys::{error_context_stack, PG_exception_stack};
use crate::{PgLogLevel, PgSqlErrorCode};
use std::any::Any;
use std::cell::Cell;
use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_void};
use std::panic::catch_unwind;
use std::sync::atomic::{compiler_fence, Ordering};
use std::thread::LocalKey;

extern "C" {
    fn siglongjmp(env: *mut crate::pg_sys::sigjmp_buf, val: c_int) -> c_void;
}

#[cfg(target_os = "linux")]
extern "C" {
    #[link_name = "__sigsetjmp"]
    fn sigsetjmp(env: *mut crate::pg_sys::sigjmp_buf, savemask: c_int) -> c_int;
}

#[cfg(target_os = "macos")]
extern "C" {
    fn sigsetjmp(env: *mut crate::pg_sys::sigjmp_buf, savemask: c_int) -> c_int;
}

#[derive(Clone)]
struct JumpContext {
    jump_value: c_int,
}

#[derive(Debug, Clone, Copy)]
pub struct PgxPanic {
    pub message: &'static str,
    pub filename: &'static str,
    pub lineno: u32,
    pub colno: u32,
}

impl PgxPanic {
    pub fn new(message: &'static str, filename: &'static str, lineno: u32, colno: u32) -> Self {
        PgxPanic {
            message,
            filename,
            lineno,
            colno,
        }
    }
}

struct PanicLocation {
    file: String,
    line: u32,
    col: u32,
}

thread_local! { static PANIC_LOCATION: Cell<Option<PanicLocation>> = Cell::new(None) }

fn take_panic_location() -> PanicLocation {
    PANIC_LOCATION.with(|p| match p.take() {
        Some(location) => location,

        // this case shouldn't happen
        None => PanicLocation {
            file: "<unknown>".to_string(),
            line: 0,
            col: 0,
        },
    })
}

pub(crate) fn register_pg_guard_panic_handler() {
    std::panic::set_hook(Box::new(|info| {
        PANIC_LOCATION.with(|p| {
            let existing = p.take();

            p.replace(if existing.is_none() {
                match info.location() {
                    Some(location) => Some(PanicLocation {
                        file: location.file().to_string(),
                        line: location.line(),
                        col: location.column(),
                    }),
                    None => None,
                }
            } else {
                existing
            })
        });
    }))
}

#[inline]
fn inc_depth(depth: &'static LocalKey<Cell<usize>>) {
    depth.with(|depth| depth.replace(depth.get() + 1));
}

#[inline]
fn dec_depth(depth: &'static LocalKey<Cell<usize>>) {
    depth.with(|depth| depth.replace(depth.get() - 1));
}

#[inline]
fn get_depth(depth: &'static LocalKey<Cell<usize>>) -> usize {
    depth.with(|depth| depth.get())
}

#[inline]
pub fn guard<R, F: Fn() -> R>(f: F) -> R
where
    F: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    thread_local! { static DEPTH: Cell<usize> = Cell::new(0) }

    let result = unsafe {
        // remember where Postgres would like to jump to
        let prev_exception_stack = PG_exception_stack;
        let prev_error_context_stack = error_context_stack;

        //
        // setup the longjmp context and run our wrapped function inside
        // a catch_unwind() block
        //
        // we do this so that we can catch any panic!() that might occur
        // in the wrapped function, including those we generate in response
        // to an elog(ERROR) via longjmp
        //
        let result = catch_unwind(|| {
            let mut jmp_buff = MaybeUninit::uninit();

            // set a jump point so that should the wrapped function execute an
            // elog(ERROR), it'll longjmp back here, instead of somewhere inside Postgres
            compiler_fence(Ordering::SeqCst);
            let jump_value = sigsetjmp(jmp_buff.as_mut_ptr(), 0);

            if jump_value != 0 {
                // caught an elog(ERROR), so convert it into a panic!()
                panic!(JumpContext { jump_value });
            }

            // lie to Postgres about where it should jump when it does an elog(ERROR)
            PG_exception_stack = jmp_buff.as_mut_ptr();

            // run our wrapped function and return its result
            inc_depth(&DEPTH);
            f()
        });

        // restore Postgres' understanding of where it should longjmp
        dec_depth(&DEPTH);
        PG_exception_stack = prev_exception_stack;
        error_context_stack = prev_error_context_stack;

        // return our result -- it could be Ok(), or it could be an Err()
        result
    };

    match result {
        // the result is Ok(), so just return it
        Ok(result) => result,

        // the result is an Err(), which means we caught a panic!() up above in catch_rewind()
        // if we're at nesting depth zero then we'll report it to Postgres, otherwise we'll
        // simply rethrow it
        Err(e) => {
            if get_depth(&DEPTH) == 0 {
                let location = take_panic_location();

                // we're not wrapping a function
                match downcast_err(e) {
                    // the error is a String, which means it was originally a Rust panic!(), so
                    // translate it into an elog(ERROR), including the code location that caused
                    // the panic!()
                    Ok(message) => {
                        crate::log::ereport(
                            PgLogLevel::ERROR,
                            PgSqlErrorCode::ERRCODE_INTERNAL_ERROR,
                            &message,
                            &location.file,
                            location.line,
                            location.col,
                        );
                        unreachable!("ereport() failed at depth==0 with message: {}", message);
                    }

                    // the error is a JumpContext, so we need to longjmp back into Postgres
                    Err(jump_context) => unsafe {
                        compiler_fence(Ordering::SeqCst);
                        siglongjmp(PG_exception_stack, jump_context.jump_value);
                        unreachable!("siglongjmp failed");
                    },
                }
            } else {
                // we're at least one level deep in nesting so rethrow the panic!()
                rethrow_panic(e)
            }
        }
    }
}

/// rethrow whatever the `e` error is as a Rust `panic!()`
fn rethrow_panic(e: Box<dyn Any + Send>) -> ! {
    match downcast_err(e) {
        Ok(message) => panic!(message),
        Err(jump_context) => panic!(jump_context),
    }
}

/// convert types of `e` that we understand/expect into either a
/// `Ok(String)` or a `Err<JumpContext>`
fn downcast_err(e: Box<dyn Any + Send>) -> Result<String, JumpContext> {
    if let Some(cxt) = e.downcast_ref::<JumpContext>() {
        Err(cxt.clone())
    } else if let Some(s) = e.downcast_ref::<&str>() {
        Ok((*s).to_string())
    } else if let Some(s) = e.downcast_ref::<String>() {
        Ok(s.to_string())
    } else if let Some(s) = e.downcast_ref::<PgxPanic>() {
        Ok(format!(
            "{}: {}:{}:{}",
            s.message, s.filename, s.lineno, s.colno
        ))
    } else {
        // not a type we understand, so use a generic string
        Ok("Box<Any>".to_string())
    }
}
