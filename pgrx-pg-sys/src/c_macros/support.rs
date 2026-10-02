//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! C expression support for generated macros under the checked signed-char LP64 profile.
//!
//! C type identity survives equal Rust representations: `long` and `long long`
//! remain different kinds even though both store an `i64`. Raw 64-bit and
//! pointer-sized Rust integers therefore require an explicit `CValue` tag.
//! The integer helpers reject operations outside the accepted C value domain before
//! an invalid Rust operation can occur. The expression module additionally models
//! typed pointers, floats, enums, record storage and unsafe places. Generated native
//! capabilities retain their caller access and FFI obligations.
//!
//! The generator must validate the C profile and emit its Rust target guard.
//! Unsigned arithmetic wraps. Signed arithmetic follows `Undefined` or
//! `Wrapping`; division and shift-count restrictions apply independently.
//! Signed right shifts use Clang's arithmetic shift implementation choice.
//! Helpers are runtime operations; their trait calls do not establish const use.

use core::marker::PhantomData;

pub mod expression;
pub mod expression_result;
pub mod statements;

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// One fundamental C integer identity in the supported target model.
pub trait CInteger: sealed::Sealed + Copy + core::fmt::Debug + Eq {
    type Repr: Copy + core::fmt::Debug + Eq;
    type Promoted: PromotedInteger;
    /// Ordinary value identity after compiler-owned expression metadata is lost
    /// at a variable or public evaluated-value boundary.
    type Boundary: CInteger<Repr = Self::Repr>;
    const BITS: u32;
    const SIGNED: bool;
    const RANK: u8;

    #[doc(hidden)]
    fn encode(value: Self::Repr) -> u128;
    #[doc(hidden)]
    fn decode(bits: u128) -> Self::Repr;
}

/// A C integer type after integer promotion.
pub trait PromotedInteger: CInteger<Promoted = Self> {}

macro_rules! integer_kinds {
    ($(($kind:ident, $repr:ty, $bits:literal, $signed:literal, $rank:literal, $promoted:ident)),+ $(,)?) => {
        $(
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct $kind;
            impl sealed::Sealed for $kind {}
            impl CInteger for $kind {
                type Repr = $repr;
                type Promoted = $promoted;
                type Boundary = Self;
                const BITS: u32 = $bits;
                const SIGNED: bool = $signed;
                const RANK: u8 = $rank;
                fn encode(value: Self::Repr) -> u128 { value as u128 }
                fn decode(bits: u128) -> Self::Repr { bits as $repr }
            }
        )+
    };
}

integer_kinds!(
    (CChar, i8, 8, true, 1, CInt),
    (CSignedChar, i8, 8, true, 1, CInt),
    (CUnsignedChar, u8, 8, false, 1, CInt),
    (CShort, i16, 16, true, 2, CInt),
    (CUnsignedShort, u16, 16, false, 2, CInt),
    (CInt, i32, 32, true, 3, CInt),
    (CUnsignedInt, u32, 32, false, 3, CUnsignedInt),
    (CLong, i64, 64, true, 4, CLong),
    (CUnsignedLong, u64, 64, false, 4, CUnsignedLong),
    (CLongLong, i64, 64, true, 5, CLongLong),
    (CUnsignedLongLong, u64, 64, false, 5, CUnsignedLongLong),
    (CInt128, i128, 128, true, 6, CInt128),
    (CUnsignedInt128, u128, 128, false, 6, CUnsignedInt128),
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CBool;
impl sealed::Sealed for CBool {}
impl CInteger for CBool {
    type Repr = bool;
    type Promoted = CInt;
    type Boundary = Self;
    const BITS: u32 = 8;
    const SIGNED: bool = false;
    const RANK: u8 = 0;
    fn encode(value: bool) -> u128 {
        u128::from(value)
    }
    fn decode(bits: u128) -> bool {
        // Conversion to _Bool tests the source value, before any narrowing.
        bits != 0
    }
}

macro_rules! promoted_kinds {
    ($($kind:ident),+ $(,)?) => { $(impl PromotedInteger for $kind {})+ };
}
promoted_kinds!(
    CInt,
    CUnsignedInt,
    CLong,
    CUnsignedLong,
    CLongLong,
    CUnsignedLongLong,
    CInt128,
    CUnsignedInt128,
);

/// A value with its C type identity. This representation is not a blanket FFI ABI promise.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CValue<K: CInteger> {
    repr: K::Repr,
    kind: PhantomData<K>,
}

