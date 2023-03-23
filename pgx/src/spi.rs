/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! Safe access to Postgres' *Server Programming Interface* (SPI).

use crate::{
    pg_sys, register_xact_callback, FromDatum, IntoDatum, Json, PgMemoryContexts, PgOid,
    PgXactCallbackEvent, TryFromDatumError,
};
use core::fmt::Formatter;
use pgx_pg_sys::panic::ErrorReportable;
use std::ffi::{CStr, CString};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, Index};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};

pub type Result<T> = std::result::Result<T, Error>;

/// These match the Postgres `#define`d constants prefixed `SPI_OK_*` that you can find in `pg_sys`.
#[derive(Debug, PartialEq)]
#[repr(i32)]
#[non_exhaustive]
pub enum SpiOkCodes {
    Connect = 1,
    Finish = 2,
    Fetch = 3,
    Utility = 4,
    Select = 5,
    SelInto = 6,
    Insert = 7,
    Delete = 8,
    Update = 9,
    Cursor = 10,
    InsertReturning = 11,
    DeleteReturning = 12,
    UpdateReturning = 13,
    Rewritten = 14,
    RelRegister = 15,
    RelUnregister = 16,
    TdRegister = 17,
    /// Added in Postgres 15
    Merge = 18,
}

/// These match the Postgres `#define`d constants prefixed `SPI_ERROR_*` that you can find in `pg_sys`.
/// It is hypothetically possible for a Postgres-defined status code to be `0`, AKA `NULL`, however,
/// this should not usually occur in Rust code paths. If it does happen, please report such bugs to the pgx repo.
#[derive(thiserror::Error, Debug, PartialEq)]
#[repr(i32)]
pub enum SpiErrorCodes {
    Connect = -1,
    Copy = -2,
    OpUnknown = -3,
    Unconnected = -4,
    #[allow(dead_code)]
    Cursor = -5, /* not used anymore */
    Argument = -6,
    Param = -7,
    Transaction = -8,
    NoAttribute = -9,
    NoOutFunc = -10,
    TypUnknown = -11,
    RelDuplicate = -12,
    RelNotFound = -13,
}

impl std::fmt::Display for SpiErrorCodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

/// A safe wrapper around [`pg_sys::quote_identifier`]. Returns a properly quoted identifier. For
/// instance for a column or table name such as `"my-table-name"`
pub fn quote_identifier<StringLike: AsRef<str>>(ident: StringLike) -> String {
    let ident_cstr = CString::new(ident.as_ref()).unwrap();
    // SAFETY: quote_identifier expects a null terminated string and returns one.
    let quoted_cstr = unsafe {
        let quoted_ptr = pg_sys::quote_identifier(ident_cstr.as_ptr());
        CStr::from_ptr(quoted_ptr)
    };
    quoted_cstr.to_str().unwrap().to_string()
}

/// A safe wrapper around [`pg_sys::quote_qualified_identifier`]. Returns a properly quoted name of
/// the following format qualifier.ident. A common usecase is to qualify a table_name for example
/// `"my schema"."my table"`
pub fn quote_qualified_identifier<StringLike: AsRef<str>>(
    qualifier: StringLike,
    ident: StringLike,
) -> String {
    let qualifier_cstr = CString::new(qualifier.as_ref()).unwrap();
    let ident_cstr = CString::new(ident.as_ref()).unwrap();
    // SAFETY: quote_qualified_identifier expects null terminated strings and returns one.
    let quoted_cstr = unsafe {
        let quoted_ptr =
            pg_sys::quote_qualified_identifier(qualifier_cstr.as_ptr(), ident_cstr.as_ptr());
        CStr::from_ptr(quoted_ptr)
    };
    quoted_cstr.to_str().unwrap().to_string()
}

/// A safe wrapper around [`pg_sys::quote_literal_cstr`]. Returns a properly quoted literal such as
/// a `TEXT` literal like `'my string with spaces'`.
pub fn quote_literal<StringLike: AsRef<str>>(literal: StringLike) -> String {
    let literal_cstr = CString::new(literal.as_ref()).unwrap();
    // SAFETY: quote_literal_cstr expects a null terminated string and returns one.
    let quoted_cstr = unsafe {
        let quoted_ptr = pg_sys::quote_literal_cstr(literal_cstr.as_ptr());
        CStr::from_ptr(quoted_ptr)
    };
    quoted_cstr.to_str().unwrap().to_string()
}

#[derive(Debug)]
pub struct UnknownVariant;

impl TryFrom<libc::c_int> for SpiOkCodes {
    // Yes, this gives us nested results.
    type Error = std::result::Result<SpiErrorCodes, UnknownVariant>;

    fn try_from(code: libc::c_int) -> std::result::Result<SpiOkCodes, Self::Error> {
        // Cast to assure that we're obeying repr rules even on platforms where c_ints are not 4 bytes wide,
        // as we don't support any but we may wish to in the future.
        match code as i32 {
            err @ -13..=-1 => Err(Ok(
                // SAFETY: These values are described in SpiError, thus they are inbounds for transmute
                unsafe { mem::transmute::<i32, SpiErrorCodes>(err) },
            )),
            ok @ 1..=18 => Ok(
                //SAFETY: These values are described in SpiOk, thus they are inbounds for transmute
                unsafe { mem::transmute::<i32, SpiOkCodes>(ok) },
            ),
            _unknown => Err(Err(UnknownVariant)),
        }
    }
}

impl TryFrom<libc::c_int> for SpiErrorCodes {
    // Yes, this gives us nested results.
    type Error = std::result::Result<SpiOkCodes, UnknownVariant>;

    fn try_from(code: libc::c_int) -> std::result::Result<SpiErrorCodes, Self::Error> {
        match SpiOkCodes::try_from(code) {
            Ok(ok) => Err(Ok(ok)),
            Err(Ok(err)) => Ok(err),
            Err(Err(unknown)) => Err(Err(unknown)),
        }
    }
}

