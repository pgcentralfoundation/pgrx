// Shared generated C macro support.

impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_QueryOperand__length<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_QueryOperand__length<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__QueryOperand__length(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__QueryOperand__length(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__QueryOperand__length(pointer)
            } else {
                __pgrx_bitfield__get__QueryOperand__length(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace for Bitfield_QueryOperand__length<c::expression::ReadWrite> {
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__QueryOperand__length(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__QueryOperand__length(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__QueryOperand__length(self.address, value)
            } else {
                __pgrx_bitfield__set__QueryOperand__length(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_distance;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryOperand, _bitfield_1) * 8 + 12 == 76
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__QueryOperand__distance(
        p: *const crate::__pgrx_c_bindings::QueryOperand,
    ) -> u32;
    fn __pgrx_bitfield__set__QueryOperand__distance(
        p: *mut crate::__pgrx_c_bindings::QueryOperand,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__QueryOperand__distance(
        p: *const crate::__pgrx_c_bindings::QueryOperand,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__QueryOperand__distance(
        p: *mut crate::__pgrx_c_bindings::QueryOperand,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__QueryOperand__distance(
        p: *const crate::__pgrx_c_bindings::QueryOperand,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__QueryOperand__distance(
        p: *mut crate::__pgrx_c_bindings::QueryOperand,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__QueryOperand__distance(
        p: *const crate::__pgrx_c_bindings::QueryOperand,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__QueryOperand__distance(
        p: *mut crate::__pgrx_c_bindings::QueryOperand,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_QueryOperand__distance<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::QueryOperand>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_QueryOperand__distance<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_QueryOperand__distance<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_distance, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperand>
{
    type Output = Bitfield_QueryOperand__distance<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_QueryOperand__distance::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_QueryOperand__distance<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_QueryOperand__distance<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__QueryOperand__distance(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__QueryOperand__distance(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__QueryOperand__distance(pointer)
            } else {
                __pgrx_bitfield__get__QueryOperand__distance(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_QueryOperand__distance<c::expression::ReadWrite>
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
                    __pgrx_bitfield__set_unaligned_volatile__QueryOperand__distance(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__QueryOperand__distance(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__QueryOperand__distance(self.address, value)
            } else {
                __pgrx_bitfield__set__QueryOperand__distance(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::QueryOperator>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::QueryOperator>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::QueryOperator {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryOperator, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperator>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::QueryOperator, <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperator> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_oper;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryOperator, oper) == 1);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperator>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::QueryOperator, <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperator> as c::expression::OrdinaryField<Field_oper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oper) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryOperator, distance) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distance>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperator>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::QueryOperator, <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperator> as c::expression::OrdinaryField<Field_distance>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distance) };
#[doc(hidden)]
pub struct Field_left;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryOperator, left) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_left>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperator>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::QueryOperator, <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperator> as c::expression::OrdinaryField<Field_left>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).left) };
#[doc(hidden)]
pub struct Field_val;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeBound>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeBound>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RangeBound {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeBound, val) == 0);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeBound>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeBound, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeBound> as c::expression::OrdinaryField<Field_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).val) };
#[doc(hidden)]
pub struct Field_infinite;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeBound, infinite) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_infinite>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeBound>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeBound, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeBound> as c::expression::OrdinaryField<Field_infinite>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).infinite) };
#[doc(hidden)]
pub struct Field_inclusive;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeBound, inclusive) == 9);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inclusive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeBound>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeBound, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeBound> as c::expression::OrdinaryField<Field_inclusive>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inclusive) };
#[doc(hidden)]
pub struct Field_lower;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeBound, lower) == 10);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeBound>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeBound, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeBound> as c::expression::OrdinaryField<Field_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lower) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeType>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeType>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RangeType {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeType, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeType>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RangeType,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeType> as c::expression::OrdinaryField<
        Field_vl_len_,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
#[doc(hidden)]
pub struct Field_rangetypid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeType, rangetypid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rangetypid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeType>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RangeType,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeType> as c::expression::OrdinaryField<
        Field_rangetypid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rangetypid) };
#[doc(hidden)]
pub struct Field_value;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ScalarItem>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ScalarItem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ScalarItem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScalarItem, value) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScalarItem>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScalarItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScalarItem> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
#[doc(hidden)]
pub struct Field_tupno;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScalarItem, tupno) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tupno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScalarItem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScalarItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScalarItem> as c::expression::OrdinaryField<Field_tupno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tupno) };
#[doc(hidden)]
pub struct Field_id;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedInvalCatalogMsg>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedInvalCatalogMsg>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedInvalCatalogMsg {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalCatalogMsg, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatalogMsg>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalCatalogMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatalogMsg> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
#[doc(hidden)]
pub struct Field_dbId;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalCatalogMsg, dbId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatalogMsg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalCatalogMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatalogMsg> as c::expression::OrdinaryField<Field_dbId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbId) };
#[doc(hidden)]
pub struct Field_catId;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalCatalogMsg, catId) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_catId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatalogMsg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalCatalogMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatalogMsg> as c::expression::OrdinaryField<Field_catId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).catId) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedInvalCatcacheMsg>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedInvalCatcacheMsg>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedInvalCatcacheMsg {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalCatcacheMsg, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatcacheMsg>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalCatcacheMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatcacheMsg> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalCatcacheMsg, dbId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatcacheMsg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalCatcacheMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatcacheMsg> as c::expression::OrdinaryField<Field_dbId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbId) };
#[doc(hidden)]
pub struct Field_hashValue;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalCatcacheMsg, hashValue) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashValue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatcacheMsg>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalCatcacheMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalCatcacheMsg> as c::expression::OrdinaryField<Field_hashValue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashValue) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedInvalRelcacheMsg>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedInvalRelcacheMsg>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedInvalRelcacheMsg {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalRelcacheMsg, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelcacheMsg>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalRelcacheMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelcacheMsg> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalRelcacheMsg, dbId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelcacheMsg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalRelcacheMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelcacheMsg> as c::expression::OrdinaryField<Field_dbId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbId) };
#[doc(hidden)]
pub struct Field_relId;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalRelcacheMsg, relId) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelcacheMsg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalRelcacheMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelcacheMsg> as c::expression::OrdinaryField<Field_relId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relId) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedInvalRelmapMsg>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedInvalRelmapMsg>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedInvalRelmapMsg {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalRelmapMsg, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelmapMsg>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalRelmapMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelmapMsg> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalRelmapMsg, dbId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelmapMsg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalRelmapMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalRelmapMsg> as c::expression::OrdinaryField<Field_dbId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbId) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedInvalSmgrMsg>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedInvalSmgrMsg>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedInvalSmgrMsg {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalSmgrMsg, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSmgrMsg>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalSmgrMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSmgrMsg> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
#[doc(hidden)]
pub struct Field_backend_hi;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalSmgrMsg, backend_hi) == 1);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_hi>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSmgrMsg>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalSmgrMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSmgrMsg> as c::expression::OrdinaryField<Field_backend_hi>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_hi) };
#[doc(hidden)]
pub struct Field_backend_lo;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalSmgrMsg, backend_lo) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_lo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSmgrMsg>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalSmgrMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSmgrMsg> as c::expression::OrdinaryField<Field_backend_lo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_lo) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedInvalSnapshotMsg>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedInvalSnapshotMsg>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedInvalSnapshotMsg {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalSnapshotMsg, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSnapshotMsg>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalSnapshotMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSnapshotMsg> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalSnapshotMsg, dbId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSnapshotMsg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalSnapshotMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSnapshotMsg> as c::expression::OrdinaryField<Field_dbId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbId) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalSnapshotMsg, relId) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSnapshotMsg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalSnapshotMsg, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalSnapshotMsg> as c::expression::OrdinaryField<Field_relId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relId) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedInvalidationMessage>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedInvalidationMessage>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedInvalidationMessage {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedInvalidationMessage, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalidationMessage>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedInvalidationMessage, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedInvalidationMessage> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
#[doc(hidden)]
pub struct Field_size;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ShmemIndexEnt>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ShmemIndexEnt>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ShmemIndexEnt {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemIndexEnt, size) == 56);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemIndexEnt>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemIndexEnt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemIndexEnt> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_allocated_size;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemIndexEnt, allocated_size) == 64);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allocated_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemIndexEnt>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemIndexEnt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemIndexEnt> as c::expression::OrdinaryField<Field_allocated_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allocated_size) };
#[doc(hidden)]
pub struct Field_salen;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SockAddr>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SockAddr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SockAddr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SockAddr, salen) == 128);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_salen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SockAddr>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SockAddr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SockAddr> as c::expression::OrdinaryField<
        Field_salen,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).salen) };
