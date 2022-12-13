/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! Safe access to Postgres' *Server Programming Interface* (SPI).

use crate::{pg_sys, FromDatum, IntoDatum, Json, PgMemoryContexts, PgOid};
use std::collections::HashMap;
use std::ffi::CStr;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, Index, IndexMut};
use std::ptr::NonNull;

/// These match the Postgres `#define`d constants prefixed `SPI_OK_*` that you can find in `pg_sys`.
#[derive(Debug, PartialEq)]
#[repr(i32)]
#[non_exhaustive]
pub enum SpiOk {
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
#[derive(Debug, PartialEq)]
#[repr(i32)]
pub enum SpiError {
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

#[derive(Debug)]
pub struct UnknownVariant;

impl TryFrom<libc::c_int> for SpiOk {
    // Yes, this gives us nested results.
    type Error = Result<SpiError, UnknownVariant>;

    fn try_from(code: libc::c_int) -> Result<SpiOk, Result<SpiError, UnknownVariant>> {
        // Cast to assure that we're obeying repr rules even on platforms where c_ints are not 4 bytes wide,
        // as we don't support any but we may wish to in the future.
        match code as i32 {
            err @ -13..=-1 => Err(Ok(
                // SAFETY: These values are described in SpiError, thus they are inbounds for transmute
                unsafe { mem::transmute::<i32, SpiError>(err) },
            )),
            ok @ 1..=18 => Ok(
                //SAFETY: These values are described in SpiOk, thus they are inbounds for transmute
                unsafe { mem::transmute::<i32, SpiOk>(ok) },
            ),
            _unknown => Err(Err(UnknownVariant)),
        }
    }
}

impl TryFrom<libc::c_int> for SpiError {
    // Yes, this gives us nested results.
    type Error = Result<SpiOk, UnknownVariant>;

    fn try_from(code: libc::c_int) -> Result<SpiError, Result<SpiOk, UnknownVariant>> {
        match SpiOk::try_from(code) {
            Ok(ok) => Err(Ok(ok)),
            Err(Ok(err)) => Ok(err),
            Err(Err(unknown)) => Err(Err(unknown)),
        }
    }
}

pub struct Spi;

// TODO: should `'conn` be invariant?
pub struct SpiClient<'conn>(PhantomData<&'conn SpiConnection>);

/// a struct to manage our SPI connection lifetime
struct SpiConnection(PhantomData<*mut ()>);

impl SpiConnection {
    /// Connect to Postgres' SPI system
    fn connect() -> Self {
        // connect to SPI
        Spi::check_status(unsafe { pg_sys::SPI_connect() });
        SpiConnection(PhantomData)
    }
}

impl Drop for SpiConnection {
    /// when SpiConnection is dropped, we make sure to disconnect from SPI
    fn drop(&mut self) {
        // disconnect from SPI
        Spi::check_status(unsafe { pg_sys::SPI_finish() });
    }
}

impl SpiConnection {
    /// Return a client that with a lifetime scoped to this connection.
    fn client(&self) -> SpiClient<'_> {
        SpiClient(PhantomData)
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
        read_only: bool,
        limit: Option<i64>,
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
    type Result = SpiTupleTable;

    fn execute(
        self,
        client: &SpiClient,
        read_only: bool,
        limit: Option<i64>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        self.as_str().execute(client, read_only, limit, arguments)
    }

    fn open_cursor<'c: 'cc, 'cc>(
        self,
        client: &'cc SpiClient<'c>,
        args: Self::Arguments,
    ) -> SpiCursor<'c> {
        self.as_str().open_cursor(client, args)
    }
}

impl<'a> Query for &'a str {
    type Arguments = Option<Vec<(PgOid, Option<pg_sys::Datum>)>>;
    type Result = SpiTupleTable;