/// Set of possible errors `pgx` might return while working with Postgres SPI
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    /// An underlying [`SpiErrorCodes`] given to us by Postgres
    #[error("SPI error: {0:?}")]
    SpiError(#[from] SpiErrorCodes),

    /// Some kind of problem understanding how to convert a Datum
    #[error("Datum error: {0}")]
    DatumError(#[from] TryFromDatumError),

    /// An incorrect number of arguments were supplied to a prepared statement
    #[error("Argument count mismatch (expected {expected}, got {got})")]
    PreparedStatementArgumentMismatch { expected: usize, got: usize },

    /// [`SpiTupleTable`] is positioned outside its bounds
    #[error("SpiTupleTable positioned before the start or after the end")]
    InvalidPosition,

    /// Postgres could not find the specified cursor by name
    #[error("Cursor named {0} not found")]
    CursorNotFound(String),

    /// The [`pg_sys::SPI_tuptable`] is null
    #[error("The active `SPI_tuptable` is NULL")]
    NoTupleTable,
}

pub struct Spi;

static MUTABLE_MODE: AtomicBool = AtomicBool::new(false);
impl Spi {
    #[inline]
    fn is_read_only() -> bool {
        MUTABLE_MODE.load(Ordering::Relaxed) == false
    }

    #[inline]
    fn clear_mutable() {
        MUTABLE_MODE.store(false, Ordering::Relaxed)
    }

    /// Postgres docs say:
    ///
    /// ```text
    ///    It is generally unwise to mix read-only and read-write commands within a single function
    ///    using SPI; that could result in very confusing behavior, since the read-only queries
    ///    would not see the results of any database updates done by the read-write queries.
    ///```
    ///
    /// We extend this to mean "within a single transaction".  We set the static `MUTABLE_MODE`
    /// here, and register callbacks for both transaction COMMIT and ABORT to clear it, if it's
    /// the first time in.  This way, once Spi has entered "mutable mode", it stays that way until
    /// the current transaction is finished.
    fn mark_mutable() {
        if Spi::is_read_only() {
            register_xact_callback(PgXactCallbackEvent::Commit, || Spi::clear_mutable());
            register_xact_callback(PgXactCallbackEvent::Abort, || Spi::clear_mutable());

            MUTABLE_MODE.store(true, Ordering::Relaxed)
        }
    }
}

// TODO: should `'conn` be invariant?
pub struct SpiClient<'conn> {
    __marker: PhantomData<&'conn SpiConnection>,
}

/// a struct to manage our SPI connection lifetime
struct SpiConnection(PhantomData<*mut ()>);

impl SpiConnection {
    /// Connect to Postgres' SPI system
    fn connect() -> Result<Self> {
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
    fn client(&self) -> SpiClient<'_> {
        SpiClient { __marker: PhantomData }
    }
}

/// A generalized interface to what constitutes a query
///
/// Its primary purpose is to abstract away differences between
/// one-off statements and prepared statements, but it can potentially
/// be implemented for other types, provided they can be converted into a query.
pub trait Query {
    type Arguments;
    type Result;

    /// Execute a query given a client and other arguments
    fn execute(
        self,
        client: &SpiClient,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> Self::Result;

    /// Open a cursor for the query
    fn open_cursor<'c: 'cc, 'cc>(
        self,
        client: &'cc SpiClient<'c>,
        args: Self::Arguments,
    ) -> SpiCursor<'c>;
}

impl<'a> Query for &'a String {
    type Arguments = Option<Vec<(PgOid, Option<pg_sys::Datum>)>>;
    type Result = Result<SpiTupleTable>;

    fn execute(
        self,
        client: &SpiClient,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        self.as_str().execute(client, limit, arguments)
    }

    fn open_cursor<'c: 'cc, 'cc>(
        self,
        client: &'cc SpiClient<'c>,
        args: Self::Arguments,
    ) -> SpiCursor<'c> {
        self.as_str().open_cursor(client, args)
    }
}

fn prepare_datum(datum: Option<pg_sys::Datum>) -> (pg_sys::Datum, std::os::raw::c_char) {
    match datum {
        Some(datum) => (datum, ' ' as std::os::raw::c_char),
        None => (pg_sys::Datum::from(0usize), 'n' as std::os::raw::c_char),
    }
}

impl<'a> Query for &'a str {
    type Arguments = Option<Vec<(PgOid, Option<pg_sys::Datum>)>>;
    type Result = Result<SpiTupleTable>;

    /// # Panics
    ///
    /// This function will panic if somehow the specified query contains a null byte.
    fn execute(
        self,
        _client: &SpiClient,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        // SAFETY: no concurrent access
        unsafe {
            pg_sys::SPI_tuptable = std::ptr::null_mut();
        }

        let src = CString::new(self).expect("query contained a null byte");
        let status_code = match arguments {
            Some(args) => {
                let nargs = args.len();
                let (types, data): (Vec<_>, Vec<_>) = args.into_iter().unzip();
                let mut argtypes = types.into_iter().map(PgOid::value).collect::<Vec<_>>();
                let (mut datums, nulls): (Vec<_>, Vec<_>) =
                    data.into_iter().map(prepare_datum).unzip();

                // SAFETY: arguments are prepared above
                unsafe {
                    pg_sys::SPI_execute_with_args(
                        src.as_ptr(),
                        nargs as i32,
                        argtypes.as_mut_ptr(),
                        datums.as_mut_ptr(),
                        nulls.as_ptr(),
                        Spi::is_read_only(),
                        limit.unwrap_or(0),
                    )
                }
            }
            // SAFETY: arguments are prepared above
            None => unsafe {
                pg_sys::SPI_execute(src.as_ptr(), Spi::is_read_only(), limit.unwrap_or(0))
            },
        };

        Ok(SpiClient::prepare_tuple_table(status_code)?)
    }