#[doc(hidden)]
pub struct Field_spins;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpinDelayStatus>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpinDelayStatus>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpinDelayStatus {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpinDelayStatus, spins) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spins>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpinDelayStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus> as c::expression::OrdinaryField<Field_spins>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spins) };
#[doc(hidden)]
pub struct Field_delays;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpinDelayStatus, delays) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_delays>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpinDelayStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus> as c::expression::OrdinaryField<Field_delays>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).delays) };
#[doc(hidden)]
pub struct Field_cur_delay;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpinDelayStatus, cur_delay) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cur_delay>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpinDelayStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus> as c::expression::OrdinaryField<Field_cur_delay>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cur_delay) };
#[doc(hidden)]
pub struct Field_file;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpinDelayStatus, file) == 16);
const _: () = {
    assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
    assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_file>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadOnly,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpinDelayStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus> as c::expression::OrdinaryField<Field_file>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).file) };
#[doc(hidden)]
pub struct Field_line;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpinDelayStatus, line) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_line>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpinDelayStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus> as c::expression::OrdinaryField<Field_line>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).line) };
#[doc(hidden)]
pub struct Field_func;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpinDelayStatus, func) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_func>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadOnly,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpinDelayStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpinDelayStatus> as c::expression::OrdinaryField<Field_func>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).func) };
#[doc(hidden)]
pub struct Field_eqopr;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StdAnalyzeData>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StdAnalyzeData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::StdAnalyzeData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StdAnalyzeData, eqopr) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eqopr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdAnalyzeData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdAnalyzeData, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdAnalyzeData> as c::expression::OrdinaryField<Field_eqopr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eqopr) };
#[doc(hidden)]
pub struct Field_eqfunc;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StdAnalyzeData, eqfunc) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eqfunc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdAnalyzeData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdAnalyzeData, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdAnalyzeData> as c::expression::OrdinaryField<Field_eqfunc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eqfunc) };
#[doc(hidden)]
pub struct Field_ltopr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StdAnalyzeData, ltopr) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ltopr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdAnalyzeData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdAnalyzeData, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdAnalyzeData> as c::expression::OrdinaryField<Field_ltopr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ltopr) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StopList>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StopList>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::StopList {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StopList, len) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StopList>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::StopList,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::StopList> as c::expression::OrdinaryField<
        Field_len,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).len) };
#[doc(hidden)]
pub struct Field_cfgId;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TSConfigCacheEntry>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TSConfigCacheEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TSConfigCacheEntry {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSConfigCacheEntry, cfgId) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cfgId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSConfigCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSConfigCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSConfigCacheEntry> as c::expression::OrdinaryField<Field_cfgId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cfgId) };
#[doc(hidden)]
pub struct Field_isvalid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSConfigCacheEntry, isvalid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSConfigCacheEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSConfigCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSConfigCacheEntry> as c::expression::OrdinaryField<Field_isvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isvalid) };
#[doc(hidden)]
pub struct Field_prsId;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSConfigCacheEntry, prsId) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prsId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSConfigCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSConfigCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSConfigCacheEntry> as c::expression::OrdinaryField<Field_prsId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prsId) };
#[doc(hidden)]
pub struct Field_lenmap;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSConfigCacheEntry, lenmap) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lenmap>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSConfigCacheEntry>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSConfigCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSConfigCacheEntry> as c::expression::OrdinaryField<Field_lenmap>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lenmap) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TSLexeme>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TSLexeme>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TSLexeme {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSLexeme, nvariant) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nvariant>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSLexeme>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TSLexeme,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TSLexeme> as c::expression::OrdinaryField<
        Field_nvariant,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nvariant) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSLexeme, flags) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSLexeme>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TSLexeme,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TSLexeme> as c::expression::OrdinaryField<
        Field_flags,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TSQueryData>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TSQueryData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TSQueryData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSQueryData, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSQueryData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSQueryData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSQueryData> as c::expression::OrdinaryField<Field_vl_len_>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSQueryData, size) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSQueryData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSQueryData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSQueryData> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSQueryData, data) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSQueryData>
{
    type Member = c::expression::CFlexibleArray<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        crate::__pgrx_c_bindings::__IncompleteArrayField<::core::ffi::c_char>,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSQueryData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSQueryData> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TSVectorData>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TSVectorData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TSVectorData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSVectorData, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSVectorData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSVectorData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSVectorData> as c::expression::OrdinaryField<Field_vl_len_>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSVectorData, size) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSVectorData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSVectorData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSVectorData> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
#[doc(hidden)]
pub struct Field_entries;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSVectorData, entries) == 8);
impl c::expression::OffsetField<Field_entries>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSVectorData>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::TSVectorData, entries);
}
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::__IncompleteArrayField<crate::__pgrx_c_bindings::WordEntry>,
        >() == 0
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::__IncompleteArrayField<crate::__pgrx_c_bindings::WordEntry>,
        >() == 4
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_entries>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSVectorData>
{
    type Member = c::expression::CFlexibleArray<
        c::expression::CRecord<crate::__pgrx_c_bindings::WordEntry>,
        crate::__pgrx_c_bindings::__IncompleteArrayField<crate::__pgrx_c_bindings::WordEntry>,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSVectorData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSVectorData> as c::expression::OrdinaryField<Field_entries>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).entries) };
