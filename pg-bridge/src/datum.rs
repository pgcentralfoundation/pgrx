use crate::{
    pg_sys, rust_str_to_text_p, text_to_rust_str_unchecked, DatumCompatible, PgBox,
    PgMemoryContexts,
};
use std::convert::{Infallible, TryFrom};
use std::ffi::CStr;
use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct PgDatum<T>(Option<pg_sys::Datum>, PhantomData<T>)
where
    T: DatumCompatible<T>;

impl<T> PgDatum<T>
where
    T: DatumCompatible<T>,
{
    /// Wrap an outside `pg_sys::Datum` value, along with noting if it's NULL or not
    #[inline]
    pub fn new(datum: pg_sys::Datum, is_null: bool) -> Self {
        PgDatum(if is_null { None } else { Some(datum) }, PhantomData)
    }

    /// Instantiate a typed NULL Datum
    #[inline]
    pub fn null() -> Self {
        PgDatum(None, PhantomData)
    }

    /// Is this datum NULL?
    #[inline]
    pub fn is_null(&self) -> bool {
        self.0.is_none()
    }

    /// Copy the backing bytes of the Datum to a different `MemoryContext`
    ///
    /// The implementation relies on the `DatumCompatible<T>` trait implementations
    /// to do the right thing for the type the Datum represents.
    ///
    /// Copying bytes isn't strictly necessary for pass-by-value Datum types
    #[inline]
    pub fn copy_into(&self, memory_context: &mut PgMemoryContexts) -> PgDatum<T> {
        match self.0 {
            Some(t) => {
                let t = t as *const T;
                unsafe { t.as_ref() }.unwrap().copy_into(memory_context)
            }
            None => PgDatum::null(),
        }
    }

    /// - `Some(datum)` means a valid Datum value
    /// - `None` means a NULL Datum value (which is distinct from `Some(0)`
    #[inline]
    pub fn as_pg_datum(&self) -> Option<pg_sys::Datum> {
        self.0
    }

    /// Return the underlying [pg_sys::Datum], even if it represents a NULL.  In that
    /// case the return value is simply `0`
    #[inline]
    pub fn as_raw_pg_datum(&self) -> pg_sys::Datum {
        match self.0 {
            Some(datum) => datum,
            None => 0,
        }
    }

    #[inline]
    pub fn into_datum(self) -> pg_sys::Datum {
        match self.0 {
            Some(datum) => datum,
            None => 0,
        }
    }
}

impl<T> Into<pg_sys::Datum> for PgDatum<T>
where
    T: DatumCompatible<T>,
{
    #[inline]
    fn into(self) -> pg_sys::Datum {
        match self.0 {
            Some(datum) => datum as pg_sys::Datum,
            None => 0 as pg_sys::Datum,
        }
    }
}

impl From<pg_sys::Datum> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(datum: pg_sys::Datum) -> Self {
        PgDatum::new(datum, false)
    }
}

impl<T> From<PgBox<T>> for PgDatum<PgBox<T>>
where
    T: crate::datum_compatible::DatumCompatible<T>,
{
    #[inline]
    fn from(val: PgBox<T>) -> Self {
        let is_null = val.is_null();
        PgDatum::new(val.into_datum(), is_null)
    }
}

impl<'a> From<&'a std::ffi::CStr> for PgDatum<&'a std::ffi::CStr> {
    fn from(val: &'a CStr) -> Self {
        PgDatum::new(val.as_ptr() as pg_sys::Datum, false)
    }
}

impl<'a> From<&'a str> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: &'a str) -> Self {
        PgDatum::new(rust_str_to_text_p(val) as pg_sys::Datum, false)
    }
}

impl From<&String> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: &String) -> Self {
        PgDatum::new(rust_str_to_text_p(val.as_str()) as pg_sys::Datum, false)
    }
}

impl From<i8> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: i8) -> Self {
        PgDatum::new(val as pg_sys::Datum, false)
    }
}
impl From<i16> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: i16) -> Self {
        PgDatum::new(val as pg_sys::Datum, false)
    }
}
impl From<i32> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: i32) -> Self {
        PgDatum::new(val as pg_sys::Datum, false)
    }
}
impl From<i64> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: i64) -> Self {
        PgDatum::new(val as pg_sys::Datum, false)
    }
}
impl From<u8> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: u8) -> Self {
        PgDatum::new(val as pg_sys::Datum, false)
    }
}
impl From<u16> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: u16) -> Self {
        PgDatum::new(val as pg_sys::Datum, false)
    }
}
impl From<u32> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: u32) -> Self {
        PgDatum::new(val as pg_sys::Datum, false)
    }
}
impl From<u64> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: u64) -> Self {
        PgDatum::new(val as pg_sys::Datum, false)
    }
}