    fn execute(
        self,
        _client: &SpiClient,
        read_only: bool,
        limit: Option<i64>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        unsafe {
            pg_sys::SPI_tuptable = std::ptr::null_mut();
        }

        let src = std::ffi::CString::new(self).expect("query contained a null byte");
        let status_code = match arguments {
            Some(args) => {
                let nargs = args.len();
                let mut argtypes = vec![];
                let mut datums = vec![];
                let mut nulls = vec![];

                for (argtype, datum) in args {
                    argtypes.push(argtype.value());

                    match datum {
                        Some(datum) => {
                            // ' ' here means that the datum is not null
                            datums.push(datum);
                            nulls.push(' ' as std::os::raw::c_char);
                        }

                        None => {
                            // 'n' here means that the datum is null
                            datums.push(pg_sys::Datum::from(0usize));
                            nulls.push('n' as std::os::raw::c_char);
                        }
                    }
                }

                unsafe {
                    pg_sys::SPI_execute_with_args(
                        src.as_ptr(),
                        nargs as i32,
                        argtypes.as_mut_ptr(),
                        datums.as_mut_ptr(),
                        nulls.as_ptr(),
                        read_only,
                        limit.unwrap_or(0),
                    )
                }
            }
            None => unsafe { pg_sys::SPI_execute(src.as_ptr(), read_only, limit.unwrap_or(0)) },
        };

        SpiClient::prepare_tuple_table(status_code)
    }

    fn open_cursor<'c: 'cc, 'cc>(
        self,
        _client: &'cc SpiClient<'c>,
        args: Self::Arguments,
    ) -> SpiCursor<'c> {
        let src = std::ffi::CString::new(self).expect("query contained a null byte");
        let args = args.unwrap_or_default();

        let nargs = args.len();
        let mut argtypes = vec![];
        let mut datums = vec![];
        let mut nulls = vec![];

        for (argtype, datum) in args {
            argtypes.push(argtype.value());

            match datum {
                Some(datum) => {
                    // ' ' here means that the datum is not null
                    datums.push(datum);
                    nulls.push(' ' as std::os::raw::c_char);
                }

                None => {
                    // 'n' here means that the datum is null
                    datums.push(pg_sys::Datum::from(0usize));
                    nulls.push('n' as std::os::raw::c_char);
                }
            }
        }

        let ptr = NonNull::new(unsafe {
            pg_sys::SPI_cursor_open_with_args(
                std::ptr::null_mut(), // let postgres assign a name
                src.as_ptr(),
                nargs as i32,
                argtypes.as_mut_ptr(),
                datums.as_mut_ptr(),
                nulls.as_ptr(),
                false,
                0,
            )
        })
        .expect("Portal ptr was null");
        SpiCursor { ptr, _phantom: PhantomData }
    }
}

#[derive(Debug)]
pub struct SpiTupleTable {
    #[allow(dead_code)]
    status_code: SpiOk,
    table: *mut pg_sys::SPITupleTable,
    size: usize,
    tupdesc: Option<pg_sys::TupleDesc>,
    current: isize,
}

/// Represents a single `pg_sys::Datum` inside a `SpiHeapTupleData`
pub struct SpiHeapTupleDataEntry {
    datum: Option<pg_sys::Datum>,
    type_oid: pg_sys::Oid,
}

/// Represents the set of `pg_sys::Datum`s in a `pg_sys::HeapTuple`
pub struct SpiHeapTupleData {
    tupdesc: pg_sys::TupleDesc,
    entries: HashMap<usize, SpiHeapTupleDataEntry>,
}

impl Spi {
    pub fn get_one<A: FromDatum + IntoDatum>(query: &str) -> Option<A> {
        Spi::connect(|client| {
            let result = client.select(query, Some(1), None).first().get_one();
            Ok(result)
        })
    }

    pub fn get_two<A: FromDatum + IntoDatum, B: FromDatum + IntoDatum>(
        query: &str,
    ) -> (Option<A>, Option<B>) {
        Spi::connect(|client| {
            let (a, b) = client.select(query, Some(1), None).first().get_two::<A, B>();
            Ok(Some((a, b)))
        })
        .unwrap()
    }

    pub fn get_three<
        A: FromDatum + IntoDatum,
        B: FromDatum + IntoDatum,
        C: FromDatum + IntoDatum,
    >(
        query: &str,
    ) -> (Option<A>, Option<B>, Option<C>) {
        Spi::connect(|client| {
            let (a, b, c) = client.select(query, Some(1), None).first().get_three::<A, B, C>();
            Ok(Some((a, b, c)))
        })
        .unwrap()
    }