#[doc(hidden)]
pub struct Field_tli;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TimeLineHistoryEntry>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TimeLineHistoryEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TimeLineHistoryEntry {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TimeLineHistoryEntry, tli) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tli>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TimeLineHistoryEntry>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TimeLineHistoryEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TimeLineHistoryEntry> as c::expression::OrdinaryField<Field_tli>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tli) };
#[doc(hidden)]
pub struct Field_begin;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TimeLineHistoryEntry, begin) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_begin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TimeLineHistoryEntry>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TimeLineHistoryEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TimeLineHistoryEntry> as c::expression::OrdinaryField<Field_begin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).begin) };
#[doc(hidden)]
pub struct Field_end;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TimeLineHistoryEntry, end) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_end>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TimeLineHistoryEntry>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TimeLineHistoryEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TimeLineHistoryEntry> as c::expression::OrdinaryField<Field_end>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).end) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TimeTzADT>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TimeTzADT>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TimeTzADT {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TimeTzADT, time) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TimeTzADT>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TimeTzADT,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TimeTzADT> as c::expression::OrdinaryField<
        Field_time,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).time) };
#[doc(hidden)]
pub struct Field_zone;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TimeTzADT, zone) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_zone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TimeTzADT>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TimeTzADT,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TimeTzADT> as c::expression::OrdinaryField<
        Field_zone,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).zone) };
#[doc(hidden)]
pub struct Field_tai_size;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ToastAttrInfo>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ToastAttrInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ToastAttrInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ToastAttrInfo, tai_size) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tai_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ToastAttrInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ToastAttrInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ToastAttrInfo> as c::expression::OrdinaryField<Field_tai_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tai_size) };
#[doc(hidden)]
pub struct Field_tai_colflags;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ToastAttrInfo, tai_colflags) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tai_colflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ToastAttrInfo>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ToastAttrInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ToastAttrInfo> as c::expression::OrdinaryField<Field_tai_colflags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tai_colflags) };
#[doc(hidden)]
pub struct Field_tai_compression;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ToastAttrInfo, tai_compression) == 13
);
const _: () = {
    assert!(::core::mem::size_of::<::core::ffi::c_char>() == 1);
    assert!(::core::mem::align_of::<::core::ffi::c_char>() == 1);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tai_compression>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ToastAttrInfo>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 13;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ToastAttrInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ToastAttrInfo> as c::expression::OrdinaryField<Field_tai_compression>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tai_compression) };
#[doc(hidden)]
pub struct Field_ttc_flags;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ToastTupleContext>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ToastTupleContext>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ToastTupleContext {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ToastTupleContext, ttc_flags) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ttc_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ToastTupleContext>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ToastTupleContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ToastTupleContext> as c::expression::OrdinaryField<Field_ttc_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ttc_flags) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_backendId;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VirtualTransactionId>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VirtualTransactionId>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::VirtualTransactionId {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VirtualTransactionId, backendId) == 0
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backendId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VirtualTransactionId>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VirtualTransactionId, <<c::expression::CRecord<crate::__pgrx_c_bindings::VirtualTransactionId> as c::expression::OrdinaryField<Field_backendId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backendId) };
#[doc(hidden)]
pub struct Field_localTransactionId;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VirtualTransactionId, localTransactionId)
        == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_localTransactionId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VirtualTransactionId>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VirtualTransactionId, <<c::expression::CRecord<crate::__pgrx_c_bindings::VirtualTransactionId> as c::expression::OrdinaryField<Field_localTransactionId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).localTransactionId) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalRcvData>() == 2264);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalRcvData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WalRcvData {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, pid) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pid) };
