use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::pg_sys::{self, PgOid};
use crate::spi::{PreparedStatement, Query, Spi, SpiCursor, SpiError, SpiResult, SpiTupleTable};

use super::query::PreparableQuery;

// TODO: should `'conn` be invariant?
pub struct SpiClient<'conn> {
    __marker: PhantomData<&'conn SpiConnection>,
}

impl<'conn> SpiClient<'conn> {
    /// Prepares a statement that is valid for the lifetime of the client
    pub fn prepare<Q: PreparableQuery<'conn>>(
        &self,
        query: Q,
        args: Option<Vec<PgOid>>,
    ) -> SpiResult<PreparedStatement<'conn>> {
        query.prepare(self, args)
    }

    /// Prepares a mutating statement that is valid for the lifetime of the client
    pub fn prepare_mut<Q: PreparableQuery<'conn>>(
        &self,
        query: Q,
        args: Option<Vec<PgOid>>,
    ) -> SpiResult<PreparedStatement<'conn>> {
        query.prepare_mut(self, args)
    }

    /// perform a SELECT statement
    pub fn select<Q: Query<'conn>>(
        &self,
        query: Q,
        limit: Option<libc::c_long>,
        args: Q::Arguments,
    ) -> SpiResult<SpiTupleTable<'conn>> {
        query.execute(self, limit, args)
    }

    /// perform any query (including utility statements) that modify the database in some way
    pub fn update<Q: Query<'conn>>(
        &mut self,
        query: Q,
        limit: Option<libc::c_long>,
        args: Q::Arguments,
    ) -> SpiResult<SpiTupleTable<'conn>> {
        Spi::mark_mutable();
        query.execute(self, limit, args)
    }

    pub(super) fn prepare_tuple_table(
        status_code: i32,
    ) -> std::result::Result<SpiTupleTable<'conn>, SpiError> {
        Ok(SpiTupleTable {
            status_code: Spi::check_status(status_code)?,
            // SAFETY: no concurrent access
            table: unsafe { pg_sys::SPI_tuptable.as_mut()},
            #[cfg(feature = "pg12")]
            size: unsafe { pg_sys::SPI_processed as usize },
            #[cfg(not(feature = "pg12"))]
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
    ///
    /// See [`try_open_cursor`][Self::try_open_cursor] which will return an [`SpiError`] rather than panicking.
    ///
    /// # Panics
    ///
    /// Panics if a cursor wasn't opened.
    pub fn open_cursor<Q: Query<'conn>>(&self, query: Q, args: Q::Arguments) -> SpiCursor<'conn> {
        self.try_open_cursor(query, args).unwrap()
    }

    /// Set up a cursor that will execute the specified query
    ///
    /// Rows may be then fetched using [`SpiCursor::fetch`].
    ///
    /// See [`SpiCursor`] docs for usage details.
    pub fn try_open_cursor<Q: Query<'conn>>(
        &self,
        query: Q,
        args: Q::Arguments,
    ) -> SpiResult<SpiCursor<'conn>> {
        query.try_open_cursor(self, args)
    }

    /// Set up a cursor that will execute the specified update (mutating) query
    ///
    /// Rows may be then fetched using [`SpiCursor::fetch`].
    ///
    /// See [`SpiCursor`] docs for usage details.
    ///
    /// See [`try_open_cursor_mut`][Self::try_open_cursor_mut] which will return an [`SpiError`] rather than panicking.
    ///
    /// # Panics
    ///
    /// Panics if a cursor wasn't opened.
    pub fn open_cursor_mut<Q: Query<'conn>>(
        &mut self,
        query: Q,
        args: Q::Arguments,
    ) -> SpiCursor<'conn> {
        Spi::mark_mutable();
        self.try_open_cursor_mut(query, args).unwrap()
    }

    /// Set up a cursor that will execute the specified update (mutating) query
    ///
    /// Rows may be then fetched using [`SpiCursor::fetch`].
    ///
    /// See [`SpiCursor`] docs for usage details.
    pub fn try_open_cursor_mut<Q: Query<'conn>>(
        &mut self,
        query: Q,
        args: Q::Arguments,
    ) -> SpiResult<SpiCursor<'conn>> {
        Spi::mark_mutable();
        query.try_open_cursor(self, args)
    }

    /// Find a cursor in transaction by name
    ///
    /// A cursor for a query can be opened using [`SpiClient::open_cursor`].
    /// Cursor are automatically closed on drop unless [`SpiCursor::detach_into_name`] is used.
    /// Returned name can be used with this method to retrieve the open cursor.
    ///
    /// See [`SpiCursor`] docs for usage details.
    pub fn find_cursor(&self, name: &str) -> SpiResult<SpiCursor<'conn>> {
        use pgrx_pg_sys::AsPgCStr;

        let ptr = NonNull::new(unsafe { pg_sys::SPI_cursor_find(name.as_pg_cstr()) })
            .ok_or(SpiError::CursorNotFound(name.to_string()))?;
        Ok(SpiCursor { ptr, __marker: PhantomData })
    }
}

