/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{pg_sys, FromDatum, IntoDatum, PgBox};

impl FromDatum for pg_sys::BOX {
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            let the_box = datum.cast_mut_ptr::<pg_sys::BOX>();
            Some(the_box.read())
        }
    }
}

impl IntoDatum for pg_sys::BOX {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        unsafe {
            let boxed = PgBox::<pg_sys::BOX>::alloc0();
            std::ptr::copy(&self, boxed.as_ptr(), std::mem::size_of::<pg_sys::BOX>());
            boxed.into_datum()
        }
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::BOXOID
    }
}

impl FromDatum for pg_sys::Point {
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            let point: *mut Self = datum.cast_mut_ptr();
            Some(point.read())
        }
    }
}

impl IntoDatum for pg_sys::Point {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        unsafe {
            let boxed = PgBox::<pg_sys::Point>::alloc0();
            std::ptr::copy(&self, boxed.as_ptr(), std::mem::size_of::<pg_sys::Point>());
            boxed.into_datum()
        }
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::POINTOID
    }
}
