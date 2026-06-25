//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! `generic_agg` — a worked example that proves why pgrx needs `utils/datum.h`.
//!
//! It implements a polymorphic aggregate:
//!
//! ```sql
//! count_changes(anyelement) -> bigint
//! ```
//!
//! which counts how many times the input value differs from the previous row
//! (a classic run-length / change-detection aggregate).
//!
//! The aggregate receives a raw `pg_sys::Datum` whose concrete type is only
//! known at runtime (`anyelement`). To carry "the value seen so far" forward
//! across transition calls, it must:
//!
//! * `datumCopy`  — deep-copy a (possibly pass-by-reference) value into the
//!   long-lived aggregate memory context, so it survives the per-tuple context
//!   being recycled before the next row, and
//! * `datumIsEqual` — byte-compare two type-erased values to decide whether the
//!   value changed.
//!
//! Both functions are declared only in `utils/datum.h`. Without that header an
//! extension author would have to hand-redeclare them `extern "C"`.

use pgrx::aggregate::*;
use pgrx::prelude::*;
use pgrx::{AnyElement, Internal};

pgrx::pg_module_magic!(name, version);

/// Per-group transition state, allocated in the aggregate's memory context.
///
/// `prev` holds a **copy** (made with `datumCopy`) of the most-recently-seen value, so it stays valid across transition calls even for pass-by-reference types whose original Datum lives in the short-lived per-tuple context.
#[derive(Clone, Copy)]
struct ChangeState {
    changes: i64,
    typlen: i16,
    typbyval: bool,
    type_known: bool,
    has_prev: bool,
    prev: pg_sys::Datum,
}

/// Marker type naming the SQL aggregate `count_changes`.
#[derive(AggregateName)]
#[aggregate_name = "count_changes"]
struct CountChanges;

#[pg_aggregate]
impl Aggregate<CountChanges> for CountChanges {
    type State = Internal;
    type Args = Option<AnyElement>;
    type Finalize = i64;

    fn state(
        mut current: Self::State,
        arg: Self::Args,
        fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        // NULL inputs aren't a "change"; leave the state untouched.
        let elem = match arg {
            Some(e) => e,
            None => return current,
        };

        unsafe {
            // Transition functions run with CurrentMemoryContext set to a *short-lived* per-tuple context. Anything we need on the next row (the state struct AND the copied Datum) must instead be allocated n the long-lived aggregate context obtained here.
            let mut agg_ctx: pg_sys::MemoryContext = core::ptr::null_mut();
            if pg_sys::AggCheckCallContext(fcinfo, &mut agg_ctx) == 0 {
                error!("count_changes can only be used as an aggregate");
            }
            let old_ctx = pg_sys::MemoryContextSwitchTo(agg_ctx);

            let st = current.get_or_insert_with(|| ChangeState {
                changes: 0,
                typlen: 0,
                typbyval: false,
                type_known: false,
                has_prev: false,
                prev: pg_sys::Datum::null(),
            });

            // Resolve (typlen, typbyval) once from the runtime element type.
            if !st.type_known {
                pg_sys::get_typlenbyval(elem.oid(), &mut st.typlen, &mut st.typbyval);
                st.type_known = true;
            }

            let cur = elem.datum();
            // datumCopy/datumIsEqual take C `int` for typlen.
            let typlen = st.typlen as i32;

            if st.has_prev {
                // datumIsEqual is a *byte-level* comparison.
                if !pg_sys::datumIsEqual(st.prev, cur, st.typbyval, typlen) {
                    st.changes += 1;
                    // Release the previous copy (no-op for pass-by-value types).
                    if !st.typbyval {
                        pg_sys::pfree(st.prev.cast_mut_ptr());
                    }
                    st.prev = pg_sys::datumCopy(cur, st.typbyval, typlen);
                }
            } else {
                st.prev = pg_sys::datumCopy(cur, st.typbyval, typlen);
                st.has_prev = true;
            }

            pg_sys::MemoryContextSwitchTo(old_ctx);
        }

        current
    }

    fn finalize(
        current: Self::State,
        _direct_args: Self::OrderedSetArgs,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::Finalize {
        // No rows (or all-NULL input) => zero changes.
        unsafe { current.get::<ChangeState>().map(|s| s.changes).unwrap_or(0) }
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    #[allow(unused_imports)]
    use pgrx::prelude::*;

    // text is pass-by-reference: exercises the datumCopy/datumIsEqual path.
    // Ordered a,a,b,b,b,c => 2.
    #[pg_test]
    fn count_changes_text() {
        let changes = Spi::get_one::<i64>(
            "SELECT count_changes(v ORDER BY ord) FROM \
             (VALUES (1,'a'),(2,'a'),(3,'b'),(4,'b'),(5,'b'),(6,'c')) t(ord, v)",
        )
        .expect("SPI result was NULL");
        assert_eq!(changes, Some(2));
    }

    // numeric exercises the varlena path (typlen = -1).
    // Ordered 1.5,1.5,1.5,2.0,2.0,3.0,1.5 => 3.
    #[pg_test]
    fn count_changes_numeric() {
        let changes = Spi::get_one::<i64>(
            "SELECT count_changes(v ORDER BY ord) FROM (VALUES \
             (1,1.5::numeric),(2,1.5),(3,1.5),(4,2.0),(5,2.0),(6,3.0),(7,1.5)) t(ord, v)",
        )
        .expect("SPI result was NULL");
        assert_eq!(changes, Some(3));
    }

    // int4 is pass-by-value: proves the no-op-copy branch and byte-compare.
    // Ordered 1,1,2,2,2,1,1 => 2.
    #[pg_test]
    fn count_changes_int4() {
        let changes = Spi::get_one::<i64>(
            "SELECT count_changes(v ORDER BY ord) FROM \
             (VALUES (1,1),(2,1),(3,2),(4,2),(5,2),(6,1),(7,1)) t(ord, v)",
        )
        .expect("SPI result was NULL");
        assert_eq!(changes, Some(2));
    }

    // NULLs are ignored, not counted as changes.
    // Ordered non-null subsequence a,a,b,b => 1.
    #[pg_test]
    fn count_changes_ignores_nulls() {
        let changes = Spi::get_one::<i64>(
            "SELECT count_changes(v ORDER BY ord) FROM (VALUES \
             (1,'a'),(2,NULL),(3,'a'),(4,'b'),(5,NULL),(6,'b')) t(ord, v)",
        )
        .expect("SPI result was NULL");
        assert_eq!(changes, Some(1));
    }

    // No rows => 0.
    #[pg_test]
    fn count_changes_empty() {
        let changes =
            Spi::get_one::<i64>("SELECT count_changes(v) FROM (SELECT 1 WHERE false) t(v)")
                .expect("SPI result was NULL");
        assert_eq!(changes, Some(0));
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