impl<K: CInteger> CValue<K> {
    /// Tag native storage with an explicit C identity.
    pub const fn new(repr: K::Repr) -> Self {
        Self { repr, kind: PhantomData }
    }

    /// Extract native storage for a checked binding or another explicit conversion.
    pub const fn get(self) -> K::Repr {
        self.repr
    }

    fn bits(self) -> u128 {
        K::encode(self.repr)
    }

    fn from_bits(bits: u128) -> Self {
        Self::new(K::decode(bits))
    }
}

impl<K: CInteger> sealed::Sealed for CValue<K> {}

/// An accepted C scalar input. This trait is sealed to the documented finite family.
pub trait IntoCValue: sealed::Sealed {
    type Kind: CInteger;
    fn into_c_value(self) -> CValue<Self::Kind>;
}

impl<K: CInteger> IntoCValue for CValue<K> {
    type Kind = K;
    fn into_c_value(self) -> Self {
        self
    }
}

macro_rules! raw_inputs {
    ($(($repr:ty, $kind:ident)),+ $(,)?) => {
        $(
            impl sealed::Sealed for $repr {}
            impl IntoCValue for $repr {
                type Kind = $kind;
                fn into_c_value(self) -> CValue<$kind> { CValue::new(self) }
            }
        )+
    };
}
raw_inputs!(
    (bool, CBool),
    (i8, CSignedChar),
    (u8, CUnsignedChar),
    (i16, CShort),
    (u16, CUnsignedShort),
    (i32, CInt),
    (u32, CUnsignedInt),
    (i128, CInt128),
    (u128, CUnsignedInt128),
);

/// The usual arithmetic conversion between two promoted C identities.
pub trait Common<Rhs: PromotedInteger>: PromotedInteger {
    type Output: PromotedInteger;
}

impl<K: PromotedInteger> Common<K> for K {
    type Output = K;
}

// Only distinct unordered promoted pairs are listed; the rules are symmetric.
// In LP64, unsigned long plus signed long long becomes unsigned long long:
// the latter's higher rank cannot make its equal-width signed range sufficient.
macro_rules! common_pairs {
    ($(($left:ident, $right:ident, $output:ident)),+ $(,)?) => {
        $(
            impl Common<$right> for $left { type Output = $output; }
            impl Common<$left> for $right { type Output = $output; }
        )+
    };
}
common_pairs!(
    (CInt, CUnsignedInt, CUnsignedInt),
    (CInt, CLong, CLong),
    (CInt, CUnsignedLong, CUnsignedLong),
    (CInt, CLongLong, CLongLong),
    (CInt, CUnsignedLongLong, CUnsignedLongLong),
    (CInt, CInt128, CInt128),
    (CInt, CUnsignedInt128, CUnsignedInt128),
    (CUnsignedInt, CLong, CLong),
    (CUnsignedInt, CUnsignedLong, CUnsignedLong),
    (CUnsignedInt, CLongLong, CLongLong),
    (CUnsignedInt, CUnsignedLongLong, CUnsignedLongLong),
    (CUnsignedInt, CInt128, CInt128),
    (CUnsignedInt, CUnsignedInt128, CUnsignedInt128),
    (CLong, CUnsignedLong, CUnsignedLong),
    (CLong, CLongLong, CLongLong),
    (CLong, CUnsignedLongLong, CUnsignedLongLong),
    (CLong, CInt128, CInt128),
    (CLong, CUnsignedInt128, CUnsignedInt128),
    (CUnsignedLong, CLongLong, CUnsignedLongLong),
    (CUnsignedLong, CUnsignedLongLong, CUnsignedLongLong),
    (CUnsignedLong, CInt128, CInt128),
    (CUnsignedLong, CUnsignedInt128, CUnsignedInt128),
    (CLongLong, CUnsignedLongLong, CUnsignedLongLong),
    (CLongLong, CInt128, CInt128),
    (CLongLong, CUnsignedInt128, CUnsignedInt128),
    (CUnsignedLongLong, CInt128, CInt128),
    (CUnsignedLongLong, CUnsignedInt128, CUnsignedInt128),
    (CInt128, CUnsignedInt128, CUnsignedInt128),
);

