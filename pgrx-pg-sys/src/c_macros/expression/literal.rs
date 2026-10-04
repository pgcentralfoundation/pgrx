//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Preserve source facts that ordinary native values cannot express.
//!
//! A proved integer zero literal may participate in C null-pointer conversions;
//! a runtime zero or floating zero may not. The emitter computes the source fact
//! and this module checks its storage before granting the capability. Integer
//! rank and promotion remain those of the underlying kind, while evaluated
//! input and storage boundaries erase the source-only identity.

use super::{
    CExprValue, CInteger, CNullConstant, CValue, FloatType, FloatValue, IntoExpression, sealed,
};
use core::marker::PhantomData;

/// A source-proved zero integer constant expression, rather than a runtime zero.
pub trait ZeroInteger: CInteger {
    /// Assert that source-tagged null-constant storage is actually integer zero before permitting contextual pointer conversion.
    #[doc(hidden)]
    fn verify_zero(value: Self::Repr) {
        assert_eq!(Self::encode(value), 0, "C null constant must have integer value zero");
    }
}
/// Retain the source-proved integer zero capability needed for contextual null conversion.
impl<K: CInteger> ZeroInteger for CNullConstant<K> {}

/// Retain a caller literal's source-proved zero fact until a value/storage boundary erases it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CIntegerLiteral<K: CInteger, const ZERO: bool>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<K>,
);
/// Keep this runtime family within crate-owned C capability registration.
impl<K: CInteger, const ZERO: bool> sealed::Sealed for CIntegerLiteral<K, ZERO> {}
/// Retain C rank, width, signedness, and promotion rules independently of native Rust storage.
impl<K: CInteger, const ZERO: bool> CInteger for CIntegerLiteral<K, ZERO> {
    /// Native bits/values used to store this C integer kind without erasing its rank.
    type Repr = K::Repr;
    /// C kind selected by integer promotion before unary, shift, or arithmetic operations.
    type Promoted = K::Promoted;
    /// Ordinary C identity after source-only constant or bitfield metadata is lost at a value boundary.
    type Boundary = K::Boundary;
    /// Width of the declared C storage representation before promotion.
    const BITS: u32 = K::BITS;
    /// Whether the C kind interprets its storage bits as a signed value.
    const SIGNED: bool = K::SIGNED;
    /// C integer rank used by usual arithmetic conversions independently of storage width.
    const RANK: u8 = K::RANK;
    /// Metadata and nominal enum identity cannot create an absent underlying C integer.
    const AVAILABLE: bool = K::AVAILABLE;
    /// Preserve the underlying C integer's preferred scalar alignment.
    const ALIGNMENT: usize = K::ALIGNMENT;
    /// Preserve its independently measured array alignment.
    const ARRAY_ALIGNMENT: usize = <K as CInteger>::ARRAY_ALIGNMENT;
    /// Convert admitted integer or enum storage to the representation used by C conversion rules.
    fn encode(value: Self::Repr) -> u128 {
        let bits = K::encode(value);
        if ZERO {
            assert_eq!(bits, 0, "proved literal zero has nonzero storage");
        }
        bits
    }
    /// Recover the declared integer or enum representation from checked conversion bits.
    fn decode(bits: u128) -> Self::Repr {
        if ZERO {
            assert_eq!(bits, 0, "proved literal zero has nonzero storage");
        }
        K::decode(bits)
    }
}
/// Retain the source-proved integer zero capability needed for contextual null conversion.
impl<K: CInteger> ZeroInteger for CIntegerLiteral<K, true> {}

/// Attach a source-token zero fact to integer literals without turning floating zero into a null constant.
pub trait LiteralValue<const ZERO: bool>: CExprValue {
    /// C result representation selected by this operand family's capability.
    type Output: CExprValue;
    /// Retain an integer literal's proved zero fact; floating literals keep their ordinary floating identity.
    fn literal(self) -> Self::Output;
}
/// Attach the caller literal's source fact through its existing C scalar representation.
impl<K: CInteger, const ZERO: bool> LiteralValue<ZERO> for CValue<K> {
    /// C result representation selected by this operand family's capability.
    type Output = CValue<CIntegerLiteral<K, ZERO>>;
    /// Retain an integer literal's proved zero fact; floating literals keep their ordinary floating identity.
    fn literal(self) -> Self::Output {
        CIntegerLiteral::<K, ZERO>::encode(self.get());
        CValue::new(self.get())
    }
}
/// Attach the caller literal's source fact through its existing C scalar representation.
impl<F: FloatType, const ZERO: bool> LiteralValue<ZERO> for FloatValue<F> {
    /// C result representation selected by this operand family's capability.
    type Output = Self;
    /// Retain an integer literal's proved zero fact; floating literals keep their ordinary floating identity.
    fn literal(self) -> Self {
        self
    }
}

/// Tag a literal with the compile-time zero fact calculated from its source token.
/// Floating literals remain floating values, even when numerically zero.
/// Native literal types follow Rust inference; a suffix can establish the input
/// type when an associated conditional result leaves that inference ambiguous.
pub fn value<const ZERO: bool, V: IntoExpression>(
    value: V,
) -> <V::Value as LiteralValue<ZERO>>::Output
where
    V::Value: LiteralValue<ZERO>,
{
    value.into_expression().literal()
}
