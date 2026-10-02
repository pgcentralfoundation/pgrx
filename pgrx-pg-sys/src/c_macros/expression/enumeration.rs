//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! C enum identities and object access without Rust enum validity assumptions.

use super::*;

/// A canonical enum declaration established by the C compiler.
pub trait EnumIdentity: sealed::Sealed + Copy + core::fmt::Debug + Eq {}

/// Numeric enum values retain their declaration identity and compatible integer type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CEnum<I: EnumIdentity, K: CInteger>(PhantomData<(I, K)>);
impl<I: EnumIdentity, K: CInteger> sealed::Sealed for CEnum<I, K> {}
impl<I: EnumIdentity, K: CInteger> CInteger for CEnum<I, K> {
    type Repr = K::Repr;
    type Promoted = K::Promoted;
    type Boundary = Self;
    const BITS: u32 = K::BITS;
    const SIGNED: bool = K::SIGNED;
    const RANK: u8 = K::RANK;
    fn encode(value: Self::Repr) -> u128 {
        K::encode(value)
    }
    fn decode(bits: u128) -> Self::Repr {
        K::decode(bits)
    }
}

/// A Rust binding representation of one compiler-established C enum.
///
/// # Safety
/// `Self` and `K::Repr` must have identical size, alignment and integer bit
/// representation. `decode` must preserve every valid Rust input's value.
/// `encode` must return a valid Rust value or reject the conversion; C values
/// outside Rust's variants must never be materialized as `Self`.
pub unsafe trait EnumStorage<I: EnumIdentity, K: CInteger>: Sized {
    fn decode(value: Self) -> K::Repr;
    fn encode(value: K::Repr) -> Self;
}

/// An enum object whose raw storage can contain C-valid, unnamed enum values.
/// Reads and writes use the compatible integer representation, never `R` loads.
pub struct CEnumObject<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>>(PhantomData<(I, K, R)>);
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> Copy for CEnumObject<I, K, R> {}
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> Clone for CEnumObject<I, K, R> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> sealed::Sealed for CEnumObject<I, K, R> {}
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> CType for CEnumObject<I, K, R> {
    type Storage = R;
    type Value = CValue<CEnum<I, K>>;
    fn from_storage(value: R) -> Self::Value {
        CValue::new(R::decode(value))
    }
    fn into_storage(value: Self::Value) -> R {
        R::encode(value.get())
    }
}
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> CompleteObject for CEnumObject<I, K, R> {}
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>, const B: bool> AllowedProfile<B>
    for CEnumObject<I, K, R>
{
}
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> PointeeIdentity for CEnumObject<I, K, R> {
    type Identity = CEnum<I, K>;
    type Volatility = NonVolatile;
}
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> record::OwnedMemberObject
    for CEnumObject<I, K, R>
{
}
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> record::OwnedMemberObject
    for CVolatile<CEnumObject<I, K, R>>
{
}

impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>, V: CastTo<CEnum<I, K>>>
    CastTo<CEnumObject<I, K, R>> for V
{
    fn cast_to(self) -> CValue<CEnum<I, K>> {
        self.cast_to()
    }
}
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>, V: ImplicitTo<CEnum<I, K>>>
    ImplicitTo<CEnumObject<I, K, R>> for V
{
    fn implicit_to(self) -> CValue<CEnum<I, K>> {
        self.implicit_to()
    }
}

unsafe fn read_enum<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>>(
    address: *mut R,
    access: Access,
) -> CValue<CEnum<I, K>> {
    assert!(
        !(access.volatile && access.unaligned),
        "unaligned volatile C enum access is unsupported"
    );
    // SAFETY: EnumStorage establishes identical integer layout. The caller
    // establishes initialization, provenance and access rights for these bytes.
    // No Rust enum value or reference is created, including for unnamed C values.
    let value = unsafe {
        let pointer = address.cast::<K::Repr>();
        if access.volatile {
            pointer.read_volatile()
        } else if access.unaligned {
            pointer.read_unaligned()
        } else {
            pointer.read()
        }
    };
    CValue::new(value)
}
unsafe fn write_enum<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>>(
    address: *mut R,
    access: Access,
    value: CValue<CEnum<I, K>>,
) {
    assert!(
        !(access.volatile && access.unaligned),
        "unaligned volatile C enum access is unsupported"
    );
    // SAFETY: The caller establishes writable raw C storage with sufficient
    // provenance and access permissions. EnumStorage proves the integer layout.
    // The write neither reads nor drops an old R and need not produce a valid R;
    // callers must not materialize Rust values/references while bits are invalid.
    unsafe {
        let pointer = address.cast::<K::Repr>();
        if access.volatile {
            pointer.write_volatile(value.get());
        } else if access.unaligned {
            pointer.write_unaligned(value.get());
        } else {
            pointer.write(value.get());
        }
    }
}

// SAFETY: Each adapter reads only the declared enum's initialized integer bytes
// and honors the access qualifiers without loading the restricted Rust enum.
unsafe impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>, Q: Qualifier> ReadObject<Q>
    for CEnumObject<I, K, R>
{
    type RValue = CEnum<I, K>;
    unsafe fn read_object(place: Place<Self, Q>) -> CValue<CEnum<I, K>> {
        // SAFETY: The caller establishes this object's readable C access contract.
        unsafe { read_enum::<I, K, R>(place.address, place.access) }
    }
}
// SAFETY: Volatile qualification is preserved in the supplied place metadata.
unsafe impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>, Q: Qualifier> ReadObject<Q>
    for CVolatile<CEnumObject<I, K, R>>
{
    type RValue = CEnum<I, K>;
    unsafe fn read_object(place: Place<Self, Q>) -> CValue<CEnum<I, K>> {
        // SAFETY: The caller establishes the volatile object's readable contract.
        unsafe { read_enum::<I, K, R>(place.address, place.access) }
    }
}
// SAFETY: Only ReadWrite places implement this capability. Integer writes retain
// the declared C enum identity and never materialize a restricted Rust enum.
unsafe impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> WritePlace
    for Place<CEnumObject<I, K, R>>
{
    type Assignment = CEnum<I, K>;
    unsafe fn store(self, value: CValue<CEnum<I, K>>) -> CValue<CEnum<I, K>> {
        // SAFETY: The caller establishes writable raw enum storage.
        unsafe { write_enum(self.address, self.access, value) }
        value
    }
}
// SAFETY: Same raw integer write contract, with inherited volatile access.
unsafe impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> WritePlace
    for Place<CVolatile<CEnumObject<I, K, R>>>
{
    type Assignment = CEnum<I, K>;
    unsafe fn store(self, value: CValue<CEnum<I, K>>) -> CValue<CEnum<I, K>> {
        // SAFETY: The caller establishes writable volatile raw enum storage.
        unsafe { write_enum(self.address, self.access, value) }
        value
    }
}