/// Two accepted inputs with a statically determined common arithmetic type.
pub trait ArithmeticInput<Rhs: IntoCValue>: IntoCValue {
    type Common: PromotedInteger;
    #[doc(hidden)]
    fn common_values(self, rhs: Rhs) -> (CValue<Self::Common>, CValue<Self::Common>);
}

impl<L: IntoCValue, R: IntoCValue> ArithmeticInput<R> for L
where
    <L::Kind as CInteger>::Promoted: Common<<R::Kind as CInteger>::Promoted>,
{
    type Common =
        <<L::Kind as CInteger>::Promoted as Common<<R::Kind as CInteger>::Promoted>>::Output;
    fn common_values(self, rhs: R) -> (CValue<Self::Common>, CValue<Self::Common>) {
        (cast(self), cast(rhs))
    }
}

/// Establish integer input membership without changing C identity.
pub fn value<V: IntoCValue>(input: V) -> CValue<V::Kind> {
    input.into_c_value()
}

/// C integer conversion, including two's-complement signed narrowing and `_Bool` truth conversion.
pub fn cast<K: CInteger, V: IntoCValue>(input: V) -> CValue<K> {
    CValue::from_bits(input.into_c_value().bits())
}

/// Apply C integer promotion, including promotion of `_Bool` and narrow integers to `int`.
pub fn promote<V: IntoCValue>(input: V) -> CValue<<V::Kind as CInteger>::Promoted> {
    cast(input)
}

/// A selected compiler's signed arithmetic behavior.
pub trait OverflowPolicy: sealed::Sealed {
    const WRAPPING: bool;
}

/// Signed overflow is outside the accepted C domain; checked helpers panic on it.
pub struct Undefined;
impl sealed::Sealed for Undefined {}
impl OverflowPolicy for Undefined {
    const WRAPPING: bool = false;
}

/// Clang's `-fwrapv` signed arithmetic behavior.
pub struct Wrapping;
impl sealed::Sealed for Wrapping {}
impl OverflowPolicy for Wrapping {
    const WRAPPING: bool = true;
}

fn signed_bounds<K: CInteger>() -> (i128, i128) {
    if K::BITS == 128 {
        (i128::MIN, i128::MAX)
    } else {
        let high_bit = 1_i128 << (K::BITS - 1);
        (-high_bit, high_bit - 1)
    }
}

fn checked_signed<K: CInteger>(result: Option<i128>) -> CValue<K> {
    let result = result.expect("C signed arithmetic overflow");
    let (min, max) = signed_bounds::<K>();
    assert!((min..=max).contains(&result), "C signed arithmetic overflow");
    CValue::from_bits(result as u128)
}

macro_rules! arithmetic {
    ($name:ident, $wrapping:ident, $checked:ident) => {
        pub fn $name<P: OverflowPolicy, L: ArithmeticInput<R>, R: IntoCValue>(
            left: L,
            right: R,
        ) -> CValue<L::Common> {
            let (left, right) = left.common_values(right);
            if <L::Common as CInteger>::SIGNED && !P::WRAPPING {
                checked_signed((left.bits() as i128).$checked(right.bits() as i128))
            } else {
                CValue::from_bits(left.bits().$wrapping(right.bits()))
            }
        }
    };
}
arithmetic!(add, wrapping_add, checked_add);
arithmetic!(sub, wrapping_sub, checked_sub);
arithmetic!(mul, wrapping_mul, checked_mul);

pub fn neg<P: OverflowPolicy, V: IntoCValue>(input: V) -> CValue<<V::Kind as CInteger>::Promoted> {
    let input = promote(input);
    if <<V::Kind as CInteger>::Promoted as CInteger>::SIGNED && !P::WRAPPING {
        checked_signed((input.bits() as i128).checked_neg())
    } else {
        CValue::from_bits(0_u128.wrapping_sub(input.bits()))
    }
}

