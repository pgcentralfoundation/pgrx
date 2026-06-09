//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! # Composite-shaped custom types and `Array<CustomType>`
//!
//! When a `PostgresType` is shaped like a record, you can still return `Vec<T>`
//! from `#[pg_extern]` (mapping to a SQL array of the type) and accept an
//! `Array<T>` in. This file demonstrates both directions, including null handling
//! for arrays of nullable elements.

use pgrx::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(PostgresType, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Pair {
    pub a: i32,
    pub b: String,
}

#[pg_extern]
fn make_pairs() -> Vec<Pair> {
    vec![Pair { a: 1, b: "one".into() }, Pair { a: 2, b: "two".into() }]
}

#[pg_extern]
fn sum_pair_a(arr: Array<Pair>) -> i64 {
    let mut total: i64 = 0;
    for p in arr.iter().flatten() {
        total += p.a as i64;
    }
    total
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn composite_returns_array() {
        let pairs = Spi::get_one::<Vec<Pair>>("SELECT make_pairs()").expect("query failed");
        assert_eq!(
            pairs,
            Some(vec![Pair { a: 1, b: "one".into() }, Pair { a: 2, b: "two".into() },])
        );
    }

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn composite_array_in() {
        let total = Spi::get_one::<i64>(
            r#"SELECT sum_pair_a(ARRAY[
                '{"a":10,"b":"x"}'::Pair,
                '{"a":32,"b":"y"}'::Pair
            ])"#,
        )
        .expect("query failed");
        assert_eq!(total, Some(42));
    }

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn composite_array_with_nulls() {
        // `flatten()` skips NULL entries.
        let total = Spi::get_one::<i64>(
            r#"SELECT sum_pair_a(ARRAY[
                '{"a":7,"b":"k"}'::Pair,
                NULL::Pair
            ])"#,
        )
        .expect("query failed");
        assert_eq!(total, Some(7));
    }
}
