#![allow(non_snake_case)]

pub mod log;

use libc::sigset_t;
use std::any::Any;
use std::cell::Cell;
use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_void};
use std::panic::catch_unwind;
use std::sync::atomic::{compiler_fence, Ordering};
use std::thread::LocalKey;

pub use pg_guard_attr::{pg_extern, pg_guard};

extern "C" {
    fn sigsetjmp(env: *mut sigjmp_buf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut sigjmp_buf, val: c_int) -> c_void;
    static mut PG_exception_stack: *mut sigjmp_buf;
}

#[repr(C)]
struct sigjmp_buf {
    __jmpbuf: [i64; 9],
    __mask_was_saved: c_int,
    __saved_mask: sigset_t,
}

#[derive(Clone)]
struct JumpContext {
    jump_value: c_int,
}

thread_local! { static PANIC_LOCATION: Cell<Option<String>> = Cell::new(None) }

fn take_panic_location() -> String {
    PANIC_LOCATION.with(|p| match p.take() {
        Some(s) => s,
        None => "<unknown>".to_string(),
    })
}

pub fn register_panic_handler() {
    std::panic::set_hook(Box::new(|info| {
        PANIC_LOCATION.with(|p| {
            let newval = Some(match p.take() {
                Some(s) => s,
                None => match info.location() {
                    Some(location) => format!("{}", location),
                    None => "<unknown>".to_string(),
                },
            });

            p.replace(newval)
        });
    }))
}

fn inc_depth(depth: &'static LocalKey<Cell<usize>>) {
    depth.with(|depth| depth.replace(depth.get() + 1));
}

fn dec_depth(depth: &'static LocalKey<Cell<usize>>) {
    depth.with(|depth| depth.replace(depth.get() - 1));
}

fn get_depth(depth: &'static LocalKey<Cell<usize>>) -> usize {
    depth.with(|depth| depth.get())
}

pub fn guard<R, F: FnOnce() -> R>(f: F) -> R
where
    F: std::panic::UnwindSafe,
{
    thread_local! { static DEPTH: Cell<usize> = Cell::new(0) }

    let result = unsafe {
        // remember where Postgres would like to jump to
        let prev_exception_stack = PG_exception_stack;

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
                    Ok(message) => error!("caught Rust panic at {}: {}", location, message),

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

///
/// rethrow whatever the `e` error is as a Rust `panic!()`
///
fn rethrow_panic(e: Box<dyn Any + Send>) -> ! {
    match downcast_err(e) {
        Ok(message) => panic!(message),
        Err(jump_context) => panic!(jump_context),
    }
}

///
/// convert types of `e` that we understand/expect into either a
/// `Ok(String)` or a `Err<JumpContext>`
///
fn downcast_err(e: Box<dyn Any + Send>) -> Result<String, JumpContext> {
    if let Some(cxt) = e.downcast_ref::<JumpContext>() {
        Err(cxt.clone())
    } else if let Some(s) = e.downcast_ref::<&str>() {
        Ok(s.to_string())
    } else if let Some(s) = e.downcast_ref::<String>() {
        Ok(s.to_string())
    } else {
        // not a type we understand, so use a generic string
        Ok("Box<Any>".to_string())
    }
}
