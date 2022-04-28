//! Provides a safe interface to Postgres `HeapTuple` objects.
use crate::{
    heap_getattr_raw, pg_sys, AllocatedByPostgres, AllocatedByRust, FromDatum, FromDatumResult,
    IntoDatum, PgBox, PgTupleDesc, TriggerTuple, TryFromDatumError, WhoAllocated,
};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::NonZeroUsize;

/// Describes errors that can occur when trying to create a new [PgHeapTuple].
#[derive(Debug, Copy, Clone)]
pub enum PgHeapTupleCreationError {
    NullTuple,
    IncorrectAttributeCount,
}

impl Display for PgHeapTupleCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PgHeapTupleCreationError::NullTuple => write!(f, "Null tuple"),
            PgHeapTupleCreationError::IncorrectAttributeCount => {
                write!(f, "Incorrect attribute count")
            }
        }
    }
}

impl Error for PgHeapTupleCreationError {}

pub type PgHeapTupleCreationResult<'a, AllocatedBy> =
    Result<PgHeapTuple<'a, AllocatedBy>, PgHeapTupleCreationError>;

/// A [PgHeapTuple] is a lightweight wrapper around Postgres' [pg_sys::HeapTuple] object and a [PgTupleDesc].
///
/// In order to access the attributes within a [pg_sys::HeapTuple], the [PgTupleDesc] is required
/// to describe its structure.
///
/// [PgHeapTuple]s can be created from existing (Postgres-provided) [pg_sys::HeapTuple] pointers, from
/// [pg_sys::TriggerData] pointers, from a composite datum, or created from scratch using raw Datums.
///
/// A [PgHeapTuple] can either be considered to be allocated by Postgres or by the Rust runtime. If
/// allocated by Postgres, it is not mutable until [PgHeapTuple::into_owned] is called.
pub struct PgHeapTuple<'a, AllocatedBy: WhoAllocated<pg_sys::HeapTupleData>> {
    tuple: PgBox<pg_sys::HeapTupleData, AllocatedBy>,
    tupdesc: PgTupleDesc<'a>,
}

impl<'a> PgHeapTuple<'a, AllocatedByPostgres> {
    /// Creates a new [PgHeapTuple] from a [PgTupleDesc] and a [pg_sys::HeapTuple] pointer.  The
    /// returned [PgHeapTuple] will be considered by have been allocated by Postgres and is not mutable
    /// until [PgHeapTuple::into_owned] is called.
    ///
    /// ## Safety
    ///
    /// This function is unsafe as we cannot guarantee that the [pg_sys::HeapTuple] pointer is valid,
    /// nor can we guaratee that the provided [PgTupleDesc] properly describes the structure of
    /// the heap tuple.
    pub unsafe fn from_heap_tuple(tupdesc: PgTupleDesc<'a>, heap_tuple: pg_sys::HeapTuple) -> Self {
        Self {
            tuple: PgBox::from_pg(heap_tuple),
            tupdesc,
        }
    }

    /// Creates a new [PgHeapTuple] from one of the two (`Current` or `New`) trigger tuples.  The returned
    /// [PgHeapTuple] will be considered by have been allocated by Postgres and is not mutable until
    /// [PgHeapTuple::into_owned] is called.  
    ///
    /// ## Errors
    /// - [PgHeapTupleCreationError::NullTuple] if the specified `TriggerTuple` is `null`
    ///
    /// ## Safety
    ///
    /// This function is unsafe as we cannot guarantee that any pointers in the `trigger_data`
    /// argument are valid.
    pub unsafe fn from_trigger_data(
        trigger_data: &'a pg_sys::TriggerData,
        which_tuple: TriggerTuple,
    ) -> PgHeapTupleCreationResult<'a, AllocatedByPostgres> {
        let tupdesc =
            PgTupleDesc::from_pg_unchecked(trigger_data.tg_relation.as_ref().unwrap().rd_att);

        let tuple = match which_tuple {
            TriggerTuple::Current => trigger_data.tg_trigtuple,
            TriggerTuple::New => trigger_data.tg_newtuple,
        };

        if tuple.is_null() {
            return Err(PgHeapTupleCreationError::NullTuple);
        }

        Ok(PgHeapTuple::from_heap_tuple(tupdesc, tuple))
    }

    /// Consumes a `[PgHeapTuple]` considered to be allocated by Postgres and transforms it into one
    /// that is considered allocated by Rust.  This is accomplished by copying the underlying [pg_sys::HeapTupleData].
    pub fn into_owned(self) -> PgHeapTuple<'a, AllocatedByRust> {
        let copy = unsafe { pg_sys::heap_copytuple(self.tuple.into_pg()) };
        PgHeapTuple {
            tuple: unsafe { PgBox::<pg_sys::HeapTupleData, AllocatedByRust>::from_rust(copy) },
            tupdesc: self.tupdesc,
        }
    }
}

