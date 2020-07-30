// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use crate::{pg_sys, FromDatum, IntoDatum};

#[derive(Debug, Clone, Copy)]
pub struct AnyElement {
    datum: pg_sys::Datum,
    typoid: pg_sys::Oid,
}

impl AnyElement {
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

impl FromDatum for AnyElement {
    #[inline]
    unsafe fn from_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<AnyElement> {
        if is_null {
            None
        } else {
            Some(AnyElement { datum, typoid })
        }
    }
}

impl IntoDatum for AnyElement {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.datum)
    }

    fn type_oid() -> u32 {
        pg_sys::ANYELEMENTOID
    }
}
