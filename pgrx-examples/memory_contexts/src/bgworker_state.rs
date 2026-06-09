//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! # Background worker state and `TopMemoryContext`
//!
//! A bgworker outlives transactions, queries, and even sessions. State that
//! must persist across iterations of its main loop CANNOT live in the current
//! per-transaction context — that context gets reset between transactions and
//! the state would be silently freed.
//!
//! The right home for long-lived bgworker state is `TopMemoryContext`, which
//! is never reset for the lifetime of the backend.
//!
//! This example registers a tiny worker that maintains a per-iteration counter
//! and logs every wake-up. It does NOT include `#[pg_test]` because driving a
//! bgworker reliably from inside the test framework requires infrastructure
//! outside the scope of this example. To see it run:
//!
//! ```text
//! # In ${PGRX_HOME}/data-17/postgresql.conf:
//! shared_preload_libraries = 'memory_contexts'
//! # Then:
//! cargo pgrx run pg17 memory_contexts
//! # Watch the postmaster log; you should see one line per 5s.
//! ```

use pgrx::PgMemoryContexts;
use pgrx::bgworkers::*;
use pgrx::prelude::*;
use std::time::Duration;

/// Called from `_PG_init`. Builds and `.load()`s the worker descriptor.
pub fn register_bgworker() {
    BackgroundWorkerBuilder::new("memory_contexts demo worker")
        .set_function("memory_contexts_worker_main")
        .set_library("memory_contexts")
        .set_argument(123i32.into_datum())
        .load();
}

#[pg_guard]
#[unsafe(no_mangle)]
pub extern "C-unwind" fn memory_contexts_worker_main(arg: pg_sys::Datum) {
    BackgroundWorker::attach_signal_handlers(SignalWakeFlags::SIGHUP | SignalWakeFlags::SIGTERM);

    // Round-trip the value passed via set_argument() — proves the builder's arg plumbing works and shows where you'd plug in worker-specific config (a queue id, a partition number, etc.).
    let init_arg = unsafe { i32::from_polymorphic_datum(arg, false, pg_sys::INT4OID) };
    log!("memory_contexts demo worker starting (arg={})", init_arg.unwrap_or(-1));

    // Allocate the long-lived counter under TopMemoryContext.
    let counter: *mut i64 = unsafe {
        PgMemoryContexts::TopMemoryContext.switch_to(|_ctx| {
            // 8 bytes, zeroed
            let p = pg_sys::palloc0(std::mem::size_of::<i64>()) as *mut i64;
            *p = 0;
            p
        })
    };

    while BackgroundWorker::wait_latch(Some(Duration::from_secs(5))) {
        // SAFETY: counter was allocated in TopMemoryContext above and is never freed for the lifetime of this backend.
        let n = unsafe {
            *counter += 1;
            *counter
        };
        log!("memory_contexts demo worker tick {n}");
    }

    log!("memory_contexts demo worker exiting");
}