    fn open_cursor<'c: 'cc, 'cc>(
        self,
        _client: &'cc SpiClient<'c>,
        args: Self::Arguments,
    ) -> SpiCursor<'c> {
        let src = CString::new(self).expect("query contained a null byte");
        let args = args.unwrap_or_default();

        let nargs = args.len();
        let (types, data): (Vec<_>, Vec<_>) = args.into_iter().unzip();
        let mut argtypes = types.into_iter().map(PgOid::value).collect::<Vec<_>>();
        let (mut datums, nulls): (Vec<_>, Vec<_>) = data.into_iter().map(prepare_datum).unzip();

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
                Spi::is_read_only(),
                0,
            ))
        };
        SpiCursor { ptr, __marker: PhantomData }
    }
}

#[derive(Debug)]
pub struct SpiTupleTable {
    #[allow(dead_code)]
    status_code: SpiOkCodes,
    table: Option<*mut pg_sys::SPITupleTable>,
    size: usize,
    current: isize,
}

/// Represents a single `pg_sys::Datum` inside a `SpiHeapTupleData`
pub struct SpiHeapTupleDataEntry {
    datum: Option<pg_sys::Datum>,
    type_oid: pg_sys::Oid,
}

/// Represents the set of `pg_sys::Datum`s in a `pg_sys::HeapTuple`
pub struct SpiHeapTupleData {
    tupdesc: NonNull<pg_sys::TupleDescData>,
    // offset by 1!
    entries: Vec<SpiHeapTupleDataEntry>,
}

impl Spi {
    pub fn get_one<A: FromDatum + IntoDatum>(query: &str) -> Result<Option<A>> {
        Spi::connect(|mut client| client.update(query, Some(1), None)?.first().get_one())
    }

    pub fn get_two<A: FromDatum + IntoDatum, B: FromDatum + IntoDatum>(
        query: &str,
    ) -> Result<(Option<A>, Option<B>)> {
        Spi::connect(|mut client| client.update(query, Some(1), None)?.first().get_two::<A, B>())
    }

    pub fn get_three<
        A: FromDatum + IntoDatum,
        B: FromDatum + IntoDatum,
        C: FromDatum + IntoDatum,
    >(
        query: &str,
    ) -> Result<(Option<A>, Option<B>, Option<C>)> {
        Spi::connect(|mut client| {
            client.update(query, Some(1), None)?.first().get_three::<A, B, C>()
        })
    }

