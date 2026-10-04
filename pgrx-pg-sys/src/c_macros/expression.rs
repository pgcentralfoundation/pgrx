//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Typed values and raw places used by generated C expression macros.
//!
//! A place is an address, not a Rust reference or a loaded record. Reading and
//! writing it require the caller to establish the original C access contract,
//! together with Rust's allocation, aliasing, alignment and value-validity rules.
//!
//! CType pairs compiler-facing markers with binding storage, while CExprValue
//! represents evaluated values and Place retains only a raw location plus access
//! metadata. Sealed capabilities implement C promotions, casts, short-circuit
//! consumers, pointer operations, and assignment without asking Rust primitive
//! operators to infer C semantics. The generator supplies nominal field, enum, and
//! callback registrations from the selected compiler profile and fresh bindings.
//!
//! A supported type pairing does not prove a pointer’s allocation or a native
//! function’s preconditions. Raw access, field projection, and indirect calls keep
//! explicit unsafe contracts; numeric domain checks cannot establish provenance,
//! initialization, aliasing permissions, or PostgreSQL backend-thread eligibility.

use super::{CInteger, CValue, IntoCValue, OverflowPolicy, PromotedInteger, sealed};
use core::marker::PhantomData;

/// Preserve caller literal zero facts until expression/storage boundaries remove source-only metadata.
/// This supplies contextual null conversions without mistaking runtime zero for a C constant expression.
pub mod literal;
/// Model source-proved `(void *)0` separately from ordinary void-pointer values.
/// This preserves the C conversions to object and function pointers without widening pointer capabilities.
pub mod null;
pub use literal::ZeroInteger;
/// Retain nominal C enum identities and compatible integer storage.
/// Raw enum access avoids materializing values outside a Rust binding's valid discriminants.
pub mod enumeration;
pub use enumeration::{CEnum, CEnumObject, EnumIdentity, EnumStorage};

/// A compiler-established C identity and its binding storage representation.
pub trait CType: sealed::Sealed + Copy {
    /// Rust binding representation used for this declared C object or value.
    type Storage;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value;
    /// Whether access to the declared C object requires volatile operations.
    const VOLATILE: bool = false;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(value: Self::Storage) -> Self::Value;
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> Self::Storage;
}

/// A complete C object whose Rust storage has its compiler-verified size.
/// Incomplete records and void have no such capability, even if bindgen gives
/// them a sized placeholder. Function-pointer storage is a complete object.
pub trait CompleteObject: CType {}
/// Expose object-size capability for this complete C storage family.
impl<K: CInteger> CompleteObject for K {}
/// Expose object-size capability for this complete C storage family.
impl<K: CInteger> CompleteObject for CStoredInteger<K> {}
/// Expose object-size capability for this complete C storage family.
impl<K: CInteger, R: IntegerStorage<K>> CompleteObject for CIntegerStorage<K, R> {}
/// Expose object-size capability for this complete C storage family.
impl<M: CompleteObject> CompleteObject for CVolatile<M> {}
/// Expose object-size capability for this complete C storage family.
impl<M: CType, Q: Qualifier> CompleteObject for CPointer<M, Q> {}
/// Expose object-size capability for this complete C storage family.
impl<M: CompleteObject, const N: usize> CompleteObject for CArray<M, N> {}
/// Expose object-size capability for this complete C storage family.
impl<R> CompleteObject for CRecord<R> {}
/// Expose object-size capability for this complete C storage family.
impl<S: FunctionSignature> CompleteObject for CFunction<S> {}
/// Expose object-size capability for this complete C storage family.
impl CompleteObject for CFloat {}
/// Expose object-size capability for this complete C storage family.
impl CompleteObject for CDouble {}

/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<K: CInteger> CType for K {
    /// Rust binding representation used for this declared C object or value.
    type Storage = K::Repr;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = CValue<K>;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(value: Self::Storage) -> Self::Value {
        CValue::new(value)
    }
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> Self::Storage {
        value.get()
    }
}

/// A compiler-established bit-field value before its integer promotion.
///
/// Its original base representation controls sizeof and explicit conversion.
/// Original C accessors establish the field's representable range; arithmetic
/// uses the separately proved promotion instead of assuming the base type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CBitfield<Base: CInteger, Promoted: PromotedInteger>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<(Base, Promoted)>,
);
/// Keep this runtime family within crate-owned C capability registration.
impl<Base: CInteger, Promoted: PromotedInteger> sealed::Sealed for CBitfield<Base, Promoted> {}
/// Retain C rank, width, signedness, and promotion rules independently of native Rust storage.
impl<Base: CInteger, Promoted: PromotedInteger> CInteger for CBitfield<Base, Promoted> {
    /// Native bits/values used to store this C integer kind without erasing its rank.
    type Repr = Base::Repr;
    /// C kind selected by integer promotion before unary, shift, or arithmetic operations.
    type Promoted = Promoted;
    /// Ordinary C identity after source-only constant or bitfield metadata is lost at a value boundary.
    type Boundary = Base::Boundary;
    /// Width of the declared C storage representation before promotion.
    const BITS: u32 = Base::BITS;
    /// Whether the C kind interprets its storage bits as a signed value.
    const SIGNED: bool = Base::SIGNED;
    /// C integer rank used by usual arithmetic conversions independently of storage width.
    const RANK: u8 = Base::RANK;
    /// Convert admitted integer or enum storage to the representation used by C conversion rules.
    fn encode(value: Self::Repr) -> u128 {
        Base::encode(value)
    }
    /// Recover the declared integer or enum representation from checked conversion bits.
    fn decode(bits: u128) -> Self::Repr {
        Base::decode(bits)
    }
}

/// The integer type of a compiler-proven zero integer constant expression.
/// Its integer promotion removes the null-constant tag, while sizeof retains
/// the original integer representation. Ordinary runtime integers are untagged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CNullConstant<K: CInteger>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<K>,
);
/// Keep this runtime family within crate-owned C capability registration.
impl<K: CInteger> sealed::Sealed for CNullConstant<K> {}
/// Retain C rank, width, signedness, and promotion rules independently of native Rust storage.
impl<K: CInteger> CInteger for CNullConstant<K> {
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
    /// Convert admitted integer or enum storage to the representation used by C conversion rules.
    fn encode(value: Self::Repr) -> u128 {
        let bits = K::encode(value);
        assert_eq!(bits, 0, "C null constant must have integer value zero");
        bits
    }
    /// Recover the declared integer or enum representation from checked conversion bits.
    fn decode(bits: u128) -> Self::Repr {
        assert_eq!(bits, 0, "C null constant must have integer value zero");
        K::decode(bits)
    }
}
/// Name a zero-valued integer expression whose source identity permits contextual C null conversion.
pub type NullConstant<K> = CValue<CNullConstant<K>>;

/// Preserve compiler-established null-constant identity until a conversion.
/// Generation must prove the operand is an integer constant expression; a
/// runtime zero test alone does not establish C null-constant semantics.
pub fn null_constant<K: CInteger>(value: CValue<K>) -> NullConstant<K> {
    CNullConstant::<K>::encode(value.get());
    CValue::new(value.get())
}

/// Volatility belongs to the declared object; lvalue conversion removes it.
pub struct CVolatile<M: CType>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<M>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<M: CType> Copy for CVolatile<M> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<M: CType> Clone for CVolatile<M> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<M: CType> sealed::Sealed for CVolatile<M> {}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<M: CType> CType for CVolatile<M> {
    /// Rust binding representation used for this declared C object or value.
    type Storage = M::Storage;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = M::Value;
    /// Whether access to the declared C object requires volatile operations.
    const VOLATILE: bool = true;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(value: Self::Storage) -> Self::Value {
        M::from_storage(value)
    }
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> Self::Storage {
        M::into_storage(value)
    }
}

/// One evaluated value whose C identity has been retained.
pub trait CExprValue: sealed::Sealed {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker: CType<Value = Self>;
}
/// Associate an evaluated value with its exact C declaration marker.
impl<K: CInteger> CExprValue for CValue<K> {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = K;
}

/// A tagged integer stored in Rust while retaining its underlying C identity.
pub struct CStoredInteger<K: CInteger>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<K>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<K: CInteger> Copy for CStoredInteger<K> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<K: CInteger> Clone for CStoredInteger<K> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<K: CInteger> sealed::Sealed for CStoredInteger<K> {}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<K: CInteger> CType for CStoredInteger<K> {
    /// Rust binding representation used for this declared C object or value.
    type Storage = CValue<K>;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = CValue<K::Boundary>;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(value: Self::Storage) -> Self::Value {
        CValue::new(value.get())
    }
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> Self::Storage {
        CValue::new(value.get())
    }
}

/// A verified binding integer storage representation and its C value identity.
/// Generated adapters must preserve every bit admitted by the binding type.
pub trait IntegerStorage<K: CInteger>: sealed::Sealed + Copy {
    /// Recover the declared integer or enum representation from checked conversion bits.
    fn decode(self) -> CValue<K>;
    /// Convert admitted integer or enum storage to the representation used by C conversion rules.
    fn encode(value: CValue<K>) -> Self;
}

/// Pair a binding storage wrapper with its C integer identity instead of inferring rank from Rust width.
pub struct CIntegerStorage<K: CInteger, R: IntegerStorage<K>>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<(K, R)>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<K: CInteger, R: IntegerStorage<K>> Copy for CIntegerStorage<K, R> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<K: CInteger, R: IntegerStorage<K>> Clone for CIntegerStorage<K, R> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<K: CInteger, R: IntegerStorage<K>> sealed::Sealed for CIntegerStorage<K, R> {}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<K: CInteger, R: IntegerStorage<K>> CType for CIntegerStorage<K, R> {
    /// Rust binding representation used for this declared C object or value.
    type Storage = R;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = CValue<K>;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(value: R) -> Self::Value {
        value.decode()
    }
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> R {
        R::encode(value)
    }
}

/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for usize {}
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for isize {}
/// Bridge verified 64-bit pointer-sized binding storage to its explicit C rank.
#[cfg(target_pointer_width = "64")]
macro_rules! pointer_sized_integer_storage {
    ($(($storage:ty, $kind:ty, $repr:ty)),+ $(,)?) => { $(
        /// Bridge verified pointer-sized Rust storage to this explicitly selected C integer rank.
        impl IntegerStorage<$kind> for $storage {
            /// Tag native pointer-sized storage with the explicitly selected equal-width C integer kind.
            fn decode(self) -> CValue<$kind> { CValue::new(self as $repr) }
            /// Extract the tagged integer into the verified pointer-sized storage without changing its bits.
            fn encode(value: CValue<$kind>) -> Self { value.get() as Self }
        }
    )+ };
}
#[cfg(target_pointer_width = "64")]
pointer_sized_integer_storage!(
    (usize, super::CUnsignedLong, u64),
    (usize, super::CUnsignedLongLong, u64),
    (isize, super::CLong, i64),
    (isize, super::CLongLong, i64),
);

/// A native binding storage type with an unambiguous C identity.
///
/// Generated adapters implement this for records and checked opaque bindings.
/// Equal-width C integer identities need explicit tags instead of an ambiguous
/// blanket mapping for `i64`, `u64`, `isize` or `usize`.
pub trait NativeType: sealed::Sealed + Sized {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker: CType<Storage = Self>;
}
/// Native storage for a complete, compiler/bindgen-verified C record.
/// Generated layout witnesses establish its size, alignment and field types.
/// This does not require a whole record to be initialized or `Copy`; native
/// value inputs separately require `Copy`, while raw places retain their unsafe
/// access contract.
// Registration stays within the defining crate. A public registration trait
// paired with blanket sealing would let callers bypass the native type gate.
pub(crate) trait NativeRecord: Sized {}
/// Keep this runtime family within crate-owned C capability registration.
impl<R: NativeRecord> sealed::Sealed for R {}
/// Associate admitted native storage with its C marker for macro input and place resolution.
impl<R: NativeRecord> NativeType for R {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CRecord<Self>;
}
/// Associate admitted native storage with its C marker for macro input and place resolution.
impl<K: CInteger> NativeType for CValue<K> {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CStoredInteger<K>;
}

/// Register native scalar storage only where the C identity is unambiguous.
macro_rules! native_integer_types {
    ($(($storage:ty, $kind:ty)),+ $(,)?) => { $(
        /// Associate native scalar storage with its unique admitted C type marker.
        impl NativeType for $storage {
            /// Compiler-facing C identity attached to this native Rust storage.
            type Marker = $kind; }
    )+ };
}
native_integer_types!(
    (bool, super::CBool),
    (i8, super::CSignedChar),
    (u8, super::CUnsignedChar),
    (i16, super::CShort),
    (u16, super::CUnsignedShort),
    (i32, super::CInt),
    (u32, super::CUnsignedInt),
    (i128, super::CInt128),
    (u128, super::CUnsignedInt128),
);

