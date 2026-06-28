//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Coalesced cache invalidation.
//!
//! The naive approach — `NOTIFY` once per changed row — falls apart on a bulk
//! `UPDATE`/`DELETE`: a statement touching a million rows would try to queue a
//! million notifications and overrun the async queue (`max_notify_queue_pages`).
//!
//! This module shows the correct pattern. A row-level trigger only *accumulates*
//! the affected coarse key (here, `category`) into a per-transaction set held in
//! backend-local memory. A single `PreCommit` transaction callback then emits one
//! `NOTIFY` per distinct key, just before the transaction commits. A million row
//! changes in one category collapse into a single notification. A matching
//! `Abort` callback drops the accumulated keys if the transaction rolls back, so
//! nothing leaks into the next transaction on this backend.
//!
//! This requires Rust/C-level access: hooking the transaction lifecycle and
//! holding state across trigger invocations is impossible from a plain SQL
//! trigger. `register_xact_callback` is pgrx's safe wrapper over
//! `RegisterXactCallback`; the callbacks it registers live only for the current
//! transaction, which is exactly the lifetime we want.

use pgrx::prelude::*;
use std::cell::{Cell, RefCell};
use std::collections::BTreeSet;

thread_local! {
    /// Distinct coarse keys dirtied by the current transaction. A Postgres backend is single-threaded, so a `thread_local` is effectively transaction/backend-local state. It is drained on commit and cleared on abort, so it never grows beyond the distinct keys of one transaction.
    static DIRTY: RefCell<BTreeSet<i64>> = RefCell::new(BTreeSet::new());
    /// Whether this transaction has already registered its end-of-transaction callbacks. Reset when either callback fires, so the next transaction on this backend re-arms.
    static ARMED: Cell<bool> = const { Cell::new(false) };
}

/// Record that `category` was touched, arming the end-of-transaction flush on  first call within a transaction.
fn mark_dirty(category: i64) {
    DIRTY.with(|d| d.borrow_mut().insert(category));

    if !ARMED.with(Cell::get) {
        ARMED.with(|a| a.set(true));
        // Emit the coalesced notifications immediately before commit...
        pgrx::register_xact_callback(pgrx::PgXactCallbackEvent::PreCommit, flush_dirty);
        // ...or throw the accumulated keys away if the transaction aborts.
        pgrx::register_xact_callback(pgrx::PgXactCallbackEvent::Abort, discard_dirty);
    }
}

/// `PreCommit`: emit exactly one `NOTIFY` per distinct dirtied key. Runs while the transaction is still open, which is required for `NOTIFY`.
fn flush_dirty() {
    ARMED.with(|a| a.set(false));
    // `take` drains and clears the set in one step.
    let categories = DIRTY.with(|d| std::mem::take(&mut *d.borrow_mut()));
    for category in categories {
        // One compact payload per key — the coalescing win — not one per row.
        let _ = crate::notify::notify("category_invalidation", &category.to_string());
    }
}

/// `Abort`: discard the accumulated keys so a rolled-back transaction does not leak dirty state into the next transaction on this backend.
fn discard_dirty() {
    ARMED.with(|a| a.set(false));
    DIRTY.with(|d| d.borrow_mut().clear());
}

/// Row trigger: accumulate the affected `category` from the new row (INSERT / UPDATE) and the old row (UPDATE / DELETE). An UPDATE that moves a row between categories dirties both. No `NOTIFY` happens here — that is deferred to commit.
#[pg_trigger]
fn inventory_coalesce<'a>(
    trigger: &'a pgrx::PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, impl WhoAllocated>>, Box<dyn std::error::Error>> {
    for tuple in [trigger.new(), trigger.old()] {
        if let Some(tuple) = tuple {
            if let Some(category) = tuple.get_by_name::<i64>("category")? {
                mark_dirty(category);
            }
        }
    }
    // AFTER row triggers ignore the return value.
    Ok(None::<PgHeapTuple<'a, pgrx::AllocatedByPostgres>>)
}

extension_sql!(
    r#"
CREATE TABLE inventory (
    id        bigserial NOT NULL PRIMARY KEY,
    sku       text NOT NULL,
    category  bigint NOT NULL
);

CREATE TRIGGER inventory_coalesce
    AFTER INSERT OR UPDATE OR DELETE ON inventory
    FOR EACH ROW EXECUTE PROCEDURE inventory_coalesce();
"#,
    name = "create_inventory_trigger",
    requires = [inventory_coalesce]
);
