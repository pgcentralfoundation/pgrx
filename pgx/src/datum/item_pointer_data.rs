// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use crate::{
    item_pointer_get_both, item_pointer_set_all, pg_sys, FromDatum, IntoDatum, PgMemoryContexts,
};

impl FromDatum for pg_sys::ItemPointerData {
    #[inline]
    unsafe fn from_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _typoid: u32,
    ) -> Option<pg_sys::ItemPointerData> {
        if is_null {
            None
        } else {
            let tid = datum as *mut pg_sys::ItemPointerData;
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
        let tid_ptr =
            PgMemoryContexts::CurrentMemoryContext.palloc_struct::<pg_sys::ItemPointerData>();
        let (blockno, offno) = item_pointer_get_both(tid);

        item_pointer_set_all(unsafe { &mut *tid_ptr }, blockno, offno);

        Some(tid_ptr as pg_sys::Datum)
    }

    fn type_oid() -> u32 {
        pg_sys::TIDOID
    }
}
