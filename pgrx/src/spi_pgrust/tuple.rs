//! pgrust: `SpiTupleTable` / `SpiHeapTupleData` over pgrust's tuple table
//! handles. Rows are read through `tuptable_with`, so nothing here holds a
//! pointer into the table; column names and type OIDs are copied out.

use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Index;

use crate::memcxt::PgMemoryContexts;
use crate::pg_sys::panic::ErrorReportable;
use crate::pg_sys::pgrust::spi_native as spi;
use crate::prelude::*;

use super::{SpiError, SpiErrorCodes, SpiOkCodes, SpiResult};

#[derive(Debug)]
pub struct SpiTupleTable<'conn> {
    #[allow(dead_code)]
    pub(super) status_code: SpiOkCodes,
    pub(super) table: Option<spi::TuptabHandle>,
    pub(super) size: usize,
    pub(super) current: isize,
    pub(super) __marker: PhantomData<&'conn ()>,
}

impl<'conn> SpiTupleTable<'conn> {
    pub fn first(mut self) -> Self {
        self.current = 0;
        self
    }

    pub fn rewind(mut self) -> Self {
        self.current = -1;
        self
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_one<A: FromDatum + IntoDatum>(&self) -> SpiResult<Option<A>> {
        self.get(1)
    }

    pub fn get_two<A: FromDatum + IntoDatum, B: FromDatum + IntoDatum>(
        &self,
    ) -> SpiResult<(Option<A>, Option<B>)> {
        let a = self.get::<A>(1)?;
        let b = self.get::<B>(2)?;
        Ok((a, b))
    }

    pub fn get_three<
        A: FromDatum + IntoDatum,
        B: FromDatum + IntoDatum,
        C: FromDatum + IntoDatum,
    >(
        &self,
    ) -> SpiResult<(Option<A>, Option<B>, Option<C>)> {
        let a = self.get::<A>(1)?;
        let b = self.get::<B>(2)?;
        let c = self.get::<C>(3)?;
        Ok((a, b, c))
    }

    #[inline]
    fn with_table<R>(&self, f: impl for<'mcx> FnOnce(&spi::TuptabData<'mcx>) -> R) -> SpiResult<R> {
        let h = self.table.ok_or(SpiError::NoTupleTable)?;
        Ok(spi::tuptable_with(h, f))
    }

    pub fn get_heap_tuple(&self) -> SpiResult<Option<SpiHeapTupleData<'conn>>> {
        if self.size == 0 || self.table.is_none() {
            Ok(None)
        } else if self.current < 0 || self.current as usize >= self.size {
            Err(SpiError::InvalidPosition)
        } else {
            let cur = self.current as usize;
            self.with_table(|t| SpiHeapTupleData::from_row(&t.tupdesc, &t.vals[cur]))
                .map(Some)
        }
    }

    pub fn get<T: IntoDatum + FromDatum>(&self, ordinal: usize) -> SpiResult<Option<T>> {
        let datum = self.get_datum_by_ordinal(ordinal)?;
        let oid = self.column_type_oid(ordinal)?.value();
        match datum {
            Some(datum) => unsafe {
                // Copy out of the SPI table into the caller's context, which
                // outlives the connection.
                T::try_from_datum_in_memory_context(
                    PgMemoryContexts::CurrentMemoryContext,
                    datum,
                    false,
                    oid,
                )
                .map_err(SpiError::DatumError)
            },
            None => Ok(None),
        }
    }

    pub fn get_datum_by_ordinal(&self, ordinal: usize) -> SpiResult<Option<pg_sys::Datum>> {
        self.check_ordinal_bounds(ordinal)?;
        if self.current < 0 || self.current as usize >= self.size {
            return Err(SpiError::InvalidPosition);
        }
        let cur = self.current as usize;
        self.with_table(|t| {
            let (d, isnull) = spi::SPI_getbinval(&t.vals[cur], &t.tupdesc, ordinal as i32);
            if isnull {
                None
            } else {
                Some(pg_sys::Datum::from(d.as_usize()))
            }
        })
    }

    pub fn get_datum_by_name<S: AsRef<str>>(&self, name: S) -> SpiResult<Option<pg_sys::Datum>> {
        self.get_datum_by_ordinal(self.column_ordinal(name)?)
    }

    pub fn columns(&self) -> SpiResult<usize> {
        self.with_table(|t| t.tupdesc.natts as usize)
    }

    #[inline]
    fn check_ordinal_bounds(&self, ordinal: usize) -> SpiResult<()> {
        if ordinal < 1 || ordinal > self.columns()? {
            Err(SpiError::SpiError(SpiErrorCodes::NoAttribute))
        } else {
            Ok(())
        }
    }

    pub fn column_type_oid(&self, ordinal: usize) -> SpiResult<PgOid> {
        self.check_ordinal_bounds(ordinal)?;
        self.with_table(|t| {
            PgOid::from(pg_sys::Oid::from_u32(spi::SPI_gettypeid(
                &t.tupdesc,
                ordinal as i32,
            )))
        })
    }

    pub fn column_name(&self, ordinal: usize) -> SpiResult<String> {
        self.check_ordinal_bounds(ordinal)?;
        self.with_table(|t| {
            String::from_utf8_lossy(t.tupdesc.attrs[ordinal - 1].attname.name_str()).into_owned()
        })
    }

    pub fn column_ordinal<S: AsRef<str>>(&self, name: S) -> SpiResult<usize> {
        let n = self.with_table(|t| spi::SPI_fnumber(&t.tupdesc, name.as_ref()))?;
        if n == spi::SPI_ERROR_NOATTRIBUTE || n <= 0 {
            Err(SpiError::SpiError(SpiErrorCodes::NoAttribute))
        } else {
            Ok(n as usize)
        }
    }
}