    pub fn get_one_with_args<A: FromDatum + IntoDatum>(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> Result<Option<A>> {
        Spi::connect(|mut client| client.update(query, Some(1), Some(args))?.first().get_one())
    }

    pub fn get_two_with_args<A: FromDatum + IntoDatum, B: FromDatum + IntoDatum>(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> Result<(Option<A>, Option<B>)> {
        Spi::connect(|mut client| {
            client.update(query, Some(1), Some(args))?.first().get_two::<A, B>()
        })
    }

    pub fn get_three_with_args<
        A: FromDatum + IntoDatum,
        B: FromDatum + IntoDatum,
        C: FromDatum + IntoDatum,
    >(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> Result<(Option<A>, Option<B>, Option<C>)> {
        Spi::connect(|mut client| {
            client.update(query, Some(1), Some(args))?.first().get_three::<A, B, C>()
        })
    }

    /// just run an arbitrary SQL statement.
    ///
    /// ## Safety
    ///
    /// The statement runs in read/write mode
    pub fn run(query: &str) -> std::result::Result<(), Error> {
        Spi::run_with_args(query, None)
    }

    /// run an arbitrary SQL statement with args.
    ///
    /// ## Safety
    ///
    /// The statement runs in read/write mode
    pub fn run_with_args(
        query: &str,
        args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>,
    ) -> std::result::Result<(), Error> {
        Spi::connect(|mut client| client.update(query, None, args)).map(|_| ())
    }

    /// explain a query, returning its result in json form
    pub fn explain(query: &str) -> Result<Json> {
        Spi::explain_with_args(query, None)
    }

    /// explain a query with args, returning its result in json form
    pub fn explain_with_args(
        query: &str,
        args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>,
    ) -> Result<Json> {
        Ok(Spi::connect(|mut client| {
            client
                .update(&format!("EXPLAIN (format json) {}", query), None, args)?
                .first()
                .get_one::<Json>()
        })?
        .unwrap())
    }

    /// Execute SPI commands via the provided `SpiClient`.
    ///
    /// While inside the provided closure, code executes under a short-lived "SPI Memory Context",
    /// and Postgres will completely free that context when this function is finished.
    ///
    /// pgx' SPI API endeavors to return Datum values from functions like `::get_one()` that are
    /// automatically copied into the into the `CurrentMemoryContext` at the time of this
    /// function call.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pgx::prelude::*;
    /// # fn foo() -> spi::Result<Option<String>> {
    /// let name = Spi::connect(|client| {
    ///     client.select("SELECT 'Bob'", None, None)?.first().get_one()
    /// })?;
    /// assert_eq!(name, Some("Bob"));
    /// # return Ok(name.map(str::to_string))
    /// # }
    /// ```
    ///
    /// Note that `SpiClient` is scoped to the connection lifetime and cannot be returned.  The
    /// following code will not compile:
    ///
    /// ```rust,compile_fail
    /// use pgx::prelude::*;
    /// let cant_return_client = Spi::connect(|client| client);
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if for some reason it's unable to "connect" to Postgres' SPI
    /// system.  At the time of this writing, that's actually impossible as the underlying function
    /// ([`pg_sys::SPI_connect()`]) **always** returns a successful response.
    pub fn connect<R, F: FnOnce(SpiClient<'_>) -> R>(f: F) -> R {
        // connect to SPI
        //
        // Postgres documents (https://www.postgresql.org/docs/current/spi-spi-connect.html) that
        // `pg_sys::SPI_connect()` can return `pg_sys::SPI_ERROR_CONNECT`, but in fact, if you
        // trace through the code back to (at least) pg11, it does not.  SPI_connect() always returns
        // `pg_sys::SPI_OK_CONNECT` (or it'll raise an error).
        //
        // So we make that an exceptional condition here and explicitly expect `SpiConnect::connect()`
        // to always succeed.
        //
        // The primary driver for this is not that we think we're smarter than Postgres, it's that
        // otherwise this function would need to return a `Result<R, spi::Error>` and that's a
        // fucking nightmare for users to deal with.  There's ample discussion around coming to
        // this decision at https://github.com/tcdi/pgx/pull/977
        let connection =
            SpiConnection::connect().expect("SPI_connect indicated an unexpected failure");

        // run the provided closure within the memory context that SPI_connect()
        // just put us un.  We'll disconnect from SPI when the closure is finished.
        // If there's a panic or elog(ERROR), we don't care about also disconnecting from
        // SPI b/c Postgres will do that for us automatically
        f(connection.client())
    }

    #[track_caller]
    pub fn check_status(status_code: i32) -> std::result::Result<SpiOkCodes, Error> {
        match SpiOkCodes::try_from(status_code) {
            Ok(ok) => Ok(ok),
            Err(Err(UnknownVariant)) => panic!("unrecognized SPI status code: {status_code}"),
            Err(Ok(code)) => Err(Error::SpiError(code)),
        }
    }
}

impl<'a> SpiClient<'a> {
    /// perform a SELECT statement
    pub fn select<Q: Query>(
        &self,
        query: Q,
        limit: Option<libc::c_long>,
        args: Q::Arguments,
    ) -> Q::Result {
        self.execute(query, limit, args)
    }

    /// perform any query (including utility statements) that modify the database in some way
    pub fn update<Q: Query>(
        &mut self,
        query: Q,
        limit: Option<libc::c_long>,
        args: Q::Arguments,
    ) -> Q::Result {
        Spi::mark_mutable();
        self.execute(query, limit, args)
    }

    fn execute<Q: Query>(
        &self,
        query: Q,
        limit: Option<libc::c_long>,
        args: Q::Arguments,
    ) -> Q::Result {
        query.execute(&self, limit, args)
    }

    fn prepare_tuple_table(status_code: i32) -> std::result::Result<SpiTupleTable, Error> {
        Ok(SpiTupleTable {
            status_code: Spi::check_status(status_code)?,
            // SAFETY: no concurrent access
            table: unsafe {
                if pg_sys::SPI_tuptable.is_null() {
                    None
                } else {
                    Some(pg_sys::SPI_tuptable)
                }
            },
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
    pub fn open_cursor<Q: Query>(&self, query: Q, args: Q::Arguments) -> SpiCursor {
        query.open_cursor(&self, args)
    }

    /// Set up a cursor that will execute the specified update (mutating) query
    ///
    /// Rows may be then fetched using [`SpiCursor::fetch`].
    ///
    /// See [`SpiCursor`] docs for usage details.
    pub fn open_cursor_mut<Q: Query>(&mut self, query: Q, args: Q::Arguments) -> SpiCursor {
        Spi::mark_mutable();
        query.open_cursor(&self, args)
    }

    /// Find a cursor in transaction by name
    ///
    /// A cursor for a query can be opened using [`SpiClient::open_cursor`].
    /// Cursor are automatically closed on drop unless [`SpiCursor::detach_into_name`] is used.
    /// Returned name can be used with this method to retrieve the open cursor.
    ///
    /// See [`SpiCursor`] docs for usage details.
    pub fn find_cursor(&self, name: &str) -> Result<SpiCursor> {
        use pgx_pg_sys::AsPgCStr;

        let ptr = NonNull::new(unsafe { pg_sys::SPI_cursor_find(name.as_pg_cstr()) })
            .ok_or(Error::CursorNotFound(name.to_string()))?;
        Ok(SpiCursor { ptr, __marker: PhantomData })
    }
}

type CursorName = String;

/// An SPI Cursor from a query
///
/// Represents a Postgres cursor (internally, a portal), allowing to retrieve result rows a few
/// at a time. Moreover, a cursor can be left open within a transaction, and accessed in
/// multiple independent Spi sessions within the transaction.
///
/// A cursor can be created via [`SpiClient::open_cursor()`] from a query.
/// Cursors are automatically closed on drop, unless explicitly left open using
/// [`Self::detach_into_name()`], which returns the cursor name; cursors left open can be retrieved
/// by name (in the same transaction) via [`SpiClient::find_cursor()`].
///
/// # Important notes about memory usage
/// Result sets ([`SpiTupleTable`]s) returned by [`SpiCursor::fetch()`] will not be freed until
/// the current Spi session is complete;
/// this is a Pgx limitation that might get lifted in the future.
///
/// In the meantime, if you're using cursors to limit memory usage, make sure to use
/// multiple separate Spi sessions, retrieving the cursor by name.
///
/// # Examples
/// ## Simple cursor
/// ```rust,no_run
/// use pgx::prelude::*;
/// # fn foo() -> spi::Result<()> {
/// Spi::connect(|mut client| {
///     let mut cursor = client.open_cursor("SELECT * FROM generate_series(1, 5)", None);
///     assert_eq!(Some(1u32), cursor.fetch(1)?.get_one::<u32>()?);
///     assert_eq!(Some(2u32), cursor.fetch(2)?.get_one::<u32>()?);
///     assert_eq!(Some(3u32), cursor.fetch(3)?.get_one::<u32>()?);
///     Ok::<_, pgx::spi::Error>(())
///     // <--- all three SpiTupleTable get freed by Spi::connect at this point
/// })
/// # }
/// ```
///
/// ## Cursor by name
/// ```rust,no_run
/// use pgx::prelude::*;
/// # fn foo() -> spi::Result<()> {
/// let cursor_name = Spi::connect(|mut client| {
///     let mut cursor = client.open_cursor("SELECT * FROM generate_series(1, 5)", None);
///     assert_eq!(Ok(Some(1u32)), cursor.fetch(1)?.get_one::<u32>());
///     Ok::<_, spi::Error>(cursor.detach_into_name()) // <-- cursor gets dropped here
///     // <--- first SpiTupleTable gets freed by Spi::connect at this point
/// })?;
/// Spi::connect(|mut client| {
///     let mut cursor = client.find_cursor(&cursor_name)?;
///     assert_eq!(Ok(Some(2u32)), cursor.fetch(1)?.get_one::<u32>());
///     drop(cursor); // <-- cursor gets dropped here
///     // ... more code ...
///     Ok(())
///     // <--- second SpiTupleTable gets freed by Spi::connect at this point
/// })
/// # }
/// ```
pub struct SpiCursor<'client> {
    ptr: NonNull<pg_sys::PortalData>,
    __marker: PhantomData<&'client SpiClient<'client>>,
}

impl SpiCursor<'_> {
    /// Fetch up to `count` rows from the cursor, moving forward
    ///
    /// If `fetch` runs off the end of the available rows, an empty [`SpiTupleTable`] is returned.
    pub fn fetch(&mut self, count: libc::c_long) -> std::result::Result<SpiTupleTable, Error> {
        // SAFETY: no concurrent access
        unsafe {
            pg_sys::SPI_tuptable = std::ptr::null_mut();
        }
        // SAFETY: SPI functions to create/find cursors fail via elog, so self.ptr is valid if we successfully set it
        unsafe { pg_sys::SPI_cursor_fetch(self.ptr.as_mut(), true, count) }
        Ok(SpiClient::prepare_tuple_table(SpiOkCodes::Fetch as i32)?)
    }

    /// Consume the cursor, returning its name
    ///
    /// The actual Postgres cursor is kept alive for the duration of the transaction.
    /// This allows to fetch it in a later SPI session within the same transaction
    /// using [`SpiClient::find_cursor()`]
    ///
    /// # Panics
    ///
    /// This function will panic if the cursor's name contains a null byte.
    pub fn detach_into_name(self) -> CursorName {
        // SAFETY: SPI functions to create/find cursors fail via elog, so self.ptr is valid if we successfully set it
        let cursor_ptr = unsafe { self.ptr.as_ref() };
        // Forget self, as to avoid closing the cursor in `drop`
        // No risk leaking rust memory, as Self is just a thin wrapper around a NonNull ptr
        std::mem::forget(self);
        // SAFETY: name is a null-terminated, valid string pointer from postgres
        unsafe { CStr::from_ptr(cursor_ptr.name) }
            .to_str()
            .expect("cursor name is not valid UTF8")
            .to_string()
    }
}

impl Drop for SpiCursor<'_> {
    fn drop(&mut self) {
        // SAFETY: SPI functions to create/find cursors fail via elog, so self.ptr is valid if we successfully set it
        unsafe {
            pg_sys::SPI_cursor_close(self.ptr.as_mut());
        }
    }
}

/// Client lifetime-bound prepared statement
pub struct PreparedStatement<'a> {
    plan: NonNull<pg_sys::_SPI_plan>,
    __marker: PhantomData<&'a ()>,
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

impl<'a> Query for &'a OwnedPreparedStatement {
    type Arguments = Option<Vec<Option<pg_sys::Datum>>>;
    type Result = Result<SpiTupleTable>;

    fn execute(
        self,
        client: &SpiClient,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        (&self.0).execute(client, limit, arguments)
    }

    fn open_cursor<'c: 'cc, 'cc>(
        self,
        client: &'cc SpiClient<'c>,
        args: Self::Arguments,
    ) -> SpiCursor<'c> {
        (&self.0).open_cursor(client, args)
    }
}

impl Query for OwnedPreparedStatement {
    type Arguments = Option<Vec<Option<pg_sys::Datum>>>;
    type Result = Result<SpiTupleTable>;

