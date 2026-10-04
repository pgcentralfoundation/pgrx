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

/// Extend scalar C arithmetic with typed values, raw places, and declaration-driven native capabilities.
/// The generator selects operations; this runtime preserves their type/evaluation and access contracts.
pub mod expression;
/// Expose public macro results through one evaluated wrapper for predictable Rust inference.
/// Native extraction remains explicit so C semantic identity can survive further macro composition.
pub mod expression_result;
/// Guard statement-local name capture before Rust hygiene can change textual C substitution semantics.
/// The bounded token check runs during const evaluation and rejects uncertain invocation shapes.
pub mod statements;

/// Restrict C capability implementation to this crate and its generated native registrations.
/// External callers cannot widen the supported type families through trait implementations.
pub(crate) mod sealed {
    /// Keep the admitted C value and operation families under crate-owned registration.
    pub trait Sealed {}
}

/// One fundamental C integer identity in the supported target model.
pub trait CInteger: sealed::Sealed + Copy + core::fmt::Debug + Eq {
    /// Native bits/values used to store this C integer kind without erasing its rank.
    type Repr: Copy + core::fmt::Debug + Eq;
    /// C kind selected by integer promotion before unary, shift, or arithmetic operations.
    type Promoted: PromotedInteger;
    /// Ordinary value identity after compiler-owned expression metadata is lost
    /// at a variable or public evaluated-value boundary.
    type Boundary: CInteger<Repr = Self::Repr>;
    /// Width of the declared C storage representation before promotion.
    const BITS: u32;
    /// Whether the C kind interprets its storage bits as a signed value.
    const SIGNED: bool;
    /// C integer rank used by usual arithmetic conversions independently of storage width.
    const RANK: u8;

    /// Convert admitted integer or enum storage to the representation used by C conversion rules.
    #[doc(hidden)]
    fn encode(value: Self::Repr) -> u128;
    /// Recover the declared integer or enum representation from checked conversion bits.
    #[doc(hidden)]
    fn decode(bits: u128) -> Self::Repr;
}

/// A C integer type after integer promotion.
pub trait PromotedInteger: CInteger<Promoted = Self> {}

