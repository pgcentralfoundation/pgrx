//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! The source-level null constant `(void *)` applied to an integer ICE zero.
//!
//! C permits this constant to convert to function pointers; an ordinary void
//! pointer, even with null storage, has no such implicit conversion. Public and
//! native value boundaries deliberately erase this source-expression identity.

use super::{
    AllowedProfile, CExprValue, CFunction, CInteger, CPointer, CType, CValue, CVoid, CastTo,
    CompleteObject, Equality, FunctionSignature, FunctionValue, IntoExpression, Pointer, Qualifier,
    Select, Truth, ZeroInteger, implicit, sealed,
};

#[derive(Clone, Copy)]
pub struct CNullVoid;
impl sealed::Sealed for CNullVoid {}
impl CompleteObject for CNullVoid {}
impl<const B: bool> AllowedProfile<B> for CNullVoid {}

#[derive(Clone, Copy)]
pub struct NullVoidPointer(Pointer<CVoid>);
impl sealed::Sealed for NullVoidPointer {}
impl CExprValue for NullVoidPointer {
    type Marker = CNullVoid;
}
impl IntoExpression for NullVoidPointer {
    type Value = Pointer<CVoid>;
    fn into_expression(self) -> Self::Value {
        self.0
    }
}
impl super::DereferenceAddress for NullVoidPointer {
    type Output = Pointer<CVoid>;
    fn dereference_address(self) -> Self::Output {
        // Cancellation evaluates the pointer as an ordinary result, so its
        // `(void *)` integer-ICE source identity no longer applies.
        self.0
    }
}
impl CType for CNullVoid {
    type Storage = *mut core::ffi::c_void;
    type Value = NullVoidPointer;
    fn from_storage(value: Self::Storage) -> Self::Value {
        null_void(Pointer::new(value))
    }
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

impl Truth for NullVoidPointer {
    fn truth(self) -> bool {
        false
    }
}
impl<M: CType, Q: Qualifier> super::ImplicitTo<CPointer<M, Q>> for NullVoidPointer {
    fn implicit_to(self) -> Pointer<M, Q> {
        Pointer::new(Q::from_mut(core::ptr::null_mut()))
    }
}
impl<S: FunctionSignature> super::ImplicitTo<CFunction<S>> for NullVoidPointer {
    fn implicit_to(self) -> FunctionValue<S> {
        FunctionValue::new(S::null())
    }
}
impl super::ImplicitTo<super::super::CBool> for NullVoidPointer {
    fn implicit_to(self) -> CValue<super::super::CBool> {
        CValue::new(false)
    }
}
impl<M: CType, Q: Qualifier> CastTo<CPointer<M, Q>> for NullVoidPointer {
    fn cast_to(self) -> Pointer<M, Q> {
        implicit::<CPointer<M, Q>, _>(self)
    }
}
impl<S: FunctionSignature> CastTo<CFunction<S>> for NullVoidPointer {
    fn cast_to(self) -> FunctionValue<S> {
        implicit::<CFunction<S>, _>(self)
    }
}
impl<K: CInteger> CastTo<K> for NullVoidPointer {
    fn cast_to(self) -> CValue<K> {
        <Pointer<CVoid> as CastTo<K>>::cast_to(self.0)
    }
}

impl<M: CType, Q: Qualifier> Equality<NullVoidPointer> for Pointer<M, Q> {
    fn equal(self, _: NullVoidPointer) -> bool {
        self.as_mut_address().is_null()
    }
    fn not_equal(self, _: NullVoidPointer) -> bool {
        !self.as_mut_address().is_null()
    }
}
impl<M: CType, Q: Qualifier> Equality<Pointer<M, Q>> for NullVoidPointer {
    fn equal(self, rhs: Pointer<M, Q>) -> bool {
        rhs.as_mut_address().is_null()
    }
    fn not_equal(self, rhs: Pointer<M, Q>) -> bool {
        !rhs.as_mut_address().is_null()
    }
}
impl<S: FunctionSignature> Equality<NullVoidPointer> for FunctionValue<S> {
    fn equal(self, _: NullVoidPointer) -> bool {
        self.address().is_null()
    }
    fn not_equal(self, _: NullVoidPointer) -> bool {
        !self.address().is_null()
    }
}
impl<S: FunctionSignature> Equality<FunctionValue<S>> for NullVoidPointer {
    fn equal(self, rhs: FunctionValue<S>) -> bool {
        rhs.address().is_null()
    }
    fn not_equal(self, rhs: FunctionValue<S>) -> bool {
        !rhs.address().is_null()
    }
}
impl Equality<Self> for NullVoidPointer {
    fn equal(self, _: Self) -> bool {
        true
    }
    fn not_equal(self, _: Self) -> bool {
        false
    }
}
impl<K: ZeroInteger> Equality<CValue<K>> for NullVoidPointer {
    fn equal(self, rhs: CValue<K>) -> bool {
        K::verify_zero(rhs.get());
        true
    }
    fn not_equal(self, rhs: CValue<K>) -> bool {
        K::verify_zero(rhs.get());
        false
    }
}
impl<K: ZeroInteger> Equality<NullVoidPointer> for CValue<K> {
    fn equal(self, rhs: NullVoidPointer) -> bool {
        rhs.equal(self)
    }
    fn not_equal(self, rhs: NullVoidPointer) -> bool {
        rhs.not_equal(self)
    }
}

impl<M: CType, Q: Qualifier> Select<NullVoidPointer> for Pointer<M, Q> {
    type Output = Self;
    fn select(arm: super::super::Either<Self, NullVoidPointer>) -> Self {
        match arm {
            super::super::Either::Left(pointer) => pointer,
            super::super::Either::Right(null) => implicit::<CPointer<M, Q>, _>(null),
        }
    }
}
impl<M: CType, Q: Qualifier> Select<Pointer<M, Q>> for NullVoidPointer {
    type Output = Pointer<M, Q>;
    fn select(arm: super::super::Either<Self, Pointer<M, Q>>) -> Self::Output {
        match arm {
            super::super::Either::Left(null) => implicit::<CPointer<M, Q>, _>(null),
            super::super::Either::Right(pointer) => pointer,
        }
    }
}
impl<S: FunctionSignature> Select<NullVoidPointer> for FunctionValue<S> {
    type Output = Self;
    fn select(arm: super::super::Either<Self, NullVoidPointer>) -> Self {
        match arm {
            super::super::Either::Left(pointer) => pointer,
            super::super::Either::Right(null) => implicit::<CFunction<S>, _>(null),
        }
    }
}
impl<S: FunctionSignature> Select<FunctionValue<S>> for NullVoidPointer {
    type Output = FunctionValue<S>;
    fn select(arm: super::super::Either<Self, FunctionValue<S>>) -> Self::Output {
        match arm {
            super::super::Either::Left(null) => implicit::<CFunction<S>, _>(null),
            super::super::Either::Right(pointer) => pointer,
        }
    }
}
// A conditional pointer result is an ordinary value, even if both arms are NPCs.
impl Select<Self> for NullVoidPointer {
    type Output = Pointer<CVoid>;
    fn select(arm: super::super::Either<Self, Self>) -> Self::Output {
        match arm {
            super::super::Either::Left(null) | super::super::Either::Right(null) => null.0,
        }
    }
}
impl<K: ZeroInteger> Select<CValue<K>> for NullVoidPointer {
    type Output = Pointer<CVoid>;
    fn select(arm: super::super::Either<Self, CValue<K>>) -> Self::Output {
        match arm {
            super::super::Either::Left(null) => null.0,
            super::super::Either::Right(null) => implicit::<CPointer<CVoid>, _>(null),
        }
    }
}
impl<K: ZeroInteger> Select<NullVoidPointer> for CValue<K> {
    type Output = Pointer<CVoid>;
    fn select(arm: super::super::Either<Self, NullVoidPointer>) -> Self::Output {
        match arm {
            super::super::Either::Left(null) => implicit::<CPointer<CVoid>, _>(null),
            super::super::Either::Right(null) => null.0,
        }
    }
}
