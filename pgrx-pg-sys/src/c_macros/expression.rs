//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Typed values and raw places used by generated C expression macros.
//!
//! A place is an address, not a Rust reference or a loaded record. Reading and
//! writing it require the caller to establish the original C access contract,
//! together with Rust's allocation, aliasing, alignment and value-validity rules.

use super::{CInteger, CValue, IntoCValue, OverflowPolicy, PromotedInteger, sealed};
use core::marker::PhantomData;

pub mod literal;
pub mod null;
pub use literal::ZeroInteger;
pub mod enumeration;
pub use enumeration::{CEnum, CEnumObject, EnumIdentity, EnumStorage};

/// A compiler-established C identity and its binding storage representation.
pub trait CType: sealed::Sealed + Copy {
    type Storage;
    type Value;
    const VOLATILE: bool = false;
    fn from_storage(value: Self::Storage) -> Self::Value;
    fn into_storage(value: Self::Value) -> Self::Storage;
}

/// A complete C object whose Rust storage has its compiler-verified size.
/// Incomplete records and void have no such capability, even if bindgen gives
/// them a sized placeholder. Function-pointer storage is a complete object.
pub trait CompleteObject: CType {}
impl<K: CInteger> CompleteObject for K {}
impl<K: CInteger> CompleteObject for CStoredInteger<K> {}
impl<K: CInteger, R: IntegerStorage<K>> CompleteObject for CIntegerStorage<K, R> {}
impl<M: CompleteObject> CompleteObject for CVolatile<M> {}
impl<M: CType, Q: Qualifier> CompleteObject for CPointer<M, Q> {}
impl<M: CompleteObject, const N: usize> CompleteObject for CArray<M, N> {}
impl<R> CompleteObject for CRecord<R> {}
impl<S: FunctionSignature> CompleteObject for CFunction<S> {}
impl CompleteObject for CFloat {}
impl CompleteObject for CDouble {}

