//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Evaluated public macro results retain their C identity behind one Rust type.

use super::{CInteger, CValue, IntoCValue, expression::*, sealed};

/// Native result extraction, including C void, for the sealed value families.
pub trait NativeResult: CExprValue {
    type Native;
    fn native(self) -> Self::Native;
}
impl<K: CInteger> NativeResult for CValue<K> {
    type Native = K::Repr;
    fn native(self) -> Self::Native {
        self.get()
    }
}
impl<M: CType, Q: Qualifier> NativeResult for Pointer<M, Q> {
    type Native = Q::Raw<M::Storage>;
    fn native(self) -> Self::Native {
        self.get()
    }
}
impl<F: FloatType> NativeResult for FloatValue<F> {
    type Native = F::Storage;
    fn native(self) -> Self::Native {
        self.get()
    }
}
impl<R: Copy> NativeResult for RecordValue<R> {
    type Native = R;
    fn native(self) -> Self::Native {
        self.get()
    }
}
impl<R> NativeResult for RawRecordValue<R> {
    type Native = core::mem::MaybeUninit<R>;
    fn native(self) -> Self::Native {
        self.get()
    }
}
impl<S: FunctionSignature> NativeResult for FunctionValue<S> {
    type Native = S::Pointer;
    fn native(self) -> Self::Native {
        self.get()
    }
}
impl<M: CType<Storage: Copy>, const N: usize> NativeResult for ArrayValue<M, N> {
    type Native = [M::Storage; N];
    fn native(self) -> Self::Native {
        self.get()
    }
}
impl NativeResult for () {
    type Native = ();
    fn native(self) {}
}

/// An already evaluated C expression at a public Rust macro boundary.
///
/// The concrete outer type lets Rust resolve `get` before defaulting integer
/// literals, even when the inner conditional result is an associated type.
/// This wrapper carries neither a place nor a deferred load, and makes no FFI
/// ABI promise. `into_value` preserves the exact tagged C identity.
#[derive(Clone, Copy)]
pub struct CExpression<V: CExprValue>(V);
impl<V: CExprValue> CExpression<V> {
    pub fn into_value(self) -> V {
        self.0
    }
}
impl<V: NativeResult> CExpression<V> {
    pub fn get(self) -> V::Native {
        self.0.native()
    }
}
impl<V: CExprValue> sealed::Sealed for CExpression<V> {}
impl<V: CExprValue> IntoExpression for CExpression<V> {
    type Value = V;
    fn into_expression(self) -> V {
        self.0
    }
}
impl<K: CInteger> IntoCValue for CExpression<CValue<K>> {
    type Kind = K;
    fn into_c_value(self) -> CValue<K> {
        self.0
    }
}
pub fn finish<T: IntoExpression>(value: T) -> CExpression<T::Value> {
    CExpression(value.into_expression())
}

#[cfg(test)]
mod tests {
    use super::super::{CInt, Either};
    use super::*;

    #[test]
    fn unsuffixed_conditional_inputs_keep_integer_fallback_and_c_identity() {
        let result = finish(select(if truth(gt(input(7), input(4))) {
            Either::Left(input(7))
        } else {
            Either::Right(input(4))
        }));
        assert_eq!(result.get(), 7);
        let _: CValue<CInt> = result.into_value();
        assert_eq!(finish(()).get(), ());
    }
}