impl<'a> PgHeapTuple<'a, AllocatedByRust> {
    /// Create a new [PgHeapTuple] from a [PgTupleDesc] from an iterator of Datums.
    ///
    /// ## Errors
    /// - [PgHeapTupleCreationError::IncorrectAttributeCount] if the number of items in the iterator
    /// does not match the number of attributes in the [PgTupleDesc].
    pub fn from_datums<I: IntoIterator<Item = Option<pg_sys::Datum>>>(
        tupdesc: PgTupleDesc<'a>,
        datums: I,
    ) -> PgHeapTupleCreationResult<'a, AllocatedByRust> {
        let iter = datums.into_iter();
        let mut datums = Vec::<pg_sys::Datum>::with_capacity(iter.size_hint().1.unwrap_or(1));
        let mut nulls = Vec::<bool>::with_capacity(iter.size_hint().1.unwrap_or(1));
        iter.for_each(|datum| {
            nulls.push(datum.is_none());
            datums.push(datum.unwrap_or(0.into()));
        });
        if datums.len() != tupdesc.len() {
            return Err(PgHeapTupleCreationError::IncorrectAttributeCount);
        }

        unsafe {
            let formed_tuple =
                pg_sys::heap_form_tuple(tupdesc.as_ptr(), datums.as_mut_ptr(), nulls.as_mut_ptr());

            Ok(Self {
                tuple: PgBox::<pg_sys::HeapTupleData, AllocatedByRust>::from_rust(formed_tuple),
                tupdesc,
            })
        }
    }

    /// Creates a new [PgHeapTuple] from an opaque Datum that should be a "composite" type.
    ///
    /// The Datum should be a pointer to a [pg_sys::HeapTupleHeader].  Typically, this will be used
    /// in situations when working with SQL `ROW(...)` constructors, or a composite SQL type such as
    ///
    /// ```sql
    /// CREATE TYPE my_composite AS (name text, age i32);
    /// ```
    ///
    /// ## Safety
    ///
    /// This function is unsafe as we cannot guarantee that the provided Datum is a valid [pg_sys::HeapTupleHeader]
    /// pointer.
    pub unsafe fn from_composite_datum(composite: pg_sys::Datum) -> Self {
        let htup_header = pg_sys::pg_detoast_datum(composite.ptr_cast()) as pg_sys::HeapTupleHeader;
        let tup_type = crate::heap_tuple_header_get_type_id(htup_header);
        let tup_typmod = crate::heap_tuple_header_get_typmod(htup_header);
        let tupdesc = pg_sys::lookup_rowtype_tupdesc(tup_type, tup_typmod);

        let mut data = PgBox::<pg_sys::HeapTupleData>::alloc0();

        data.t_len = crate::heap_tuple_header_get_datum_length(htup_header) as u32;
        data.t_data = htup_header;

        Self {
            tuple: data,
            tupdesc: PgTupleDesc::from_pg(tupdesc),
        }
    }

    /// Given the name for an attribute in this [PgHeapTuple], change its value.
    ///
    /// Attribute names are case sensitive.
    ///
    /// ## Errors
    ///
    /// - return [TryFromDatumError::NoSuchAttributeName] if the attribute does not exist
    /// - return [TryFromDatumError::IncompatibleTypes] if the Rust type of the `value` is not
    /// compatible with the attribute's Postgres type
    pub fn set_by_name<T: IntoDatum>(&mut self, attname: &str, value: T) -> FromDatumResult<()> {
        match self.get_attribute_by_name(attname) {
            None => Err(TryFromDatumError::NoSuchAttributeName(attname.to_string())),
            Some((attnum, _)) => self.set_by_index(attnum, value),
        }
    }

    /// Given the index for an attribute in this [PgHeapTuple], change its value.
    ///
    /// Attribute numbers start at 1, not 0.
    ///
    /// ## Errors
    /// - return [TryFromDatumError::NoSuchAttributeNumber] if the attribute does not exist
    /// - return [TryFromDatumError::IncompatibleTypes] if the Rust type of the `value` is not
    /// compatible with the attribute's Postgres type
    pub fn set_by_index<T: IntoDatum>(
        &mut self,
        attno: NonZeroUsize,
        value: T,
    ) -> FromDatumResult<()> {
        unsafe {
            match self.get_attribute_by_index(attno) {
                None => return Err(TryFromDatumError::NoSuchAttributeNumber(attno)),
                Some(att) => {
                    if !T::is_compatible_with(att.atttypid) {
                        return Err(TryFromDatumError::IncompatibleTypes);
                    }
                }
            }

            let mut datums = (0..self.tupdesc.len())
                .map(|i| pg_sys::Datum::from(i))
                .collect::<Vec<_>>();
            let mut nulls = (0..self.tupdesc.len()).map(|_| false).collect::<Vec<_>>();
            let mut do_replace = (0..self.tupdesc.len()).map(|_| false).collect::<Vec<_>>();

            let datum = value.into_datum();
            let attno = attno.get() - 1;

            nulls[attno] = datum.is_none();
            datums[attno] = datum.unwrap_or(0.into());
            do_replace[attno] = true;

            let new_tuple = PgBox::<pg_sys::HeapTupleData, AllocatedByRust>::from_rust(
                pg_sys::heap_modify_tuple(
                    self.tuple.as_ptr(),
                    self.tupdesc.as_ptr(),
                    datums.as_mut_ptr(),
                    nulls.as_mut_ptr(),
                    do_replace.as_mut_ptr(),
                ),
            );
            let old_tuple = std::mem::replace(&mut self.tuple, new_tuple);
            drop(old_tuple);
            Ok(Some(()))
        }
    }
}

