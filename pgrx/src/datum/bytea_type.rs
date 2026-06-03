#![deny(unsafe_op_in_unsafe_fn)]
use crate::callconv::{Arg, ArgAbi, BoxRet, FcInfo};
use crate::datum::Datum;
use crate::nullable::Nullable;
use crate::pg_sys;
use crate::varlena::varlena_to_byte_slice;
use core::marker::PhantomData;
use core::ops::Deref;
use std::ptr::NonNull;

use pgrx_sql_entity_graph::metadata::{
    ArgumentError, ReturnsError, ReturnsRef, SqlMappingRef, SqlTranslatable,
};

/// A lifetime-safe reference to detoasted bytea data.
///
/// Wraps a detoasted `pg_sys::varlena` pointer and provides zero-copy access to the underlying bytes via `Deref<Target = [u8]>`. The lifetime parameter ties it to the function call context.
pub struct Bytea<'fcx> {
    ptr: NonNull<pg_sys::varlena>,
    _lifetime: PhantomData<&'fcx ()>,
}

impl<'fcx> Bytea<'fcx> {
    /// Returns the underlying varlena pointer.
    pub fn as_varlena_ptr(&self) -> *const pg_sys::varlena {
        self.ptr.as_ptr()
    }

    /// Returns the byte content as a slice.
    pub fn as_bytes(&self) -> &[u8] {
        self
    }

    /// Returns the length of the bytea data (excluding varlena header).
    pub fn len(&self) -> usize {
        (**self).len()
    }

    /// Returns true if the bytea data is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Deref for Bytea<'_> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe { varlena_to_byte_slice(self.ptr.as_ptr()) }
    }
}

impl AsRef<[u8]> for Bytea<'_> {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

unsafe impl<'fcx> ArgAbi<'fcx> for Bytea<'fcx> {
    unsafe fn unbox_arg_unchecked(arg: Arg<'_, 'fcx>) -> Self {
        unsafe {
            let datum: pg_sys::Datum = arg
                .unbox_arg_using_from_datum::<pg_sys::Datum>()
                .expect("bytea argument must not be null");
            let varlena = pg_sys::pg_detoast_datum_packed(datum.cast_mut_ptr());
            let ptr = NonNull::new(varlena).expect("pg_detoast_datum_packed returned null");
            Bytea { ptr, _lifetime: PhantomData }
        }
    }

    unsafe fn unbox_nullable_arg(arg: Arg<'_, 'fcx>) -> Nullable<Self> {
        if arg.is_null() {
            Nullable::Null
        } else {
            Nullable::Valid(unsafe { Self::unbox_arg_unchecked(arg) })
        }
    }
}

unsafe impl BoxRet for Bytea<'_> {
    unsafe fn box_into<'fcx>(self, fcinfo: &mut FcInfo<'fcx>) -> Datum<'fcx> {
        unsafe { fcinfo.return_raw_datum(pg_sys::Datum::from(self.ptr.as_ptr())) }
    }
}

unsafe impl SqlTranslatable for Bytea<'_> {
    const TYPE_IDENT: &'static str = "bytea";
    const TYPE_ORIGIN: pgrx_sql_entity_graph::metadata::TypeOrigin =
        pgrx_sql_entity_graph::metadata::TypeOrigin::External;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Ok(SqlMappingRef::literal("bytea"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("bytea")));
}
