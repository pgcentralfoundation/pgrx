//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Safe access to Postgres' *Server Programming Interface* (SPI).

use crate::datum::{DatumWithOid, FromDatum, IntoDatum, Json, TryFromDatumError};
use crate::pg_sys;
use core::fmt::Formatter;
use std::ffi::{CStr, CString};
use std::fmt::Debug;

mod client;
mod cursor;
mod query;
mod tuple;
pub use client::SpiClient;
pub use cursor::SpiCursor;
pub use query::{OwnedPreparedStatement, PreparedStatement, Query};
pub use tuple::{SpiHeapTupleData, SpiHeapTupleDataEntry, SpiTupleTable};

pub type SpiResult<T> = std::result::Result<T, SpiError>;
pub use SpiResult as Result;

/// These match the Postgres `#define`d constants prefixed `SPI_OK_*` that you can find in `pg_sys`.
#[derive(Debug, PartialEq)]
#[repr(i32)]
#[non_exhaustive]
pub enum SpiOkCodes {
    Connect = pg_sys::SPI_OK_CONNECT as i32,
    Finish = pg_sys::SPI_OK_FINISH as i32,
    Fetch = pg_sys::SPI_OK_FETCH as i32,
    Utility = pg_sys::SPI_OK_UTILITY as i32,
    Select = pg_sys::SPI_OK_SELECT as i32,
    SelInto = pg_sys::SPI_OK_SELINTO as i32,
    Insert = pg_sys::SPI_OK_INSERT as i32,
    Delete = pg_sys::SPI_OK_DELETE as i32,
    Update = pg_sys::SPI_OK_UPDATE as i32,
    Cursor = pg_sys::SPI_OK_CURSOR as i32,
    InsertReturning = pg_sys::SPI_OK_INSERT_RETURNING as i32,
    DeleteReturning = pg_sys::SPI_OK_DELETE_RETURNING as i32,
    UpdateReturning = pg_sys::SPI_OK_UPDATE_RETURNING as i32,
    Rewritten = pg_sys::SPI_OK_REWRITTEN as i32,
    RelRegister = pg_sys::SPI_OK_REL_REGISTER as i32,
    RelUnregister = pg_sys::SPI_OK_REL_UNREGISTER as i32,
    TdRegister = pg_sys::SPI_OK_TD_REGISTER as i32,
    /// Added in Postgres 15
    Merge = pg_sys::SPI_OK_MERGE as i32,
}

/// These match the Postgres `#define`d constants prefixed `SPI_ERROR_*` that you can find in `pg_sys`.
/// It is hypothetically possible for a Postgres-defined status code to be `0`, AKA `NULL`, however,
/// this should not usually occur in Rust code paths. If it does happen, please report such bugs to the pgrx repo.
#[derive(thiserror::Error, Debug, PartialEq)]
#[repr(i32)]
pub enum SpiErrorCodes {
    Connect = pg_sys::SPI_ERROR_CONNECT as i32,
    Copy = pg_sys::SPI_ERROR_COPY as i32,
    OpUnknown = pg_sys::SPI_ERROR_OPUNKNOWN as i32,
    Unconnected = pg_sys::SPI_ERROR_UNCONNECTED as i32,
    #[allow(dead_code)]
    Cursor = pg_sys::SPI_ERROR_CURSOR as i32, /* not used anymore */
    Argument = pg_sys::SPI_ERROR_ARGUMENT as i32,
    Param = pg_sys::SPI_ERROR_PARAM as i32,
    Transaction = pg_sys::SPI_ERROR_TRANSACTION as i32,
    NoAttribute = pg_sys::SPI_ERROR_NOATTRIBUTE as i32,
    NoOutFunc = pg_sys::SPI_ERROR_NOOUTFUNC as i32,
    TypUnknown = pg_sys::SPI_ERROR_TYPUNKNOWN as i32,
    RelDuplicate = pg_sys::SPI_ERROR_REL_DUPLICATE as i32,
    RelNotFound = pg_sys::SPI_ERROR_REL_NOT_FOUND as i32,
}

