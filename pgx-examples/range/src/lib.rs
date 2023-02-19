/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgx::prelude::*;

pgx::pg_module_magic!();

#[pg_extern]
fn range(s: i32, e: i32) -> pgx::Range<i32> {
    (s..e).into()
}

#[pg_extern]
fn range_from(s: i32) -> pgx::Range<i32> {
    (s..).into()
}

#[pg_extern]
fn range_full() -> pgx::Range<i32> {
    (..).into()
}

#[pg_extern]
fn range_inclusive(s: i32, e: i32) -> pgx::Range<i32> {
    (s..=e).into()
}

#[pg_extern]
fn range_to(e: i32) -> pgx::Range<i32> {
    (..e).into()
}

#[pg_extern]
fn range_to_inclusive(e: i32) -> pgx::Range<i32> {
    (..=e).into()
}

#[pg_extern]
fn empty() -> pgx::Range<i32> {
    pgx::Range::empty()
}

#[pg_extern]
fn infinite() -> pgx::Range<i32> {
    pgx::Range::infinite()
}

#[pg_extern]
fn assert_range(r: pgx::Range<i32>, s: i32, e: i32) -> bool {
    r == (s..e).into()
}
