//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! C enum identities and object access without Rust enum validity assumptions.
//!
//! CEnum carries the nominal compiler declaration and its compatible integer kind
//! through arithmetic, even when multiple enums share one native representation.
//! CEnumObject separately describes binding storage that may be a restricted Rust
//! enum. Its raw place reads and writes use the compatible integer representation,
//! so a valid unnamed C enum value never requires constructing an invalid Rust
//! variant. Explicit storage conversion uses the checked EnumStorage bridge.
//!
//! Private NumericEnum registration also permits the exact compatible-integer
//! pointer relationship established by C. It cannot infer a nominal enum identity
//! from an arbitrary equal-width Rust integer, and raw enum access retains the
//! allocation, initialization, aliasing, and access-qualification obligations.

use super::*;

/// A canonical enum declaration established by the C compiler.
pub trait EnumIdentity: sealed::Sealed + Copy + core::fmt::Debug + Eq {}

/// Numeric enum values retain their declaration identity and compatible integer type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CEnum<I: EnumIdentity, K: CInteger>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<(I, K)>,
);
/// Keep this runtime family within crate-owned C capability registration.
impl<I: EnumIdentity, K: CInteger> sealed::Sealed for CEnum<I, K> {}
/// Retain C rank, width, signedness, and promotion rules independently of native Rust storage.
impl<I: EnumIdentity, K: CInteger> CInteger for CEnum<I, K> {
    /// Native bits/values used to store this C integer kind without erasing its rank.
    type Repr = K::Repr;
    /// C kind selected by integer promotion before unary, shift, or arithmetic operations.
    type Promoted = K::Promoted;
    /// Ordinary C identity after source-only constant or bitfield metadata is lost at a value boundary.
    type Boundary = Self;
    /// Width of the declared C storage representation before promotion.
    const BITS: u32 = K::BITS;
    /// Whether the C kind interprets its storage bits as a signed value.
    const SIGNED: bool = K::SIGNED;
    /// C integer rank used by usual arithmetic conversions independently of storage width.
    const RANK: u8 = K::RANK;
    /// Convert admitted integer or enum storage to the representation used by C conversion rules.
    fn encode(value: Self::Repr) -> u128 {
        K::encode(value)
    }
    /// Recover the declared integer or enum representation from checked conversion bits.
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
    /// Decode an already valid Rust storage value to the compiler-compatible integer magnitude.
    fn decode(value: Self) -> K::Repr;
    /// Encode the compatible integer as valid Rust storage, rejecting unsupported enum discriminants.
    fn encode(value: K::Repr) -> Self;
}

/// The exact compatible integer kind established for a generated enum identity.
pub(crate) trait NumericEnum: EnumIdentity {
    /// Exact integer kind registered by the compiler for this nominal enum identity.
    type Compatible: CInteger;
}

// Registration permits exactly the compiler-established integer identity in
// either pointer-compatibility direction. Concrete primitive roots preserve
// nominal enum distinctions and remain disjoint from reflexive compatibility.
/// Admit an enum and its exact compiler-registered integer kind in both pointer compatibility directions.
macro_rules! numeric_enum_compatibility {
    ($($kind:ty),+ $(,)?) => {$(
        /// Permit compatibility only between this enum identity and its compiler-registered integer kind.
        impl<I: NumericEnum<Compatible = $kind>> CompatibleIdentity<$kind>
            for CEnum<I, $kind>
        {}
        /// Permit compatibility only between this enum identity and its compiler-registered integer kind.
        impl<I: NumericEnum<Compatible = $kind>> CompatibleIdentity<CEnum<I, $kind>>
            for $kind
        {}
    )+};
}
numeric_enum_compatibility!(
    super::super::CBool,
    super::super::CChar,
    super::super::CSignedChar,
    super::super::CUnsignedChar,
    super::super::CShort,
    super::super::CUnsignedShort,
    super::super::CInt,
    super::super::CUnsignedInt,
    super::super::CLong,
    super::super::CUnsignedLong,
    super::super::CLongLong,
    super::super::CUnsignedLongLong,
    super::super::CInt128,
    super::super::CUnsignedInt128,
);

