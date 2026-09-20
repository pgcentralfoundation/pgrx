//! pgrust: `SpiClient` over pgrust's SPI crate (connection + tuple table).

use std::marker::PhantomData;

use crate::datum::DatumWithOid;
use crate::pg_sys::PgOid;
use crate::pg_sys::pgrust::spi_native as spi;
use crate::spi::{PreparedStatement, Query, Spi, SpiCursor, SpiError, SpiResult, SpiTupleTable};

use super::query::PreparableQuery;

pub struct SpiClient<'conn> {
    __marker: PhantomData<&'conn ()>,
}

impl<'conn> SpiClient<'conn> {
    pub(super) fn connect() -> SpiResult<Self> {
        let status = crate::pg_sys::pgrust::unwrap_pg(spi::SPI_connect());
        Spi::check_status(status)?;
        Ok(SpiClient {
            __marker: PhantomData,
        })
    }

    pub fn prepare<Q: PreparableQuery<'conn>>(
        &self,
        query: Q,
        args: &[PgOid],
    ) -> SpiResult<PreparedStatement<'conn>> {
        query.prepare(self, args)
    }

    pub fn prepare_mut<Q: PreparableQuery<'conn>>(
        &self,
        query: Q,
        args: &[PgOid],
    ) -> SpiResult<PreparedStatement<'conn>> {
        query.prepare_mut(self, args)
    }

    pub fn select<'mcx, Q: Query<'conn>>(
        &self,
        query: Q,
        limit: Option<libc::c_long>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiTupleTable<'conn>> {
        query.execute(self, limit, args)
    }

    pub fn update<'mcx, Q: Query<'conn>>(
        &mut self,
        query: Q,
        limit: Option<libc::c_long>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiTupleTable<'conn>> {
        Spi::mark_mutable();
        query.execute(self, limit, args)
    }

    /// The result table of the SPI call that just returned `status_code`.
    pub(super) fn prepare_tuple_table(
        status_code: i32,
    ) -> std::result::Result<SpiTupleTable<'conn>, SpiError> {
        let table = spi::SPI_tuptable();
        let size = match table {
            Some(h) => spi::tuptable_with(h, |t| t.vals.len()),
            None => spi::SPI_processed() as usize,
        };
        Ok(SpiTupleTable {
            status_code: Spi::check_status(status_code)?,
            table,
            size,
            current: -1,
            __marker: PhantomData,
        })
    }

    pub fn open_cursor<'mcx, Q: Query<'conn>>(
        &self,
        query: Q,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiCursor<'conn> {
        self.try_open_cursor(query, args).unwrap()
    }

    pub fn try_open_cursor<'mcx, Q: Query<'conn>>(
        &self,
        query: Q,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiCursor<'conn>> {
        query.try_open_cursor(self, args)
    }

    pub fn open_cursor_mut<'mcx, Q: Query<'conn>>(
        &mut self,
        query: Q,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiCursor<'conn> {
        Spi::mark_mutable();
        self.try_open_cursor_mut(query, args).unwrap()
    }

    pub fn try_open_cursor_mut<'mcx, Q: Query<'conn>>(
        &mut self,
        query: Q,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiCursor<'conn>> {
        Spi::mark_mutable();
        query.try_open_cursor(self, args)
    }

    pub fn find_cursor(&self, name: &str) -> SpiResult<SpiCursor<'conn>> {
        let inner = spi::SPI_cursor_find(name).ok_or(SpiError::CursorNotFound(name.to_string()))?;
        Ok(SpiCursor {
            inner: Some(inner),
            name: name.to_string(),
            __marker: PhantomData,
        })
    }
}

impl Drop for SpiClient<'_> {
    fn drop(&mut self) {
        if let Ok(status) = spi::SPI_finish() {
            Spi::check_status(status).ok();
        }
    }
}