impl std::fmt::Display for SpiErrorCodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
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
/// `"my schema"."my table"`.
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
        // Match declared variants rather than assuming the generated constants
        // form contiguous ranges suitable for unchecked enum conversion.
        match code {
            pg_sys::SPI_ERROR_CONNECT => Err(Ok(SpiErrorCodes::Connect)),
            pg_sys::SPI_ERROR_COPY => Err(Ok(SpiErrorCodes::Copy)),
            pg_sys::SPI_ERROR_OPUNKNOWN => Err(Ok(SpiErrorCodes::OpUnknown)),
            pg_sys::SPI_ERROR_UNCONNECTED => Err(Ok(SpiErrorCodes::Unconnected)),
            pg_sys::SPI_ERROR_CURSOR => Err(Ok(SpiErrorCodes::Cursor)),
            pg_sys::SPI_ERROR_ARGUMENT => Err(Ok(SpiErrorCodes::Argument)),
            pg_sys::SPI_ERROR_PARAM => Err(Ok(SpiErrorCodes::Param)),
            pg_sys::SPI_ERROR_TRANSACTION => Err(Ok(SpiErrorCodes::Transaction)),
            pg_sys::SPI_ERROR_NOATTRIBUTE => Err(Ok(SpiErrorCodes::NoAttribute)),
            pg_sys::SPI_ERROR_NOOUTFUNC => Err(Ok(SpiErrorCodes::NoOutFunc)),
            pg_sys::SPI_ERROR_TYPUNKNOWN => Err(Ok(SpiErrorCodes::TypUnknown)),
            pg_sys::SPI_ERROR_REL_DUPLICATE => Err(Ok(SpiErrorCodes::RelDuplicate)),
            pg_sys::SPI_ERROR_REL_NOT_FOUND => Err(Ok(SpiErrorCodes::RelNotFound)),
            value if value == pg_sys::SPI_OK_CONNECT as i32 => Ok(Self::Connect),
            value if value == pg_sys::SPI_OK_FINISH as i32 => Ok(Self::Finish),
            value if value == pg_sys::SPI_OK_FETCH as i32 => Ok(Self::Fetch),
            value if value == pg_sys::SPI_OK_UTILITY as i32 => Ok(Self::Utility),
            value if value == pg_sys::SPI_OK_SELECT as i32 => Ok(Self::Select),
            value if value == pg_sys::SPI_OK_SELINTO as i32 => Ok(Self::SelInto),
            value if value == pg_sys::SPI_OK_INSERT as i32 => Ok(Self::Insert),
            value if value == pg_sys::SPI_OK_DELETE as i32 => Ok(Self::Delete),
            value if value == pg_sys::SPI_OK_UPDATE as i32 => Ok(Self::Update),
            value if value == pg_sys::SPI_OK_CURSOR as i32 => Ok(Self::Cursor),
            value if value == pg_sys::SPI_OK_INSERT_RETURNING as i32 => Ok(Self::InsertReturning),
            value if value == pg_sys::SPI_OK_DELETE_RETURNING as i32 => Ok(Self::DeleteReturning),
            value if value == pg_sys::SPI_OK_UPDATE_RETURNING as i32 => Ok(Self::UpdateReturning),
            value if value == pg_sys::SPI_OK_REWRITTEN as i32 => Ok(Self::Rewritten),
            value if value == pg_sys::SPI_OK_REL_REGISTER as i32 => Ok(Self::RelRegister),
            value if value == pg_sys::SPI_OK_REL_UNREGISTER as i32 => Ok(Self::RelUnregister),
            value if value == pg_sys::SPI_OK_TD_REGISTER as i32 => Ok(Self::TdRegister),
            value if value == pg_sys::SPI_OK_MERGE as i32 => Ok(Self::Merge),
            _ => Err(Err(UnknownVariant)),
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

/// Set of possible errors `pgrx` might return while working with Postgres SPI.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum SpiError {
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

pub type Error = SpiError;

pub struct Spi;

impl Spi {
    /// Determines if the current transaction can still be `read_only = true` for purposes of SPI
    /// queries.  This is detected in such a way that prior mutable commands within this transaction
    /// (even those not executed via pgrx's Spi) will influence whether or not we can consider the
    /// transaction `read_only = true`.  This is what we want as the user will expect an otherwise
    /// read-only statement like SELECT to see the results of prior statements.
    ///
    /// Postgres documentation says:
    ///
    /// > It is generally unwise to mix read-only and read-write commands within a single function
    /// > using SPI; that could result in very confusing behavior, since the read-only queries
    /// > would not see the results of any database updates done by the read-write queries.
    ///
    /// PGRX interprets this to mean that within a transaction, it's fine to execute Spi commands
    /// as `read_only = true` until the first mutable statement (DDL or DML).  From that point
    /// forward **all** statements must be executed as `read_only = false`.
    fn is_xact_still_immutable() -> bool {
        unsafe {
            // SAFETY:  `pg_sys::GetCurrentTransactionIdIfAny()` will always return a valid
            // TransactionId value, even if it's `InvalidTransactionId`.
            let current_xid = pg_sys::GetCurrentTransactionIdIfAny();

            // no assigned TransactionId means no mutation has occurred in this transaction
            current_xid == pg_sys::TransactionId::INVALID
        }
    }

    /// Let Postgres know that we intend to perform some kind of mutating operation in this transaction.
    ///
    /// From this point forward, within the current transaction, [`Spi::is_xact_still_immutable()`] will
    /// return `false`.
    fn mark_mutable() {
        unsafe {
            // SAFETY:  `pg_sys::GetCurrentTransactionId()` will return a valid, possibly newly-created
            // TransactionId or it'll raise an ERROR trying.

            // The act of marking this transaction mutable is simply asking Postgres for the current
            // TransactionId in a way where it will assign one if necessary
            let _ = pg_sys::GetCurrentTransactionId();
        }
    }

    pub fn get_one<A: FromDatum + IntoDatum>(query: &str) -> Result<Option<A>> {
        Spi::connect_mut(|client| client.update(query, Some(1), &[])?.first().get_one())
    }

    pub fn get_two<A: FromDatum + IntoDatum, B: FromDatum + IntoDatum>(
        query: &str,
    ) -> Result<(Option<A>, Option<B>)> {
        Spi::connect_mut(|client| client.update(query, Some(1), &[])?.first().get_two::<A, B>())
    }

    pub fn get_three<
        A: FromDatum + IntoDatum,
        B: FromDatum + IntoDatum,
        C: FromDatum + IntoDatum,
    >(
        query: &str,
    ) -> Result<(Option<A>, Option<B>, Option<C>)> {
        Spi::connect_mut(|client| {
            client.update(query, Some(1), &[])?.first().get_three::<A, B, C>()
        })
    }

    pub fn get_one_with_args<'mcx, A: FromDatum + IntoDatum>(
        query: &str,
        args: &[DatumWithOid<'mcx>],
    ) -> Result<Option<A>> {
        Spi::connect_mut(|client| client.update(query, Some(1), args)?.first().get_one())
    }

    pub fn get_two_with_args<'mcx, A: FromDatum + IntoDatum, B: FromDatum + IntoDatum>(
        query: &str,
        args: &[DatumWithOid<'mcx>],
    ) -> Result<(Option<A>, Option<B>)> {
        Spi::connect_mut(|client| client.update(query, Some(1), args)?.first().get_two::<A, B>())
    }

    pub fn get_three_with_args<
        'mcx,
        A: FromDatum + IntoDatum,
        B: FromDatum + IntoDatum,
        C: FromDatum + IntoDatum,
    >(
        query: &str,
        args: &[DatumWithOid<'mcx>],
    ) -> Result<(Option<A>, Option<B>, Option<C>)> {
        Spi::connect_mut(|client| {
            client.update(query, Some(1), args)?.first().get_three::<A, B, C>()
        })
    }

    /// Just run an arbitrary SQL statement.
    ///
    /// ## Safety
    ///
    /// The statement runs in read/write mode.
    pub fn run(query: &str) -> std::result::Result<(), Error> {
        Spi::run_with_args(query, &[])
    }

    /// Run an arbitrary SQL statement with args.
    ///
    /// ## Safety
    ///
    /// The statement runs in read/write mode.
    pub fn run_with_args<'mcx>(
        query: &str,
        args: &[DatumWithOid<'mcx>],
    ) -> std::result::Result<(), Error> {
        Spi::connect_mut(|client| client.update(query, None, args).map(|_| ()))
    }

    /// Explain a query, returning its result in JSON form.
    pub fn explain(query: &str) -> Result<Json> {
        Spi::explain_with_args(query, &[])
    }

    /// Explain a query with args, returning its result in JSON form.
    pub fn explain_with_args<'mcx>(query: &str, args: &[DatumWithOid<'mcx>]) -> Result<Json> {
        Ok(Spi::connect_mut(|client| {
            client
                .update(&format!("EXPLAIN (format json) {query}"), None, args)?
                .first()
                .get_one::<Json>()
        })?
        .unwrap())
    }

    /// Execute SPI read-only commands via the provided `SpiClient`.
    ///
    /// While inside the provided closure, code executes under a short-lived "SPI Memory Context",
    /// and Postgres will completely free that context when this function is finished.
    ///
    /// pgrx' SPI API endeavors to return Datum values from functions like `::get_one()` that are
    /// automatically copied into the into the `CurrentMemoryContext` at the time of this
    /// function call.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pgrx::prelude::*;
    /// # fn foo() -> spi::Result<Option<String>> {
    /// let name = Spi::connect(|client| {
    ///     client.select("SELECT 'Bob'", None, &[])?.first().get_one()
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
    /// use pgrx::prelude::*;
    /// let cant_return_client = Spi::connect(|client| client);
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if for some reason it's unable to "connect" to Postgres' SPI
    /// system.  At the time of this writing, that's actually impossible as the underlying function
    /// ([`pg_sys::SPI_connect()`]) **always** returns a successful response.
    pub fn connect<R, F>(f: F) -> R
    where
        F: FnOnce(&SpiClient<'_>) -> R,
    {
        Self::connect_mut(|client| f(client))
    }

    /// Execute SPI mutating commands via the provided `SpiClient`.
    ///
    /// While inside the provided closure, code executes under a short-lived "SPI Memory Context",
    /// and Postgres will completely free that context when this function is finished.
    ///
    /// pgrx' SPI API endeavors to return Datum values from functions like `::get_one()` that are
    /// automatically copied into the into the `CurrentMemoryContext` at the time of this
    /// function call.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pgrx::prelude::*;
    /// # fn foo() -> spi::Result<()> {
    /// Spi::connect_mut(|client| {
    ///     client.update("INSERT INTO users VALUES ('Bob')", None, &[])?;
    ///    Ok(())
    /// })
    /// # }
    /// ```
    ///
    /// Note that `SpiClient` is scoped to the connection lifetime and cannot be returned.  The
    /// following code will not compile:
    ///
    /// ```rust,compile_fail
    /// use pgrx::prelude::*;
    /// let cant_return_client = Spi::connect(|client| client);
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if for some reason it's unable to "connect" to Postgres' SPI
    /// system.  At the time of this writing, that's actually impossible as the underlying function
    /// ([`pg_sys::SPI_connect()`]) **always** returns a successful response.
    pub fn connect_mut<R, F>(f: F) -> R
    where
        F: FnOnce(&mut SpiClient<'_>) -> R, /* TODO: redesign this with 2 lifetimes:
                                            - 'conn ~= CurrentMemoryContext after connection
                                            - 'ret ~= SPI_palloc's context
                                            */
    {
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
        // this decision at https://github.com/pgcentralfoundation/pgrx/pull/977
        let mut client = SpiClient::connect().expect("SPI_connect indicated an unexpected failure");

        // run the provided closure within the memory context that SPI_connect()
        // just put us un.  We'll disconnect from SPI when the closure is finished.
        // If there's a panic or elog(ERROR), we don't care about also disconnecting from
        // SPI b/c Postgres will do that for us automatically
        f(&mut client)
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
