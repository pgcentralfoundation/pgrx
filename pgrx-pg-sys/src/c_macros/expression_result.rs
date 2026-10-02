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

/// Apply C return assignment conversion to an unambiguous native result type.
///
/// Generated return statements use Rust's enclosing result type to select its
/// C marker. The source value retains its null-constant identity until this
/// conversion; normalizing it through `IntoExpression` first would lose that
/// distinction. Equal-width native integers with ambiguous C identities use
/// [`return_value_as`] with an explicit marker instead.
pub fn return_value<R: NativeType, V: ImplicitTo<R::Marker>>(value: V) -> R {
    return_value_as::<R::Marker, _>(value)
}

/// Apply C return assignment conversion to an explicit C result marker.
///
/// This uses implicit assignment conversion, so an ordinary runtime integer
/// cannot become a pointer and pointer qualification cannot be discarded.
pub fn return_value_as<M: CType, V: ImplicitTo<M>>(value: V) -> M::Storage {
    M::into_storage(implicit::<M, _>(value))
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

    #[test]
    fn returns_apply_assignment_conversion_before_native_extraction() {
        let narrowed: u8 = return_value(CValue::<CInt>::new(257));
        assert_eq!(narrowed, 1);
        let negative: u32 = return_value(CValue::<CInt>::new(-1));
        assert_eq!(negative, u32::MAX);
        let truth: bool = return_value(CValue::<CInt>::new(256));
        assert!(truth);
        let falsehood: bool = return_value(CValue::<CInt>::new(0));
        assert!(!falsehood);
        let explicit = return_value_as::<super::super::CUnsignedLong, _>(CValue::<CInt>::new(-1));
        assert_eq!(explicit, u64::MAX);
    }

    #[test]
    fn returns_preserve_pointer_qualification_and_null_constant_identity() {
        let mut value = 7i32;
        let pointer = core::ptr::addr_of_mut!(value);
        let qualified: *const i32 = return_value(input(pointer));
        assert_eq!(qualified, pointer.cast_const());
        let null: *const i32 = return_value(null_constant(CValue::<CInt>::new(0)));
        assert!(null.is_null());
        let null_void: *mut i32 =
            return_value(null::null_void(Pointer::<CVoid>::new(core::ptr::null_mut())));
        assert!(null_void.is_null());
        let nonnull: bool = return_value(input(pointer));
        assert!(nonnull);
    }
}
