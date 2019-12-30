use crate::{pg_sys, rust_str_to_text_p, PgBox, PgDatum, PgMemoryContexts};

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
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}
impl DatumCompatible<i16> for i16 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<i16> {
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}
impl DatumCompatible<i32> for i32 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<i32> {
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}
impl DatumCompatible<i64> for i64 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<i64> {
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}

impl DatumCompatible<u8> for u8 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<u8> {
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}
impl DatumCompatible<u16> for u16 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<u16> {
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}
impl DatumCompatible<u32> for u32 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<u32> {
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}
impl DatumCompatible<u64> for u64 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<u64> {
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}

impl DatumCompatible<f32> for f32 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<f32> {
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}
impl DatumCompatible<f64> for f64 {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<f64> {
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}

impl DatumCompatible<bool> for bool {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<bool> {
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}
impl DatumCompatible<char> for char {
    fn copy_into(&self, _memory_context: &mut PgMemoryContexts) -> PgDatum<char> {
        PgDatum::new(*self as pg_sys::Datum, false)
    }
}

impl<'a> DatumCompatible<&'a str> for &'a str {
    fn copy_into(&self, memory_context: &mut PgMemoryContexts) -> PgDatum<&'a str> {
        memory_context.switch_to(|| {
            let copy = rust_str_to_text_p(self);
            PgDatum::new(copy as pg_sys::Datum, false)
        })
    }
}

impl<'a> DatumCompatible<&'a pg_sys::varlena> for &'a pg_sys::varlena {
    fn copy_into(&self, memory_context: &mut PgMemoryContexts) -> PgDatum<&'a pg_sys::varlena> {
        let size = unsafe { crate::varlena_size(*self as *const pg_sys::varlena) };

        PgDatum::new(
            memory_context.copy_ptr_into(*self as *const pg_sys::varlena as crate::void_ptr, size)
                as pg_sys::Datum,
            false,
        )
    }
}
