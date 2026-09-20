//! pgrust: `SpiCursor` over pgrust's SPI cursors (portal-backed).

use std::marker::PhantomData;

use crate::pg_sys::pgrust::spi_native as spi;

use super::{SpiClient, SpiOkCodes, SpiResult, SpiTupleTable};

type CursorName = String;

pub struct SpiCursor<'client> {
    pub(crate) inner: Option<spi::SpiCursor>,
    pub(crate) name: String,
    pub(crate) __marker: PhantomData<&'client SpiClient<'client>>,
}

impl SpiCursor<'_> {
    pub fn fetch(&mut self, count: libc::c_long) -> SpiResult<SpiTupleTable<'_>> {
        let cursor = self.inner.as_ref().expect("cursor was detached");
        crate::pg_sys::pgrust::unwrap_pg(spi::SPI_cursor_fetch(cursor, true, count));
        SpiClient::prepare_tuple_table(SpiOkCodes::Fetch as i32)
    }

    /// Detach: the portal stays open under its name; the cursor object is
    /// forgotten without closing it.
    pub fn detach_into_name(mut self) -> CursorName {
        let name = std::mem::take(&mut self.name);
        if let Some(inner) = self.inner.take() {
            std::mem::forget(inner);
        }
        std::mem::forget(self);
        name
    }
}

impl Drop for SpiCursor<'_> {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            let _ = spi::SPI_cursor_close(inner);
        }
    }
}
