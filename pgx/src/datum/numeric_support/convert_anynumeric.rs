//! Conversion implementations for from a thing into [AnyNumeric]
use core::ffi::CStr;
use core::str::FromStr;

use pg_sys::AsPgCStr;

use crate::numeric_support::call_numeric_func;
use crate::numeric_support::convert::from_primitive_helper;
use crate::numeric_support::error::Error;
use crate::{pg_sys, AnyNumeric, IntoDatum, Numeric};

impl<const P: u32, const S: u32> From<Numeric<P, S>> for AnyNumeric {
    #[inline]
    fn from(n: Numeric<P, S>) -> Self {
        n.0
    }
}

macro_rules! anynumeric_from_signed {
    ($ty:ty, $as_:ty, $func:ident) => {
        impl From<$ty> for AnyNumeric {
            #[inline]
            fn from(value: $ty) -> Self {
                call_numeric_func(pg_sys::$func, vec![(value as $as_).into_datum()])
            }
        }
    };
}

macro_rules! anynumeric_from_oversized_primitive {
    ($ty:ty, $signed:ty) => {
        impl From<$ty> for AnyNumeric {
            #[inline]
            fn from(value: $ty) -> Self {
                match <$signed>::try_from(value) {
                    Ok(value) => AnyNumeric::from(value),
                    Err(_) => AnyNumeric::try_from(value.to_string().as_str()).unwrap(),
                }
            }
        }
    };
}

anynumeric_from_signed!(isize, i64, int8_numeric);
anynumeric_from_signed!(i64, i64, int8_numeric);
anynumeric_from_signed!(i32, i32, int4_numeric);
anynumeric_from_signed!(i16, i16, int2_numeric);
anynumeric_from_signed!(i8, i16, int2_numeric);

anynumeric_from_oversized_primitive!(usize, i64);
anynumeric_from_oversized_primitive!(u64, i64);
anynumeric_from_oversized_primitive!(u32, i32);
anynumeric_from_oversized_primitive!(u16, i16);
anynumeric_from_oversized_primitive!(u8, i8);

anynumeric_from_oversized_primitive!(i128, i64);
anynumeric_from_oversized_primitive!(u128, i64);

macro_rules! anynumeric_from_float {
    ($ty:ty, $func:ident) => {
        impl TryFrom<$ty> for AnyNumeric {
            type Error = Error;

            #[inline]
            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                // these versions of Postgres can't represent +/-Infinity as a NUMERIC
                // so we run through a PgTryBuilder to ask Postgres to do the conversion which will
                // simply return the proper Error
                #[cfg(any(feature = "pg11", feature = "pg12", feature = "pg13"))]
                {
                    if value.is_infinite() {
                        return from_primitive_helper::<_, 0, 0>(value, pg_sys::$func)
                            .map(|n| n.into());
                    }
                }

                Ok(call_numeric_func(pg_sys::$func, vec![value.into_datum()]))
            }
        }
    };
}

anynumeric_from_float!(f32, float4_numeric);
anynumeric_from_float!(f64, float8_numeric);

impl TryFrom<&CStr> for AnyNumeric {
    type Error = Error;

    #[inline]
    fn try_from(value: &CStr) -> Result<Self, Self::Error> {
        AnyNumeric::from_str(value.to_string_lossy().as_ref())
    }
}

impl TryFrom<&str> for AnyNumeric {
    type Error = Error;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        AnyNumeric::from_str(value)
    }
}

impl FromStr for AnyNumeric {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unsafe {
            let ptr = s.as_pg_cstr();
            let cstr = CStr::from_ptr(ptr);
            let numeric =
                from_primitive_helper::<_, 0, 0>(cstr, pg_sys::numeric_in).map(|n| n.into());
            pg_sys::pfree(ptr.cast());
            numeric
        }
    }
}