/// Share primitive enum storage conversions while keeping nominal identities registered separately.
macro_rules! numeric_enum_storage {
    ($($repr:ty),+ $(,)?) => {$(
        // SAFETY: Registration fixes the compatible kind, whose representation
        // is exactly this primitive. The identity conversion preserves every
        // valid storage value and cannot create an invalid Rust enum variant.
        /// Use identity storage conversion where registration proves this primitive is the enum’s compatible representation.
        unsafe impl<I: NumericEnum<Compatible = K>, K: CInteger<Repr = $repr>>
            EnumStorage<I, K> for $repr
        {
            /// Preserve the primitive value as the enum’s exact compatible integer representation.
            fn decode(value: Self) -> $repr {
                value
            }
            /// Preserve the compatible primitive value without constructing a restricted Rust enum variant.
            fn encode(value: $repr) -> Self {
                value
            }
        }
    )+};
}
numeric_enum_storage!(bool, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

/// An enum object whose raw storage can contain C-valid, unnamed enum values.
/// Reads and writes use the compatible integer representation, never `R` loads.
pub struct CEnumObject<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<(I, K, R)>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> Copy for CEnumObject<I, K, R> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> Clone for CEnumObject<I, K, R> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> sealed::Sealed for CEnumObject<I, K, R> {}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> CType for CEnumObject<I, K, R> {
    /// Rust binding representation used for this declared C object or value.
    type Storage = R;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = CValue<CEnum<I, K>>;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(value: R) -> Self::Value {
        CValue::new(R::decode(value))
    }
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> R {
        R::encode(value.get())
    }
}
/// Expose object-size capability for this complete C storage family.
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> CompleteObject for CEnumObject<I, K, R> {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>, const B: bool> AllowedProfile<B>
    for CEnumObject<I, K, R>
{
}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> PointeeIdentity for CEnumObject<I, K, R> {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = CEnum<I, K>;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = NonVolatile;
}
/// Permit by-value temporary member use without array decay into dangling storage.
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> record::OwnedMemberObject
    for CEnumObject<I, K, R>
{
}
/// Permit by-value temporary member use without array decay into dangling storage.
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> record::OwnedMemberObject
    for CVolatile<CEnumObject<I, K, R>>
{
}

/// Admit this explicit C conversion without treating equal Rust layouts as equivalent C types.
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>, V: CastTo<CEnum<I, K>>>
    CastTo<CEnumObject<I, K, R>> for V
{
    /// Apply the explicit conversion admitted for this source/destination C type pair.
    fn cast_to(self) -> CValue<CEnum<I, K>> {
        self.cast_to()
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>, V: ImplicitTo<CEnum<I, K>>>
    ImplicitTo<CEnumObject<I, K, R>> for V
{
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> CValue<CEnum<I, K>> {
        self.implicit_to()
    }
}

/// Read compatible integer bytes directly, preserving C enum values outside Rust’s declared variants.
///
/// # Safety
/// `address` must have provenance and readable bounds for initialized `K::Repr`
/// bytes. It must be aligned unless `access.unaligned` is set. The access must
/// obey aliasing and concurrency rules, and `access.volatile` must describe the
/// C object's qualification. Unaligned volatile access is rejected.
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
/// Store compatible integer bytes without constructing a restricted Rust enum or reading its old value.
///
/// # Safety
/// `address` must have provenance and writable bounds for `K::Repr` bytes. It
/// must be aligned unless `access.unaligned` is set. The access must obey
/// aliasing and concurrency rules, and `access.volatile` must describe the C
/// object's qualification. Unaligned volatile access is rejected. The write
/// can leave an invalid Rust enum discriminant; no `R` value or reference may
/// be materialized until the stored bytes form a valid `R`.
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
/// Provide declared object-to-value conversion without introducing a whole-record validity requirement.
unsafe impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>, Q: Qualifier> ReadObject<Q>
    for CEnumObject<I, K, R>
{
    /// Canonical C expression kind after object-to-value conversion.
    type RValue = CEnum<I, K>;
    /// Apply C object-to-value conversion with the declared access metadata.
    ///
    /// # Safety
    /// The place must satisfy `read_enum` for live, initialized compatible-integer
    /// storage: provenance, readable bounds, alignment, aliasing, and concurrency
    /// permissions must hold, with truthful volatile/unaligned metadata. No valid
    /// whole `R` enum value is required or materialized by this read.
    unsafe fn read_object(place: Place<Self, Q>) -> CValue<CEnum<I, K>> {
        // SAFETY: The caller establishes this object's readable C access contract.
        unsafe { read_enum::<I, K, R>(place.address, place.access) }
    }
}
// SAFETY: Volatile qualification is preserved in the supplied place metadata.
/// Provide declared object-to-value conversion without introducing a whole-record validity requirement.
unsafe impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>, Q: Qualifier> ReadObject<Q>
    for CVolatile<CEnumObject<I, K, R>>
{
    /// Canonical C expression kind after object-to-value conversion.
    type RValue = CEnum<I, K>;
    /// Apply C object-to-value conversion with the declared access metadata.
    ///
    /// # Safety
    /// The place must satisfy `read_enum` for live, initialized compatible-integer
    /// storage: provenance, readable bounds, alignment, aliasing, and concurrency
    /// permissions must hold, with truthful volatile/unaligned metadata. No valid
    /// whole `R` enum value is required or materialized by this read.
    unsafe fn read_object(place: Place<Self, Q>) -> CValue<CEnum<I, K>> {
        // SAFETY: The caller establishes the volatile object's readable contract.
        unsafe { read_enum::<I, K, R>(place.address, place.access) }
    }
}
// SAFETY: Only ReadWrite places implement this capability. Integer writes retain
// the declared C enum identity and never materialize a restricted Rust enum.
/// Expose assignment only for this admitted writable destination family.
unsafe impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> WritePlace
    for Place<CEnumObject<I, K, R>>
{
    /// C destination kind used for assignment conversion before storing the object.
    type Assignment = CEnum<I, K>;
    /// Write the converted destination value without reading or dropping the previous object.
    ///
    /// # Safety
    /// The place must satisfy `write_enum` for live writable compatible-integer
    /// storage, including provenance, bounds, alignment, aliasing, concurrency,
    /// and truthful access metadata. Previous contents need not be initialized.
    /// The write may leave an invalid `R` discriminant; typed Rust reads or references
    /// are forbidden until storage again forms a valid `R`.
    unsafe fn store(self, value: CValue<CEnum<I, K>>) -> CValue<CEnum<I, K>> {
        // SAFETY: The caller establishes writable raw enum storage.
        unsafe { write_enum(self.address, self.access, value) }
        value
    }
}
// SAFETY: Same raw integer write contract, with inherited volatile access.
/// Expose assignment only for this admitted writable destination family.
unsafe impl<I: EnumIdentity, K: CInteger, R: EnumStorage<I, K>> WritePlace
    for Place<CVolatile<CEnumObject<I, K, R>>>
{
    /// C destination kind used for assignment conversion before storing the object.
    type Assignment = CEnum<I, K>;
    /// Write the converted destination value without reading or dropping the previous object.
    ///
    /// # Safety
    /// The place must satisfy `write_enum` for live writable compatible-integer
    /// storage, including provenance, bounds, alignment, aliasing, concurrency,
    /// and truthful access metadata. Previous contents need not be initialized.
    /// The write may leave an invalid `R` discriminant; typed Rust reads or references
    /// are forbidden until storage again forms a valid `R`.
    unsafe fn store(self, value: CValue<CEnum<I, K>>) -> CValue<CEnum<I, K>> {
        // SAFETY: The caller establishes writable volatile raw enum storage.
        unsafe { write_enum(self.address, self.access, value) }
        value
    }
}