#[doc(hidden)]
pub struct Field_walRcvState;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, walRcvState) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walRcvState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_WalRcvState,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_walRcvState>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).walRcvState) };
#[doc(hidden)]
pub struct Field_startTime;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, startTime) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startTime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_startTime>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).startTime) };
#[doc(hidden)]
pub struct Field_receiveStart;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, receiveStart) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_receiveStart>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_receiveStart>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).receiveStart) };
#[doc(hidden)]
pub struct Field_receiveStartTLI;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, receiveStartTLI) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_receiveStartTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_receiveStartTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).receiveStartTLI) };
#[doc(hidden)]
pub struct Field_flushedUpto;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, flushedUpto) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flushedUpto>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_flushedUpto>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flushedUpto) };
#[doc(hidden)]
pub struct Field_receivedTLI;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, receivedTLI) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_receivedTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_receivedTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).receivedTLI) };
#[doc(hidden)]
pub struct Field_latestChunkStart;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, latestChunkStart) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_latestChunkStart>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_latestChunkStart>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).latestChunkStart) };
#[doc(hidden)]
pub struct Field_lastMsgSendTime;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, lastMsgSendTime) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastMsgSendTime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_lastMsgSendTime>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastMsgSendTime) };
#[doc(hidden)]
pub struct Field_lastMsgReceiptTime;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, lastMsgReceiptTime) == 80
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastMsgReceiptTime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_lastMsgReceiptTime>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastMsgReceiptTime) };
#[doc(hidden)]
pub struct Field_latestWalEnd;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, latestWalEnd) == 88);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_latestWalEnd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_latestWalEnd>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).latestWalEnd) };
#[doc(hidden)]
pub struct Field_latestWalEndTime;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, latestWalEndTime) == 96);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_latestWalEndTime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_latestWalEndTime>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).latestWalEndTime) };
#[doc(hidden)]
pub struct Field_sender_port;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, sender_port) == 2156);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sender_port>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2156;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_sender_port>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sender_port) };
#[doc(hidden)]
pub struct Field_is_temp_slot;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, is_temp_slot) == 2224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_temp_slot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_is_temp_slot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_temp_slot) };
#[doc(hidden)]
pub struct Field_ready_to_display;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, ready_to_display) == 2225
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ready_to_display>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2225;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_ready_to_display>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ready_to_display) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, mutex) == 2240);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_mutex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mutex) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_force_reply;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, force_reply) == 2256);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_reply>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2256;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_force_reply>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_reply) };
#[doc(hidden)]
pub struct Field_logical;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalRcvStreamOptions>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalRcvStreamOptions>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WalRcvStreamOptions {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvStreamOptions, logical) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_logical>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvStreamOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvStreamOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvStreamOptions> as c::expression::OrdinaryField<Field_logical>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).logical) };
#[doc(hidden)]
pub struct Field_startpoint;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvStreamOptions, startpoint) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startpoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvStreamOptions>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvStreamOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvStreamOptions> as c::expression::OrdinaryField<Field_startpoint>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).startpoint) };
#[doc(hidden)]
pub struct Field_sync_standbys_status;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalSndCtlData>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalSndCtlData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WalSndCtlData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalSndCtlData, sync_standbys_status) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_standbys_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalSndCtlData>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalSndCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalSndCtlData> as c::expression::OrdinaryField<Field_sync_standbys_status>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_standbys_status) };
#[doc(hidden)]
pub struct Field_numWindowFuncs;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowFuncLists>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowFuncLists>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WindowFuncLists {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFuncLists, numWindowFuncs) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numWindowFuncs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncLists>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFuncLists, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncLists> as c::expression::OrdinaryField<Field_numWindowFuncs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numWindowFuncs) };
#[doc(hidden)]
pub struct Field_maxWinRef;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFuncLists, maxWinRef) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxWinRef>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncLists>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFuncLists, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFuncLists> as c::expression::OrdinaryField<Field_maxWinRef>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxWinRef) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WordEntry>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WordEntry>() == 4);
};
#[doc(hidden)]
pub struct Field_haspos;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WordEntry {}
const _: () = {
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WordEntry, _bitfield_1) * 8 + 0 == 0);
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__WordEntry__haspos(
        p: *const crate::__pgrx_c_bindings::WordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set__WordEntry__haspos(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__WordEntry__haspos(
        p: *const crate::__pgrx_c_bindings::WordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__WordEntry__haspos(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__WordEntry__haspos(
        p: *const crate::__pgrx_c_bindings::WordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__WordEntry__haspos(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__WordEntry__haspos(
        p: *const crate::__pgrx_c_bindings::WordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__WordEntry__haspos(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_WordEntry__haspos<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::WordEntry>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_WordEntry__haspos<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_WordEntry__haspos<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_haspos, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WordEntry>
{
    type Output = Bitfield_WordEntry__haspos<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_WordEntry__haspos::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace for Bitfield_WordEntry__haspos<Q> {
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_WordEntry__haspos<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__WordEntry__haspos(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__WordEntry__haspos(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__WordEntry__haspos(pointer)
            } else {
                __pgrx_bitfield__get__WordEntry__haspos(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace for Bitfield_WordEntry__haspos<c::expression::ReadWrite> {
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__WordEntry__haspos(self.address, value)
                } else {
                    __pgrx_bitfield__set_unaligned__WordEntry__haspos(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__WordEntry__haspos(self.address, value)
            } else {
                __pgrx_bitfield__set__WordEntry__haspos(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
const _: () = {
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WordEntry, _bitfield_1) * 8 + 1 == 1);
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__WordEntry__len(p: *const crate::__pgrx_c_bindings::WordEntry) -> u32;
    fn __pgrx_bitfield__set__WordEntry__len(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__WordEntry__len(
        p: *const crate::__pgrx_c_bindings::WordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__WordEntry__len(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__WordEntry__len(
        p: *const crate::__pgrx_c_bindings::WordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__WordEntry__len(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__WordEntry__len(
        p: *const crate::__pgrx_c_bindings::WordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__WordEntry__len(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_WordEntry__len<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::WordEntry>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_WordEntry__len<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_WordEntry__len<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_len, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WordEntry>
{
    type Output = Bitfield_WordEntry__len<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_WordEntry__len::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace for Bitfield_WordEntry__len<Q> {
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace for Bitfield_WordEntry__len<Q> {
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__WordEntry__len(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__WordEntry__len(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__WordEntry__len(pointer)
            } else {
                __pgrx_bitfield__get__WordEntry__len(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace for Bitfield_WordEntry__len<c::expression::ReadWrite> {
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__WordEntry__len(self.address, value)
                } else {
                    __pgrx_bitfield__set_unaligned__WordEntry__len(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__WordEntry__len(self.address, value)
            } else {
                __pgrx_bitfield__set__WordEntry__len(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::WordEntry, _bitfield_1) * 8 + 12 == 12
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__WordEntry__pos(p: *const crate::__pgrx_c_bindings::WordEntry) -> u32;
    fn __pgrx_bitfield__set__WordEntry__pos(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__WordEntry__pos(
        p: *const crate::__pgrx_c_bindings::WordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__WordEntry__pos(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__WordEntry__pos(
        p: *const crate::__pgrx_c_bindings::WordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__WordEntry__pos(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__WordEntry__pos(
        p: *const crate::__pgrx_c_bindings::WordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__WordEntry__pos(
        p: *mut crate::__pgrx_c_bindings::WordEntry,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_WordEntry__pos<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::WordEntry>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_WordEntry__pos<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_WordEntry__pos<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_pos, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WordEntry>
{
    type Output = Bitfield_WordEntry__pos<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_WordEntry__pos::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace for Bitfield_WordEntry__pos<Q> {
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace for Bitfield_WordEntry__pos<Q> {
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__WordEntry__pos(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__WordEntry__pos(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__WordEntry__pos(pointer)
            } else {
                __pgrx_bitfield__get__WordEntry__pos(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace for Bitfield_WordEntry__pos<c::expression::ReadWrite> {
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__WordEntry__pos(self.address, value)
                } else {
                    __pgrx_bitfield__set_unaligned__WordEntry__pos(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__WordEntry__pos(self.address, value)
            } else {
                __pgrx_bitfield__set__WordEntry__pos(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_npos;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WordEntryPosVector>() == 2);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WordEntryPosVector>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WordEntryPosVector {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WordEntryPosVector, npos) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_npos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WordEntryPosVector>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WordEntryPosVector, <<c::expression::CRecord<crate::__pgrx_c_bindings::WordEntryPosVector> as c::expression::OrdinaryField<Field_npos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).npos) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WordEntryPosVector, pos) == 2);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::__IncompleteArrayField<u16>>() == 0);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::__IncompleteArrayField<u16>>() == 2);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WordEntryPosVector>
{
    type Member = c::expression::CFlexibleArray<
        c::CUnsignedShort,
        crate::__pgrx_c_bindings::__IncompleteArrayField<u16>,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WordEntryPosVector, <<c::expression::CRecord<crate::__pgrx_c_bindings::WordEntryPosVector> as c::expression::OrdinaryField<Field_pos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pos) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WordEntryPosVector1>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WordEntryPosVector1>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WordEntryPosVector1 {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WordEntryPosVector1, npos) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_npos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WordEntryPosVector1>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WordEntryPosVector1, <<c::expression::CRecord<crate::__pgrx_c_bindings::WordEntryPosVector1> as c::expression::OrdinaryField<Field_npos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).npos) };
#[doc(hidden)]
pub struct Field_processed;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::_SPI_connection>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::_SPI_connection>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::_SPI_connection {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_connection, processed) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_SPI_connection, <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection> as c::expression::OrdinaryField<Field_processed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).processed) };
#[doc(hidden)]
pub struct Field_execSubid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_connection, execSubid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_execSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_SPI_connection, <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection> as c::expression::OrdinaryField<Field_execSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).execSubid) };
#[doc(hidden)]
pub struct Field_connectSubid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_connection, connectSubid) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_connectSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_SPI_connection, <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection> as c::expression::OrdinaryField<Field_connectSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).connectSubid) };
#[doc(hidden)]
pub struct Field_atomic;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_connection, atomic) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atomic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_SPI_connection, <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection> as c::expression::OrdinaryField<Field_atomic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atomic) };
#[doc(hidden)]
pub struct Field_internal_xact;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_connection, internal_xact) == 73
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_internal_xact>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 73;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_SPI_connection, <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection> as c::expression::OrdinaryField<Field_internal_xact>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).internal_xact) };
#[doc(hidden)]
pub struct Field_outer_processed;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_connection, outer_processed) == 80
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outer_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_SPI_connection, <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection> as c::expression::OrdinaryField<Field_outer_processed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outer_processed) };
#[doc(hidden)]
pub struct Field_outer_result;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::_SPI_connection, outer_result) == 96);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outer_result>
    for c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::_SPI_connection, <<c::expression::CRecord<crate::__pgrx_c_bindings::_SPI_connection> as c::expression::OrdinaryField<Field_outer_result>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outer_result) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::oidvector>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::oidvector>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::oidvector {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::oidvector, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::oidvector>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::oidvector,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::oidvector> as c::expression::OrdinaryField<
        Field_vl_len_,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
#[doc(hidden)]
pub struct Field_ndim;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::oidvector, ndim) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndim>
    for c::expression::CRecord<crate::__pgrx_c_bindings::oidvector>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::oidvector,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::oidvector> as c::expression::OrdinaryField<
        Field_ndim,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndim) };
#[doc(hidden)]
pub struct Field_dataoffset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::oidvector, dataoffset) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dataoffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::oidvector>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::oidvector,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::oidvector> as c::expression::OrdinaryField<
        Field_dataoffset,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dataoffset) };
#[doc(hidden)]
pub struct Field_elemtype;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::oidvector, elemtype) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elemtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::oidvector>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::oidvector,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::oidvector> as c::expression::OrdinaryField<
        Field_elemtype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elemtype) };
#[doc(hidden)]
pub struct Field_dim1;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::oidvector, dim1) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dim1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::oidvector>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::oidvector,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::oidvector> as c::expression::OrdinaryField<
        Field_dim1,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dim1) };
#[doc(hidden)]
pub struct Field_lbound1;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::oidvector, lbound1) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lbound1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::oidvector>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::oidvector,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::oidvector> as c::expression::OrdinaryField<
        Field_lbound1,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lbound1) };