impl<'a, AllocatedBy: WhoAllocated<pg_sys::HeapTupleData>> PgHeapTuple<'a, AllocatedBy> {
    /// Consume this [PgHeapTuple] and return a Datum representation, which is a pointer to the
    /// underlying [pg_sys::HeapTupleData] struct.
    pub fn into_datum(self) -> Option<pg_sys::Datum> {
        self.tuple.into_datum()
    }

    /// Consume this [PgHeapTuple] and return a Datum representation, which is a pointer to a
    /// [pg_sys::HeapTupleHeaderData] struct, containing the tuple data and the corresponding
    /// tuple descriptor information.
    pub fn into_composite_datum(self) -> Option<pg_sys::Datum> {
        unsafe {
            Some(pg_sys::heap_copy_tuple_as_datum(
                self.tuple.as_ptr(),
                self.tupdesc.as_ptr(),
            ))
        }
    }

    /// Returns the number of attributes in this [PgHeapTuple].
    #[inline]
    pub fn len(&self) -> usize {
        self.tupdesc.len()
    }

    /// Returns an iterator over the attributes in this [PgHeapTuple].
    ///
    /// The return value is `(attribute_number: NonZeroUsize, attribute_info: &pg_sys::FormData_pg_attribute)`.
    pub fn attributes(
        &'a self,
    ) -> impl std::iter::Iterator<Item = (NonZeroUsize, &'a pg_sys::FormData_pg_attribute)> {
        self.tupdesc
            .iter()
            .enumerate()
            .map(|(i, att)| (NonZeroUsize::new(i + 1).unwrap(), att))
    }

    /// Get the attribute information for the specified attribute number.  
    ///
    /// Returns `None` if the attribute number is out of bounds.
    #[inline]
    pub fn get_attribute_by_index(
        &'a self,
        index: NonZeroUsize,
    ) -> Option<&'a pg_sys::FormData_pg_attribute> {
        self.tupdesc.get(index.get() - 1)
    }

    /// Get the attribute information for the specified attribute, by name.
    ///
    /// Returns `None` if the attribute name is not found.
    pub fn get_attribute_by_name(
        &'a self,
        name: &str,
    ) -> Option<(NonZeroUsize, &'a pg_sys::FormData_pg_attribute)> {
        for i in 0..self.len() {
            let i = NonZeroUsize::new(i + 1).unwrap();
            let att = self.get_attribute_by_index(i).unwrap();
            if att.name() == name {
                return Some((i, att));
            }
        }

        None
    }

    /// Retrieve the value of the specified attribute, by name.
    ///
    /// Attribute names are case-insensitive.
    ///
    /// ## Errors
    /// - return [TryFromDatumError::NoSuchAttributeName] if the attribute does not exist
    /// - return [TryFromDatumError::IncompatibleTypes] if the Rust type of the `value` is not
    /// compatible with the attribute's Postgres type
    pub fn get_by_name<T: FromDatum + IntoDatum + 'static>(
        &self,
        attname: &str,
    ) -> FromDatumResult<T> {
        // find the attribute number by name
        for att in self.tupdesc.iter() {
            if att.name() == attname {
                // we found the named attribute, so go get it from the HeapTuple
                return self.get_by_index(NonZeroUsize::new(att.attnum as usize).unwrap());
            }
        }

        // no attribute with the specified name
        Err(TryFromDatumError::NoSuchAttributeName(attname.to_owned()))
    }

    /// Retrieve the value of the specified attribute, by index.
    ///
    /// Attribute numbers start at 1, not 0.
    ///
    /// ## Errors
    /// - return [TryFromDatumError::NoSuchAttributeNumber] if the attribute does not exist
    /// - return [TryFromDatumError::IncompatibleTypes] if the Rust type of the `value` is not
    /// compatible with the attribute's Postgres type
    pub fn get_by_index<T: FromDatum + IntoDatum + 'static>(
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
