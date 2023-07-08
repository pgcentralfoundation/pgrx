use std::cmp::Ordering;

use crate::{direct_function_call, pg_sys, AnyNumeric, Numeric};

impl PartialEq<AnyNumeric> for AnyNumeric {
    /// Postgres implementation detail:  Unlike in Rust, Postgres considers `NaN` as equal
    #[inline]
    fn eq(&self, other: &AnyNumeric) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_eq, &[self.as_datum(), other.as_datum()]).unwrap()
        }
    }

    #[inline]
    fn ne(&self, other: &AnyNumeric) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_ne, &[self.as_datum(), other.as_datum()]).unwrap()
        }
    }
}

/// Postgres implementation detail:  Unlike in Rust, Postgres considers `NaN` as equal
impl Eq for AnyNumeric {}

impl PartialOrd for AnyNumeric {
    /// Postgres implementation detail:  Unlike in Rust, Postgres considers `NaN` as equal
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_lt, &[self.as_datum(), other.as_datum()]).unwrap()
        }
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_le, &[self.as_datum(), other.as_datum()]).unwrap()
        }
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_gt, &[self.as_datum(), other.as_datum()]).unwrap()
        }
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_ge, &[self.as_datum(), other.as_datum()]).unwrap()
        }
    }
}

impl Ord for AnyNumeric {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp: i32 = unsafe {
            direct_function_call(pg_sys::numeric_cmp, &[self.as_datum(), other.as_datum()]).unwrap()
        };
        if cmp < 0 {
            Ordering::Less
        } else if cmp > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl<const P: u32, const S: u32> PartialEq for Numeric<P, S> {
    /// Postgres implementation detail:  Unlike in Rust, Postgres considers `NaN` as equal
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_anynumeric().eq(other.as_anynumeric())
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.as_anynumeric().ne(other.as_anynumeric())
    }
}

/// Postgres implementation detail:  Unlike in Rust, Postgres considers `NaN` as equal
impl<const P: u32, const S: u32> Eq for Numeric<P, S> {}

impl<const P: u32, const S: u32> PartialOrd for Numeric<P, S> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_anynumeric().partial_cmp(other.as_anynumeric())
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        self.as_anynumeric().lt(other.as_anynumeric())
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        self.as_anynumeric().le(other.as_anynumeric())
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        self.as_anynumeric().gt(other.as_anynumeric())
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        self.as_anynumeric().ge(other.as_anynumeric())
    }
}

impl<const P: u32, const S: u32> Ord for Numeric<P, S> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_anynumeric().cmp(other.as_anynumeric())
    }
}