#[doc(hidden)]
pub struct Field_code;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_local_to_utf_combined>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_local_to_utf_combined>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::pg_local_to_utf_combined {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_local_to_utf_combined, code) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_code>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_local_to_utf_combined>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_local_to_utf_combined, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_local_to_utf_combined> as c::expression::OrdinaryField<Field_code>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).code) };
#[doc(hidden)]
pub struct Field_utf1;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_local_to_utf_combined, utf1) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_utf1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_local_to_utf_combined>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_local_to_utf_combined, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_local_to_utf_combined> as c::expression::OrdinaryField<Field_utf1>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).utf1) };
#[doc(hidden)]
pub struct Field_utf2;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_local_to_utf_combined, utf2) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_utf2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_local_to_utf_combined>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_local_to_utf_combined, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_local_to_utf_combined> as c::expression::OrdinaryField<Field_utf2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).utf2) };
#[doc(hidden)]
pub struct Field_b1root;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_mb_radix_tree>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_mb_radix_tree>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::pg_mb_radix_tree {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b1root) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b1root>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b1root>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b1root) };
#[doc(hidden)]
pub struct Field_b1_lower;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b1_lower) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b1_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b1_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b1_lower) };
#[doc(hidden)]
pub struct Field_b1_upper;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b1_upper) == 21);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b1_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b1_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b1_upper) };
#[doc(hidden)]
pub struct Field_b2root;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b2root) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b2root>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b2root>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b2root) };
#[doc(hidden)]
pub struct Field_b2_1_lower;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b2_1_lower) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b2_1_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b2_1_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b2_1_lower) };
#[doc(hidden)]
pub struct Field_b2_1_upper;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b2_1_upper) == 29);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b2_1_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 29;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b2_1_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b2_1_upper) };
#[doc(hidden)]
pub struct Field_b2_2_lower;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b2_2_lower) == 30);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b2_2_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b2_2_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b2_2_lower) };
#[doc(hidden)]
pub struct Field_b2_2_upper;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b2_2_upper) == 31);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b2_2_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 31;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b2_2_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b2_2_upper) };
#[doc(hidden)]
pub struct Field_b3root;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b3root) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b3root>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b3root>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b3root) };
#[doc(hidden)]
pub struct Field_b3_1_lower;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b3_1_lower) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b3_1_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b3_1_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b3_1_lower) };
#[doc(hidden)]
pub struct Field_b3_1_upper;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b3_1_upper) == 37);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b3_1_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 37;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b3_1_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b3_1_upper) };
#[doc(hidden)]
pub struct Field_b3_2_lower;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b3_2_lower) == 38);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b3_2_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 38;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b3_2_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b3_2_lower) };
#[doc(hidden)]
pub struct Field_b3_2_upper;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b3_2_upper) == 39);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b3_2_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 39;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b3_2_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b3_2_upper) };
#[doc(hidden)]
pub struct Field_b3_3_lower;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b3_3_lower) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b3_3_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b3_3_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b3_3_lower) };
#[doc(hidden)]
pub struct Field_b3_3_upper;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b3_3_upper) == 41);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b3_3_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b3_3_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b3_3_upper) };
#[doc(hidden)]
pub struct Field_b4root;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b4root) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b4root>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b4root>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b4root) };
#[doc(hidden)]
pub struct Field_b4_1_lower;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b4_1_lower) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b4_1_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b4_1_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b4_1_lower) };
#[doc(hidden)]
pub struct Field_b4_1_upper;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b4_1_upper) == 49);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b4_1_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 49;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b4_1_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b4_1_upper) };
#[doc(hidden)]
pub struct Field_b4_2_lower;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b4_2_lower) == 50);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b4_2_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 50;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b4_2_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b4_2_lower) };
#[doc(hidden)]
pub struct Field_b4_2_upper;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b4_2_upper) == 51);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b4_2_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 51;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b4_2_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b4_2_upper) };
#[doc(hidden)]
pub struct Field_b4_3_lower;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b4_3_lower) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b4_3_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b4_3_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b4_3_lower) };
#[doc(hidden)]
pub struct Field_b4_3_upper;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b4_3_upper) == 53);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b4_3_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 53;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b4_3_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b4_3_upper) };
#[doc(hidden)]
pub struct Field_b4_4_lower;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b4_4_lower) == 54);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b4_4_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 54;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b4_4_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b4_4_lower) };
#[doc(hidden)]
pub struct Field_b4_4_upper;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_mb_radix_tree, b4_4_upper) == 55);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_b4_4_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 55;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_mb_radix_tree, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_mb_radix_tree> as c::expression::OrdinaryField<Field_b4_4_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).b4_4_upper) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_utf_to_local_combined>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_utf_to_local_combined>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::pg_utf_to_local_combined {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_utf_to_local_combined, utf1) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_utf1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_utf_to_local_combined>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_utf_to_local_combined, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_utf_to_local_combined> as c::expression::OrdinaryField<Field_utf1>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).utf1) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_utf_to_local_combined, utf2) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_utf2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_utf_to_local_combined>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_utf_to_local_combined, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_utf_to_local_combined> as c::expression::OrdinaryField<Field_utf2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).utf2) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_utf_to_local_combined, code) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_code>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_utf_to_local_combined>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_utf_to_local_combined, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_utf_to_local_combined> as c::expression::OrdinaryField<Field_code>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).code) };
#[doc(hidden)]
pub struct Field_maxmblen;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_wchar_tbl>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_wchar_tbl>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::pg_wchar_tbl {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_wchar_tbl, maxmblen) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxmblen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_wchar_tbl>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_wchar_tbl, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_wchar_tbl> as c::expression::OrdinaryField<Field_maxmblen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxmblen) };
#[doc(hidden)]
pub struct Field_opttype;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::relopt_parse_elt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::relopt_parse_elt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::relopt_parse_elt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::relopt_parse_elt, opttype) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::relopt_parse_elt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__relopt_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::relopt_parse_elt, <<c::expression::CRecord<crate::__pgrx_c_bindings::relopt_parse_elt> as c::expression::OrdinaryField<Field_opttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opttype) };
#[doc(hidden)]
pub struct Field_offset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::relopt_parse_elt, offset) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::relopt_parse_elt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::relopt_parse_elt, <<c::expression::CRecord<crate::__pgrx_c_bindings::relopt_parse_elt> as c::expression::OrdinaryField<Field_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).offset) };
#[doc(hidden)]
pub struct Field_ai_grantee;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AclItem>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AclItem>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AclItem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AclItem, ai_grantee) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ai_grantee>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AclItem>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AclItem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AclItem> as c::expression::OrdinaryField<
        Field_ai_grantee,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ai_grantee) };
