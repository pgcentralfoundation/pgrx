//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Cross-cutting smoke test that exercises three `PostgresType` in/out variants
//! in miniature. A failure here means the macro layer regressed independently
//! of the `postgres_type_variants` example crate building.

use core::ffi::CStr;
use pgrx::datum::PgVarlena;
use pgrx::prelude::*;
use pgrx::{InOutFuncs, PgVarlenaInOutFuncs, StringInfo};
use serde::{Deserialize, Serialize};

#[derive(PostgresType, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SmokeJson {
    pub n: i32,
}

#[pg_extern]
fn smoke_json_id(j: SmokeJson) -> SmokeJson {
    j
}

#[derive(PostgresType, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[inoutfuncs]
pub struct SmokeText {
    pub s: String,
}

impl InOutFuncs for SmokeText {
    fn input(input: &CStr) -> Self {
        SmokeText { s: input.to_str().expect("utf-8").to_string() }
    }
    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&self.s);
    }
}

#[pg_extern]
fn smoke_text_id(t: SmokeText) -> SmokeText {
    t
}

#[derive(Copy, Clone, PostgresType)]
#[bikeshed_postgres_type_manually_impl_from_into_datum]
#[pgvarlena_inoutfuncs]
pub struct SmokeVarlena {
    pub a: u32,
}

impl PgVarlenaInOutFuncs for SmokeVarlena {
    fn input(input: &CStr) -> PgVarlena<Self> {
        let a: u32 = input.to_str().expect("utf-8").parse().expect("number");
        let mut v = PgVarlena::<Self>::new();
        v.a = a;
        v
    }
    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&self.a.to_string());
    }
}

#[pg_extern]
fn smoke_varlena_double(v: PgVarlena<SmokeVarlena>) -> i64 {
    (v.a as i64).wrapping_mul(2)
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use super::*;

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn smoke_json() {
        let v = Spi::get_one::<SmokeJson>(r#"SELECT smoke_json_id('{"n":7}'::SmokeJson)"#)
            .expect("query failed");
        assert_eq!(v, Some(SmokeJson { n: 7 }));
    }

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn smoke_text() {
        let v = Spi::get_one::<SmokeText>("SELECT smoke_text_id('hello'::SmokeText)")
            .expect("query failed");
        assert_eq!(v, Some(SmokeText { s: "hello".into() }));
    }

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn smoke_varlena() {
        let v = Spi::get_one::<i64>("SELECT smoke_varlena_double('21'::SmokeVarlena)")
            .expect("query failed");
        assert_eq!(v, Some(42));
    }
}