macro_rules! division {
    ($name:ident, $operator:tt) => {
        pub fn $name<L: ArithmeticInput<R>, R: IntoCValue>(
            left: L,
            right: R,
        ) -> CValue<L::Common> {
            let (left, right) = left.common_values(right);
            assert!(right.bits() != 0, "C integer division by zero");
            if <L::Common as CInteger>::SIGNED {
                let (left, right) = (left.bits() as i128, right.bits() as i128);
                let (min, _) = signed_bounds::<L::Common>();
                assert!(left != min || right != -1, "C signed division overflow");
                CValue::from_bits((left $operator right) as u128)
            } else {
                CValue::from_bits(left.bits() $operator right.bits())
            }
        }
    };
}
division!(div, /);
division!(rem, %);

macro_rules! bitwise {
    ($name:ident, $operator:tt) => {
        pub fn $name<L: ArithmeticInput<R>, R: IntoCValue>(
            left: L,
            right: R,
        ) -> CValue<L::Common> {
            let (left, right) = left.common_values(right);
            CValue::from_bits(left.bits() $operator right.bits())
        }
    };
}
bitwise!(bitand, &);
bitwise!(bitor, |);
bitwise!(bitxor, ^);

pub fn bitnot<V: IntoCValue>(input: V) -> CValue<<V::Kind as CInteger>::Promoted> {
    CValue::from_bits(!promote(input).bits())
}

macro_rules! comparisons {
    ($name:ident, $operator:tt) => {
        pub fn $name<L: ArithmeticInput<R>, R: IntoCValue>(
            left: L,
            right: R,
        ) -> CValue<CInt> {
            let (left, right) = left.common_values(right);
            let result = if <L::Common as CInteger>::SIGNED {
                (left.bits() as i128) $operator (right.bits() as i128)
            } else {
                left.bits() $operator right.bits()
            };
            CValue::new(i32::from(result))
        }
    };
}
comparisons!(eq, ==);
comparisons!(ne, !=);
comparisons!(lt, <);
comparisons!(le, <=);
comparisons!(gt, >);
comparisons!(ge, >=);

pub fn truth<V: IntoCValue>(input: V) -> bool {
    input.into_c_value().bits() != 0
}

pub fn logical_not<V: IntoCValue>(input: V) -> CValue<CInt> {
    CValue::new(i32::from(!truth(input)))
}

fn shift_count<K: CInteger, V: IntoCValue>(input: V) -> u32 {
    let count = promote(input);
    assert!(
        !<<V::Kind as CInteger>::Promoted as CInteger>::SIGNED || (count.bits() as i128) >= 0,
        "negative C shift count"
    );
    assert!(count.bits() < u128::from(K::BITS), "C shift count exceeds width");
    count.bits() as u32
}

pub fn shl<P: OverflowPolicy, L: IntoCValue, R: IntoCValue>(
    left: L,
    right: R,
) -> CValue<<L::Kind as CInteger>::Promoted> {
    let left = promote(left);
    let count = shift_count::<<L::Kind as CInteger>::Promoted, _>(right);
    // LLVM 21 Clang's EmitShl uses a plain shl under -fwrapv, without a
    // signed-base UB check; shift-count restrictions remain independent.
    // https://github.com/llvm/llvm-project/blob/llvmorg-21.1.0/clang/lib/CodeGen/CGExprScalar.cpp#L4364-L4444
    if <<L::Kind as CInteger>::Promoted as CInteger>::SIGNED && !P::WRAPPING {
        let left = left.bits() as i128;
        let (_, max) = signed_bounds::<<L::Kind as CInteger>::Promoted>();
        assert!(left >= 0 && left <= (max >> count), "C signed left shift overflow");
        CValue::from_bits((left << count) as u128)
    } else {
        CValue::from_bits(left.bits() << count)
    }
}

pub fn shr<L: IntoCValue, R: IntoCValue>(
    left: L,
    right: R,
) -> CValue<<L::Kind as CInteger>::Promoted> {
    let left = promote(left);
    let count = shift_count::<<L::Kind as CInteger>::Promoted, _>(right);
    if <<L::Kind as CInteger>::Promoted as CInteger>::SIGNED {
        CValue::from_bits(((left.bits() as i128) >> count) as u128)
    } else {
        CValue::from_bits(left.bits() >> count)
    }
}

