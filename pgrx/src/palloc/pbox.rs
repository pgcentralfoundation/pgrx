use crate::callconv::{BoxRet, FcInfo};
use crate::datum::{BorrowDatum, Datum};
use crate::memcx::MemCx;
use crate::pg_sys;
use core::marker::PhantomData;
use core::ptr::NonNull;

use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

/** As [`Box<T, A>`][stdbox] where `A` is a [`MemCx`]


[stdbox]: alloc::boxed::Box
*/
pub struct PBox<'mcx, T: ?Sized> {
    ptr: NonNull<T>,
    _cx: PhantomData<MemCx<'mcx>>,
}

impl<'mcx, T: ?Sized> PBox<'mcx, T> {
    pub unsafe fn from_raw_in(ptr: NonNull<T>, _cx: &MemCx<'mcx>) -> PBox<'mcx, T> {
        PBox { ptr, _cx: PhantomData }
    }
}

impl<'mcx, T: Sized> PBox<'mcx, T> {
    pub fn new_in(val: T, memcx: &MemCx<'mcx>) -> Self {
        const { assert!(align_of::<T>() <= 8) };
        let ptr = memcx.alloc_bytes(size_of::<T>()).cast();
        // We were guaranteed an appropriately sized allocation to write to,
        // and we have asserted our alignment maximum was upheld
        unsafe { ptr.write(val) };
        PBox { ptr, _cx: PhantomData }
    }
}

unsafe impl<'mcx, T> BoxRet for PBox<'mcx, T>
where
    T: ?Sized + BorrowDatum,
{
    unsafe fn box_into<'fcx>(self, fcinfo: &mut FcInfo<'fcx>) -> Datum<'fcx> {
        // SAFETY: by proxy
        unsafe { fcinfo.return_raw_datum(pg_sys::Datum::from(self.ptr.cast::<u8>().as_ptr())) }
    }
}

unsafe impl<'mcx, T> SqlTranslatable for PBox<'mcx, T>
where
    T: SqlTranslatable + ?Sized,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        T::argument_sql()
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        T::return_sql()
    }
}
