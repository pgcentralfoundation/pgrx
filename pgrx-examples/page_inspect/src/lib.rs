//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Real extension demonstrating the safe `pgrx::buffer::PgBuffer` wrapper.
//!
//! The three exported functions cover the whole public API surface:
//!
//! * `page_stats`           — pin + share-lock + header / max_offset / item iteration
//! * `page_first_item_len`  — bounds-checked item access
//! * `page_mark_dirty_try`  — non-blocking exclusive lock + mark_dirty
//! * `page_is_dirty`        — escape-hatch FFI via `PgBuffer::as_raw` (pg17+)

use pgrx::buffer::PgBuffer;
use pgrx::iter::TableIterator;
use pgrx::prelude::*;
use pgrx::rel::PgRelation;

::pgrx::pg_module_magic!(name, version);

/// Returns a one-row description of the requested page.
///
/// `valid_items` counts line pointers whose `Page::item` returns `Some` — i.e.
/// LP_NORMAL with in-bounds `(lp_off, lp_len)`. Corrupt or dead items are excluded thanks to the wrapper's bounds checks.
#[pg_extern]
fn page_stats(
    relname: &str,
    blockno: i32,
) -> TableIterator<
    'static,
    (
        name!(pd_lower, i32),
        name!(pd_upper, i32),
        name!(pd_special, i32),
        name!(max_offset, i32),
        name!(valid_items, i32),
    ),
> {
    let rel = PgRelation::open_with_name_and_share_lock(relname)
        .unwrap_or_else(|e| panic!("open_with_name_and_share_lock({relname}): {e}"));

    // SAFETY: `rel` lives until the end of this function and outlives `buf`.
    let mut buf = unsafe { PgBuffer::read(&rel, blockno as u32) };
    let guard = buf.read_lock();
    let page = guard.page();
    let header = guard.header();

    let max = page.max_offset();
    let valid = (1..=max).filter(|off| page.item(*off).is_some()).count() as i32;

    TableIterator::once((
        header.lower() as i32,
        header.upper() as i32,
        header.special() as i32,
        max as i32,
        valid,
    ))
}

/// Demonstrates the non-blocking exclusive-lock path. Returns `true` when the `ConditionalLockBuffer` call succeeded and we marked the page dirty.
#[pg_extern]
fn page_mark_dirty_try(relname: &str, blockno: i32) -> bool {
    let rel = PgRelation::open_with_name_and_share_lock(relname)
        .unwrap_or_else(|e| panic!("open_with_name_and_share_lock({relname}): {e}"));

    let mut buf = unsafe { PgBuffer::read(&rel, blockno as u32) };
    match buf.try_write_lock() {
        Some(mut g) => {
            g.mark_dirty();
            true
        }
        None => false,
    }
}

/// Returns whether the buffer for `(relname, blockno)` is currently marked dirty.
///
/// Uses `pg_sys::BufferIsDirty` (only available on pg17+). The buffer pin is held by [`PgBuffer`] while we call the raw FFI — that pin is what makes reading the dirty flag well-defined; the wrapper's `as_raw()` is the intentional escape hatch for callers who need APIs the wrapper hasn't surfaced yet (here: `is_dirty`).
#[cfg(any(feature = "pg17", feature = "pg18"))]
#[pg_extern]
fn page_is_dirty(relname: &str, blockno: i32) -> bool {
    let rel = PgRelation::open_with_name_and_share_lock(relname)
        .unwrap_or_else(|e| panic!("open_with_name_and_share_lock({relname}): {e}"));

    let buf = unsafe { PgBuffer::read(&rel, blockno as u32) };
    // SAFETY: `buf` owns the pin, so `BufferIsDirty(buf.as_raw())` is valid for the duration of this call.
    unsafe { pgrx::pg_sys::BufferIsDirty(buf.as_raw()) }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    fn setup(name: &str) {
        Spi::run(&format!("DROP TABLE IF EXISTS {name};")).unwrap();
        Spi::run(&format!("CREATE TABLE {name}(v int);")).unwrap();
        Spi::run(&format!("INSERT INTO {name} VALUES (1),(2),(3);")).unwrap();
    }

    #[pg_test]
    fn test_page_stats_returns_sensible_values() {
        setup("ps_t1");
        let row = Spi::get_three::<i32, i32, i32>(
            "SELECT max_offset, valid_items, pd_lower FROM page_stats('ps_t1', 0)",
        )
        .unwrap();
        let (max_off, valid, lower) = (row.0.unwrap(), row.1.unwrap(), row.2.unwrap());
        assert!(max_off >= 3, "expected at least 3 line pointers, got {max_off}");
        assert!(valid >= 3, "expected at least 3 live items, got {valid}");
        assert!(lower > 0, "pd_lower should be non-zero on a populated page");
    }

    #[pg_test]
    fn test_page_mark_dirty_try_succeeds_when_free() {
        setup("ps_t3");
        let ok = Spi::get_one::<bool>("SELECT page_mark_dirty_try('ps_t3', 0)").unwrap().unwrap();
        assert!(ok, "try_write_lock should succeed on a freshly-pinned buffer in this backend");
    }

    #[cfg(any(feature = "pg17", feature = "pg18"))]
    #[pg_test]
    fn test_page_is_dirty_after_mark() {
        setup("ps_t4");
        let _ = Spi::get_one::<bool>("SELECT page_mark_dirty_try('ps_t4', 0)").unwrap().unwrap();
        let dirty = Spi::get_one::<bool>("SELECT page_is_dirty('ps_t4', 0)").unwrap().unwrap();
        assert!(dirty, "page must be dirty after page_mark_dirty_try");
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
