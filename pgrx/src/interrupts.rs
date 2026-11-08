//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Servicing pending PostgreSQL interrupts from safe code.

use crate::pg_sys;

/// Service a pending query cancel, backend termination, or other interrupt, as PostgreSQL's
/// `CHECK_FOR_INTERRUPTS()` does.
///
/// Call this in long loops so that a cancel or `pg_terminate_backend()` takes effect. When an
/// interrupt needs one, PostgreSQL raises an ERROR or FATAL, which reaches Rust as a panic that
/// unwinds to the nearest `#[pg_guard]` boundary. Do not call it while holding a
/// [`PgSpinLock`](crate::spinlock::PgSpinLock): PostgreSQL does not release a spinlock when it raises an
/// error.
///
/// # Panics
///
/// Panics if called from any thread other than the one pgrx treats as the active Postgres thread,
/// before reading any backend state.
#[inline]
pub fn check_for_interrupts() {
    pg_sys::submodules::thread_check::check_active_thread();
    // SAFETY: the check above establishes that this is the backend thread, so reading
    // `InterruptPending` does not race with its signal handlers and `ProcessInterrupts` may run.
    // pgrx's FFI guard turns an ERROR it raises into a Rust panic, which safe code must already
    // tolerate.
    #[allow(unused_unsafe, reason = "documentation snapshot shells need no unsafe block")]
    unsafe {
        pg_sys::CHECK_FOR_INTERRUPTS!();
    }
}
