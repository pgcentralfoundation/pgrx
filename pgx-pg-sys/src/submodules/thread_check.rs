//! Enforces thread-safety in `pgx`.
//!
//! It's UB to call into Postgres functions from multiple threads. We handle
//! this by remembering the first thread to call into postgres functions, and
//! panicking if we're called from a different thread.
//!
//! This is called from the current crate from inside the setjmp shim, as that
//! code is *definitely* unsound to call in the presense of multiple threads.
//!
//! This is somewhat heavyhanded, and should be called from fewer places in the
//! future...

use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicUsize, Ordering};

#[track_caller]
pub(crate) fn check_active_thread() {
    static ACTIVE_THREAD: AtomicUsize = AtomicUsize::new(0);
    let current_thread = nonzero_thread_id();
    // Relaxed is sufficient as we're only interested in the effects on a single
    // atomic variable, and don't need synchronization beyond that.
    return match ACTIVE_THREAD.load(Ordering::Relaxed) {
        0 => init_active_thread(current_thread, &ACTIVE_THREAD),
        thread_id => {
            if current_thread.get() != thread_id {
                thread_id_check_failed();
            }
        }
    };
}

#[track_caller]
fn init_active_thread(tid: NonZeroUsize, atom: &AtomicUsize) {
    match atom.compare_exchange(0, tid.get(), Ordering::Relaxed, Ordering::Relaxed) {
        Ok(_) => {
            // All good!
        }
        Err(_) => {
            thread_id_check_failed();
        }
    }
}

#[cold]
#[inline(never)]
#[track_caller]
fn thread_id_check_failed() -> ! {
    panic!("`pgx` may not not be used from multiple threads.");
}

fn nonzero_thread_id() -> NonZeroUsize {
    // Returns the `addr()` of a thread local variable.
    //
    // For now this is reasonably efficient, but could be (substantially, for
    // our use) improved by using a pointer to the thread control block, which
    // can avoid going through `__tls_get_addr`.
    //
    // Sadly, doing that would require some fiddley platform-specific inline
    // assembly, which is enough of a pain that it's not worth bothering with
    // for now. That said, in the future if this becomes a performance issue,
    // that'd be a good fix.
    //
    // For examples of how to do this, see the how checks for cross-thread frees
    // on are implemented in thread-pooling allocators, ex:
    // - https://github.com/microsoft/mimalloc/blob/f2712f4a8f038a7fb4df2790f4c3b7e3ed9e219b/include/mimalloc-internal.h#L871
    // - https://github.com/mjansson/rpmalloc/blob/f56e2f6794eab5c280b089c90750c681679fde92/rpmalloc/rpmalloc.c#L774
    // and so on.
    std::thread_local! {
        static BYTE: u8 = const { 0 };
    }
    BYTE.with(|p: &u8| {
        // Note: Avoid triggering the `unstable_name_collisions` lint.
        let addr = sptr::Strict::addr(p as *const u8);
        // SAFETY: `&u8` is always nonnull, so it's address is always nonzero.
        unsafe { NonZeroUsize::new_unchecked(addr) }
    })
}
