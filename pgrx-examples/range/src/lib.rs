use pgrx::prelude::*;

pgrx::pg_module_magic!();

#[pg_extern]
fn range(s: i32, e: i32) -> pgrx::Range<i32> {
    (s..e).into()
}

#[pg_extern]
fn range_from(s: i32) -> pgrx::Range<i32> {
    (s..).into()
}

#[pg_extern]
fn range_full() -> pgrx::Range<i32> {
    (..).into()
}

#[pg_extern]
fn range_inclusive(s: i32, e: i32) -> pgrx::Range<i32> {
    (s..=e).into()
}

#[pg_extern]
fn range_to(e: i32) -> pgrx::Range<i32> {
    (..e).into()
}

#[pg_extern]
fn range_to_inclusive(e: i32) -> pgrx::Range<i32> {
    (..=e).into()
}

#[pg_extern]
fn empty() -> pgrx::Range<i32> {
    pgrx::Range::empty()
}

#[pg_extern]
fn infinite() -> pgrx::Range<i32> {
    pgrx::Range::infinite()
}

#[pg_extern]
fn assert_range(r: pgrx::Range<i32>, s: i32, e: i32) -> bool {
    r == (s..e).into()
}
