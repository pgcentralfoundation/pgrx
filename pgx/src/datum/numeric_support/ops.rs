use std::ops::{
    Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::numeric_support::call_numeric_func;
use crate::{pg_sys, AnyNumeric, Numeric};

impl<const P: u32, const S: u32> Deref for Numeric<P, S> {
    type Target = AnyNumeric;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

macro_rules! anynumeric_math_op {
    ($opname:ident, $trait_fnname:ident, $pg_func:ident) => {
        impl $opname<AnyNumeric> for AnyNumeric {
            type Output = AnyNumeric;

            #[inline]
            fn $trait_fnname(self, rhs: AnyNumeric) -> Self::Output {
                call_numeric_func(pg_sys::$pg_func, vec![self.as_datum(), rhs.as_datum()])
            }
        }
    };
}

anynumeric_math_op!(Add, add, numeric_add);
anynumeric_math_op!(Sub, sub, numeric_sub);
anynumeric_math_op!(Mul, mul, numeric_mul);
anynumeric_math_op!(Div, div, numeric_div);
anynumeric_math_op!(Rem, rem, numeric_mod);

macro_rules! numeric_math_op {
    ($opname:ident, $trait_fnname:ident, $pg_func:ident) => {
        /// Doing this operation on two [`Numeric<P, S>`] instances results in a new [`AnyNumeric`].
        impl<const P: u32, const S: u32, const Q: u32, const T: u32> $opname<Numeric<Q, T>>
            for Numeric<P, S>
        {
            type Output = AnyNumeric;

            #[inline]
            fn $trait_fnname(self, rhs: Numeric<Q, T>) -> Self::Output {
                call_numeric_func(pg_sys::$pg_func, vec![self.as_datum(), rhs.as_datum()])
            }
        }
    };
}

numeric_math_op!(Add, add, numeric_add);
numeric_math_op!(Sub, sub, numeric_sub);
numeric_math_op!(Mul, mul, numeric_mul);
numeric_math_op!(Div, div, numeric_div);
numeric_math_op!(Rem, rem, numeric_mod);

impl Neg for AnyNumeric {
    type Output = AnyNumeric;

    #[inline]
    fn neg(self) -> Self::Output {
        call_numeric_func(pg_sys::numeric_uminus, vec![self.as_datum()])
    }
}

impl<const P: u32, const S: u32> Neg for Numeric<P, S> {
    type Output = Numeric<P, S>;

    #[inline]
    fn neg(self) -> Self::Output {
        Numeric(call_numeric_func(pg_sys::numeric_uminus, vec![self.as_datum()]))
    }
}

macro_rules! anynumeric_assign_op_from_anynumeric {
    ($opname:ident, $trait_fname:ident, $pg_func:ident) => {
        impl $opname<AnyNumeric> for AnyNumeric {
            fn $trait_fname(&mut self, rhs: AnyNumeric) {
                *self = call_numeric_func(pg_sys::$pg_func, vec![self.as_datum(), rhs.as_datum()]);
            }
        }
    };
}

anynumeric_assign_op_from_anynumeric!(AddAssign, add_assign, numeric_add);
anynumeric_assign_op_from_anynumeric!(SubAssign, sub_assign, numeric_sub);
anynumeric_assign_op_from_anynumeric!(MulAssign, mul_assign, numeric_mul);
anynumeric_assign_op_from_anynumeric!(DivAssign, div_assign, numeric_div);
anynumeric_assign_op_from_anynumeric!(RemAssign, rem_assign, numeric_mod);

macro_rules! anynumeric_assign_op_from_primitive {
    ($opname:ident, $trait_fname:ident, $ty:ty, $op:tt) => {
        impl $opname<$ty> for AnyNumeric {
            #[inline]
            fn $trait_fname(&mut self, rhs: $ty) {
                *self = self.copy() $op AnyNumeric::from(rhs);
            }
        }
    }
}

anynumeric_assign_op_from_primitive!(AddAssign, add_assign, i128, +);
anynumeric_assign_op_from_primitive!(AddAssign, add_assign, isize, +);
anynumeric_assign_op_from_primitive!(AddAssign, add_assign, i64, +);
anynumeric_assign_op_from_primitive!(AddAssign, add_assign, i32, +);
anynumeric_assign_op_from_primitive!(AddAssign, add_assign, i16, +);
anynumeric_assign_op_from_primitive!(AddAssign, add_assign, i8, +);
anynumeric_assign_op_from_primitive!(AddAssign, add_assign, u128, +);
anynumeric_assign_op_from_primitive!(AddAssign, add_assign, usize, +);
anynumeric_assign_op_from_primitive!(AddAssign, add_assign, u64, +);
anynumeric_assign_op_from_primitive!(AddAssign, add_assign, u32, +);
anynumeric_assign_op_from_primitive!(AddAssign, add_assign, u16, +);
anynumeric_assign_op_from_primitive!(AddAssign, add_assign, u8, +);

anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, i128, -);
anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, isize, -);
anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, i64, -);
anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, i32, -);
anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, i16, -);
anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, i8, -);
anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, u128, -);
anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, usize, -);
anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, u64, -);
anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, u32, -);
anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, u16, -);
anynumeric_assign_op_from_primitive!(SubAssign, sub_assign, u8, -);

anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, i128, *);
anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, isize, *);
anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, i64, *);
anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, i32, *);
anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, i16, *);
anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, i8, *);
anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, u128, *);
anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, usize, *);
anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, u64, *);
anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, u32, *);
anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, u16, *);
anynumeric_assign_op_from_primitive!(MulAssign, mul_assign, u8, *);

anynumeric_assign_op_from_primitive!(DivAssign, div_assign, i128, /);
anynumeric_assign_op_from_primitive!(DivAssign, div_assign, isize, /);
anynumeric_assign_op_from_primitive!(DivAssign, div_assign, i64, /);
anynumeric_assign_op_from_primitive!(DivAssign, div_assign, i32, /);
anynumeric_assign_op_from_primitive!(DivAssign, div_assign, i16, /);
anynumeric_assign_op_from_primitive!(DivAssign, div_assign, i8, /);
anynumeric_assign_op_from_primitive!(DivAssign, div_assign, u128, /);
anynumeric_assign_op_from_primitive!(DivAssign, div_assign, usize, /);
anynumeric_assign_op_from_primitive!(DivAssign, div_assign, u64, /);
anynumeric_assign_op_from_primitive!(DivAssign, div_assign, u32, /);
anynumeric_assign_op_from_primitive!(DivAssign, div_assign, u16, /);
anynumeric_assign_op_from_primitive!(DivAssign, div_assign, u8, /);

anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, i128, %);
anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, isize, %);
anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, i64, %);
anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, i32, %);
anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, i16, %);
anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, i8, %);
anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, u128, %);
anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, usize, %);
anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, u64, %);
anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, u32, %);
anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, u16, %);
anynumeric_assign_op_from_primitive!(RemAssign, rem_assign, u8, %);

macro_rules! anynumeric_assign_op_from_float {
    ($opname:ident, $trait_fname:ident, $ty:ty, $op:tt) => {
        impl $opname<$ty> for AnyNumeric {
            #[inline]
            fn $trait_fname(&mut self, rhs: $ty) {
                // these versions of Postgres could produce an error when unwrapping a try_from(float)
                #[cfg(any(feature = "pg11", feature = "pg12", feature = "pg13"))]
                {
                    *self = self.copy() $op AnyNumeric::try_from(rhs).unwrap();
                }

                // these versions won't, so we use .unwrap_unchecked()
                #[cfg(any(feature = "pg14", feature = "pg15"))]
                {
                    unsafe {
                        *self = self.copy() $op AnyNumeric::try_from(rhs).unwrap_unchecked();
                    }
                }
            }
        }
    };
}

anynumeric_assign_op_from_float!(AddAssign, add_assign, f32, +);
anynumeric_assign_op_from_float!(SubAssign, sub_assign, f32, -);
anynumeric_assign_op_from_float!(MulAssign, mul_assign, f32, *);
anynumeric_assign_op_from_float!(DivAssign, div_assign, f32, /);
anynumeric_assign_op_from_float!(RemAssign, rem_assign, f32, %);

anynumeric_assign_op_from_float!(AddAssign, add_assign, f64, +);
anynumeric_assign_op_from_float!(SubAssign, sub_assign, f64, -);
anynumeric_assign_op_from_float!(MulAssign, mul_assign, f64, *);
anynumeric_assign_op_from_float!(DivAssign, div_assign, f64, /);
anynumeric_assign_op_from_float!(RemAssign, rem_assign, f64, %);
