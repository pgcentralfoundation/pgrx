/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{pg_sys, FromDatum, IntoDatum};
use pgx_utils::sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

#[derive(Debug, Clone, Copy)]
pub struct AnyArray {
    datum: pg_sys::Datum,
    typoid: pg_sys::Oid,
}

impl AnyArray {
    pub fn datum(&self) -> pg_sys::Datum {
        self.datum
    }

    pub fn oid(&self) -> pg_sys::Oid {
        self.typoid
    }

    #[inline]
    pub fn into<T: FromDatum>(&self) -> Option<T> {
        unsafe { T::from_datum(self.datum(), false, self.oid()) }
    }
}

impl FromDatum for AnyArray {
    const GET_TYPOID: bool = true;

    #[inline]
    unsafe fn from_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<AnyArray> {
        FromDatum::from_datum_with_typid(datum, is_null, typoid)
    }

    #[inline]
    unsafe fn from_datum_with_typid(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<AnyArray> {
        if is_null {
            None
        } else {
            Some(AnyArray { datum, typoid })
        }
    }
}

impl IntoDatum for AnyArray {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.datum)
    }

    fn type_oid() -> u32 {
        pg_sys::ANYARRAYOID
    }
}

unsafe impl SqlTranslatable for AnyArray {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("anyarray"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("anyarray")))
    }
}
