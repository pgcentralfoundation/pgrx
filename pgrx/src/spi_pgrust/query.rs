//! pgrust: `Query`/`PreparableQuery` over pgrust's SPI crate.

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ops::Deref;

use super::{Spi, SpiClient, SpiCursor, SpiError, SpiResult, SpiTupleTable};
use crate::datum::DatumWithOid;
use crate::pg_sys::pgrust::spi_native as spi;
use crate::pg_sys::pgrust::{NativeDatum, unwrap_pg};
use crate::pg_sys::{self, PgOid};

pub trait Query<'conn>: Sized {
    fn execute<'mcx>(
        self,
        client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiTupleTable<'conn>>;

    #[deprecated(since = "0.12.2", note = "undefined behavior")]
    fn open_cursor<'mcx>(
        self,
        client: &SpiClient<'conn>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiCursor<'conn> {
        self.try_open_cursor(client, args).unwrap()
    }

    fn try_open_cursor<'mcx>(
        self,
        client: &SpiClient<'conn>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiCursor<'conn>>;
}

pub trait PreparableQuery<'conn>: Query<'conn> {
    fn prepare(
        self,
        client: &SpiClient<'conn>,
        args: &[PgOid],
    ) -> SpiResult<PreparedStatement<'conn>>;
    fn prepare_mut(
        self,
        client: &SpiClient<'conn>,
        args: &[PgOid],
    ) -> SpiResult<PreparedStatement<'conn>>;
}

fn args_to_native<'mcx>(args: &[DatumWithOid<'mcx>]) -> (Vec<u32>, Vec<NativeDatum>, Vec<bool>) {
    let mut argtypes = Vec::with_capacity(args.len());
    let mut values = Vec::with_capacity(args.len());
    let mut nulls = Vec::with_capacity(args.len());
    for arg in args {
        argtypes.push(arg.oid().to_u32());
        match arg.datum() {
            Some(d) => {
                values.push(NativeDatum::from_usize(d.sans_lifetime().value()));
                nulls.push(false);
            }
            None => {
                values.push(NativeDatum::null());
                nulls.push(true);
            }
        }
    }
    (argtypes, values, nulls)
}

fn execute<'conn, 'mcx>(
    cmd: &CStr,
    args: &[DatumWithOid<'mcx>],
    limit: Option<libc::c_long>,
) -> SpiResult<SpiTupleTable<'conn>> {
    let src = cmd.to_str().expect("query is not valid UTF-8");
    let read_only = Spi::is_xact_still_immutable();
    let tcount = limit.unwrap_or(0) as i64;
    let status = if args.is_empty() {
        unwrap_pg(spi::SPI_execute(src, read_only, tcount))
    } else {
        // prepare + execute_plan honours the row limit, which the
        // one-shot with-args entry point does not take.
        let (argtypes, values, nulls) = args_to_native(args);
        let plan = unwrap_pg(spi::SPI_prepare(src, &argtypes));
        let r = spi::SPI_execute_plan(plan, &values, &nulls, read_only, tcount);
        spi::SPI_freeplan(plan);
        unwrap_pg(r)
    };
    SpiClient::prepare_tuple_table(status)
}

fn open_cursor<'conn, 'mcx>(
    cmd: &CStr,
    args: &[DatumWithOid<'mcx>],
) -> SpiResult<SpiCursor<'conn>> {
    let src = cmd.to_str().expect("query is not valid UTF-8");
    let (argtypes, values, nulls) = args_to_native(args);
    let inner = unwrap_pg(spi::SPI_cursor_open_extended(
        None,
        src,
        &argtypes,
        &values,
        &nulls,
        Spi::is_xact_still_immutable(),
        0,
    ));
    Ok(SpiCursor {
        inner: Some(inner),
        name: String::new(),
        __marker: PhantomData,
    })
}

fn prepare<'conn>(
    cmd: &CStr,
    args: &[PgOid],
    mutating: bool,
) -> SpiResult<PreparedStatement<'conn>> {
    let src = cmd.to_str().expect("query is not valid UTF-8");
    let argtypes: Vec<u32> = args.iter().map(|a| a.value().to_u32()).collect();
    let plan = unwrap_pg(spi::SPI_prepare(src, &argtypes));
    if plan.is_null() {
        return Err(Spi::check_status(spi::SPI_result())
            .err()
            .unwrap_or(SpiError::NoTupleTable));
    }
    Ok(PreparedStatement {
        plan,
        __marker: PhantomData,
        mutating,
    })
}