    pub fn get_one_with_args<A: FromDatum + IntoDatum>(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> Option<A> {
        Spi::connect(|client| Ok(client.select(query, Some(1), Some(args)).first().get_one()))
    }

    pub fn get_two_with_args<A: FromDatum + IntoDatum, B: FromDatum + IntoDatum>(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> (Option<A>, Option<B>) {
        Spi::connect(|client| {
            let (a, b) = client.select(query, Some(1), Some(args)).first().get_two::<A, B>();
            Ok(Some((a, b)))
        })
        .unwrap()
    }

    pub fn get_three_with_args<
        A: FromDatum + IntoDatum,
        B: FromDatum + IntoDatum,
        C: FromDatum + IntoDatum,
    >(
        query: &str,
        args: Vec<(PgOid, Option<pg_sys::Datum>)>,
    ) -> (Option<A>, Option<B>, Option<C>) {
        Spi::connect(|client| {
            let (a, b, c) =
                client.select(query, Some(1), Some(args)).first().get_three::<A, B, C>();
            Ok(Some((a, b, c)))
        })
        .unwrap()
    }

    /// just run an arbitrary SQL statement.
    ///
    /// ## Safety
    ///
    /// The statement runs in read/write mode
    pub fn run(query: &str) {
        Spi::run_with_args(query, None)
    }

    /// run an arbitrary SQL statement with args.
    ///
    /// ## Safety
    ///
    /// The statement runs in read/write mode
    pub fn run_with_args(query: &str, args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>) {
        Spi::execute(|client| {
            client.update(query, None, args);
        })
    }

    /// explain a query, returning its result in json form
    pub fn explain(query: &str) -> Json {
        Spi::explain_with_args(query, None)
    }

    /// explain a query with args, returning its result in json form
    pub fn explain_with_args(
        query: &str,
        args: Option<Vec<(PgOid, Option<pg_sys::Datum>)>>,
    ) -> Json {
        Spi::connect(|client| {
            let table =
                client.update(&format!("EXPLAIN (format json) {}", query), None, args).first();
            Ok(Some(table.get_one::<Json>().expect("failed to get json EXPLAIN result")))
        })
        .unwrap()
    }

    /// execute SPI commands via the provided `SpiClient`
    pub fn execute<F: FnOnce(SpiClient) + std::panic::UnwindSafe>(f: F) {
        Spi::connect(|client| {
            f(client);
            Ok(Some(()))
        });
    }

    /// execute SPI commands via the provided `SpiClient` and return a value from SPI which is
    /// automatically copied into the `CurrentMemoryContext` at the time of this function call
    ///
    /// Note that `SpiClient` is scoped to the connection lifetime and the following code will
    /// not compile:
    ///
    /// ```rust,compile_fail
    /// use pgx::*;
    /// Spi::connect(|client| Ok(Some(client)));
    /// ```
    pub fn connect<R, F: FnOnce(SpiClient<'_>) -> std::result::Result<Option<R>, SpiError>>(
        f: F,
    ) -> Option<R> {
        // connect to SPI
        let connection = SpiConnection::connect();

        // run the provided closure within the memory context that SPI_connect()
        // just put us un.  We'll disconnect from SPI when the closure is finished.
        // If there's a panic or elog(ERROR), we don't care about also disconnecting from
        // SPI b/c Postgres will do that for us automatically
        f(connection.client()).unwrap()
    }

    pub fn check_status(status_code: i32) -> SpiOk {
        match SpiOk::try_from(status_code) {
            Ok(ok) => ok,
            Err(Err(UnknownVariant)) => panic!("unrecognized SPI status code: {status_code}"),
            Err(Ok(code)) => panic!("{code:?}"),
        }
    }
}

impl<'a> SpiClient<'a> {
    /// perform a SELECT statement
    pub fn select<Q: Query>(&self, query: Q, limit: Option<i64>, args: Q::Arguments) -> Q::Result {
        // Postgres docs say:
        //
        //    It is generally unwise to mix read-only and read-write commands within a single function
        //    using SPI; that could result in very confusing behavior, since the read-only queries
        //    would not see the results of any database updates done by the read-write queries.
        //
        // As such, we don't actually set read-only to true here

        // TODO:  can we detect if the command counter (or something?) has incremented and if yes
        //        then we set read_only=false, else we can set it to true?
        //        Is this even a good idea?
        self.execute(query, false, limit, args)
    }

    /// perform any query (including utility statements) that modify the database in some way
    pub fn update<Q: Query>(&self, query: Q, limit: Option<i64>, args: Q::Arguments) -> Q::Result {
        self.execute(query, false, limit, args)
    }

    fn execute<Q: Query>(
        &self,
        query: Q,
        read_only: bool,
        limit: Option<i64>,
        args: Q::Arguments,
    ) -> Q::Result {
        query.execute(&self, read_only, limit, args)
    }

    fn prepare_tuple_table(status_code: i32) -> SpiTupleTable {
        SpiTupleTable {
            status_code: Spi::check_status(status_code),
            table: unsafe { pg_sys::SPI_tuptable },
            size: unsafe { pg_sys::SPI_processed as usize },
            tupdesc: if unsafe { pg_sys::SPI_tuptable }.is_null() {
                None
            } else {
                Some(unsafe { (*pg_sys::SPI_tuptable).tupdesc })
            },
            current: -1,
        }
    }

    /// Set up a cursor that will execute the specified query
    ///
    /// Rows may be then fetched using [`SpiCursor::fetch`].
    ///
    /// See [`SpiCursor`] docs for usage details.
    pub fn open_cursor<Q: Query>(&self, query: Q, args: Q::Arguments) -> SpiCursor<'a> {
        query.open_cursor(&self, args)
    }

    /// Find a cursor in transaction by name
    ///
    /// A cursor for a query can be opened using [`SpiClient::open_cursor`].
    /// Cursor are automatically closed on drop unless [`SpiCursor::detach_into_name`] is used.
    /// Returned name can be used with this method to retrieve the open cursor.
    ///
    /// See [`SpiCursor`] docs for usage details.
    pub fn find_cursor(&self, name: &str) -> SpiCursor {
        use pgx_pg_sys::AsPgCStr;

        let ptr = NonNull::new(unsafe { pg_sys::SPI_cursor_find(name.as_pg_cstr()) })
            .unwrap_or_else(|| panic!("cursor named \"{}\" not found", name));
        SpiCursor { ptr, _phantom: PhantomData }
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
/// use pgx::Spi;
/// Spi::connect(|mut client| {
///     let mut cursor = client.open_cursor("SELECT * FROM generate_series(1, 5)", None);
///     assert_eq!(Some(1u32), cursor.fetch(1).get_one());
///     assert_eq!(Some(2u32), cursor.fetch(2).get_one());
///     assert_eq!(Some(3u32), cursor.fetch(3).get_one());
///     Ok(None::<()>)
///     // <--- all three SpiTupleTable get freed by Spi::connect at this point
/// });
/// ```
///
/// ## Cursor by name
/// ```rust,no_run
/// use pgx::Spi;
/// let cursor_name = Spi::connect(|mut client| {
///     let mut cursor = client.open_cursor("SELECT * FROM generate_series(1, 5)", None);
///     assert_eq!(Some(1u32), cursor.fetch(1).get_one());
///     Ok(Some(cursor.detach_into_name())) // <-- cursor gets dropped here
///     // <--- first SpiTupleTable gets freed by Spi::connect at this point
/// }).unwrap();
/// Spi::connect(|mut client| {
///     let mut cursor = client.find_cursor(&cursor_name);
///     assert_eq!(Some(2u32), cursor.fetch(1).get_one());
///     drop(cursor); // <-- cursor gets dropped here
///     // ... more code ...
///     Ok(None::<()>)
///     // <--- second SpiTupleTable gets freed by Spi::connect at this point
/// });
/// ```
pub struct SpiCursor<'client> {
    ptr: NonNull<pg_sys::PortalData>,
    _phantom: PhantomData<&'client SpiClient<'client>>,
}

impl SpiCursor<'_> {
    /// Fetch up to `count` rows from the cursor, moving forward
    ///
    /// If `fetch` runs off the end of the available rows, an empty [`SpiTupleTable`] is returned.
    pub fn fetch(&mut self, count: i64) -> SpiTupleTable {
        unsafe {
            pg_sys::SPI_tuptable = std::ptr::null_mut();
        }
        // SAFETY: SPI functions to create/find cursors fail via elog, so self.ptr is valid if we successfully set it
        unsafe { pg_sys::SPI_cursor_fetch(self.ptr.as_mut(), true, count) }
        SpiTupleTable {
            status_code: SpiOk::Fetch,
            table: unsafe { pg_sys::SPI_tuptable },
            size: unsafe { pg_sys::SPI_processed as usize },
            tupdesc: if unsafe { pg_sys::SPI_tuptable }.is_null() {
                None
            } else {
                Some(unsafe { (*pg_sys::SPI_tuptable).tupdesc })
            },
            current: -1,
        }
    }

    /// Consume the cursor, returning its name
    ///
    /// The actual Postgres cursor is kept alive for the duration of the transaction.
    /// This allows to fetch it in a later SPI session within the same transaction
    /// using [`SpiClient::find_cursor()`]
    pub fn detach_into_name(self) -> CursorName {
        // SAFETY: SPI functions to create/find cursors fail via elog, so self.ptr is valid if we successfully set it
        let cursor_ptr = unsafe { self.ptr.as_ref() };
        // Forget self, as to avoid closing the cursor in `drop`
        // No risk leaking rust memory, as Self is just a thin wrapper around a NonNull ptr
        std::mem::forget(self);
        // SAFETY: name is a null-terminated, valid string pointer from postgres
        unsafe { std::ffi::CStr::from_ptr(cursor_ptr.name) }
            .to_str()
            .expect("non-utf8 cursor name")
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

/// Errors during prepared statements execution
#[derive(thiserror::Error, Debug)]
pub enum PreparedStatementError {
    #[error("argument count mismatch (expected {expected}, got {got})")]
    ArgumentCountMismatch { expected: usize, got: usize },
}

/// Client lifetime-bound prepared statement
pub struct PreparedStatement<'a> {
    phantom: PhantomData<&'a ()>,
    plan: pg_sys::SPIPlanPtr,
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
            pg_sys::SPI_freeplan(self.0.plan);
        }
    }
}

