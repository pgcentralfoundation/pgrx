// Shared generated C macro support.

#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2> as c::expression::OrdinaryField<Field_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).offset) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioResult>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioResult>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioResult {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioResult, _bitfield_1) * 8 + 0 == 0
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
unsafe extern "C" {
    fn __pgrx_bitfield__get__PgAioResult__id(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set__PgAioResult__id(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__PgAioResult__id(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__PgAioResult__id(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__PgAioResult__id(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__PgAioResult__id(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__PgAioResult__id(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__PgAioResult__id(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Bitfield_PgAioResult__id<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::PgAioResult>,
    access: c::expression::Access,
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Copy for Bitfield_PgAioResult__id<Q> {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Clone for Bitfield_PgAioResult__id<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_id, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioResult>
{
    type Output = Bitfield_PgAioResult__id<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_PgAioResult__id::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace for Bitfield_PgAioResult__id<Q> {
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace for Bitfield_PgAioResult__id<Q> {
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__PgAioResult__id(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__PgAioResult__id(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__PgAioResult__id(pointer)
            } else {
                __pgrx_bitfield__get__PgAioResult__id(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace for Bitfield_PgAioResult__id<c::expression::ReadWrite> {
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__PgAioResult__id(self.address, value)
                } else {
                    __pgrx_bitfield__set_unaligned__PgAioResult__id(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__PgAioResult__id(self.address, value)
            } else {
                __pgrx_bitfield__set__PgAioResult__id(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioResult, _bitfield_1) * 8 + 6 == 6
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
unsafe extern "C" {
    fn __pgrx_bitfield__get__PgAioResult__status(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set__PgAioResult__status(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__PgAioResult__status(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__PgAioResult__status(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__PgAioResult__status(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__PgAioResult__status(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__PgAioResult__status(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__PgAioResult__status(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Bitfield_PgAioResult__status<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::PgAioResult>,
    access: c::expression::Access,
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Copy for Bitfield_PgAioResult__status<Q> {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Clone for Bitfield_PgAioResult__status<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_status, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioResult>
{
    type Output = Bitfield_PgAioResult__status<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_PgAioResult__status::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace for Bitfield_PgAioResult__status<Q> {
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_PgAioResult__status<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__PgAioResult__status(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__PgAioResult__status(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__PgAioResult__status(pointer)
            } else {
                __pgrx_bitfield__get__PgAioResult__status(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace for Bitfield_PgAioResult__status<c::expression::ReadWrite> {
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__PgAioResult__status(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__PgAioResult__status(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__PgAioResult__status(self.address, value)
            } else {
                __pgrx_bitfield__set__PgAioResult__status(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_error_data;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioResult, _bitfield_1) * 8 + 9 == 9
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
unsafe extern "C" {
    fn __pgrx_bitfield__get__PgAioResult__error_data(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set__PgAioResult__error_data(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__PgAioResult__error_data(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__PgAioResult__error_data(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__PgAioResult__error_data(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__PgAioResult__error_data(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__PgAioResult__error_data(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__PgAioResult__error_data(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Bitfield_PgAioResult__error_data<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::PgAioResult>,
    access: c::expression::Access,
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Copy for Bitfield_PgAioResult__error_data<Q> {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Clone for Bitfield_PgAioResult__error_data<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_error_data, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioResult>
{
    type Output = Bitfield_PgAioResult__error_data<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_PgAioResult__error_data::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_PgAioResult__error_data<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_PgAioResult__error_data<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__PgAioResult__error_data(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__PgAioResult__error_data(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__PgAioResult__error_data(pointer)
            } else {
                __pgrx_bitfield__get__PgAioResult__error_data(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_PgAioResult__error_data<c::expression::ReadWrite>
{
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__PgAioResult__error_data(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__PgAioResult__error_data(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__PgAioResult__error_data(self.address, value)
            } else {
                __pgrx_bitfield__set__PgAioResult__error_data(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioResult, result) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_result>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioResult> as c::expression::OrdinaryField<Field_result>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).result) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1>() == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1>() == 4
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1 {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1, blockNum)
        == 12
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blockNum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1> as c::expression::OrdinaryField<Field_blockNum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blockNum) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1, nblocks) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1> as c::expression::OrdinaryField<Field_nblocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nblocks) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_aio_index;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioWaitRef>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioWaitRef>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioWaitRef {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioWaitRef, aio_index) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aio_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioWaitRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef> as c::expression::OrdinaryField<Field_aio_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aio_index) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_generation_upper;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioWaitRef, generation_upper) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generation_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioWaitRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef> as c::expression::OrdinaryField<Field_generation_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generation_upper) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_generation_lower;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioWaitRef, generation_lower) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generation_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioWaitRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef> as c::expression::OrdinaryField<Field_generation_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generation_lower) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgBackendStatus>() == 440);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgBackendStatus>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_query_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 424;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_st_plan_id;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_plan_id) == 432);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_plan_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_plan_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_plan_id) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_Backend>() == 2920);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_Backend>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_Backend {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_Backend, stat_reset_timestamp) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_Backend>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_Backend, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_Backend> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_num_performed;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, num_performed)
        == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_performed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_num_performed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_performed) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        restartpoints_timed
    ) == 24
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restartpoints_timed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        restartpoints_requested
    ) == 32
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restartpoints_requested>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        restartpoints_performed
    ) == 40
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restartpoints_performed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, write_time) == 48
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_write_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, sync_time) == 56
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, buffers_written)
        == 64
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffers_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_slru_written;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, slru_written) == 72
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slru_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_slru_written>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slru_written) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        stat_reset_timestamp
    ) == 80
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_IO>() == 51848);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_IO>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatDBEntry>() == 264);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatDBEntry>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_parallel_workers_to_launch;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatDBEntry,
        parallel_workers_to_launch
    ) == 240
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_workers_to_launch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_parallel_workers_to_launch>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_workers_to_launch) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_parallel_workers_launched;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatDBEntry,
        parallel_workers_launched
    ) == 248
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_workers_launched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_parallel_workers_launched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_workers_launched) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, stat_reset_timestamp)
        == 256
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatSubEntry>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatSubEntry>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatSubEntry, stat_reset_timestamp)
        == 72
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatTabEntry>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatTabEntry>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_total_vacuum_time;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, total_vacuum_time)
        == 184
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_vacuum_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_total_vacuum_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_vacuum_time) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_total_autovacuum_time;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, total_autovacuum_time)
        == 192
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_autovacuum_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_total_autovacuum_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_autovacuum_time) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_total_analyze_time;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, total_analyze_time)
        == 200
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_analyze_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_total_analyze_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_analyze_time) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_total_autoanalyze_time;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, total_autoanalyze_time)
        == 208
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_autoanalyze_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_total_autoanalyze_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_autoanalyze_time) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_WalCounters>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_WalCounters>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_WalCounters {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_records) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_records>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters> as c::expression::OrdinaryField<Field_wal_records>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_records) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_fpi) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_fpi>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters> as c::expression::OrdinaryField<Field_wal_fpi>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_fpi) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_bytes) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters> as c::expression::OrdinaryField<Field_wal_bytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_bytes) };
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_buffers_full) == 24
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_buffers_full>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters> as c::expression::OrdinaryField<Field_wal_buffers_full>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_buffers_full) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_WalStats>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_WalStats>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, stat_reset_timestamp) == 32
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, disabled_nodes) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_disabled_nodes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_disabled_nodes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).disabled_nodes) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 160);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_queryId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_planId;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, planId) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_planId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_planId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).planId) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, hasReturning) == 24);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasReturning>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, hasModifyingCTE) == 25);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasModifyingCTE>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, canSetTag) == 26);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canSetTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 26;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, transientPlan) == 27);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transientPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 27;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, dependsOnRole) == 28);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, parallelModeNeeded) == 29
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeNeeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 29;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, jitFlags) == 32);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jitFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_partPruneInfos;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, partPruneInfos) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partPruneInfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_partPruneInfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partPruneInfos) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, rtable) == 56);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, permInfos) == 72);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_permInfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, resultRelations) == 80);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, appendRelations) == 88);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, subplans) == 96);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, rowMarks) == 112);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, relationOids) == 120);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, invalItems) == 128);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, paramExecTypes) == 136);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramExecTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, stmt_location) == 152);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, stmt_len) == 156);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 168);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrteperminfos) == 72
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrowmarks) == 80);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrowmarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, resultRelations) == 88
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, appendRelations) == 96
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, partPruneInfos) == 104
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partPruneInfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_partPruneInfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partPruneInfos) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, relationOids) == 112);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, invalItems) == 120);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, paramExecTypes) == 128
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramExecTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPHId) == 136);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPHId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastRowMarkId) == 140);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastRowMarkId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPlanNodeId) == 144
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPlanNodeId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, transientPlan) == 148);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transientPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 148;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, dependsOnRole) == 149);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 149;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeOK) == 150
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeOK>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 150;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeNeeded) == 151
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeNeeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 151;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, maxParallelHazard) == 152
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxParallelHazard>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 704);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_group_rtindex;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, group_rtindex) == 604);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_group_rtindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 604;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_group_rtindex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).group_rtindex) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, partPruneInfos) == 696);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partPruneInfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 696;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_partPruneInfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partPruneInfos) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PortalData>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PortalData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, status) == 128);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PortalStatus,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, portalPinned) == 132);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_portalPinned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, autoHeld) == 133);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_autoHeld>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 133;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, atStart) == 192);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atStart>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, atEnd) == 193);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atEnd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 193;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, portalPos) == 200);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_portalPos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, creation_time) == 208);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_creation_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, visible) == 216);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_visible>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ProjectionPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ProjectionPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ProjectionPath, dummypp) == 88);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dummypp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ProjectionPath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Publication>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Publication>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_pubgencols_type;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Publication, pubgencols_type) == 20);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubgencols_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Publication>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PublishGencolsType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Publication, <<c::expression::CRecord<crate::__pgrx_c_bindings::Publication> as c::expression::OrdinaryField<Field_pubgencols_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pubgencols_type) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PublicationDesc>() == 10);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PublicationDesc>() == 1);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_gencols_valid_for_update;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationDesc, gencols_valid_for_update)
        == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gencols_valid_for_update>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc> as c::expression::OrdinaryField<Field_gencols_valid_for_update>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).gencols_valid_for_update) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_gencols_valid_for_delete;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationDesc, gencols_valid_for_delete)
        == 9
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gencols_valid_for_delete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc> as c::expression::OrdinaryField<Field_gencols_valid_for_delete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).gencols_valid_for_delete) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Query>() == 280);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Query>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_queryId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_hasGroupRTE;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, hasGroupRTE) == 53);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasGroupRTE>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 53;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_hasGroupRTE,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasGroupRTE) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, isReturn) == 54);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isReturn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 54;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, returningList) == 152);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupClause) == 160);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupDistinct) == 168);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupDistinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupingSets) == 176);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupingSets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, windowClause) == 192);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_windowClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, distinctClause) == 200);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinctClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, sortClause) == 208);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, limitOption) == 232);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_limitOption>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LimitOption,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, rowMarks) == 240);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, constraintDeps) == 256);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constraintDeps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, withCheckOptions) == 264);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_withCheckOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, stmt_location) == 272);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, stmt_len) == 276);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 276;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeTblEntry>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeTblEntry>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_groupexprs;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, groupexprs) == 200);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_groupexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupexprs) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, lateral) == 208);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lateral>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, inFromCl) == 209);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inFromCl>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 209;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, securityQuals) == 216);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_securityQuals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RawColumnDefault, generated) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RawColumnDefault>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_persistence;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReadBuffersOperation>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReadBuffersOperation>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReadBuffersOperation, persistence) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_persistence>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReadBuffersOperation, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation> as c::expression::OrdinaryField<Field_persistence>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).persistence) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_nblocks_done;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReadBuffersOperation, nblocks_done) == 50
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nblocks_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 50;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReadBuffersOperation, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation> as c::expression::OrdinaryField<Field_nblocks_done>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nblocks_done) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RecursiveUnionPath>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RecursiveUnionPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RecursiveUnionPath, distinctList) == 96
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinctList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RecursiveUnionPath, wtParam) == 104);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wtParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionPath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_pid) == 1472);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1472;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_crashed_at) == 1480
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_crashed_at>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1480;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_shmem_slot) == 1488
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_shmem_slot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1488;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_terminate) == 1492
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_terminate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1492;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBufferTXN>() == 336);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBufferTXN>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, ninvalidations_distributed)
        == 240
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ninvalidations_distributed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, size) == 312);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, total_size) == 320);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReplicationSlot>() == 288);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReplicationSlot>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_saved_restart_lsn;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, last_saved_restart_lsn)
        == 280
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_saved_restart_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_last_saved_restart_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_saved_restart_lsn) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ResultRelInfo>() == 400);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ResultRelInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ri_extraUpdatedCols_valid;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_extraUpdatedCols_valid)
        == 56
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_extraUpdatedCols_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_extraUpdatedCols_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_extraUpdatedCols_valid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_projectNewInfoValid) == 88
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_projectNewInfoValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_needLockTagTuple) == 89
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_needLockTagTuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 89;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_usesFdwDirectModify) == 176
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_usesFdwDirectModify>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumSlots) == 180);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumSlots>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumSlotsInitialized) == 184
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumSlotsInitialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_BatchSize) == 188);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_BatchSize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 188;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_WithCheckOptions) == 208
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_WithCheckOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_WithCheckOptionExprs)
        == 216
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_WithCheckOptionExprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumGeneratedNeededI) == 256
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumGeneratedNeededI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumGeneratedNeededU) == 260
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumGeneratedNeededU>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 260;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_returningList) == 264
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_onConflictArbiterIndexes)
        == 280
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_onConflictArbiterIndexes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_ChildToRootMapValid) == 344
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_ChildToRootMapValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_RootToChildMapValid) == 360
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_RootToChildMapValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_ancestorResultRels) == 392
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_ancestorResultRels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 392;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReturningClause>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReturningClause>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReturningClause {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningClause, type_) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningClause, options) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningClause, exprs) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause> as c::expression::OrdinaryField<Field_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exprs) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_retlevelsup;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReturningExpr>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReturningExpr>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReturningExpr {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningExpr, retlevelsup) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retlevelsup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningExpr> as c::expression::OrdinaryField<Field_retlevelsup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retlevelsup) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_retold;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningExpr, retold) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningExpr> as c::expression::OrdinaryField<Field_retold>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retold) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReturningOption>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReturningOption>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReturningOption {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningOption, type_) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningOption, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_option;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningOption, option) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_option>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ReturningOptionKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningOption, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption> as c::expression::OrdinaryField<Field_option>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).option) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningOption, value) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningOption, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningOption, location) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningOption, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowCompareExpr, cmptype) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmptype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CompareType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowCompareExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr> as c::expression::OrdinaryField<Field_cmptype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmptype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_bytesSent;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SerializeMetrics>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SerializeMetrics>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SerializeMetrics {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SerializeMetrics, bytesSent) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bytesSent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SerializeMetrics>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SerializeMetrics, <<c::expression::CRecord<crate::__pgrx_c_bindings::SerializeMetrics> as c::expression::OrdinaryField<Field_bytesSent>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bytesSent) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetOpPath>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetOpPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpPath, cmd) == 96);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SetOpCmd,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpPath, strategy) == 100);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SetOpStrategy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_groupList;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpPath, groupList) == 104);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOpPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath> as c::expression::OrdinaryField<
        Field_groupList,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupList) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetOpState>() == 352);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetOpState>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, setop_done) == 200);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_setop_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, numOutput) == 208);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numOutput>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, numCols) == 216);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOpState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState> as c::expression::OrdinaryField<Field_numCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numCols) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_need_init;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, need_init) == 296);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_need_init>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOpState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState> as c::expression::OrdinaryField<Field_need_init>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).need_init) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, table_filled) == 336);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_table_filled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_numTuples;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetOpStatePerInput>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetOpStatePerInput>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SetOpStatePerInput {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpStatePerInput, numTuples) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numTuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpStatePerInput>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOpStatePerInput, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpStatePerInput> as c::expression::OrdinaryField<Field_numTuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numTuples) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_needGroup;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpStatePerInput, needGroup) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_needGroup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpStatePerInput>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOpStatePerInput, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpStatePerInput> as c::expression::OrdinaryField<Field_needGroup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).needGroup) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation>() == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation>() == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation, num_workers)
        == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::SharedIndexScanInstrumentation>() == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::SharedIndexScanInstrumentation>() == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedIndexScanInstrumentation {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedIndexScanInstrumentation, num_workers)
        == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedIndexScanInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedIndexScanInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedIndexScanInstrumentation> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_low_elem;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SkipSupportData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SkipSupportData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SkipSupportData {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SkipSupportData, low_elem) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_low_elem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SkipSupportData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SkipSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SkipSupportData> as c::expression::OrdinaryField<Field_low_elem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).low_elem) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_high_elem;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SkipSupportData, high_elem) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_high_elem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SkipSupportData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SkipSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SkipSupportData> as c::expression::OrdinaryField<Field_high_elem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).high_elem) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SnapBuild>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SnapBuild>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SnapBuild {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, state) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_SnapBuildState,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_state,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).state) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, context) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_context,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).context) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, xmin) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_xmin,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmin) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, xmax) == 20);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_xmax,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmax) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_start_decoding_at;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, start_decoding_at) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_start_decoding_at>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_start_decoding_at,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).start_decoding_at) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, two_phase_at) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_two_phase_at>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_two_phase_at,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).two_phase_at) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_initial_xmin_horizon;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, initial_xmin_horizon) == 40
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initial_xmin_horizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_initial_xmin_horizon,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initial_xmin_horizon) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_building_full_snapshot;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, building_full_snapshot) == 44
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_building_full_snapshot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_building_full_snapshot,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).building_full_snapshot) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_in_slot_creation;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, in_slot_creation) == 45);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_slot_creation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 45;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_in_slot_creation,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_slot_creation) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_serialized_snapshot;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, last_serialized_snapshot) == 56
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_serialized_snapshot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_last_serialized_snapshot,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_serialized_snapshot) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_next_phase_at;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, next_phase_at) == 72);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_next_phase_at>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_next_phase_at,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).next_phase_at) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1 {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1, xcnt) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xcnt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1> as c::expression::OrdinaryField<Field_xcnt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xcnt) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_xcnt_space;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1, xcnt_space) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xcnt_space>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1> as c::expression::OrdinaryField<Field_xcnt_space>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xcnt_space) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_includes_all_transactions;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1,
        includes_all_transactions
    ) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_includes_all_transactions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1> as c::expression::OrdinaryField<Field_includes_all_transactions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).includes_all_transactions) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_2>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_2>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_2 {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_2, xcnt) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xcnt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_2>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_2> as c::expression::OrdinaryField<Field_xcnt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xcnt) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SnapBuildOnDisk>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SnapBuildOnDisk>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SnapBuildOnDisk {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuildOnDisk, magic) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuildOnDisk, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk> as c::expression::OrdinaryField<Field_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magic) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_checksum;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuildOnDisk, checksum) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_checksum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuildOnDisk, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk> as c::expression::OrdinaryField<Field_checksum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).checksum) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuildOnDisk, version) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuildOnDisk, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk> as c::expression::OrdinaryField<Field_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).version) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuildOnDisk, length) == 12);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuildOnDisk, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk> as c::expression::OrdinaryField<Field_length>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).length) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SnapshotData>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SnapshotData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, snapXactCompletionCount) == 96
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapXactCompletionCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_reverse_sort;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, reverse_sort) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reverse_sort>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortGroupClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause> as c::expression::OrdinaryField<Field_reverse_sort>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reverse_sort) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, nulls_first) == 17);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nulls_first>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, hashable) == 18);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_argtype;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StatsArgInfo>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StatsArgInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::StatsArgInfo {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StatsArgInfo, argtype) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_argtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StatsArgInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StatsArgInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::StatsArgInfo> as c::expression::OrdinaryField<Field_argtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).argtype) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StdRdOptions>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StdRdOptions>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, user_catalog_table) == 104
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_user_catalog_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, parallel_workers) == 108
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, vacuum_index_cleanup) == 112
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_index_cleanup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__StdRdOptIndexCleanup,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, vacuum_truncate) == 116
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_truncate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
#[cfg(feature = "pg18")]
#[doc(hidden)]
pub struct Field_vacuum_truncate_set;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, vacuum_truncate_set) == 117
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_truncate_set>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 117;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdRdOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions> as c::expression::OrdinaryField<Field_vacuum_truncate_set>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vacuum_truncate_set) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubPlanState>() == 192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubPlanState>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, curArray) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curArray>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, havehashrows) == 96);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_havehashrows>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, havenullrows) == 97);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_havenullrows>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 97;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, numCols) == 128);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestModifyInPlace {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestModifyInPlace, type_) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestModifyInPlace, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestModifyInPlace, funcid) == 4
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestModifyInPlace, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace> as c::expression::OrdinaryField<Field_funcid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestModifyInPlace, args) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestModifyInPlace, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestModifyInPlace, paramid) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestModifyInPlace, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace> as c::expression::OrdinaryField<Field_paramid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TBMIterateResult>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TBMIterateResult>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TBMIterateResult, lossy) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lossy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TBMIterateResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult> as c::expression::OrdinaryField<Field_lossy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lossy) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TBMIterateResult, recheck) == 5);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recheck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TBMIterator>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TBMIterator>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TBMIterator {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TBMIterator, shared) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterator>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TBMIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterator> as c::expression::OrdinaryField<Field_shared>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).shared) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableScanDescData>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableScanDescData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableScanDescData, rs_flags) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidStoreIterResult>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidStoreIterResult>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_has_generated_virtual;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleConstr, has_generated_virtual) == 30
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_generated_virtual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleConstr, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr> as c::expression::OrdinaryField<Field_has_generated_virtual>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_generated_virtual) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_compact_attrs;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, compact_attrs) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::OffsetField<Field_compact_attrs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, compact_attrs);
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleHashEntryData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleHashEntryData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashEntryData, status) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashEntryData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashEntryData, hash) == 12);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashEntryData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleHashTableData>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleHashTableData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_additionalsize;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashTableData, additionalsize) == 64
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_additionalsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleHashTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData> as c::expression::OrdinaryField<Field_additionalsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).additionalsize) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TypeCacheEntry>() == 496);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TypeCacheEntry>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typarray) == 28);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typarray>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_typarray>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typarray) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typcollation) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typcollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, btree_opf) == 36);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btree_opf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, btree_opintype) == 40
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btree_opintype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_opf) == 44);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_opf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_opintype) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_opintype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, eq_opr) == 52);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eq_opr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, lt_opr) == 56);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lt_opr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, gt_opr) == 60);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gt_opr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, cmp_proc) == 64);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmp_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_proc) == 68);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_extended_proc) == 72
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_extended_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, tupDesc_identifier) == 280
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tupDesc_identifier>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, rng_opfamily) == 296);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rng_opfamily>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, rng_collation) == 300
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rng_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 300;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, domainBaseType) == 456
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_domainBaseType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 456;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, domainBaseTypmod) == 460
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_domainBaseTypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 460;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, flags) == 472);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 472;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UniquePath>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UniquePath>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniquePath, umethod) == 88);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_umethod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__UniquePathMethod,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniquePath, in_operators) == 96);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_operators>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniquePath, uniq_exprs) == 104);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_uniq_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UniqueRelInfo>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UniqueRelInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::UniqueRelInfo {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniqueRelInfo, type_) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UniqueRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_self_join;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniqueRelInfo, self_join) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_self_join>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UniqueRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo> as c::expression::OrdinaryField<Field_self_join>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).self_join) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_extra_clauses;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniqueRelInfo, extra_clauses) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extra_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UniqueRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo> as c::expression::OrdinaryField<Field_extra_clauses>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extra_clauses) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UpperUniquePath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UpperUniquePath>() == 8);
};
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UpperUniquePath, numkeys) == 88);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UpperUniquePath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Var>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Var>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varreturningtype) == 36);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varreturningtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VarReturningType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Var,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Var> as c::expression::OrdinaryField<
        Field_varreturningtype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varreturningtype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varnosyn) == 40);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varattnosyn) == 44);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varattnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, location) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_jumble_args;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableSetStmt, jumble_args) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumble_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt> as c::expression::OrdinaryField<Field_jumble_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumble_args) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableSetStmt, is_local) == 25);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_local>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableSetStmt, location) == 28);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::WalReceiverConn,
                    *const ::core::ffi::c_char,
                    *const bool,
                    *const bool,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::WalReceiverConn,
                    *const ::core::ffi::c_char,
                    *const bool,
                    *const bool,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_alter_slot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__const__0x5f426f6f6c__0p__0c__const__0x5f426f6f6c__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalSnd, replyTime) == 80);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replyTime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalSnd>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalSnd, kind) == 88);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalSnd>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ReplicationKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowAgg>() == 232);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowAgg>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, winref) == 112);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_winref>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, partNumCols) == 116);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partNumCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, ordNumCols) == 144);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ordNumCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, frameOptions) == 176);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frameOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, runCondition) == 200);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runCondition>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, runConditionOrig) == 208);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runConditionOrig>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, startInRangeFunc) == 216);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startInRangeFunc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, endInRangeFunc) == 220);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_endInRangeFunc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 220;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, inRangeColl) == 224);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeColl>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, inRangeAsc) == 228);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeAsc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, inRangeNullsFirst) == 229);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeNullsFirst>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 229;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, topWindow) == 230);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_topWindow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 230;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowAggPath>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowAggPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggPath, qual) == 96);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_qual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggPath, runCondition) == 104);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runCondition>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggPath, topwindow) == 112);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_topwindow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggPath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, use_pass_through) == 494
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_use_pass_through>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 494;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, top_window) == 495);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_top_window>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 495;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, currentgroup) == 504);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_currentgroup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 504;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frameheadgroup) == 512
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frameheadgroup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 512;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frametailgroup) == 520
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frametailgroup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 520;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, groupheadpos) == 528);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupheadpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, grouptailpos) == 536);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grouptailpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 536;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, all_first) == 576);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_all_first>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 576;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, partition_spooled) == 577
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partition_spooled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 577;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_next_partition;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, next_partition) == 578
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_next_partition>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 578;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_next_partition>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).next_partition) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, more_partitions) == 579
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_more_partitions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 579;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, framehead_valid) == 580
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_framehead_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 580;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frametail_valid) == 581
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frametail_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 581;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, grouptail_valid) == 582
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grouptail_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 582;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*const ::core::ffi::c_void>() == 8);
    assert!(::core::mem::align_of::<*const ::core::ffi::c_void>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecData>
{
    type Member = c::expression::CPointer<c::expression::CVoid, c::expression::ReadOnly>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_locale_struct>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_locale_struct>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_collate_is_c;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_locale_struct, collate_is_c) == 2);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collate_is_c>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_locale_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct> as c::expression::OrdinaryField<Field_collate_is_c>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collate_is_c) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ctype_is_c;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_locale_struct, ctype_is_c) == 3);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctype_is_c>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 3;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_locale_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct> as c::expression::OrdinaryField<Field_ctype_is_c>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctype_is_c) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_locale_struct, is_default) == 4);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_default>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_locale_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct> as c::expression::OrdinaryField<Field_is_default>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_default) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_casemap_full;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1,
        casemap_full
    ) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_casemap_full>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1,
    >
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1> as c::expression::OrdinaryField<Field_casemap_full>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).casemap_full) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_heap_inplace>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_heap_inplace>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_inplace, dbId) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_inplace, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace> as c::expression::OrdinaryField<Field_dbId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbId) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_inplace, tsId) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tsId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_inplace, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace> as c::expression::OrdinaryField<Field_tsId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tsId) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_inplace, relcacheInitFileInval) == 12
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relcacheInitFileInval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_inplace, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace> as c::expression::OrdinaryField<Field_relcacheInitFileInval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relcacheInitFileInval) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_inplace, nmsgs) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nmsgs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_inplace, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace> as c::expression::OrdinaryField<Field_nmsgs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nmsgs) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_xact_stats_item>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_xact_stats_item>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_objid_lo;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_xact_stats_item, objid_lo) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objid_lo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_stats_item>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_xact_stats_item, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_stats_item> as c::expression::OrdinaryField<Field_objid_lo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objid_lo) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_objid_hi;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_xact_stats_item, objid_hi) == 12);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objid_hi>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_stats_item>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_xact_stats_item, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_stats_item> as c::expression::OrdinaryField<Field_objid_hi>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objid_hi) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1>() == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1>() == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1 {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TBMIterator__bindgen_ty_1>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TBMIterator__bindgen_ty_1>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TBMIterator__bindgen_ty_1 {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioTargetData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioTargetData>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioTargetData {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::derives_hash {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::derives_hash {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidRangePath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidRangePath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1__bindgen_ty_1,
        >() == 12
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1__bindgen_ty_1,
        >() == 2
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1__bindgen_ty_1
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::TBMPrivateIterator {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::TBMPrivateIterator {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubqueryScanPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubqueryScanPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SortPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SortPath>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReplicationSlotCtlData>() == 288);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReplicationSlotCtlData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelPathStr>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelPathStr>() == 1);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RelPathStr {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ProjectSetPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ProjectSetPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::ProcSignalHeader {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::ProcSignalHeader {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_PendingIO>() == 2880);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_PendingIO>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_PendingIO {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BktypeIO>() == 2880);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BktypeIO>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BackendPending>() == 2880);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BackendPending>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_BackendPending {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioTargetInfo>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioTargetInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioTargetInfo {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioReturn>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioReturn>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioReturn {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioHandleCallbacks>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioHandleCallbacks>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioHandleCallbacks {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NestPath>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NestPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MinMaxAggPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MinMaxAggPath>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeAppendPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeAppendPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MaterialPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MaterialPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GroupResultPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GroupResultPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GroupPath>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GroupPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ForeignPath>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ForeignPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_48>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_48>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_48
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_47>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_47>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_47
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_39>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_39>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_36>(
        ) == 40
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_36>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_29>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_29>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_22>(
        ) == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_22>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_18>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_18>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_17>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_17>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_12>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_12>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::DynamicFileList {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::DynamicFileList {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateForeignTableStmt>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateForeignTableStmt>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyToRoutine>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyToRoutine>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CopyToRoutine {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFromRoutine>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFromRoutine>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CopyFromRoutine {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapOrPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapOrPath>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanDescData>() == 736);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanDescData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapHeapScanDescData {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapAndPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapAndPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioOpData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioOpData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioOpData {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelPathStr>>() == 72
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelPathStr>>() == 1
);
