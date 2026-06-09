//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! # Variant 1: default JSON in/out via Serde
//!
//! `#[derive(PostgresType)]` with no `#[inoutfuncs]` attribute and a `Serialize +
//! Deserialize` bound emits `*_in`/`*_out` functions that go through Serde's JSON
//! representation. Easiest path; ideal when the type is JSON-shaped or you don't
//! care about a custom textual representation.
//!
//! See sibling modules for the other three variants.

use pgrx::prelude::*;
use serde::{Deserialize, Serialize};

// Note: we deliberately avoid the name `Point` because Postgres has a built-in `point` type (2D geometry), and a derived input function `point_in` would collide. Pick a name not already in pg_catalog when shipping a custom type.
#[derive(PostgresType, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

#[pg_extern]
fn coord_origin() -> Coord {
    Coord { x: 0.0, y: 0.0 }
}

#[pg_extern]
fn coord_translate(p: Coord, dx: f64, dy: f64) -> Coord {
    Coord { x: p.x + dx, y: p.y + dy }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn json_default_roundtrip() {
        let p =
            Spi::get_one::<Coord>(r#"SELECT '{"x":3.0,"y":4.0}'::Coord"#).expect("query failed");
        assert_eq!(p, Some(Coord { x: 3.0, y: 4.0 }));
    }

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn json_default_translate() {
        let p = Spi::get_one::<Coord>(
            r#"SELECT coord_translate('{"x":1.0,"y":2.0}'::Coord, 10.0, 20.0)"#,
        )
        .expect("query failed");
        assert_eq!(p, Some(Coord { x: 11.0, y: 22.0 }));
    }
}
