//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! # Variant 2: custom text format via `#[inoutfuncs]` + `InOutFuncs`
//!
//! Add `#[inoutfuncs]` to your `PostgresType` and implement `InOutFuncs` to define
//! a custom textual representation. The generated `*_in`/`*_out` will call your
//! `input(&CStr) -> Self` / `output(&self, &mut StringInfo)` instead of going through
//! Serde JSON. Use this when the SQL surface should look like a domain-specific
//! syntax (e.g. `"3+4i"` for complex numbers, `"R5C7"` for cell refs, etc.).

use core::ffi::CStr;
use pgrx::prelude::*;
use pgrx::{InOutFuncs, StringInfo};
use serde::{Deserialize, Serialize};

#[derive(PostgresType, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[inoutfuncs]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl InOutFuncs for Complex {
    fn input(input: &CStr) -> Self {
        // Parses "<re>+<im>i" or "<re>-<im>i". Errors panic into Postgres ERROR.
        let s = input.to_str().expect("invalid UTF-8");
        let s = s.trim();
        let i_pos = s.rfind('i').expect("expected trailing 'i'");
        let body = &s[..i_pos];
        let split = body
            .rfind(['+', '-'])
            .filter(|&p| p > 0)
            .expect("expected sign between real and imaginary parts");
        let re: f64 = body[..split].parse().expect("invalid real part");
        let im: f64 = body[split..].parse().expect("invalid imaginary part");
        Complex { re, im }
    }

    fn output(&self, buffer: &mut StringInfo) {
        if self.im >= 0.0 {
            buffer.push_str(&format!("{}+{}i", self.re, self.im));
        } else {
            buffer.push_str(&format!("{}{}i", self.re, self.im));
        }
    }
}

#[pg_extern]
fn complex_add(a: Complex, b: Complex) -> Complex {
    Complex { re: a.re + b.re, im: a.im + b.im }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn inoutfuncs_parse() {
        let c = Spi::get_one::<Complex>("SELECT '3+4i'::Complex").expect("query failed");
        assert_eq!(c, Some(Complex { re: 3.0, im: 4.0 }));
    }

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn inoutfuncs_format() {
        let s = Spi::get_one::<String>("SELECT '3-4i'::Complex::text").expect("query failed");
        assert_eq!(s.as_deref(), Some("3-4i"));
    }

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn inoutfuncs_add() {
        let c = Spi::get_one::<Complex>("SELECT complex_add('1+2i'::Complex, '3+4i'::Complex)")
            .expect("query failed");
        assert_eq!(c, Some(Complex { re: 4.0, im: 6.0 }));
    }
}
