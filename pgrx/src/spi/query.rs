use std::ffi::CString;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

use libc::c_char;

use super::{Spi, SpiClient, SpiCursor, SpiError, SpiResult, SpiTupleTable};
use crate::pg_sys::{self, PgOid};

/// A generalized interface to what constitutes a query
///
/// Its primary purpose is to abstract away differences between
/// one-off statements and prepared statements, but it can potentially
/// be implemented for other types, provided they can be converted into a query.
pub trait Query<'conn> {
    type Arguments;

    /// Execute a query given a client and other arguments
    fn execute(
        self,
        client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> SpiResult<SpiTupleTable<'conn>>;

    /// Open a cursor for the query
    fn open_cursor(self, client: &SpiClient<'conn>, args: Self::Arguments) -> SpiCursor<'conn>;
}

impl<'conn> Query<'conn> for &String {
    type Arguments = Option<Vec<(PgOid, Option<pg_sys::Datum>)>>;

    fn execute(
        self,
        client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> SpiResult<SpiTupleTable<'conn>> {
        self.as_str().execute(client, limit, arguments)
    }

    fn open_cursor(self, client: &SpiClient<'conn>, args: Self::Arguments) -> SpiCursor<'conn> {
        self.as_str().open_cursor(client, args)
    }
}

fn prepare_datum(datum: Option<pg_sys::Datum>) -> (pg_sys::Datum, std::os::raw::c_char) {
    match datum {
        Some(datum) => (datum, ' ' as std::os::raw::c_char),
        None => (pg_sys::Datum::from(0usize), 'n' as std::os::raw::c_char),
    }
}

fn args_to_datums(
    args: Vec<(PgOid, Option<pg_sys::Datum>)>,
) -> (Vec<pg_sys::Oid>, Vec<pg_sys::Datum>, Vec<c_char>) {
    let mut argtypes = Vec::with_capacity(args.len());
    let mut datums = Vec::with_capacity(args.len());
    let mut nulls = Vec::with_capacity(args.len());

    for (types, maybe_datum) in args {
        let (datum, null) = prepare_datum(maybe_datum);

        argtypes.push(types.value());
        datums.push(datum);
        nulls.push(null);
    }

    (argtypes, datums, nulls)
}

impl<'conn> Query<'conn> for &str {
    type Arguments = Option<Vec<(PgOid, Option<pg_sys::Datum>)>>;

    /// # Panics
    ///
    /// This function will panic if somehow the specified query contains a null byte.
    fn execute(
        self,
        _client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> SpiResult<SpiTupleTable<'conn>> {
        // SAFETY: no concurrent access
        unsafe {
            pg_sys::SPI_tuptable = std::ptr::null_mut();
        }

        let src = CString::new(self).expect("query contained a null byte");
        let status_code = match arguments {
            Some(args) => {
                let nargs = args.len();
                let (mut argtypes, mut datums, nulls) = args_to_datums(args);

                // SAFETY: arguments are prepared above
                unsafe {
                    pg_sys::SPI_execute_with_args(
                        src.as_ptr(),
                        nargs as i32,
                        argtypes.as_mut_ptr(),
                        datums.as_mut_ptr(),
                        nulls.as_ptr(),
                        Spi::is_xact_still_immutable(),
                        limit.unwrap_or(0),
                    )
                }
            }
            // SAFETY: arguments are prepared above
            None => unsafe {
                pg_sys::SPI_execute(
                    src.as_ptr(),
                    Spi::is_xact_still_immutable(),
                    limit.unwrap_or(0),
                )
            },
        };

        SpiClient::prepare_tuple_table(status_code)
    }

    fn open_cursor(self, _client: &SpiClient<'conn>, args: Self::Arguments) -> SpiCursor<'conn> {
        let src = CString::new(self).expect("query contained a null byte");
        let args = args.unwrap_or_default();

        let nargs = args.len();
        let (mut argtypes, mut datums, nulls) = args_to_datums(args);

        let ptr = unsafe {
            // SAFETY: arguments are prepared above and SPI_cursor_open_with_args will never return
            // the null pointer.  It'll raise an ERROR if something is invalid for it to create the cursor
            NonNull::new_unchecked(pg_sys::SPI_cursor_open_with_args(
                std::ptr::null_mut(), // let postgres assign a name
                src.as_ptr(),
                nargs as i32,
                argtypes.as_mut_ptr(),
                datums.as_mut_ptr(),
                nulls.as_ptr(),
                Spi::is_xact_still_immutable(),
                0,
            ))
        };
        SpiCursor { ptr, __marker: PhantomData }
    }
}

/// Client lifetime-bound prepared statement
pub struct PreparedStatement<'conn> {
    pub(super) plan: NonNull<pg_sys::_SPI_plan>,
    pub(super) __marker: PhantomData<&'conn ()>,
}

/// Static lifetime-bound prepared statement
pub struct OwnedPreparedStatement(PreparedStatement<'static>);

impl Deref for OwnedPreparedStatement {
    type Target = PreparedStatement<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for OwnedPreparedStatement {
    fn drop(&mut self) {
        unsafe {
            pg_sys::SPI_freeplan(self.0.plan.as_ptr());
        }
    }
}

impl<'conn> Query<'conn> for &OwnedPreparedStatement {
    type Arguments = Option<Vec<Option<pg_sys::Datum>>>;

    fn execute(
        self,
        client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> SpiResult<SpiTupleTable<'conn>> {
        (&self.0).execute(client, limit, arguments)
    }

    fn open_cursor(self, client: &SpiClient<'conn>, args: Self::Arguments) -> SpiCursor<'conn> {
        (&self.0).open_cursor(client, args)
    }
}

impl<'conn> Query<'conn> for OwnedPreparedStatement {
    type Arguments = Option<Vec<Option<pg_sys::Datum>>>;

    fn execute(
        self,
        client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> SpiResult<SpiTupleTable<'conn>> {
        (&self.0).execute(client, limit, arguments)
    }

    fn open_cursor(self, client: &SpiClient<'conn>, args: Self::Arguments) -> SpiCursor<'conn> {
        (&self.0).open_cursor(client, args)
    }
}

impl<'conn> PreparedStatement<'conn> {
    /// Converts prepared statement into an owned prepared statement
    ///
    /// These statements have static lifetime and are freed only when dropped
    pub fn keep(self) -> OwnedPreparedStatement {
        // SAFETY: self.plan is initialized in `SpiClient::prepare` and `PreparedStatement`
        // is consumed. If it wasn't consumed, a subsequent call to `keep` would trigger
        // an SPI_ERROR_ARGUMENT as per `SPI_keepplan` implementation.
        unsafe {
            pg_sys::SPI_keepplan(self.plan.as_ptr());
        }
        OwnedPreparedStatement(PreparedStatement { __marker: PhantomData, plan: self.plan })
    }
}

impl<'conn: 'stmt, 'stmt> Query<'conn> for &'stmt PreparedStatement<'conn> {
    type Arguments = Option<Vec<Option<pg_sys::Datum>>>;

    fn execute(
        self,
        _client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> SpiResult<SpiTupleTable<'conn>> {
        // SAFETY: no concurrent access
        unsafe {
            pg_sys::SPI_tuptable = std::ptr::null_mut();
        }
        let args = arguments.unwrap_or_default();
        let nargs = args.len();

        let expected = unsafe { pg_sys::SPI_getargcount(self.plan.as_ptr()) } as usize;

        if nargs != expected {
            return Err(SpiError::PreparedStatementArgumentMismatch { expected, got: nargs });
        }

        let (mut datums, mut nulls): (Vec<_>, Vec<_>) = args.into_iter().map(prepare_datum).unzip();

        // SAFETY: all arguments are prepared above
        let status_code = unsafe {
            pg_sys::SPI_execute_plan(
                self.plan.as_ptr(),
                datums.as_mut_ptr(),
                nulls.as_mut_ptr(),
                Spi::is_xact_still_immutable(),
                limit.unwrap_or(0),
            )
        };

        SpiClient::prepare_tuple_table(status_code)
    }

    fn open_cursor(self, _client: &SpiClient<'conn>, args: Self::Arguments) -> SpiCursor<'conn> {
        let args = args.unwrap_or_default();

        let (mut datums, nulls): (Vec<_>, Vec<_>) = args.into_iter().map(prepare_datum).unzip();

        // SAFETY: arguments are prepared above and SPI_cursor_open will never return the null
        // pointer.  It'll raise an ERROR if something is invalid for it to create the cursor
        let ptr = unsafe {
            NonNull::new_unchecked(pg_sys::SPI_cursor_open(
                std::ptr::null_mut(), // let postgres assign a name
                self.plan.as_ptr(),
                datums.as_mut_ptr(),
                nulls.as_ptr(),
                Spi::is_xact_still_immutable(),
            ))
        };
        SpiCursor { ptr, __marker: PhantomData }
    }
}

impl<'conn> Query<'conn> for PreparedStatement<'conn> {
    type Arguments = Option<Vec<Option<pg_sys::Datum>>>;

    fn execute(
        self,
        client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> SpiResult<SpiTupleTable<'conn>> {
        (&self).execute(client, limit, arguments)
    }

    fn open_cursor(self, client: &SpiClient<'conn>, args: Self::Arguments) -> SpiCursor<'conn> {
        (&self).open_cursor(client, args)
    }
}