/// Define the finite C integer rank/storage table used by promotion, casts, and common-type selection.
macro_rules! integer_kinds {
    ($(($kind:ident, $repr:ty, $bits:literal, $signed:literal, $rank:literal, $promoted:ident)),+ $(,)?) => {
        $(
            /// Nominal marker preserving this C integer kind’s rank independently of its native Rust storage.
            #[derive(Clone, Copy, Debug, PartialEq, Eq)]
            pub struct $kind;
            /// Keep this admitted scalar or type-marker family under crate-owned capability registration.
            impl sealed::Sealed for $kind {}
            /// Retain this fundamental C kind’s distinct rank and promotion even when Rust storage widths coincide.
            impl CInteger for $kind {
                /// Native representation of this C kind, kept separate from its arithmetic rank.
                type Repr = $repr;
                /// C kind required by integer promotion before unary or usual arithmetic conversion.
                type Promoted = $promoted;
                /// Ordinary integer identity retained after evaluated-value boundaries remove source-only metadata.
                type Boundary = Self;
                /// Width of this C kind’s declared storage, before any integer promotion.
                const BITS: u32 = $bits;
                /// Whether the C kind interprets its representation as signed.
                const SIGNED: bool = $signed;
                /// C integer rank, preserving distinctions between equal-width Rust representations.
                const RANK: u8 = $rank;
                /// Encode this native integer in the shared bit carrier used by C conversion and arithmetic checks.
                fn encode(value: Self::Repr) -> u128 { value as u128 }
                /// Narrow checked result bits to this C kind’s native storage representation.
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

/// Keep C `_Bool` separate from integer kinds so conversion tests truth before any narrowing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CBool;
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for CBool {}
/// Retain C rank, width, signedness, and promotion rules independently of native Rust storage.
impl CInteger for CBool {
    /// Native bits/values used to store this C integer kind without erasing its rank.
    type Repr = bool;
    /// C kind selected by integer promotion before unary, shift, or arithmetic operations.
    type Promoted = CInt;
    /// Ordinary C identity after source-only constant or bitfield metadata is lost at a value boundary.
    type Boundary = Self;
    /// Width of the declared C storage representation before promotion.
    const BITS: u32 = 8;
    /// Whether the C kind interprets its storage bits as a signed value.
    const SIGNED: bool = false;
    /// C integer rank used by usual arithmetic conversions independently of storage width.
    const RANK: u8 = 0;
    /// Convert admitted integer or enum storage to the representation used by C conversion rules.
    fn encode(value: bool) -> u128 {
        u128::from(value)
    }
    /// Recover the declared integer or enum representation from checked conversion bits.
    fn decode(bits: u128) -> bool {
        // Conversion to _Bool tests the source value, before any narrowing.
        bits != 0
    }
}

/// Register the kinds that already satisfy C integer promotion without adding duplicate implementations.
macro_rules! promoted_kinds {
    ($($kind:ident),+ $(,)?) => { $(
        /// Register a C kind that is already its own integer-promotion result.
        impl PromotedInteger for $kind {})+ };
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
    /// Store the native integer representation while the marker retains its distinct C rank.
    repr: K::Repr,
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    kind: PhantomData<K>,
}

/// Construct and extract scalar storage while preserving its declared C rank and conversion rules.
impl<K: CInteger> CValue<K> {
    /// Tag native storage with an explicit C identity.
    pub const fn new(repr: K::Repr) -> Self {
        Self { repr, kind: PhantomData }
    }

    /// Extract native storage for a checked binding or another explicit conversion.
    pub const fn get(self) -> K::Repr {
        self.repr
    }

    /// Encode native storage for width/rank-aware C conversion and arithmetic checks.
    fn bits(self) -> u128 {
        K::encode(self.repr)
    }

    /// Decode a checked integer bit result into the declared C storage representation.
    fn from_bits(bits: u128) -> Self {
        Self::new(K::decode(bits))
    }
}

/// Keep this runtime family within crate-owned C capability registration.
impl<K: CInteger> sealed::Sealed for CValue<K> {}

/// An accepted C scalar input. This trait is sealed to the documented finite family.
pub trait IntoCValue: sealed::Sealed {
    /// C integer identity retained by the admitted native or tagged scalar input.
    type Kind: CInteger;
    /// Normalize this admitted scalar input while retaining its selected C integer identity.
    fn into_c_value(self) -> CValue<Self::Kind>;
}

/// Admit this scalar input without inferring an ambiguous C rank from Rust width.
impl<K: CInteger> IntoCValue for CValue<K> {
    /// C integer identity retained by the admitted native or tagged scalar input.
    type Kind = K;
    /// Normalize this admitted scalar input while retaining its selected C integer identity.
    fn into_c_value(self) -> Self {
        self
    }
}

/// Admit only native integer widths with an unambiguous modeled C identity.
macro_rules! raw_inputs {
    ($(($repr:ty, $kind:ident)),+ $(,)?) => {
        $(
            /// Keep this admitted scalar or type-marker family under crate-owned capability registration.
            impl sealed::Sealed for $repr {}
            /// Admit this native scalar only because the modeled C identity is unambiguous.
            impl IntoCValue for $repr {
                /// Unique modeled C integer kind associated with this admitted native input.
                type Kind = $kind;
                /// Tag an already evaluated native input with its unambiguous C integer identity.
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
    /// C result representation selected by this operand family's capability.
    type Output: PromotedInteger;
}

/// Select the C common integer identity after promotion of both operand kinds.
impl<K: PromotedInteger> Common<K> for K {
    /// C result representation selected by this operand family's capability.
    type Output = K;
}

// Only distinct unordered promoted pairs are listed; the rules are symmetric.
// In LP64, unsigned long plus signed long long becomes unsigned long long:
// the latter's higher rank cannot make its equal-width signed range sufficient.
/// Encode both operand orders of each C usual-arithmetic-conversion pair.
macro_rules! common_pairs {
    ($(($left:ident, $right:ident, $output:ident)),+ $(,)?) => {
        $(
            /// Select the C common promoted type for this operand order using rank and signedness rules.
            impl Common<$right> for $left {
                /// C result identity selected by this admitted operand family after required common conversion.
                type Output = $output; }
            /// Select the C common promoted type for this operand order using rank and signedness rules.
            impl Common<$left> for $right {
                /// C result identity selected by this admitted operand family after required common conversion.
                type Output = $output; }
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
    /// Promoted C common kind used to convert both arithmetic operands.
    type Common: PromotedInteger;
    /// Promote and convert both operands to the same C usual-arithmetic-conversion kind.
    #[doc(hidden)]
    fn common_values(self, rhs: Rhs) -> (CValue<Self::Common>, CValue<Self::Common>);
}

/// Convert admitted operands to their shared promoted C integer representation.
impl<L: IntoCValue, R: IntoCValue> ArithmeticInput<R> for L
where
    <L::Kind as CInteger>::Promoted: Common<<R::Kind as CInteger>::Promoted>,
{
    /// Promoted C common kind used to convert both arithmetic operands.
    type Common =
        <<L::Kind as CInteger>::Promoted as Common<<R::Kind as CInteger>::Promoted>>::Output;
    /// Promote and convert both operands to the same C usual-arithmetic-conversion kind.
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
    /// Whether signed arithmetic follows the inspected wrapping policy rather than rejecting overflow.
    const WRAPPING: bool;
}

/// Signed overflow is outside the accepted C domain; checked helpers panic on it.
pub struct Undefined;
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for Undefined {}
/// Select the signed arithmetic policy recorded by compiler inspection.
impl OverflowPolicy for Undefined {
    /// Whether signed arithmetic follows the inspected wrapping policy rather than rejecting overflow.
    const WRAPPING: bool = false;
}

/// Clang's `-fwrapv` signed arithmetic behavior.
pub struct Wrapping;
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for Wrapping {}
/// Select the signed arithmetic policy recorded by compiler inspection.
impl OverflowPolicy for Wrapping {
    /// Whether signed arithmetic follows the inspected wrapping policy rather than rejecting overflow.
    const WRAPPING: bool = true;
}

/// Calculate the signed value domain of the selected C integer kind before overflow validation.
fn signed_bounds<K: CInteger>() -> (i128, i128) {
    if K::BITS == 128 {
        (i128::MIN, i128::MAX)
    } else {
        let high_bit = 1_i128 << (K::BITS - 1);
        (-high_bit, high_bit - 1)
    }
}

/// Reject undefined signed overflow before encoding a valid result in the selected C representation.
fn checked_signed<K: CInteger>(result: Option<i128>) -> CValue<K> {
    let result = result.expect("C signed arithmetic overflow");
    let (min, max) = signed_bounds::<K>();
    assert!((min..=max).contains(&result), "C signed arithmetic overflow");
    CValue::from_bits(result as u128)
}

/// Share checked/wrapping integer arithmetic dispatch without replacing the inspected overflow policy.
macro_rules! arithmetic {
    ($name:ident, $wrapping:ident, $checked:ident) => {
        /// Apply integer promotion and common conversion, checking or wrapping signed results according to the selected C policy.
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

/// Apply promoted unary negation under the inspected signed-overflow policy.
pub fn neg<P: OverflowPolicy, V: IntoCValue>(input: V) -> CValue<<V::Kind as CInteger>::Promoted> {
    let input = promote(input);
    if <<V::Kind as CInteger>::Promoted as CInteger>::SIGNED && !P::WRAPPING {
        checked_signed((input.bits() as i128).checked_neg())
    } else {
        CValue::from_bits(0_u128.wrapping_sub(input.bits()))
    }
}

/// Share quotient/remainder validation so both reject the same undefined signed and zero-divisor cases.
macro_rules! division {
    ($name:ident, $operator:tt) => {
        /// Compute the common integer quotient or remainder after rejecting zero divisors and undefined signed overflow.
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

/// Implement bitwise operators after C common-type conversion instead of Rust-width inference.
macro_rules! bitwise {
    ($name:ident, $operator:tt) => {
        /// Operate on the common promoted integer bits and preserve the resulting C identity.
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

/// Invert the promoted C integer representation, preserving the promoted result identity.
pub fn bitnot<V: IntoCValue>(input: V) -> CValue<<V::Kind as CInteger>::Promoted> {
    CValue::from_bits(!promote(input).bits())
}

/// Share integer comparison lowering while retaining C common conversion and `int` result identity.
macro_rules! comparisons {
    ($name:ident, $operator:tt) => {
        /// Compare the common promoted representation with its C signedness and return a C int truth value.
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

/// Test the C scalar truth value used by lazy logical expressions and statement conditions.
pub fn truth<V: IntoCValue>(input: V) -> bool {
    input.into_c_value().bits() != 0
}

/// Return C logical negation as an `int` value rather than a Rust Boolean.
pub fn logical_not<V: IntoCValue>(input: V) -> CValue<CInt> {
    CValue::new(i32::from(!truth(input)))
}

/// Validate the promoted shift count against the promoted left operand width.
fn shift_count<K: CInteger, V: IntoCValue>(input: V) -> u32 {
    let count = promote(input);
    assert!(
        !<<V::Kind as CInteger>::Promoted as CInteger>::SIGNED || (count.bits() as i128) >= 0,
        "negative C shift count"
    );
    assert!(count.bits() < u128::from(K::BITS), "C shift count exceeds width");
    count.bits() as u32
}

/// Apply C left-shift promotion and reject counts or signed results outside the inspected policy.
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

/// Apply C right-shift promotion and the modeled compiler choice for signed right shifts.
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
    /// Retain only the evaluated left conditional arm for subsequent common-type conversion.
    Left(
        /// Carry the sole evaluated arm; the other conditional branch has no runtime value here.
        L,
    ),
    /// Retain only the evaluated right conditional arm for subsequent common-type conversion.
    Right(
        /// Carry the sole evaluated arm; the other conditional branch has no runtime value here.
        R,
    ),
}

/// Convert only the evaluated arm using the C arithmetic conditional's common type.
pub fn select<L: ArithmeticInput<R>, R: IntoCValue>(arm: Either<L, R>) -> CValue<L::Common> {
    match arm {
        Either::Left(left) => cast(left),
        Either::Right(right) => cast(right),
    }
}

/// Check integer promotion, distinct C ranks, cast truth/sign behavior, and usual arithmetic conversion.
/// Boundary cases cover overflow policies, division and shift domains; effect counters prove
/// that conditional conversion evaluates only the selected arm.
#[cfg(test)]
mod tests {
    //! Small deterministic cases exercise the finite C integer model before the
    //! expression layer adds pointers and places. Typed assertions preserve promotion
    //! and common-result identity; rejection cases cover arithmetic outside the
    //! supported C domain, and counters retain lazy conditional evaluation.

    use super::*;

    /// Verify small-integer promotions and keep equal-width `long` and `long long` as distinct C ranks.
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

    /// Check Boolean casts test the original value and signed integer widening preserves negative values.
    #[test]
    fn casts_test_truth_before_narrowing_and_sign_extend() {
        assert!(cast::<CBool, _>(256_i32).get());
        assert_eq!(cast::<CUnsignedChar, _>(-1_i32).get(), 255);
        assert_eq!(cast::<CInt128, _>(-1_i8).get(), -1);
        assert_eq!(cast::<CInt, _>(u32::MAX).get(), -1);
        assert_eq!(cast::<CUnsignedInt128, _>(-1_i32).get(), u128::MAX);
    }

    /// Prove unsigned arithmetic wraps while signed operations follow the selected undefined or wrapping policy.
    #[test]
    fn signed_and_unsigned_arithmetic_have_separate_policies() {
        assert_eq!(add::<Wrapping, _, _>(i32::MAX, 1_i32).get(), i32::MIN);
        assert_eq!(neg::<Wrapping, _>(i32::MIN).get(), i32::MIN);
        assert_eq!(add::<Undefined, _, _>(u32::MAX, 1_u32).get(), 0);
        assert!(std::panic::catch_unwind(|| add::<Undefined, _, _>(i32::MAX, 1_i32)).is_err());
        assert!(std::panic::catch_unwind(|| neg::<Undefined, _>(i128::MIN)).is_err());
    }

    /// Reject zero division and the signed minimum/-1 pair for both quotient and remainder.
    #[test]
    fn division_checks_both_quotient_and_remainder_domains() {
        assert_eq!(div(-7_i32, 3_i32).get(), -2);
        assert_eq!(rem(-7_i32, 3_i32).get(), -1);
        assert!(std::panic::catch_unwind(|| div(i32::MIN, -1_i32)).is_err());
        assert!(std::panic::catch_unwind(|| rem(i32::MIN, -1_i32)).is_err());
        assert!(std::panic::catch_unwind(|| rem(1_u32, 0_u32)).is_err());
    }

    /// Check invalid shift counts and signed left-shift domains after C integer promotion.
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

    /// Verify comparisons use C common-type conversion and return an `int` truth value.
    #[test]
    fn comparisons_return_c_int_after_common_conversion() {
        let result: CValue<CInt> = lt(-1_i32, 1_u32);
        assert_eq!(result.get(), 0);
        assert_eq!(logical_not(false).get(), 1);
        assert_eq!(bitnot(0_u8).get(), -1);
    }

    /// Count branch effects to prove common-type conversion never evaluates the unselected conditional arm.
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
