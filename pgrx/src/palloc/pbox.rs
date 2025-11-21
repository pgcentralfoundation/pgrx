use crate::callconv::{BoxRet, FcInfo};
use crate::datum::{BorrowDatum, Datum};
use crate::layout::PassBy;
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
        let datum = match T::PASS {
            PassBy::Value => {
                // start with a zeroed Datum, just to minimize funny business
                let mut datum = pg_sys::Datum::null();
                // SAFETY: we know this type is Sized and has a size less than the Datum, and
                // a PBox must have an initialized pointee, so a copy is sound-by-construction
                unsafe {
                    let size = size_of_val(&*self.ptr.as_ptr());
                    debug_assert!(size <= size_of::<pg_sys::Datum>());
                    // using `BorrowDatum::point_from` handles endianness
                    let datum_ptr = T::point_from(NonNull::from_mut(&mut datum).cast::<u8>());
                    datum_ptr.cast::<u8>().copy_from_nonoverlapping(self.ptr.cast(), size);
                }
                datum
            }
            PassBy::Ref => pg_sys::Datum::from(self.ptr.cast::<u8>().as_ptr()),
        };
        // SAFETY: by proxy, BorroWDatum is an `unsafe` trait so the above impl must be correct
        unsafe { fcinfo.return_raw_datum(datum) }
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
