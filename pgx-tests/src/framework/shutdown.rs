/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use std::sync::{Mutex, PoisonError};

/// Register a shutdown hook to be called when the process exits.
///
/// Note that shutdown hooks are only run on the client, so must be added from
/// your `setup` function, not the `#[pg_test]` itself.
#[track_caller]
pub fn add_shutdown_hook<F: FnOnce()>(func: F)
where
    F: Send + 'static,
{
    SHUTDOWN_HOOKS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .push(ShutdownHook { source: core::panic::Location::caller(), callback: Box::new(func) });
}

pub(super) fn register_shutdown_hook() {
    unsafe {
        libc::atexit(run_shutdown_hooks);
    }
}

/// The `atexit` callback.
///
/// If we panic from `atexit`, we end up causing `exit` to unwind. Unwinding
/// from a nounwind + noreturn function can cause some destructors to run twice,
/// causing (for example) libtest to SIGSEGV.
///
/// This ends up looking like a memory bug in either `pgx` or the user code, and
/// is very hard to track down, so we go to some lengths to prevent it.
/// Essentially:
///
/// - Panics in each user hook are caught and reported.
/// - As a stop-gap a abort-on-drop panic guard is used to ensure there isn't a
///   place we missed.
///
/// We also write to stderr directly instead, since otherwise our output will
/// sometimes be redirected.
extern "C" fn run_shutdown_hooks() {
    let guard = PanicGuard;
    let mut any_panicked = false;
    let mut hooks = SHUTDOWN_HOOKS.lock().unwrap_or_else(PoisonError::into_inner);
    // Note: run hooks in the opposite order they were registered.
    for hook in std::mem::take(&mut *hooks).into_iter().rev() {
        any_panicked |= hook.run().is_err();
    }
    if any_panicked {
        write_stderr("error: one or more shutdown hooks panicked (see `stderr` for details).\n");
        std::process::abort()
    }
    core::mem::forget(guard);
}

/// Prevent panics in a block of code.
///
/// Prints a message and aborts in its drop. Intended usage is like:
/// ```ignore
/// let guard = PanicGuard;
/// // ...code that absolutely must never unwind goes here...
/// core::mem::forget(guard);
/// ```
struct PanicGuard;
impl Drop for PanicGuard {
    fn drop(&mut self) {
        write_stderr("Failed to catch panic in the `atexit` callback, aborting!\n");
        std::process::abort();
    }
}

static SHUTDOWN_HOOKS: Mutex<Vec<ShutdownHook>> = Mutex::new(Vec::new());

struct ShutdownHook {
    source: &'static core::panic::Location<'static>,
    callback: Box<dyn FnOnce() + Send>,
}

impl ShutdownHook {
    fn run(self) -> Result<(), ()> {
        let Self { source, callback } = self;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(callback));
        if let Err(e) = result {
            let msg = failure_message(&e);
            write_stderr(&format!(
                "error: shutdown hook (registered at {source}) panicked: {msg}\n"
            ));
            Err(())
        } else {
            Ok(())
        }
    }
}

fn failure_message(e: &(dyn std::any::Any + Send)) -> &str {
    if let Some(&msg) = e.downcast_ref::<&'static str>() {
        msg
    } else if let Some(msg) = e.downcast_ref::<String>() {
        msg.as_str()
    } else {
        "<panic payload of unknown type>"
    }
}

// Write to stderr, bypassing libtest's output redirection. Doesn't append `\n`.
fn write_stderr(s: &str) {
    loop {
        let res = unsafe { libc::write(libc::STDERR_FILENO, s.as_ptr().cast(), s.len()) };
        // Handle EINTR to ensure we don't drop messages.
        // `Error::last_os_error()` just reads from errno, so it's fine to use
        // here.
        if res >= 0 || std::io::Error::last_os_error().kind() != std::io::ErrorKind::Interrupted {
            break;
        }
    }
}