/// The evaluated arm of a lazy conditional, retaining both arms' types for conversion.
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// Convert only the evaluated arm using the C arithmetic conditional's common type.
pub fn select<L: ArithmeticInput<R>, R: IntoCValue>(arm: Either<L, R>) -> CValue<L::Common> {
    match arm {
        Either::Left(left) => cast(left),
        Either::Right(right) => cast(right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_promotions_and_distinct_ranks() {
        let _: CValue<CUnsignedChar> = value(255_u8);
        let promoted: CValue<CInt> = promote(255_u8);
        assert_eq!(promoted.get(), 255);
        let mixed: CValue<CUnsignedLongLong> = add::<Wrapping, _, _>(
            CValue::<CUnsignedLong>::new(u64::MAX),
            CValue::<CLongLong>::new(0),
        );
        assert_eq!(mixed.get(), u64::MAX);
        let wider: CValue<CInt128> = add::<Undefined, _, _>(mixed, 1_i128);
        assert_eq!(wider.get(), i128::from(u64::MAX) + 1);
    }

    #[test]
    fn casts_test_truth_before_narrowing_and_sign_extend() {
        assert!(cast::<CBool, _>(256_i32).get());
        assert_eq!(cast::<CUnsignedChar, _>(-1_i32).get(), 255);
        assert_eq!(cast::<CInt128, _>(-1_i8).get(), -1);
        assert_eq!(cast::<CInt, _>(u32::MAX).get(), -1);
        assert_eq!(cast::<CUnsignedInt128, _>(-1_i32).get(), u128::MAX);
    }

    #[test]
    fn signed_and_unsigned_arithmetic_have_separate_policies() {
        assert_eq!(add::<Wrapping, _, _>(i32::MAX, 1_i32).get(), i32::MIN);
        assert_eq!(neg::<Wrapping, _>(i32::MIN).get(), i32::MIN);
        assert_eq!(add::<Undefined, _, _>(u32::MAX, 1_u32).get(), 0);
        assert!(std::panic::catch_unwind(|| add::<Undefined, _, _>(i32::MAX, 1_i32)).is_err());
        assert!(std::panic::catch_unwind(|| neg::<Undefined, _>(i128::MIN)).is_err());
    }

    #[test]
    fn division_checks_both_quotient_and_remainder_domains() {
        assert_eq!(div(-7_i32, 3_i32).get(), -2);
        assert_eq!(rem(-7_i32, 3_i32).get(), -1);
        assert!(std::panic::catch_unwind(|| div(i32::MIN, -1_i32)).is_err());
        assert!(std::panic::catch_unwind(|| rem(i32::MIN, -1_i32)).is_err());
        assert!(std::panic::catch_unwind(|| rem(1_u32, 0_u32)).is_err());
    }

    #[test]
    fn shift_domains_and_left_result_promotion() {
        let promoted: CValue<CInt> = shl::<Undefined, _, _>(1_u8, 15_i32);
        assert_eq!(promoted.get(), 32768);
        assert_eq!(shl::<Wrapping, _, _>(1_i32, 31_i32).get(), i32::MIN);
        assert_eq!(shr(-4_i32, 1_i32).get(), -2);
        assert!(std::panic::catch_unwind(|| shl::<Undefined, _, _>(1_i32, 31_i32)).is_err());
        assert!(std::panic::catch_unwind(|| shl::<Wrapping, _, _>(1_i32, 32_i32)).is_err());
        assert!(std::panic::catch_unwind(|| shr(1_i32, -1_i32)).is_err());
        assert!(std::panic::catch_unwind(|| shr(1_i32, u128::MAX)).is_err());
    }

    #[test]
    fn comparisons_return_c_int_after_common_conversion() {
        let result: CValue<CInt> = lt(-1_i32, 1_u32);
        assert_eq!(result.get(), 0);
        assert_eq!(logical_not(false).get(), 1);
        assert_eq!(bitnot(0_u8).get(), -1);
    }

    #[test]
    fn conditional_only_evaluates_selected_arm() {
        let mut calls = 0;
        let arm = if truth(1_i32) {
            Either::Left(-1_i32)
        } else {
            calls += 1;
            Either::Right(1_u32)
        };
        let result: CValue<CUnsignedInt> = select(arm);
        assert_eq!(result.get(), u32::MAX);
        assert_eq!(calls, 0);
    }
}
