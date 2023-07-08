//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::{pg_sys, AnyNumeric, FromDatum, IntoDatum, Numeric, PgMemoryContexts};

impl FromDatum for AnyNumeric {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            let numeric = pg_sys::pg_detoast_datum(datum.cast_mut_ptr()) as pg_sys::Numeric;
            let is_copy = !std::ptr::eq(
                numeric.cast::<pg_sys::NumericData>(),
                datum.cast_mut_ptr::<pg_sys::NumericData>(),
            );
            Some(AnyNumeric { inner: numeric, need_pfree: is_copy })
        }
    }

    unsafe fn from_datum_in_memory_context(
        mut memory_context: PgMemoryContexts,
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            memory_context.switch_to(|_| {
                // copy the Datum into this MemoryContext and then create the AnyNumeric over that
                let copy = pg_sys::pg_detoast_datum_copy(datum.cast_mut_ptr());
                Some(AnyNumeric { inner: copy.cast(), need_pfree: true })
            })
        }
    }
}

impl IntoDatum for AnyNumeric {
    #[inline]
    fn into_datum(mut self) -> Option<pg_sys::Datum> {
        // we're giving it to Postgres so we don't want our drop impl to free the inner pointer
        self.need_pfree = false;
        Some(pg_sys::Datum::from(self.inner))
    }

    #[inline]
    fn type_oid() -> pg_sys::Oid {
        pg_sys::NUMERICOID
    }
}

impl<const P: u32, const S: u32> FromDatum for Numeric<P, S> {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        typoid: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            Some(Numeric(AnyNumeric::from_polymorphic_datum(datum, false, typoid).unwrap()))
        }
    }
}

impl<const P: u32, const S: u32> IntoDatum for Numeric<P, S> {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        self.0.into_datum()
    }

    #[inline]
    fn type_oid() -> pg_sys::Oid {
        pg_sys::NUMERICOID
    }
}