impl From<f32> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: f32) -> Self {
        PgDatum::new(f32::to_bits(val) as pg_sys::Datum, false)
    }
}
impl From<f64> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: f64) -> Self {
        PgDatum::new(f64::to_bits(val) as pg_sys::Datum, false)
    }
}
impl From<bool> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: bool) -> Self {
        PgDatum::new(val as u8 as pg_sys::Datum, false)
    }
}
impl From<char> for PgDatum<pg_sys::Datum> {
    #[inline]
    fn from(val: char) -> Self {
        PgDatum::new(val as pg_sys::Datum, false)
    }
}

impl<T> From<*mut T> for PgDatum<T>
where
    T: DatumCompatible<T>,
{
    #[inline]
    fn from(val: *mut T) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl<T> From<PgBox<T>> for PgDatum<T>
where
    T: DatumCompatible<T> + Debug,
{
    #[inline]
    fn from(val: PgBox<T>) -> Self {
        PgDatum::<T>::from(val.into_pg())
    }
}

//
// From trait implementations for primitive types
//

impl From<bool> for PgDatum<bool> {
    #[inline]
    fn from(val: bool) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<char> for PgDatum<char> {
    #[inline]
    fn from(val: char) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<i8> for PgDatum<i8> {
    #[inline]
    fn from(val: i8) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<i16> for PgDatum<i16> {
    #[inline]
    fn from(val: i16) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<i32> for PgDatum<i32> {
    #[inline]
    fn from(val: i32) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<i64> for PgDatum<i64> {
    #[inline]
    fn from(val: i64) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<u8> for PgDatum<u8> {
    #[inline]
    fn from(val: u8) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<u16> for PgDatum<u16> {
    #[inline]
    fn from(val: u16) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<u32> for PgDatum<u32> {
    #[inline]
    fn from(val: u32) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<u64> for PgDatum<u64> {
    #[inline]
    fn from(val: u64) -> Self {
        PgDatum(Some(val as pg_sys::Datum), PhantomData)
    }
}

impl From<f32> for PgDatum<f32> {
    #[inline]
    fn from(val: f32) -> Self {
        PgDatum(Some(f32::to_bits(val) as pg_sys::Datum), PhantomData)
    }
}

impl From<f64> for PgDatum<f64> {
    #[inline]
    fn from(val: f64) -> Self {
        PgDatum(Some(f64::to_bits(val) as pg_sys::Datum), PhantomData)
    }
}

/// Rust [&str]s are represented as Postgres-allocated `varlena` inside a PgDatum, allocated
/// within `CurrentMemoryContext`
impl<'a> From<&'a str> for PgDatum<&'a str> {
    #[inline]
    fn from(val: &str) -> Self {
        PgDatum(Some(rust_str_to_text_p(val) as pg_sys::Datum), PhantomData)
    }
}

/// Rust [String]s are represented as Postgres-allocated `varlena` inside a PgDatum, allocated
/// within `CurrentMemoryContext`
impl From<String> for PgDatum<String> {
    #[inline]
    fn from(val: String) -> Self {
        PgDatum(
            Some(rust_str_to_text_p(val.as_str()) as pg_sys::Datum),
            PhantomData,
        )
    }
}

//
// TryFrom trait implementations for primitive types
//

impl TryFrom<PgDatum<i8>> for i8 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<i8>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i8),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<i16>> for i16 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<i16>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i16),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<i32>> for i32 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<i32>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i32),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<i64>> for i64 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<i64>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i64),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<u8>> for u8 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<u8>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u8),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<u16>> for u16 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<u16>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u16),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<u32>> for u32 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<u32>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u32),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<u64>> for u64 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<u64>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u64),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<f32>> for f32 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<f32>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(f32::from_bits(datum as u32)),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<f64>> for f64 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<f64>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(f64::from_bits(datum as u64)),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<bool>> for bool {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<bool>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum != 0),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<char>> for char {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<char>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u8 as char),
            None => Err("Datum is NULL"),
        }
    }
}

//
// TryFrom trait implementations for pointer types
//

