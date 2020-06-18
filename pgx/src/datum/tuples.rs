// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use crate::{pg_sys, FromDatum, IntoDatum};

impl<A, B> IntoDatum for (Option<A>, Option<B>)
where
    A: IntoDatum,
    B: IntoDatum,
{
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let vec = vec![self.0.into_datum(), self.1.into_datum()];
        vec.into_datum()
    }

    fn type_oid() -> pg_sys::Oid {
        0
    }
}

impl<A, B, C> IntoDatum for (Option<A>, Option<B>, Option<C>)
where
    A: IntoDatum,
    B: IntoDatum,
    C: IntoDatum,
{
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let vec = vec![
            self.0.into_datum(),
            self.1.into_datum(),
            self.2.into_datum(),
        ];
        vec.into_datum()
    }

    fn type_oid() -> pg_sys::Oid {
        0
    }
}

impl<A, B> FromDatum for (Option<A>, Option<B>)
where
    A: FromDatum + IntoDatum,
    B: FromDatum + IntoDatum,
{
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, typoid: pg_sys::Oid) -> Option<Self>
    where
        Self: Sized,
    {
        let mut vec = Vec::<Option<pg_sys::Datum>>::from_datum(datum, is_null, typoid).unwrap();
        let b = vec.pop().unwrap();
        let a = vec.pop().unwrap();

        let a_datum = if a.is_some() {
            A::from_datum(a.unwrap(), false, A::type_oid())
        } else {
            None
        };

        let b_datum = if b.is_some() {
            B::from_datum(b.unwrap(), false, B::type_oid())
        } else {
            None
        };

        Some((a_datum, b_datum))
    }
}

impl<A, B, C> FromDatum for (Option<A>, Option<B>, Option<C>)
where
    A: FromDatum + IntoDatum,
    B: FromDatum + IntoDatum,
    C: FromDatum + IntoDatum,
{
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool, typoid: pg_sys::Oid) -> Option<Self>
    where
        Self: Sized,
    {
        let mut vec = Vec::<Option<pg_sys::Datum>>::from_datum(datum, is_null, typoid).unwrap();
        let c = vec.pop().unwrap();
        let b = vec.pop().unwrap();
        let a = vec.pop().unwrap();

        let a_datum = if a.is_some() {
            A::from_datum(a.unwrap(), false, A::type_oid())
        } else {
            None
        };

        let b_datum = if b.is_some() {
            B::from_datum(b.unwrap(), false, B::type_oid())
        } else {
            None
        };

        let c_datum = if c.is_some() {
            C::from_datum(c.unwrap(), false, C::type_oid())
        } else {
            None
        };

        Some((a_datum, b_datum, c_datum))
    }
}
