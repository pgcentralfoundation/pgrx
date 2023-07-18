use std::ffi::CString;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::pg_sys::{self, PgOid};
use crate::spi::{PreparedStatement, Query, Spi, SpiCursor, SpiError, SpiResult, SpiTupleTable};

pub struct SpiClient {
    // We need `SpiClient` to be publicly accessible but not constructable because we rely
    // on it being properly constructed in order for its Drop impl, which calles `pg_sys::SPI_finish()`,
    // to work as expected
    _priv_constructor: (),
}

impl SpiClient {
    /// Connect to Postgres' SPI system
    pub(super) fn connect() -> SpiResult<Self> {
        // connect to SPI
        //
        // SPI_connect() is documented as being able to return SPI_ERROR_CONNECT, so we have to
        // assume it could.  The truth seems to be that it never actually does.  The one user
        // of SpiConnection::connect() returns `spi::Result` anyways, so it's no big deal
        Spi::check_status(unsafe { pg_sys::SPI_connect() })?;
        Ok(SpiClient { _priv_constructor: () })
    }

    /// Prepares a statement that is valid for the lifetime of the client
    ///
    /// # Panics
    ///
    /// This function will panic if the supplied `query` string contained a NULL byte
    pub fn prepare(&self, query: &str, args: Option<Vec<PgOid>>) -> SpiResult<PreparedStatement> {
        let src = CString::new(query).expect("query contained a null byte");
        let args = args.unwrap_or_default();
        let nargs = args.len();

        // SAFETY: all arguments are prepared above
        let plan = unsafe {
            pg_sys::SPI_prepare(
                src.as_ptr(),
                nargs as i32,
                args.into_iter().map(PgOid::value).collect::<Vec<_>>().as_mut_ptr(),
            )
        };
        Ok(PreparedStatement {
            plan: NonNull::new(plan).ok_or_else(|| {
                Spi::check_status(unsafe {
                    // SAFETY: no concurrent usage
                    pg_sys::SPI_result
                })
                .err()
                .unwrap()
            })?,
            __marker: PhantomData,
        })
    }

    /// perform a SELECT statement
    pub fn select<'client, Q: Query<'client>>(
        &'client self,
        query: Q,
        limit: Option<libc::c_long>,
        args: Q::Arguments,
    ) -> SpiResult<SpiTupleTable<'client>> {
        query.execute(self, limit, args)
    }

    /// perform any query (including utility statements) that modify the database in some way
    pub fn update<'client, Q: Query<'client>>(
        &'client mut self,
        query: Q,
        limit: Option<libc::c_long>,
        args: Q::Arguments,
    ) -> SpiResult<SpiTupleTable<'client>> {
        Spi::mark_mutable();
        query.execute(self, limit, args)
    }

    pub(super) fn prepare_tuple_table(
        &self,
        status_code: i32,
    ) -> std::result::Result<SpiTupleTable, SpiError> {
        Ok(SpiTupleTable {
            status_code: Spi::check_status(status_code)?,
            // SAFETY: no concurrent access
            table: unsafe { pg_sys::SPI_tuptable.as_mut()},
            #[cfg(any(feature = "pg11", feature = "pg12"))]
            size: unsafe { pg_sys::SPI_processed as usize },
            #[cfg(not(any(feature = "pg11", feature = "pg12")))]
            // SAFETY: no concurrent access
            size: unsafe {
                if pg_sys::SPI_tuptable.is_null() {
                    pg_sys::SPI_processed as usize
                } else {
                    (*pg_sys::SPI_tuptable).numvals as usize
                }
            },
            current: -1,
        })
    }

    /// Set up a cursor that will execute the specified query
    ///
    /// Rows may be then fetched using [`SpiCursor::fetch`].
    ///
    /// See [`SpiCursor`] docs for usage details.
    pub fn open_cursor<'client, Q: Query<'client>>(
        &'client self,
        query: Q,
        args: Q::Arguments,
    ) -> SpiCursor<'client> {
        query.open_cursor(&self, args)
    }

    /// Set up a cursor that will execute the specified update (mutating) query
    ///
    /// Rows may be then fetched using [`SpiCursor::fetch`].
    ///
    /// See [`SpiCursor`] docs for usage details.
    pub fn open_cursor_mut<'client, Q: Query<'client>>(
        &'client mut self,
        query: Q,
        args: Q::Arguments,
    ) -> SpiCursor<'client> {
        Spi::mark_mutable();
        query.open_cursor(self, args)
    }

    /// Find a cursor in transaction by name
    ///
    /// A cursor for a query can be opened using [`SpiClient::open_cursor`].
    /// Cursor are automatically closed on drop unless [`SpiCursor::detach_into_name`] is used.
    /// Returned name can be used with this method to retrieve the open cursor.
    ///
    /// See [`SpiCursor`] docs for usage details.
    pub fn find_cursor(&self, name: &str) -> SpiResult<SpiCursor> {
        use pgrx_pg_sys::AsPgCStr;

        let ptr = NonNull::new(unsafe { pg_sys::SPI_cursor_find(name.as_pg_cstr()) })
            .ok_or(SpiError::CursorNotFound(name.to_string()))?;
        Ok(SpiCursor { ptr, client: self })
    }
}

impl Drop for SpiClient {
    fn drop(&mut self) {
        // best efforts to disconnect from SPI
        // SPI_finish() would only complain if we hadn't previously called SPI_connect() and
        // SpiClient will prevent that from happening (assuming users don't go unsafe{})
        Spi::check_status(unsafe { pg_sys::SPI_finish() }).ok();
    }
}
