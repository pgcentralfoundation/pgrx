use crate::{pg_sys, rust_str_to_text_p, text_to_rust_str_unchecked};

//
// for converting a pg_sys::Datum and a corresponding "is_null" bool into a typed Option
//

pub trait FromDatum<T>: Sized {
    fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<T>;
}

/// for bool
impl FromDatum<bool> for bool {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<bool> {
        if is_null {
            None
        } else {
            Some(datum != 0)
        }
    }
}

/// for char
impl FromDatum<i8> for i8 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<i8> {
        if is_null {
            None
        } else {
            Some(datum as i8)
        }
    }
}

/// for smallint
impl FromDatum<i16> for i16 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<i16> {
        if is_null {
            None
        } else {
            Some(datum as i16)
        }
    }
}

/// for integer
impl FromDatum<i32> for i32 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<i32> {
        if is_null {
            None
        } else {
            Some(datum as i32)
        }
    }
}

/// for bigint
impl FromDatum<i64> for i64 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<i64> {
        if is_null {
            None
        } else {
            Some(datum as i64)
        }
    }
}

/// for real
impl FromDatum<f32> for f32 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<f32> {
        if is_null {
            None
        } else {
            Some(f32::from_bits(datum as u32))
        }
    }
}

/// for double precision
impl FromDatum<f64> for f64 {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<f64> {
        if is_null {
            None
        } else {
            Some(f64::from_bits(datum as u64))
        }
    }
}

/// for text, varchar
impl<'a> FromDatum<&'a str> for &'a str {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<&'a str> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a varlena Datum was flagged as non-null but the datum is zero");
        } else {
            let varlena = unsafe { pg_sys::pg_detoast_datum(datum as *mut pg_sys::varlena) };
            Some(unsafe { text_to_rust_str_unchecked(varlena) })
        }
    }
}

impl FromDatum<String> for String {
    #[inline]
    fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<String> {
        if is_null {
            None
        } else if datum == 0 {
            panic!("a varlena Datum was flagged as non-null but the datum is zero");
        } else {
            let varlena = unsafe { pg_sys::pg_detoast_datum(datum as *mut pg_sys::varlena) };
            Some(unsafe { text_to_rust_str_unchecked(varlena) }.to_string())
        }
    }
}

/// for NULL -- always converts to a `None`, even if the is_null argument is false
impl FromDatum<()> for () {
    #[inline]
    fn from_datum(_datum: usize, _is_null: bool) -> Option<()> {
        None
    }
}

//
// for converting primitive types into Datums
//
// Primitive types can never be null, so we do a direct
// cast of the primitive type to pg_sys::Datum
//
//
pub trait IntoDatum<T>: Sized {
    fn into_datum(self) -> Option<pg_sys::Datum>;
}

/// for bool
impl IntoDatum<bool> for bool {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some((if self { 1 } else { 0 }) as pg_sys::Datum)
    }
}

/// for smallint
impl IntoDatum<i16> for i16 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self as pg_sys::Datum)
    }
}

/// for integer
impl IntoDatum<i32> for i32 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self as pg_sys::Datum)
    }
}

/// for bigint
impl IntoDatum<i64> for i64 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self as pg_sys::Datum)
    }
}

/// for real
impl IntoDatum<f32> for f32 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.to_bits() as pg_sys::Datum)
    }
}

/// for double precision
impl IntoDatum<f64> for f64 {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.to_bits() as pg_sys::Datum)
    }
}

/// for text, varchar
impl<'a> IntoDatum<&'a str> for &'a str {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(rust_str_to_text_p(&self) as pg_sys::Datum)
    }
}

impl IntoDatum<String> for String {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        self.as_str().into_datum()
    }
}

/// for NULL -- always converts to `None`
impl IntoDatum<()> for () {
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        None
    }
}
