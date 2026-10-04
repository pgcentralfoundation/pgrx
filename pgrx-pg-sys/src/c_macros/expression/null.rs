//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! The source-level null constant `(void *)` applied to an integer ICE zero.
//!
//! C permits this constant to convert to function pointers; an ordinary void
//! pointer, even with null storage, has no such implicit conversion. Public and
//! native value boundaries deliberately erase this source-expression identity.

/// Reuse the surrounding C identity and conversion capabilities so this module shares the sealed runtime model.
use super::{
    AllowedProfile, CExprValue, CFunction, CInteger, CPointer, CType, CValue, CVoid, CastTo,
    CompleteObject, Equality, FunctionSignature, FunctionValue, IntoExpression, Pointer, Qualifier,
    Select, Truth, ZeroInteger, implicit, sealed,
};

/// Identify source-proved `(void *)0`, whose contextual conversions differ from an ordinary void pointer.
#[derive(Clone, Copy)]
pub struct CNullVoid;
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for CNullVoid {}
/// Expose object-size capability for this complete C storage family.
impl CompleteObject for CNullVoid {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<const B: bool> AllowedProfile<B> for CNullVoid {}

/// Retain the null-pointer-constant source fact without granting it to arbitrary null void-pointer storage.
#[derive(Clone, Copy)]
pub struct NullVoidPointer(
    /// Store the checked null void-pointer address while retaining its source-expression identity.
    Pointer<CVoid>,
);
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for NullVoidPointer {}
/// Associate an evaluated value with its exact C declaration marker.
impl CExprValue for NullVoidPointer {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CNullVoid;
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl IntoExpression for NullVoidPointer {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = Pointer<CVoid>;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self::Value {
        self.0
    }
}
/// Provide C `&*` cancellation without target access, including null addresses.
impl super::DereferenceAddress for NullVoidPointer {
    /// Object pointer or function designator produced by the C address interpretation.
    type Output = Pointer<CVoid>;
    /// Cancel C address/dereference operators without reading or invoking the designated target.
    fn dereference_address(self) -> Self::Output {
        // Cancellation evaluates the pointer as an ordinary result, so its
        // `(void *)` integer-ICE source identity no longer applies.
        self.0
    }
}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl CType for CNullVoid {
    /// Rust binding representation used for this declared C object or value.
    type Storage = *mut core::ffi::c_void;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = NullVoidPointer;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(value: Self::Storage) -> Self::Value {
        null_void(Pointer::new(value))
    }
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> Self::Storage {
        value.0.get()
    }
}

/// Retain the source identity of a proved zero integer ICE cast to unqualified void*.
///
/// The generator must establish the C source shape, not merely null storage.
/// Qualifying void, casting an already converted pointer, and evaluating a
/// runtime integer cannot establish this identity. The storage check prevents a
/// false proof from silently replacing a non-null address with a null pointer.
pub fn null_void(pointer: Pointer<CVoid>) -> NullVoidPointer {
    assert!(pointer.as_mut_address().is_null(), "C void null constant must have null storage");
    NullVoidPointer(pointer)
}

/// Expose C zero/null testing without converting the operand to a Rust Boolean value family.
impl Truth for NullVoidPointer {
    /// Test the admitted C scalar against zero/null without changing the value's type identity.
    fn truth(self) -> bool {
        false
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<M: CType, Q: Qualifier> super::ImplicitTo<CPointer<M, Q>> for NullVoidPointer {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> Pointer<M, Q> {
        Pointer::new(Q::from_mut(core::ptr::null_mut()))
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<S: FunctionSignature> super::ImplicitTo<CFunction<S>> for NullVoidPointer {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> FunctionValue<S> {
        FunctionValue::new(S::null())
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl super::ImplicitTo<super::super::CBool> for NullVoidPointer {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> CValue<super::super::CBool> {
        CValue::new(false)
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<M: CType, Q: Qualifier> CastTo<CPointer<M, Q>> for NullVoidPointer {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> Pointer<M, Q> {
        implicit::<CPointer<M, Q>, _>(self)
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<S: FunctionSignature> CastTo<CFunction<S>> for NullVoidPointer {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> FunctionValue<S> {
        implicit::<CFunction<S>, _>(self)
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<K: CInteger> CastTo<K> for NullVoidPointer {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> CValue<K> {
        <Pointer<CVoid> as CastTo<K>>::cast_to(self.0)
    }
}

/// Admit C equality for this value pair without widening pointer ordering rules.
impl<M: CType, Q: Qualifier> Equality<NullVoidPointer> for Pointer<M, Q> {
    /// Compare admitted values under C equality rules, including source-proved null constants.
    fn equal(self, _: NullVoidPointer) -> bool {
        self.as_mut_address().is_null()
    }
    /// Apply the matching C inequality rule without widening the admitted operand types.
    fn not_equal(self, _: NullVoidPointer) -> bool {
        !self.as_mut_address().is_null()
    }
}
/// Admit C equality for this value pair without widening pointer ordering rules.
impl<M: CType, Q: Qualifier> Equality<Pointer<M, Q>> for NullVoidPointer {
    /// Compare admitted values under C equality rules, including source-proved null constants.
    fn equal(self, rhs: Pointer<M, Q>) -> bool {
        rhs.as_mut_address().is_null()
    }
    /// Apply the matching C inequality rule without widening the admitted operand types.
    fn not_equal(self, rhs: Pointer<M, Q>) -> bool {
        !rhs.as_mut_address().is_null()
    }
}
/// Admit C equality for this value pair without widening pointer ordering rules.
impl<S: FunctionSignature> Equality<NullVoidPointer> for FunctionValue<S> {
    /// Compare admitted values under C equality rules, including source-proved null constants.
    fn equal(self, _: NullVoidPointer) -> bool {
        self.address().is_null()
    }
    /// Apply the matching C inequality rule without widening the admitted operand types.
    fn not_equal(self, _: NullVoidPointer) -> bool {
        !self.address().is_null()
    }
}
/// Admit C equality for this value pair without widening pointer ordering rules.
impl<S: FunctionSignature> Equality<FunctionValue<S>> for NullVoidPointer {
    /// Compare admitted values under C equality rules, including source-proved null constants.
    fn equal(self, rhs: FunctionValue<S>) -> bool {
        rhs.address().is_null()
    }
    /// Apply the matching C inequality rule without widening the admitted operand types.
    fn not_equal(self, rhs: FunctionValue<S>) -> bool {
        !rhs.address().is_null()
    }
}
/// Admit C equality for this value pair without widening pointer ordering rules.
impl Equality<Self> for NullVoidPointer {
    /// Compare admitted values under C equality rules, including source-proved null constants.
    fn equal(self, _: Self) -> bool {
        true
    }
    /// Apply the matching C inequality rule without widening the admitted operand types.
    fn not_equal(self, _: Self) -> bool {
        false
    }
}
/// Admit C equality for this value pair without widening pointer ordering rules.
impl<K: ZeroInteger> Equality<CValue<K>> for NullVoidPointer {
    /// Compare admitted values under C equality rules, including source-proved null constants.
    fn equal(self, rhs: CValue<K>) -> bool {
        K::verify_zero(rhs.get());
        true
    }
    /// Apply the matching C inequality rule without widening the admitted operand types.
    fn not_equal(self, rhs: CValue<K>) -> bool {
        K::verify_zero(rhs.get());
        false
    }
}
/// Admit C equality for this value pair without widening pointer ordering rules.
impl<K: ZeroInteger> Equality<NullVoidPointer> for CValue<K> {
    /// Compare admitted values under C equality rules, including source-proved null constants.
    fn equal(self, rhs: NullVoidPointer) -> bool {
        rhs.equal(self)
    }
    /// Apply the matching C inequality rule without widening the admitted operand types.
    fn not_equal(self, rhs: NullVoidPointer) -> bool {
        rhs.not_equal(self)
    }
}

/// Determine conditional result identity from both types and convert only the selected arm.
impl<M: CType, Q: Qualifier> Select<NullVoidPointer> for Pointer<M, Q> {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Self;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::super::Either<Self, NullVoidPointer>) -> Self {
        match arm {
            super::super::Either::Left(pointer) => pointer,
            super::super::Either::Right(null) => implicit::<CPointer<M, Q>, _>(null),
        }
    }
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl<M: CType, Q: Qualifier> Select<Pointer<M, Q>> for NullVoidPointer {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Pointer<M, Q>;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::super::Either<Self, Pointer<M, Q>>) -> Self::Output {
        match arm {
            super::super::Either::Left(null) => implicit::<CPointer<M, Q>, _>(null),
            super::super::Either::Right(pointer) => pointer,
        }
    }
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl<S: FunctionSignature> Select<NullVoidPointer> for FunctionValue<S> {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Self;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::super::Either<Self, NullVoidPointer>) -> Self {
        match arm {
            super::super::Either::Left(pointer) => pointer,
            super::super::Either::Right(null) => implicit::<CFunction<S>, _>(null),
        }
    }
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl<S: FunctionSignature> Select<FunctionValue<S>> for NullVoidPointer {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = FunctionValue<S>;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::super::Either<Self, FunctionValue<S>>) -> Self::Output {
        match arm {
            super::super::Either::Left(null) => implicit::<CFunction<S>, _>(null),
            super::super::Either::Right(pointer) => pointer,
        }
    }
}
// A conditional pointer result is an ordinary value, even if both arms are NPCs.
/// Determine conditional result identity from both types and convert only the selected arm.
impl Select<Self> for NullVoidPointer {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Pointer<CVoid>;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::super::Either<Self, Self>) -> Self::Output {
        match arm {
            super::super::Either::Left(null) | super::super::Either::Right(null) => null.0,
        }
    }
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl<K: ZeroInteger> Select<CValue<K>> for NullVoidPointer {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Pointer<CVoid>;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::super::Either<Self, CValue<K>>) -> Self::Output {
        match arm {
            super::super::Either::Left(null) => null.0,
            super::super::Either::Right(null) => implicit::<CPointer<CVoid>, _>(null),
        }
    }
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl<K: ZeroInteger> Select<NullVoidPointer> for CValue<K> {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Pointer<CVoid>;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::super::Either<Self, NullVoidPointer>) -> Self::Output {
        match arm {
            super::super::Either::Left(null) => implicit::<CPointer<CVoid>, _>(null),
            super::super::Either::Right(null) => null.0,
        }
    }
}