impl<K: CInteger> CType for K {
    type Storage = K::Repr;
    type Value = CValue<K>;
    fn from_storage(value: Self::Storage) -> Self::Value {
        CValue::new(value)
    }
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
pub struct CBitfield<Base: CInteger, Promoted: PromotedInteger>(PhantomData<(Base, Promoted)>);
impl<Base: CInteger, Promoted: PromotedInteger> sealed::Sealed for CBitfield<Base, Promoted> {}
impl<Base: CInteger, Promoted: PromotedInteger> CInteger for CBitfield<Base, Promoted> {
    type Repr = Base::Repr;
    type Promoted = Promoted;
    type Boundary = Base::Boundary;
    const BITS: u32 = Base::BITS;
    const SIGNED: bool = Base::SIGNED;
    const RANK: u8 = Base::RANK;
    fn encode(value: Self::Repr) -> u128 {
        Base::encode(value)
    }
    fn decode(bits: u128) -> Self::Repr {
        Base::decode(bits)
    }
}

/// The integer type of a compiler-proven zero integer constant expression.
/// Its integer promotion removes the null-constant tag, while sizeof retains
/// the original integer representation. Ordinary runtime integers are untagged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CNullConstant<K: CInteger>(PhantomData<K>);
impl<K: CInteger> sealed::Sealed for CNullConstant<K> {}
impl<K: CInteger> CInteger for CNullConstant<K> {
    type Repr = K::Repr;
    type Promoted = K::Promoted;
    type Boundary = K::Boundary;
    const BITS: u32 = K::BITS;
    const SIGNED: bool = K::SIGNED;
    const RANK: u8 = K::RANK;
    fn encode(value: Self::Repr) -> u128 {
        let bits = K::encode(value);
        assert_eq!(bits, 0, "C null constant must have integer value zero");
        bits
    }
    fn decode(bits: u128) -> Self::Repr {
        assert_eq!(bits, 0, "C null constant must have integer value zero");
        K::decode(bits)
    }
}
pub type NullConstant<K> = CValue<CNullConstant<K>>;

/// Preserve compiler-established null-constant identity until a conversion.
/// Generation must prove the operand is an integer constant expression; a
/// runtime zero test alone does not establish C null-constant semantics.
pub fn null_constant<K: CInteger>(value: CValue<K>) -> NullConstant<K> {
    CNullConstant::<K>::encode(value.get());
    CValue::new(value.get())
}

/// Volatility belongs to the declared object; lvalue conversion removes it.
pub struct CVolatile<M: CType>(PhantomData<M>);
impl<M: CType> Copy for CVolatile<M> {}
impl<M: CType> Clone for CVolatile<M> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<M: CType> sealed::Sealed for CVolatile<M> {}
impl<M: CType> CType for CVolatile<M> {
    type Storage = M::Storage;
    type Value = M::Value;
    const VOLATILE: bool = true;
    fn from_storage(value: Self::Storage) -> Self::Value {
        M::from_storage(value)
    }
    fn into_storage(value: Self::Value) -> Self::Storage {
        M::into_storage(value)
    }
}

/// One evaluated value whose C identity has been retained.
pub trait CExprValue: sealed::Sealed {
    type Marker: CType<Value = Self>;
}
impl<K: CInteger> CExprValue for CValue<K> {
    type Marker = K;
}

/// A tagged integer stored in Rust while retaining its underlying C identity.
pub struct CStoredInteger<K: CInteger>(PhantomData<K>);
impl<K: CInteger> Copy for CStoredInteger<K> {}
impl<K: CInteger> Clone for CStoredInteger<K> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<K: CInteger> sealed::Sealed for CStoredInteger<K> {}
impl<K: CInteger> CType for CStoredInteger<K> {
    type Storage = CValue<K>;
    type Value = CValue<K::Boundary>;
    fn from_storage(value: Self::Storage) -> Self::Value {
        CValue::new(value.get())
    }
    fn into_storage(value: Self::Value) -> Self::Storage {
        CValue::new(value.get())
    }
}

/// A verified binding integer storage representation and its C value identity.
/// Generated adapters must preserve every bit admitted by the binding type.
pub trait IntegerStorage<K: CInteger>: sealed::Sealed + Copy {
    fn decode(self) -> CValue<K>;
    fn encode(value: CValue<K>) -> Self;
}

pub struct CIntegerStorage<K: CInteger, R: IntegerStorage<K>>(PhantomData<(K, R)>);
impl<K: CInteger, R: IntegerStorage<K>> Copy for CIntegerStorage<K, R> {}
impl<K: CInteger, R: IntegerStorage<K>> Clone for CIntegerStorage<K, R> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<K: CInteger, R: IntegerStorage<K>> sealed::Sealed for CIntegerStorage<K, R> {}
impl<K: CInteger, R: IntegerStorage<K>> CType for CIntegerStorage<K, R> {
    type Storage = R;
    type Value = CValue<K>;
    fn from_storage(value: R) -> Self::Value {
        value.decode()
    }
    fn into_storage(value: Self::Value) -> R {
        R::encode(value)
    }
}

impl sealed::Sealed for usize {}
impl sealed::Sealed for isize {}
#[cfg(target_pointer_width = "64")]
macro_rules! pointer_sized_integer_storage {
    ($(($storage:ty, $kind:ty, $repr:ty)),+ $(,)?) => { $(
        impl IntegerStorage<$kind> for $storage {
            fn decode(self) -> CValue<$kind> { CValue::new(self as $repr) }
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
    type Marker: CType<Storage = Self>;
}
impl<K: CInteger> NativeType for CValue<K> {
    type Marker = CStoredInteger<K>;
}

macro_rules! native_integer_types {
    ($(($storage:ty, $kind:ty)),+ $(,)?) => { $(
        impl NativeType for $storage { type Marker = $kind; }
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
    type Value: CExprValue;
    fn into_expression(self) -> Self::Value;
}
impl<K: CInteger> IntoExpression for CValue<K> {
    type Value = CValue<K::Boundary>;
    fn into_expression(self) -> Self::Value {
        CValue::new(self.get())
    }
}
macro_rules! integer_inputs {
    ($($storage:ty),+ $(,)?) => { $(
        impl IntoExpression for $storage {
            type Value = CValue<<Self as IntoCValue>::Kind>;
            fn into_expression(self) -> Self::Value { super::value(self) }
        }
    )+ };
}
integer_inputs!(bool, i8, u8, i16, u16, i32, u32, i128, u128);
pub fn input<T: IntoExpression>(value: T) -> T::Value {
    value.into_expression()
}

/// Permit floating expressions only after the generator proves their C profile.
pub trait AllowedProfile<const FLOATS: bool>: CType {}
impl<K: CInteger, const B: bool> AllowedProfile<B> for K {}
impl<K: CInteger, const B: bool> AllowedProfile<B> for CStoredInteger<K> {}
impl<K: CInteger, R: IntegerStorage<K>, const B: bool> AllowedProfile<B> for CIntegerStorage<K, R> {}
impl<M: AllowedProfile<B>, const B: bool> AllowedProfile<B> for CVolatile<M> {}
impl<M: CType, Q: Qualifier, const B: bool> AllowedProfile<B> for CPointer<M, Q> {}
impl<R, const B: bool> AllowedProfile<B> for CRecord<R> {}
impl<R, const B: bool> AllowedProfile<B> for COpaque<R> {}
impl<M: AllowedProfile<B>, const N: usize, const B: bool> AllowedProfile<B> for CArray<M, N> {}
impl<const B: bool> AllowedProfile<B> for CVoid {}
impl AllowedProfile<true> for CFloat {}
impl AllowedProfile<true> for CDouble {}
pub fn profile_input<const B: bool, T: IntoExpression>(value: T) -> T::Value
where
    <T::Value as CExprValue>::Marker: AllowedProfile<B>,
{
    input(value)
}
pub fn profile_value<const B: bool, V: CExprValue>(value: V) -> V
where
    V::Marker: AllowedProfile<B>,
{
    value
}

#[derive(Clone, Copy, Debug)]
pub struct ReadOnly;
#[derive(Clone, Copy, Debug)]
pub struct ReadWrite;
impl sealed::Sealed for ReadOnly {}
impl sealed::Sealed for ReadWrite {}

/// Pointer qualification is retained independently from its pointee identity.
pub trait Qualifier: sealed::Sealed + Copy {
    type Raw<T>: Copy;
    fn into_mut<T>(pointer: Self::Raw<T>) -> *mut T;
    fn from_mut<T>(pointer: *mut T) -> Self::Raw<T>;
}
impl Qualifier for ReadOnly {
    type Raw<T> = *const T;
    fn into_mut<T>(pointer: *const T) -> *mut T {
        pointer.cast_mut()
    }
    fn from_mut<T>(pointer: *mut T) -> *const T {
        pointer.cast_const()
    }
}
impl Qualifier for ReadWrite {
    type Raw<T> = *mut T;
    fn into_mut<T>(pointer: *mut T) -> *mut T {
        pointer
    }
    fn from_mut<T>(pointer: *mut T) -> *mut T {
        pointer
    }
}

pub struct CPointer<M: CType, Q: Qualifier = ReadWrite>(PhantomData<(M, Q)>);
impl<M: CType, Q: Qualifier> Copy for CPointer<M, Q> {}
impl<M: CType, Q: Qualifier> Clone for CPointer<M, Q> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<M: CType, Q: Qualifier> sealed::Sealed for CPointer<M, Q> {}
pub type CConstPointer<M> = CPointer<M, ReadOnly>;

/// A raw address tagged with its C pointee identity and qualification.
pub struct Pointer<M: CType, Q: Qualifier = ReadWrite> {
    raw: Q::Raw<M::Storage>,
    access: Access,
    marker: PhantomData<(M, Q)>,
}
impl<M: CType, Q: Qualifier> Copy for Pointer<M, Q> {}
impl<M: CType, Q: Qualifier> Clone for Pointer<M, Q> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<M: CType, Q: Qualifier> sealed::Sealed for Pointer<M, Q> {}
impl<M: CType, Q: Qualifier> Pointer<M, Q> {
    /// Tag an address without accessing it or creating a reference.
    pub const fn new(raw: Q::Raw<M::Storage>) -> Self {
        Self {
            raw,
            access: Access { volatile: M::VOLATILE, unaligned: false },
            marker: PhantomData,
        }
    }
    pub const fn get(self) -> Q::Raw<M::Storage> {
        self.raw
    }
    pub fn as_mut_address(self) -> *mut M::Storage {
        Q::into_mut(self.raw)
    }
    /// Retain compiler-established access properties of a projected address.
    /// This describes an access; reading or writing still requires its unsafe
    /// allocation, initialization, aliasing, and alignment contract.
    pub const fn with_access(self, access: Access) -> Self {
        Self { access, ..self }
    }
    pub const fn access(self) -> Access {
        self.access
    }
}
impl<M: CType, Q: Qualifier> CType for CPointer<M, Q> {
    type Storage = Q::Raw<M::Storage>;
    type Value = Pointer<M, Q>;
    fn from_storage(value: Self::Storage) -> Self::Value {
        Pointer::new(value)
    }
    fn into_storage(value: Self::Value) -> Self::Storage {
        value.get()
    }
}
impl<M: CType, Q: Qualifier> CExprValue for Pointer<M, Q> {
    type Marker = CPointer<M, Q>;
}
impl<M: CType, Q: Qualifier> IntoExpression for Pointer<M, Q> {
    type Value = Self;
    fn into_expression(self) -> Self {
        self
    }
}
impl<T: NativeType> sealed::Sealed for *mut T {}
impl<T: NativeType> sealed::Sealed for *const T {}
impl<T: NativeType> IntoExpression for *mut T {
    type Value = Pointer<T::Marker>;
    fn into_expression(self) -> Self::Value {
        Pointer::new(self)
    }
}
impl<T: NativeType> IntoExpression for *const T {
    type Value = Pointer<T::Marker, ReadOnly>;
    fn into_expression(self) -> Self::Value {
        Pointer::new(self)
    }
}
impl<T: NativeType> NativeType for *mut T {
    type Marker = CPointer<T::Marker>;
}
impl<T: NativeType> NativeType for *const T {
    type Marker = CConstPointer<T::Marker>;
}

/// An exact compiler/bindgen-verified native function-pointer signature.
/// Generated markers retain C argument/result identities even if Rust storage
/// aliases erase differences between equal-width C integer types.
pub trait FunctionSignature: sealed::Sealed + Copy {
    type Pointer: Copy;
    /// Construct the exact binding representation of a null function pointer.
    fn null() -> Self::Pointer;
    /// Obtain the actual native function address, mapping C null to a null pointer.
    /// A guarded Rust wrapper function is not the original C ABI function address.
    fn address(pointer: Self::Pointer) -> *const ();
}

/// A generated exact-signature adapter for an indirect native C call.
/// Conversion and null checks occur before entering the PostgreSQL error guard;
/// only Copy ABI storage belongs inside that guard, with result decoding after.
pub trait Call<Args>: FunctionSignature {
    type Output: CExprValue;
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
pub struct CFunction<S: FunctionSignature>(PhantomData<S>);
impl<S: FunctionSignature> Copy for CFunction<S> {}
impl<S: FunctionSignature> Clone for CFunction<S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<S: FunctionSignature> sealed::Sealed for CFunction<S> {}
pub struct FunctionValue<S: FunctionSignature>(S::Pointer);
impl<S: FunctionSignature> Copy for FunctionValue<S> {}
impl<S: FunctionSignature> Clone for FunctionValue<S> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<S: FunctionSignature> sealed::Sealed for FunctionValue<S> {}
impl<S: FunctionSignature> FunctionValue<S> {
    pub const fn new(pointer: S::Pointer) -> Self {
        Self(pointer)
    }
    pub const fn get(self) -> S::Pointer {
        self.0
    }
    pub fn address(self) -> *const () {
        S::address(self.0)
    }
}
impl<S: FunctionSignature> CType for CFunction<S> {
    type Storage = S::Pointer;
    type Value = FunctionValue<S>;
    fn from_storage(pointer: Self::Storage) -> Self::Value {
        FunctionValue::new(pointer)
    }
    fn into_storage(value: Self::Value) -> Self::Storage {
        value.get()
    }
}
impl<S: FunctionSignature> CExprValue for FunctionValue<S> {
    type Marker = CFunction<S>;
}
impl<S: FunctionSignature> IntoExpression for FunctionValue<S> {
    type Value = Self;
    fn into_expression(self) -> Self {
        self
    }
}
impl<S: FunctionSignature> ScalarObject for CFunction<S> {
    type Canonical = Self;
}
impl<S: FunctionSignature, const B: bool> AllowedProfile<B> for CFunction<S> {}
impl<S: FunctionSignature> Truth for FunctionValue<S> {
    fn truth(self) -> bool {
        !self.address().is_null()
    }
}
impl<S: FunctionSignature> CastTo<CFunction<S>> for FunctionValue<S> {
    fn cast_to(self) -> Self {
        self
    }
}
impl<S: FunctionSignature, K: CInteger> CastTo<K> for FunctionValue<S> {
    fn cast_to(self) -> CValue<K> {
        super::cast(CValue::<super::CUnsignedLong>::new(self.address().expose_provenance() as u64))
    }
}
impl<S: FunctionSignature, M: CType, Q: Qualifier> CastTo<CPointer<M, Q>> for FunctionValue<S> {
    fn cast_to(self) -> Pointer<M, Q> {
        Pointer::new(Q::from_mut(self.address().cast_mut().cast()))
    }
}

/// A fixed array retains its complete storage identity until C array decay.
pub struct CArray<M: CType, const N: usize>(PhantomData<M>);
impl<M: CType, const N: usize> Copy for CArray<M, N> {}
impl<M: CType, const N: usize> Clone for CArray<M, N> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<M: CType, const N: usize> sealed::Sealed for CArray<M, N> {}
pub struct ArrayValue<M: CType, const N: usize>([M::Storage; N]);
impl<M: CType<Storage: Copy>, const N: usize> Copy for ArrayValue<M, N> {}
impl<M: CType<Storage: Copy>, const N: usize> Clone for ArrayValue<M, N> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<M: CType, const N: usize> sealed::Sealed for ArrayValue<M, N> {}
impl<M: CType, const N: usize> ArrayValue<M, N> {
    pub const fn new(value: [M::Storage; N]) -> Self {
        Self(value)
    }
    pub fn get(self) -> [M::Storage; N] {
        self.0
    }
}
impl<M: CType, const N: usize> CType for CArray<M, N> {
    type Storage = [M::Storage; N];
    type Value = ArrayValue<M, N>;
    fn from_storage(value: Self::Storage) -> Self::Value {
        ArrayValue::new(value)
    }
    fn into_storage(value: Self::Value) -> Self::Storage {
        value.get()
    }
}
impl<M: CType<Storage: Copy>, const N: usize> CExprValue for ArrayValue<M, N> {
    type Marker = CArray<M, N>;
}
impl<M: CType<Storage: Copy>, const N: usize> IntoExpression for ArrayValue<M, N> {
    type Value = Self;
    fn into_expression(self) -> Self {
        self
    }
}
impl<T: NativeType, const N: usize> sealed::Sealed for [T; N] {}
impl<T: NativeType, const N: usize> NativeType for [T; N] {
    type Marker = CArray<T::Marker, N>;
}
impl<T: NativeType + Copy, const N: usize> IntoExpression for [T; N] {
    type Value = ArrayValue<T::Marker, N>;
    fn into_expression(self) -> Self::Value {
        ArrayValue::new(self)
    }
}

/// A record marker remains available even when no whole-record load is valid.
pub struct CRecord<R>(PhantomData<R>);
impl<R> Copy for CRecord<R> {}
impl<R> Clone for CRecord<R> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<R> sealed::Sealed for CRecord<R> {}
pub struct RecordValue<R>(R);
impl<R: Copy> Copy for RecordValue<R> {}
impl<R: Copy> Clone for RecordValue<R> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<R> sealed::Sealed for RecordValue<R> {}
impl<R> RecordValue<R> {
    pub const fn new(value: R) -> Self {
        Self(value)
    }
    pub fn get(self) -> R {
        self.0
    }
}
impl<R> CType for CRecord<R> {
    type Storage = R;
    type Value = RecordValue<R>;
    fn from_storage(value: R) -> Self::Value {
        RecordValue::new(value)
    }
    fn into_storage(value: Self::Value) -> R {
        value.get()
    }
}
impl<R: Copy> CExprValue for RecordValue<R> {
    type Marker = CRecord<R>;
}
impl<R: Copy> IntoExpression for RecordValue<R> {
    type Value = Self;
    fn into_expression(self) -> Self {
        self
    }
}

pub mod record;
pub use record::{CRawRecord, RawRecordValue, RecordExpression};

/// The binding placeholder for a compiler-proven incomplete record.
/// It may occur behind a pointer, but cannot be loaded, sized, or indexed.
pub struct COpaque<R>(PhantomData<R>);
impl<R> Copy for COpaque<R> {}
impl<R> Clone for COpaque<R> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<R> sealed::Sealed for COpaque<R> {}
impl<R> CType for COpaque<R> {
    type Storage = R;
    type Value = ();
    fn from_storage(_: R) {
        panic!("an incomplete C record cannot be loaded")
    }
    fn into_storage(_: ()) -> R {
        panic!("an incomplete C record has no storage value")
    }
}

/// A C flexible array member in its verified binding wrapper storage.
/// Expression conversion exposes its first element address without reading a
/// Rust zero-length placeholder or copying any partially initialized elements.
pub struct CFlexibleArray<M: CType, R>(PhantomData<(M, R)>);
impl<M: CType, R> Copy for CFlexibleArray<M, R> {}
impl<M: CType, R> Clone for CFlexibleArray<M, R> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<M: CType, R> sealed::Sealed for CFlexibleArray<M, R> {}
impl<M: CType, R> CType for CFlexibleArray<M, R> {
    type Storage = R;
    type Value = ();
    fn from_storage(_: R) {
        panic!("a C flexible array does not have a whole storage value")
    }
    fn into_storage(_: ()) -> R {
        panic!("a C flexible array does not have a whole storage value")
    }
}
impl<M: AllowedProfile<B>, R, const B: bool> AllowedProfile<B> for CFlexibleArray<M, R> {}

/// `void` is never loaded; this byte storage exists only for void-pointer casts.
#[derive(Clone, Copy, Debug)]
pub struct CVoid;
impl sealed::Sealed for CVoid {}
impl sealed::Sealed for core::ffi::c_void {}
impl NativeType for core::ffi::c_void {
    type Marker = CVoid;
}
impl CType for CVoid {
    type Storage = core::ffi::c_void;
    type Value = ();
    fn from_storage(_: Self::Storage) {}
    fn into_storage(_: ()) -> Self::Storage {
        panic!("C void has no storage value")
    }
}
impl sealed::Sealed for () {}
impl CExprValue for () {
    type Marker = CVoid;
}
impl IntoExpression for () {
    type Value = Self;
    fn into_expression(self) -> Self {}
}

/// Qualifiers and packing are properties of a C place, not its loaded value.
#[derive(Clone, Copy, Debug, Default)]
pub struct Access {
    pub volatile: bool,
    pub unaligned: bool,
}

/// An address with a C storage identity and access qualification.
pub struct Place<M: CType, Q: Qualifier = ReadWrite> {
    address: *mut M::Storage,
    access: Access,
    marker: PhantomData<(M, Q)>,
}
impl<M: CType, Q: Qualifier> Copy for Place<M, Q> {}
impl<M: CType, Q: Qualifier> Clone for Place<M, Q> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<M: CType, Q: Qualifier> Place<M, Q> {
    /// Describe an address; no read, write, reference or validity check occurs.
    pub fn new(address: Q::Raw<M::Storage>, access: Access) -> Self {
        Self { address: Q::into_mut(address), access, marker: PhantomData }
    }
    pub fn pointer(self) -> Pointer<M, Q> {
        Pointer::new(Q::from_mut(self.address)).with_access(self.access)
    }
    pub fn access(self) -> Access {
        self.access
    }
    /// Change access metadata established by the compiler's record layout.
    pub fn with_access(self, access: Access) -> Self {
        Self { access, ..self }
    }
}
pub fn place<M: CType>(address: *mut M::Storage) -> Place<M> {
    Place::new(address, Access { volatile: M::VOLATILE, unaligned: false })
}
pub fn const_place<M: CType>(address: *const M::Storage) -> Place<M, ReadOnly> {
    Place::new(address, Access { volatile: M::VOLATILE, unaligned: false })
}
pub fn native_place<T: NativeType>(address: *mut T) -> Place<T::Marker> {
    place(address)
}
pub fn native_const_place<T: NativeType>(address: *const T) -> Place<T::Marker, ReadOnly> {
    const_place(address)
}
pub fn pointee<M: CType, Q: Qualifier>(pointer: Pointer<M, Q>) -> Place<M, Q> {
    Place::new(pointer.get(), pointer.access())
}
/// Addressable objects and function designators retain different C identities.
pub trait Address: Copy {
    type Output: CExprValue;
    fn address(self) -> Self::Output;
}
impl<M: CType, Q: Qualifier> Address for Place<M, Q> {
    type Output = Pointer<M, Q>;
    fn address(self) -> Self::Output {
        self.pointer()
    }
}
impl<S: FunctionSignature> Address for FunctionValue<S> {
    type Output = Self;
    fn address(self) -> Self {
        self
    }
}
pub fn address<P: Address>(place: P) -> P::Output {
    Address::address(place)
}

/// C dereference reads an object but only designates an indirect function.
pub trait DereferenceValue: CExprValue {
    type Output: CExprValue;
    /// # Safety
    /// Object pointers must satisfy the target place's initialized-read
    /// contract. Function pointers are merely designated, without an invocation.
    unsafe fn dereference(self) -> Self::Output;
}
impl<M: ReadObject<Q>, Q: Qualifier> DereferenceValue for Pointer<M, Q>
where
    <M::RValue as CType>::Value: CExprValue,
{
    type Output = <M::RValue as CType>::Value;
    unsafe fn dereference(self) -> Self::Output {
        // SAFETY: The caller establishes this declared object's read contract.
        unsafe { load(pointee(self)) }
    }
}
impl<S: FunctionSignature> DereferenceValue for FunctionValue<S> {
    type Output = Self;
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
    type Output: CExprValue;
    fn dereference_address(self) -> Self::Output;
}
impl<M: CType, Q: Qualifier> DereferenceAddress for Pointer<M, Q> {
    type Output = Self;
    fn dereference_address(self) -> Self {
        self
    }
}
impl<S: FunctionSignature> DereferenceAddress for FunctionValue<S> {
    type Output = Self;
    fn dereference_address(self) -> Self {
        self
    }
}
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
    type Object: CType;
    type Type: CType;
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
    type Canonical: CType<Value = Self::Value, Storage: Copy>;
}
impl<K: CInteger> ScalarObject for K {
    type Canonical = K;
}
impl<K: CInteger> ScalarObject for CStoredInteger<K> {
    type Canonical = K::Boundary;
}
impl<K: CInteger, R: IntegerStorage<K>> ScalarObject for CIntegerStorage<K, R> {
    type Canonical = K;
}
impl<M: CType, Q: Qualifier> ScalarObject for CPointer<M, Q> {
    type Canonical = Self;
}
impl<M: ScalarObject> ScalarObject for CVolatile<M> {
    type Canonical = M::Canonical;
}

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
    type RValue: CType;
    /// # Safety
    /// Scalar loads require initialized valid storage. Raw aggregate copies may
    /// contain uninitialized fields. Array decay requires only the declared
    /// live address/allocation contract, without element reads.
    unsafe fn read_object(place: Place<Self, Q>) -> <Self::RValue as CType>::Value;
}
// SAFETY: ScalarObject establishes Copy storage and a value-preserving canonical
// conversion; read_scalar performs exactly the metadata-selected access.
unsafe impl<M: ScalarObject, Q: Qualifier> ReadObject<Q> for M {
    type RValue = M::Canonical;
    unsafe fn read_object(place: Place<Self, Q>) -> <Self::RValue as CType>::Value {
        // SAFETY: The caller establishes this scalar object's load contract.
        unsafe { read_scalar(place) }
    }
}
// SAFETY: An array's expression conversion returns its original first-element
// address. No element or aggregate storage is read.
unsafe impl<M: CType, const N: usize, Q: Qualifier> ReadObject<Q> for CArray<M, N> {
    type RValue = CPointer<M, Q>;
    unsafe fn read_object(place: Place<Self, Q>) -> Pointer<M, Q> {
        decay(place)
    }
}
// SAFETY: Volatile array qualification is transferred to the pointee marker;
// decay does not itself read any volatile element.
unsafe impl<M: CType, const N: usize, Q: Qualifier> ReadObject<Q> for CVolatile<CArray<M, N>> {
    type RValue = CPointer<CVolatile<M>, Q>;
    unsafe fn read_object(place: Place<Self, Q>) -> Pointer<CVolatile<M>, Q> {
        Pointer::new(Q::from_mut(place.address.cast()))
            .with_access(Access { volatile: true, ..place.access() })
    }
}

// SAFETY: Generation proves the wrapper starts at the declared flexible-array
// address with the element's alignment. Only that address is converted; no
// object representation or element is read.
unsafe impl<M: CType, R, Q: Qualifier> ReadObject<Q> for CFlexibleArray<M, R> {
    type RValue = CPointer<M, Q>;
    unsafe fn read_object(place: Place<Self, Q>) -> Pointer<M, Q> {
        Pointer::new(Q::from_mut(place.address.cast())).with_access(place.access())
    }
}
// SAFETY: As above; volatile array decay transfers the qualifier without an
// element read or any access through a Rust reference.
unsafe impl<M: CType, R, Q: Qualifier> ReadObject<Q> for CVolatile<CFlexibleArray<M, R>> {
    type RValue = CPointer<CVolatile<M>, Q>;
    unsafe fn read_object(place: Place<Self, Q>) -> Pointer<CVolatile<M>, Q> {
        Pointer::new(Q::from_mut(place.address.cast()))
            .with_access(Access { volatile: true, ..place.access() })
    }
}

// SAFETY: ReadObject owns exact object-to-value conversion and its access contract.
unsafe impl<M: ReadObject<Q>, Q: Qualifier> ReadPlace for Place<M, Q> {
    type Object = M;
    type Type = M::RValue;
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
    type Assignment: CType;
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
unsafe impl<M: ScalarObject> WritePlace for Place<M> {
    type Assignment = M::Canonical;
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
    type Object: CompleteObject;
}
impl<M: CompleteObject, Q: Qualifier> SizeablePlace for Place<M, Q> {
    type Object = M;
}

/// A field offset verified against the compiler's layout and actual Rust bindings.
/// Unlike a field projection, this capability never accesses object storage.
pub trait OffsetField<F>: CType {
    /// A verified nested record marker, or `()` when the path cannot continue.
    /// Leaf fields need no supported value representation to have an offset.
    type Member;
    const OFFSET: usize;
}

impl<F, M: OffsetField<F>> OffsetField<F> for CVolatile<M> {
    type Member = M::Member;
    const OFFSET: usize = <M as OffsetField<F>>::OFFSET;
}

/// End of a compiler-verified field path.
pub struct OffsetEnd;

/// One field followed by the remaining field path.
pub struct OffsetStep<F, Tail>(PhantomData<(F, Tail)>);

/// Compose field offsets without constructing an address or evaluating an operand.
pub trait OffsetPath<P> {
    const OFFSET: usize;
    const DEPTH: usize;
}

impl<M> OffsetPath<OffsetEnd> for M {
    const OFFSET: usize = 0;
    const DEPTH: usize = 0;
}

impl<M: OffsetField<F>, F, Tail> OffsetPath<OffsetStep<F, Tail>> for M
where
    M::Member: OffsetPath<Tail>,
{
    const DEPTH: usize = match <M::Member as OffsetPath<Tail>>::DEPTH.checked_add(1) {
        Some(depth) => depth,
        None => panic!("C offsetof path exceeds the 64-field bound"),
    };
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
    type Qualified: Copy;
    fn qualify_volatile(self) -> Self::Qualified;
}
impl<M: CType, Q: Qualifier> VolatilePlace for Place<M, Q> {
    type Qualified = Place<CVolatile<M>, Q>;
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
    type Output: Copy;
    /// # Safety
    /// The base must permit an in-bounds projection to the declared field; only
    /// the accessed field needs initialization when the returned place is loaded.
    unsafe fn project(place: Place<Self, Q>) -> Self::Output;
}
// SAFETY: Retagging the base preserves storage and access metadata. The underlying
// Field contract requires its projection to preserve inherited volatile access.
unsafe impl<F, M: Field<F, Q>, Q: Qualifier> Field<F, Q> for CVolatile<M>
where
    M::Output: VolatilePlace,
{
    type Output = <M::Output as VolatilePlace>::Qualified;
    unsafe fn project(base: Place<Self, Q>) -> Self::Output {
        let mut access = base.access();
        access.volatile = true;
        let base = Place::<M, Q>::new(Q::from_mut(base.address), access);
        // SAFETY: The caller's base allocation contract is unchanged by retagging.
        unsafe { M::project(base) }.qualify_volatile()
    }
}

/// # Safety
/// The base address must satisfy the field projection's allocation contract.
pub unsafe fn project<F, M: Field<F, Q>, Q: Qualifier>(place: Place<M, Q>) -> M::Output {
    // SAFETY: The caller establishes the base projection contract; the generated
    // Field implementation establishes its exact layout and access qualifiers.
    unsafe { M::project(place) }
}

/// Read a field from a fully initialized, copied C record value.
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

/// # Safety
/// The place must satisfy the initialized read contract of ReadPlace::load.
pub unsafe fn load<P: ReadPlace>(place: P) -> <P::Type as CType>::Value {
    // SAFETY: The caller establishes this place's complete load contract.
    unsafe { place.load() }
}

/// An explicit C conversion whose target identity is established by analysis.
pub trait CastTo<M: CType>: CExprValue {
    fn cast_to(self) -> M::Value;
}
impl<M: CType, V: CastTo<M>> CastTo<CVolatile<M>> for V {
    fn cast_to(self) -> M::Value {
        <V as CastTo<M>>::cast_to(self)
    }
}
impl<K: CInteger, L: CInteger> CastTo<K> for CValue<L> {
    fn cast_to(self) -> CValue<K> {
        super::cast(self)
    }
}
impl<K: CInteger, L: CInteger> CastTo<CStoredInteger<K>> for CValue<L> {
    fn cast_to(self) -> CValue<K::Boundary> {
        super::cast(self)
    }
}
impl<K: CInteger, R: IntegerStorage<K>, V: CastTo<K>> CastTo<CIntegerStorage<K, R>> for V {
    fn cast_to(self) -> CValue<K> {
        <V as CastTo<K>>::cast_to(self)
    }
}
impl<V: CExprValue + Copy> CastTo<CVoid> for V {
    fn cast_to(self) {
        let _ = self;
    }
}
impl<M: CType, Q: Qualifier, N: CType, R: Qualifier> CastTo<CPointer<N, R>> for Pointer<M, Q> {
    fn cast_to(self) -> Pointer<N, R> {
        Pointer::new(R::from_mut(self.as_mut_address().cast()))
            .with_access(Access { volatile: N::VOLATILE, ..self.access() })
    }
}
impl<K: CInteger, M: CType, Q: Qualifier> CastTo<K> for Pointer<M, Q> {
    fn cast_to(self) -> CValue<K> {
        super::cast(CValue::<super::CUnsignedLong>::new(
            self.as_mut_address().expose_provenance() as u64
        ))
    }
}
impl<K: CInteger, M: CType, Q: Qualifier> CastTo<CPointer<M, Q>> for CValue<K> {
    fn cast_to(self) -> Pointer<M, Q> {
        let address = super::cast::<super::CUnsignedLong, _>(self).get() as usize;
        Pointer::new(Q::from_mut(core::ptr::with_exposed_provenance_mut(address)))
    }
}
impl<K: ZeroInteger, S: FunctionSignature> CastTo<CFunction<S>> for CValue<K> {
    fn cast_to(self) -> FunctionValue<S> {
        K::verify_zero(self.get());
        FunctionValue::new(S::null())
    }
}
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
    fn implicit_to(self) -> M::Value;
}
pub fn implicit<M: CType, V: ImplicitTo<M>>(value: V) -> M::Value {
    value.implicit_to()
}
impl<K: CInteger, L: CInteger> ImplicitTo<K> for CValue<L> {
    fn implicit_to(self) -> CValue<K> {
        cast::<K, _>(self)
    }
}
impl<M: CType, V: ImplicitTo<M>> ImplicitTo<CVolatile<M>> for V {
    fn implicit_to(self) -> M::Value {
        <V as ImplicitTo<M>>::implicit_to(self)
    }
}
impl<K: CInteger, V: ImplicitTo<K::Boundary>> ImplicitTo<CStoredInteger<K>> for V {
    fn implicit_to(self) -> CValue<K::Boundary> {
        <V as ImplicitTo<K::Boundary>>::implicit_to(self)
    }
}
impl<K: CInteger, R: IntegerStorage<K>, V: ImplicitTo<K>> ImplicitTo<CIntegerStorage<K, R>> for V {
    fn implicit_to(self) -> CValue<K> {
        <V as ImplicitTo<K>>::implicit_to(self)
    }
}
impl<R: Copy> ImplicitTo<CRecord<R>> for RecordValue<R> {
    fn implicit_to(self) -> Self {
        self
    }
}
impl<S: FunctionSignature> ImplicitTo<CFunction<S>> for FunctionValue<S> {
    fn implicit_to(self) -> Self {
        self
    }
}
impl<K: ZeroInteger, M: CType, Q: Qualifier> ImplicitTo<CPointer<M, Q>> for CValue<K> {
    fn implicit_to(self) -> Pointer<M, Q> {
        K::verify_zero(self.get());
        Pointer::new(Q::from_mut(core::ptr::null_mut()))
    }
}
impl<K: ZeroInteger, S: FunctionSignature> ImplicitTo<CFunction<S>> for CValue<K> {
    fn implicit_to(self) -> FunctionValue<S> {
        cast::<CFunction<S>, _>(self)
    }
}
impl<M: CType, Q: Qualifier> ImplicitTo<super::CBool> for Pointer<M, Q> {
    fn implicit_to(self) -> CValue<super::CBool> {
        CValue::new(!self.as_mut_address().is_null())
    }
}
impl<S: FunctionSignature> ImplicitTo<super::CBool> for FunctionValue<S> {
    fn implicit_to(self) -> CValue<super::CBool> {
        CValue::new(!self.address().is_null())
    }
}

/// Type identities used only to check C pointer compatibility.
pub struct PointerIdentity<I, V, Q>(PhantomData<(I, V, Q)>);
pub struct ArrayIdentity<I, V, const N: usize>(PhantomData<(I, V)>);
pub struct IncompleteArrayIdentity<I, V>(PhantomData<(I, V)>);
pub struct NonVolatile;
pub struct Volatile;
/// A non-void pointee storage object that can be erased through C void pointers.
/// CFunction stores a function-pointer object; FunctionValue itself denotes the
/// function address and deliberately has no implicit void-pointer conversion.
pub trait NonVoidIdentity {}
impl<K: CInteger> NonVoidIdentity for K {}
impl<R> NonVoidIdentity for CRecord<R> {}
impl<R> NonVoidIdentity for COpaque<R> {}
impl NonVoidIdentity for CFloat {}
impl NonVoidIdentity for CDouble {}
impl<S: FunctionSignature> NonVoidIdentity for CFunction<S> {}
impl<I, V, Q> NonVoidIdentity for PointerIdentity<I, V, Q> {}
impl<I, V, const N: usize> NonVoidIdentity for ArrayIdentity<I, V, N> {}
impl<I, V> NonVoidIdentity for IncompleteArrayIdentity<I, V> {}
pub trait CompatibleIdentity<Target> {}
impl<I> CompatibleIdentity<I> for I {}
impl<I: NonVoidIdentity> CompatibleIdentity<CVoid> for I {}
impl<I: NonVoidIdentity> CompatibleIdentity<I> for CVoid {}
impl<I, V, const N: usize> CompatibleIdentity<IncompleteArrayIdentity<I, V>>
    for ArrayIdentity<I, V, N>
{
}
impl<I, V, const N: usize> CompatibleIdentity<ArrayIdentity<I, V, N>>
    for IncompleteArrayIdentity<I, V>
{
}
pub trait AddVolatile<Target> {}
impl AddVolatile<NonVolatile> for NonVolatile {}
impl AddVolatile<Volatile> for NonVolatile {}
impl AddVolatile<Volatile> for Volatile {}
pub trait AddConst<Target: Qualifier>: Qualifier {}
impl AddConst<ReadWrite> for ReadWrite {}
impl AddConst<ReadOnly> for ReadWrite {}
impl AddConst<ReadOnly> for ReadOnly {}
pub trait PointeeIdentity: CType {
    type Identity;
    type Volatility;
}
impl<K: CInteger> PointeeIdentity for K {
    type Identity = K;
    type Volatility = NonVolatile;
}
impl<K: CInteger> PointeeIdentity for CStoredInteger<K> {
    type Identity = K::Boundary;
    type Volatility = NonVolatile;
}
impl<K: CInteger, R: IntegerStorage<K>> PointeeIdentity for CIntegerStorage<K, R> {
    type Identity = K;
    type Volatility = NonVolatile;
}
impl<R> PointeeIdentity for CRecord<R> {
    type Identity = Self;
    type Volatility = NonVolatile;
}
impl<R> PointeeIdentity for COpaque<R> {
    type Identity = Self;
    type Volatility = NonVolatile;
}
impl PointeeIdentity for CVoid {
    type Identity = Self;
    type Volatility = NonVolatile;
}
impl PointeeIdentity for CFloat {
    type Identity = Self;
    type Volatility = NonVolatile;
}
impl PointeeIdentity for CDouble {
    type Identity = Self;
    type Volatility = NonVolatile;
}
impl<S: FunctionSignature> PointeeIdentity for CFunction<S> {
    type Identity = Self;
    type Volatility = NonVolatile;
}
impl<M: PointeeIdentity> PointeeIdentity for CVolatile<M> {
    type Identity = M::Identity;
    type Volatility = Volatile;
}
impl<M: PointeeIdentity, Q: Qualifier> PointeeIdentity for CPointer<M, Q> {
    type Identity = PointerIdentity<M::Identity, M::Volatility, Q>;
    type Volatility = NonVolatile;
}
impl<M: PointeeIdentity, const N: usize> PointeeIdentity for CArray<M, N> {
    type Identity = ArrayIdentity<M::Identity, M::Volatility, N>;
    type Volatility = NonVolatile;
}
impl<M: PointeeIdentity, R> PointeeIdentity for CFlexibleArray<M, R> {
    type Identity = IncompleteArrayIdentity<M::Identity, M::Volatility>;
    type Volatility = NonVolatile;
}
impl<M: PointeeIdentity, Q: AddConst<R>, N: PointeeIdentity, R: Qualifier>
    ImplicitTo<CPointer<N, R>> for Pointer<M, Q>
where
    M::Identity: CompatibleIdentity<N::Identity>,
    M::Volatility: AddVolatile<N::Volatility>,
{
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

#[derive(Clone, Copy, Debug)]
pub struct CFloat;
#[derive(Clone, Copy, Debug)]
pub struct CDouble;
impl sealed::Sealed for CFloat {}
impl sealed::Sealed for CDouble {}

/// A native IEEE C float under a validated no-excess-precision profile.
pub trait FloatType: CType<Value = FloatValue<Self>, Storage: Copy> {}

pub struct FloatValue<F: FloatType>(F::Storage);
impl<F: FloatType> Copy for FloatValue<F> {}
impl<F: FloatType> Clone for FloatValue<F> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<F: FloatType> sealed::Sealed for FloatValue<F> {}
impl<F: FloatType> FloatValue<F> {
    pub const fn new(value: F::Storage) -> Self {
        Self(value)
    }
    pub const fn get(self) -> F::Storage {
        self.0
    }
}
impl<F: FloatType> CExprValue for FloatValue<F> {
    type Marker = F;
}
impl<F: FloatType> IntoExpression for FloatValue<F> {
    type Value = Self;
    fn into_expression(self) -> Self {
        self
    }
}

macro_rules! floating_type {
    ($marker:ty, $storage:ty) => {
        impl CType for $marker {
            type Storage = $storage;
            type Value = FloatValue<Self>;
            fn from_storage(value: $storage) -> Self::Value {
                FloatValue::new(value)
            }
            fn into_storage(value: Self::Value) -> $storage {
                value.get()
            }
        }
        impl FloatType for $marker {}
        impl ScalarObject for $marker {
            type Canonical = Self;
        }
        impl sealed::Sealed for $storage {}
        impl NativeType for $storage {
            type Marker = $marker;
        }
        impl IntoExpression for $storage {
            type Value = FloatValue<$marker>;
            fn into_expression(self) -> Self::Value {
                FloatValue::new(self)
            }
        }
        impl<K: CInteger> CastTo<$marker> for CValue<K> {
            fn cast_to(self) -> FloatValue<$marker> {
                let bits = K::encode(self.get());
                FloatValue::new(if K::SIGNED {
                    (bits as i128) as $storage
                } else {
                    bits as $storage
                })
            }
        }
        impl<K: CInteger> CastTo<K> for FloatValue<$marker> {
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
        impl<K: CInteger> ImplicitTo<$marker> for CValue<K> {
            fn implicit_to(self) -> FloatValue<$marker> {
                cast::<$marker, _>(self)
            }
        }
        impl<K: CInteger> ImplicitTo<K> for FloatValue<$marker> {
            fn implicit_to(self) -> CValue<K> {
                cast::<K, _>(self)
            }
        }
    };
}
floating_type!(CFloat, f32);
floating_type!(CDouble, f64);
macro_rules! float_cast {
    ($from:ty, $to:ty, $repr:ty) => {
        impl CastTo<$to> for FloatValue<$from> {
            fn cast_to(self) -> FloatValue<$to> {
                FloatValue::new(self.get() as $repr)
            }
        }
        impl ImplicitTo<$to> for FloatValue<$from> {
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
    fn truth(self) -> bool;
}
impl<K: CInteger> Truth for CValue<K> {
    fn truth(self) -> bool {
        super::truth(self)
    }
}
impl<M: CType, Q: Qualifier> Truth for Pointer<M, Q> {
    fn truth(self) -> bool {
        !self.as_mut_address().is_null()
    }
}
impl Truth for FloatValue<CFloat> {
    fn truth(self) -> bool {
        self.get() != 0.0
    }
}
impl Truth for FloatValue<CDouble> {
    fn truth(self) -> bool {
        self.get() != 0.0
    }
}
pub fn truth<V: Truth>(value: V) -> bool {
    value.truth()
}
pub fn logical_not<V: Truth>(value: V) -> CValue<super::CInt> {
    CValue::new(i32::from(!value.truth()))
}

/// Unary promotion applies to integers; it leaves floating identities intact.
pub trait Positive: CExprValue {
    type Output: CExprValue;
    fn positive(self) -> Self::Output;
}
impl<K: CInteger> Positive for CValue<K> {
    type Output = CValue<K::Promoted>;
    fn positive(self) -> Self::Output {
        super::promote(self)
    }
}
impl<F: FloatType> Positive for FloatValue<F> {
    type Output = Self;
    fn positive(self) -> Self {
        self
    }
}
pub fn positive<V: Positive>(value: V) -> V::Output {
    value.positive()
}

pub trait Negate<P: OverflowPolicy>: CExprValue {
    type Output: CExprValue;
    fn negate(self) -> Self::Output;
}
impl<K: CInteger, P: OverflowPolicy> Negate<P> for CValue<K> {
    type Output = CValue<K::Promoted>;
    fn negate(self) -> Self::Output {
        super::neg::<P, _>(self)
    }
}
impl<P: OverflowPolicy> Negate<P> for FloatValue<CFloat> {
    type Output = Self;
    fn negate(self) -> Self {
        Self::new(-self.get())
    }
}
impl<P: OverflowPolicy> Negate<P> for FloatValue<CDouble> {
    type Output = Self;
    fn negate(self) -> Self {
        Self::new(-self.get())
    }
}
pub fn neg<P: OverflowPolicy, V: Negate<P>>(value: V) -> V::Output {
    value.negate()
}
pub fn bitnot<K: CInteger>(value: CValue<K>) -> CValue<K::Promoted> {
    super::bitnot(value)
}

macro_rules! arithmetic_operator {
    ($trait:ident, $method:ident, $integer:ident, $operator:tt) => {
        pub trait $trait<Rhs: CExprValue, P: OverflowPolicy>: CExprValue {
            type Output: CExprValue;
            fn $method(self, rhs: Rhs) -> Self::Output;
        }
        impl<L: CInteger, R: CInteger, P: OverflowPolicy> $trait<CValue<R>, P> for CValue<L>
        where
            CValue<L>: super::ArithmeticInput<CValue<R>>,
        {
            type Output = CValue<<CValue<L> as super::ArithmeticInput<CValue<R>>>::Common>;
            fn $method(self, rhs: CValue<R>) -> Self::Output {
                super::$integer::<P, _, _>(self, rhs)
            }
        }
        pub fn $method<P: OverflowPolicy, L: $trait<R, P>, R: CExprValue>(
            left: L,
            right: R,
        ) -> L::Output {
            left.$method(right)
        }
        float_arithmetic!($trait, $method, $operator);
    };
}
macro_rules! float_arithmetic_pair {
    ($trait:ident, $method:ident, $operator:tt, $left:ty, $right:ty, $output:ty) => {
        impl<P: OverflowPolicy> $trait<FloatValue<$right>, P> for FloatValue<$left> {
            type Output = FloatValue<$output>;
            fn $method(self, rhs: FloatValue<$right>) -> Self::Output {
                FloatValue::new(cast::<$output, _>(self).get() $operator cast::<$output, _>(rhs).get())
            }
        }
    };
}
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
macro_rules! float_integer_arithmetic {
    ($trait:ident, $method:ident, $operator:tt, $float:ty) => {
        impl<K: CInteger, P: OverflowPolicy> $trait<CValue<K>, P> for FloatValue<$float> {
            type Output = Self;
            fn $method(self, rhs: CValue<K>) -> Self {
                FloatValue::new(self.get() $operator cast::<$float, _>(rhs).get())
            }
        }
        impl<K: CInteger, P: OverflowPolicy> $trait<FloatValue<$float>, P> for CValue<K> {
            type Output = FloatValue<$float>;
            fn $method(self, rhs: FloatValue<$float>) -> Self::Output {
                FloatValue::new(cast::<$float, _>(self).get() $operator rhs.get())
            }
        }
    };
}
arithmetic_operator!(Add, add, add, +);
arithmetic_operator!(Subtract, sub, sub, -);
arithmetic_operator!(Multiply, mul, mul, *);

macro_rules! integer_operator {
    ($trait:ident, $method:ident, $integer:ident) => {
        pub trait $trait<Rhs: CExprValue>: CExprValue {
            type Output: CExprValue;
            fn $method(self, rhs: Rhs) -> Self::Output;
        }
        impl<L: CInteger, R: CInteger> $trait<CValue<R>> for CValue<L>
        where
            CValue<L>: super::ArithmeticInput<CValue<R>>,
        {
            type Output = CValue<<CValue<L> as super::ArithmeticInput<CValue<R>>>::Common>;
            fn $method(self, rhs: CValue<R>) -> Self::Output {
                super::$integer(self, rhs)
            }
        }
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

macro_rules! float_division {
    ($left:ty, $right:ty, $output:ty) => {
        impl Divide<FloatValue<$right>> for FloatValue<$left> {
            type Output = FloatValue<$output>;
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
macro_rules! float_integer_division {
    ($float:ty) => {
        impl<K: CInteger> Divide<CValue<K>> for FloatValue<$float> {
            type Output = Self;
            fn div(self, rhs: CValue<K>) -> Self {
                Self::new(self.get() / cast::<$float, _>(rhs).get())
            }
        }
        impl<K: CInteger> Divide<FloatValue<$float>> for CValue<K> {
            type Output = FloatValue<$float>;
            fn div(self, rhs: FloatValue<$float>) -> Self::Output {
                FloatValue::new(cast::<$float, _>(self).get() / rhs.get())
            }
        }
    };
}
float_integer_division!(CFloat);
float_integer_division!(CDouble);

pub fn shl<P: OverflowPolicy, L: CInteger, R: CInteger>(
    left: CValue<L>,
    right: CValue<R>,
) -> CValue<L::Promoted> {
    super::shl::<P, _, _>(left, right)
}
pub fn shr<L: CInteger, R: CInteger>(left: CValue<L>, right: CValue<R>) -> CValue<L::Promoted> {
    super::shr(left, right)
}

fn pointer_count<K: CInteger>(value: CValue<K>) -> isize {
    let bits = K::encode(value.get());
    if K::SIGNED {
        isize::try_from(bits as i128).expect("C pointer offset exceeds addressable allocation")
    } else {
        isize::try_from(bits).expect("C pointer offset exceeds addressable allocation")
    }
}
impl<M: CompleteObject, Q: Qualifier, K: CInteger, P: OverflowPolicy> Add<CValue<K>, P>
    for Pointer<M, Q>
{
    type Output = Self;
    fn add(self, rhs: CValue<K>) -> Self {
        assert!(
            core::mem::size_of::<M::Storage>() != 0,
            "C pointer arithmetic needs nonempty storage"
        );
        Pointer::new(Q::from_mut(self.as_mut_address().wrapping_offset(pointer_count(rhs))))
            .with_access(self.access())
    }
}
impl<M: CompleteObject, Q: Qualifier, K: CInteger, P: OverflowPolicy> Add<Pointer<M, Q>, P>
    for CValue<K>
{
    type Output = Pointer<M, Q>;
    fn add(self, rhs: Pointer<M, Q>) -> Self::Output {
        <Pointer<M, Q> as Add<CValue<K>, P>>::add(rhs, self)
    }
}
impl<M: CompleteObject, Q: Qualifier, K: CInteger, P: OverflowPolicy> Subtract<CValue<K>, P>
    for Pointer<M, Q>
{
    type Output = Self;
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
impl<M: CompleteObject, L: Qualifier, R: Qualifier, P: OverflowPolicy> Subtract<Pointer<M, R>, P>
    for Pointer<M, L>
{
    type Output = CValue<super::CLong>;
    fn sub(self, rhs: Pointer<M, R>) -> Self::Output {
        let size = core::mem::size_of::<M::Storage>();
        assert!(size != 0, "C pointer subtraction needs nonempty storage");
        // Addresses are sufficient for the numeric result. Unlike offset_from,
        // this does not perform an unsafe Rust operation for different allocations.
        // Analysis admits only C invocations whose pointers are in the same array.
        let bytes = (self.as_mut_address().addr() as i128) - (rhs.as_mut_address().addr() as i128);
        assert!(bytes % size as i128 == 0, "C pointer subtraction needs a whole element distance");
        CValue::new(i64::try_from(bytes / size as i128).expect("C ptrdiff_t overflow"))
    }
}
/// C permits both `pointer[index]` and `index[pointer]`.
pub trait Subscript<Rhs: CExprValue>: CExprValue {
    type Element: CType;
    type Qualification: Qualifier;
    fn subscript(self, rhs: Rhs) -> Place<Self::Element, Self::Qualification>;
}
impl<M: CompleteObject, Q: Qualifier, K: CInteger> Subscript<CValue<K>> for Pointer<M, Q> {
    type Element = M;
    type Qualification = Q;
    fn subscript(self, rhs: CValue<K>) -> Place<M, Q> {
        pointee(add::<super::Undefined, _, _>(self, rhs))
    }
}
impl<M: CompleteObject, Q: Qualifier, K: CInteger> Subscript<Pointer<M, Q>> for CValue<K> {
    type Element = M;
    type Qualification = Q;
    fn subscript(self, rhs: Pointer<M, Q>) -> Place<M, Q> {
        rhs.subscript(self)
    }
}
pub fn index<L: Subscript<R>, R: CExprValue>(
    left: L,
    right: R,
) -> Place<L::Element, L::Qualification> {
    left.subscript(right)
}

/// Comparison dispatch retains integer promotions and floating NaN behavior.
pub trait Compare<Rhs: CExprValue>: CExprValue {
    fn eq(self, rhs: Rhs) -> bool;
    fn ne(self, rhs: Rhs) -> bool;
    fn lt(self, rhs: Rhs) -> bool;
    fn le(self, rhs: Rhs) -> bool;
    fn gt(self, rhs: Rhs) -> bool;
    fn ge(self, rhs: Rhs) -> bool;
}
/// Equality additionally permits C's tagged null constants against pointers.
/// Ordering deliberately retains the stricter Compare operand requirement.
pub trait Equality<Rhs: CExprValue>: CExprValue {
    fn equal(self, rhs: Rhs) -> bool;
    fn not_equal(self, rhs: Rhs) -> bool;
}
pub fn eq<L: Equality<R>, R: CExprValue>(left: L, right: R) -> CValue<super::CInt> {
    CValue::new(i32::from(left.equal(right)))
}
pub fn ne<L: Equality<R>, R: CExprValue>(left: L, right: R) -> CValue<super::CInt> {
    CValue::new(i32::from(left.not_equal(right)))
}
macro_rules! compare_functions {
    ($($method:ident),+ $(,)?) => { $(
        pub fn $method<L: Compare<R>, R: CExprValue>(left: L, right: R) -> CValue<super::CInt> {
            CValue::new(i32::from(left.$method(right)))
        }
    )+ };
}
compare_functions!(lt, le, gt, ge);
macro_rules! integer_compare_methods {
    ($($method:ident),+ $(,)?) => { $(
        fn $method(self, rhs: CValue<R>) -> bool { super::$method(self, rhs).get() != 0 }
    )+ };
}
impl<L: CInteger, R: CInteger> Compare<CValue<R>> for CValue<L>
where
    CValue<L>: super::ArithmeticInput<CValue<R>>,
{
    integer_compare_methods!(eq, ne, lt, le, gt, ge);
}
macro_rules! equality_from_compare {
    ($rhs:ty) => {
        fn equal(self, rhs: $rhs) -> bool {
            <Self as Compare<$rhs>>::eq(self, rhs)
        }
        fn not_equal(self, rhs: $rhs) -> bool {
            <Self as Compare<$rhs>>::ne(self, rhs)
        }
    };
}
impl<L: CInteger, R: CInteger> Equality<CValue<R>> for CValue<L>
where
    CValue<L>: super::ArithmeticInput<CValue<R>>,
{
    equality_from_compare!(CValue<R>);
}
macro_rules! native_compare_methods {
    ($left:expr, $right:expr, $rhs:ty) => {
        fn eq(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left == right
        }
        fn ne(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left != right
        }
        fn lt(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left < right
        }
        fn le(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left <= right
        }
        fn gt(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left > right
        }
        fn ge(self, rhs: $rhs) -> bool {
            let (left, right) = ($left(self), $right(rhs));
            left >= right
        }
    };
}
macro_rules! float_compare_pair {
    ($left:ty, $right:ty, $common:ty) => {
        impl Compare<FloatValue<$right>> for FloatValue<$left> {
            native_compare_methods!(
                |v| cast::<$common, _>(v).get(),
                |v| cast::<$common, _>(v).get(),
                FloatValue<$right>
            );
        }
        impl Equality<FloatValue<$right>> for FloatValue<$left> {
            equality_from_compare!(FloatValue<$right>);
        }
    };
}
float_compare_pair!(CFloat, CFloat, CFloat);
float_compare_pair!(CFloat, CDouble, CDouble);
float_compare_pair!(CDouble, CFloat, CDouble);
float_compare_pair!(CDouble, CDouble, CDouble);
macro_rules! float_integer_compare {
    ($float:ty) => {
        impl<K: CInteger> Compare<CValue<K>> for FloatValue<$float> {
            native_compare_methods!(|v: Self| v.get(), |v| cast::<$float, _>(v).get(), CValue<K>);
        }
        impl<K: CInteger> Equality<CValue<K>> for FloatValue<$float> {
            equality_from_compare!(CValue<K>);
        }
        impl<K: CInteger> Compare<FloatValue<$float>> for CValue<K> {
            native_compare_methods!(
                |v| cast::<$float, _>(v).get(),
                |v: FloatValue<$float>| v.get(),
                FloatValue<$float>
            );
        }
        impl<K: CInteger> Equality<FloatValue<$float>> for CValue<K> {
            equality_from_compare!(FloatValue<$float>);
        }
    };
}
float_integer_compare!(CFloat);
float_integer_compare!(CDouble);
/// Ordering requires compatible complete object pointees; void and unrelated
/// object identities cannot gain ordering from the equality void conversion.
pub trait SameIdentity<Target> {}
impl<I> SameIdentity<I> for I {}
impl<M: PointeeIdentity, Q: Qualifier, N: PointeeIdentity, R: Qualifier> Compare<Pointer<N, R>>
    for Pointer<M, Q>
where
    M::Identity: SameIdentity<N::Identity> + NonVoidIdentity,
{
    native_compare_methods!(|v: Self| v.as_mut_address().addr(), |v: Pointer<N, R>| v.as_mut_address().addr(), Pointer<N, R>);
}
impl<M: PointeeIdentity, Q: Qualifier, N: PointeeIdentity, R: Qualifier> Equality<Pointer<N, R>>
    for Pointer<M, Q>
where
    M::Identity: CompatibleIdentity<N::Identity>,
{
    fn equal(self, rhs: Pointer<N, R>) -> bool {
        self.as_mut_address().addr() == rhs.as_mut_address().addr()
    }
    fn not_equal(self, rhs: Pointer<N, R>) -> bool {
        self.as_mut_address().addr() != rhs.as_mut_address().addr()
    }
}
impl<S: FunctionSignature> Equality<FunctionValue<S>> for FunctionValue<S> {
    fn equal(self, rhs: Self) -> bool {
        self.address() == rhs.address()
    }
    fn not_equal(self, rhs: Self) -> bool {
        self.address() != rhs.address()
    }
}

macro_rules! null_equality {
    ($pointer:ty, $nonnull:expr, $( $bound:tt )*) => {
        impl<K: ZeroInteger, $($bound)*> Equality<CValue<K>> for $pointer {
            fn equal(self, rhs: CValue<K>) -> bool {
                K::verify_zero(rhs.get());
                !$nonnull(self)
            }
            fn not_equal(self, rhs: CValue<K>) -> bool {
                K::verify_zero(rhs.get());
                $nonnull(self)
            }
        }
        impl<K: ZeroInteger, $($bound)*> Equality<$pointer> for CValue<K> {
            fn equal(self, rhs: $pointer) -> bool {
                <$pointer as Equality<Self>>::equal(rhs, self)
            }
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
    type Output: CExprValue;
    fn select(arm: super::Either<Self, Rhs>) -> Self::Output;
}
impl<L: CInteger, R: CInteger> Select<CValue<R>> for CValue<L>
where
    CValue<L>: super::ArithmeticInput<CValue<R>>,
{
    type Output = CValue<<CValue<L> as super::ArithmeticInput<CValue<R>>>::Common>;
    fn select(arm: super::Either<Self, CValue<R>>) -> Self::Output {
        super::select(arm)
    }
}
macro_rules! float_select_pair {
    ($left:ty, $right:ty, $common:ty) => {
        impl Select<FloatValue<$right>> for FloatValue<$left> {
            type Output = FloatValue<$common>;
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
macro_rules! float_integer_select {
    ($float:ty) => {
        impl<K: CInteger> Select<CValue<K>> for FloatValue<$float> {
            type Output = Self;
            fn select(arm: super::Either<Self, CValue<K>>) -> Self {
                select_as::<$float, _, _>(arm)
            }
        }
        impl<K: CInteger> Select<FloatValue<$float>> for CValue<K> {
            type Output = FloatValue<$float>;
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
    type Output: Qualifier;
}
impl CommonQualifier<ReadWrite> for ReadWrite {
    type Output = ReadWrite;
}
impl CommonQualifier<ReadOnly> for ReadWrite {
    type Output = ReadOnly;
}
impl CommonQualifier<ReadWrite> for ReadOnly {
    type Output = ReadOnly;
}
impl CommonQualifier<ReadOnly> for ReadOnly {
    type Output = ReadOnly;
}
impl<M: CType, Q: CommonQualifier<R>, R: Qualifier> Select<Pointer<M, R>> for Pointer<M, Q> {
    type Output = Pointer<M, Q::Output>;
    fn select(arm: super::Either<Self, Pointer<M, R>>) -> Self::Output {
        select_as::<CPointer<M, Q::Output>, _, _>(arm)
    }
}
macro_rules! null_select {
    ($pointer:ty, $marker:ty, $( $bound:tt )*) => {
        impl<K: ZeroInteger, $($bound)*> Select<CValue<K>> for $pointer {
            type Output = Self;
            fn select(arm: super::Either<Self, CValue<K>>) -> Self {
                match arm {
                    super::Either::Left(value) => value,
                    super::Either::Right(value) => implicit::<$marker, _>(value),
                }
            }
        }
        impl<K: ZeroInteger, $($bound)*> Select<$pointer> for CValue<K> {
            type Output = $pointer;
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
impl<S: FunctionSignature> Select<FunctionValue<S>> for FunctionValue<S> {
    type Output = Self;
    fn select(arm: super::Either<Self, Self>) -> Self {
        match arm {
            super::Either::Left(value) | super::Either::Right(value) => value,
        }
    }
}
impl<R: Copy> Select<RecordValue<R>> for RecordValue<R> {
    type Output = Self;
    fn select(arm: super::Either<Self, Self>) -> Self {
        match arm {
            super::Either::Left(v) | super::Either::Right(v) => v,
        }
    }
}
impl Select<()> for () {
    type Output = Self;
    fn select(_: super::Either<Self, Self>) {}
}
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

#[cfg(test)]
mod tests {
    use super::super::{
        CBool, CInt, CLong, CLongLong, CUnsignedChar, CUnsignedInt, CUnsignedLong, Undefined,
    };
    use super::*;
    use std::cell::Cell;
    use std::fmt::Write;
    use std::mem::MaybeUninit;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[repr(C)]
    struct Partial {
        first: u32,
        second: bool,
    }
    struct First;

    #[repr(C)]
    struct OffsetInner {
        lead: u8,
        value: u32,
    }
    #[repr(C, packed)]
    struct OffsetOuter {
        lead: u8,
        nested: OffsetInner,
        trailing: [u8; 0],
    }
    struct NestedOffset;
    struct ValueOffset;
    struct TrailingOffset;
    struct LeadOffset;
    impl OffsetField<NestedOffset> for CRecord<OffsetOuter> {
        type Member = CRecord<OffsetInner>;
        const OFFSET: usize = core::mem::offset_of!(OffsetOuter, nested);
    }
    impl OffsetField<ValueOffset> for CRecord<OffsetInner> {
        type Member = ();
        const OFFSET: usize = core::mem::offset_of!(OffsetInner, value);
    }
    impl OffsetField<TrailingOffset> for CRecord<OffsetOuter> {
        type Member = ();
        const OFFSET: usize = core::mem::offset_of!(OffsetOuter, trailing);
    }
    impl OffsetField<LeadOffset> for CRecord<OffsetOuter> {
        type Member = ();
        const OFFSET: usize = core::mem::offset_of!(OffsetOuter, lead);
    }

    #[test]
    fn offset_paths_compose_packed_layout_without_object_storage() {
        type Path = OffsetStep<NestedOffset, OffsetStep<ValueOffset, OffsetEnd>>;
        const OFFSET: CValue<CUnsignedLong> = offset_of::<CRecord<OffsetOuter>, Path>();
        assert_eq!(OFFSET.get(), core::mem::offset_of!(OffsetOuter, nested.value) as u64);
        assert_eq!(<CRecord<OffsetOuter> as OffsetPath<Path>>::DEPTH, 2);
        assert_eq!(offset_of::<CVolatile<CRecord<OffsetOuter>>, Path>().get(), OFFSET.get());
    }

    #[test]
    fn offset_leaf_does_not_require_a_value_or_complete_member_type() {
        type Trailing = OffsetStep<TrailingOffset, OffsetEnd>;
        assert_eq!(
            offset_of::<CRecord<OffsetOuter>, Trailing>().get(),
            core::mem::offset_of!(OffsetOuter, trailing) as u64
        );
        assert_eq!(offset_of::<CRecord<OffsetOuter>, OffsetStep<LeadOffset, OffsetEnd>>().get(), 0);
    }

    // SAFETY: The compiler checks the actual field type and projection. Only the
    // first field is projected, with no record read or qualification broadening.
    unsafe impl Field<First, ReadWrite> for CRecord<Partial> {
        type Output = Place<CUnsignedInt>;
        unsafe fn project(base: Place<Self>) -> Self::Output {
            // SAFETY: The caller supplies an allocation containing Partial. A
            // raw field projection doesn't require the other field initialized.
            let address = unsafe { &raw mut (*base.pointer().get()).first };
            place(address).with_access(base.access())
        }
    }

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

    #[derive(Clone, Copy)]
    struct CountingPlace<'a> {
        slot: *mut u8,
        reads: &'a Cell<u32>,
        writes: &'a Cell<u32>,
    }
    // SAFETY: This test adapter counts and delegates one load to a typed place.
    unsafe impl ReadPlace for CountingPlace<'_> {
        type Object = CUnsignedChar;
        type Type = CUnsignedChar;
        unsafe fn load(self) -> CValue<CUnsignedChar> {
            self.reads.set(self.reads.get() + 1);
            // SAFETY: The caller's contract applies to this same u8 allocation.
            unsafe { load(place::<CUnsignedChar>(self.slot)) }
        }
    }
    // SAFETY: The adapter delegates one Copy store without accessing old storage.
    unsafe impl WritePlace for CountingPlace<'_> {
        type Assignment = CUnsignedChar;
        unsafe fn store(self, value: CValue<CUnsignedChar>) -> CValue<CUnsignedChar> {
            self.writes.set(self.writes.get() + 1);
            // SAFETY: The caller establishes writable storage for this u8.
            unsafe { place::<CUnsignedChar>(self.slot).store(value) }
        }
    }

    #[derive(Clone, Copy)]
    struct UnsignedBitfield(*mut u8);
    // SAFETY: The adapter accesses only this initialized one-byte test slot,
    // retaining the seven-bit value and its compiler-established promotion.
    unsafe impl ReadPlace for UnsignedBitfield {
        type Object = CUnsignedInt;
        type Type = CBitfield<CUnsignedInt, CInt>;
        unsafe fn load(self) -> CValue<Self::Type> {
            // SAFETY: The caller establishes a readable initialized byte slot.
            CValue::new(u32::from(unsafe { self.0.read() } & 0x7f))
        }
    }
    // SAFETY: Assignment first converts to the declared unsigned C base type,
    // then the adapter narrows to seven bits and retains the bit-field identity.
    unsafe impl WritePlace for UnsignedBitfield {
        type Assignment = CUnsignedInt;
        unsafe fn store(self, value: CValue<CUnsignedInt>) -> CValue<Self::Type> {
            let stored = (value.get() & 0x7f) as u8;
            // SAFETY: The caller establishes writable storage for this byte slot.
            unsafe { self.0.write(stored) };
            CValue::new(u32::from(stored))
        }
    }

    #[derive(Clone, Copy)]
    struct LongBitfield(*mut u64);
    // SAFETY: This initialized test slot models a seven-bit unsigned long field
    // whose original C compiler establishes int as its arithmetic promotion.
    unsafe impl ReadPlace for LongBitfield {
        type Object = CUnsignedLong;
        type Type = CBitfield<CUnsignedLong, CInt>;
        unsafe fn load(self) -> CValue<Self::Type> {
            // SAFETY: The caller establishes an initialized live u64 allocation.
            CValue::new(unsafe { self.0.read() } & 0x7f)
        }
    }
    // SAFETY: The test adapter stores the narrowed field value using the original
    // unsigned long representation, without reading uninitialized adjacent data.
    unsafe impl WritePlace for LongBitfield {
        type Assignment = CUnsignedLong;
        unsafe fn store(self, value: CValue<CUnsignedLong>) -> CValue<Self::Type> {
            let value = value.get() & 0x7f;
            // SAFETY: The caller establishes writable storage for this u64 slot.
            unsafe { self.0.write(value) };
            CValue::new(value)
        }
    }

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

    #[derive(Clone, Copy)]
    struct IncrementSignature;
    impl sealed::Sealed for IncrementSignature {}
    impl FunctionSignature for IncrementSignature {
        type Pointer = Option<unsafe extern "C-unwind" fn(i32) -> i32>;
        fn null() -> Self::Pointer {
            None
        }
        fn address(pointer: Self::Pointer) -> *const () {
            pointer.map_or(core::ptr::null(), |function| function as *const ())
        }
    }
    unsafe extern "C-unwind" fn increment(value: i32) -> i32 {
        value + 1
    }
    impl<V: ImplicitTo<CInt>> Call<(V,)> for IncrementSignature {
        type Output = CValue<CInt>;
        unsafe fn call(pointer: Self::Pointer, args: (V,)) -> Self::Output {
            let value = implicit::<CInt, _>(args.0).get();
            let callable = pointer.expect("C indirect call requires a non-null function pointer");
            // SAFETY: The caller establishes this exact fixture signature; the
            // native test function neither calls PostgreSQL nor requires guards.
            let result = unsafe { callable(value) };
            CValue::new(result)
        }
    }

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

    static NEXT_ORACLE: AtomicU64 = AtomicU64::new(0);
    struct OracleDirectory(PathBuf);
    impl OracleDirectory {
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
    impl Drop for OracleDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

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
            ("ambiguous_rank", "let _=expression::input(1_u64);", "E0277"),
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