#[doc(hidden)]
pub struct Field_ai_grantor;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AclItem, ai_grantor) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ai_grantor>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AclItem>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AclItem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AclItem> as c::expression::OrdinaryField<
        Field_ai_grantor,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ai_grantor) };
#[doc(hidden)]
pub struct Field_ai_privs;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AclItem, ai_privs) == 8);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ai_privs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AclItem>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AclItem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AclItem> as c::expression::OrdinaryField<
        Field_ai_privs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ai_privs) };
#[doc(hidden)]
pub struct Field_transitionSpace;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggClauseCosts>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggClauseCosts>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggClauseCosts {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggClauseCosts, transitionSpace) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transitionSpace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggClauseCosts>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggClauseCosts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggClauseCosts> as c::expression::OrdinaryField<Field_transitionSpace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transitionSpace) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ArrayType>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ArrayType>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ArrayType {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayType, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayType>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayType,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayType> as c::expression::OrdinaryField<
        Field_vl_len_,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayType, ndim) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndim>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayType>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayType,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayType> as c::expression::OrdinaryField<
        Field_ndim,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndim) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayType, dataoffset) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dataoffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayType>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayType,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayType> as c::expression::OrdinaryField<
        Field_dataoffset,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dataoffset) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayType, elemtype) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elemtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayType>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayType,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayType> as c::expression::OrdinaryField<
        Field_elemtype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elemtype) };
#[doc(hidden)]
pub struct Field_maplen;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AttrMap>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AttrMap>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AttrMap {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AttrMap, maplen) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maplen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AttrMap>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AttrMap,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AttrMap> as c::expression::OrdinaryField<
        Field_maplen,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maplen) };
#[doc(hidden)]
pub struct Field_nwords;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Bitmapset>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Bitmapset>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Bitmapset {}
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Bitmapset, nwords) == 0);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nwords>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Bitmapset,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset> as c::expression::OrdinaryField<
        Field_nwords,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nwords) };
#[doc(hidden)]
pub struct Field_bt_blkno;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BrinTuple>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BrinTuple>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BrinTuple {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinTuple, bt_blkno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bt_blkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinTuple>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::BrinTuple,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinTuple> as c::expression::OrdinaryField<
        Field_bt_blkno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bt_blkno) };
#[doc(hidden)]
pub struct Field_bt_info;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BrinTuple, bt_info) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bt_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BrinTuple>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::BrinTuple,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BrinTuple> as c::expression::OrdinaryField<
        Field_bt_info,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bt_info) };
#[doc(hidden)]
pub struct Field_shared_blks_hit;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferUsage>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferUsage>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BufferUsage {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferUsage, shared_blks_hit) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shared_blks_hit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferUsage, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage> as c::expression::OrdinaryField<Field_shared_blks_hit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).shared_blks_hit) };
#[doc(hidden)]
pub struct Field_shared_blks_read;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferUsage, shared_blks_read) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shared_blks_read>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferUsage, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage> as c::expression::OrdinaryField<Field_shared_blks_read>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).shared_blks_read) };
#[doc(hidden)]
pub struct Field_shared_blks_dirtied;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferUsage, shared_blks_dirtied) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shared_blks_dirtied>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferUsage, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage> as c::expression::OrdinaryField<Field_shared_blks_dirtied>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).shared_blks_dirtied) };
#[doc(hidden)]
pub struct Field_shared_blks_written;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferUsage, shared_blks_written) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shared_blks_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferUsage, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage> as c::expression::OrdinaryField<Field_shared_blks_written>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).shared_blks_written) };
#[doc(hidden)]
pub struct Field_local_blks_hit;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferUsage, local_blks_hit) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_local_blks_hit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferUsage, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage> as c::expression::OrdinaryField<Field_local_blks_hit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).local_blks_hit) };
#[doc(hidden)]
pub struct Field_local_blks_read;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferUsage, local_blks_read) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_local_blks_read>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferUsage, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage> as c::expression::OrdinaryField<Field_local_blks_read>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).local_blks_read) };
#[doc(hidden)]
pub struct Field_local_blks_dirtied;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferUsage, local_blks_dirtied) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_local_blks_dirtied>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferUsage, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage> as c::expression::OrdinaryField<Field_local_blks_dirtied>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).local_blks_dirtied) };
#[doc(hidden)]
pub struct Field_local_blks_written;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferUsage, local_blks_written) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_local_blks_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferUsage, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage> as c::expression::OrdinaryField<Field_local_blks_written>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).local_blks_written) };
#[doc(hidden)]
pub struct Field_temp_blks_read;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferUsage, temp_blks_read) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_temp_blks_read>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferUsage, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage> as c::expression::OrdinaryField<Field_temp_blks_read>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).temp_blks_read) };
#[doc(hidden)]
pub struct Field_temp_blks_written;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferUsage, temp_blks_written) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_temp_blks_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferUsage, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferUsage> as c::expression::OrdinaryField<Field_temp_blks_written>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).temp_blks_written) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyStmt>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CopyStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyStmt, type_) == 0);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NodeTag>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NodeTag>() == 4);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::CopyStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_attlist;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyStmt, attlist) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::CopyStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyStmt> as c::expression::OrdinaryField<
        Field_attlist,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attlist) };
#[doc(hidden)]
pub struct Field_is_from;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyStmt, is_from) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_from>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::CopyStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyStmt> as c::expression::OrdinaryField<
        Field_is_from,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_from) };
#[doc(hidden)]
pub struct Field_is_program;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyStmt, is_program) == 33);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_program>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::CopyStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyStmt> as c::expression::OrdinaryField<
        Field_is_program,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_program) };
