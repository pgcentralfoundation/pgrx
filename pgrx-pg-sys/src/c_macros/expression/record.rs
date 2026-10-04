//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Copy C aggregate representations without asserting whole-record Rust validity.
//!
//! C macros may copy records with fields that have not yet been initialized.
//! Their bytes stay in `MaybeUninit` so copying and address-only member
//! projection neither load an invalid Rust value nor run a hidden destructor.
//! Only a selected member needs initialization when it is read. Materializing
//! the complete binding record remains an explicit unsafe operation with
//! separate validity, ownership, aliasing, and lifetime obligations.

use super::*;
use core::mem::MaybeUninit;

/// Identify aggregate value storage that may contain uninitialized fields and is never loaded as `R`.
pub struct CRawRecord<R>(
    /// Carry C identity and qualification at the type level without runtime storage or ownership.
    PhantomData<R>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<R> Copy for CRawRecord<R> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<R> Clone for CRawRecord<R> {
    /// Copy this admitted value or type marker without accessing any designated allocation.
    fn clone(&self) -> Self {
        *self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<R> sealed::Sealed for CRawRecord<R> {}
/// Expose object-size capability for this complete C storage family.
impl<R> CompleteObject for CRawRecord<R> {}
/// Gate this expression family on the compiler profile facts required by its C representation.
impl<R, const B: bool> AllowedProfile<B> for CRawRecord<R> {}

/// A copied C aggregate that need not contain a valid initialized Rust record.
/// No operation on this wrapper runs the record's destructor.
pub struct RawRecordValue<R>(
    /// Keep possibly uninitialized aggregate bytes inside storage that never drops an `R`.
    MaybeUninit<R>,
);
/// Permit bit/metadata copying of this admitted representation without accessing a pointed-to object.
impl<R: Copy> Copy for RawRecordValue<R> {}
/// Provide the value or metadata copy used by this representation's expression operations.
impl<R> Clone for RawRecordValue<R> {
    /// Copy raw aggregate bytes without reading fields as `R` or running its destructor.
    fn clone(&self) -> Self {
        // SAFETY: MaybeUninit admits uninitialized bytes and preserves the
        // non-padding representation/provenance. Neither copy drops an R.
        Self(unsafe { core::ptr::read(&self.0) })
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<R> sealed::Sealed for RawRecordValue<R> {}
/// Keep raw aggregate construction and extraction separate from the unsafe whole-record initialization assertion.
impl<R> RawRecordValue<R> {
    /// Retain raw aggregate storage without asserting whole-record initialization.
    pub const fn new(storage: MaybeUninit<R>) -> Self {
        Self(storage)
    }
    /// Move an initialized record into raw storage without scheduling its hidden destructor.
    pub const fn initialized(value: R) -> Self {
        Self(MaybeUninit::new(value))
    }
    /// Expose raw aggregate storage without materializing an initialized Rust record.
    pub fn get(self) -> MaybeUninit<R> {
        self.0
    }
    /// Materialize the whole Rust binding record after proving all of its fields.
    ///
    /// # Safety
    /// Every non-padding byte required by R must be initialized, and all fields
    /// must satisfy their Rust validity, aliasing, lifetime and ownership
    /// invariants. Extracting multiple byte copies must not duplicate ownership
    /// or reference permissions; C aggregate copying alone does not prove this.
    pub unsafe fn assume_initialized(self) -> R {
        // SAFETY: The caller establishes the complete Rust record invariant.
        unsafe { self.0.assume_init() }
    }
}
/// Connect the declared C identity to its binding storage and evaluated value representations.
impl<R> CType for CRawRecord<R> {
    /// Rust binding representation used for this declared C object or value.
    type Storage = MaybeUninit<R>;
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = RawRecordValue<R>;
    /// Wrap admitted binding storage with the declared C value identity.
    fn from_storage(storage: Self::Storage) -> Self::Value {
        RawRecordValue::new(storage)
    }
    /// Extract or encode the exact binding representation for an explicit storage boundary.
    fn into_storage(value: Self::Value) -> Self::Storage {
        value.get()
    }
}
/// Associate an evaluated value with its exact C declaration marker.
impl<R> CExprValue for RawRecordValue<R> {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CRawRecord<R>;
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<R> IntoExpression for RawRecordValue<R> {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = Self;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self {
        self
    }
}
/// Keep this runtime family within crate-owned C capability registration.
impl<R: NativeType<Marker = CRecord<R>>> sealed::Sealed for MaybeUninit<R> {}
/// Associate admitted native storage with its C marker for macro input and place resolution.
impl<R: NativeType<Marker = CRecord<R>>> NativeType for MaybeUninit<R> {
    /// C declaration identity attached to this expression value or registered native storage.
    type Marker = CRawRecord<R>;
}
/// Normalize this admitted input at the public expression boundary while retaining its C family.
impl<R: NativeType<Marker = CRecord<R>>> IntoExpression for MaybeUninit<R> {
    /// Evaluated C expression representation produced by this type or input conversion.
    type Value = RawRecordValue<R>;
    /// Normalize the input at an evaluated expression boundary, dropping source-only metadata where required.
    fn into_expression(self) -> Self::Value {
        RawRecordValue::new(self)
    }
}
/// Keep object identity distinct from `void` in pointer compatibility and ordering.
impl<R> NonVoidIdentity for CRawRecord<R> {}
/// Separate nominal pointee identity from volatility for C pointer composition.
impl<R> PointeeIdentity for CRawRecord<R> {
    /// Nominal pointee identity used independently of access qualification.
    type Identity = CRecord<R>;
    /// Declared volatile qualification used by directional pointer conversion checks.
    type Volatility = <CRecord<R> as PointeeIdentity>::Volatility;
}

/// Initialized Rust inputs and raw C aggregate results share address-only member access.
pub trait RecordExpression: CExprValue {
    /// Underlying binding record whose representation is retained without requiring initialization.
    type Record;
    /// Expose aggregate representation through `MaybeUninit` without materializing uninitialized fields.
    fn raw_storage(self) -> MaybeUninit<Self::Record>;
}
/// Provide address-only temporary member support from initialized or raw aggregate representation.
impl<R> RecordExpression for RawRecordValue<R> {
    /// Underlying binding record whose representation is retained without requiring initialization.
    type Record = R;
    /// Expose aggregate representation through `MaybeUninit` without materializing uninitialized fields.
    fn raw_storage(self) -> MaybeUninit<R> {
        self.get()
    }
}
/// Provide address-only temporary member support from initialized or raw aggregate representation.
impl<R: Copy> RecordExpression for RecordValue<R> {
    /// Underlying binding record whose representation is retained without requiring initialization.
    type Record = R;
    /// Expose aggregate representation through `MaybeUninit` without materializing uninitialized fields.
    fn raw_storage(self) -> MaybeUninit<R> {
        MaybeUninit::new(self.get())
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<R> ImplicitTo<CRawRecord<R>> for RawRecordValue<R> {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> Self {
        self
    }
}
/// Admit this C assignment conversion while retaining qualification and source-constant restrictions.
impl<R: Copy> ImplicitTo<CRawRecord<R>> for RecordValue<R> {
    /// Apply the stricter C assignment conversion admitted for this operand pair.
    fn implicit_to(self) -> RawRecordValue<R> {
        RawRecordValue::initialized(self.get())
    }
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl<R> Select<RawRecordValue<R>> for RawRecordValue<R> {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Self;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::super::Either<Self, Self>) -> Self {
        match arm {
            super::super::Either::Left(value) | super::super::Either::Right(value) => value,
        }
    }
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl<R: Copy> Select<RawRecordValue<R>> for RecordValue<R> {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = RawRecordValue<R>;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::super::Either<Self, RawRecordValue<R>>) -> Self::Output {
        match arm {
            super::super::Either::Left(value) => RawRecordValue::initialized(value.get()),
            super::super::Either::Right(value) => value,
        }
    }
}
/// Determine conditional result identity from both types and convert only the selected arm.
impl<R: Copy> Select<RecordValue<R>> for RawRecordValue<R> {
    /// Common conditional result determined from both operand types without evaluating both arms.
    type Output = Self;
    /// Convert only the evaluated conditional arm to the common result identity.
    fn select(arm: super::super::Either<Self, RecordValue<R>>) -> Self {
        match arm {
            super::super::Either::Left(value) => value,
            super::super::Either::Right(value) => RawRecordValue::initialized(value.get()),
        }
    }
}

/// Fields copied from a temporary cannot decay an array into a dangling pointer.
pub trait OwnedMemberObject: CType {}
/// Permit by-value temporary member use without array decay into dangling storage.
impl<M: ScalarObject> OwnedMemberObject for M {}
/// Permit by-value temporary member use without array decay into dangling storage.
impl<R> OwnedMemberObject for CRecord<R> {}
/// Permit by-value temporary member use without array decay into dangling storage.
impl<R> OwnedMemberObject for CRawRecord<R> {}
/// Permit by-value temporary member use without array decay into dangling storage.
impl<R> OwnedMemberObject for CVolatile<CRecord<R>> {}

/// Infer a temporary record member's declared size without reading/decaying it.
/// Bitfields have no SizeablePlace capability, including non-lvalue members.
pub fn size_of_member_type<F, V: RecordExpression>(
    _: Option<V>,
) -> CValue<super::super::CUnsignedLong>
where
    CRecord<V::Record>: Field<F, ReadOnly>,
    <CRecord<V::Record> as Field<F, ReadOnly>>::Output: SizeablePlace,
{
    size_of::<<<CRecord<V::Record> as Field<F, ReadOnly>>::Output as SizeablePlace>::Object>()
}

// SAFETY: MaybeUninit has R's size/alignment and permits its partially initialized
// representation. Delegating projection does not read any part of the record.
/// Project exact typed fields while retaining the caller's allocation and access obligations.
unsafe impl<R, F, Q: Qualifier> Field<F, Q> for CRawRecord<R>
where
    CRecord<R>: Field<F, Q>,
{
    /// Qualified field place produced without loading the containing record.
    type Output = <CRecord<R> as Field<F, Q>>::Output;
    /// Resolve the declared field address while retaining allocation provenance, packing, and access qualification.
    ///
    /// # Safety
    /// The base must permit the delegated field projection within a live allocation;
    /// the containing record may be partial, and only a subsequently read field needs initialization.
    unsafe fn project(base: Place<Self, Q>) -> Self::Output {
        let original = Place::<CRecord<R>, Q>::new(Q::from_mut(base.address.cast()), base.access);
        // SAFETY: The storage cast preserves the original address/layout; the
        // caller establishes the delegated projection's allocation contract.
        unsafe { <CRecord<R> as Field<F, Q>>::project(original) }
    }
}

/// Copy aggregate representation bytes without loading an initialized `R`.
///
/// # Safety
/// Address must cover a live readable R allocation, with alignment/permissions
/// matching access. Individual fields may remain uninitialized.
unsafe fn read_record<R>(address: *mut MaybeUninit<R>, access: Access) -> RawRecordValue<R> {
    assert!(
        !access.volatile,
        "volatile aggregate copies require a compiler-verified access primitive"
    );
    // SAFETY: The caller proves the live aggregate bounds and read permissions.
    // MaybeUninit accepts uninitialized fields, and unaligned reads are selected
    // only by compiler-owned place metadata. No R or field reference is created.
    RawRecordValue::new(unsafe {
        if access.unaligned { address.read_unaligned() } else { address.read() }
    })
}

// SAFETY: Aggregate lvalue conversion copies MaybeUninit<R> through the original
// R address and never materializes R's potentially invalid fields.
/// Provide declared object-to-value conversion without introducing a whole-record validity requirement.
unsafe impl<R, Q: Qualifier> ReadObject<Q> for CRecord<R> {
    /// Canonical C expression kind after object-to-value conversion.
    type RValue = CRawRecord<R>;
    /// Apply C object-to-value conversion with the declared access metadata.
    ///
    /// # Safety
    /// The place must cover a live readable aggregate with valid bounds, aliasing,
    /// and alignment matching its access metadata; its fields may remain uninitialized.
    unsafe fn read_object(place: Place<Self, Q>) -> RawRecordValue<R> {
        // SAFETY: The caller establishes live aggregate storage/read permissions.
        unsafe { read_record(place.address.cast(), place.access) }
    }
}
// SAFETY: The raw storage representation is copied without initializing R.
/// Provide declared object-to-value conversion without introducing a whole-record validity requirement.
unsafe impl<R, Q: Qualifier> ReadObject<Q> for CRawRecord<R> {
    /// Canonical C expression kind after object-to-value conversion.
    type RValue = Self;
    /// Apply C object-to-value conversion with the declared access metadata.
    ///
    /// # Safety
    /// The place must cover a live readable aggregate with valid bounds, aliasing,
    /// and alignment matching its access metadata; its fields may remain uninitialized.
    unsafe fn read_object(place: Place<Self, Q>) -> RawRecordValue<R> {
        // SAFETY: The caller establishes live aggregate storage/read permissions.
        unsafe { read_record(place.address, place.access) }
    }
}

/// Assign raw aggregate bytes without loading or dropping the previous record.
///
/// # Safety
/// Address must cover a live writable R allocation with alignment/permissions
/// matching access, and the C aggregate copy must respect aliasing and ownership.
unsafe fn store_record<R>(
    address: *mut MaybeUninit<R>,
    access: Access,
    value: RawRecordValue<R>,
) -> RawRecordValue<R> {
    assert!(
        !access.volatile,
        "volatile aggregate copies require a compiler-verified access primitive"
    );
    let result = value.clone();
    let storage = value.get();
    // SAFETY: The caller proves writable aggregate storage and alias permissions.
    // Writing MaybeUninit neither reads/drops the previous R nor requires every
    // field to be initialized. Compiler metadata selects unaligned access.
    unsafe {
        if access.unaligned {
            address.write_unaligned(storage);
        } else {
            address.write(storage);
        }
    }
    result
}
// SAFETY: Stores copy the C aggregate representation without reading/dropping R.
/// Expose assignment only for this admitted writable destination family.
unsafe impl<R> WritePlace for Place<CRecord<R>> {
    /// C destination kind used for assignment conversion before storing the object.
    type Assignment = CRawRecord<R>;
    /// Write the converted destination value without reading or dropping the previous object.
    ///
    /// # Safety
    /// The place must cover live writable aggregate storage with valid bounds,
    /// alignment, aliasing, and ownership permissions for the byte copy; no old-value load is performed.
    unsafe fn store(self, value: RawRecordValue<R>) -> RawRecordValue<R> {
        // SAFETY: The caller establishes writable aggregate storage.
        unsafe { store_record(self.address.cast(), self.access, value) }
    }
}
// SAFETY: The raw aggregate's representation and write contract are identical.
/// Expose assignment only for this admitted writable destination family.
unsafe impl<R> WritePlace for Place<CRawRecord<R>> {
    /// C destination kind used for assignment conversion before storing the object.
    type Assignment = CRawRecord<R>;
    /// Write the converted destination value without reading or dropping the previous object.
    ///
    /// # Safety
    /// The place must cover live writable aggregate storage with valid bounds,
    /// alignment, aliasing, and ownership permissions for the byte copy; no old-value load is performed.
    unsafe fn store(self, value: RawRecordValue<R>) -> RawRecordValue<R> {
        // SAFETY: The caller establishes writable aggregate storage.
        unsafe { store_record(self.address, self.access, value) }
    }
}

/// Prove raw aggregate copying and field assignment never require whole-record initialization.
/// A destructor counter separately checks that raw storage cannot drop its hidden non-Copy record.
#[cfg(test)]
mod tests {
    use super::super::super::CUnsignedInt;
    use super::*;

    /// Provide a record with independently initialized fields to test address-only projection and raw aggregate copies.
    #[repr(C)]
    struct Partial {
        /// Give the fixture a separately addressable field whose initialization and qualification can be tested independently.
        first: u32,
        /// Leave a Rust-validity-sensitive field uninitialized to detect accidental whole-record loads.
        validity: bool,
    }
    /// Keep this runtime family within crate-owned C capability registration.
    impl sealed::Sealed for Partial {}
    /// Associate admitted native storage with its C marker for macro input and place resolution.
    impl NativeType for Partial {
        /// C declaration identity attached to this expression value or registered native storage.
        type Marker = CRecord<Self>;
    }
    /// Select the independently initialized integer field in the record projection fixtures.
    struct First;
    // SAFETY: This exact repr(C) field projection retains Q and never loads R.
    /// Project exact typed fields while retaining the caller's allocation and access obligations.
    unsafe impl<Q: Qualifier> Field<First, Q> for CRecord<Partial> {
        /// Qualified field place produced without loading the containing record.
        type Output = Place<CUnsignedInt, Q>;
        /// Resolve the declared field address while retaining allocation provenance, packing, and access qualification.
        ///
        /// # Safety
        /// The base must permit the delegated field projection within a live allocation;
        /// the containing record may be partial, and only a subsequently read field needs initialization.
        unsafe fn project(base: Place<Self, Q>) -> Self::Output {
            // SAFETY: The caller proves this complete Partial allocation; no
            // reference is formed and its other field may remain uninitialized.
            let first = unsafe { &raw mut (*base.address).first };
            Place::new(Q::from_mut(first), base.access)
        }
    }

    /// Prove partial non-`Copy` aggregates can be copied and assigned without materializing their uninitialized fields.
    #[test]
    fn partial_noncopy_records_can_be_copied_assigned_and_projected() {
        let mut source = MaybeUninit::<Partial>::uninit();
        let source_place = place::<CRecord<Partial>>(source.as_mut_ptr());
        // SAFETY: Only first is initialized; raw aggregate operations admit the
        // remaining uninitialized bool and read no unrelated field.
        let value = unsafe {
            assign(project::<First, _, _>(source_place), input(37_i32));
            load(source_place)
        };
        // SAFETY: The selected first field was initialized above.
        assert_eq!(unsafe { member::<First, _>(value.clone()) }.get(), 37);
        let mut destination = MaybeUninit::<Partial>::uninit();
        // SAFETY: Destination has full writable aggregate extent; assignment
        // copies raw bytes without reading its previous uninitialized storage.
        let assigned = unsafe { assign(native_place(&raw mut destination), value) };
        // SAFETY: Both selected first fields retain their initialized value.
        assert_eq!(unsafe { member::<First, _>(assigned) }.get(), 37);
        assert_eq!(
            unsafe { load(project::<First, _, _>(native_const_place(&raw const destination))) }
                .get(),
            37
        );
        let first = Pointer::<CRecord<Partial>>::new(destination.as_mut_ptr());
        let raw: Pointer<CRawRecord<Partial>> = implicit::<CPointer<CRawRecord<Partial>>, _>(first);
        assert_eq!(raw.get().cast::<Partial>(), first.get());
        let size = size_of_value_type(None::<RawRecordValue<Partial>>);
        assert_eq!(size.get(), core::mem::size_of::<Partial>() as u64);
    }

    /// Use a destructor counter to prove raw aggregate wrappers never run the hidden record destructor.
    #[test]
    fn raw_storage_does_not_drop_r_even_when_r_is_not_copy() {
        use core::sync::atomic::{AtomicUsize, Ordering};
        /// Count forbidden hidden-record destruction in the raw aggregate storage regression.
        static DROPS: AtomicUsize = AtomicUsize::new(0);
        /// Expose accidental whole-record destruction through a test-local counter.
        struct HasDrop;
        /// Clean up this fixture's owned resource or expose unexpected record destruction.
        impl Drop for HasDrop {
            /// Increment the fixture counter to detect accidental hidden-record destruction.
            fn drop(&mut self) {
                DROPS.fetch_add(1, Ordering::Relaxed);
            }
        }
        let value = RawRecordValue::initialized(HasDrop);
        let duplicate = value.clone();
        drop(value);
        drop(duplicate);
        assert_eq!(DROPS.load(Ordering::Relaxed), 0);
    }
}
