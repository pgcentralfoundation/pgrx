//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! # Variant 4: hand-rolled `FromDatum`/`IntoDatum`, no `PostgresType` derive
//!
//! You don't *need* `#[derive(PostgresType)]` to ship a custom type. This file
//! demonstrates the minimum machinery: implement `FromDatum`, `IntoDatum`,
//! `SqlTranslatable`, `ArgAbi`, `BoxRet`, write `*_in`/`*_out` `#[pg_extern]`s,
//! and emit the `CREATE TYPE` SQL via `extension_sql!`.
//!
//! Use this when:
//!
//! * You want full control over the SQL representation (alignment, length).
//! * The Serde JSON path is unacceptable AND `InOutFuncs`/`PgVarlena` don't fit.
//! * You're studying what `derive(PostgresType)` actually generates (see #1384).

use pgrx::callconv::{ArgAbi, BoxRet};
use pgrx::datum::Datum;
use pgrx::pg_sys::Oid;
use pgrx::pgrx_sql_entity_graph::metadata::{
    ArgumentError, ReturnsError, ReturnsRef, SqlMappingRef, SqlTranslatable,
};
use pgrx::prelude::*;
use pgrx::{StringInfo, rust_regtypein};
use std::error::Error;
use std::ffi::CStr;

/// 24-bit unsigned integer stored as the low 24 bits of an `int4`.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct U24(pub u32);

unsafe impl SqlTranslatable for U24 {
    const TYPE_IDENT: &'static str = pgrx::pgrx_resolved_type!(U24);
    const TYPE_ORIGIN: pgrx::pgrx_sql_entity_graph::metadata::TypeOrigin =
        pgrx::pgrx_sql_entity_graph::metadata::TypeOrigin::ThisExtension;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Ok(SqlMappingRef::literal("u24"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("u24")));
}

impl FromDatum for U24 {
    unsafe fn from_polymorphic_datum(datum: pg_sys::Datum, is_null: bool, _: Oid) -> Option<Self> {
        if is_null { None } else { Some(U24(datum.value() as u32 & 0x00FF_FFFF)) }
    }
}

impl IntoDatum for U24 {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(pg_sys::Datum::from(self.0 as i64))
    }
    fn type_oid() -> Oid {
        rust_regtypein::<Self>()
    }
}

unsafe impl<'fcx> ArgAbi<'fcx> for U24
where
    Self: 'fcx,
{
    unsafe fn unbox_arg_unchecked(arg: pgrx::callconv::Arg<'_, 'fcx>) -> Self {
        unsafe { arg.unbox_arg_using_from_datum().unwrap() }
    }
}

unsafe impl BoxRet for U24 {
    unsafe fn box_into<'fcx>(self, fcinfo: &mut pgrx::callconv::FcInfo<'fcx>) -> Datum<'fcx> {
        unsafe { fcinfo.return_raw_datum(pg_sys::Datum::from(self.0 as i64)) }
    }
}

#[pg_extern(immutable, parallel_safe, requires = ["u24_shell"])]
fn u24_in(input: &CStr) -> Result<U24, Box<dyn Error>> {
    let v: u32 = input.to_str()?.parse()?;
    if v > 0x00FF_FFFF {
        return Err("value exceeds 24 bits".into());
    }
    Ok(U24(v))
}

#[pg_extern(immutable, parallel_safe, requires = ["u24_shell"])]
fn u24_out(value: U24) -> &'static CStr {
    let mut s = StringInfo::new();
    s.push_str(&value.0.to_string());
    unsafe { s.leak_cstr() }
}

extension_sql!("CREATE TYPE u24;", name = "u24_shell", bootstrap);

extension_sql!(
    r#"
CREATE TYPE u24 (
    INPUT = u24_in,
    OUTPUT = u24_out,
    LIKE = int4
);
"#,
    name = "u24_concrete",
    creates = [Type(U24)],
    requires = ["u24_shell", u24_in, u24_out],
);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;
    use std::error::Error;

    #[pg_test]
    fn handrolled_in_out() -> Result<(), Box<dyn Error>> {
        let v = Spi::get_one::<U24>("SELECT '16777215'::u24")?;
        assert_eq!(v, Some(U24(0x00FF_FFFF)));
        let s = Spi::get_one::<String>("SELECT '42'::u24::text")?;
        assert_eq!(s.as_deref(), Some("42"));
        Ok(())
    }
}
