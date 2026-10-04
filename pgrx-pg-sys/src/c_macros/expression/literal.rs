//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Caller literals retain integer-constant-expression identity until value storage.

use super::{
    CExprValue, CInteger, CNullConstant, CValue, FloatType, FloatValue, IntoExpression, sealed,
};
use core::marker::PhantomData;

/// A source-proved zero integer constant expression, rather than a runtime zero.
pub trait ZeroInteger: CInteger {
    #[doc(hidden)]
    fn verify_zero(value: Self::Repr) {
        assert_eq!(Self::encode(value), 0, "C null constant must have integer value zero");
    }
}
impl<K: CInteger> ZeroInteger for CNullConstant<K> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CIntegerLiteral<K: CInteger, const ZERO: bool>(PhantomData<K>);
impl<K: CInteger, const ZERO: bool> sealed::Sealed for CIntegerLiteral<K, ZERO> {}
impl<K: CInteger, const ZERO: bool> CInteger for CIntegerLiteral<K, ZERO> {
    type Repr = K::Repr;
    type Promoted = K::Promoted;
    type Boundary = K::Boundary;
    const BITS: u32 = K::BITS;
    const SIGNED: bool = K::SIGNED;
    const RANK: u8 = K::RANK;
    fn encode(value: Self::Repr) -> u128 {
        let bits = K::encode(value);
        if ZERO {
            assert_eq!(bits, 0, "proved literal zero has nonzero storage");
        }
        bits
    }
    fn decode(bits: u128) -> Self::Repr {
        if ZERO {
            assert_eq!(bits, 0, "proved literal zero has nonzero storage");
        }
        K::decode(bits)
    }
}
impl<K: CInteger> ZeroInteger for CIntegerLiteral<K, true> {}

pub trait LiteralValue<const ZERO: bool>: CExprValue {
    type Output: CExprValue;
    fn literal(self) -> Self::Output;
}
impl<K: CInteger, const ZERO: bool> LiteralValue<ZERO> for CValue<K> {
    type Output = CValue<CIntegerLiteral<K, ZERO>>;
    fn literal(self) -> Self::Output {
        CIntegerLiteral::<K, ZERO>::encode(self.get());
        CValue::new(self.get())
    }
}
impl<F: FloatType, const ZERO: bool> LiteralValue<ZERO> for FloatValue<F> {
    type Output = Self;
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
