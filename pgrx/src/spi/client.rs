use std::ffi::{CStr, CString};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, Index};
use std::ptr::NonNull;

use crate::spi::Result as SpiResult;
use crate::spi::{Spi, PreparedStatement};
use crate::pg_sys::{self, PgOid};

// TODO: should `'conn` be invariant?
pub struct SpiClient<'conn> {
    __marker: PhantomData<&'conn SpiConnection>,
}

impl<'conn> SpiClient<'conn> {
    /// Prepares a statement that is valid for the lifetime of the client
    ///
    /// # Panics
    ///
    /// This function will panic if the supplied `query` string contained a NULL byte
    pub fn prepare(
        &self,
        query: &str,
        args: Option<Vec<PgOid>>,
    ) -> SpiResult<PreparedStatement<'conn>> {
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
}
