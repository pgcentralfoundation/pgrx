//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::{
    item_pointer_get_both, item_pointer_set_all, pg_sys, FromDatum, IntoDatum, PgMemoryContexts,
};

impl FromDatum for pg_sys::ItemPointerData {
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: pg_sys::Oid,
    ) -> Option<pg_sys::ItemPointerData> {
        if is_null {
            None
        } else {
            let tid = datum.cast_mut_ptr();
            let (blockno, offno) = item_pointer_get_both(*tid);
            let mut tid_copy = pg_sys::ItemPointerData::default();

            item_pointer_set_all(&mut tid_copy, blockno, offno);
            Some(tid_copy)
        }
    }
}

impl IntoDatum for pg_sys::ItemPointerData {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        let tid = self;
        let tid_ptr = unsafe {
            // SAFETY:  CurrentMemoryContext is always valid
            PgMemoryContexts::CurrentMemoryContext.palloc_struct::<pg_sys::ItemPointerData>()
        };
        let (blockno, offno) = item_pointer_get_both(tid);

        item_pointer_set_all(unsafe { &mut *tid_ptr }, blockno, offno);

        Some(tid_ptr.into())
    }

    fn type_oid() -> pg_sys::Oid {
        pg_sys::TIDOID
    }
}
