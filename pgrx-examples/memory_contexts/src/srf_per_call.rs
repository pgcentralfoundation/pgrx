//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! # SRFs and memory-context lifetimes
//!
//! A Set-Returning Function in Postgres has two relevant contexts:
//!
//! * **`multi_call_memory_ctx`** — lives across all calls of the SRF for one row;
//!   anything stored here must be safe to read on the *next* call.
//! * **`per_query_ctx`** (effectively the executor's context) — lives for the
//!   whole query; you can stash long-lived computed state here.
//!
//! When you write a pgrx SRF using `SetOfIterator` or `TableIterator`, pgrx
//! handles the multi-call protocol AND the contexts for you: the iterator
//! itself is materialized once, and per-row values are returned from inside
//! the right context. You almost never need to touch the contexts directly
//! from a Rust SRF — the framework does it.
//!
//! The two functions below make the contract explicit:
//!
//! 1. `iter_count` — a tiny SRF returning a known sequence; the underlying
//!    iterator state lives in the multi-call context for the duration of the
//!    SRF, and pgrx switches contexts on each call automatically.
//! 2. `materialized_pairs` — pre-computes all rows in one shot; ideal for small
//!    result sets where you don't want per-call work. Memory for the produced
//!    `Vec` lives until the executor reclaims it after the query finishes.

use pgrx::prelude::*;

#[pg_extern]
fn iter_count(start: i64, end: i64) -> SetOfIterator<'static, i64> {
    SetOfIterator::new(start..end)
}

#[pg_extern]
fn materialized_pairs(n: i32) -> TableIterator<'static, (name!(idx, i32), name!(square, i64))> {
    let rows: Vec<(i32, i64)> = (0..n).map(|i| (i, (i as i64) * (i as i64))).collect();
    TableIterator::new(rows.into_iter())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn srf_iter_count_sum() {
        let total = Spi::get_one::<i64>("SELECT sum(x)::bigint FROM iter_count(0, 5) AS x")
            .expect("query failed");
        // 0+1+2+3+4 = 10
        assert_eq!(total, Some(10));
    }

    #[pg_test]
    fn srf_materialized_pairs_count() {
        let c = Spi::get_one::<i64>("SELECT count(*) FROM materialized_pairs(10)")
            .expect("query failed");
        assert_eq!(c, Some(10));
    }

    #[pg_test]
    fn srf_materialized_pairs_value() {
        let v = Spi::get_one::<i64>("SELECT square FROM materialized_pairs(10) WHERE idx = 7")
            .expect("query failed");
        assert_eq!(v, Some(49));
    }
}