/// Normalize a native input while retaining its C identity.
pub trait IntoExpression: sealed::Sealed {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value: CExprValue;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self::Value;
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<R: NativeRecord + Copy> IntoExpression for R {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = RecordValue<Self>;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self::Value {
        RecordValue::new(self)
    }
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<K: CInteger> IntoExpression for CValue<K> {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = CValue<K::Boundary>;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self::Value {
        CValue::new(self.get())
    }
}
/// Share native integer input normalization without admitting ambiguous 64-bit ranks.
macro_rules! integer_inputs {
    ($($storage:ty),+ $(,)?) => { $(
        /// Normalize this native scalar into a tagged C expression without inferring ambiguous ranks.
        impl IntoExpression for $storage {
            /// Evaluated tagged C value produced from this admitted native storage.
            type Value = CValue<<Self as IntoCValue>::Kind>;
            /// Normalize this evaluated scalar using the runtime’s unique native-input C identity.
            fn into_expression(self) -> Self::Value { super::value(self) }
        }
    )+ };
}
integer_inputs!(bool, i8, u8, i16, u16, i32, u32, i128, u128);
/// Normalize an already evaluated caller operand into the sealed C expression family.
pub fn input<T: IntoExpression>(value: T) -> T::Value {
    value.into_expression()
}

/// Permit floating expressions only after the generator proves their C profile.
pub trait AllowedProfile<const FLOATS: bool>: CType {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<K: CInteger, const B: bool> AllowedProfile<B> for K {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<K: CInteger, const B: bool> AllowedProfile<B> for CStoredInteger<K> {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<K: CInteger, R: IntegerStorage<K>, const B: bool> AllowedProfile<B> for CIntegerStorage<K, R> {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<M: AllowedProfile<B>, const B: bool> AllowedProfile<B> for CVolatile<M> {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<M: CType, Q: Qualifier, const B: bool> AllowedProfile<B> for CPointer<M, Q> {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<R, const B: bool> AllowedProfile<B> for CRecord<R> {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<R, const B: bool> AllowedProfile<B> for COpaque<R> {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<M: AllowedProfile<B>, const N: usize, const B: bool> AllowedProfile<B> for CArray<M, N> {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<const B: bool> AllowedProfile<B> for CVoid {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl AllowedProfile<true> for CFloat {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl AllowedProfile<true> for CDouble {}
/// Normalize a caller operand only when its C marker is admitted by the inspected floating-point profile.
pub fn profile_input<const B: bool, T: IntoExpression>(value: T) -> T::Value
where
    <T::Value as CExprValue>::Marker: AllowedProfile<B>,
{
    input(value)
}
/// Check the profile capability of an existing C value without normalizing away source-expression metadata.
pub fn profile_value<const B: bool, V: CExprValue>(value: V) -> V
where
    V::Marker: AllowedProfile<B>,
{
    value
}

/// Preserve a verified C `__builtin_expect` value and its fixed expectation.
///
/// The emitter converts and evaluates both operands once before this call.
/// A mismatch is a cold path, including for non-Boolean `long` expectations;
/// it does not change the returned value or impose a safety precondition.
/// The emitter leaves dynamic expectations unhinted and suppresses nested
/// hints when an enclosing expectation takes precedence through groups or casts.
#[inline(always)]
pub fn expect(value: CValue<super::CLong>, expected: CValue<super::CLong>) -> CValue<super::CLong> {
    if value.get() == expected.get() {
        // Distinct equivalent returns retain the hinted branch until the
        // surrounding C conversions and consuming condition have inlined.
        expected
    } else {
        core::hint::cold_path();
        value
    }
}

/// Carry C const access at the type level so generated stores cannot gain write permission.
#[derive(Clone, Copy, Debug)]
pub struct ReadOnly;
/// Mark mutable C access while allowing projections and implicit conversions to narrow it.
#[derive(Clone, Copy, Debug)]
pub struct ReadWrite;
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for ReadOnly {}
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for ReadWrite {}

/// Pointer qualification is retained independently from its pointee identity.
pub trait Qualifier: sealed::Sealed + Copy {
    /// Combine the base and declared access without granting additional writes.
    type Narrow<Declared: Qualifier>: Qualifier;
    /// Native raw pointer shape selected by read-only versus mutable qualification.
    type Raw<T>: Copy;
    /// Normalize raw address storage for internal mechanics without granting additional write permission.
    fn into_mut<T>(pointer: Self::Raw<T>) -> *mut T;
    /// Restore the raw pointer shape required by the retained qualification marker.
    fn from_mut<T>(pointer: *mut T) -> Self::Raw<T>;
}
/// Encode raw pointer shape and directional access narrowing at the type level.
impl Qualifier for ReadOnly {
    /// Combined access qualification that cannot grant writes forbidden by either input.
    type Narrow<Declared: Qualifier> = ReadOnly;
    /// Native raw pointer shape selected by read-only versus mutable qualification.
    type Raw<T> = *const T;
    /// Normalize raw address storage for internal mechanics without granting additional write permission.
    fn into_mut<T>(pointer: *const T) -> *mut T {
        pointer.cast_mut()
    }
    /// Restore the raw pointer shape required by the retained qualification marker.
    fn from_mut<T>(pointer: *mut T) -> *const T {
        pointer.cast_const()
    }
}
/// Encode raw pointer shape and directional access narrowing at the type level.
impl Qualifier for ReadWrite {
    /// Combined access qualification that cannot grant writes forbidden by either input.
    type Narrow<Declared: Qualifier> = Declared;
    /// Native raw pointer shape selected by read-only versus mutable qualification.
    type Raw<T> = *mut T;
    /// Normalize raw address storage for internal mechanics without granting additional write permission.
    fn into_mut<T>(pointer: *mut T) -> *mut T {
        pointer
    }
    /// Restore the raw pointer shape required by the retained qualification marker.
    fn from_mut<T>(pointer: *mut T) -> *mut T {
        pointer
    }
}

/// Represent a declared C pointer type independently of an evaluated address or pointee storage.
pub struct CPointer<M: CType, Q: Qualifier = ReadWrite>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<(M, Q)>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<M: CType, Q: Qualifier> Copy for CPointer<M, Q> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<M: CType, Q: Qualifier> Clone for CPointer<M, Q> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<M: CType, Q: Qualifier> sealed::Sealed for CPointer<M, Q> {}
/// Select a read-only C pointer while retaining the pointee's nominal identity.
pub type CConstPointer<M> = CPointer<M, ReadOnly>;

/// A raw address tagged with its C pointee identity and qualification.
pub struct Pointer<M: CType, Q: Qualifier = ReadWrite> {
    /// Retain the original raw allocation address without creating a reference or proving access validity.
    raw: Q::Raw<M::Storage>,
    /// Carry declared volatility and packing through pointer/field projection without granting access rights.
    access: Access,
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    marker: PhantomData<(M, Q)>,
}
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<M: CType, Q: Qualifier> Copy for Pointer<M, Q> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<M: CType, Q: Qualifier> Clone for Pointer<M, Q> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<M: CType, Q: Qualifier> sealed::Sealed for Pointer<M, Q> {}
/// Expose tagged raw addresses and their access metadata without reading the designated object.
impl<M: CType, Q: Qualifier> Pointer<M, Q> {
    /// Tag an address without accessing it or creating a reference.
    pub const fn new(raw: Q::Raw<M::Storage>) -> Self {
        Self {
            raw,
            access: Access { volatile: M::VOLATILE, unaligned: false },
            marker: PhantomData,
        }
    }
    /// Extract the evaluated family's native storage at an explicit boundary without promising a C ABI.
    pub const fn get(self) -> Q::Raw<M::Storage> {
        self.raw
    }
    /// Expose the raw address for internal projection; the qualification marker still governs access capabilities.
    pub fn as_mut_address(self) -> *mut M::Storage {
        Q::into_mut(self.raw)
    }
    /// Retain compiler-established access properties of a projected address.
    /// This describes an access; reading or writing still requires its unsafe
    /// allocation, initialization, aliasing, and alignment contract.
    pub const fn with_access(self, access: Access) -> Self {
        Self { access, ..self }
    }
    /// Expose compiler-established packing and volatile metadata for subsequent place operations.
    pub const fn access(self) -> Access {
        self.access
    }
}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<M: CType, Q: Qualifier> CType for CPointer<M, Q> {
    /// Rust binding representation used for this declared C object or value.
    type Storage = Q::Raw<M::Storage>;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = Pointer<M, Q>;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(value: Self::Storage) -> Self::Value {
        Pointer::new(value)
    }
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> Self::Storage {
        value.get()
    }
}
/// Associate an evaluated value with its exact C declaration marker.
impl<M: CType, Q: Qualifier> CExprValue for Pointer<M, Q> {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CPointer<M, Q>;
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<M: CType, Q: Qualifier> IntoExpression for Pointer<M, Q> {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = Self;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self {
        self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<T: NativeType> sealed::Sealed for *mut T {}
/// Keep this runtime family within crate-owned C capability registration.
impl<T: NativeType> sealed::Sealed for *const T {}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<T: NativeType> IntoExpression for *mut T {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = Pointer<T::Marker>;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self::Value {
        Pointer::new(self)
    }
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<T: NativeType> IntoExpression for *const T {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = Pointer<T::Marker, ReadOnly>;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self::Value {
        Pointer::new(self)
    }
}
/// Associate admitted native storage with its C marker for macro input and place resolution.
impl<T: NativeType> NativeType for *mut T {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CPointer<T::Marker>;
}
/// Associate admitted native storage with its C marker for macro input and place resolution.
impl<T: NativeType> NativeType for *const T {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CConstPointer<T::Marker>;
}

/// An exact compiler/bindgen-verified native function-pointer signature.
/// Generated markers retain C argument/result identities even if Rust storage
/// aliases erase differences between equal-width C integer types.
pub trait FunctionSignature: sealed::Sealed + Copy {
    /// Exact nullable native function-pointer representation whose ABI is separately verified.
    type Pointer: Copy;
    /// Construct the exact binding representation of a null function pointer.
    fn null() -> Self::Pointer;
    /// Obtain the actual native function address, mapping C null to a null pointer.
    /// A guarded Rust wrapper function is not the original C ABI function address.
    fn address(pointer: Self::Pointer) -> *const ();
}

/// Shared null/address operations for one physical Rust function-pointer shape.
/// Generated local families vary over native argument/result storage, ABI and
/// arity. These operations do not call the function or inspect its value types;
/// the concrete C signature separately owns compiler and binding validation.
pub trait PhysicalFunctionPointer: sealed::Sealed {
    /// Exact nullable native function-pointer representation whose ABI is separately verified.
    type Pointer: Copy;
    /// Construct the exact nullable native callback representation without creating a callable address.
    fn null() -> Self::Pointer;
    /// Retain the object/function address interpretation required by this C operand family.
    fn address(pointer: Self::Pointer) -> *const ();
}

/// An exact compiler-owned C identity using shared physical-pointer operations.
/// Its physical family is instantiated with the original binding storage;
/// distinct C prototypes retain separate identities even when that storage is
/// equal. Family bounds verify the stored pointer's precise Rust ABI and arity.
pub trait NativeFunctionSignature: sealed::Sealed + Copy {
    /// Shared physical function-pointer family without erasing the distinct C signature identity.
    type Physical: PhysicalFunctionPointer;
}

/// Retain exact callback storage, null representation, and original native address.
impl<S: NativeFunctionSignature> FunctionSignature for S {
    /// Exact nullable native function-pointer representation whose ABI is separately verified.
    type Pointer = <S::Physical as PhysicalFunctionPointer>::Pointer;

    /// Construct the exact nullable native callback representation without creating a callable address.
    fn null() -> Self::Pointer {
        S::Physical::null()
    }

    /// Retain the object/function address interpretation required by this C operand family.
    fn address(pointer: Self::Pointer) -> *const () {
        S::Physical::address(pointer)
    }
}

/// A generated exact-signature adapter for an indirect native C call.
/// Conversion and null checks occur before entering the PostgreSQL error guard;
/// captured ABI storage must have no destructors, with result decoding after.
pub trait Call<Args>: FunctionSignature {
    /// Tagged result returned after exact callback ABI decoding.
    type Output: CExprValue;
    /// Invoke this exact callback signature through its generated argument, result, and native-guard adapter.
    ///
    /// # Safety
    /// The function pointer must be non-null and valid for its exact native C
    /// signature. Arguments must satisfy the original target's allocation,
    /// lifetime, aliasing, initialization, and callback requirements. PostgreSQL
    /// targets additionally require the permitted backend thread and its error
    /// and panic guards; a guarded Rust wrapper is not a C function address.
    unsafe fn call(pointer: Self::Pointer, args: Args) -> Self::Output;
}

/// Invoke the generated adapter without changing evaluation or guard placement.
///
/// # Safety
/// The pointer and arguments must satisfy Call::call's complete native target
/// contract, including PostgreSQL backend-thread and error-guard obligations.
pub unsafe fn invoke<S: Call<Args>, Args>(function: FunctionValue<S>, args: Args) -> S::Output {
    // SAFETY: The caller establishes the exact target and argument contract;
    // the generated sealed adapter owns conversions and native guard placement.
    unsafe { S::call(function.get(), args) }
}
/// Represent one exact C function-pointer object type for calls, storage, and pointer compatibility.
pub struct CFunction<S: FunctionSignature>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<S>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<S: FunctionSignature> Copy for CFunction<S> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<S: FunctionSignature> Clone for CFunction<S> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<S: FunctionSignature> sealed::Sealed for CFunction<S> {}
/// Carry nullable native callback storage with an exact C signature, separate from object pointers.
pub struct FunctionValue<S: FunctionSignature>(
    /// Store the exact nullable callback representation without invoking it.
    S::Pointer,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<S: FunctionSignature> Copy for FunctionValue<S> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<S: FunctionSignature> Clone for FunctionValue<S> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<S: FunctionSignature> sealed::Sealed for FunctionValue<S> {}
/// Construct and inspect exact nullable callback storage without invoking it.
impl<S: FunctionSignature> FunctionValue<S> {
    /// Tag an already supplied native representation without reading a C object or invoking a callback.
    pub const fn new(pointer: S::Pointer) -> Self {
        Self(pointer)
    }
    /// Extract the evaluated family's native storage at an explicit boundary without promising a C ABI.
    pub const fn get(self) -> S::Pointer {
        self.0
    }
    /// Retain the object/function address interpretation required by this C operand family.
    pub fn address(self) -> *const () {
        S::address(self.0)
    }
}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<S: FunctionSignature> CType for CFunction<S> {
    /// Rust binding representation used for this declared C object or value.
    type Storage = S::Pointer;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = FunctionValue<S>;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(pointer: Self::Storage) -> Self::Value {
        FunctionValue::new(pointer)
    }
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> Self::Storage {
        value.get()
    }
}
/// Associate an evaluated value with its exact C declaration marker.
impl<S: FunctionSignature> CExprValue for FunctionValue<S> {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CFunction<S>;
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<S: FunctionSignature> IntoExpression for FunctionValue<S> {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = Self;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self {
        self
    }
}
/// Enable scalar raw loads/stores through the canonical C value conversion.
impl<S: FunctionSignature> ScalarObject for CFunction<S> {
    /// Loaded scalar identity after removing object-only qualification or source metadata.
    type Canonical = Self;
}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<S: FunctionSignature, const B: bool> AllowedProfile<B> for CFunction<S> {}
/// Expose C zero/null testing without converting the operand to a Rust Boolean value family.
impl<S: FunctionSignature> Truth for FunctionValue<S> {
    /// Test the admitted C scalar against zero/null without changing the value's type identity.
    fn truth(self) -> bool {
        !self.address().is_null()
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<S: FunctionSignature> CastTo<CFunction<S>> for FunctionValue<S> {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> Self {
        self
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<S: FunctionSignature, K: CInteger> CastTo<K> for FunctionValue<S> {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> CValue<K> {
        super::cast(CValue::<super::CUnsignedLong>::new(self.address().expose_provenance() as u64))
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<S: FunctionSignature, M: CType, Q: Qualifier> CastTo<CPointer<M, Q>> for FunctionValue<S> {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> Pointer<M, Q> {
        Pointer::new(Q::from_mut(self.address().cast_mut().cast()))
    }
}

/// A fixed array retains its complete storage identity until C array decay.
pub struct CArray<M: CType, const N: usize>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<M>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<M: CType, const N: usize> Copy for CArray<M, N> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<M: CType, const N: usize> Clone for CArray<M, N> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<M: CType, const N: usize> sealed::Sealed for CArray<M, N> {}
/// Retain complete fixed-array storage until an operation requests C array-to-pointer decay.
pub struct ArrayValue<M: CType, const N: usize>(
    /// Keep complete element storage until an explicit value extraction or address-based decay.
    [M::Storage; N],
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<M: CType<Storage: Copy>, const N: usize> Copy for ArrayValue<M, N> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<M: CType<Storage: Copy>, const N: usize> Clone for ArrayValue<M, N> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<M: CType, const N: usize> sealed::Sealed for ArrayValue<M, N> {}
/// Keep complete native array storage available before explicit extraction or C pointer decay.
impl<M: CType, const N: usize> ArrayValue<M, N> {
    /// Tag an already supplied native representation without reading a C object or invoking a callback.
    pub const fn new(value: [M::Storage; N]) -> Self {
        Self(value)
    }
    /// Extract the evaluated family's native storage at an explicit boundary without promising a C ABI.
    pub fn get(self) -> [M::Storage; N] {
        self.0
    }
}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<M: CType, const N: usize> CType for CArray<M, N> {
    /// Rust binding representation used for this declared C object or value.
    type Storage = [M::Storage; N];
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = ArrayValue<M, N>;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(value: Self::Storage) -> Self::Value {
        ArrayValue::new(value)
    }
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> Self::Storage {
        value.get()
    }
}
/// Associate an evaluated value with its exact C declaration marker.
impl<M: CType<Storage: Copy>, const N: usize> CExprValue for ArrayValue<M, N> {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CArray<M, N>;
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<M: CType<Storage: Copy>, const N: usize> IntoExpression for ArrayValue<M, N> {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = Self;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self {
        self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<T: NativeType, const N: usize> sealed::Sealed for [T; N] {}
/// Associate admitted native storage with its C marker for macro input and place resolution.
impl<T: NativeType, const N: usize> NativeType for [T; N] {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CArray<T::Marker, N>;
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<T: NativeType + Copy, const N: usize> IntoExpression for [T; N] {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = ArrayValue<T::Marker, N>;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self::Value {
        ArrayValue::new(self)
    }
}

/// A record marker remains available even when no whole-record load is valid.
pub struct CRecord<R>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<R>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<R> Copy for CRecord<R> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<R> Clone for CRecord<R> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<R> sealed::Sealed for CRecord<R> {}
/// Wrap an initialized Rust record value separately from raw, possibly partial C aggregate copies.
pub struct RecordValue<R>(
    /// Hold initialized native record storage separately from raw partial-aggregate values.
    R,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<R: Copy> Copy for RecordValue<R> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<R: Copy> Clone for RecordValue<R> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<R> sealed::Sealed for RecordValue<R> {}
/// Construct and extract initialized record expressions separately from raw partial aggregate copies.
impl<R> RecordValue<R> {
    /// Tag an already supplied native representation without reading a C object or invoking a callback.
    pub const fn new(value: R) -> Self {
        Self(value)
    }
    /// Extract the evaluated family's native storage at an explicit boundary without promising a C ABI.
    pub fn get(self) -> R {
        self.0
    }
}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<R> CType for CRecord<R> {
    /// Rust binding representation used for this declared C object or value.
    type Storage = R;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = RecordValue<R>;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(value: R) -> Self::Value {
        RecordValue::new(value)
    }
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> R {
        value.get()
    }
}
/// Associate an evaluated value with its exact C declaration marker.
impl<R: Copy> CExprValue for RecordValue<R> {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CRecord<R>;
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<R: Copy> IntoExpression for RecordValue<R> {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = Self;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self {
        self
    }
}

/// Keep partial aggregate copies in raw storage until selected fields are accessed.
/// This supports C value copying without requiring the whole Rust record to be initialized or `Copy`.
pub mod record;
pub use record::{CRawRecord, RawRecordValue, RecordExpression};

/// The binding placeholder for a compiler-proven incomplete record.
/// It may occur behind a pointer, but cannot be loaded, sized, or indexed.
pub struct COpaque<R>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<R>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<R> Copy for COpaque<R> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<R> Clone for COpaque<R> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<R> sealed::Sealed for COpaque<R> {}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<R> CType for COpaque<R> {
    /// Rust binding representation used for this declared C object or value.
    type Storage = R;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = ();
    /// Reject whole-value conversion because this declared C family has no loadable complete representation.
    fn from_storage(_: R) {
        panic!("an incomplete C record cannot be loaded")
    }
    /// Reject creation of whole native storage for a C type that has no ordinary value representation.
    fn into_storage(_: ()) -> R {
        panic!("an incomplete C record has no storage value")
    }
}

/// A C flexible array member in its verified binding wrapper storage.
/// Expression conversion exposes its first element address without reading a
/// Rust zero-length placeholder or copying any partially initialized elements.
pub struct CFlexibleArray<M: CType, R>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<(M, R)>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<M: CType, R> Copy for CFlexibleArray<M, R> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<M: CType, R> Clone for CFlexibleArray<M, R> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<M: CType, R> sealed::Sealed for CFlexibleArray<M, R> {}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<M: CType, R> CType for CFlexibleArray<M, R> {
    /// Rust binding representation used for this declared C object or value.
    type Storage = R;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = ();
    /// Reject whole-value conversion because this declared C family has no loadable complete representation.
    fn from_storage(_: R) {
        panic!("a C flexible array does not have a whole storage value")
    }
    /// Reject creation of whole native storage for a C type that has no ordinary value representation.
    fn into_storage(_: ()) -> R {
        panic!("a C flexible array does not have a whole storage value")
    }
}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<M: AllowedProfile<B>, R, const B: bool> AllowedProfile<B> for CFlexibleArray<M, R> {}

/// `void` is never loaded; this byte storage exists only for void-pointer casts.
#[derive(Clone, Copy, Debug)]
pub struct CVoid;
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for CVoid {}
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for core::ffi::c_void {}
/// Associate admitted native storage with its C marker for macro input and place resolution.
impl NativeType for core::ffi::c_void {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CVoid;
}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl CType for CVoid {
    /// Rust binding representation used for this declared C object or value.
    type Storage = core::ffi::c_void;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = ();
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(_: Self::Storage) {}
    /// Reject creation of whole native storage for a C type that has no ordinary value representation.
    fn into_storage(_: ()) -> Self::Storage {
        panic!("C void has no storage value")
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for () {}
/// Associate an evaluated value with its exact C declaration marker.
impl CExprValue for () {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CVoid;
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl IntoExpression for () {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = Self;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self {}
}

/// Qualifiers and packing are properties of a C place, not its loaded value.
#[derive(Clone, Copy, Debug, Default)]
pub struct Access {
    /// Require volatile operations when the declared C object or enclosing field is volatile.
    pub volatile: bool,
    /// Select unaligned raw copies for packing-derived addresses; volatile unaligned access remains rejected.
    pub unaligned: bool,
}

/// An address with a C storage identity and access qualification.
pub struct Place<M: CType, Q: Qualifier = ReadWrite> {
    /// Retain the original raw allocation address without creating a reference or proving access validity.
    address: *mut M::Storage,
    /// Carry declared volatility and packing through pointer/field projection without granting access rights.
    access: Access,
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    marker: PhantomData<(M, Q)>,
}
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<M: CType, Q: Qualifier> Copy for Place<M, Q> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<M: CType, Q: Qualifier> Clone for Place<M, Q> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Carry a raw object address with its declared C identity, access qualification, and packing metadata.
impl<M: CType, Q: Qualifier> Place<M, Q> {
    /// Describe an address; no read, write, reference or validity check occurs.
    pub fn new(address: Q::Raw<M::Storage>, access: Access) -> Self {
        Self { address: Q::into_mut(address), access, marker: PhantomData }
    }
    /// Expose this place's tagged address without loading storage or creating a Rust reference.
    pub fn pointer(self) -> Pointer<M, Q> {
        Pointer::new(Q::from_mut(self.address)).with_access(self.access)
    }
    /// Expose compiler-established packing and volatile metadata for subsequent place operations.
    pub fn access(self) -> Access {
        self.access
    }
    /// Change access metadata established by the compiler's record layout.
    pub fn with_access(self, access: Access) -> Self {
        Self { access, ..self }
    }
}
/// Describe mutable storage with the declared C marker before an explicit unsafe read or write.
pub fn place<M: CType>(address: *mut M::Storage) -> Place<M> {
    Place::new(address, Access { volatile: M::VOLATILE, unaligned: false })
}
/// Describe read-only storage while retaining the declared C identity and volatile access requirement.
pub fn const_place<M: CType>(address: *const M::Storage) -> Place<M, ReadOnly> {
    Place::new(address, Access { volatile: M::VOLATILE, unaligned: false })
}
/// Resolve a binding storage type to its registered C marker for mutable lvalue lowering.
pub fn native_place<T: NativeType>(address: *mut T) -> Place<T::Marker> {
    place(address)
}
/// Resolve a binding storage type to its registered C marker without granting write access.
pub fn native_const_place<T: NativeType>(address: *const T) -> Place<T::Marker, ReadOnly> {
    const_place(address)
}

/// Preserve a native raw dereference's qualification during read projection.
///
/// Borrowing a raw pointer retains its pointee permissions without accessing
/// the allocation. Other Rust dereference owners retain shared read access.
/// Returned places borrow no owner: callers must keep the dereference operand
/// alive until every unsafe access and satisfy the place's access contract.
#[doc(hidden)]
pub trait NativeDerefRead {
    /// C result representation selected by this operand family's capability.
    type Output: Copy;
    /// Project the native dereference operand without moving its owner or broadening raw-pointer access.
    fn __pgrx_c_read_deref_place(self) -> Self::Output;
}

/// Preserve native dereference qualification while borrowing any owning Rust operand.
impl<T: NativeType> NativeDerefRead for &*mut T {
    /// C result representation selected by this operand family's capability.
    type Output = Place<T::Marker>;
    /// Project the native dereference operand without moving its owner or broadening raw-pointer access.
    fn __pgrx_c_read_deref_place(self) -> Self::Output {
        native_place(*self)
    }
}

/// Preserve native dereference qualification while borrowing any owning Rust operand.
impl<T: NativeType> NativeDerefRead for &*const T {
    /// C result representation selected by this operand family's capability.
    type Output = Place<T::Marker, ReadOnly>;
    /// Project the native dereference operand without moving its owner or broadening raw-pointer access.
    fn __pgrx_c_read_deref_place(self) -> Self::Output {
        native_const_place(*self)
    }
}

/// Preserve native dereference qualification while borrowing any owning Rust operand.
impl<D: core::ops::Deref<Target: NativeType> + ?Sized> NativeDerefRead for &&D {
    /// C result representation selected by this operand family's capability.
    type Output = Place<<D::Target as NativeType>::Marker, ReadOnly>;
    /// Project the native dereference operand without moving its owner or broadening raw-pointer access.
    fn __pgrx_c_read_deref_place(self) -> Self::Output {
        native_const_place(core::ptr::from_ref(core::ops::Deref::deref(*self)))
    }
}

/// Turn a tagged pointer into an address-only place while preserving access and qualification.
pub fn pointee<M: CType, Q: Qualifier>(pointer: Pointer<M, Q>) -> Place<M, Q> {
    Place::new(pointer.get(), pointer.access())
}
/// Addressable objects and function designators retain different C identities.
pub trait Address: Copy {
    /// Object pointer or function designator produced by the C address interpretation.
    type Output: CExprValue;
    /// Retain the object/function address interpretation required by this C operand family.
    fn address(self) -> Self::Output;
}
/// Keep C object address-taking distinct from function designator conversion.
impl<M: CType, Q: Qualifier> Address for Place<M, Q> {
    /// Object pointer or function designator produced by the C address interpretation.
    type Output = Pointer<M, Q>;
    /// Retain the object/function address interpretation required by this C operand family.
    fn address(self) -> Self::Output {
        self.pointer()
    }
}
/// Keep C object address-taking distinct from function designator conversion.
impl<S: FunctionSignature> Address for FunctionValue<S> {
    /// Object pointer or function designator produced by the C address interpretation.
    type Output = Self;
    /// Retain the object/function address interpretation required by this C operand family.
    fn address(self) -> Self {
        self
    }
}
/// Dispatch C address-taking for object places and function designators without loading storage.
pub fn address<P: Address>(place: P) -> P::Output {
    Address::address(place)
}

/// C dereference reads an object but only designates an indirect function.
pub trait DereferenceValue: CExprValue {
    /// C result representation selected by this operand family's capability.
    type Output: CExprValue;
    /// Apply C object-to-value conversion or retain an indirect function designator, according to this value family.
    ///
    /// # Safety
    /// Object pointers must satisfy the target place's initialized-read
    /// contract. Function pointers are merely designated, without an invocation.
    unsafe fn dereference(self) -> Self::Output;
}
/// Select object loading versus function designation without conflating C operand kinds.
impl<M: ReadObject<Q>, Q: Qualifier> DereferenceValue for Pointer<M, Q>
where
    <M::RValue as CType>::Value: CExprValue,
{
    /// C result representation selected by this operand family's capability.
    type Output = <M::RValue as CType>::Value;
    /// Apply the supported C object-read or function-designator interpretation of dereference.
    ///
    /// # Safety
    /// The pointer must satisfy the target object’s `ReadPlace::load` contract,
    /// including live readable bounds, provenance, alignment, access qualifiers,
    /// and aliasing permissions. Scalar storage must be initialized and valid.
    unsafe fn dereference(self) -> Self::Output {
        // SAFETY: The caller establishes this declared object's read contract.
        unsafe { load(pointee(self)) }
    }
}
/// Select object loading versus function designation without conflating C operand kinds.
impl<S: FunctionSignature> DereferenceValue for FunctionValue<S> {
    /// C result representation selected by this operand family's capability.
    type Output = Self;
    /// Apply the supported C object-read or function-designator interpretation of dereference.
    ///
    /// # Safety
    /// This function-designator implementation performs no storage access or
    /// native call and imposes no additional allocation or initialization requirement.
    unsafe fn dereference(self) -> Self {
        self
    }
}
/// Read an indirect object or retain a function designator's exact signature.
///
/// # Safety
/// The value must satisfy DereferenceValue::dereference's read contract.
pub unsafe fn dereference<V: DereferenceValue>(value: V) -> V::Output {
    // SAFETY: The caller establishes the specific declared object's contract.
    unsafe { value.dereference() }
}

/// Cancel C's `&*` operators without accessing the target, even for null.
pub trait DereferenceAddress: CExprValue {
    /// Object pointer or function designator produced by the C address interpretation.
    type Output: CExprValue;
    /// Cancel C address/dereference operators without reading or invoking the designated target.
    fn dereference_address(self) -> Self::Output;
}
/// Provide C `&*` cancellation without target access, including null addresses.
impl<M: CType, Q: Qualifier> DereferenceAddress for Pointer<M, Q> {
    /// Object pointer or function designator produced by the C address interpretation.
    type Output = Self;
    /// Cancel C address/dereference operators without reading or invoking the designated target.
    fn dereference_address(self) -> Self {
        self
    }
}
/// Provide C `&*` cancellation without target access, including null addresses.
impl<S: FunctionSignature> DereferenceAddress for FunctionValue<S> {
    /// Object pointer or function designator produced by the C address interpretation.
    type Output = Self;
    /// Cancel C address/dereference operators without reading or invoking the designated target.
    fn dereference_address(self) -> Self {
        self
    }
}
/// Implement the C cancellation of `&*` without reading or calling the pointed-to target.
pub fn dereference_address<V: DereferenceAddress>(value: V) -> V::Output {
    value.dereference_address()
}
/// Apply C array-to-pointer conversion without copying any array element.
pub fn decay<M: CType, const N: usize, Q: Qualifier>(
    place: Place<CArray<M, N>, Q>,
) -> Pointer<M, Q> {
    Pointer::new(Q::from_mut(place.pointer().as_mut_address().cast())).with_access(place.access())
}

/// A place whose load returns a known C value, including generated bitfield places.
///
/// # Safety
/// Implementations must read exactly the declared C object, honor its qualifiers,
/// and create no intermediate references or ownership of non-Copy storage.
pub unsafe trait ReadPlace: Copy {
    /// Declared C object kind retained for access or unevaluated object-size checks.
    type Object: CType;
    /// C expression result kind produced by a place load.
    type Type: CType;
    /// Read the declared C place using its access metadata and return its established expression type.
    ///
    /// # Safety
    /// The object must be alive with sufficient allocation provenance,
    /// alignment and aliasing permissions. Every scalar field read must be
    /// initialized and valid; raw aggregate copies may retain uninitialized
    /// fields without materializing the whole Rust record.
    unsafe fn load(self) -> <Self::Type as CType>::Value;
}

/// A scalar object whose copied storage produces its canonical C value.
/// Generated opaque-storage adapters preserve their scalar identity here.
pub trait ScalarObject: CType<Storage: Copy, Value: Copy> {
    /// Loaded scalar identity after removing object-only qualification or source metadata.
    type Canonical: CType<Value = Self::Value, Storage: Copy>;
}
/// Enable scalar raw loads/stores through the canonical C value conversion.
impl<K: CInteger> ScalarObject for K {
    /// Loaded scalar identity after removing object-only qualification or source metadata.
    type Canonical = K;
}
/// Enable scalar raw loads/stores through the canonical C value conversion.
impl<K: CInteger> ScalarObject for CStoredInteger<K> {
    /// Loaded scalar identity after removing object-only qualification or source metadata.
    type Canonical = K::Boundary;
}
/// Enable scalar raw loads/stores through the canonical C value conversion.
impl<K: CInteger, R: IntegerStorage<K>> ScalarObject for CIntegerStorage<K, R> {
    /// Loaded scalar identity after removing object-only qualification or source metadata.
    type Canonical = K;
}
/// Enable scalar raw loads/stores through the canonical C value conversion.
impl<M: CType, Q: Qualifier> ScalarObject for CPointer<M, Q> {
    /// Loaded scalar identity after removing object-only qualification or source metadata.
    type Canonical = Self;
}
/// Enable scalar raw loads/stores through the canonical C value conversion.
impl<M: ScalarObject> ScalarObject for CVolatile<M> {
    /// Loaded scalar identity after removing object-only qualification or source metadata.
    type Canonical = M::Canonical;
}

/// Load one canonical C scalar using the place metadata to choose aligned, unaligned, or volatile access.
///
/// # Safety
/// The place must designate live, initialized, valid scalar storage with readable bounds and permissions, valid aliasing, and alignment matching its access metadata.
unsafe fn read_scalar<M: ScalarObject, Q: Qualifier>(place: Place<M, Q>) -> M::Value {
    assert!(
        !(place.access.volatile && place.access.unaligned),
        "unaligned volatile C access is unsupported"
    );
    // SAFETY: The caller establishes the object's initialized access validity;
    // metadata selects the corresponding alignment and volatile operation.
    let storage = unsafe {
        if place.access.volatile {
            place.address.read_volatile()
        } else if place.access.unaligned {
            place.address.read_unaligned()
        } else {
            place.address.read()
        }
    };
    M::from_storage(storage)
}
/// Store converted scalar bytes without reading or dropping the old C object.
///
/// # Safety
/// The place must designate live writable scalar storage with sufficient bounds and aliasing permissions, and alignment matching its access metadata.
unsafe fn write_scalar<M: ScalarObject>(place: Place<M>, value: M::Value) {
    assert!(
        !(place.access.volatile && place.access.unaligned),
        "unaligned volatile C access is unsupported"
    );
    let storage = M::into_storage(value);
    // SAFETY: The caller establishes writable storage. This does not read or
    // drop any old value; metadata selects the required access operation.
    unsafe {
        if place.access.volatile {
            place.address.write_volatile(storage);
        } else if place.access.unaligned {
            place.address.write_unaligned(storage);
        } else {
            place.address.write(storage);
        }
    }
}

/// C object-to-value conversion loads a scalar, copies a raw aggregate, or decays an array.
///
/// # Safety
/// Implementations must honor the complete declared object's access contract,
/// including qualifiers, and must not copy array elements during array decay.
pub unsafe trait ReadObject<Q: Qualifier>: CType {
    /// Canonical C expression kind after object-to-value conversion.
    type RValue: CType;
    /// Perform the declared object’s scalar load, raw aggregate copy, or unevaluated array decay.
    ///
    /// # Safety
    /// Scalar loads require initialized valid storage. Raw aggregate copies may
    /// contain uninitialized fields. Array decay requires only the declared
    /// live address/allocation contract, without element reads.
    unsafe fn read_object(place: Place<Self, Q>) -> <Self::RValue as CType>::Value;
}
// SAFETY: ScalarObject establishes Copy storage and a value-preserving canonical
// conversion; read_scalar performs exactly the metadata-selected access.
/// Provide declared object-to-value conversion without introducing a whole-record validity requirement.
unsafe impl<M: ScalarObject, Q: Qualifier> ReadObject<Q> for M {
    /// Canonical C expression kind after object-to-value conversion.
    type RValue = M::Canonical;
    /// Apply C object-to-value conversion with the declared access metadata.
    ///
    /// # Safety
    /// The place must satisfy `read_scalar`: live initialized valid scalar storage
    /// with readable bounds, provenance, alignment, and aliasing permissions. Its
    /// access metadata must preserve the declared volatile and packing requirements.
    unsafe fn read_object(place: Place<Self, Q>) -> <Self::RValue as CType>::Value {
        // SAFETY: The caller establishes this scalar object's load contract.
        unsafe { read_scalar(place) }
    }
}
// SAFETY: An array's expression conversion returns its original first-element
// address. No element or aggregate storage is read.
/// Provide declared object-to-value conversion without introducing a whole-record validity requirement.
unsafe impl<M: CType, const N: usize, Q: Qualifier> ReadObject<Q> for CArray<M, N> {
    /// Canonical C expression kind after object-to-value conversion.
    type RValue = CPointer<M, Q>;
    /// Apply C object-to-value conversion with the declared access metadata.
    ///
    /// # Safety
    /// The place must satisfy the declared array’s live allocation and address
    /// contract, including provenance and inherited access permissions. Decay reads
    /// no elements, so element initialization is not required; an empty flexible
    /// array may produce its permitted one-past address.
    unsafe fn read_object(place: Place<Self, Q>) -> Pointer<M, Q> {
        decay(place)
    }
}
// SAFETY: Volatile array qualification is transferred to the pointee marker;
// decay does not itself read any volatile element.
/// Provide declared object-to-value conversion without introducing a whole-record validity requirement.
unsafe impl<M: CType, const N: usize, Q: Qualifier> ReadObject<Q> for CVolatile<CArray<M, N>> {
    /// Canonical C expression kind after object-to-value conversion.
    type RValue = CPointer<CVolatile<M>, Q>;
    /// Apply C object-to-value conversion with the declared access metadata.
    ///
    /// # Safety
    /// The place must satisfy the declared array’s live allocation and address
    /// contract, including provenance and inherited access permissions. Decay reads
    /// no elements, so element initialization is not required; an empty flexible
    /// array may produce its permitted one-past address.
    unsafe fn read_object(place: Place<Self, Q>) -> Pointer<CVolatile<M>, Q> {
        Pointer::new(Q::from_mut(place.address.cast()))
            .with_access(Access { volatile: true, ..place.access() })
    }
}

// SAFETY: Generation proves the wrapper starts at the declared flexible-array
// address with the element's alignment. Only that address is converted; no
// object representation or element is read.
/// Provide declared object-to-value conversion without introducing a whole-record validity requirement.
unsafe impl<M: CType, R, Q: Qualifier> ReadObject<Q> for CFlexibleArray<M, R> {
    /// Canonical C expression kind after object-to-value conversion.
    type RValue = CPointer<M, Q>;
    /// Apply C object-to-value conversion with the declared access metadata.
    ///
    /// # Safety
    /// The place must satisfy the declared array’s live allocation and address
    /// contract, including provenance and inherited access permissions. Decay reads
    /// no elements, so element initialization is not required; an empty flexible
    /// array may produce its permitted one-past address.
    unsafe fn read_object(place: Place<Self, Q>) -> Pointer<M, Q> {
        Pointer::new(Q::from_mut(place.address.cast())).with_access(place.access())
    }
}
// SAFETY: As above; volatile array decay transfers the qualifier without an
// element read or any access through a Rust reference.
/// Provide declared object-to-value conversion without introducing a whole-record validity requirement.
unsafe impl<M: CType, R, Q: Qualifier> ReadObject<Q> for CVolatile<CFlexibleArray<M, R>> {
    /// Canonical C expression kind after object-to-value conversion.
    type RValue = CPointer<CVolatile<M>, Q>;
    /// Apply C object-to-value conversion with the declared access metadata.
    ///
    /// # Safety
    /// The place must satisfy the declared array’s live allocation and address
    /// contract, including provenance and inherited access permissions. Decay reads
    /// no elements, so element initialization is not required; an empty flexible
    /// array may produce its permitted one-past address.
    unsafe fn read_object(place: Place<Self, Q>) -> Pointer<CVolatile<M>, Q> {
        Pointer::new(Q::from_mut(place.address.cast()))
            .with_access(Access { volatile: true, ..place.access() })
    }
}

// SAFETY: ReadObject owns exact object-to-value conversion and its access contract.
/// Expose the declared C load capability through its exact object conversion contract.
unsafe impl<M: ReadObject<Q>, Q: Qualifier> ReadPlace for Place<M, Q> {
    /// Declared C object kind retained for access or unevaluated object-size checks.
    type Object = M;
    /// C expression result kind produced by a place load.
    type Type = M::RValue;
    /// Perform the declared place read and retain its C expression result identity.
    ///
    /// # Safety
    /// The place must satisfy `M::read_object` for its declared object and access
    /// metadata. Scalar reads require initialized valid storage; raw aggregate copies
    /// may retain uninitialized fields, and array decay performs no element reads.
    /// Live bounds, provenance, alignment, aliasing, and concurrency rules still apply.
    unsafe fn load(self) -> <Self::Type as CType>::Value {
        // SAFETY: The caller establishes this declared object's access contract.
        unsafe { M::read_object(self) }
    }
}

/// A readable place whose declared C object may be modified.
///
/// # Safety
/// Stores must honor the declared object representation and access qualifiers,
/// and must not read or drop the previous value for a plain assignment.
pub unsafe trait WritePlace: ReadPlace {
    /// C destination kind used for assignment conversion before storing the object.
    type Assignment: CType;
    /// Write the converted C destination value once and return the stored expression result.
    ///
    /// # Safety
    /// The object must be alive and writable with sufficient provenance,
    /// alignment and aliasing permissions; existing references may not forbid it.
    /// Raw aggregate/enum stores may leave bytes invalid for the Rust binding
    /// type. The caller must prevent typed Rust reads or references until that
    /// type's full initialization and validity are restored.
    unsafe fn store(
        self,
        value: <Self::Assignment as CType>::Value,
    ) -> <Self::Type as CType>::Value;
}
// SAFETY: Write capability exists only for ReadWrite places. The operation writes
// Copy storage directly and never reads/drops the previous object.
/// Expose assignment only for this admitted writable destination family.
unsafe impl<M: ScalarObject> WritePlace for Place<M> {
    /// C destination kind used for assignment conversion before storing the object.
    type Assignment = M::Canonical;
    /// Write the converted destination value without reading or dropping the previous object.
    ///
    /// # Safety
    /// The place must satisfy `write_scalar` for live writable scalar storage,
    /// including bounds, provenance, alignment, aliasing, concurrency, and truthful
    /// access metadata. The previous contents need not be initialized or read.
    unsafe fn store(
        self,
        value: <Self::Assignment as CType>::Value,
    ) -> <Self::Type as CType>::Value {
        // SAFETY: The caller establishes writable scalar storage; the canonical
        // C value preserves this object's storage conversion.
        unsafe { write_scalar(self, value) }
        value
    }
}

/// Ordinary object places have a C size; bit-field places intentionally do not.
pub trait SizeablePlace: Copy {
    /// Declared C object kind retained for access or unevaluated object-size checks.
    type Object: CompleteObject;
}
/// Retain complete-object size for unevaluated places while excluding bitfields.
impl<M: CompleteObject, Q: Qualifier> SizeablePlace for Place<M, Q> {
    /// Declared C object kind retained for access or unevaluated object-size checks.
    type Object = M;
}

/// A field offset verified against the compiler's layout and actual Rust bindings.
/// Unlike a field projection, this capability never accesses object storage.
pub trait OffsetField<F>: CType {
    /// A verified nested record marker, or `()` when the path cannot continue.
    /// Leaf fields need no supported value representation to have an offset.
    type Member;
    /// Compiler/layout-established byte offset used without loading object storage.
    const OFFSET: usize;
}

/// Retain compiler-described member offsets without accessing record storage.
impl<F, M: OffsetField<F>> OffsetField<F> for CVolatile<M> {
    /// Declared field kind or nested path marker supplied by the compiler-owned layout description.
    type Member = M::Member;
    /// Compiler/layout-established byte offset used without loading object storage.
    const OFFSET: usize = <M as OffsetField<F>>::OFFSET;
}

/// End of a compiler-verified field path.
pub struct OffsetEnd;

/// One field followed by the remaining field path.
pub struct OffsetStep<F, Tail>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<(F, Tail)>,
);

/// Compose field offsets without constructing an address or evaluating an operand.
pub trait OffsetPath<P> {
    /// Compiler/layout-established byte offset used without loading object storage.
    const OFFSET: usize;
    /// Number of fields in the bounded offset path, checked before accumulating offsets.
    const DEPTH: usize;
}

/// Compose a bounded path of verified byte offsets without evaluating a C operand.
impl<M> OffsetPath<OffsetEnd> for M {
    /// Compiler/layout-established byte offset used without loading object storage.
    const OFFSET: usize = 0;
    /// Number of fields in the bounded offset path, checked before accumulating offsets.
    const DEPTH: usize = 0;
}

/// Compose a bounded path of verified byte offsets without evaluating a C operand.
impl<M: OffsetField<F>, F, Tail> OffsetPath<OffsetStep<F, Tail>> for M
where
    M::Member: OffsetPath<Tail>,
{
    /// Number of fields in the bounded offset path, checked before accumulating offsets.
    const DEPTH: usize = match <M::Member as OffsetPath<Tail>>::DEPTH.checked_add(1) {
        Some(depth) => depth,
        None => panic!("C offsetof path exceeds the 64-field bound"),
    };
    /// Compiler/layout-established byte offset used without loading object storage.
    const OFFSET: usize = {
        assert!(
            <Self as OffsetPath<OffsetStep<F, Tail>>>::DEPTH <= 64,
            "C offsetof path exceeds the 64-field bound"
        );
        match <M as OffsetField<F>>::OFFSET.checked_add(<M::Member as OffsetPath<Tail>>::OFFSET) {
            Some(offset) => offset,
            None => panic!("C offsetof path offset exceeds usize"),
        }
    };
}

/// Return an offset with the selected profile's verified LP64 C `size_t` identity.
pub const fn offset_of<M: OffsetPath<P>, P>() -> CValue<super::CUnsignedLong> {
    CValue::new(M::OFFSET as u64)
}

/// Add an inherited volatile qualification to a projected object's type.
pub trait VolatilePlace: Copy {
    /// Projected representation with inherited volatile access retained.
    type Qualified: Copy;
    /// Transfer inherited volatile access into the projected object's type and metadata.
    fn qualify_volatile(self) -> Self::Qualified;
}
/// Transfer containing-object volatility to the projected member representation.
impl<M: CType, Q: Qualifier> VolatilePlace for Place<M, Q> {
    /// Projected representation with inherited volatile access retained.
    type Qualified = Place<CVolatile<M>, Q>;
    /// Transfer inherited volatile access into the projected object's type and metadata.
    fn qualify_volatile(self) -> Self::Qualified {
        Place::new(Q::from_mut(self.address), Access { volatile: true, ..self.access })
    }
}

/// Project one compiler-established field without loading its containing record.
///
/// # Safety
/// Implementations must preserve field type, offset, qualification and packing.
/// They may narrow write qualification but must never broaden it.
pub unsafe trait Field<F, Q: Qualifier>: CType {
    /// Qualified field place produced without loading the containing record.
    type Output: Copy;
    /// Designate the compiler-established member without loading or requiring initialization of its containing record.
    ///
    /// # Safety
    /// The base must permit an in-bounds projection to the declared field; only
    /// the accessed field needs initialization when the returned place is loaded.
    unsafe fn project(place: Place<Self, Q>) -> Self::Output;
}

/// Function shape for uncalled witnesses of exact native field storage.
/// Runtime field projection never executes these compile-time type checks.
///
/// # Safety
/// Calling a function requires a base pointer into a live containing
/// allocation that permits the field address in bounds, or the permitted
/// one-past address of a flexible-array field. It forms no reference and reads
/// no storage, so the containing record need not be initialized. Its raw result
/// may be unaligned and grants no additional read or write permissions.
pub type FieldProjection<Owner, Storage> = unsafe fn(*mut Owner) -> *mut Storage;

/// Exact metadata for an ordinary field of a compiler-established C record.
///
/// # Safety
/// `OFFSET` must designate the field's actual storage within the containing
/// record, with `Member::Storage` matching its full C/Rust representation.
/// Implementations must prove each promoted field's type and layout, including
/// transparent storage wrappers. `ACCESS` must retain packing and volatile
/// requirements; `ALIGNMENT` must be the nonzero declared member alignment.
/// The defaults require the record and member storage alignments to equal their
/// compiler-established C alignments, and `Member::VOLATILE` to include every
/// enclosing volatile member. These follow from the generated layout and type
/// witnesses; different storage representations must override the constants.
/// `Declared` must be ReadOnly when any enclosing member is const, otherwise
/// ReadWrite. Its composition with the containing place must never broaden
/// write access.
pub unsafe trait OrdinaryField<F>: CType {
    /// Declared field kind or nested path marker supplied by the compiler-owned layout description.
    type Member: CType;
    /// Member constness used to narrow the containing place's write permission.
    type Declared: Qualifier;
    /// Compiler/layout-established byte offset used without loading object storage.
    const OFFSET: usize;
    /// Declared member alignment needed to select aligned versus packed access.
    const ALIGNMENT: usize = core::mem::align_of::<<Self::Member as CType>::Storage>();
    /// Packing and volatility facts retained by an ordinary field projection.
    const ACCESS: Access = Access {
        volatile: <Self::Member as CType>::VOLATILE,
        unaligned: core::mem::align_of::<Self::Storage>() < Self::ALIGNMENT
            || !Self::OFFSET.is_multiple_of(Self::ALIGNMENT),
    };
}

// SAFETY: OrdinaryField proves the exact typed field representation and layout.
// This projection keeps the caller's allocation provenance and access metadata,
// narrows qualification as established by that proof, and reads no storage.
/// Project exact typed fields while retaining the caller's allocation and access obligations.
unsafe impl<R, F, Q: Qualifier> Field<F, Q> for CRecord<R>
where
    Self: OrdinaryField<F>,
{
    /// Qualified field place produced without loading the containing record.
    type Output =
        Place<<Self as OrdinaryField<F>>::Member, Q::Narrow<<Self as OrdinaryField<F>>::Declared>>;
    /// Resolve the declared field address while retaining allocation provenance, packing, and access qualification.
    ///
    /// # Safety
    /// The base must have live containing-allocation provenance and permit the
    /// declared member address in bounds, or the permitted one-past address of a
    /// flexible-array member. Projection reads no storage and requires no whole-record
    /// initialization. The result grants no additional read or write permissions.
    unsafe fn project(base: Place<Self, Q>) -> Self::Output {
        // SAFETY: The caller provides the containing allocation and permits this
        // in-bounds (or flexible-array one-past) field projection. OrdinaryField
        // proves its byte offset and exact storage type. No reference is formed.
        let address = unsafe {
            base.pointer()
                .as_mut_address()
                .cast::<u8>()
                .add(<Self as OrdinaryField<F>>::OFFSET)
                .cast()
        };
        Place::new(
            Q::Narrow::<<Self as OrdinaryField<F>>::Declared>::from_mut(address),
            Access {
                volatile: base.access().volatile || <Self as OrdinaryField<F>>::ACCESS.volatile,
                unaligned: (<Self as OrdinaryField<F>>::ALIGNMENT > 1 && base.access().unaligned)
                    || <Self as OrdinaryField<F>>::ACCESS.unaligned,
            },
        )
    }
}
// SAFETY: Retagging the base preserves storage and access metadata. The underlying
// Field contract requires its projection to preserve inherited volatile access.
/// Project exact typed fields while retaining the caller's allocation and access obligations.
unsafe impl<F, M: Field<F, Q>, Q: Qualifier> Field<F, Q> for CVolatile<M>
where
    M::Output: VolatilePlace,
{
    /// Qualified field place produced without loading the containing record.
    type Output = <M::Output as VolatilePlace>::Qualified;
    /// Resolve the declared field address while retaining allocation provenance, packing, and access qualification.
    ///
    /// # Safety
    /// The base must have live containing-allocation provenance and permit the
    /// declared member address in bounds, or the permitted one-past address of a
    /// flexible-array member. Projection reads no storage and requires no whole-record
    /// initialization. The result grants no additional read or write permissions.
    unsafe fn project(base: Place<Self, Q>) -> Self::Output {
        let mut access = base.access();
        access.volatile = true;
        let base = Place::<M, Q>::new(Q::from_mut(base.address), access);
        // SAFETY: The caller's base allocation contract is unchanged by retagging.
        unsafe { M::project(base) }.qualify_volatile()
    }
}

/// Designate the compiler-established member without loading or requiring initialization of its containing record.
///
/// # Safety
/// The base address must satisfy the field projection's allocation contract.
pub unsafe fn project<F, M: Field<F, Q>, Q: Qualifier>(place: Place<M, Q>) -> M::Output {
    // SAFETY: The caller establishes the base projection contract; the generated
    // Field implementation establishes its exact layout and access qualifiers.
    unsafe { M::project(place) }
}

/// Read the selected field from a copied C record's raw storage.
///
/// # Safety
/// The field's declared C access must be valid, including a union's active member
/// and the valid-value requirements of any field storage representation.
#[allow(clippy::type_complexity)] // Preserve the exact rvalue produced by the generated field place.
pub unsafe fn member<F, V: RecordExpression>(
    record: V,
) -> <<<CRecord<V::Record> as Field<F, ReadOnly>>::Output as ReadPlace>::Type as CType>::Value
where
    CRecord<V::Record>: Field<F, ReadOnly> + CType<Storage = V::Record>,
    <CRecord<V::Record> as Field<F, ReadOnly>>::Output: ReadPlace,
    <<CRecord<V::Record> as Field<F, ReadOnly>>::Output as ReadPlace>::Object:
        record::OwnedMemberObject,
{
    let storage = record.raw_storage();
    // SAFETY: The local raw record remains alive during address-only projection.
    // Only the selected field is read; its validity is established by the caller.
    unsafe {
        load(project::<F, CRecord<V::Record>, ReadOnly>(const_place::<CRecord<V::Record>>(
            storage.as_ptr(),
        )))
    }
}

/// Read the declared C place using its access metadata and return its established expression type.
///
/// # Safety
/// The place must satisfy the initialized read contract of ReadPlace::load.
pub unsafe fn load<P: ReadPlace>(place: P) -> <P::Type as CType>::Value {
    // SAFETY: The caller establishes this place's complete load contract.
    unsafe { place.load() }
}

/// An explicit C conversion whose target identity is established by analysis.
pub trait CastTo<M: CType>: CExprValue {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> M::Value;
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<M: CType, V: CastTo<M>> CastTo<CVolatile<M>> for V {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> M::Value {
        <V as CastTo<M>>::cast_to(self)
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<K: CInteger, L: CInteger> CastTo<K> for CValue<L> {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> CValue<K> {
        super::cast(self)
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<K: CInteger, L: CInteger> CastTo<CStoredInteger<K>> for CValue<L> {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> CValue<K::Boundary> {
        super::cast(self)
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<K: CInteger, R: IntegerStorage<K>, V: CastTo<K>> CastTo<CIntegerStorage<K, R>> for V {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> CValue<K> {
        <V as CastTo<K>>::cast_to(self)
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<V: CExprValue> CastTo<CVoid> for V {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) {
        let _ = self;
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<M: CType, Q: Qualifier, N: CType, R: Qualifier> CastTo<CPointer<N, R>> for Pointer<M, Q> {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> Pointer<N, R> {
        Pointer::new(R::from_mut(self.as_mut_address().cast()))
            .with_access(Access { volatile: N::VOLATILE, ..self.access() })
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<K: CInteger, M: CType, Q: Qualifier> CastTo<K> for Pointer<M, Q> {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> CValue<K> {
        super::cast(CValue::<super::CUnsignedLong>::new(
            self.as_mut_address().expose_provenance() as u64
        ))
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<K: CInteger, M: CType, Q: Qualifier> CastTo<CPointer<M, Q>> for CValue<K> {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> Pointer<M, Q> {
        let address = super::cast::<super::CUnsignedLong, _>(self).get() as usize;
        Pointer::new(Q::from_mut(core::ptr::with_exposed_provenance_mut(address)))
    }
}
/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<K: ZeroInteger, S: FunctionSignature> CastTo<CFunction<S>> for CValue<K> {
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> FunctionValue<S> {
        K::verify_zero(self.get());
        FunctionValue::new(S::null())
    }
}
/// Dispatch an explicit C cast through the admitted source/destination capabilities instead of Rust cast rules.
pub fn cast<M: CType, V: CastTo<M>>(value: V) -> M::Value {
    value.cast_to()
}

/// Retain a verified binding's name without losing its distinct C type identity.
/// Rust aliases can share storage while differing in C integer rank or qualifiers.
pub fn cast_as<S, M: CType<Storage = S>, V: CastTo<M>>(value: V) -> M::Value {
    cast::<M, V>(value)
}

/// A C assignment/prototype conversion, excluding conversions needing a cast.
pub trait ImplicitTo<M: CType>: CExprValue {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> M::Value;
}
/// Apply C assignment conversion, including its stricter pointer qualification and null-constant rules.
pub fn implicit<M: CType, V: ImplicitTo<M>>(value: V) -> M::Value {
    value.implicit_to()
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<K: CInteger, L: CInteger> ImplicitTo<K> for CValue<L> {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> CValue<K> {
        cast::<K, _>(self)
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<M: CType, V: ImplicitTo<M>> ImplicitTo<CVolatile<M>> for V {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> M::Value {
        <V as ImplicitTo<M>>::implicit_to(self)
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<K: CInteger, V: ImplicitTo<K::Boundary>> ImplicitTo<CStoredInteger<K>> for V {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> CValue<K::Boundary> {
        <V as ImplicitTo<K::Boundary>>::implicit_to(self)
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<K: CInteger, R: IntegerStorage<K>, V: ImplicitTo<K>> ImplicitTo<CIntegerStorage<K, R>> for V {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> CValue<K> {
        <V as ImplicitTo<K>>::implicit_to(self)
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<R: Copy> ImplicitTo<CRecord<R>> for RecordValue<R> {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> Self {
        self
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<S: FunctionSignature> ImplicitTo<CFunction<S>> for FunctionValue<S> {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> Self {
        self
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<K: ZeroInteger, M: CType, Q: Qualifier> ImplicitTo<CPointer<M, Q>> for CValue<K> {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> Pointer<M, Q> {
        K::verify_zero(self.get());
        Pointer::new(Q::from_mut(core::ptr::null_mut()))
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<K: ZeroInteger, S: FunctionSignature> ImplicitTo<CFunction<S>> for CValue<K> {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> FunctionValue<S> {
        cast::<CFunction<S>, _>(self)
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<M: CType, Q: Qualifier> ImplicitTo<super::CBool> for Pointer<M, Q> {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> CValue<super::CBool> {
        CValue::new(!self.as_mut_address().is_null())
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<S: FunctionSignature> ImplicitTo<super::CBool> for FunctionValue<S> {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> CValue<super::CBool> {
        CValue::new(!self.address().is_null())
    }
}

/// Type identities used only to check C pointer compatibility.
pub struct PointerIdentity<I, V, Q>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<(I, V, Q)>,
);
/// Retain fixed array bounds and element qualification when checking C pointee compatibility.
pub struct ArrayIdentity<I, V, const N: usize>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<(I, V)>,
);
/// Represent unknown array bounds without erasing the element identity or qualifiers.
pub struct IncompleteArrayIdentity<I, V>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<(I, V)>,
);
/// Mark an unqualified pointee in the directional C qualifier-conversion tables.
pub struct NonVolatile;
/// Mark volatile pointee access so implicit conversions cannot discard that requirement.
pub struct Volatile;
/// A non-void pointee storage object that can be erased through C void pointers.
/// CFunction stores a function-pointer object; FunctionValue itself denotes the
/// function address and deliberately has no implicit void-pointer conversion.
pub trait NonVoidIdentity {}
/// Keep object identity distinct from `void` in pointer compatibility and ordering.
impl<K: CInteger> NonVoidIdentity for K {}
/// Keep object identity distinct from `void` in pointer compatibility and ordering.
impl<R> NonVoidIdentity for CRecord<R> {}
/// Keep object identity distinct from `void` in pointer compatibility and ordering.
impl<R> NonVoidIdentity for COpaque<R> {}
/// Keep object identity distinct from `void` in pointer compatibility and ordering.
impl NonVoidIdentity for CFloat {}
/// Keep object identity distinct from `void` in pointer compatibility and ordering.
impl NonVoidIdentity for CDouble {}
/// Keep object identity distinct from `void` in pointer compatibility and ordering.
impl<S: FunctionSignature> NonVoidIdentity for CFunction<S> {}
/// Keep object identity distinct from `void` in pointer compatibility and ordering.
impl<I, V, Q> NonVoidIdentity for PointerIdentity<I, V, Q> {}
/// Keep object identity distinct from `void` in pointer compatibility and ordering.
impl<I, V, const N: usize> NonVoidIdentity for ArrayIdentity<I, V, N> {}
/// Keep object identity distinct from `void` in pointer compatibility and ordering.
impl<I, V> NonVoidIdentity for IncompleteArrayIdentity<I, V> {}
/// Permit only the modeled C pointee relationships, rather than accepting equal Rust layouts.
pub trait CompatibleIdentity<Target> {}
/// Register only this modeled C pointee relationship rather than arbitrary layout equality.
impl<I> CompatibleIdentity<I> for I {}
/// Register only this modeled C pointee relationship rather than arbitrary layout equality.
impl<I: NonVoidIdentity> CompatibleIdentity<CVoid> for I {}
/// Register only this modeled C pointee relationship rather than arbitrary layout equality.
impl<I: NonVoidIdentity> CompatibleIdentity<I> for CVoid {}
/// Register only this modeled C pointee relationship rather than arbitrary layout equality.
impl<I, V, const N: usize> CompatibleIdentity<IncompleteArrayIdentity<I, V>>
    for ArrayIdentity<I, V, N>
{
}
/// Register only this modeled C pointee relationship rather than arbitrary layout equality.
impl<I, V, const N: usize> CompatibleIdentity<ArrayIdentity<I, V, N>>
    for IncompleteArrayIdentity<I, V>
{
}
/// Encode the directional implicit conversion that may add volatility but cannot remove it.
pub trait AddVolatile<Target> {}
/// Permit qualification addition without admitting an implicit volatility-removing conversion.
impl AddVolatile<NonVolatile> for NonVolatile {}
/// Permit qualification addition without admitting an implicit volatility-removing conversion.
impl AddVolatile<Volatile> for NonVolatile {}
/// Permit qualification addition without admitting an implicit volatility-removing conversion.
impl AddVolatile<Volatile> for Volatile {}
/// Encode the directional implicit conversion that may add const access but cannot grant writes.
pub trait AddConst<Target: Qualifier>: Qualifier {}
/// Permit const addition or preservation without granting implicit write access.
impl AddConst<ReadWrite> for ReadWrite {}
/// Permit const addition or preservation without granting implicit write access.
impl AddConst<ReadOnly> for ReadWrite {}
/// Permit const addition or preservation without granting implicit write access.
impl AddConst<ReadOnly> for ReadOnly {}
/// Separate nominal C pointee identity from declared volatility for compatibility checks.
pub trait PointeeIdentity: CType {
    /// Nominal pointee identity used independently of access qualification.
    type Identity;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<K: CInteger> PointeeIdentity for K {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = K;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<K: CInteger> PointeeIdentity for CStoredInteger<K> {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = K::Boundary;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<K: CInteger, R: IntegerStorage<K>> PointeeIdentity for CIntegerStorage<K, R> {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = K;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<R> PointeeIdentity for CRecord<R> {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = Self;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<R> PointeeIdentity for COpaque<R> {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = Self;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl PointeeIdentity for CVoid {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = Self;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl PointeeIdentity for CFloat {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = Self;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl PointeeIdentity for CDouble {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = Self;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<S: FunctionSignature> PointeeIdentity for CFunction<S> {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = Self;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<M: PointeeIdentity> PointeeIdentity for CVolatile<M> {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = M::Identity;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = Volatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<M: PointeeIdentity, Q: Qualifier> PointeeIdentity for CPointer<M, Q> {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = PointerIdentity<M::Identity, M::Volatility, Q>;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<M: PointeeIdentity, const N: usize> PointeeIdentity for CArray<M, N> {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = ArrayIdentity<M::Identity, M::Volatility, N>;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<M: PointeeIdentity, R> PointeeIdentity for CFlexibleArray<M, R> {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = IncompleteArrayIdentity<M::Identity, M::Volatility>;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<M: PointeeIdentity, Q: AddConst<R>, N: PointeeIdentity, R: Qualifier>
    ImplicitTo<CPointer<N, R>> for Pointer<M, Q>
where
    M::Identity: CompatibleIdentity<N::Identity>,
    M::Volatility: AddVolatile<N::Volatility>,
{
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> Pointer<N, R> {
        cast::<CPointer<N, R>, _>(self)
    }
}

/// Assign once and return the destination C type after its assignment conversion.
///
/// # Safety
/// The place must satisfy its writable object contract. Plain assignment does
/// not require the previous contents to be initialized.
pub unsafe fn assign<P: WritePlace, V: ImplicitTo<P::Assignment>>(
    place: P,
    value: V,
) -> <P::Type as CType>::Value {
    let value = value.implicit_to();
    // SAFETY: The caller establishes writable storage; conversion happened before
    // the store and no old contents are accessed.
    unsafe { place.store(value) }
}

/// Query known C storage without evaluating any expression.
pub fn size_of<M: CompleteObject>() -> CValue<super::CUnsignedLong> {
    CValue::new(core::mem::size_of::<M::Storage>() as u64)
}
/// Return compiler-verified object alignment with the modeled LP64 `size_t` identity.
pub fn align_of<M: CompleteObject>() -> CValue<super::CUnsignedLong> {
    CValue::new(core::mem::align_of::<M::Storage>() as u64)
}
/// Infer an unevaluated expression's type from a guaranteed-None witness.
/// Generated code supplies `if false { Some(operand) } else { None }`, so Rust
/// checks the operand type without evaluating or capturing its arguments.
pub fn size_of_value_type<V: CExprValue>(_: Option<V>) -> CValue<super::CUnsignedLong>
where
    V::Marker: CompleteObject,
{
    size_of::<V::Marker>()
}
/// Size a declared place from a guaranteed-None witness, retaining array storage.
pub fn size_of_place_type<P: SizeablePlace>(_: Option<P>) -> CValue<super::CUnsignedLong>
where
    P::Object: CompleteObject,
{
    size_of::<P::Object>()
}

/// Identify modeled C `float` independently of native storage and its profile admission.
#[derive(Clone, Copy, Debug)]
pub struct CFloat;
/// Identify modeled C `double` independently of native storage and its profile admission.
#[derive(Clone, Copy, Debug)]
pub struct CDouble;
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for CFloat {}
/// Keep this runtime family within crate-owned C capability registration.
impl sealed::Sealed for CDouble {}

/// A native IEEE C float under a validated no-excess-precision profile.
pub trait FloatType: CType<Value = FloatValue<Self>, Storage: Copy> {}

/// Carry floating storage with its declared C identity until a modeled conversion or operation.
pub struct FloatValue<F: FloatType>(
    /// Store the modeled floating value while its marker retains the C floating kind.
    F::Storage,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<F: FloatType> Copy for FloatValue<F> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<F: FloatType> Clone for FloatValue<F> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<F: FloatType> sealed::Sealed for FloatValue<F> {}
/// Construct and extract floating values without erasing their modeled C floating identity.
impl<F: FloatType> FloatValue<F> {
    /// Tag an already supplied native representation without reading a C object or invoking a callback.
    pub const fn new(value: F::Storage) -> Self {
        Self(value)
    }
    /// Extract the evaluated family's native storage at an explicit boundary without promising a C ABI.
    pub const fn get(self) -> F::Storage {
        self.0
    }
}
/// Associate an evaluated value with its exact C declaration marker.
impl<F: FloatType> CExprValue for FloatValue<F> {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = F;
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<F: FloatType> IntoExpression for FloatValue<F> {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = Self;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self {
        self
    }
}

/// Register a modeled floating kind and its bounded C scalar conversions for the approved profile.
macro_rules! floating_type {
    ($marker:ty, $storage:ty) => {
        /// Connect the modeled floating kind to its native storage and approved C conversion rules.
        impl CType for $marker {
            /// Native float representation used only under the generator-approved C floating profile.
            type Storage = $storage;
            /// Evaluated tagged C value produced from this admitted native storage.
            type Value = FloatValue<Self>;
            /// Tag a native floating value without changing its approved representation.
            fn from_storage(value: $storage) -> Self::Value {
                FloatValue::new(value)
            }
            /// Extract the native floating representation at an explicit binding or conversion boundary.
            fn into_storage(value: Self::Value) -> $storage {
                value.get()
            }
        }
        /// Admit this modeled floating kind only through the runtime’s supported float/double family.
        impl FloatType for $marker {}
        /// Expose floating scalar load/store conversion without erasing its C kind.
        impl ScalarObject for $marker {
            /// Unqualified floating value kind produced by object-to-value conversion.
            type Canonical = Self;
        }
        /// Keep this admitted scalar or type-marker family under crate-owned capability registration.
        impl sealed::Sealed for $storage {}
        /// Attach the unique C float or double marker to this native Rust representation.
        impl NativeType for $storage {
            /// Compiler-facing C identity attached to this native Rust storage.
            type Marker = $marker;
        }
        /// Normalize native floating inputs into the tagged C expression family.
        impl IntoExpression for $storage {
            /// Evaluated tagged C value produced from this admitted native storage.
            type Value = FloatValue<$marker>;
            /// Normalize the evaluated native float while retaining its C float or double identity.
            fn into_expression(self) -> Self::Value {
                FloatValue::new(self)
            }
        }
        /// Respect C integer signedness when converting its numeric value to floating storage.
        impl<K: CInteger> CastTo<$marker> for CValue<K> {
            /// Convert signed or unsigned C integer magnitude to the modeled floating representation.
            fn cast_to(self) -> FloatValue<$marker> {
                let bits = K::encode(self.get());
                FloatValue::new(if K::SIGNED {
                    (bits as i128) as $storage
                } else {
                    bits as $storage
                })
            }
        }
        /// Check float-to-integer range and _Bool truth rules before constructing the destination C value.
        impl<K: CInteger> CastTo<K> for FloatValue<$marker> {
            /// Convert to the C integer target, testing _Bool truth or checking the truncated numeric range before encoding.
            fn cast_to(self) -> CValue<K> {
                let value = self.get();
                if K::RANK == 0 {
                    return CValue::new(K::decode(u128::from(value != 0.0)));
                }
                let value = value.trunc();
                let bits = if K::SIGNED { K::BITS - 1 } else { K::BITS };
                let upper = (2.0 as $storage).powi(bits as i32);
                let lower = if K::SIGNED { -upper } else { 0.0 };
                assert!(
                    value >= lower && value < upper,
                    "C floating to integer conversion out of range"
                );
                let encoded = if K::SIGNED { (value as i128) as u128 } else { value as u128 };
                CValue::new(K::decode(encoded))
            }
        }
        /// Apply admitted C scalar assignment conversion through the corresponding checked cast.
        impl<K: CInteger> ImplicitTo<$marker> for CValue<K> {
            /// Reuse the admitted scalar cast for C assignment conversion to this target kind.
            fn implicit_to(self) -> FloatValue<$marker> {
                cast::<$marker, _>(self)
            }
        }
        /// Apply admitted C scalar assignment conversion through the corresponding checked cast.
        impl<K: CInteger> ImplicitTo<K> for FloatValue<$marker> {
            /// Reuse the admitted scalar cast for C assignment conversion to this target kind.
            fn implicit_to(self) -> CValue<K> {
                cast::<K, _>(self)
            }
        }
    };
}
floating_type!(CFloat, f32);
floating_type!(CDouble, f64);
/// Share explicit and assignment conversion between the two modeled floating representations.
macro_rules! float_cast {
    ($from:ty, $to:ty, $repr:ty) => {
        /// Expose the admitted source-to-destination floating conversion in the C value family.
        impl CastTo<$to> for FloatValue<$from> {
            /// Convert the evaluated floating value to the destination C float or double representation.
            fn cast_to(self) -> FloatValue<$to> {
                FloatValue::new(self.get() as $repr)
            }
        }
        /// Expose the admitted source-to-destination floating conversion in the C value family.
        impl ImplicitTo<$to> for FloatValue<$from> {
            /// Convert the evaluated floating value to the destination C float or double representation.
            fn implicit_to(self) -> FloatValue<$to> {
                cast::<$to, _>(self)
            }
        }
    };
}
float_cast!(CFloat, CFloat, f32);
float_cast!(CFloat, CDouble, f64);
float_cast!(CDouble, CFloat, f32);
float_cast!(CDouble, CDouble, f64);

/// C scalar truth includes non-null pointers and floating NaNs.
pub trait Truth: CExprValue {
    /// Test the admitted C scalar against zero/null without changing the value's type identity.
    fn truth(self) -> bool;
}
/// Expose C zero/null testing without converting the operand to a Rust Boolean value family.
impl<K: CInteger> Truth for CValue<K> {
    /// Test the admitted C scalar against zero/null without changing the value's type identity.
    fn truth(self) -> bool {
        super::truth(self)
    }
}
/// Expose C zero/null testing without converting the operand to a Rust Boolean value family.
impl<M: CType, Q: Qualifier> Truth for Pointer<M, Q> {
    /// Test the admitted C scalar against zero/null without changing the value's type identity.
    fn truth(self) -> bool {
        !self.as_mut_address().is_null()
    }
}
/// Expose C zero/null testing without converting the operand to a Rust Boolean value family.
impl Truth for FloatValue<CFloat> {
    /// Test the admitted C scalar against zero/null without changing the value's type identity.
    fn truth(self) -> bool {
        self.get() != 0.0
    }
}
/// Expose C zero/null testing without converting the operand to a Rust Boolean value family.
impl Truth for FloatValue<CDouble> {
    /// Test the admitted C scalar against zero/null without changing the value's type identity.
    fn truth(self) -> bool {
        self.get() != 0.0
    }
}
/// Test the C scalar truth value used by lazy logical expressions and statement conditions.
pub fn truth<V: Truth>(value: V) -> bool {
    value.truth()
}
/// Return C logical negation as an `int` value rather than a Rust Boolean.
pub fn logical_not<V: Truth>(value: V) -> CValue<super::CInt> {
    CValue::new(i32::from(!value.truth()))
}

/// Unary promotion applies to integers; it leaves floating identities intact.
pub trait Positive: CExprValue {
    /// C result representation selected by this operand family's capability.
    type Output: CExprValue;
    /// Produce the promoted unary-plus result without another operand evaluation.
    fn positive(self) -> Self::Output;
}
/// Apply the C unary-plus rule for this admitted expression family.
impl<K: CInteger> Positive for CValue<K> {
    /// C result representation selected by this operand family's capability.
    type Output = CValue<K::Promoted>;
    /// Produce the promoted unary-plus result without another operand evaluation.
    fn positive(self) -> Self::Output {
        super::promote(self)
    }
}
/// Apply the C unary-plus rule for this admitted expression family.
impl<F: FloatType> Positive for FloatValue<F> {
    /// C result representation selected by this operand family's capability.
    type Output = Self;
    /// Produce the promoted unary-plus result without another operand evaluation.
    fn positive(self) -> Self {
        self
    }
}
/// Apply C unary-plus promotion through the operand family without introducing another evaluation.
pub fn positive<V: Positive>(value: V) -> V::Output {
    value.positive()
}

/// Dispatch promoted C unary negation under the selected signed-overflow policy.
pub trait Negate<P: OverflowPolicy>: CExprValue {
    /// C result representation selected by this operand family's capability.
    type Output: CExprValue;
    /// Produce promoted unary negation under the selected signed-overflow policy.
    fn negate(self) -> Self::Output;
}
/// Apply the C unary-negation rule under the selected overflow policy.
impl<K: CInteger, P: OverflowPolicy> Negate<P> for CValue<K> {
    /// C result representation selected by this operand family's capability.
    type Output = CValue<K::Promoted>;
    /// Produce promoted unary negation under the selected signed-overflow policy.
    fn negate(self) -> Self::Output {
        super::neg::<P, _>(self)
    }
}
/// Apply the C unary-negation rule under the selected overflow policy.
impl<P: OverflowPolicy> Negate<P> for FloatValue<CFloat> {
    /// C result representation selected by this operand family's capability.
    type Output = Self;
    /// Produce promoted unary negation under the selected signed-overflow policy.
    fn negate(self) -> Self {
        Self::new(-self.get())
    }
}
/// Apply the C unary-negation rule under the selected overflow policy.
impl<P: OverflowPolicy> Negate<P> for FloatValue<CDouble> {
    /// C result representation selected by this operand family's capability.
    type Output = Self;
    /// Produce promoted unary negation under the selected signed-overflow policy.
    fn negate(self) -> Self {
        Self::new(-self.get())
    }
}
/// Apply promoted unary negation under the inspected signed-overflow policy.
pub fn neg<P: OverflowPolicy, V: Negate<P>>(value: V) -> V::Output {
    value.negate()
}
/// Invert the promoted C integer representation, preserving the promoted result identity.
pub fn bitnot<K: CInteger>(value: CValue<K>) -> CValue<K::Promoted> {
    super::bitnot(value)
}

/// Define expression-family arithmetic dispatch that retains C promotions and overflow policy.
macro_rules! arithmetic_operator {
    ($trait:ident, $method:ident, $integer:ident, $operator:tt) => {
        /// Describe admitted C operator pairs and their result identity without relying on Rust primitive operator inference.
        pub trait $trait<Rhs: CExprValue, P: OverflowPolicy>: CExprValue {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output: CExprValue;
            /// Apply this C operator and return the result identity established by the operand capability.
            fn $method(self, rhs: Rhs) -> Self::Output;
        }
        /// Use the integer runtime’s promotion, common conversion, and selected signed-overflow policy.
        impl<L: CInteger, R: CInteger, P: OverflowPolicy> $trait<CValue<R>, P> for CValue<L>
        where
            CValue<L>: super::ArithmeticInput<CValue<R>>,
        {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = CValue<<CValue<L> as super::ArithmeticInput<CValue<R>>>::Common>;
            /// Delegate to the integer semantic runtime after the operand family proves promotion and common conversion.
            fn $method(self, rhs: CValue<R>) -> Self::Output {
                super::$integer::<P, _, _>(self, rhs)
            }
        }
        /// Dispatch an analyzed C operator through the operand family’s admitted semantic capability.
        pub fn $method<P: OverflowPolicy, L: $trait<R, P>, R: CExprValue>(
            left: L,
            right: R,
        ) -> L::Output {
            left.$method(right)
        }
        float_arithmetic!($trait, $method, $operator);
    };
}
/// Select the C common floating representation for a concrete operand pair.
macro_rules! float_arithmetic_pair {
    ($trait:ident, $method:ident, $operator:tt, $left:ty, $right:ty, $output:ty) => {
        /// Convert both floating operands to the C common representation before this operation.
        impl<P: OverflowPolicy> $trait<FloatValue<$right>, P> for FloatValue<$left> {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = FloatValue<$output>;
            /// Apply the native floating operator after the required C common-type conversions.
            fn $method(self, rhs: FloatValue<$right>) -> Self::Output {
                FloatValue::new(cast::<$output, _>(self).get() $operator cast::<$output, _>(rhs).get())
            }
        }
    };
}
/// Instantiate all modeled float/double arithmetic pairs through the shared operator capability.
macro_rules! float_arithmetic {
    ($trait:ident, $method:ident, $operator:tt) => {
        float_arithmetic_pair!($trait, $method, $operator, CFloat, CFloat, CFloat);
        float_arithmetic_pair!($trait, $method, $operator, CFloat, CDouble, CDouble);
        float_arithmetic_pair!($trait, $method, $operator, CDouble, CFloat, CDouble);
        float_arithmetic_pair!($trait, $method, $operator, CDouble, CDouble, CDouble);
        float_integer_arithmetic!($trait, $method, $operator, CFloat);
        float_integer_arithmetic!($trait, $method, $operator, CDouble);
    };
}
/// Convert mixed integer/floating operands to the C floating result kind before arithmetic.
macro_rules! float_integer_arithmetic {
    ($trait:ident, $method:ident, $operator:tt, $float:ty) => {
        /// Convert the integer operand to the floating representation selected by C in this operand order.
        impl<K: CInteger, P: OverflowPolicy> $trait<CValue<K>, P> for FloatValue<$float> {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = Self;
            /// Apply the native floating operator after the required C common-type conversions.
            fn $method(self, rhs: CValue<K>) -> Self {
                FloatValue::new(self.get() $operator cast::<$float, _>(rhs).get())
            }
        }
        /// Convert the integer operand to the floating representation selected by C in this operand order.
        impl<K: CInteger, P: OverflowPolicy> $trait<FloatValue<$float>, P> for CValue<K> {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = FloatValue<$float>;
            /// Apply the native floating operator after the required C common-type conversions.
            fn $method(self, rhs: FloatValue<$float>) -> Self::Output {
                FloatValue::new(cast::<$float, _>(self).get() $operator rhs.get())
            }
        }
    };
}
arithmetic_operator!(Add, add, add, +);
arithmetic_operator!(Subtract, sub, sub, -);
arithmetic_operator!(Multiply, mul, mul, *);

/// Share non-policy integer operators through the same promoted/common C value family.
macro_rules! integer_operator {
    ($trait:ident, $method:ident, $integer:ident) => {
        /// Describe admitted C operator pairs and their result identity without relying on Rust primitive operator inference.
        pub trait $trait<Rhs: CExprValue>: CExprValue {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output: CExprValue;
            /// Apply this C operator and return the result identity established by the operand capability.
            fn $method(self, rhs: Rhs) -> Self::Output;
        }
        /// Route admitted integer operands through C promotion and common conversion.
        impl<L: CInteger, R: CInteger> $trait<CValue<R>> for CValue<L>
        where
            CValue<L>: super::ArithmeticInput<CValue<R>>,
        {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = CValue<<CValue<L> as super::ArithmeticInput<CValue<R>>>::Common>;
            /// Delegate to the integer semantic runtime after the operand family proves promotion and common conversion.
            fn $method(self, rhs: CValue<R>) -> Self::Output {
                super::$integer(self, rhs)
            }
        }
        /// Dispatch an analyzed C operator through the operand family’s admitted semantic capability.
        pub fn $method<L: $trait<R>, R: CExprValue>(left: L, right: R) -> L::Output {
            left.$method(right)
        }
    };
}
integer_operator!(Divide, div, div);
integer_operator!(Remainder, rem, rem);
integer_operator!(BitAnd, bitand, bitand);
integer_operator!(BitOr, bitor, bitor);
integer_operator!(BitXor, bitxor, bitxor);

/// Apply the common floating representation before the native division operation.
macro_rules! float_division {
    ($left:ty, $right:ty, $output:ty) => {
        /// Perform division only after converting both operands to the C common floating representation.
        impl Divide<FloatValue<$right>> for FloatValue<$left> {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = FloatValue<$output>;
            /// Divide the converted floating operands and retain the selected C floating result identity.
            fn div(self, rhs: FloatValue<$right>) -> Self::Output {
                FloatValue::new(cast::<$output, _>(self).get() / cast::<$output, _>(rhs).get())
            }
        }
    };
}
float_division!(CFloat, CFloat, CFloat);
float_division!(CFloat, CDouble, CDouble);
float_division!(CDouble, CFloat, CDouble);
float_division!(CDouble, CDouble, CDouble);
/// Apply C mixed floating/integer conversion before division in either operand order.
macro_rules! float_integer_division {
    ($float:ty) => {
        /// Convert the integer operand before mixed C floating division in this operand order.
        impl<K: CInteger> Divide<CValue<K>> for FloatValue<$float> {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = Self;
            /// Divide the converted floating operands and retain the selected C floating result identity.
            fn div(self, rhs: CValue<K>) -> Self {
                Self::new(self.get() / cast::<$float, _>(rhs).get())
            }
        }
        /// Convert the integer operand before mixed C floating division in this operand order.
        impl<K: CInteger> Divide<FloatValue<$float>> for CValue<K> {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = FloatValue<$float>;
            /// Divide the converted floating operands and retain the selected C floating result identity.
            fn div(self, rhs: FloatValue<$float>) -> Self::Output {
                FloatValue::new(cast::<$float, _>(self).get() / rhs.get())
            }
        }
    };
}
float_integer_division!(CFloat);
float_integer_division!(CDouble);

/// Apply C left-shift promotion and reject counts or signed results outside the inspected policy.
pub fn shl<P: OverflowPolicy, L: CInteger, R: CInteger>(
    left: CValue<L>,
    right: CValue<R>,
) -> CValue<L::Promoted> {
    super::shl::<P, _, _>(left, right)
}
/// Apply C right-shift promotion and the modeled compiler choice for signed right shifts.
pub fn shr<L: CInteger, R: CInteger>(left: CValue<L>, right: CValue<R>) -> CValue<L::Promoted> {
    super::shr(left, right)
}

/// Reject offsets that cannot fit the native allocation-address range before pointer arithmetic.
fn pointer_count<K: CInteger>(value: CValue<K>) -> isize {
    let bits = K::encode(value.get());
    if K::SIGNED {
        isize::try_from(bits as i128).expect("C pointer offset exceeds addressable allocation")
    } else {
        isize::try_from(bits).expect("C pointer offset exceeds addressable allocation")
    }
}
/// Select C scalar common conversion or complete-object pointer stride for addition.
impl<M: CompleteObject, Q: Qualifier, K: CInteger, P: OverflowPolicy> Add<CValue<K>, P>
    for Pointer<M, Q>
{
    /// C result representation selected by this operand family's capability.
    type Output = Self;
    /// Apply C addition after selecting scalar conversion or complete-object pointer stride.
    fn add(self, rhs: CValue<K>) -> Self {
        assert!(
            core::mem::size_of::<M::Storage>() != 0,
            "C pointer arithmetic needs nonempty storage"
        );
        Pointer::new(Q::from_mut(self.as_mut_address().wrapping_offset(pointer_count(rhs))))
            .with_access(self.access())
    }
}
/// Select C scalar common conversion or complete-object pointer stride for addition.
impl<M: CompleteObject, Q: Qualifier, K: CInteger, P: OverflowPolicy> Add<Pointer<M, Q>, P>
    for CValue<K>
{
    /// C result representation selected by this operand family's capability.
    type Output = Pointer<M, Q>;
    /// Apply C addition after selecting scalar conversion or complete-object pointer stride.
    fn add(self, rhs: Pointer<M, Q>) -> Self::Output {
        <Pointer<M, Q> as Add<CValue<K>, P>>::add(rhs, self)
    }
}
/// Select the admitted scalar or compatible-pointer subtraction rule.
impl<M: CompleteObject, Q: Qualifier, K: CInteger, P: OverflowPolicy> Subtract<CValue<K>, P>
    for Pointer<M, Q>
{
    /// C result representation selected by this operand family's capability.
    type Output = Self;
    /// Apply C subtraction with the admitted scalar or compatible-pointer result identity.
    fn sub(self, rhs: CValue<K>) -> Self {
        assert!(
            core::mem::size_of::<M::Storage>() != 0,
            "C pointer arithmetic needs nonempty storage"
        );
        Pointer::new(Q::from_mut(
            self.as_mut_address().wrapping_offset(pointer_count(rhs).wrapping_neg()),
        ))
        .with_access(self.access())
    }
}
/// Select the admitted scalar or compatible-pointer subtraction rule.
impl<
    M: CompleteObject + PointeeIdentity,
    N: CompleteObject + PointeeIdentity,
    L: Qualifier,
    R: Qualifier,
    P: OverflowPolicy,
> Subtract<Pointer<N, R>, P> for Pointer<M, L>
where
    M::Identity: CompatibleIdentity<N::Identity> + NonVoidIdentity,
    N::Identity: NonVoidIdentity,
{
    /// C result representation selected by this operand family's capability.
    type Output = CValue<super::CLong>;
    /// Apply C subtraction with the admitted scalar or compatible-pointer result identity.
    fn sub(self, rhs: Pointer<N, R>) -> Self::Output {
        let size = core::mem::size_of::<M::Storage>();
        assert_eq!(
            size,
            core::mem::size_of::<N::Storage>(),
            "C pointer subtraction needs identical storage strides"
        );
        assert!(size != 0, "C pointer subtraction needs nonempty storage");
        // Addresses are sufficient for the numeric result. Unlike offset_from,
        // this does not perform an unsafe Rust operation for different allocations.
        // The original C invocation requires elements of the same array (or its
        // one-past endpoint); callers retain that domain obligation.
        let bytes = (self.as_mut_address().addr() as i128) - (rhs.as_mut_address().addr() as i128);
        assert!(bytes % size as i128 == 0, "C pointer subtraction needs a whole element distance");
        CValue::new(i64::try_from(bytes / size as i128).expect("C ptrdiff_t overflow"))
    }
}
/// C permits both `pointer[index]` and `index[pointer]`.
pub trait Subscript<Rhs: CExprValue>: CExprValue {
    /// C element kind designated by either accepted subscript operand order.
    type Element: CType;
    /// Access qualification retained when constructing the indexed element place.
    type Qualification: Qualifier;
    /// Produce an address-only qualified element place for the accepted C operand order.
    fn subscript(self, rhs: Rhs) -> Place<Self::Element, Self::Qualification>;
}
/// Accept this C subscript operand order while retaining element qualification.
impl<M: CompleteObject, Q: Qualifier, K: CInteger> Subscript<CValue<K>> for Pointer<M, Q> {
    /// C element kind designated by either accepted subscript operand order.
    type Element = M;
    /// Access qualification retained when constructing the indexed element place.
    type Qualification = Q;
    /// Produce an address-only qualified element place for the accepted C operand order.
    fn subscript(self, rhs: CValue<K>) -> Place<M, Q> {
        pointee(add::<super::Undefined, _, _>(self, rhs))
    }
}
/// Accept this C subscript operand order while retaining element qualification.
impl<M: CompleteObject, Q: Qualifier, K: CInteger> Subscript<Pointer<M, Q>> for CValue<K> {
    /// C element kind designated by either accepted subscript operand order.
    type Element = M;
    /// Access qualification retained when constructing the indexed element place.
    type Qualification = Q;
    /// Produce an address-only qualified element place for the accepted C operand order.
    fn subscript(self, rhs: Pointer<M, Q>) -> Place<M, Q> {
        rhs.subscript(self)
    }
}
/// Resolve either C subscript operand order to the same qualified, address-only element place.
pub fn index<L: Subscript<R>, R: CExprValue>(
    left: L,
    right: R,
) -> Place<L::Element, L::Qualification> {
    left.subscript(right)
}

/// Comparison dispatch retains integer promotions and floating NaN behavior.
pub trait Compare<Rhs: CExprValue>: CExprValue {
    /// Test equality after the operand pair selects its C common representation.
    fn eq(self, rhs: Rhs) -> bool;
    /// Test inequality after the operand pair selects its C common representation.
    fn ne(self, rhs: Rhs) -> bool;
    /// Test strict ordering after the operand pair selects its C common representation.
    fn lt(self, rhs: Rhs) -> bool;
    /// Test inclusive ordering after the operand pair selects its C common representation.
    fn le(self, rhs: Rhs) -> bool;
    /// Test reverse strict ordering after the operand pair selects its C common representation.
    fn gt(self, rhs: Rhs) -> bool;
    /// Test reverse inclusive ordering after the operand pair selects its C common representation.
    fn ge(self, rhs: Rhs) -> bool;
}
/// Equality additionally permits C's tagged null constants against pointers.
/// Ordering deliberately retains the stricter Compare operand requirement.
pub trait Equality<Rhs: CExprValue>: CExprValue {
    /// Compare admitted values under C equality rules, including source-proved null constants.
    fn equal(self, rhs: Rhs) -> bool;
    /// Apply the matching C inequality rule without widening the admitted operand types.
    fn not_equal(self, rhs: Rhs) -> bool;
}
/// Return C equality as an `int`, including the admitted pointer/null-constant cases.
pub fn eq<L: Equality<R>, R: CExprValue>(left: L, right: R) -> CValue<super::CInt> {
    CValue::new(i32::from(left.equal(right)))
}
/// Return C inequality as an `int`, retaining the stricter operand-family capabilities.
pub fn ne<L: Equality<R>, R: CExprValue>(left: L, right: R) -> CValue<super::CInt> {
    CValue::new(i32::from(left.not_equal(right)))
}
/// Convert shared ordering capability results to C `int` truth values.
macro_rules! compare_functions {
    ($($method:ident),+ $(,)?) => { $(
        /// Expose the admitted ordering comparison as a C int result instead of a Rust Boolean.
        pub fn $method<L: Compare<R>, R: CExprValue>(left: L, right: R) -> CValue<super::CInt> {
            CValue::new(i32::from(left.$method(right)))
        }
    )+ };
}
compare_functions!(lt, le, gt, ge);
/// Delegate comparison methods to the integer promotion/common-conversion implementation.
macro_rules! integer_compare_methods {
    ($($method:ident),+ $(,)?) => { $(
        /// Compare promoted integer values through the shared C common-conversion runtime.
        fn $method(self, rhs: CValue<R>) -> bool { super::$method(self, rhs).get() != 0 }
    )+ };
}
/// Provide C comparison after the operand pair's required conversion/identity checks.
impl<L: CInteger, R: CInteger> Compare<CValue<R>> for CValue<L>
where
    CValue<L>: super::ArithmeticInput<CValue<R>>,
{
    integer_compare_methods!(eq, ne, lt, le, gt, ge);
}
/// Reuse ordinary equality comparison while preserving the separate null-constant equality capability.
macro_rules! equality_from_compare {
    ($rhs:ty) => {
        /// Reuse the ordinary comparison capability for C equality of this admitted operand pair.
        fn equal(self, rhs: $rhs) -> bool {
            <Self as Compare<$rhs>>::eq(self, rhs)
        }
        /// Reuse the ordinary comparison capability for C inequality without adding null-constant cases.
        fn not_equal(self, rhs: $rhs) -> bool {
            <Self as Compare<$rhs>>::ne(self, rhs)
        }
    };
}
/// Admit C equality for this value pair without widening pointer ordering rules.
impl<L: CInteger, R: CInteger> Equality<CValue<R>> for CValue<L>
where
    CValue<L>: super::ArithmeticInput<CValue<R>>,
{
    equality_from_compare!(CValue<R>);
}
/// Share native comparison mechanics after the caller selects compatible C operand representations.
macro_rules! native_compare_methods {
    ($left:expr, $right:expr, $rhs:ty) => {
        /// Apply native equality after the enclosing capability selects the compatible C representations.
        fn eq(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left == right
        }
        /// Apply native inequality after the enclosing capability selects the compatible C representations.
        fn ne(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left != right
        }
        /// Apply native less-than ordering after the enclosing capability selects the compatible C representations.
        fn lt(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left < right
        }
        /// Apply native less-than-or-equal ordering after the enclosing capability selects the compatible C representations.
        fn le(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left <= right
        }
        /// Apply native greater-than ordering after the enclosing capability selects the compatible C representations.
        fn gt(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left > right
        }
        /// Apply native greater-than-or-equal ordering after the enclosing capability selects the compatible C representations.
        fn ge(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left >= right
        }
    };
}
/// Choose the C common float/double representation before NaN-aware native comparisons.
macro_rules! float_compare_pair {
    ($left:ty, $right:ty, $common:ty) => {
        /// Apply the common C floating representation while retaining native NaN comparison behavior.
        impl Compare<FloatValue<$right>> for FloatValue<$left> {
            native_compare_methods!(
                |v| cast::<$common, _>(v).get(),
                |v| cast::<$common, _>(v).get(),
                FloatValue<$right>
            );
        }
        /// Apply the common C floating representation while retaining native NaN comparison behavior.
        impl Equality<FloatValue<$right>> for FloatValue<$left> {
            equality_from_compare!(FloatValue<$right>);
        }
    };
}
float_compare_pair!(CFloat, CFloat, CFloat);
float_compare_pair!(CFloat, CDouble, CDouble);
float_compare_pair!(CDouble, CFloat, CDouble);
float_compare_pair!(CDouble, CDouble, CDouble);
/// Convert mixed scalar comparisons to the C floating representation in both operand orders.
macro_rules! float_integer_compare {
    ($float:ty) => {
        /// Convert mixed integer/floating values to the C floating comparison representation.
        impl<K: CInteger> Compare<CValue<K>> for FloatValue<$float> {
            native_compare_methods!(|v: Self| v.get(), |v| cast::<$float, _>(v).get(), CValue<K>);
        }
        /// Convert mixed integer/floating values to the C floating comparison representation.
        impl<K: CInteger> Equality<CValue<K>> for FloatValue<$float> {
            equality_from_compare!(CValue<K>);
        }
        /// Convert mixed integer/floating values to the C floating comparison representation.
        impl<K: CInteger> Compare<FloatValue<$float>> for CValue<K> {
            native_compare_methods!(
                |v| cast::<$float, _>(v).get(),
                |v: FloatValue<$float>| v.get(),
                FloatValue<$float>
            );
        }
        /// Convert mixed integer/floating values to the C floating comparison representation.
        impl<K: CInteger> Equality<FloatValue<$float>> for CValue<K> {
            equality_from_compare!(FloatValue<$float>);
        }
    };
}
float_integer_compare!(CFloat);
float_integer_compare!(CDouble);
/// Ordering requires compatible object pointees; void and unrelated
/// object identities cannot gain ordering from the equality void conversion.
impl<M: PointeeIdentity, Q: Qualifier, N: PointeeIdentity, R: Qualifier> Compare<Pointer<N, R>>
    for Pointer<M, Q>
where
    M::Identity: CompatibleIdentity<N::Identity> + NonVoidIdentity,
    N::Identity: NonVoidIdentity,
{
    native_compare_methods!(|v: Self| v.as_mut_address().addr(), |v: Pointer<N, R>| v.as_mut_address().addr(), Pointer<N, R>);
}
/// Admit C equality for this value pair without widening pointer ordering rules.
impl<M: PointeeIdentity, Q: Qualifier, N: PointeeIdentity, R: Qualifier> Equality<Pointer<N, R>>
    for Pointer<M, Q>
where
    M::Identity: CompatibleIdentity<N::Identity>,
{
    /// Compare admitted values under C equality rules, including source-proved null constants.
    fn equal(self, rhs: Pointer<N, R>) -> bool {
        self.as_mut_address().addr() == rhs.as_mut_address().addr()
    }
    /// Apply the matching C inequality rule without widening the admitted operand types.
    fn not_equal(self, rhs: Pointer<N, R>) -> bool {
        self.as_mut_address().addr() != rhs.as_mut_address().addr()
    }
}
/// Admit C equality for this value pair without widening pointer ordering rules.
impl<S: FunctionSignature> Equality<FunctionValue<S>> for FunctionValue<S> {
    /// Compare admitted values under C equality rules, including source-proved null constants.
    fn equal(self, rhs: Self) -> bool {
        self.address() == rhs.address()
    }
    /// Apply the matching C inequality rule without widening the admitted operand types.
    fn not_equal(self, rhs: Self) -> bool {
        self.address() != rhs.address()
    }
}

/// Permit source-proved zero constants in pointer equality without admitting ordinary runtime integers.
macro_rules! null_equality {
    ($pointer:ty, $nonnull:expr, $( $bound:tt )*) => {
        /// Admit pointer equality with a source-proved null constant without widening pointer ordering.
        impl<K: ZeroInteger, $($bound)*> Equality<CValue<K>> for $pointer {
            /// Verify the source-proved zero constant and test the admitted pointer’s nullness without target access.
            fn equal(self, rhs: CValue<K>) -> bool {
                K::verify_zero(rhs.get());
                !$nonnull(self)
            }
            /// Verify the source-proved zero constant and test the admitted pointer’s nullness without target access.
            fn not_equal(self, rhs: CValue<K>) -> bool {
                K::verify_zero(rhs.get());
                $nonnull(self)
            }
        }
        /// Admit pointer equality with a source-proved null constant without widening pointer ordering.
        impl<K: ZeroInteger, $($bound)*> Equality<$pointer> for CValue<K> {
            /// Reuse the pointer/null comparison in reversed operand order without converting an ordinary runtime integer.
            fn equal(self, rhs: $pointer) -> bool {
                <$pointer as Equality<Self>>::equal(rhs, self)
            }
            /// Reuse the pointer/null comparison in reversed operand order without converting an ordinary runtime integer.
            fn not_equal(self, rhs: $pointer) -> bool {
                <$pointer as Equality<Self>>::not_equal(rhs, self)
            }
        }
    };
}
null_equality!(Pointer<M, Q>, |v: Pointer<M, Q>| !v.as_mut_address().is_null(), M: CType, Q: Qualifier);
null_equality!(FunctionValue<S>, |v: FunctionValue<S>| !v.address().is_null(), S: FunctionSignature);

/// Convert the selected arm to a compiler-established conditional result type.
pub fn select_as<M: CType, L: CastTo<M>, R: CastTo<M>>(arm: super::Either<L, R>) -> M::Value {
    match arm {
        super::Either::Left(value) => value.cast_to(),
        super::Either::Right(value) => value.cast_to(),
    }
}

/// Determine a conditional's result identity from both unevaluated arm types.
pub trait Select<Rhs: CExprValue>: CExprValue + Sized {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output: CExprValue;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::Either<Self, Rhs>) -> Self::Output;
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl<L: CInteger, R: CInteger> Select<CValue<R>> for CValue<L>
where
    CValue<L>: super::ArithmeticInput<CValue<R>>,
{
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = CValue<<CValue<L> as super::ArithmeticInput<CValue<R>>>::Common>;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::Either<Self, CValue<R>>) -> Self::Output {
        super::select(arm)
    }
}
/// Determine the floating conditional result from both types while converting only the selected arm.
macro_rules! float_select_pair {
    ($left:ty, $right:ty, $common:ty) => {
        /// Determine the common floating conditional result from both types without evaluating both arms.
        impl Select<FloatValue<$right>> for FloatValue<$left> {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = FloatValue<$common>;
            /// Convert only the evaluated conditional arm to the floating result selected from both arm types.
            fn select(arm: super::Either<Self, FloatValue<$right>>) -> Self::Output {
                select_as::<$common, _, _>(arm)
            }
        }
    };
}
float_select_pair!(CFloat, CFloat, CFloat);
float_select_pair!(CFloat, CDouble, CDouble);
float_select_pair!(CDouble, CFloat, CDouble);
float_select_pair!(CDouble, CDouble, CDouble);
/// Determine mixed conditional results without evaluating the unselected integer or floating branch.
macro_rules! float_integer_select {
    ($float:ty) => {
        /// Select a common floating result for mixed arms while converting only the evaluated branch.
        impl<K: CInteger> Select<CValue<K>> for FloatValue<$float> {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = Self;
            /// Convert only the evaluated conditional arm to the floating result selected from both arm types.
            fn select(arm: super::Either<Self, CValue<K>>) -> Self {
                select_as::<$float, _, _>(arm)
            }
        }
        /// Select a common floating result for mixed arms while converting only the evaluated branch.
        impl<K: CInteger> Select<FloatValue<$float>> for CValue<K> {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = FloatValue<$float>;
            /// Convert only the evaluated conditional arm to the floating result selected from both arm types.
            fn select(arm: super::Either<Self, FloatValue<$float>>) -> Self::Output {
                select_as::<$float, _, _>(arm)
            }
        }
    };
}
float_integer_select!(CFloat);
float_integer_select!(CDouble);
/// Conditional pointer qualification is the union of both arms' qualifiers.
pub trait CommonQualifier<Rhs: Qualifier>: Qualifier {
    /// C result representation selected by this operand family's capability.
    type Output: Qualifier;
}
/// Combine conditional pointer permissions without broadening either arm's write access.
impl<L: Qualifier, R: Qualifier> CommonQualifier<R> for L {
    /// C result representation selected by this operand family's capability.
    type Output = L::Narrow<R>;
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl<M: CType, Q: CommonQualifier<R>, R: Qualifier> Select<Pointer<M, R>> for Pointer<M, Q> {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Pointer<M, Q::Output>;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::Either<Self, Pointer<M, R>>) -> Self::Output {
        select_as::<CPointer<M, Q::Output>, _, _>(arm)
    }
}
/// Admit C pointer/null conditional arms while converting only the branch that was evaluated.
macro_rules! null_select {
    ($pointer:ty, $marker:ty, $( $bound:tt )*) => {
        /// Keep the pointer result identity when one conditional arm is a source-proved null constant.
        impl<K: ZeroInteger, $($bound)*> Select<CValue<K>> for $pointer {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = Self;
            /// Return the evaluated pointer arm or convert the evaluated source-proved zero constant to the common pointer kind.
            fn select(arm: super::Either<Self, CValue<K>>) -> Self {
                match arm {
                    super::Either::Left(value) => value,
                    super::Either::Right(value) => implicit::<$marker, _>(value),
                }
            }
        }
        /// Keep the pointer result identity when one conditional arm is a source-proved null constant.
        impl<K: ZeroInteger, $($bound)*> Select<$pointer> for CValue<K> {
            /// C result identity selected by this admitted operand family after required common conversion.
            type Output = $pointer;
            /// Return the evaluated pointer arm or convert the evaluated source-proved zero constant to the common pointer kind.
            fn select(arm: super::Either<Self, $pointer>) -> Self::Output {
                match arm {
                    super::Either::Left(value) => implicit::<$marker, _>(value),
                    super::Either::Right(value) => value,
                }
            }
        }
    };
}
null_select!(Pointer<M, Q>, CPointer<M, Q>, M: CType, Q: Qualifier);
null_select!(FunctionValue<S>, CFunction<S>, S: FunctionSignature);
/// Determine conditional result identity from both types and convert only the selected arm.
impl<S: FunctionSignature> Select<FunctionValue<S>> for FunctionValue<S> {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Self;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::Either<Self, Self>) -> Self {
        match arm {
            super::Either::Left(value) | super::Either::Right(value) => value,
        }
    }
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl<R: Copy> Select<RecordValue<R>> for RecordValue<R> {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Self;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::Either<Self, Self>) -> Self {
        match arm {
            super::Either::Left(v) | super::Either::Right(v) => v,
        }
    }
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl Select<()> for () {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Self;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(_: super::Either<Self, Self>) {}
}
/// Convert only the selected conditional arm using the result identity determined from both arm types.
pub fn select<L: Select<R>, R: CExprValue>(arm: super::Either<L, R>) -> L::Output {
    L::select(arm)
}

/// Read, compute and write once for a compound C assignment.
///
/// # Safety
/// The place must be initialized and satisfy its complete readable/writable
/// object contract. The computation must not invalidate that storage.
pub unsafe fn modify<
    P: WritePlace,
    R: CExprValue,
    V: ImplicitTo<P::Assignment>,
    F: FnOnce(<P::Type as CType>::Value, R) -> V,
>(
    place: P,
    rhs: R,
    operation: F,
) -> <P::Type as CType>::Value {
    // SAFETY: The caller establishes initialized readable storage.
    let previous = unsafe { place.load() };
    let value = operation(previous, rhs).implicit_to();
    // SAFETY: The caller establishes that storage remains writable after the computation.
    unsafe { place.store(value) }
}

/// Read, compute and write once, retaining the old C value for a postfix operator.
///
/// # Safety
/// The place must satisfy the same initialized read/write contract as modify.
pub unsafe fn post_modify<
    P: WritePlace,
    R: CExprValue,
    V: ImplicitTo<P::Assignment>,
    F: FnOnce(<P::Type as CType>::Value, R) -> V,
>(
    place: P,
    rhs: R,
    operation: F,
) -> <P::Assignment as CType>::Value
where
    <P::Type as CType>::Value: ImplicitTo<P::Assignment> + Copy,
{
    // SAFETY: The caller establishes initialized readable storage.
    let previous = unsafe { place.load() };
    let result = previous.implicit_to();
    let value = operation(previous, rhs).implicit_to();
    // SAFETY: The caller establishes writable storage after the computation.
    unsafe {
        place.store(value);
    }
    result
}

/// Exercise C typed scalar, pointer, callback, and raw-place semantics against native C oracles.
/// rustc rejection cases keep qualifiers, nominal identities, null constants, and unsafe access
/// boundaries distinct; effect counters check assignment and conditional evaluation.
#[cfg(test)]
mod tests {
    //! Fixtures separate compiler-valid C values from Rust binding storage and raw
    //! allocation access. Differential cases check conversions and effects against C;
    //! compile-fail consumers prove that invalid operand families do not acquire
    //! capabilities merely because their native representations have equal widths.

    use super::super::{
        CBool, CInt, CLong, CLongLong, CUnsignedChar, CUnsignedInt, CUnsignedLong, Undefined,
    };
    use super::*;
    use std::cell::Cell;
    use std::fmt::Write;
    use std::mem::MaybeUninit;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    /// Provide a record with independently initialized fields to test address-only projection and raw aggregate copies.
    #[repr(C)]
    struct Partial {
        /// Give the fixture a separately addressable field whose initialization and qualification can be tested independently.
        first: u32,
        /// Give the fixture a separately addressable field whose initialization and qualification can be tested independently.
        second: bool,
    }
    /// Select the independently initialized integer field in the record projection fixtures.
    struct First;
    /// Select the Boolean field whose validity and const access differ from the first field.
    struct Second;

    /// Provide a packed nested layout for offset composition and alignment checks.
    #[repr(C)]
    struct OffsetInner {
        /// Shift the following fixture member so nested offset composition cannot accidentally assume offset zero.
        lead: u8,
        /// Provide the initialized scalar target or packed leaf used by the fixture's access checks.
        value: u32,
    }
    /// Provide nested and trailing-array fields to test offsets without requiring member value capabilities.
    #[repr(C, packed)]
    struct OffsetOuter {
        /// Shift the following fixture member so nested offset composition cannot accidentally assume offset zero.
        lead: u8,
        /// Contain the packed inner record whose member offset must be added to this outer offset.
        nested: OffsetInner,
        /// Provide an incomplete trailing array leaf that needs an offset but no loadable value.
        trailing: [u8; 0],
    }
    /// Select the nested record whose field offset is composed with a leaf offset.
    struct NestedOffset;
    /// Select the packed inner value as the leaf of a compiler-described offset path.
    struct ValueOffset;
    /// Select the flexible-array-style leaf to test offsets without complete member storage.
    struct TrailingOffset;
    /// Select the first field to verify the zero-offset path case.
    struct LeadOffset;
    /// Retain compiler-described member offsets without accessing record storage.
    impl OffsetField<NestedOffset> for CRecord<OffsetOuter> {
        /// Declared field kind or nested path marker supplied by the compiler-owned layout description.
        type Member = CRecord<OffsetInner>;
        /// Compiler/layout-established byte offset used without loading object storage.
        const OFFSET: usize = core::mem::offset_of!(OffsetOuter, nested);
    }
    /// Retain compiler-described member offsets without accessing record storage.
    impl OffsetField<ValueOffset> for CRecord<OffsetInner> {
        /// Declared field kind or nested path marker supplied by the compiler-owned layout description.
        type Member = ();
        /// Compiler/layout-established byte offset used without loading object storage.
        const OFFSET: usize = core::mem::offset_of!(OffsetInner, value);
    }
    /// Retain compiler-described member offsets without accessing record storage.
    impl OffsetField<TrailingOffset> for CRecord<OffsetOuter> {
        /// Declared field kind or nested path marker supplied by the compiler-owned layout description.
        type Member = ();
        /// Compiler/layout-established byte offset used without loading object storage.
        const OFFSET: usize = core::mem::offset_of!(OffsetOuter, trailing);
    }
    /// Retain compiler-described member offsets without accessing record storage.
    impl OffsetField<LeadOffset> for CRecord<OffsetOuter> {
        /// Declared field kind or nested path marker supplied by the compiler-owned layout description.
        type Member = ();
        /// Compiler/layout-established byte offset used without loading object storage.
        const OFFSET: usize = core::mem::offset_of!(OffsetOuter, lead);
    }

    /// Prove nested offset composition agrees with Rust layout without constructing or reading a record.
    #[test]
    fn offset_paths_compose_packed_layout_without_object_storage() {
        /// Compose the fixture's nested record and value offset markers without any object access.
        type Path = OffsetStep<NestedOffset, OffsetStep<ValueOffset, OffsetEnd>>;
        /// Compiler/layout-established byte offset used without loading object storage.
        const OFFSET: CValue<CUnsignedLong> = offset_of::<CRecord<OffsetOuter>, Path>();
        assert_eq!(OFFSET.get(), core::mem::offset_of!(OffsetOuter, nested.value) as u64);
        assert_eq!(<CRecord<OffsetOuter> as OffsetPath<Path>>::DEPTH, 2);
        assert_eq!(offset_of::<CVolatile<CRecord<OffsetOuter>>, Path>().get(), OFFSET.get());
    }

    /// Prove a leaf offset remains available for flexible or otherwise unsupported member value types.
    #[test]
    fn offset_leaf_does_not_require_a_value_or_complete_member_type() {
        /// Select the incomplete trailing member as a leaf whose byte offset remains available.
        type Trailing = OffsetStep<TrailingOffset, OffsetEnd>;
        assert_eq!(
            offset_of::<CRecord<OffsetOuter>, Trailing>().get(),
            core::mem::offset_of!(OffsetOuter, trailing) as u64
        );
        assert_eq!(offset_of::<CRecord<OffsetOuter>, OffsetStep<LeadOffset, OffsetEnd>>().get(), 0);
    }

    // SAFETY: The compiler checks the actual field type and projection. Only the
    // first field is projected, with no record read or qualification broadening.
    /// Describe the fixture member layout and qualification used by address-only field projection.
    unsafe impl OrdinaryField<First> for CRecord<Partial> {
        /// Declared field kind or nested path marker supplied by the compiler-owned layout description.
        type Member = CUnsignedInt;
        /// Member constness used to narrow the containing place's write permission.
        type Declared = ReadWrite;
        /// Compiler/layout-established byte offset used without loading object storage.
        const OFFSET: usize = core::mem::offset_of!(Partial, first);
    }

    // SAFETY: This uncalled witness verifies the metadata's exact Rust member type.
    /// Check the fixture's exact native field storage type without executing the projection witness.
    const _: unsafe fn(*mut Partial) -> *mut u32 =
        |base| unsafe { core::ptr::addr_of_mut!((*base).first) };

    // SAFETY: Partial's repr(C) bool field has its verified native bool storage,
    // byte alignment and declared offset. Qualification only narrows to read access.
    /// Describe the fixture member layout and qualification used by address-only field projection.
    unsafe impl OrdinaryField<Second> for CRecord<Partial> {
        /// Declared field kind or nested path marker supplied by the compiler-owned layout description.
        type Member = super::super::CBool;
        /// Member constness used to narrow the containing place's write permission.
        type Declared = ReadOnly;
        /// Compiler/layout-established byte offset used without loading object storage.
        const OFFSET: usize = core::mem::offset_of!(Partial, second);
    }

    // SAFETY: This uncalled witness verifies the metadata's exact Rust member type.
    /// Check the fixture's exact native field storage type without executing the projection witness.
    const _: unsafe fn(*mut Partial) -> *mut bool =
        |base| unsafe { core::ptr::addr_of_mut!((*base).second) };

    /// Check that field projection retains packing/volatile metadata and never broadens write permission.
    #[test]
    fn ordinary_field_metadata_preserves_inherited_access_and_narrows_qualification() {
        let mut record = Partial { first: 17, second: true };
        let base = place::<CRecord<Partial>>(&raw mut record)
            .with_access(Access { volatile: true, unaligned: true });
        // SAFETY: The live record provides both declared field allocation extents.
        let first: Place<CUnsignedInt> = unsafe { project::<First, _, _>(base) };
        assert!(first.access().volatile && first.access().unaligned);
        // SAFETY: As above. A byte-aligned field needs no inherited unaligned access.
        let second: Place<super::super::CBool, ReadOnly> = unsafe { project::<Second, _, _>(base) };
        assert!(second.access().volatile && !second.access().unaligned);
        // SAFETY: The initialized bool remains live and may be read with volatile access.
        assert!(unsafe { load(second) }.get());
        let base = const_place::<CRecord<Partial>>(&raw const record);
        // SAFETY: The initialized live first field permits read-only projection.
        let first: Place<CUnsignedInt, ReadOnly> = unsafe { project::<First, _, _>(base) };
        // SAFETY: The first field is initialized and alive with aligned read access.
        assert_eq!(unsafe { load(first) }.get(), 17);
        // SAFETY: The live bool remains readable through both const qualifiers.
        let second: Place<super::super::CBool, ReadOnly> = unsafe { project::<Second, _, _>(base) };
        // SAFETY: The bool is initialized, aligned, and readable for this load.
        assert!(unsafe { load(second) }.get());
    }

    /// Prove plain field assignment initializes only the selected field without loading the containing aggregate.
    #[test]
    fn field_assignment_does_not_load_uninitialized_record_or_destination() {
        let mut partial = MaybeUninit::<Partial>::uninit();
        let base = place::<CRecord<Partial>>(partial.as_mut_ptr());
        // SAFETY: The MaybeUninit allocation has the complete Partial extent.
        let field = unsafe { project::<First, _, _>(base) };
        // SAFETY: The field is writable; assignment requires no old value.
        let value = unsafe { assign(field, input(259_i32)) };
        assert_eq!(value.get(), 259_u32);
        // SAFETY: Only the first field is read after its initialization.
        assert_eq!(unsafe { load(field) }.get(), 259_u32);
    }

    /// Record each scalar read and write so mutation tests can prove exact access counts.
    #[derive(Clone, Copy)]
    struct CountingPlace<'a> {
        /// Designate initialized fixture storage whose reads and writes are counted separately.
        slot: *mut u8,
        /// Count destination loads so compound and postfix tests can detect repeated evaluation.
        reads: &'a Cell<u32>,
        /// Count destination stores so mutation tests can detect repeated updates.
        writes: &'a Cell<u32>,
    }
    // SAFETY: This test adapter counts and delegates one load to a typed place.
    /// Expose the declared C load capability through its exact object conversion contract.
    unsafe impl ReadPlace for CountingPlace<'_> {
        /// Declared C object kind retained for access or unevaluated object-size checks.
        type Object = CUnsignedChar;
        /// C expression result kind produced by a place load.
        type Type = CUnsignedChar;
        /// Perform the declared place read and retain its C expression result identity.
        ///
        /// # Safety
        /// The fixture slot must satisfy `ReadPlace::load` for a live initialized byte
        /// allocation, including readable bounds, provenance, and aliasing permissions.
        unsafe fn load(self) -> CValue<CUnsignedChar> {
            self.reads.set(self.reads.get() + 1);
            // SAFETY: The caller's contract applies to this same u8 allocation.
            unsafe { load(place::<CUnsignedChar>(self.slot)) }
        }
    }
    // SAFETY: The adapter delegates one Copy store without accessing old storage.
    /// Expose assignment only for this admitted writable destination family.
    unsafe impl WritePlace for CountingPlace<'_> {
        /// C destination kind used for assignment conversion before storing the object.
        type Assignment = CUnsignedChar;
        /// Write the converted destination value without reading or dropping the previous object.
        ///
        /// # Safety
        /// The backing pointer must designate a live writable byte slot with
        /// sufficient bounds, provenance, and aliasing permissions. Previous contents
        /// need not be initialized; this store does not read them.
        unsafe fn store(self, value: CValue<CUnsignedChar>) -> CValue<CUnsignedChar> {
            self.writes.set(self.writes.get() + 1);
            // SAFETY: The caller establishes writable storage for this u8.
            unsafe { place::<CUnsignedChar>(self.slot).store(value) }
        }
    }

    /// Emulate an unsigned narrow field with explicit promotion and destination truncation.
    #[derive(Clone, Copy)]
    struct UnsignedBitfield(
        /// Designate fixture backing storage for a width-limited field without creating a reference.
        *mut u8,
    );
    // SAFETY: The adapter accesses only this initialized one-byte test slot,
    // retaining the seven-bit value and its compiler-established promotion.
    /// Expose the declared C load capability through its exact object conversion contract.
    unsafe impl ReadPlace for UnsignedBitfield {
        /// Declared C object kind retained for access or unevaluated object-size checks.
        type Object = CUnsignedInt;
        /// C expression result kind produced by a place load.
        type Type = CBitfield<CUnsignedInt, CInt>;
        /// Perform the declared place read and retain its C expression result identity.
        ///
        /// # Safety
        /// The backing pointer must designate a live initialized byte slot with
        /// sufficient readable bounds, provenance, and aliasing permissions.
        unsafe fn load(self) -> CValue<Self::Type> {
            // SAFETY: The caller establishes a readable initialized byte slot.
            CValue::new(u32::from(unsafe { self.0.read() } & 0x7f))
        }
    }
    // SAFETY: Assignment first converts to the declared unsigned C base type,
    // then the adapter narrows to seven bits and retains the bit-field identity.
    /// Expose assignment only for this admitted writable destination family.
    unsafe impl WritePlace for UnsignedBitfield {
        /// C destination kind used for assignment conversion before storing the object.
        type Assignment = CUnsignedInt;
        /// Write the converted destination value without reading or dropping the previous object.
        ///
        /// # Safety
        /// The backing pointer must designate a live writable byte slot
        /// with sufficient bounds, provenance, and aliasing permissions. Previous
        /// contents need not be initialized; this store does not read them.
        unsafe fn store(self, value: CValue<CUnsignedInt>) -> CValue<Self::Type> {
            let stored = (value.get() & 0x7f) as u8;
            // SAFETY: The caller establishes writable storage for this byte slot.
            unsafe { self.0.write(stored) };
            CValue::new(u32::from(stored))
        }
    }

    /// Emulate a narrow field whose declared base rank differs from its promoted expression rank.
    #[derive(Clone, Copy)]
    struct LongBitfield(
        /// Designate fixture backing storage for a width-limited field without creating a reference.
        *mut u64,
    );
    // SAFETY: This initialized test slot models a seven-bit unsigned long field
    // whose original C compiler establishes int as its arithmetic promotion.
    /// Expose the declared C load capability through its exact object conversion contract.
    unsafe impl ReadPlace for LongBitfield {
        /// Declared C object kind retained for access or unevaluated object-size checks.
        type Object = CUnsignedLong;
        /// C expression result kind produced by a place load.
        type Type = CBitfield<CUnsignedLong, CInt>;
        /// Perform the declared place read and retain its C expression result identity.
        ///
        /// # Safety
        /// The backing pointer must designate a live aligned initialized `u64` slot
        /// with sufficient readable bounds, provenance, and aliasing permissions.
        unsafe fn load(self) -> CValue<Self::Type> {
            // SAFETY: The caller establishes an initialized live u64 allocation.
            CValue::new(unsafe { self.0.read() } & 0x7f)
        }
    }
    // SAFETY: The test adapter stores the narrowed field value using the original
    // unsigned long representation, without reading uninitialized adjacent data.
    /// Expose assignment only for this admitted writable destination family.
    unsafe impl WritePlace for LongBitfield {
        /// C destination kind used for assignment conversion before storing the object.
        type Assignment = CUnsignedLong;
        /// Write the converted destination value without reading or dropping the previous object.
        ///
        /// # Safety
        /// The backing pointer must designate a live aligned writable `u64` slot
        /// with sufficient bounds, provenance, and aliasing permissions. Previous
        /// contents need not be initialized; this store does not read them.
        unsafe fn store(self, value: CValue<CUnsignedLong>) -> CValue<Self::Type> {
            let value = value.get() & 0x7f;
            // SAFETY: The caller establishes writable storage for this u64 slot.
            unsafe { self.0.write(value) };
            CValue::new(value)
        }
    }

    /// Check bitfield source type, arithmetic promotion, and `sizeof` retain their distinct C meanings.
    #[test]
    fn bitfield_contexts_preserve_base_size_until_arithmetic_promotion() {
        let value = CValue::<CBitfield<CUnsignedLong, CInt>>::new(127);
        assert_eq!(size_of_value_type(Some(value)).get(), 8);
        assert_eq!(bitnot(value).get(), -128);
        let conditional =
            select(super::super::Either::Left::<_, CValue<CBitfield<CUnsignedLong, CInt>>>(value));
        let _: CValue<CInt> = conditional;
        assert_eq!(size_of_value_type(Some(conditional)).get(), 4);
        let mut storage = 126_u64;
        let target = LongBitfield(&raw mut storage);
        // SAFETY: This local test slot is initialized, live, and writable.
        let prefix = unsafe { modify(target, input(1_i32), add::<Undefined, _, _>) };
        assert_eq!(size_of_value_type(Some(prefix)).get(), 8);
        assert_eq!(bitnot(prefix).get(), -128);
        // SAFETY: The previous operation initialized the same live test slot.
        let postfix: CValue<CUnsignedLong> =
            unsafe { post_modify(target, input(1_i32), add::<Undefined, _, _>) };
        assert_eq!(size_of_value_type(Some(postfix)).get(), 8);
        assert_eq!(bitnot(postfix).get(), 18446744073709551488);
        assert_eq!(storage, 0);
    }

    /// Prove bitfield stores perform base-type assignment conversion before truncating to the field width.
    #[test]
    fn bitfield_assignment_converts_to_unsigned_base_before_width_narrowing() {
        let mut storage = 7_u8;
        let target = UnsignedBitfield(&raw mut storage);
        // SAFETY: The local byte remains live and writable for the bitfield adapter.
        let result: CValue<CBitfield<CUnsignedInt, CInt>> =
            unsafe { assign(target, input(2147483648.0_f64)) };
        assert_eq!((result.get(), storage), (0, 0));
        // SAFETY: The stored byte is initialized and remains readable/writable.
        let result = unsafe { modify(target, input(255_i32), add::<Undefined, _, _>) };
        assert_eq!((result.get(), storage), (127, 127));
        // SAFETY: The same byte remains initialized and writable for postfix update.
        let old = unsafe { post_modify(target, input(1_i32), add::<Undefined, _, _>) };
        assert_eq!((old.get(), storage), (127, 0));
        let _: CValue<CUnsignedInt> = old;
        assert_eq!(bitnot(old).get(), 4294967168);
    }

    /// Count reads and writes to prove compound/postfix operations evaluate the destination once and convert before storing.
    #[test]
    fn compound_and_postfix_operators_load_and_store_once_with_destination_conversion() {
        let mut slot = 250_u8;
        let reads = Cell::new(0);
        let writes = Cell::new(0);
        let target = CountingPlace { slot: &raw mut slot, reads: &reads, writes: &writes };
        // SAFETY: The local u8 is initialized, live and writable without aliases.
        let result = unsafe { modify(target, input(10_i32), add::<Undefined, _, _>) };
        assert_eq!(result.get(), 4_u8);
        assert_eq!((slot, reads.get(), writes.get()), (4, 1, 1));
        // SAFETY: The same initialized local remains valid for the read/write.
        let previous = unsafe { post_modify(target, input(300_i32), add::<Undefined, _, _>) };
        assert_eq!(previous.get(), 4_u8);
        assert_eq!((slot, reads.get(), writes.get()), (48, 2, 2));
    }

    /// Check tagged pointer casts, offsets, and differences retain declared identities and access qualifications.
    #[test]
    fn pointers_keep_rank_qualification_and_provenance_at_cast_boundaries() {
        let mut values = [17_i64, 23_i64, 41_i64];
        let longs = Pointer::<CLong>::new(values.as_mut_ptr());
        let long_longs = cast::<CPointer<CLongLong>, _>(longs);
        let _: Pointer<CLongLong> = long_longs;
        let read_only = cast::<CConstPointer<CLongLong>, _>(long_longs);
        let _: *const i64 = read_only.get();
        let address = cast::<CUnsignedLong, _>(longs);
        let restored = cast::<CPointer<CLong>, _>(address);
        // SAFETY: Integer conversion exposed the original live allocation's
        // provenance, which the restoring cast can recover for this same address.
        assert_eq!(unsafe { load(pointee(restored)) }.get(), 17);
        let third = add::<Undefined, _, _>(read_only, input(2_i32));
        // SAFETY: The third element is initialized in this live array allocation.
        assert_eq!(unsafe { load(pointee(third)) }.get(), 41);
        assert_eq!(sub::<Undefined, _, _>(third, read_only).get(), 2);
    }

    /// Exercise an intentionally unaligned field address through the supported raw copy operations.
    #[test]
    fn packed_place_access_uses_unaligned_copy_operations() {
        let mut bytes = [0_u8; 5];
        let pointer = bytes.as_mut_ptr().wrapping_add(1).cast::<u32>();
        let field =
            place::<CUnsignedInt>(pointer).with_access(Access { volatile: false, unaligned: true });
        // SAFETY: Four writable bytes remain in the allocation; metadata selects
        // unaligned access, and every u32 bit pattern is valid.
        unsafe {
            assign(field, input(0x12345678_u32));
        }
        // SAFETY: The same four initialized bytes remain available for reading.
        assert_eq!(unsafe { load(field) }.get(), 0x12345678);
    }

    /// Prove array decay needs no initialized elements while `sizeof` still measures the complete declared array.
    #[test]
    fn arrays_decay_without_loading_elements_and_preserve_unevaluated_size() {
        let mut array = MaybeUninit::<[Partial; 2]>::uninit();
        let full = place::<CArray<CRecord<Partial>, 2>>(array.as_mut_ptr());
        // SAFETY: Array-to-pointer conversion uses only this allocation's
        // address and never reads the uninitialized array or record elements.
        let decayed: Pointer<CRecord<Partial>> = unsafe { load(full) };
        let element = index(input(1_i32), decayed);
        // SAFETY: The second record and its first field lie within the array
        // allocation; no array element or containing record is loaded.
        let field = unsafe { project::<First, _, _>(element) };
        // SAFETY: This field is writable and its old contents need no initialization.
        unsafe {
            assign(field, input(37_u32));
        }
        // SAFETY: The same individual field was initialized by the assignment.
        assert_eq!(unsafe { load(field) }.get(), 37);
        let values = [17_i32, 23, 41];
        assert_eq!(size_of_value_type(if false { Some(input(values)) } else { None }).get(), 12);
        assert_eq!(
            size_of_place_type(if false {
                Some(native_const_place(&raw const values))
            } else {
                None
            })
            .get(),
            12
        );
        let pointer = decay(native_const_place(&raw const values));
        // SAFETY: The immutable array's third element is initialized and alive.
        assert_eq!(unsafe { load(index(input(2_i32), pointer)) }.get(), 41);
        let _: Pointer<CInt, ReadOnly> = pointer;
    }

    /// Check mutable/const raw pointers and borrowed Rust owners project correctly and invoke `Deref` only once.
    #[test]
    fn native_dereference_reads_retain_raw_pointer_qualifiers_and_borrow_rust_owners() {
        let mut values = [17_i32, 23];
        let mutable = &raw mut values;
        let target: Place<CArray<CInt, 2>> = (&mutable).__pgrx_c_read_deref_place();
        // SAFETY: The live mutable array is designated without element reads.
        let decayed: Pointer<CInt> = unsafe { load(target) };
        // SAFETY: The second initialized i32 is writable within that allocation.
        unsafe { assign(index(input(1_i32), decayed), input(31_i32)) };
        assert_eq!(values[1], 31);
        let immutable = &raw const values;
        let target: Place<CArray<CInt, 2>, ReadOnly> = (&immutable).__pgrx_c_read_deref_place();
        // SAFETY: The array remains alive; decay computes only its address.
        let _: Pointer<CInt, ReadOnly> = unsafe { load(target) };

        let boxed = Box::new(41_i32);
        let target: Place<CInt, ReadOnly> = (&boxed).__pgrx_c_read_deref_place();
        // SAFETY: The owner stays alive and its i32 is initialized.
        assert_eq!(unsafe { load(target) }.get(), 41);
        assert_eq!(*boxed, 41, "the operand was borrowed rather than moved");
        let mut scalar = 43_i32;
        let reference = &mut scalar;
        let target: Place<CInt, ReadOnly> = (&reference).__pgrx_c_read_deref_place();
        // SAFETY: Shared projection of the live initialized reference is readable.
        assert_eq!(unsafe { load(target) }.get(), 43);

        /// Count borrowed Rust dereferences while keeping the target storage alive for the complete access.
        struct Owner<'a> {
            /// Provide the initialized scalar target or packed leaf used by the fixture's access checks.
            value: i32,
            /// Count the owning Rust operand's dereference calls without moving that owner.
            dereferences: &'a core::cell::Cell<usize>,
        }
        /// Count native Rust dereference calls in the operand projection fixture.
        impl core::ops::Deref for Owner<'_> {
            /// Initialized scalar target used to count a borrowed fixture's dereference.
            type Target = i32;
            /// Expose the fixture's initialized scalar while incrementing the exact dereference count.
            fn deref(&self) -> &i32 {
                self.dereferences.set(self.dereferences.get() + 1);
                &self.value
            }
        }
        let dereferences = core::cell::Cell::new(0);
        let owner = Owner { value: 47, dereferences: &dereferences };
        // SAFETY: The owner remains alive and its field is initialized. The
        // adapter performs the native Rust dereference exactly once.
        assert_eq!(unsafe { load((&owner).__pgrx_c_read_deref_place()) }.get(), 47);
        assert_eq!(dereferences.get(), 1);
        // SAFETY: The Box temporary survives through the complete load expression.
        assert_eq!(unsafe { load((&Box::new(53_i32)).__pgrx_c_read_deref_place()) }.get(), 53);
    }

    /// Verify scalar access and array decay carry declared volatility into subsequent pointer operations.
    #[test]
    fn volatile_object_conversion_preserves_access_and_array_pointee_qualification() {
        let mut scalar = 17_i32;
        let target = pointee(Pointer::<CVolatile<CInt>>::new(&raw mut scalar));
        assert!(target.access().volatile);
        // SAFETY: The live initialized scalar is available for volatile access.
        let result: CValue<CInt> = unsafe { assign(target, input(23_i32)) };
        assert_eq!(result.get(), 23);
        // SAFETY: The same initialized scalar remains live and readable.
        assert_eq!(unsafe { load(target) }.get(), 23);
        let mut values = [1_i32, 2, 3];
        let array = place::<CVolatile<CArray<CInt, 3>>>(&raw mut values);
        // SAFETY: Only the live array's first-element address is computed.
        let pointer: Pointer<CVolatile<CInt>> = unsafe { load(array) };
        assert!(pointee(pointer).access().volatile);
    }

    /// Prove a volatile containing record keeps that qualification through field address-taking and dereference.
    #[test]
    fn inherited_volatile_field_qualification_survives_address_and_dereference() {
        let mut partial = MaybeUninit::<Partial>::uninit();
        let base = place::<CVolatile<CRecord<Partial>>>(partial.as_mut_ptr());
        // SAFETY: The live Partial allocation permits this first-field projection.
        let field: Place<CVolatile<CUnsignedInt>> = unsafe { project::<First, _, _>(base) };
        let pointer: Pointer<CVolatile<CUnsignedInt>> = address(field);
        assert!(pointee(pointer).access().volatile);
        // SAFETY: Only the first field is initialized by this volatile assignment.
        unsafe {
            assign(pointee(pointer), input(37_u32));
        }
        // SAFETY: The first field is now initialized, live and volatile-readable.
        assert_eq!(unsafe { load(pointee(pointer)) }.get(), 37);
    }

    /// Check admitted pointer/void conversions preserve addresses and may add, but never remove, qualifiers.
    #[test]
    fn implicit_pointer_conversions_add_qualifiers_and_preserve_object_identity() {
        let mut scalar = 17_i32;
        let pointer = Pointer::<CInt>::new(&raw mut scalar);
        let readonly: Pointer<CInt, ReadOnly> = implicit::<CConstPointer<CInt>, _>(pointer);
        assert_eq!(readonly.get(), pointer.get().cast_const());
        let qualified: Pointer<CVolatile<CVoid>, ReadOnly> =
            implicit::<CConstPointer<CVolatile<CVoid>>, _>(pointer);
        let restored: Pointer<CVolatile<CInt>, ReadOnly> =
            implicit::<CConstPointer<CVolatile<CInt>>, _>(qualified);
        assert!(pointee(restored).access().volatile);
        // SAFETY: Void conversion retained this live scalar's provenance and
        // restored its original storage identity without dropping qualifiers.
        assert_eq!(unsafe { load(pointee(restored)) }.get(), 17);
        assert!(implicit::<CBool, _>(pointer).get());
        assert_eq!(implicit::<CUnsignedInt, _>(input(2147483648.0_f64)).get(), 2147483648_u32);
        let native_void = pointer.get().cast::<core::ffi::c_void>();
        let erased: Pointer<CVoid> = input(native_void);
        assert_eq!(eq(pointer, erased).get(), 1);
        let native_const_void = native_void.cast_const();
        let _: Pointer<CVoid, ReadOnly> = input(native_const_void);
        let mut values = [17_i32, 23];
        let first = Pointer::<CInt>::new(values.as_mut_ptr());
        let second = add::<Undefined, _, _>(first, input(1_i32));
        let qualified = implicit::<CConstPointer<CVolatile<CInt>>, _>(second);
        assert_eq!(lt(first, qualified).get(), 1);
    }

    /// Verify proved zero expressions retain integer size and gain only the C-permitted contextual null conversions.
    #[test]
    fn null_constants_keep_integer_size_and_contextual_pointer_conversions() {
        let zero = null_constant(input(0_i32));
        assert_eq!(size_of_value_type(Some(zero)).get(), 4);
        let promoted: CValue<CInt> = positive(zero);
        assert_eq!(promoted.get(), 0);
        let arithmetic: CValue<CInt> = add::<Undefined, _, _>(zero, input(7_i32));
        assert_eq!(arithmetic.get(), 7);
        let converted: CValue<CUnsignedLong> = cast::<CUnsignedLong, _>(zero);
        assert_eq!(converted.get(), 0);
        let pointer: Pointer<CInt, ReadOnly> = implicit::<CConstPointer<CInt>, _>(zero);
        assert!(pointer.get().is_null());
        assert_eq!(eq(pointer, zero).get(), 1);
        assert_eq!(ne(zero, pointer).get(), 0);
        let function = implicit::<CFunction<IncrementSignature>, _>(zero);
        assert_eq!(eq(function, zero).get(), 1);
        assert_eq!(ne(zero, function).get(), 0);
        let mut scalar = 17_i32;
        let nonnull = Pointer::<CInt>::new(&raw mut scalar);
        let callable = FunctionValue::<IncrementSignature>::new(Some(increment));
        for condition in [false, true] {
            let pointer = select(if condition {
                super::super::Either::Left(nonnull)
            } else {
                super::super::Either::Right(zero)
            });
            assert_eq!(truth(pointer), condition);
            let function = select(if condition {
                super::super::Either::Left(zero)
            } else {
                super::super::Either::Right(callable)
            });
            assert_eq!(truth(function), !condition);
        }
        assert!(std::panic::catch_unwind(|| null_constant(input(1_i32))).is_err());
        assert!(
            std::panic::catch_unwind(|| {
                implicit::<CPointer<CInt>, _>(NullConstant::<CInt>::new(1))
            })
            .is_err()
        );
    }

    /// Prove storage and public-input boundaries erase source-only null/bitfield facts while retaining integer identity.
    #[test]
    fn variable_loads_remove_constant_and_bitfield_expression_metadata() {
        let zero = null_constant(input(0_i32));
        let ordinary: CValue<CInt> = input(zero);
        assert_eq!(ordinary.get(), 0);
        let field = CValue::<CBitfield<CUnsignedLong, CInt>>::new(127);
        let ordinary: CValue<CUnsignedLong> = input(field);
        assert_eq!(bitnot(ordinary).get(), 18446744073709551488);
        let mut storage = zero;
        let target = native_place(&raw mut storage);
        // SAFETY: The live local storage has the transparent integer wrapper's
        // complete representation; mutation does not create a new ICE proof.
        unsafe {
            assign(target, input(1_i32));
        }
        // SAFETY: The initialized local remains alive and readable. Loading a
        // variable returns the ordinary declared C integer identity.
        let ordinary: CValue<CInt> = unsafe { load(target) };
        assert_eq!(ordinary.get(), 1);
        let pointer: Pointer<CInt> = implicit::<CPointer<CInt>, _>(address(target));
        // SAFETY: CStoredInteger preserves the transparent representation and
        // mutable allocation provenance of this initialized int object.
        assert_eq!(unsafe { load(pointee(pointer)) }.get(), 1);
    }

    /// Check `usize`/`isize` binding bridges retain the inspected C integer rank and all native storage bits.
    #[cfg(target_pointer_width = "64")]
    #[test]
    fn pointer_sized_binding_storage_keeps_c_rank_and_native_width() {
        let mut storage = usize::MAX;
        let target = place::<CIntegerStorage<CUnsignedLong, usize>>(&raw mut storage);
        // SAFETY: This initialized usize is live and read through its actual
        // binding storage type; the adapter preserves all 64 bits.
        let initial: CValue<CUnsignedLong> = unsafe { load(target) };
        assert_eq!(initial.get(), u64::MAX);
        // SAFETY: The same native storage remains writable without aliases.
        let result: CValue<CUnsignedLong> = unsafe { assign(target, input(37_u32)) };
        assert_eq!((result.get(), storage), (37, 37));
        let signed = <CIntegerStorage<CLongLong, isize> as CType>::from_storage(-1);
        assert_eq!(signed.get(), -1_i64);
    }

    /// Register one exact native callback signature for call, address, and null-pointer tests.
    #[derive(Clone, Copy)]
    struct IncrementSignature;
    /// Keep this runtime family within crate-owned C capability registration.
    impl sealed::Sealed for IncrementSignature {}
    /// Retain exact callback storage, null representation, and original native address.
    impl FunctionSignature for IncrementSignature {
        /// Exact nullable native function-pointer representation whose ABI is separately verified.
        type Pointer = Option<unsafe extern "C-unwind" fn(i32) -> i32>;
        /// Construct the exact nullable native callback representation without creating a callable address.
        fn null() -> Self::Pointer {
            None
        }
        /// Retain the object/function address interpretation required by this C operand family.
        fn address(pointer: Self::Pointer) -> *const () {
            pointer.map_or(core::ptr::null(), |function| function as *const ())
        }
    }
    /// Provide an exact native callback fixture whose result reveals argument conversion and invocation.
    unsafe extern "C-unwind" fn increment(value: i32) -> i32 {
        value + 1
    }
    /// Invoke this exact callback signature after converting operands and checking nullable storage.
    impl<V: ImplicitTo<CInt>> Call<(V,)> for IncrementSignature {
        /// Tagged result returned after exact callback ABI decoding.
        type Output = CValue<CInt>;
        /// Invoke the exact native callback after argument conversion and null validation.
        ///
        /// # Safety
        /// The non-null pointer and converted arguments must satisfy `Call::call` for
        /// the exact native signature and target preconditions. PostgreSQL targets also
        /// require the permitted backend thread and appropriate error/panic guards; a
        /// Rust guard wrapper must not be substituted for the original C function address.
        unsafe fn call(pointer: Self::Pointer, args: (V,)) -> Self::Output {
            let value = implicit::<CInt, _>(args.0).get();
            let callable = pointer.expect("C indirect call requires a non-null function pointer");
            // SAFETY: The caller establishes this exact fixture signature; the
            // native test function neither calls PostgreSQL nor requires guards.
            let result = unsafe { callable(value) };
            CValue::new(result)
        }
    }

    /// Prove function designators survive address/dereference cancellation without invoking a callback.
    #[test]
    fn dereference_and_address_preserve_function_designators_without_a_call() {
        let function = FunctionValue::<IncrementSignature>::new(Some(increment));
        assert_eq!(address(function).address(), function.address());
        // SAFETY: Function dereference merely retains this verified signature.
        assert_eq!(unsafe { dereference(function) }.address(), function.address());
        let null = FunctionValue::<IncrementSignature>::new(None);
        assert!(dereference_address(null).get().is_none());
        let pointer = Pointer::<CInt>::new(core::ptr::null_mut());
        assert!(dereference_address(pointer).get().is_null());
        let value = 17_i32;
        // SAFETY: The local remains live and initialized for this read.
        assert_eq!(unsafe { dereference(input(&raw const value)) }.get(), value);
    }

    /// Exercise callback conversion, null testing, indirect calls, and function-pointer object storage with one exact signature.
    #[test]
    fn function_values_keep_exact_signature_and_nullable_binding_storage() {
        let mut storage: <IncrementSignature as FunctionSignature>::Pointer = Some(increment);
        let target = place::<CFunction<IncrementSignature>>(&raw mut storage);
        // SAFETY: This initialized Option stores the verified native signature.
        let function = unsafe { load(target) };
        assert!(truth(function));
        assert_eq!(
            eq(function, FunctionValue::<IncrementSignature>::new(Some(increment))).get(),
            1
        );
        let callable = function.get().expect("known non-null native fixture function");
        // SAFETY: This fixture has the exact C ABI and no backend requirements.
        assert_eq!(unsafe { callable(17) }, 18);
        // SAFETY: The sealed fixture adapter preserves this exact native ABI;
        // the fully converted argument has no backend or resource obligations.
        assert_eq!(unsafe { invoke(function, (input(17_i16),)) }.get(), 18);
        assert!(!truth(FunctionValue::<IncrementSignature>::new(None)));
        let selected = select(if true {
            super::super::Either::Left(function)
        } else {
            super::super::Either::Right(FunctionValue::<IncrementSignature>::new(None))
        });
        assert_eq!(eq(selected, function).get(), 1);

        // A pointer to the function-pointer storage is an object pointer (C
        // int (**)(int)), unlike the function address stored in FunctionValue.
        let object = Pointer::<CFunction<IncrementSignature>>::new(&raw mut storage);
        let erased = implicit::<CPointer<CVoid>, _>(object);
        let restored = implicit::<CPointer<CFunction<IncrementSignature>>, _>(erased);
        // SAFETY: The void conversion retained this initialized object's live
        // provenance and restores its exact function-pointer storage type.
        let function = unsafe { load(pointee(restored)) };
        assert_eq!(function.address(), increment as *const ());
        let mut functions = [Some(increment as unsafe extern "C-unwind" fn(i32) -> i32); 2];
        let first = Pointer::<CFunction<IncrementSignature>>::new(functions.as_mut_ptr());
        let second = add::<Undefined, _, _>(first, input(1_i32));
        assert_eq!(lt(first, second).get(), 1);
    }

    /// Count effects to distinguish unevaluated size witnesses from evaluated C void-discard expressions.
    #[test]
    fn sizeof_and_void_do_not_change_c_evaluation_counts() {
        let evaluations = Cell::new(0);
        let size = size_of_value_type(if false {
            evaluations.set(evaluations.get() + 1);
            Some(input(7_u32))
        } else {
            None
        });
        assert_eq!((size.get(), evaluations.get()), (4, 0));
        let null = Pointer::<CInt>::new(core::ptr::null_mut());
        let size = size_of_value_type(if false {
            // SAFETY: This false branch is a type witness and is never executed.
            Some(unsafe { load(pointee(null)) })
        } else {
            None
        });
        assert_eq!(size.get(), 4);
        let mut storage = 3_i32;
        let size = size_of_value_type(if false {
            // SAFETY: This branch is unconditionally false, so no load/store
            // occurs. Its type establishes the original unevaluated C operand.
            Some(unsafe {
                post_modify(native_place(&raw mut storage), input(1_i32), add::<Undefined, _, _>)
            })
        } else {
            None
        });
        assert_eq!((size.get(), storage), (4, 3));
        cast::<CVoid, _>({
            evaluations.set(1);
            input(9_i32)
        });
        assert_eq!(evaluations.get(), 1);
    }

    /// Check NaN, signed zero, truncation, and mixed float/integer conversion behavior against the modeled C rules.
    #[test]
    fn floating_conversions_and_comparisons_follow_c_scalar_rules() {
        assert!(truth(input(f64::NAN)));
        assert!(cast::<CBool, _>(input(f32::NAN)).get());
        assert!(!truth(input(-0.0_f32)));
        assert_eq!(eq(input(f32::NAN), input(f64::NAN)).get(), 0);
        assert_eq!(ne(input(f32::NAN), input(f64::NAN)).get(), 1);
        assert_eq!(lt(input(f64::NAN), input(1_i32)).get(), 0);
        assert_eq!(cast::<CUnsignedChar, _>(input(-0.75_f64)).get(), 0);
        assert_eq!(cast::<CUnsignedChar, _>(input(251.75_f64)).get(), 251);
        assert_eq!(neg::<Undefined, _>(input(0.0_f32)).get().to_bits(), (-0.0_f32).to_bits());
        let promoted: FloatValue<CDouble> = add::<Undefined, _, _>(input(1.25_f32), input(2.5_f64));
        assert_eq!(promoted.get(), 3.75);
    }

    /// Give concurrent native-oracle runs distinct owned temporary directory names.
    static NEXT_ORACLE: AtomicU64 = AtomicU64::new(0);
    /// Own isolated native-oracle scratch files and remove only that directory after validation.
    struct OracleDirectory(
        /// Identify the owned scratch directory so cleanup cannot remove another oracle's files.
        PathBuf,
    );
    /// Manage isolated native-oracle scratch storage through one owning fixture.
    impl OracleDirectory {
        /// Create isolated oracle scratch storage with a unique process-local name.
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "pgrx-expression-oracle-{}-{}",
                std::process::id(),
                NEXT_ORACLE.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir(&path).expect("create bounded C expression oracle directory");
            Self(path)
        }
    }
    /// Clean up this fixture's owned resource or expose unexpected record destruction.
    impl Drop for OracleDirectory {
        /// Remove only the native-oracle scratch directory owned by this fixture.
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// Compare runtime expression types, values, mutations, and scalar bit patterns with an independently compiled C oracle.
    #[test]
    fn runtime_expression_families_match_independent_original_c() {
        let directory = OracleDirectory::new();
        let source = directory.0.join("original.c");
        let binary = directory.0.join("original");
        std::fs::write(
            &source,
            r#"
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <math.h>
#define ASSIGN(x,v) ((x)=(v))
#define COMPOUND(x,v) ((x)+=(v))
#define POST(x) ((x)++)
#define POINTER(p,n) ((p)+(n))
#define DIFFERENCE(p,q) ((p)-(q))
_Static_assert(_Generic(ASSIGN(*(uint8_t*)0, 259), uint8_t:1, default:0), "assignment result type");
_Static_assert(_Generic(COMPOUND(*(uint8_t*)0, 300), uint8_t:1, default:0), "compound result type");
_Static_assert(_Generic(POST(*(uint8_t*)0), uint8_t:1, default:0), "postfix result type");
_Static_assert(_Generic((float)1 + (double)2, double:1, default:0), "float common type");
static uint32_t bits32(float v) { uint32_t b; memcpy(&b,&v,sizeof(b)); return b; }
static uint64_t bits64(double v) { uint64_t b; memcpy(&b,&v,sizeof(b)); return b; }
static int increment(int value) { return value + 1; }
int main(void) {
  for(unsigned i=0;i<256;i++) {
    uint8_t value=(uint8_t)i;
    uint8_t assigned=ASSIGN(value,259);
    uint8_t changed=COMPOUND(value,300);
    uint8_t previous=POST(value);
    printf("mutation %u %u %u %u %u\n",i,assigned,changed,previous,value);
  }
  for(int i=-32768;i<32768;i+=257) {
    float a=(float)i/17.0f; double b=(double)i/19.0;
    printf("float %d %u %llu %llu %d %d\n",i,bits32(a+3),
      (unsigned long long)bits64(a+b),(unsigned long long)bits64(b-((unsigned int)3)),a<b,a!=b);
  }
  int values[3]={17,23,41}; int *first=values; int *last=POINTER(first,2);
  printf("pointer %ld %d %d\n",(long)DIFFERENCE(last,first),*last, first!=NULL);
  printf("array %lu %lu %d\n",(unsigned long)sizeof(values),(unsigned long)sizeof(values[0]),2[values]);
  printf("nan %d %d %d\n",(bool)NAN,NAN==NAN,NAN!=NAN);
  printf("cast %u %u\n",(unsigned)(uint8_t)-0.75,(unsigned)(uint8_t)251.75);
  int (*function)(int)=increment; int (*null_function)(int)=NULL;
  printf("function %d %d %d %d\n",function(17),function!=NULL,null_function!=NULL,function==increment);
  struct { unsigned int value:7; } bitfield = {7};
  unsigned int assigned=(bitfield.value=2147483648.0);
  unsigned int changed=(bitfield.value+=255);
  unsigned int previous=bitfield.value++;
  printf("bitfield %u %u %u %u\n",assigned,changed,previous,(unsigned)bitfield.value);
  struct { unsigned long value:7; } longfield = {127};
  int comma_not=~(0,longfield.value);
  int assignment_not=~(longfield.value=127);
  longfield.value=126;
  int prefix_not=~++longfield.value;
  unsigned long postfix_not=~longfield.value++;
  printf("longbitfield %lu %lu %lu %lu %lu %d %d %d %lu\n",
    (unsigned long)sizeof((0,longfield.value)),(unsigned long)sizeof(longfield.value=127),
    (unsigned long)sizeof(++longfield.value),(unsigned long)sizeof(longfield.value++),
    (unsigned long)sizeof(1?longfield.value:longfield.value),
    comma_not,assignment_not,prefix_not,postfix_not);
  void *object=&function; int (**restored)(int)=object;
  printf("functionobject %d\n",(*restored)(17));
  int *null_object=0L;
  int (*null_callback)(int)=0;
  printf("null %lu %lu %d %d %d %d %d %d %d %d\n",
    (unsigned long)sizeof(0),(unsigned long)sizeof(0L),
    null_object==0,0!=null_object,null_callback==0,0!=function,
    (1?values:0)==values,(0?0:values)==values,
    (1?function:0)==function,(0?0:function)==function);
  return 0;
}
"#,
        )
        .expect("write original C expression fixture");
        let compiler = std::process::Command::new("clang")
            .args(["-std=c11", "-fwrapv", "-Wno-constant-conversion"])
            .arg(&source)
            .arg("-o")
            .arg(&binary)
            .output()
            .expect("compile original C expression oracle");
        assert!(compiler.status.success(), "{}", String::from_utf8_lossy(&compiler.stderr));
        let output =
            std::process::Command::new(&binary).output().expect("run original C expression oracle");
        assert!(output.status.success());
        let original =
            String::from_utf8(output.stdout).expect("C expression oracle produces UTF-8 records");
        let mut translated = String::new();
        for i in 0..256 {
            let mut value = i as u8;
            let target = place::<CUnsignedChar>(&raw mut value);
            // SAFETY: This local u8 remains live and exclusively accessed by raw
            // copies for each ordinary C assignment operation.
            let (assigned, changed, previous) = unsafe {
                (
                    assign(target, input(259_i32)),
                    modify(target, input(300_i32), add::<Undefined, _, _>),
                    post_modify(target, input(1_i32), add::<Undefined, _, _>),
                )
            };
            writeln!(
                translated,
                "mutation {i} {} {} {} {value}",
                assigned.get(),
                changed.get(),
                previous.get()
            )
            .unwrap();
        }
        for i in (-32768..32768).step_by(257) {
            let a = input(i as f32 / 17.0_f32);
            let b = input(i as f64 / 19.0_f64);
            writeln!(
                translated,
                "float {i} {} {} {} {} {}",
                add::<Undefined, _, _>(a, input(3_i32)).get().to_bits(),
                add::<Undefined, _, _>(a, b).get().to_bits(),
                sub::<Undefined, _, _>(b, input(3_u32)).get().to_bits(),
                lt(a, b).get(),
                ne(a, b).get()
            )
            .unwrap();
        }
        let mut values = [17_i32, 23, 41];
        let first = Pointer::<CInt>::new(values.as_mut_ptr());
        let last = add::<Undefined, _, _>(first, input(2_i32));
        // SAFETY: last points to the initialized third element of this array.
        writeln!(
            translated,
            "pointer {} {} {}",
            sub::<Undefined, _, _>(last, first).get(),
            unsafe { load(pointee(last)) }.get(),
            i32::from(truth(first))
        )
        .unwrap();
        let array = native_place(&raw mut values);
        // SAFETY: The array and its third element remain initialized and alive.
        writeln!(
            translated,
            "array {} {} {}",
            size_of::<CArray<CInt, 3>>().get(),
            size_of::<CInt>().get(),
            unsafe { load(index(input(2_i32), decay(array))) }.get()
        )
        .unwrap();
        writeln!(
            translated,
            "nan {} {} {}",
            i32::from(cast::<CBool, _>(input(f64::NAN)).get()),
            eq(input(f64::NAN), input(f64::NAN)).get(),
            ne(input(f64::NAN), input(f64::NAN)).get()
        )
        .unwrap();
        writeln!(
            translated,
            "cast {} {}",
            cast::<CUnsignedChar, _>(input(-0.75_f64)).get(),
            cast::<CUnsignedChar, _>(input(251.75_f64)).get()
        )
        .unwrap();
        let function = FunctionValue::<IncrementSignature>::new(Some(increment));
        let callable = function.get().expect("known non-null native fixture function");
        // SAFETY: The fixture has the exact verified ABI and argument types.
        writeln!(
            translated,
            "function {} {} {} {}",
            unsafe { callable(17) },
            i32::from(truth(function)),
            i32::from(truth(FunctionValue::<IncrementSignature>::new(None))),
            eq(function, FunctionValue::<IncrementSignature>::new(Some(increment))).get(),
        )
        .unwrap();
        let mut storage = 7_u8;
        let target = UnsignedBitfield(&raw mut storage);
        // SAFETY: This byte slot is live and initialized for the independent
        // C fixture's three sequential bit-field assignment operations.
        let (assigned, changed, previous) = unsafe {
            (
                assign(target, input(2147483648.0_f64)),
                modify(target, input(255_i32), add::<Undefined, _, _>),
                post_modify(target, input(1_i32), add::<Undefined, _, _>),
            )
        };
        writeln!(
            translated,
            "bitfield {} {} {} {storage}",
            assigned.get(),
            changed.get(),
            previous.get()
        )
        .unwrap();
        let mut storage = 127_u64;
        let target = LongBitfield(&raw mut storage);
        // SAFETY: These sequential operations access only this initialized live
        // slot, preserving the original C bit-field expression's context.
        let (comma, assigned, prefix, postfix) = unsafe {
            let comma = load(target);
            let assigned = assign(target, input(127_i32));
            assign(target, input(126_i32));
            let prefix = modify(target, input(1_i32), add::<Undefined, _, _>);
            let postfix = post_modify(target, input(1_i32), add::<Undefined, _, _>);
            (comma, assigned, prefix, postfix)
        };
        let conditional =
            select(super::super::Either::Left::<_, CValue<CBitfield<CUnsignedLong, CInt>>>(comma));
        writeln!(
            translated,
            "longbitfield {} {} {} {} {} {} {} {} {}",
            size_of_value_type(Some(comma)).get(),
            size_of_value_type(Some(assigned)).get(),
            size_of_value_type(Some(prefix)).get(),
            size_of_value_type(Some(postfix)).get(),
            size_of_value_type(Some(conditional)).get(),
            bitnot(comma).get(),
            bitnot(assigned).get(),
            bitnot(prefix).get(),
            bitnot(postfix).get(),
        )
        .unwrap();
        let mut function_storage = Some(increment as unsafe extern "C-unwind" fn(i32) -> i32);
        let object = Pointer::<CFunction<IncrementSignature>>::new(&raw mut function_storage);
        let erased = implicit::<CPointer<CVoid>, _>(object);
        let restored = implicit::<CPointer<CFunction<IncrementSignature>>, _>(erased);
        // SAFETY: Erasure retained the initialized function-pointer object's
        // live provenance, ABI, and exact argument and return type identity.
        let callable = unsafe { load(pointee(restored)) }.get().unwrap();
        writeln!(translated, "functionobject {}", unsafe { callable(17) }).unwrap();
        let zero = null_constant(input(0_i32));
        let long_zero = null_constant(CValue::<CLong>::new(0));
        let null_object = implicit::<CPointer<CInt>, _>(long_zero);
        let null_callback = implicit::<CFunction<IncrementSignature>, _>(zero);
        let object_selected = select(if true {
            super::super::Either::Left(first)
        } else {
            super::super::Either::Right(zero)
        });
        let object_selected_reverse = select(if false {
            super::super::Either::Left(zero)
        } else {
            super::super::Either::Right(first)
        });
        let function_selected = select(if true {
            super::super::Either::Left(function)
        } else {
            super::super::Either::Right(zero)
        });
        let function_selected_reverse = select(if false {
            super::super::Either::Left(zero)
        } else {
            super::super::Either::Right(function)
        });
        writeln!(
            translated,
            "null {} {} {} {} {} {} {} {} {} {}",
            size_of_value_type(Some(zero)).get(),
            size_of_value_type(Some(long_zero)).get(),
            eq(null_object, zero).get(),
            ne(zero, null_object).get(),
            eq(null_callback, zero).get(),
            ne(zero, function).get(),
            eq(object_selected, first).get(),
            eq(object_selected_reverse, first).get(),
            eq(function_selected, function).get(),
            eq(function_selected_reverse, function).get(),
        )
        .unwrap();
        assert_eq!(translated, original, "C and typed Rust runtime expression observations differ");
    }

    /// Compile paired C and Rust cases to distinguish callable addresses from addressable function-pointer objects.
    #[test]
    fn original_c_distinguishes_function_addresses_from_function_pointer_objects() {
        let directory = OracleDirectory::new();
        for (case, body, accepted) in [
            (
                "function_object",
                "void *erased=&function; Fn **restored=erased; (void)restored;",
                true,
            ),
            ("function_address", "void *erased=function; (void)erased;", false),
            (
                "void_to_function_address",
                "void *erased=0; Fn *restored=erased; (void)restored;",
                false,
            ),
            (
                "null_object_and_callback",
                "int *object=0L; Fn *callback=0; (void)object;(void)callback;",
                true,
            ),
            ("null_equality", "int *object=0; (void)(object==0);(void)(0!=function);", true),
            ("dynamic_zero_to_pointer", "int zero=0;int *object=zero;(void)object;", false),
            (
                "dynamic_zero_pointer_equality",
                "int zero=0;int *object=0;(void)(object==zero);",
                false,
            ),
            ("null_pointer_ordering", "int *object=0;(void)(object<0);", false),
            ("function_ordering", "Fn *peer=0;(void)(function<peer);", false),
            ("function_void_equality", "void *erased=0;(void)(function==erased);", false),
            (
                "incompatible_function_equality",
                "typedef int Other(double);Other *peer=0;(void)(function==peer);",
                false,
            ),
            (
                "unrelated_record_equality",
                "struct A{int x;};struct B{int x;};struct A *a=0;struct B *b=0;(void)(a==b);",
                false,
            ),
            ("plain_signed_char_equality", "char *a=0;signed char *b=0;(void)(a==b);", false),
            ("qualified_object_ordering", "int *a=0;const volatile int *b=0;(void)(a<b);", true),
            ("object_void_equality", "int *a=0;void *b=0;(void)(a==b);", true),
            ("function_pointer_object_ordering", "Fn **a=&function;Fn **b=0;(void)(a<b);", true),
        ] {
            let source = directory.0.join(format!("{case}.c"));
            std::fs::write(
                &source,
                format!("typedef int Fn(int); void fixture(Fn *function) {{{body}}}"),
            )
            .unwrap();
            let output = std::process::Command::new("clang")
                .args(["-std=c11", "-Werror", "-pedantic-errors", "-fsyntax-only"])
                .arg(&source)
                .output()
                .expect("check original C function-pointer conversion constraints");
            assert_eq!(
                output.status.success(),
                accepted,
                "{case}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    /// Prove native record values require `Copy` while qualified raw pointers can designate non-`Copy` records.
    #[test]
    fn native_record_inputs_require_copy_but_pointers_do_not() {
        /// Demonstrate that raw pointer admission does not require copying a whole native record.
        #[repr(C)]
        struct NonCopyRecord(
            /// Provide the scalar payload used to distinguish whole-record copying from pointer admission.
            u32,
        );
        /// Register this fixture record as compiler-described native storage independently of `Copy`.
        impl NativeRecord for NonCopyRecord {}

        /// Provide a native record that is also admitted as an initialized expression value.
        #[derive(Clone, Copy)]
        #[repr(C)]
        struct CopyRecord(
            /// Provide the scalar payload used to distinguish whole-record copying from pointer admission.
            u32,
        );
        /// Register this fixture record as compiler-described native storage independently of `Copy`.
        impl NativeRecord for CopyRecord {}

        let mut value = NonCopyRecord(7);
        let mutable: Pointer<CRecord<NonCopyRecord>> = input(&raw mut value);
        let readonly: Pointer<CRecord<NonCopyRecord>, ReadOnly> = input(&raw const value);
        assert_eq!(mutable.get().cast_const(), readonly.get());
        let copied: RecordValue<CopyRecord> = input(CopyRecord(11));
        assert_eq!(copied.get().0, 11);
    }

    /// Require Rust compilation to reject unsafe accesses, qualifier removal, and incompatible operand identities.
    #[test]
    fn access_qualification_and_explicit_unsafe_boundaries_are_checked_by_rustc() {
        let directory = OracleDirectory::new();
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let support = if manifest.ends_with("pgrx-pg-sys") {
            manifest.join("src/c_macros/support.rs")
        } else {
            manifest.join("../pgrx-pg-sys/src/c_macros/support.rs")
        }
        .canonicalize()
        .expect("locate actual shared C expression support");
        for (case, source, code) in [
            (
                "unsafe_load",
                "let mut v=1_i32; let p=expression::place::<CInt>(&raw mut v); let _=expression::load(p);",
                "E0133",
            ),
            (
                "unsafe_store",
                "let mut v=1_i32; let p=expression::place::<CInt>(&raw mut v); let _=expression::assign(p,expression::input(2_i32));",
                "E0133",
            ),
            (
                "const_store",
                "let v=1_i32; let p=expression::const_place::<CInt>(&raw const v); unsafe { let _=expression::assign(p,expression::input(2_i32)); }",
                "E0277",
            ),
            (
                "native_deref_const_array_store",
                "use expression::NativeDerefRead; let v=[1_i32,2]; let raw=&raw const v; let place=(&raw).__pgrx_c_read_deref_place(); unsafe { let pointer=expression::load(place); let _=expression::assign(expression::index(expression::input(0_i32),pointer),expression::input(3_i32)); }",
                "E0277",
            ),
            (
                "native_deref_shared_reference_store",
                "use expression::NativeDerefRead; let mut v=1_i32; let reference=&mut v; let place=(&reference).__pgrx_c_read_deref_place(); unsafe { let _=expression::assign(place,expression::input(3_i32)); }",
                "E0277",
            ),
            ("ambiguous_rank", "let _=expression::input(1_u64);", "E0277"),
            (
                "non_copy_native_record_input",
                "#[repr(C)] struct R(u32); impl expression::NativeRecord for R{} let _=expression::input(R(1));",
                "E0277",
            ),
            (
                "copy_opaque_record_input",
                "#[derive(Clone,Copy)] struct R; impl sealed::Sealed for R{} impl expression::NativeType for R{type Marker=expression::COpaque<Self>;} let _=expression::input(R);",
                "E0277",
            ),
            (
                "unverified_float_profile",
                "let _=expression::profile_input::<false,_>(1.0_f64);",
                "E0277",
            ),
            (
                "array_store",
                "let mut array=[1_i32,2]; let p=expression::native_place(&raw mut array); unsafe { let _=expression::assign(p,expression::input(array)); }",
                "E0277",
            ),
            (
                "bitfield_sizeof",
                "#[derive(Clone,Copy)]struct Bitfield; unsafe impl expression::ReadPlace for Bitfield {type Object=CUnsignedInt;type Type=CInt;unsafe fn load(self)->CValue<CInt>{CValue::new(1)}} let _=expression::size_of_place_type(Some(Bitfield));",
                "E0277",
            ),
            (
                "pointer_to_integer_implicit",
                "let p=expression::Pointer::<CInt>::new(core::ptr::null_mut()); let _=expression::implicit::<CUnsignedChar,_>(p);",
                "E0277",
            ),
            (
                "pointer_to_integer_assignment",
                "let mut byte=0_u8; let p=expression::Pointer::<CInt>::new(core::ptr::null_mut()); unsafe { let _=expression::assign(expression::native_place(&raw mut byte),p); }",
                "E0277",
            ),
            (
                "const_pointer_implicit_drop",
                "let p=expression::Pointer::<CInt,expression::ReadOnly>::new(core::ptr::null()); let _=expression::implicit::<expression::CPointer<CInt>,_>(p);",
                "E0277",
            ),
            (
                "volatile_pointer_implicit_drop",
                "let p=expression::Pointer::<expression::CVolatile<CInt>>::new(core::ptr::null_mut()); let _=expression::implicit::<expression::CPointer<CInt>,_>(p);",
                "E0277",
            ),
            (
                "nested_pointer_implicit_qualification",
                "let p=expression::Pointer::<expression::CPointer<CInt>>::new(core::ptr::null_mut()); let _=expression::implicit::<expression::CPointer<expression::CConstPointer<CInt>>,_>(p);",
                "E0277",
            ),
            (
                "distinct_pointer_c_rank",
                "let p=expression::Pointer::<CLong>::new(core::ptr::null_mut()); let _=expression::implicit::<expression::CPointer<CLongLong>,_>(p);",
                "E0277",
            ),
            (
                "unrelated_record_pointers",
                "struct A;struct B; let p=expression::Pointer::<expression::CRecord<A>>::new(core::ptr::null_mut()); let _=expression::implicit::<expression::CPointer<expression::CRecord<B>>,_>(p);",
                "E0277",
            ),
            (
                "function_address_to_void_implicit",
                "#[derive(Clone,Copy)]struct Sig;impl sealed::Sealed for Sig {} impl expression::FunctionSignature for Sig {type Pointer=Option<unsafe extern \"C-unwind\" fn(i32)->i32>;fn null()->Self::Pointer{None}fn address(p:Self::Pointer)->*const(){p.map_or(core::ptr::null(),|f|f as *const())}} let f=expression::FunctionValue::<Sig>::new(None);let _=expression::implicit::<expression::CPointer<expression::CVoid>,_>(f);",
                "E0277",
            ),
            (
                "void_to_function_address_implicit",
                "#[derive(Clone,Copy)]struct Sig;impl sealed::Sealed for Sig {} impl expression::FunctionSignature for Sig {type Pointer=Option<unsafe extern \"C-unwind\" fn(i32)->i32>;fn null()->Self::Pointer{None}fn address(p:Self::Pointer)->*const(){p.map_or(core::ptr::null(),|f|f as *const())}} let p=expression::Pointer::<expression::CVoid>::new(core::ptr::null_mut());let _=expression::implicit::<expression::CFunction<Sig>,_>(p);",
                "E0277",
            ),
            (
                "dynamic_zero_to_pointer",
                "let zero=0_i32;let _=expression::implicit::<expression::CPointer<CInt>,_>(expression::input(zero));",
                "E0277",
            ),
            (
                "dynamic_zero_pointer_equality",
                "let zero=0_i32;let p=expression::Pointer::<CInt>::new(core::ptr::null_mut());let _=expression::eq(p,expression::input(zero));",
                "E0277",
            ),
            (
                "null_constant_pointer_ordering",
                "let zero=expression::null_constant(expression::input(0_i32));let p=expression::Pointer::<CInt>::new(core::ptr::null_mut());let _=expression::lt(p,zero);",
                "E0308",
            ),
            (
                "stored_null_constant_is_dynamic",
                "let mut zero=expression::null_constant(expression::input(0_i32));let value=unsafe{expression::load(expression::native_place(&raw mut zero))};let _=expression::implicit::<expression::CPointer<CInt>,_>(value);",
                "E0277",
            ),
            (
                "function_pointer_ordering",
                "#[derive(Clone,Copy)]struct Sig;impl sealed::Sealed for Sig {} impl expression::FunctionSignature for Sig {type Pointer=Option<unsafe extern \"C-unwind\" fn(i32)->i32>;fn null()->Self::Pointer{None}fn address(p:Self::Pointer)->*const(){p.map_or(core::ptr::null(),|f|f as *const())}} let f=expression::FunctionValue::<Sig>::new(None);let _=expression::lt(f,f);",
                "E0277",
            ),
            (
                "function_void_pointer_equality",
                "#[derive(Clone,Copy)]struct Sig;impl sealed::Sealed for Sig {} impl expression::FunctionSignature for Sig {type Pointer=Option<unsafe extern \"C-unwind\" fn(i32)->i32>;fn null()->Self::Pointer{None}fn address(p:Self::Pointer)->*const(){p.map_or(core::ptr::null(),|f|f as *const())}} let f=expression::FunctionValue::<Sig>::new(None);let p=expression::Pointer::<expression::CVoid>::new(core::ptr::null_mut());let _=expression::eq(f,p);",
                "E0277",
            ),
            (
                "incompatible_function_pointer_equality",
                "#[derive(Clone,Copy)]struct A;#[derive(Clone,Copy)]struct B;impl sealed::Sealed for A {} impl sealed::Sealed for B {} impl expression::FunctionSignature for A {type Pointer=Option<unsafe extern \"C-unwind\" fn(i32)->i32>;fn null()->Self::Pointer{None}fn address(p:Self::Pointer)->*const(){p.map_or(core::ptr::null(),|f|f as *const())}} impl expression::FunctionSignature for B {type Pointer=Option<unsafe extern \"C-unwind\" fn(f64)->i32>;fn null()->Self::Pointer{None}fn address(p:Self::Pointer)->*const(){p.map_or(core::ptr::null(),|f|f as *const())}} let a=expression::FunctionValue::<A>::new(None);let b=expression::FunctionValue::<B>::new(None);let _=expression::eq(a,b);",
                "E0277",
            ),
            (
                "unrelated_record_pointer_equality",
                "struct A;struct B;let a=expression::Pointer::<expression::CRecord<A>>::new(core::ptr::null_mut());let b=expression::Pointer::<expression::CRecord<B>>::new(core::ptr::null_mut());let _=expression::eq(a,b);",
                "E0277",
            ),
            (
                "plain_signed_char_pointer_equality",
                "let a=expression::Pointer::<CChar>::new(core::ptr::null_mut());let b=expression::Pointer::<CSignedChar>::new(core::ptr::null_mut());let _=expression::eq(a,b);",
                "E0277",
            ),
            (
                "unrelated_record_pointer_ordering",
                "struct A;struct B;let a=expression::Pointer::<expression::CRecord<A>>::new(core::ptr::null_mut());let b=expression::Pointer::<expression::CRecord<B>>::new(core::ptr::null_mut());let _=expression::lt(a,b);",
                "E0271",
            ),
            (
                "void_pointer_load",
                "let p=expression::native_place(core::ptr::null_mut::<core::ffi::c_void>());let _=unsafe{expression::load(p)};",
                "E0277",
            ),
        ] {
            let program = directory.0.join(format!("{case}.rs"));
            std::fs::write(
                &program,
                format!(
                    "#[path={support:?}] pub mod support; use support::*; fn main(){{{source}}}"
                ),
            )
            .unwrap();
            let output = std::process::Command::new("rustc")
                .args(["--edition=2024", "--emit=metadata", "-A", "dead_code"])
                .arg(&program)
                .arg("-o")
                .arg(directory.0.join(format!("{case}.rmeta")))
                .output()
                .expect("compile C expression boundary fixture");
            assert!(!output.status.success(), "{case} must be rejected by the compiler");
            assert!(
                String::from_utf8_lossy(&output.stderr).contains(code),
                "{case}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