impl<'conn> Iterator for SpiTupleTable<'conn> {
    type Item = SpiHeapTupleData<'conn>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= self.size as isize {
            None
        } else {
            assert!(self.current >= 0);
            self.get_heap_tuple().unwrap_or_report()
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.size))
    }
}

pub struct SpiHeapTupleDataEntry<'conn> {
    datum: Option<pg_sys::Datum>,
    type_oid: pg_sys::Oid,
    __marker: PhantomData<&'conn ()>,
}

/// A row copied out of a tuple table: its datums, type OIDs and column names.
pub struct SpiHeapTupleData<'conn> {
    names: Vec<String>,
    // offset by 1!
    entries: Vec<SpiHeapTupleDataEntry<'conn>>,
}

impl<'conn> SpiHeapTupleData<'conn> {
    fn from_row(
        tupdesc: &crate::pg_sys::pgrust::TupleDescNative<'_>,
        row: &crate::pg_sys::pgrust::HeapTupleNative<'_>,
    ) -> Self {
        let natts = tupdesc.natts;
        let mut data = SpiHeapTupleData {
            names: Vec::with_capacity(natts as usize),
            entries: Vec::with_capacity(natts as usize),
        };
        for i in 1..=natts {
            let (d, isnull) = spi::SPI_getbinval(row, tupdesc, i);
            data.entries.push(SpiHeapTupleDataEntry {
                datum: if isnull {
                    None
                } else {
                    Some(pg_sys::Datum::from(d.as_usize()))
                },
                type_oid: pg_sys::Oid::from_u32(spi::SPI_gettypeid(tupdesc, i)),
                __marker: PhantomData,
            });
            data.names.push(
                String::from_utf8_lossy(tupdesc.attrs[i as usize - 1].attname.name_str())
                    .into_owned(),
            );
        }
        data
    }

    pub fn get<T: IntoDatum + FromDatum>(&self, ordinal: usize) -> SpiResult<Option<T>> {
        self.get_datum_by_ordinal(ordinal)
            .map(|entry| entry.value())?
    }

    pub fn get_by_name<T: IntoDatum + FromDatum, S: AsRef<str>>(
        &self,
        name: S,
    ) -> SpiResult<Option<T>> {
        self.get_datum_by_name(name.as_ref())
            .map(|entry| entry.value())?
    }

    pub fn get_datum_by_ordinal(&self, ordinal: usize) -> SpiResult<&SpiHeapTupleDataEntry<'conn>> {
        let index = ordinal.wrapping_sub(1);
        self.entries
            .get(index)
            .ok_or(SpiError::SpiError(SpiErrorCodes::NoAttribute))
    }

    pub fn get_datum_by_name<S: AsRef<str>>(
        &self,
        name: S,
    ) -> SpiResult<&SpiHeapTupleDataEntry<'conn>> {
        match self.names.iter().position(|n| n == name.as_ref()) {
            Some(i) => self.get_datum_by_ordinal(i + 1),
            None => Err(SpiError::SpiError(SpiErrorCodes::NoAttribute)),
        }
    }

    pub fn set_by_ordinal<T: IntoDatum>(&mut self, ordinal: usize, datum: T) -> SpiResult<()> {
        self.check_ordinal_bounds(ordinal)?;
        self.entries[ordinal - 1] = SpiHeapTupleDataEntry {
            datum: datum.into_datum(),
            type_oid: T::type_oid(),
            __marker: PhantomData,
        };
        Ok(())
    }

    pub fn set_by_name<T: IntoDatum>(&mut self, name: &str, datum: T) -> SpiResult<()> {
        match self.names.iter().position(|n| n == name) {
            Some(i) => self.set_by_ordinal(i + 1, datum),
            None => Err(SpiError::SpiError(SpiErrorCodes::NoAttribute)),
        }
    }

    #[inline]
    pub fn columns(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    fn check_ordinal_bounds(&self, ordinal: usize) -> SpiResult<()> {
        if ordinal < 1 || ordinal > self.columns() {
            Err(SpiError::SpiError(SpiErrorCodes::NoAttribute))
        } else {
            Ok(())
        }
    }
}

impl<'conn> SpiHeapTupleDataEntry<'conn> {
    pub fn value<T: IntoDatum + FromDatum>(&self) -> SpiResult<Option<T>> {
        match self.datum.as_ref() {
            Some(datum) => unsafe {
                T::try_from_datum_in_memory_context(
                    PgMemoryContexts::CurrentMemoryContext,
                    *datum,
                    false,
                    self.type_oid,
                )
                .map_err(SpiError::DatumError)
            },
            None => Ok(None),
        }
    }

    pub fn oid(&self) -> pg_sys::Oid {
        self.type_oid
    }
}

impl<'conn> Index<usize> for SpiHeapTupleData<'conn> {
    type Output = SpiHeapTupleDataEntry<'conn>;
    fn index(&self, index: usize) -> &Self::Output {
        self.get_datum_by_ordinal(index)
            .expect("invalid ordinal value")
    }
}

impl<'conn> Index<&str> for SpiHeapTupleData<'conn> {
    type Output = SpiHeapTupleDataEntry<'conn>;
    fn index(&self, index: &str) -> &Self::Output {
        self.get_datum_by_name(index).expect("invalid field name")
    }
}