impl<'a> Query for &'a OwnedPreparedStatement {
    type Arguments = Option<Vec<Option<pg_sys::Datum>>>;
    type Result = Result<SpiTupleTable, PreparedStatementError>;

    fn execute(
        self,
        client: &SpiClient,
        read_only: bool,
        limit: Option<i64>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        (&self.0).execute(client, read_only, limit, arguments)
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
    type Result = Result<SpiTupleTable, PreparedStatementError>;

    fn execute(
        self,
        client: &SpiClient,
        read_only: bool,
        limit: Option<i64>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        (&self.0).execute(client, read_only, limit, arguments)
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
    /// Converts prepared statement into an owner prepared statement
    ///
    /// These statements have static lifetime and are freed only when dropped
    pub fn keep(&self) -> OwnedPreparedStatement {
        unsafe {
            pg_sys::SPI_keepplan(self.plan);
        }
        OwnedPreparedStatement(PreparedStatement { phantom: PhantomData, plan: self.plan })
    }
}

impl<'a: 'b, 'b> Query for &'b PreparedStatement<'a> {
    type Arguments = Option<Vec<Option<pg_sys::Datum>>>;
    type Result = Result<SpiTupleTable, PreparedStatementError>;

    fn execute(
        self,
        _client: &SpiClient,
        read_only: bool,
        limit: Option<i64>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        unsafe {
            pg_sys::SPI_tuptable = std::ptr::null_mut();
        }
        let args = arguments.unwrap_or_default();
        let mut datums = vec![];
        let mut nulls = vec![];
        let nargs = args.len();
        let expected = unsafe { pg_sys::SPI_getargcount(self.plan) } as usize;

        if nargs != expected {
            return Err(PreparedStatementError::ArgumentCountMismatch { expected, got: nargs });
        }

        for datum in args {
            match datum {
                Some(datum) => {
                    // ' ' here means that the datum is not null
                    datums.push(datum);
                    nulls.push(' ' as std::os::raw::c_char);
                }

                None => {
                    // 'n' here means that the datum is null
                    datums.push(pg_sys::Datum::from(0usize));
                    nulls.push('n' as std::os::raw::c_char);
                }
            }
        }

        let status_code = unsafe {
            pg_sys::SPI_execute_plan(
                self.plan,
                datums.as_mut_ptr(),
                nulls.as_mut_ptr(),
                read_only,
                limit.unwrap_or(0),
            )
        };

        Ok(SpiClient::prepare_tuple_table(status_code))
    }

    fn open_cursor<'c: 'cc, 'cc>(
        self,
        _client: &'cc SpiClient<'c>,
        args: Self::Arguments,
    ) -> SpiCursor<'c> {
        let args = args.unwrap_or_default();

