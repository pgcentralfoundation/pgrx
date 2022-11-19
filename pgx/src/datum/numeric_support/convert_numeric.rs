//! Conversion implementations for converting from a thing into [`Numeric<P, S>`]
use core::str::FromStr;
use std::ffi::CStr;

use pgx_pg_sys::AsPgCStr;

use crate::numeric_support::convert::from_primitive_helper;
use crate::numeric_support::error::Error;
use crate::{pg_sys, AnyNumeric, Numeric};

impl<const P: u32, const S: u32> TryFrom<AnyNumeric> for Numeric<P, S> {
    type Error = Error;

    #[inline]
    fn try_from(value: AnyNumeric) -> Result<Self, Self::Error> {
        from_primitive_helper::<_, P, S>(value.copy(), pg_sys::numeric)
    }
}

macro_rules! numeric_try_from_signed {
    ($ty:ty, $as_:ty, $pg_func:ident) => {
        impl<const P: u32, const S: u32> TryFrom<$ty> for Numeric<P, S> {
            type Error = Error;

            #[inline]
            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                from_primitive_helper::<_, P, S>((value as $as_), pg_sys::$pg_func)
            }
        }
    };
}

numeric_try_from_signed!(i64, i64, int8_numeric);
numeric_try_from_signed!(i32, i32, int4_numeric);
numeric_try_from_signed!(i16, i16, int2_numeric);
numeric_try_from_signed!(i8, i16, int2_numeric);

macro_rules! numeric_try_from_oversized_primitive {
    ($ty:ty, $as_:ty, $pg_func:ident) => {
        impl<const P: u32, const S: u32> TryFrom<$ty> for Numeric<P, S> {
            type Error = Error;

            #[inline]
            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                match <$as_>::try_from(value) {
                    Ok(value) => from_primitive_helper::<_, P, S>(value, pg_sys::$pg_func),
                    Err(_) => Numeric::from_str(value.to_string().as_str()),
                }
            }
        }
    };
}

numeric_try_from_oversized_primitive!(i128, i64, int8_numeric);
numeric_try_from_oversized_primitive!(isize, i64, int8_numeric);

numeric_try_from_oversized_primitive!(u128, i64, int8_numeric);
numeric_try_from_oversized_primitive!(usize, i64, int8_numeric);
numeric_try_from_oversized_primitive!(u64, i64, int8_numeric);
numeric_try_from_oversized_primitive!(u32, i32, int4_numeric);
numeric_try_from_oversized_primitive!(u16, i16, int2_numeric);
numeric_try_from_oversized_primitive!(u8, i16, int2_numeric);

numeric_try_from_oversized_primitive!(f32, f32, float4_numeric);
numeric_try_from_oversized_primitive!(f64, f64, float8_numeric);

impl<const P: u32, const S: u32> TryFrom<&CStr> for Numeric<P, S> {
    type Error = Error;

    #[inline]
    fn try_from(value: &CStr) -> Result<Self, Self::Error> {
        Numeric::from_str(value.to_string_lossy().as_ref())
    }
}

impl<const P: u32, const S: u32> TryFrom<&str> for Numeric<P, S> {
    type Error = Error;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Numeric::from_str(value)
    }
}

impl<const P: u32, const S: u32> FromStr for Numeric<P, S> {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unsafe {
            let ptr = s.as_pg_cstr();
            let cstr = CStr::from_ptr(ptr);
            let numeric = from_primitive_helper::<_, P, S>(cstr, pg_sys::numeric_in);
            pg_sys::pfree(ptr.cast());
            numeric
        }
    }
}
