use std::cmp::Ordering;

use crate::{direct_function_call, pg_sys, AnyNumeric, Numeric};

impl PartialEq<AnyNumeric> for AnyNumeric {
    #[inline]
    fn eq(&self, other: &AnyNumeric) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_eq, vec![self.as_datum(), other.as_datum()])
                .unwrap()
        }
    }

    #[inline]
    fn ne(&self, other: &AnyNumeric) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_ne, vec![self.as_datum(), other.as_datum()])
                .unwrap()
        }
    }
}

impl PartialOrd for AnyNumeric {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_nan() || other.is_nan() {
            None
        } else {
            let cmp: i32 = unsafe {
                direct_function_call(pg_sys::numeric_cmp, vec![self.as_datum(), other.as_datum()])
                    .unwrap()
            };
            if cmp < 0 {
                Some(Ordering::Less)
            } else if cmp > 0 {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }

    #[inline]
    fn lt(&self, other: &Self) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_lt, vec![self.as_datum(), other.as_datum()])
                .unwrap()
        }
    }

    #[inline]
    fn le(&self, other: &Self) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_le, vec![self.as_datum(), other.as_datum()])
                .unwrap()
        }
    }

    #[inline]
    fn gt(&self, other: &Self) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_gt, vec![self.as_datum(), other.as_datum()])
                .unwrap()
        }
    }

    #[inline]
    fn ge(&self, other: &Self) -> bool {
        unsafe {
            direct_function_call(pg_sys::numeric_ge, vec![self.as_datum(), other.as_datum()])
                .unwrap()
        }
    }
}

impl<const P: u32, const S: u32> PartialEq for Numeric<P, S> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_anynumeric().eq(other.as_anynumeric())
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.as_anynumeric().ne(other.as_anynumeric())
    }
}

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
