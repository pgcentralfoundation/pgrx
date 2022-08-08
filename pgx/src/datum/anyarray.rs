/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{pg_sys, FromDatum, IntoDatum};
use pgx_utils::sql_entity_graph::metadata::{
    ArgumentError, ReturnVariant, ReturnVariantError, SqlTranslatable, SqlVariant,
};

#[derive(Debug, Clone, Copy)]
pub struct AnyArray {
    datum: pg_sys::Datum,
}

impl AnyArray {
    pub fn datum(&self) -> pg_sys::Datum {
        self.datum
    }

    #[inline]
    pub fn into<T: FromDatum>(&self) -> Option<T> {
        unsafe { T::from_datum(self.datum(), false) }
    }
}

impl FromDatum for AnyArray {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<AnyArray> {
        if is_null {
            None
        } else {
            Some(AnyArray { datum })
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

impl SqlTranslatable for AnyArray {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("anyarray")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "anyarray",
        ))))
    }
}