#[doc(hidden)]
pub struct Field_options;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyStmt, options) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::CopyStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyStmt> as c::expression::OrdinaryField<
        Field_options,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreatedbStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreatedbStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreatedbStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreatedbStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreatedbStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreatedbStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreatedbStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreatedbStmt, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreatedbStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreatedbStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreatedbStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_opcode;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExprEvalStep {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExprEvalStep, opcode) == 0);
const _: () = {
    assert!(::core::mem::size_of::<isize>() == 8);
    assert!(::core::mem::align_of::<isize>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opcode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep>
{
    type Member = c::expression::CIntegerStorage<c::CLong, isize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep> as c::expression::OrdinaryField<Field_opcode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opcode) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExtensibleNode>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExtensibleNode>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExtensibleNode {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExtensibleNode, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExtensibleNode>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExtensibleNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExtensibleNode> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_node_size;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExtensibleNodeMethods>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExtensibleNodeMethods>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExtensibleNodeMethods {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ExtensibleNodeMethods, node_size) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_node_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExtensibleNodeMethods>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExtensibleNodeMethods, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExtensibleNodeMethods> as c::expression::OrdinaryField<Field_node_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).node_size) };
#[doc(hidden)]
pub struct Field_handler;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FileTag>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FileTag>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::FileTag {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FileTag, handler) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_handler>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FileTag>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::FileTag,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::FileTag> as c::expression::OrdinaryField<
        Field_handler,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).handler) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FileTag, forknum) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_forknum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FileTag>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::FileTag,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::FileTag> as c::expression::OrdinaryField<
        Field_forknum,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).forknum) };
#[doc(hidden)]
pub struct Field_segno;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FileTag, segno) == 16);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_segno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FileTag>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::FileTag,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::FileTag> as c::expression::OrdinaryField<
        Field_segno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).segno) };
#[doc(hidden)]
pub struct Field_attrelid;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_attribute>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_attribute>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::FormData_pg_attribute {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attrelid) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attrelid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attrelid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attrelid) };
#[doc(hidden)]
pub struct Field_atttypid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, atttypid) == 68
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atttypid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_atttypid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atttypid) };
#[doc(hidden)]
pub struct Field_attstattarget;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attstattarget) == 72
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attstattarget>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attstattarget>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attstattarget) };
#[doc(hidden)]
pub struct Field_attlen;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attlen) == 76);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attlen) };
#[doc(hidden)]
pub struct Field_attnum;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attnum) == 78);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 78;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attnum) };
#[doc(hidden)]
pub struct Field_attndims;
#[cfg(any(feature = "pg15", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attndims) == 80
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attndims>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attndims>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attndims) };
#[doc(hidden)]
pub struct Field_attcacheoff;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attcacheoff) == 84
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcacheoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attcacheoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attcacheoff) };
#[doc(hidden)]
pub struct Field_atttypmod;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, atttypmod) == 88
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atttypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_atttypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atttypmod) };
#[doc(hidden)]
pub struct Field_attbyval;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attbyval) == 92
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attbyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attbyval) };
#[doc(hidden)]
pub struct Field_attalign;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attalign) == 93
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attalign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 93;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attalign>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attalign) };
#[doc(hidden)]
pub struct Field_attstorage;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attstorage) == 94
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attstorage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 94;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attstorage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attstorage) };
#[doc(hidden)]
pub struct Field_attcompression;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attcompression) == 95
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcompression>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 95;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attcompression>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attcompression) };
#[doc(hidden)]
pub struct Field_attnotnull;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attnotnull) == 96
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnotnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attnotnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attnotnull) };
#[doc(hidden)]
pub struct Field_atthasdef;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, atthasdef) == 97
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atthasdef>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 97;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_atthasdef>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atthasdef) };
#[doc(hidden)]
pub struct Field_atthasmissing;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, atthasmissing) == 98
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atthasmissing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 98;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_atthasmissing>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atthasmissing) };
#[doc(hidden)]
pub struct Field_attidentity;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attidentity) == 99
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attidentity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 99;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attidentity>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attidentity) };
#[doc(hidden)]
pub struct Field_attgenerated;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attgenerated) == 100
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attgenerated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attgenerated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attgenerated) };
#[doc(hidden)]
pub struct Field_attisdropped;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attisdropped) == 101
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attisdropped>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 101;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attisdropped>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attisdropped) };
#[doc(hidden)]
pub struct Field_attislocal;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attislocal) == 102
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attislocal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 102;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attislocal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attislocal) };
#[doc(hidden)]
pub struct Field_attinhcount;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attinhcount) == 104
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attinhcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attinhcount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attinhcount) };
#[doc(hidden)]
pub struct Field_attcollation;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attcollation) == 108
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_attribute, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute> as c::expression::OrdinaryField<Field_attcollation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attcollation) };
#[doc(hidden)]
pub struct Field_nPendingPages;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GinStatsData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GinStatsData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GinStatsData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinStatsData, nPendingPages) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nPendingPages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GinStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData> as c::expression::OrdinaryField<Field_nPendingPages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nPendingPages) };
#[doc(hidden)]
pub struct Field_nTotalPages;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinStatsData, nTotalPages) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nTotalPages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GinStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData> as c::expression::OrdinaryField<Field_nTotalPages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nTotalPages) };
#[doc(hidden)]
pub struct Field_nEntryPages;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinStatsData, nEntryPages) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nEntryPages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GinStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData> as c::expression::OrdinaryField<Field_nEntryPages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nEntryPages) };
#[doc(hidden)]
pub struct Field_nDataPages;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinStatsData, nDataPages) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nDataPages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GinStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData> as c::expression::OrdinaryField<Field_nDataPages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nDataPages) };
#[doc(hidden)]
pub struct Field_nEntries;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinStatsData, nEntries) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nEntries>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GinStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData> as c::expression::OrdinaryField<Field_nEntries>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nEntries) };
#[doc(hidden)]
pub struct Field_ginVersion;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinStatsData, ginVersion) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ginVersion>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GinStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GinStatsData> as c::expression::OrdinaryField<Field_ginVersion>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ginVersion) };
#[doc(hidden)]
pub struct Field_num_partitions;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HASHCTL>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HASHCTL>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HASHCTL {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASHCTL, num_partitions) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_partitions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::HASHCTL,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL> as c::expression::OrdinaryField<
        Field_num_partitions,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_partitions) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_ssize;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASHCTL, ssize) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ssize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::HASHCTL,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL> as c::expression::OrdinaryField<
        Field_ssize,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ssize) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_dsize;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASHCTL, dsize) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::HASHCTL,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL> as c::expression::OrdinaryField<
        Field_dsize,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dsize) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_max_dsize;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASHCTL, max_dsize) == 24);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_dsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::HASHCTL,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL> as c::expression::OrdinaryField<
        Field_max_dsize,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_dsize) };
#[doc(hidden)]
pub struct Field_keysize;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASHCTL, keysize) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_keysize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::HASHCTL,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL> as c::expression::OrdinaryField<
        Field_keysize,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).keysize) };
#[doc(hidden)]
pub struct Field_entrysize;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASHCTL, entrysize) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_entrysize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::HASHCTL,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL> as c::expression::OrdinaryField<
        Field_entrysize,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).entrysize) };