        let mut datums = vec![];
        let mut nulls = vec![];

        for datum in args {
            match datum {
                Some(datum) => {
                    // ' ' here means that the datum is not null
                    datums.push(datum);
                    nulls.push(' ' as std::os::raw::c_char);
                }

                None => {
                    // 'n' here means that the datum is null
                    datums.push(pg_sys::Datum::from(0usize));
                    nulls.push('n' as std::os::raw::c_char);
                }
            }
        }

        let ptr = NonNull::new(unsafe {
            pg_sys::SPI_cursor_open(
                std::ptr::null_mut(), // let postgres assign a name
                self.plan,
                datums.as_mut_ptr(),
                nulls.as_ptr(),
                false,
            )
        })
        .expect("Portal ptr was null");
        SpiCursor { ptr, _phantom: PhantomData }
    }
}

impl<'a> Query for PreparedStatement<'a> {
    type Arguments = Option<Vec<Option<pg_sys::Datum>>>;
    type Result = Result<SpiTupleTable, PreparedStatementError>;

    fn execute(
        self,
        client: &SpiClient,
        read_only: bool,
        limit: Option<i64>,
        arguments: Self::Arguments,
    ) -> Self::Result {
        (&self).execute(client, read_only, limit, arguments)
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
    pub fn prepare(&self, query: &str, args: Option<Vec<PgOid>>) -> PreparedStatement {
        let src = std::ffi::CString::new(query).expect("query contained a null byte");
        let args = args.unwrap_or_default();
        let nargs = args.len();

        let plan = unsafe {
            pg_sys::SPI_prepare(
                src.as_ptr(),
                nargs as i32,
                args.into_iter().map(PgOid::value).collect::<Vec<_>>().as_mut_ptr(),
            )
        };
        PreparedStatement { phantom: PhantomData, plan }
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

    /// How many rows were processed?
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_one<A: FromDatum>(&self) -> Option<A> {
        self.get_datum(1)
    }

    pub fn get_two<A: FromDatum, B: FromDatum>(&self) -> (Option<A>, Option<B>) {
        let a = self.get_datum::<A>(1);
        let b = self.get_datum::<B>(2);
        (a, b)
    }

    pub fn get_three<A: FromDatum, B: FromDatum, C: FromDatum>(
        &self,
    ) -> (Option<A>, Option<B>, Option<C>) {
        let a = self.get_datum::<A>(1);
        let b = self.get_datum::<B>(2);
        let c = self.get_datum::<C>(3);
        (a, b, c)
    }

    pub fn get_heap_tuple(&self) -> Option<SpiHeapTupleData> {
        if self.current < 0 {
            panic!("SpiTupleTable positioned before start")
        }
        if self.current as usize >= self.size {
            None
        } else {
            match self.tupdesc {
                Some(tupdesc) => unsafe {
                    let heap_tuple = std::slice::from_raw_parts((*self.table).vals, self.size)
                        [self.current as usize];

                    // SAFETY:  we know heap_tuple is valid because we just made it
                    Some(SpiHeapTupleData::new(tupdesc, heap_tuple))
                },
                None => panic!("TupDesc is NULL"),
            }
        }
    }

    pub fn get_datum<T: FromDatum>(&self, ordinal: i32) -> Option<T> {
        if self.current < 0 {
            panic!("SpiTupleTable positioned before start")
        }
        if self.current as usize >= self.size {
            None
        } else {
            match self.tupdesc {
                Some(tupdesc) => unsafe {
                    let natts = (*tupdesc).natts;

                    if ordinal < 1 || ordinal > natts {
                        None
                    } else {
                        let heap_tuple = std::slice::from_raw_parts((*self.table).vals, self.size)
                            [self.current as usize];
                        let mut is_null = false;
                        let datum =
                            pg_sys::SPI_getbinval(heap_tuple, tupdesc, ordinal, &mut is_null);

                        T::from_datum_in_memory_context(
                            PgMemoryContexts::CurrentMemoryContext
                                .parent()
                                .expect("parent memory context is absent"),
                            datum,
                            is_null,
                            pg_sys::SPI_gettypeid(tupdesc, ordinal),
                        )
                    }
                },
                None => panic!("TupDesc is NULL"),
            }
        }
    }

    /// Returns the number of columns
    pub fn columns(&self) -> usize {
        match self.tupdesc {
            Some(tupdesc) => unsafe { (*tupdesc).natts as usize },
            None => 0,
        }
    }

    /// Returns column type OID
    ///
    /// The ordinal position is 1-based
    pub fn column_type_oid(&self, ordinal: usize) -> Option<PgOid> {
        match self.tupdesc {
            Some(tupdesc) => unsafe {
                let nattrs = (*tupdesc).natts;
                if ordinal < 1 || ordinal > (nattrs as usize) {
                    None
                } else {
                    let oid = pg_sys::SPI_gettypeid(tupdesc, ordinal as i32);
                    Some(PgOid::from(oid))
                }
            },
            None => None,
        }
    }

    /// Returns column name
    ///
    /// The ordinal position is 1-based
    pub fn column_name(&self, ordinal: usize) -> Option<String> {
        match self.tupdesc {
            Some(tupdesc) => unsafe {
                let nattrs = (*tupdesc).natts;
                if ordinal < 1 || ordinal > (nattrs as usize) {
                    None
                } else {
                    let name = pg_sys::SPI_fname(tupdesc, ordinal as i32);
                    let str = CStr::from_ptr(name).to_string_lossy().into_owned();
                    pg_sys::pfree(name as *mut _);
                    Some(str)
                }
            },
            None => None,
        }
    }
}

impl SpiHeapTupleData {
    /// Create a new `SpiHeapTupleData` from its constituent parts
    pub unsafe fn new(tupdesc: pg_sys::TupleDesc, htup: *mut pg_sys::HeapTupleData) -> Self {
        let mut data = SpiHeapTupleData { tupdesc, entries: HashMap::default() };

        for i in 1..=tupdesc.as_ref().unwrap().natts {
            let mut is_null = false;
            let datum = pg_sys::SPI_getbinval(htup, tupdesc, i, &mut is_null);

            data.entries.entry(i as usize).or_insert_with(|| SpiHeapTupleDataEntry {
                datum: if is_null { None } else { Some(datum) },
                type_oid: pg_sys::SPI_gettypeid(tupdesc, i),
            });
        }

        data
    }

    /// Get a typed Datum value from this HeapTuple by its ordinal position.  
    ///
    /// The ordinal position is 1-based
    #[deprecated(since = "0.1.6", note = "Please use the `by_ordinal` function instead")]
    pub fn get_datum<T: FromDatum>(&self, ordinal: usize) -> Option<T> {
        match self.entries.get(&ordinal) {
            Some(datum) => datum.value(),
            None => None,
        }
    }

    /// Get a typed Datum value from this HeapTuple by its ordinal position.  
    ///
    /// The ordinal position is 1-based.
    ///
    /// If the specified ordinal is out of bounds a `Err(SpiError::NoAttribute)` is returned
    pub fn by_ordinal(
        &self,
        ordinal: usize,
    ) -> std::result::Result<&SpiHeapTupleDataEntry, SpiError> {
        match self.entries.get(&ordinal) {
            Some(datum) => Ok(datum),
            None => Err(SpiError::NoAttribute),
        }
    }

    /// Get a typed Datum value from this HeapTuple by its field name.  
    ///
    /// If the specified name does not exist a `Err(SpiError::NoAttribute)` is returned
    pub fn by_name(&self, name: &str) -> std::result::Result<&SpiHeapTupleDataEntry, SpiError> {
        use crate::pg_sys::AsPgCStr;
        unsafe {
            let fnumber = pg_sys::SPI_fnumber(self.tupdesc, name.as_pg_cstr());
            if fnumber == pg_sys::SPI_ERROR_NOATTRIBUTE {
                Err(SpiError::NoAttribute)
            } else {
                self.by_ordinal(fnumber as usize)
            }
        }
    }

    /// Get a mutable typed Datum value from this HeapTuple by its ordinal position.  
    ///
    /// The ordinal position is 1-based.
    ///
    /// If the specified ordinal is out of bounds a `Err(SpiError::NoAttribute)` is returned
    pub fn by_ordinal_mut(
        &mut self,
        ordinal: usize,
    ) -> std::result::Result<&mut SpiHeapTupleDataEntry, SpiError> {
        match self.entries.get_mut(&ordinal) {
            Some(datum) => Ok(datum),
            None => Err(SpiError::NoAttribute),
        }
    }

    /// Get a mutable typed Datum value from this HeapTuple by its field name.  
    ///
    /// If the specified name does not exist a `Err(SpiError::NoAttribute)` is returned
    pub fn by_name_mut(
        &mut self,
        name: &str,
    ) -> std::result::Result<&mut SpiHeapTupleDataEntry, SpiError> {
        use crate::pg_sys::AsPgCStr;
        unsafe {
            let fnumber = pg_sys::SPI_fnumber(self.tupdesc, name.as_pg_cstr());
            if fnumber == pg_sys::SPI_ERROR_NOATTRIBUTE {
                Err(SpiError::NoAttribute)
            } else {
                self.by_ordinal_mut(fnumber as usize)
            }
        }
    }

    /// Set a datum value for the specified ordinal position
    ///
    /// If the specified ordinal is out of bounds a `Err(SpiError::NoAttribute)` is returned
    pub fn set_by_ordinal<T: IntoDatum + FromDatum>(
        &mut self,
        ordinal: usize,
        datum: T,
    ) -> std::result::Result<(), SpiError> {
        unsafe {
            if ordinal < 1 || ordinal > self.tupdesc.as_ref().unwrap().natts as usize {
                Err(SpiError::NoAttribute)
            } else {
                self.entries.insert(
                    ordinal,
                    SpiHeapTupleDataEntry { datum: datum.into_datum(), type_oid: T::type_oid() },
                );
                Ok(())
            }
        }
    }

    /// Set a datum value for the specified field name
    ///
    /// If the specified name does not exist a `Err(SpiError::NoAttribute)` is returned
    pub fn set_by_name<T: IntoDatum + FromDatum>(
        &mut self,
        name: &str,
        datum: T,
    ) -> std::result::Result<(), SpiError> {
        use crate::pg_sys::AsPgCStr;
        unsafe {
            let fnumber = pg_sys::SPI_fnumber(self.tupdesc, name.as_pg_cstr());
            if fnumber == pg_sys::SPI_ERROR_NOATTRIBUTE {
                Err(SpiError::NoAttribute)
            } else {
                self.set_by_ordinal(fnumber as usize, datum)
            }
        }
    }
}

impl<Datum: IntoDatum + FromDatum> From<Datum> for SpiHeapTupleDataEntry {
    fn from(datum: Datum) -> Self {
        SpiHeapTupleDataEntry { datum: datum.into_datum(), type_oid: Datum::type_oid() }
    }
}

impl SpiHeapTupleDataEntry {
    pub fn value<T: FromDatum>(&self) -> Option<T> {
        match self.datum.as_ref() {
            Some(datum) => unsafe { T::from_polymorphic_datum(*datum, false, self.type_oid) },
            None => None,
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
        self.by_ordinal(index).expect("invalid ordinal value")
    }
}

/// Provide named indexing into a `SpiHeapTupleData`.  
///
/// If the field name doesn't exist, it will panic
impl Index<&str> for SpiHeapTupleData {
    type Output = SpiHeapTupleDataEntry;

    fn index(&self, index: &str) -> &Self::Output {
        self.by_name(index).expect("invalid field name")
    }
}

/// Provide mutable ordinal indexing into a `SpiHeapTupleData`.  
///
/// If the index is out of bounds, it will panic
impl IndexMut<usize> for SpiHeapTupleData {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.by_ordinal_mut(index).expect("invalid ordinal value")
    }
}

/// Provide mutable named indexing into a `SpiHeapTupleData`.  
///
/// If the field name doesn't exist, it will panic
impl IndexMut<&str> for SpiHeapTupleData {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.by_name_mut(index).expect("invalid field name")
    }
}

impl Iterator for SpiTupleTable {
    type Item = SpiHeapTupleData;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.size as isize {
            self.current = -1;
            None
        } else {
            self.current += 1;
            assert!(self.current >= 0);
            self.get_heap_tuple()
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

    // Removed this function as it comes with an iterator
    //fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
}