macro_rules! impl_prepared_query {
    ($t:ident, $s:ident) => {
        impl<'conn> Query<'conn> for &$t {
            #[inline]
            fn execute(
                self,
                _client: &SpiClient<'conn>,
                limit: Option<libc::c_long>,
                args: &[DatumWithOid],
            ) -> SpiResult<SpiTupleTable<'conn>> {
                execute($s(self).as_ref(), args, limit)
            }

            #[inline]
            fn try_open_cursor(
                self,
                _client: &SpiClient<'conn>,
                args: &[DatumWithOid],
            ) -> SpiResult<SpiCursor<'conn>> {
                open_cursor($s(self).as_ref(), args)
            }
        }

        impl<'conn> PreparableQuery<'conn> for &$t {
            fn prepare(
                self,
                _client: &SpiClient<'conn>,
                args: &[PgOid],
            ) -> SpiResult<PreparedStatement<'conn>> {
                prepare($s(self).as_ref(), args, false)
            }

            fn prepare_mut(
                self,
                _client: &SpiClient<'conn>,
                args: &[PgOid],
            ) -> SpiResult<PreparedStatement<'conn>> {
                prepare($s(self).as_ref(), args, true)
            }
        }
    };
}

#[inline]
fn pass_as_is<T>(s: T) -> T {
    s
}

#[inline]
fn pass_with_conv<T: AsRef<str>>(s: T) -> CString {
    CString::new(s.as_ref()).expect("query contained a null byte")
}

impl_prepared_query!(CStr, pass_as_is);
impl_prepared_query!(CString, pass_as_is);
impl_prepared_query!(String, pass_with_conv);
impl_prepared_query!(str, pass_with_conv);

pub struct PreparedStatement<'conn> {
    pub(super) plan: spi::SpiPlanPtr,
    pub(super) __marker: PhantomData<&'conn ()>,
    pub(super) mutating: bool,
}

pub struct OwnedPreparedStatement(PreparedStatement<'static>);

impl Deref for OwnedPreparedStatement {
    type Target = PreparedStatement<'static>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for OwnedPreparedStatement {
    fn drop(&mut self) {
        spi::SPI_freeplan(self.0.plan);
    }
}

impl<'conn> Query<'conn> for &OwnedPreparedStatement {
    fn execute<'mcx>(
        self,
        client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiTupleTable<'conn>> {
        (&self.0).execute(client, limit, args)
    }

    fn try_open_cursor<'mcx>(
        self,
        client: &SpiClient<'conn>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiCursor<'conn>> {
        (&self.0).try_open_cursor(client, args)
    }
}

impl<'conn> Query<'conn> for OwnedPreparedStatement {
    fn execute<'mcx>(
        self,
        client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiTupleTable<'conn>> {
        (&self.0).execute(client, limit, args)
    }

    fn try_open_cursor<'mcx>(
        self,
        client: &SpiClient<'conn>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiCursor<'conn>> {
        (&self.0).try_open_cursor(client, args)
    }
}

impl<'conn> PreparedStatement<'conn> {
    pub fn keep(self) -> OwnedPreparedStatement {
        spi::SPI_keepplan(self.plan);
        OwnedPreparedStatement(PreparedStatement {
            __marker: PhantomData,
            plan: self.plan,
            mutating: self.mutating,
        })
    }

    fn args_to_datums<'mcx>(
        &self,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<(Vec<NativeDatum>, Vec<bool>)> {
        let actual = args.len();
        let expected = spi::SPI_getargcount(self.plan) as usize;
        if expected == actual {
            let (_, values, nulls) = args_to_native(args);
            Ok((values, nulls))
        } else {
            Err(SpiError::PreparedStatementArgumentMismatch {
                expected,
                got: actual,
            })
        }
    }
}

impl<'conn: 'stmt, 'stmt> Query<'conn> for &'stmt PreparedStatement<'conn> {
    fn execute<'mcx>(
        self,
        _client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiTupleTable<'conn>> {
        let (values, nulls) = self.args_to_datums(args)?;
        let status = unwrap_pg(spi::SPI_execute_plan(
            self.plan,
            &values,
            &nulls,
            !self.mutating && Spi::is_xact_still_immutable(),
            limit.unwrap_or(0) as i64,
        ));
        SpiClient::prepare_tuple_table(status)
    }

    fn try_open_cursor<'mcx>(
        self,
        _client: &SpiClient<'conn>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiCursor<'conn>> {
        let (values, nulls) = self.args_to_datums(args)?;
        let inner = unwrap_pg(spi::SPI_cursor_open(
            None,
            self.plan,
            &values,
            &nulls,
            !self.mutating && Spi::is_xact_still_immutable(),
        ));
        Ok(SpiCursor {
            inner: Some(inner),
            name: String::new(),
            __marker: PhantomData,
        })
    }
}

impl<'conn> Query<'conn> for PreparedStatement<'conn> {
    fn execute<'mcx>(
        self,
        client: &SpiClient<'conn>,
        limit: Option<libc::c_long>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiTupleTable<'conn>> {
        (&self).execute(client, limit, args)
    }

    fn try_open_cursor<'mcx>(
        self,
        client: &SpiClient<'conn>,
        args: &[DatumWithOid<'mcx>],
    ) -> SpiResult<SpiCursor<'conn>> {
        (&self).try_open_cursor(client, args)
    }
}

// keep `pg_sys` in scope for the PgOid re-export used in signatures
#[allow(unused_imports)]
use pg_sys as _;
