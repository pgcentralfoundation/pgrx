//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Aggregate copies preserve uninitialized C fields until a selected field is read.

use super::*;
use core::mem::MaybeUninit;

pub struct CRawRecord<R>(PhantomData<R>);
impl<R> Copy for CRawRecord<R> {}
impl<R> Clone for CRawRecord<R> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<R> sealed::Sealed for CRawRecord<R> {}
impl<R> CompleteObject for CRawRecord<R> {}
impl<R, const B: bool> AllowedProfile<B> for CRawRecord<R> {}

/// A copied C aggregate that need not contain a valid initialized Rust record.
/// No operation on this wrapper runs the record's destructor.
pub struct RawRecordValue<R>(MaybeUninit<R>);
impl<R: Copy> Copy for RawRecordValue<R> {}
impl<R> Clone for RawRecordValue<R> {
    fn clone(&self) -> Self {
        // SAFETY: MaybeUninit admits uninitialized bytes and preserves the
        // non-padding representation/provenance. Neither copy drops an R.
        Self(unsafe { core::ptr::read(&self.0) })
    }
}
impl<R> sealed::Sealed for RawRecordValue<R> {}
impl<R> RawRecordValue<R> {
    pub const fn new(storage: MaybeUninit<R>) -> Self {
        Self(storage)
    }
    pub const fn initialized(value: R) -> Self {
        Self(MaybeUninit::new(value))
    }
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
impl<R> CType for CRawRecord<R> {
    type Storage = MaybeUninit<R>;
    type Value = RawRecordValue<R>;
    fn from_storage(storage: Self::Storage) -> Self::Value {
        RawRecordValue::new(storage)
    }
    fn into_storage(value: Self::Value) -> Self::Storage {
        value.get()
    }
}
impl<R> CExprValue for RawRecordValue<R> {
    type Marker = CRawRecord<R>;
}
impl<R> IntoExpression for RawRecordValue<R> {
    type Value = Self;
    fn into_expression(self) -> Self {
        self
    }
}
impl<R: NativeType<Marker = CRecord<R>>> sealed::Sealed for MaybeUninit<R> {}
impl<R: NativeType<Marker = CRecord<R>>> NativeType for MaybeUninit<R> {
    type Marker = CRawRecord<R>;
}
impl<R: NativeType<Marker = CRecord<R>>> IntoExpression for MaybeUninit<R> {
    type Value = RawRecordValue<R>;
    fn into_expression(self) -> Self::Value {
        RawRecordValue::new(self)
    }
}
impl<R> NonVoidIdentity for CRawRecord<R> {}
impl<R> PointeeIdentity for CRawRecord<R> {
    type Identity = CRecord<R>;
    type Volatility = <CRecord<R> as PointeeIdentity>::Volatility;
}

/// Initialized Rust inputs and raw C aggregate results share address-only member access.
pub trait RecordExpression: CExprValue {
    type Record;
    fn raw_storage(self) -> MaybeUninit<Self::Record>;
}
impl<R> RecordExpression for RawRecordValue<R> {
    type Record = R;
    fn raw_storage(self) -> MaybeUninit<R> {
        self.get()
    }
}
impl<R: Copy> RecordExpression for RecordValue<R> {
    type Record = R;
    fn raw_storage(self) -> MaybeUninit<R> {
        MaybeUninit::new(self.get())
    }
}
impl<R> ImplicitTo<CRawRecord<R>> for RawRecordValue<R> {
    fn implicit_to(self) -> Self {
        self
    }
}
impl<R: Copy> ImplicitTo<CRawRecord<R>> for RecordValue<R> {
    fn implicit_to(self) -> RawRecordValue<R> {
        RawRecordValue::initialized(self.get())
    }
}
impl<R> Select<RawRecordValue<R>> for RawRecordValue<R> {
    type Output = Self;
    fn select(arm: super::super::Either<Self, Self>) -> Self {
        match arm {
            super::super::Either::Left(value) | super::super::Either::Right(value) => value,
        }
    }
}
impl<R: Copy> Select<RawRecordValue<R>> for RecordValue<R> {
    type Output = RawRecordValue<R>;
    fn select(arm: super::super::Either<Self, RawRecordValue<R>>) -> Self::Output {
        match arm {
            super::super::Either::Left(value) => RawRecordValue::initialized(value.get()),
            super::super::Either::Right(value) => value,
        }
    }
}
impl<R: Copy> Select<RecordValue<R>> for RawRecordValue<R> {
    type Output = Self;
    fn select(arm: super::super::Either<Self, RecordValue<R>>) -> Self {
        match arm {
            super::super::Either::Left(value) => value,
            super::super::Either::Right(value) => RawRecordValue::initialized(value.get()),
        }
    }
}

/// Fields copied from a temporary cannot decay an array into a dangling pointer.
pub trait OwnedMemberObject: CType {}
impl<M: ScalarObject> OwnedMemberObject for M {}
impl<R> OwnedMemberObject for CRecord<R> {}
impl<R> OwnedMemberObject for CRawRecord<R> {}
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
unsafe impl<R, F, Q: Qualifier> Field<F, Q> for CRawRecord<R>
where
    CRecord<R>: Field<F, Q>,
{
    type Output = <CRecord<R> as Field<F, Q>>::Output;
    unsafe fn project(base: Place<Self, Q>) -> Self::Output {
        let original = Place::<CRecord<R>, Q>::new(Q::from_mut(base.address.cast()), base.access);
        // SAFETY: The storage cast preserves the original address/layout; the
        // caller establishes the delegated projection's allocation contract.
        unsafe { <CRecord<R> as Field<F, Q>>::project(original) }
    }
}

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
unsafe impl<R, Q: Qualifier> ReadObject<Q> for CRecord<R> {
    type RValue = CRawRecord<R>;
    unsafe fn read_object(place: Place<Self, Q>) -> RawRecordValue<R> {
        // SAFETY: The caller establishes live aggregate storage/read permissions.
        unsafe { read_record(place.address.cast(), place.access) }
    }
}
// SAFETY: The raw storage representation is copied without initializing R.
unsafe impl<R, Q: Qualifier> ReadObject<Q> for CRawRecord<R> {
    type RValue = Self;
    unsafe fn read_object(place: Place<Self, Q>) -> RawRecordValue<R> {
        // SAFETY: The caller establishes live aggregate storage/read permissions.
        unsafe { read_record(place.address, place.access) }
    }
}

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
unsafe impl<R> WritePlace for Place<CRecord<R>> {
    type Assignment = CRawRecord<R>;
    unsafe fn store(self, value: RawRecordValue<R>) -> RawRecordValue<R> {
        // SAFETY: The caller establishes writable aggregate storage.
        unsafe { store_record(self.address.cast(), self.access, value) }
    }
}
// SAFETY: The raw aggregate's representation and write contract are identical.
unsafe impl<R> WritePlace for Place<CRawRecord<R>> {
    type Assignment = CRawRecord<R>;
    unsafe fn store(self, value: RawRecordValue<R>) -> RawRecordValue<R> {
        // SAFETY: The caller establishes writable aggregate storage.
        unsafe { store_record(self.address, self.access, value) }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::CUnsignedInt;
    use super::*;

    #[repr(C)]
    struct Partial {
        first: u32,
        validity: bool,
    }
    impl sealed::Sealed for Partial {}
    impl NativeType for Partial {
        type Marker = CRecord<Self>;
    }
    struct First;
    // SAFETY: This exact repr(C) field projection retains Q and never loads R.
    unsafe impl<Q: Qualifier> Field<First, Q> for CRecord<Partial> {
        type Output = Place<CUnsignedInt, Q>;
        unsafe fn project(base: Place<Self, Q>) -> Self::Output {
            // SAFETY: The caller proves this complete Partial allocation; no
            // reference is formed and its other field may remain uninitialized.
            let first = unsafe { &raw mut (*base.address).first };
            Place::new(Q::from_mut(first), base.access)
        }
    }

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

    #[test]
    fn raw_storage_does_not_drop_r_even_when_r_is_not_copy() {
        use core::sync::atomic::{AtomicUsize, Ordering};
        static DROPS: AtomicUsize = AtomicUsize::new(0);
        struct HasDrop;
        impl Drop for HasDrop {
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
