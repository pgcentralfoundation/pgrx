//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! # Variant 3: zero-copy varlena via `PgVarlenaInOutFuncs`
//!
//! `#[pgvarlena_inoutfuncs]` makes the type an on-disk fixed binary layout that
//! Postgres can store directly without serialization. Use this when:
//!
//! * The struct is `Copy` and `#[repr(C)]`-compatible.
//! * You frequently read large numbers of rows and Serde JSON would dominate.
//! * You're willing to define a textual form for I/O at SQL boundaries.
//!
//! The trade-off vs. variant 2 is performance (no parse on read) for cost
//! (you must hand-write text I/O AND keep the binary layout stable).
//!
//! Note: `Rgb` is 3 bytes — a tight layout is the whole point of this variant.

use core::ffi::CStr;
use pgrx::datum::PgVarlena;
use pgrx::prelude::*;
use pgrx::{PgVarlenaInOutFuncs, StringInfo};

#[derive(Copy, Clone, PostgresType)]
#[bikeshed_postgres_type_manually_impl_from_into_datum]
#[pgvarlena_inoutfuncs]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PgVarlenaInOutFuncs for Rgb {
    fn input(input: &CStr) -> PgVarlena<Self> {
        // Accepts "#rrggbb"
        let s = input.to_str().expect("invalid UTF-8");
        let hex = s.strip_prefix('#').expect("expected leading '#'");
        assert_eq!(hex.len(), 6, "expected 6 hex digits");
        let r = u8::from_str_radix(&hex[0..2], 16).expect("bad red");
        let g = u8::from_str_radix(&hex[2..4], 16).expect("bad green");
        let b = u8::from_str_radix(&hex[4..6], 16).expect("bad blue");
        let mut v = PgVarlena::<Self>::new();
        v.r = r;
        v.g = g;
        v.b = b;
        v
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b));
    }
}

#[pg_extern]
fn rgb_luminance(c: PgVarlena<Rgb>) -> f64 {
    // Rec. 601 luma — purely a demonstration; arithmetic happens directly on the varlena's fields with no copy or parse.
    0.299 * c.r as f64 + 0.587 * c.g as f64 + 0.114 * c.b as f64
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn varlena_roundtrip() {
        let s = Spi::get_one::<String>("SELECT '#ff8000'::Rgb::text").expect("query failed");
        assert_eq!(s.as_deref(), Some("#ff8000"));
    }

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn varlena_compute() {
        let lum =
            Spi::get_one::<f64>("SELECT rgb_luminance('#ffffff'::Rgb)").expect("query failed");
        // 0.299 + 0.587 + 0.114 == 1.0; multiplied by 255.
        assert!((lum.unwrap() - 255.0).abs() < 0.001);
    }
}
