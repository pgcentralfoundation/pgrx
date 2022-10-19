/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{direct_function_call_as_datum, pg_sys, FromDatum, IntoDatum};

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
    fn into_datum(mut self) -> Option<pg_sys::Datum> {
        let the_box = &mut self;
        unsafe {
            direct_function_call_as_datum(
                pg_sys::box_out,
                vec![Some(pg_sys::Datum::from(the_box as *mut pg_sys::BOX))],
            )
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
    fn into_datum(mut self) -> Option<pg_sys::Datum> {
        let point = &mut self;
        unsafe {
            direct_function_call_as_datum(
                pg_sys::point_out,
                vec![Some(pg_sys::Datum::from(point as *mut _))],
            )
        }
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::POINTOID
    }
}