/// this is intended for Postgres functions that take an actual `cstring` argument, not for getting
/// a varlena argument type as a CStr.
impl<'a> TryFrom<PgDatum<&'a std::ffi::CStr>> for &'a std::ffi::CStr {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<&'a CStr>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => {
                let t = datum as *const std::os::raw::c_char;
                Ok(unsafe { std::ffi::CStr::from_ptr(t) })
            }
            None => Err("Datum is NULL"),
        }
    }
}

impl<T> TryFrom<PgDatum<PgBox<T>>> for PgBox<T>
where
    T: DatumCompatible<T>,
{
    type Error = Infallible;

    #[inline]
    fn try_from(value: PgDatum<PgBox<T>>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(PgBox::<T>::from_pg(datum as *mut T)),
            None => Ok(PgBox::null()),
        }
    }
}

impl<'a> TryFrom<PgDatum<&'a pg_sys::varlena>> for &'a str {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<&'a pg_sys::varlena>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => {
                let t = datum as *const pg_sys::varlena;
                Ok(unsafe { text_to_rust_str_unchecked(t) })
            }
            None => Err("Datum is NULL"),
        }
    }
}

impl<'a> TryFrom<PgDatum<pg_sys::Datum>> for &'a str {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => {
                let t = datum as *const pg_sys::varlena;
                Ok(unsafe { text_to_rust_str_unchecked(t) })
            }
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for String {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => {
                let t = datum as *const pg_sys::varlena;
                Ok(unsafe { text_to_rust_str_unchecked(t) }.to_string())
            }
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<String>> for String {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<String>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => {
                let t = datum as *const pg_sys::varlena;
                Ok(unsafe { text_to_rust_str_unchecked(t) }.to_string())
            }
            None => Err("Datum is NULL"),
        }
    }
}

impl<'a> TryFrom<PgDatum<&'a str>> for &'a str {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<&'a str>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => {
                let t = datum as *const pg_sys::varlena;
                Ok(unsafe { text_to_rust_str_unchecked(t) })
            }
            None => Err("Datum is NULL"),
        }
    }
}

impl<'a> TryFrom<PgDatum<&'a pg_sys::varlena>> for &'a pg_sys::varlena {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<&'a pg_sys::varlena>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => unsafe {
                let t = datum as *const pg_sys::varlena;
                Ok(t.as_ref().unwrap())
            },
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for i8 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i8),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for i16 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i16),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for i32 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i32),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for i64 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i64),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for u8 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u8),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for u16 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u16),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for u32 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u32),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for u64 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u64),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for f32 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(f32::from_bits(datum as u32)),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for f64 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(f64::from_bits(datum as u64)),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for bool {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum != 0),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<&PgDatum<pg_sys::Datum>> for char {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u8 as char),
            None => Err("Datum is NULL"),
        }
    }
}

//
// TryFrom trait implementations for pointer types
//

impl<'a> TryFrom<&PgDatum<pg_sys::Datum>> for &'a str {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => {
                let t = datum as *const pg_sys::varlena;
                Ok(unsafe { text_to_rust_str_unchecked(t) })
            }
            None => Err("Datum is NULL"),
        }
    }
}

impl<'a> TryFrom<&PgDatum<pg_sys::Datum>> for &'a pg_sys::varlena {
    type Error = &'static str;

    #[inline]
    fn try_from(value: &PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => unsafe {
                let t = datum as *const pg_sys::varlena;
                Ok(t.as_ref().unwrap())
            },
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for i8 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i8),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for i16 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i16),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for i32 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i32),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for i64 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as i64),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for u8 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u8),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for u16 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u16),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for u32 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u32),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for u64 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u64),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for f32 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(f32::from_bits(datum as u32)),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for f64 {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(f64::from_bits(datum as u64)),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for bool {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum != 0),
            None => Err("Datum is NULL"),
        }
    }
}

impl TryFrom<PgDatum<pg_sys::Datum>> for char {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => Ok(datum as u8 as char),
            None => Err("Datum is NULL"),
        }
    }
}

//
// TryFrom trait implementations for pointer types
//

impl<'a> TryFrom<PgDatum<pg_sys::Datum>> for &'a pg_sys::varlena {
    type Error = &'static str;

    #[inline]
    fn try_from(value: PgDatum<pg_sys::Datum>) -> Result<Self, Self::Error> {
        match value.as_pg_datum() {
            Some(datum) => unsafe {
                let t = datum as *const pg_sys::varlena;
                Ok(t.as_ref().unwrap())
            },
            None => Err("Datum is NULL"),
        }
    }
}