    fn execute(
        self,
        client: &SpiClient,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        (&self.0).execute(client, limit, arguments)
    }

    fn open_cursor<'c: 'cc, 'cc>(
        self,
        client: &'cc SpiClient<'c>,
        args: Self::Arguments,
    ) -> SpiCursor<'c> {
        (&self.0).open_cursor(client, args)
    }
}

impl<'a> PreparedStatement<'a> {
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

impl<'a: 'b, 'b> Query for &'b PreparedStatement<'a> {
    type Arguments = Option<Vec<Option<pg_sys::Datum>>>;
    type Result = Result<SpiTupleTable>;

    fn execute(
        self,
        _client: &SpiClient,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        // SAFETY: no concurrent access
        unsafe {
            pg_sys::SPI_tuptable = std::ptr::null_mut();
        }
        let args = arguments.unwrap_or_default();
        let nargs = args.len();

        let expected = unsafe { pg_sys::SPI_getargcount(self.plan.as_ptr()) } as usize;

        if nargs != expected {
            return Err(Error::PreparedStatementArgumentMismatch { expected, got: nargs });
        }

        let (mut datums, mut nulls): (Vec<_>, Vec<_>) = args.into_iter().map(prepare_datum).unzip();

        // SAFETY: all arguments are prepared above
        let status_code = unsafe {
            pg_sys::SPI_execute_plan(
                self.plan.as_ptr(),
                datums.as_mut_ptr(),
                nulls.as_mut_ptr(),
                Spi::is_read_only(),
                limit.unwrap_or(0),
            )
        };

        Ok(SpiClient::prepare_tuple_table(status_code)?)
    }

    fn open_cursor<'c: 'cc, 'cc>(
        self,
        _client: &'cc SpiClient<'c>,
        args: Self::Arguments,
    ) -> SpiCursor<'c> {
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
                Spi::is_read_only(),
            ))
        };
        SpiCursor { ptr, __marker: PhantomData }
    }
}

