use crate::{
    heap_getattr_raw, pg_sys, AllocatedByPostgres, AllocatedByRust, FromDatum, FromDatumResult,
    IntoDatum, PgBox, PgTupleDesc, TryFromDatumError, WhoAllocated,
};
use std::num::NonZeroUsize;

#[derive(Debug)]
pub enum TriggerTuple {
    OLD,
    NEW,
}

pub struct PgHeapTuple<'a, AllocatedBy: WhoAllocated<pg_sys::HeapTupleData>> {
    tuple: PgBox<pg_sys::HeapTupleData, AllocatedBy>,
    tupdesc: PgTupleDesc<'a>,
}

impl<'a> PgHeapTuple<'a, AllocatedByPostgres> {
    pub unsafe fn from_heap_tuple(tupdesc: PgTupleDesc<'a>, heap_tuple: pg_sys::HeapTuple) -> Self {
        Self {
            tuple: PgBox::from_pg(heap_tuple),
            tupdesc,
        }
    }

    pub unsafe fn from_trigger_data(
        trigger_data: &'a pg_sys::TriggerData,
        new_old: TriggerTuple,
    ) -> Self {
        let tupdesc =
            PgTupleDesc::from_pg_unchecked(trigger_data.tg_relation.as_ref().unwrap().rd_att);
        PgHeapTuple::from_heap_tuple(
            tupdesc,
            match new_old {
                TriggerTuple::OLD => trigger_data.tg_trigtuple,
                TriggerTuple::NEW => trigger_data.tg_newtuple,
            },
        )
    }
}

impl<'a> PgHeapTuple<'a, AllocatedByRust> {
    pub fn from_datums<I: IntoIterator<Item = Option<pg_sys::Datum>>>(
        tupdesc: PgTupleDesc<'a>,
        datums: I,
    ) -> Self {
        let iter = datums.into_iter();
        let mut datums = Vec::<pg_sys::Datum>::with_capacity(iter.size_hint().1.unwrap_or(1));
        let mut nulls = Vec::<bool>::with_capacity(iter.size_hint().1.unwrap_or(1));
        iter.for_each(|datum| {
            nulls.push(datum.is_none());
            datums.push(datum.unwrap_or(0));
        });
        if datums.len() != tupdesc.len() {
            panic!("incorrect number of datums for provided PgTupleDesc");
        }

        unsafe {
            let formed_tuple =
                pg_sys::heap_form_tuple(tupdesc.as_ptr(), datums.as_mut_ptr(), nulls.as_mut_ptr());

            Self {
                tuple: PgBox::<pg_sys::HeapTupleData, AllocatedByRust>::from_rust(formed_tuple),
                tupdesc,
            }
        }
    }

    pub unsafe fn from_composite_datum(composite: pg_sys::Datum) -> Self {
        let htup_header =
            pg_sys::pg_detoast_datum(composite as *mut pg_sys::varlena) as pg_sys::HeapTupleHeader;
        let tup_type = crate::heap_tuple_header_get_type_id(htup_header);
        let tup_typmod = crate::heap_tuple_header_get_typmod(htup_header);
        let tupdesc = pg_sys::lookup_rowtype_tupdesc(tup_type, tup_typmod);

        let mut data = PgBox::<pg_sys::HeapTupleData>::alloc();

        data.t_len = crate::heap_tuple_header_get_datum_length(htup_header) as u32;
        data.t_data = htup_header;

        Self {
            tuple: data,
            tupdesc: PgTupleDesc::from_pg_is_copy(tupdesc),
        }
    }
}

impl<'a, AllocatedBy: WhoAllocated<pg_sys::HeapTupleData>> PgHeapTuple<'a, AllocatedBy> {
    pub fn get_named<T: FromDatum + IntoDatum + 'static, A: AsRef<str>>(
        &self,
        attname: A,
    ) -> FromDatumResult<T> {
        // find the attribute number by name
        for att in self.tupdesc.iter() {
            if att.name() == attname.as_ref() {
                // we found the named attribute, so go get it from the HeapTuple
                return self.get_indexed(NonZeroUsize::new(att.attnum as usize + 1).unwrap());
            }
        }

        // no attribute with the specified name
        Err(TryFromDatumError::NoSuchAttributeName(
            attname.as_ref().to_owned(),
        ))
    }

    pub fn get_indexed<T: FromDatum + IntoDatum + 'static>(
        &self,
        attno: NonZeroUsize,
    ) -> FromDatumResult<T> {
        unsafe {
            // tuple descriptor attribute numbers are zero-based
            match self.tupdesc.get(attno.get() - 1) {
                // it's an attribute number outside the bounds of the tuple descriptor
                None => Err(TryFromDatumError::NoSuchAttributeNumber(attno)),

                // it's a valid attribute number
                Some(att) => {
                    let datum = heap_getattr_raw(self.tuple.as_ptr(), attno, self.tupdesc.as_ptr());
                    if datum.is_none() {
                        return Ok(None);
                    }

                    T::try_from_datum(datum.unwrap(), false, att.type_oid().value())
                }
            }
        }
    }
}
