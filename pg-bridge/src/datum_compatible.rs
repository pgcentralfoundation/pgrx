use crate::{pg_sys, PgBox, PgDatum, PgMemoryContexts};

/// Trait for structs that are part of Postgres' source code.
///
/// Can be implemented directly or as `#[derive(DatumCompatible)]`
pub trait DatumCompatible<T>: Sized
where
    T: DatumCompatible<T>,
{
    fn copy_into(&self, _memory_context: &mut super::PgMemoryContexts) -> PgDatum<T>;

    fn alloc() -> PgBox<T> {
        PgBox::<T>::alloc()
    }

    fn alloc0() -> PgBox<T> {
        PgBox::<T>::alloc0()
    }
}

impl DatumCompatible<()> for () {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<()> {
        PgDatum::null()
    }
}
impl DatumCompatible<pg_sys::Datum> for pg_sys::Datum {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<pg_sys::Datum> {
        PgDatum::new(*self, false)
    }
}

impl DatumCompatible<i8> for i8 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<i8> {
        PgDatum::new(self as *const _ as i8 as pg_sys::Datum, false)
    }
}
impl DatumCompatible<i16> for i16 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<i16> {
        PgDatum::new(self as *const _ as i16 as pg_sys::Datum, false)
    }
}
impl DatumCompatible<i32> for i32 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<i32> {
        PgDatum::new(self as *const _ as i32 as pg_sys::Datum, false)
    }
}
impl DatumCompatible<i64> for i64 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<i64> {
        PgDatum::new(self as *const _ as i64 as pg_sys::Datum, false)
    }
}

impl DatumCompatible<u8> for u8 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<u8> {
        PgDatum::new(self as *const _ as u8 as pg_sys::Datum, false)
    }
}
impl DatumCompatible<u16> for u16 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<u16> {
        PgDatum::new(self as *const _ as u16 as pg_sys::Datum, false)
    }
}
impl DatumCompatible<u32> for u32 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<u32> {
        PgDatum::new(self as *const _ as u32 as pg_sys::Datum, false)
    }
}
impl DatumCompatible<u64> for u64 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<u64> {
        PgDatum::new(self as *const _ as u64 as pg_sys::Datum, false)
    }
}

impl DatumCompatible<f32> for f32 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<f32> {
        PgDatum::new(
            f32::from_bits(self as *const _ as u32) as pg_sys::Datum,
            false,
        )
    }
}
impl DatumCompatible<f64> for f64 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<f64> {
        PgDatum::new(
            f64::from_bits(self as *const _ as u64) as pg_sys::Datum,
            false,
        )
    }
}

impl DatumCompatible<bool> for bool {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<bool> {
        PgDatum::new(((self as *const _ as u8) != 0) as pg_sys::Datum, false)
    }
}
impl DatumCompatible<char> for char {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<char> {
        PgDatum::new(self as *const _ as u8 as char as pg_sys::Datum, false)
    }
}

impl<'a> DatumCompatible<&'a str> for &'a str {
    fn copy_into(&self, memory_context: &mut PgMemoryContexts) -> PgDatum<&'a str> {
        let ptr = self as *const _ as *const pg_sys::varlena;
        let size = unsafe { crate::varlena_size(ptr) };

        PgDatum::new(
            memory_context.copy_ptr_into(ptr as crate::void_ptr, size) as pg_sys::Datum,
            false,
        )
    }
}

impl<'a> DatumCompatible<&'a pg_sys::varlena> for &'a pg_sys::varlena {
    fn copy_into(&self, memory_context: &mut PgMemoryContexts) -> PgDatum<&'a pg_sys::varlena> {
        let ptr = self as *const _ as *const pg_sys::varlena;
        let size = unsafe { crate::varlena_size(ptr) };

        PgDatum::new(
            memory_context.copy_ptr_into(ptr as crate::void_ptr, size) as pg_sys::Datum,
            false,
        )
    }
}