impl<'a> Query for PreparedStatement<'a> {
    type Arguments = Option<Vec<Option<pg_sys::Datum>>>;
    type Result = Result<SpiTupleTable>;

    fn execute(
        self,
        client: &SpiClient,
        limit: Option<libc::c_long>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        (&self).execute(client, limit, arguments)
    }

    fn open_cursor<'c: 'cc, 'cc>(
        self,
        client: &'cc SpiClient<'c>,
        args: Self::Arguments,
    ) -> SpiCursor<'c> {
        (&self).open_cursor(client, args)
    }
}

impl<'a> SpiClient<'a> {
    /// Prepares a statement that is valid for the lifetime of the client
    ///
    /// # Panics
    ///
    /// This function will panic if the supplied `query` string contained a NULL byte
    pub fn prepare(&self, query: &str, args: Option<Vec<PgOid>>) -> Result<PreparedStatement> {
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

impl SpiTupleTable {
    /// `SpiTupleTable`s are positioned before the start, for iteration purposes.
    ///
    /// This method moves the position to the first row.  If there are no rows, this
    /// method will silently return.
    pub fn first(mut self) -> Self {
        self.current = 0;
        self
    }

    /// Restore the state of iteration back to before the start.
    ///
    /// This is useful to iterate the table multiple times
    pub fn rewind(mut self) -> Self {
        self.current = -1;
        self
    }

    /// How many rows were processed?
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_one<A: FromDatum + IntoDatum>(&self) -> Result<Option<A>> {
        self.get(1)
    }

    pub fn get_two<A: FromDatum + IntoDatum, B: FromDatum + IntoDatum>(
        &self,
    ) -> Result<(Option<A>, Option<B>)> {
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
    ) -> Result<(Option<A>, Option<B>, Option<C>)> {
        let a = self.get::<A>(1)?;
        let b = self.get::<B>(2)?;
        let c = self.get::<C>(3)?;
        Ok((a, b, c))
    }

    #[inline(always)]
    fn get_spi_tuptable(&self) -> Result<(*mut pg_sys::SPITupleTable, *mut pg_sys::TupleDescData)> {
        let table = *self.table.as_ref().ok_or(Error::NoTupleTable)?;
        unsafe {
            // SAFETY:  we just assured that `table` is not null
            Ok((table, (*table).tupdesc))
        }
    }

    pub fn get_heap_tuple(&self) -> Result<Option<SpiHeapTupleData>> {
        if self.size == 0 || self.table.is_none() {
            // a query like "SELECT 1 LIMIT 0" is a valid "select"-style query that will not produce
            // a SPI_tuptable.  So are utility queries such as "CREATE INDEX" or "VACUUM".  We might
            // think that in the latter cases we'd want to produce an error here, but there's no
            // way to distinguish from the former.  As such, we take a gentle approach and
            // processed with "no, we don't have one, but it's okay"
            Ok(None)
        } else if self.current < 0 || self.current as usize >= self.size {
            Err(Error::InvalidPosition)
        } else {
            let (table, tupdesc) = self.get_spi_tuptable()?;
            unsafe {
                let heap_tuple =
                    std::slice::from_raw_parts((*table).vals, self.size)[self.current as usize];

                // SAFETY:  we know heap_tuple is valid because we just made it
                SpiHeapTupleData::new(tupdesc, heap_tuple)
            }
        }
    }

    /// Get a typed value by its ordinal position.
    ///
    /// The ordinal position is 1-based.
    ///
    /// # Errors
    ///
    /// If the specified ordinal is out of bounds a [`Error::SpiError(SpiError::NoAttribute)`] is returned
    /// If we have no backing tuple table a [`Error::NoTupleTable`] is returned
    ///
    /// # Panics
    ///
    /// This function will panic there is no parent MemoryContext.  This is an incredibly unlikely
    /// situation.
    pub fn get<T: IntoDatum + FromDatum>(&self, ordinal: usize) -> Result<Option<T>> {
        let (_, tupdesc) = self.get_spi_tuptable()?;
        let datum = self.get_datum_by_ordinal(ordinal)?;
        let is_null = datum.is_none();
        let datum = datum.unwrap_or_else(|| pg_sys::Datum::from(0));

        unsafe {
            // SAFETY:  we know the constraints around `datum` and `is_null` match because we
            // just got them from the underlying heap tuple
            Ok(T::try_from_datum_in_memory_context(
                PgMemoryContexts::CurrentMemoryContext
                    .parent()
                    .expect("parent memory context is absent"),
                datum,
                is_null,
                // SAFETY:  we know `self.tupdesc.is_some()` because an Ok return from
                // `self.get_datum_by_ordinal()` above already decided that for us
                pg_sys::SPI_gettypeid(tupdesc, ordinal as _),
            )?)
        }
    }

    /// Get a typed value by its name.
    ///
    /// # Errors
    ///
    /// If the specified name is invalid a [`Error::SpiError(SpiError::NoAttribute)`] is returned
    /// If we have no backing tuple table a [`Error::NoTupleTable`] is returned
    pub fn get_by_name<T: IntoDatum + FromDatum, S: AsRef<str>>(
        &self,
        name: S,
    ) -> Result<Option<T>> {
        self.get(self.column_ordinal(name)?)
    }

    /// Get a raw Datum from this HeapTuple by its ordinal position.
    ///
    /// The ordinal position is 1-based.
    ///
    /// # Errors
    ///
    /// If the specified ordinal is out of bounds a [`Error::SpiError(SpiError::NoAttribute)`] is returned
    /// If we have no backing tuple table a [`Error::NoTupleTable`] is returned
    pub fn get_datum_by_ordinal(&self, ordinal: usize) -> Result<Option<pg_sys::Datum>> {
        self.check_ordinal_bounds(ordinal)?;

        let (table, tupdesc) = self.get_spi_tuptable()?;
        if self.current < 0 || self.current as usize >= self.size {
            return Err(Error::InvalidPosition);
        }
        unsafe {
            let heap_tuple =
                std::slice::from_raw_parts((*table).vals, self.size)[self.current as usize];
            let mut is_null = false;
            let datum = pg_sys::SPI_getbinval(heap_tuple, tupdesc, ordinal as _, &mut is_null);

            if is_null {
                Ok(None)
            } else {
                Ok(Some(datum))
            }
        }
    }

    /// Get a raw Datum from this HeapTuple by its column name.
    ///
    /// # Errors
    ///
    /// If the specified name is invalid a [`Error::SpiError(SpiError::NoAttribute)`] is returned
    /// If we have no backing tuple table a [`Error::NoTupleTable`] is returned
    pub fn get_datum_by_name<S: AsRef<str>>(&self, name: S) -> Result<Option<pg_sys::Datum>> {
        self.get_datum_by_ordinal(self.column_ordinal(name)?)
    }

    /// Returns the number of columns
    pub fn columns(&self) -> Result<usize> {
        let (_, tupdesc) = self.get_spi_tuptable()?;
        // SAFETY:  we just got a valid tupdesc
        Ok(unsafe { (*tupdesc).natts as _ })
    }

    /// is the specified ordinal valid for the underlying tuple descriptor?
    #[inline]
    fn check_ordinal_bounds(&self, ordinal: usize) -> Result<()> {
        if ordinal < 1 || ordinal > self.columns()? {
            Err(Error::SpiError(SpiErrorCodes::NoAttribute))
        } else {
            Ok(())
        }
    }

    /// Returns column type OID
    ///
    /// The ordinal position is 1-based
    pub fn column_type_oid(&self, ordinal: usize) -> Result<PgOid> {
        self.check_ordinal_bounds(ordinal)?;

        let (_, tupdesc) = self.get_spi_tuptable()?;
        unsafe {
            // SAFETY:  we just got a valid tupdesc
            let oid = pg_sys::SPI_gettypeid(tupdesc, ordinal as i32);
            Ok(PgOid::from(oid))
        }
    }

    /// Returns column name of the 1-based `ordinal` position
    ///
    /// # Errors
    ///
    /// Returns [`Error::SpiError(SpiError::NoAttribute)`] if the specified ordinal value is out of bounds
    /// If we have no backing tuple table a [`Error::NoTupleTable`] is returned
    ///
    /// # Panics
    ///
    /// This function will panic if the column name at the specified ordinal position is not also
    /// a valid UTF8 string.
    pub fn column_name(&self, ordinal: usize) -> Result<String> {
        self.check_ordinal_bounds(ordinal)?;
        let (_, tupdesc) = self.get_spi_tuptable()?;
        unsafe {
            // SAFETY:  we just got a valid tupdesc and we know ordinal is in bounds
            let name = pg_sys::SPI_fname(tupdesc, ordinal as i32);

            // SAFETY:  SPI_fname will have given us a properly allocated char* since we know
            // the specified ordinal is in bounds
            let str =
                CStr::from_ptr(name).to_str().expect("column name is not value UTF8").to_string();

            // SAFETY: we just asked Postgres to allocate name for us
            pg_sys::pfree(name as *mut _);
            Ok(str)
        }
    }

    /// Returns the ordinal (1-based position) of the specified column name
    ///
    /// # Errors
    ///
    /// Returns [`Error::SpiError(SpiError::NoAttribute)`] if the specified column name isn't found
    /// If we have no backing tuple table a [`Error::NoTupleTable`] is returned
    ///
    /// # Panics
    ///
    /// This function will panic if somehow the specified name contains a null byte.
    pub fn column_ordinal<S: AsRef<str>>(&self, name: S) -> Result<usize> {
        let (_, tupdesc) = self.get_spi_tuptable()?;
        unsafe {
            let name_cstr = CString::new(name.as_ref()).expect("name contained a null byte");
            let fnumber = pg_sys::SPI_fnumber(tupdesc, name_cstr.as_ptr());

            if fnumber == pg_sys::SPI_ERROR_NOATTRIBUTE {
                Err(Error::SpiError(SpiErrorCodes::NoAttribute))
            } else {
                Ok(fnumber as usize)
            }
        }
    }
}

impl SpiHeapTupleData {
    /// Create a new `SpiHeapTupleData` from its constituent parts
    ///
    /// # Safety
    ///
    /// This is unsafe as it cannot ensure that the provided `tupdesc` and `htup` arguments
    /// are valid, palloc'd pointers.
    pub unsafe fn new(
        tupdesc: pg_sys::TupleDesc,
        htup: *mut pg_sys::HeapTupleData,
    ) -> Result<Option<Self>> {
        let tupdesc = NonNull::new(tupdesc).ok_or(Error::NoTupleTable)?;
        let mut data = SpiHeapTupleData { tupdesc, entries: Vec::new() };
        let tupdesc = tupdesc.as_ptr();

        unsafe {
            // SAFETY:  we know tupdesc is not null
            let natts = (*tupdesc).natts;
            data.entries.reserve(usize::try_from(natts as usize).unwrap_or_default());
            for i in 1..=natts {
                let mut is_null = false;
                let datum = pg_sys::SPI_getbinval(htup, tupdesc, i, &mut is_null);
                data.entries.push(SpiHeapTupleDataEntry {
                    datum: if is_null { None } else { Some(datum) },
                    type_oid: pg_sys::SPI_gettypeid(tupdesc, i),
                });
            }
        }

        Ok(Some(data))
    }

    /// Get a typed value from this HeapTuple by its ordinal position.
    ///
    /// The ordinal position is 1-based
    ///
    /// # Errors
    ///
    /// Returns a [`Error::DatumError`] if the desired Rust type is incompatible
    /// with the underlying Datum
    pub fn get<T: IntoDatum + FromDatum>(&self, ordinal: usize) -> Result<Option<T>> {
        self.get_datum_by_ordinal(ordinal).map(|entry| entry.value())?
    }

    /// Get a typed value from this HeapTuple by its name in the resultset.
    ///
    /// # Errors
    ///
    /// Returns a [`Error::DatumError`] if the desired Rust type is incompatible
    /// with the underlying Datum
    pub fn get_by_name<T: IntoDatum + FromDatum, S: AsRef<str>>(
        &self,
        name: S,
    ) -> Result<Option<T>> {
        self.get_datum_by_name(name.as_ref()).map(|entry| entry.value())?
    }

    /// Get a raw Datum from this HeapTuple by its ordinal position.
    ///
    /// The ordinal position is 1-based.
    ///
    /// # Errors
    ///
    /// If the specified ordinal is out of bounds a [`Error::SpiError(SpiError::NoAttribute)`] is returned
    pub fn get_datum_by_ordinal(
        &self,
        ordinal: usize,
    ) -> std::result::Result<&SpiHeapTupleDataEntry, Error> {
        // Wrapping because `self.entries.get(...)` will bounds check.
        let index = ordinal.wrapping_sub(1);
        self.entries.get(index).ok_or_else(|| Error::SpiError(SpiErrorCodes::NoAttribute))
    }

    /// Get a raw Datum from this HeapTuple by its field name.
    ///
    /// # Errors
    ///
    /// If the specified name isn't valid a [`Error::SpiError(SpiError::NoAttribute)`] is returned
    ///
    /// # Panics
    ///
    /// This function will panic if somehow the specified name contains a null byte.
    pub fn get_datum_by_name<S: AsRef<str>>(
        &self,
        name: S,
    ) -> std::result::Result<&SpiHeapTupleDataEntry, Error> {
        unsafe {
            let name_cstr = CString::new(name.as_ref()).expect("name contained a null byte");
            let fnumber = pg_sys::SPI_fnumber(self.tupdesc.as_ptr(), name_cstr.as_ptr());

            if fnumber == pg_sys::SPI_ERROR_NOATTRIBUTE {
                Err(Error::SpiError(SpiErrorCodes::NoAttribute))
            } else {
                self.get_datum_by_ordinal(fnumber as usize)
            }
        }
    }

    /// Set a datum value for the specified ordinal position
    ///
    /// # Errors
    ///
    /// If the specified ordinal is out of bounds a [`SpiErrorCodes::NoAttribute`] is returned
    pub fn set_by_ordinal<T: IntoDatum>(
        &mut self,
        ordinal: usize,
        datum: T,
    ) -> std::result::Result<(), Error> {
        self.check_ordinal_bounds(ordinal)?;
        self.entries[ordinal - 1] =
            SpiHeapTupleDataEntry { datum: datum.into_datum(), type_oid: T::type_oid() };
        Ok(())
    }

    /// Set a datum value for the specified field name
    ///
    /// # Errors
    ///
    /// If the specified name isn't valid a [`Error::SpiError(SpiError::NoAttribute)`] is returned
    ///
    /// # Panics
    ///
    /// This function will panic if somehow the specified name contains a null byte.
    pub fn set_by_name<T: IntoDatum>(
        &mut self,
        name: &str,
        datum: T,
    ) -> std::result::Result<(), Error> {
        unsafe {
            let name_cstr = CString::new(name).expect("name contained a null byte");
            let fnumber = pg_sys::SPI_fnumber(self.tupdesc.as_ptr(), name_cstr.as_ptr());
            if fnumber == pg_sys::SPI_ERROR_NOATTRIBUTE {
                Err(Error::SpiError(SpiErrorCodes::NoAttribute))
            } else {
                self.set_by_ordinal(fnumber as usize, datum)
            }
        }
    }

    #[inline]
    pub fn columns(&self) -> usize {
        unsafe {
            // SAFETY: we know self.tupdesc is a valid, non-null pointer because we own it
            (*self.tupdesc.as_ptr()).natts as usize
        }
    }

    /// is the specified ordinal valid for the underlying tuple descriptor?
    #[inline]
    fn check_ordinal_bounds(&self, ordinal: usize) -> std::result::Result<(), Error> {
        if ordinal < 1 || ordinal > self.columns() {
            Err(Error::SpiError(SpiErrorCodes::NoAttribute))
        } else {
            Ok(())
        }
    }
}

impl SpiHeapTupleDataEntry {
    pub fn value<T: IntoDatum + FromDatum>(&self) -> Result<Option<T>> {
        match self.datum.as_ref() {
            Some(datum) => unsafe {
                T::try_from_datum(*datum, false, self.type_oid).map_err(|e| Error::DatumError(e))
            },
            None => Ok(None),
        }
    }

    pub fn oid(&self) -> pg_sys::Oid {
        self.type_oid
    }
}

/// Provide ordinal indexing into a `SpiHeapTupleData`.
///
/// If the index is out of bounds, it will panic
impl Index<usize> for SpiHeapTupleData {
    type Output = SpiHeapTupleDataEntry;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_datum_by_ordinal(index).expect("invalid ordinal value")
    }
}

/// Provide named indexing into a `SpiHeapTupleData`.
///
/// If the field name doesn't exist, it will panic
impl Index<&str> for SpiHeapTupleData {
    type Output = SpiHeapTupleDataEntry;

    fn index(&self, index: &str) -> &Self::Output {
        self.get_datum_by_name(index).expect("invalid field name")
    }
}

impl Iterator for SpiTupleTable {
    type Item = SpiHeapTupleData;

    /// # Panics
    ///
    /// This method will panic if for some reason the underlying heap tuple cannot be retrieved
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= self.size as isize {
            None
        } else {
            assert!(self.current >= 0);
            self.get_heap_tuple().report()
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.size))
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.size
    }
}