#[doc(hidden)]
pub struct Field_header;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbContainer>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbContainer>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonbContainer {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbContainer, header) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_header>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbContainer>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbContainer, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbContainer> as c::expression::OrdinaryField<Field_header>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).header) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbValue>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbValue>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonbValue {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbValue, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__jbvType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbValue, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_tag;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LOCK>() == 152);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LOCK>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LOCK {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCK, tag) == 0);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LOCKTAG>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LOCKTAG>() == 4);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCK>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCK> as c::expression::OrdinaryField<
        Field_tag,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tag) };
#[doc(hidden)]
pub struct Field_grantMask;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCK, grantMask) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grantMask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCK>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCK> as c::expression::OrdinaryField<
        Field_grantMask,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).grantMask) };
#[doc(hidden)]
pub struct Field_waitMask;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCK, waitMask) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitMask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCK>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCK> as c::expression::OrdinaryField<
        Field_waitMask,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waitMask) };
#[doc(hidden)]
pub struct Field_nRequested;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCK, nRequested) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nRequested>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCK>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCK> as c::expression::OrdinaryField<
        Field_nRequested,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nRequested) };
#[doc(hidden)]
pub struct Field_nGranted;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCK, nGranted) == 148);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nGranted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCK>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 148;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCK> as c::expression::OrdinaryField<
        Field_nGranted,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nGranted) };
#[doc(hidden)]
pub struct Field_locktag_field1;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LOCKTAG {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCKTAG, locktag_field1) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_locktag_field1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCKTAG,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG> as c::expression::OrdinaryField<
        Field_locktag_field1,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).locktag_field1) };
#[doc(hidden)]
pub struct Field_locktag_field2;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCKTAG, locktag_field2) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_locktag_field2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCKTAG,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG> as c::expression::OrdinaryField<
        Field_locktag_field2,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).locktag_field2) };
#[doc(hidden)]
pub struct Field_locktag_field3;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCKTAG, locktag_field3) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_locktag_field3>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCKTAG,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG> as c::expression::OrdinaryField<
        Field_locktag_field3,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).locktag_field3) };
#[doc(hidden)]
pub struct Field_locktag_field4;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCKTAG, locktag_field4) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_locktag_field4>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCKTAG,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG> as c::expression::OrdinaryField<
        Field_locktag_field4,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).locktag_field4) };
#[doc(hidden)]
pub struct Field_locktag_type;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCKTAG, locktag_type) == 14);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_locktag_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 14;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCKTAG,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG> as c::expression::OrdinaryField<
        Field_locktag_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).locktag_type) };
#[doc(hidden)]
pub struct Field_locktag_lockmethodid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCKTAG, locktag_lockmethodid) == 15);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_locktag_lockmethodid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 15;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCKTAG,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG> as c::expression::OrdinaryField<
        Field_locktag_lockmethodid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).locktag_lockmethodid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::List>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::List>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::List {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::List, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::List>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::List,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::List> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::List, length) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::List>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::List,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::List> as c::expression::OrdinaryField<
        Field_length,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).length) };
#[doc(hidden)]
pub struct Field_max_length;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::List, max_length) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::List>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::List,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::List> as c::expression::OrdinaryField<
        Field_max_length,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_length) };
#[doc(hidden)]
pub struct Field_elements;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::List, elements) == 16);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ListCell>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ListCell>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elements>
    for c::expression::CRecord<crate::__pgrx_c_bindings::List>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ListCell>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::List,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::List> as c::expression::OrdinaryField<
        Field_elements,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elements) };
#[doc(hidden)]
pub struct Field_numLockModes;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockMethodData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockMethodData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LockMethodData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockMethodData, numLockModes) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numLockModes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockMethodData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockMethodData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockMethodData> as c::expression::OrdinaryField<Field_numLockModes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numLockModes) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Node>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Node>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Node {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Node, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Node>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Node,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Node> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_classId;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ObjectAddress>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ObjectAddress>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ObjectAddress {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectAddress, classId) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_classId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAddress>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectAddress, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAddress> as c::expression::OrdinaryField<Field_classId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).classId) };
#[doc(hidden)]
pub struct Field_objectId;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectAddress, objectId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objectId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAddress>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectAddress, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAddress> as c::expression::OrdinaryField<Field_objectId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objectId) };
#[doc(hidden)]
pub struct Field_objectSubId;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectAddress, objectSubId) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objectSubId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAddress>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectAddress, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAddress> as c::expression::OrdinaryField<Field_objectSubId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objectSubId) };
#[doc(hidden)]
pub struct Field_min_input_size;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGLZ_Strategy>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGLZ_Strategy>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PGLZ_Strategy {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGLZ_Strategy, min_input_size) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_min_input_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGLZ_Strategy, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy> as c::expression::OrdinaryField<Field_min_input_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).min_input_size) };
#[doc(hidden)]
pub struct Field_max_input_size;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGLZ_Strategy, max_input_size) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_input_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGLZ_Strategy, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy> as c::expression::OrdinaryField<Field_max_input_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_input_size) };
#[doc(hidden)]
pub struct Field_min_comp_rate;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGLZ_Strategy, min_comp_rate) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_min_comp_rate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGLZ_Strategy, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy> as c::expression::OrdinaryField<Field_min_comp_rate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).min_comp_rate) };
#[doc(hidden)]
pub struct Field_first_success_by;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGLZ_Strategy, first_success_by) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_first_success_by>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGLZ_Strategy, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy> as c::expression::OrdinaryField<Field_first_success_by>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).first_success_by) };
#[doc(hidden)]
pub struct Field_match_size_good;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGLZ_Strategy, match_size_good) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_match_size_good>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGLZ_Strategy, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy> as c::expression::OrdinaryField<Field_match_size_good>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).match_size_good) };
#[doc(hidden)]
pub struct Field_match_size_drop;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGLZ_Strategy, match_size_drop) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_match_size_drop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGLZ_Strategy, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGLZ_Strategy> as c::expression::OrdinaryField<Field_match_size_drop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).match_size_drop) };
#[doc(hidden)]
pub struct Field_spill_txns;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry {}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, spill_txns) == 64
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spill_txns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_spill_txns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spill_txns) };
#[doc(hidden)]
pub struct Field_spill_count;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, spill_count) == 72
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spill_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_spill_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spill_count) };
#[doc(hidden)]
pub struct Field_spill_bytes;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, spill_bytes) == 80
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spill_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_spill_bytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spill_bytes) };
#[doc(hidden)]
pub struct Field_stream_txns;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, stream_txns) == 88
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stream_txns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_stream_txns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stream_txns) };
#[doc(hidden)]
pub struct Field_stream_count;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, stream_count) == 96
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stream_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_stream_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stream_count) };
#[doc(hidden)]
pub struct Field_stream_bytes;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, stream_bytes)
        == 104
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stream_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_stream_bytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stream_bytes) };
#[doc(hidden)]
pub struct Field_total_txns;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, total_txns) == 112
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_txns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_total_txns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_txns) };
#[doc(hidden)]
pub struct Field_total_bytes;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, total_bytes) == 120
);
