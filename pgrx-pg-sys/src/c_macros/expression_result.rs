//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Keep evaluated macro results usable at the public Rust call boundary.
//!
//! A common outer `CExpression` wrapper lets unsuffixed integer inputs use Rust's
//! fallback inference even when C conditional conversion selects a result kind.
//! The wrapped value keeps its C identity for further macro composition; native
//! extraction is explicit and never repeats operand evaluation. Return helpers
//! apply C assignment conversion before extraction, including pointer qualifier
//! and source-proved null-constant rules.

use super::{CInteger, CValue, IntoCValue, expression::*, sealed};

/// Native result extraction, including C void, for the sealed value families.
pub trait NativeResult: CExprValue {
    /// Native extraction type exposed by the public evaluated-result wrapper.
    type Native;
    /// Extract the family's native representation at an explicit public macro result boundary.
    fn native(self) -> Self::Native;
}
/// Extract this evaluated value family's native storage at the public Rust macro boundary.
impl<K: CInteger> NativeResult for CValue<K> {
    /// Native extraction type exposed by the public evaluated-result wrapper.
    type Native = K::Repr;
    /// Extract the family's native representation at an explicit public macro result boundary.
    fn native(self) -> Self::Native {
        self.get()
    }
}
/// Extract this evaluated value family's native storage at the public Rust macro boundary.
impl<M: CType, Q: Qualifier> NativeResult for Pointer<M, Q> {
    /// Native extraction type exposed by the public evaluated-result wrapper.
    type Native = Q::Raw<M::Storage>;
    /// Extract the family's native representation at an explicit public macro result boundary.
    fn native(self) -> Self::Native {
        self.get()
    }
}
/// Extract this evaluated value family's native storage at the public Rust macro boundary.
impl<F: FloatType> NativeResult for FloatValue<F> {
    /// Native extraction type exposed by the public evaluated-result wrapper.
    type Native = F::Storage;
    /// Extract the family's native representation at an explicit public macro result boundary.
    fn native(self) -> Self::Native {
        self.get()
    }
}
/// Extract this evaluated value family's native storage at the public Rust macro boundary.
impl<R: Copy> NativeResult for RecordValue<R> {
    /// Native extraction type exposed by the public evaluated-result wrapper.
    type Native = R;
    /// Extract the family's native representation at an explicit public macro result boundary.
    fn native(self) -> Self::Native {
        self.get()
    }
}
/// Extract this evaluated value family's native storage at the public Rust macro boundary.
impl<R> NativeResult for RawRecordValue<R> {
    /// Native extraction type exposed by the public evaluated-result wrapper.
    type Native = core::mem::MaybeUninit<R>;
    /// Extract the family's native representation at an explicit public macro result boundary.
    fn native(self) -> Self::Native {
        self.get()
    }
}
/// Extract this evaluated value family's native storage at the public Rust macro boundary.
impl<S: FunctionSignature> NativeResult for FunctionValue<S> {
    /// Native extraction type exposed by the public evaluated-result wrapper.
    type Native = S::Pointer;
    /// Extract the family's native representation at an explicit public macro result boundary.
    fn native(self) -> Self::Native {
        self.get()
    }
}
/// Extract this evaluated value family's native storage at the public Rust macro boundary.
impl<M: CType<Storage: Copy>, const N: usize> NativeResult for ArrayValue<M, N> {
    /// Native extraction type exposed by the public evaluated-result wrapper.
    type Native = [M::Storage; N];
    /// Extract the family's native representation at an explicit public macro result boundary.
    fn native(self) -> Self::Native {
        self.get()
    }
}
/// Extract this evaluated value family's native storage at the public Rust macro boundary.
impl NativeResult for () {
    /// Native extraction type exposed by the public evaluated-result wrapper.
    type Native = ();
    /// Extract the family's native representation at an explicit public macro result boundary.
    fn native(self) {}
}

/// An already evaluated C expression at a public Rust macro boundary.
///
/// The concrete outer type lets Rust resolve `get` before defaulting integer
/// literals, even when the inner conditional result is an associated type.
/// This wrapper carries neither a place nor a deferred load, and makes no FFI
/// ABI promise. `into_value` preserves the exact tagged C identity.
#[derive(Clone, Copy)]
pub struct CExpression<V: CExprValue>(
    /// Hold the already evaluated C value so public extraction never repeats operand evaluation.
    V,
);
/// Expose evaluated macro results while preserving inference and explicit native extraction.
impl<V: CExprValue> CExpression<V> {
    /// Expose the evaluated tagged C value so later operations retain its semantic identity.
    pub fn into_value(self) -> V {
        self.0
    }
}
/// Expose evaluated macro results while preserving inference and explicit native extraction.
impl<V: NativeResult> CExpression<V> {
    /// Extract the evaluated family's native storage at an explicit boundary without promising a C ABI.
    pub fn get(self) -> V::Native {
        self.0.native()
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<V: CExprValue> sealed::Sealed for CExpression<V> {}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<V: CExprValue> IntoExpression for CExpression<V> {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = V;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> V {
        self.0
    }
}
/// Admit this scalar input without inferring an ambiguous C rank from Rust width.
impl<K: CInteger> IntoCValue for CExpression<CValue<K>> {
    /// C integer identity retained by the admitted native or tagged scalar input.
    type Kind = K;
    /// Normalize this admitted scalar input while retaining its selected C integer identity.
    fn into_c_value(self) -> CValue<K> {
        self.0
    }
}
/// Wrap an evaluated result at the public macro boundary so Rust inference and explicit native extraction share one outer type.
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

/// Check public result inference and C return conversion at explicit native extraction boundaries.
/// Pointer tests retain qualification and source-proved null identity through assignment conversion.
#[cfg(test)]
mod tests {
    use super::super::{CInt, Either};
    use super::*;

    /// Check the public result wrapper supports unsuffixed integer inference while preserving tagged conditional results.
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

    /// Verify return conversion narrows integers and tests Boolean truth before native result extraction.
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

    /// Check return assignment conversion accepts qualifier addition and source-proved null constants.
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