/// a struct to manage our SPI connection lifetime
pub(super) struct SpiConnection(PhantomData<*mut ()>);

impl SpiConnection {
    /// Connect to Postgres' SPI system
    pub(super) fn connect() -> SpiResult<Self> {
        // connect to SPI
        //
        // SPI_connect() is documented as being able to return SPI_ERROR_CONNECT, so we have to
        // assume it could.  The truth seems to be that it never actually does.  The one user
        // of SpiConnection::connect() returns `spi::Result` anyways, so it's no big deal
        Spi::check_status(unsafe { pg_sys::SPI_connect() })?;
        Ok(SpiConnection(PhantomData))
    }

    pub(super) fn connect_non_atomic() -> SpiResult<Self> {
        Spi::check_status(unsafe { pg_sys::SPI_connect_ext(pg_sys::SPI_OPT_NONATOMIC as i32) })?;
        Ok(SpiConnection(PhantomData))
    }
}

impl Drop for SpiConnection {
    /// when SpiConnection is dropped, we make sure to disconnect from SPI
    fn drop(&mut self) {
        // best efforts to disconnect from SPI
        // SPI_finish() would only complain if we hadn't previously called SPI_connect() and
        // SpiConnection should prevent that from happening (assuming users don't go unsafe{})
        Spi::check_status(unsafe { pg_sys::SPI_finish() }).ok();
    }
}

impl SpiConnection {
    /// Return a client that with a lifetime scoped to this connection.
    pub(super) fn client(&self) -> SpiClient<'_> {
        SpiClient { __marker: PhantomData }
    }

    pub(super) fn transaction(&self) -> SpiTransaction<'_> {
        SpiTransaction { _conn: PhantomData }
    }
}

/// Represents an SPI transaction.
pub struct SpiTransaction<'conn> {
    _conn: PhantomData<&'conn SpiConnection>,
}

impl<'conn> SpiTransaction<'conn> {
    /// Commits back the transaction and starts a new `SpiTransaction` with default transaction characteristics.
    pub fn commit(self) -> Self {
        unsafe { pg_sys::SPI_commit() };
        self
    }

    /// Commits back the transaction and starts a new `SpiTransaction` with the same characteristics as the just finished one.
    pub fn commit_and_chain(self) -> Self {
        unsafe { pg_sys::SPI_commit_and_chain() };
        self
    }

    /// Rolls back the transaction and starts a new `SpiTransaction` with default transaction characteristics.
    pub fn rollback(self) -> Self {
        unsafe { pg_sys::SPI_rollback() };
        self
    }

    /// Rolls back the transaction and starts a new `SpiTransaction` with the same characteristics as the just finished one.
    pub fn rollback_and_chain(self) -> Self {
        unsafe { pg_sys::SPI_rollback_and_chain() };
        self
    }
}
