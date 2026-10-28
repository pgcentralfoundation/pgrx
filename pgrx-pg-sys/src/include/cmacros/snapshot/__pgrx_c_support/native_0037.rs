// Shared generated C macro support.

#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_min_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_max_age) == 44
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_max_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_table_age)
        == 48
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_table_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, log_vacuum_min_duration) == 52
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_vacuum_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_log_vacuum_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_vacuum_min_duration) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, log_analyze_min_duration) == 56
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_analyze_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_log_analyze_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_analyze_min_duration) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation>() == 72
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation>() == 8
    );
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 344);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, initialized) == 312
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, recheck) == 336);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recheck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferHeapTupleTableSlot, buffer) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferManagerRelation, rel) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferManagerRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation> as c::expression::OrdinaryField<Field_rel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rel) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_smgr;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferManagerRelation, smgr) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_smgr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferManagerRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation> as c::expression::OrdinaryField<Field_smgr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).smgr) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CheckPoint>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CheckPoint>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_logicalDecodingEnabled;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, logicalDecodingEnabled) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_logicalDecodingEnabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_logicalDecodingEnabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).logicalDecodingEnabled) };
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextOid) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextMulti) == 44);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextMulti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextMultiOffset) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextMultiOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestXid) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestXidDB) == 60);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXidDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestMulti) == 64);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestMulti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestMultiDB) == 68);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestMultiDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, time) == 72);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestCommitTsXid) == 80);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, newestCommitTsXid) == 84);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestActiveXid) == 88);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestActiveXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CheckPointStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CheckPointStmt>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPointStmt, options) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPointStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPointStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPointStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CompactAttribute>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CompactAttribute>() == 2);
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcacheoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attlen) == 2);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attbyval) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attalignby) == 5);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attalignby>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 0
            == 48
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__attispackable<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__attispackable<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__attispackable<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_attispackable, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__attispackable<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__attispackable::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__attispackable<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__attispackable<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attispackable(
                        pointer,
                    )
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__attispackable(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__attispackable(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__attispackable(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__attispackable<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attispackable(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__attispackable(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__attispackable(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__attispackable(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 1
            == 49
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__atthasmissing<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__atthasmissing<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__atthasmissing<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_atthasmissing, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__atthasmissing<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__atthasmissing::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__atthasmissing<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__atthasmissing<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__atthasmissing(
                        pointer,
                    )
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__atthasmissing(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__atthasmissing(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__atthasmissing(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__atthasmissing<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__atthasmissing(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__atthasmissing(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__atthasmissing(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__atthasmissing(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 2
            == 50
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__attisdropped<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__attisdropped<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__attisdropped<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_attisdropped, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__attisdropped<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__attisdropped::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__attisdropped<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__attisdropped<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attisdropped(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__attisdropped(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__attisdropped(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__attisdropped(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__attisdropped<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attisdropped(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__attisdropped(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__attisdropped(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__attisdropped(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 3
            == 51
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__attgenerated<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__attgenerated<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__attgenerated<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_attgenerated, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__attgenerated<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__attgenerated::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__attgenerated<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__attgenerated<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attgenerated(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__attgenerated(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__attgenerated(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__attgenerated(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__attgenerated<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attgenerated(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__attgenerated(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__attgenerated(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__attgenerated(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attnullability) == 7
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnullability>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 7;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ControlFileData>() == 312);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ControlFileData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, unloggedLSN) == 136);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unloggedLSN>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, minRecoveryPoint) == 144
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minRecoveryPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, minRecoveryPointTLI) == 152
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minRecoveryPointTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupStartPoint) == 160
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupStartPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupEndPoint) == 168
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupEndPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupEndRequired) == 176
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupEndRequired>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, wal_level) == 180);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, wal_log_hints) == 184
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_log_hints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, MaxConnections) == 188
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_MaxConnections>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 188;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_worker_processes) == 192
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_worker_processes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_wal_senders) == 196
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_wal_senders>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 196;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_prepared_xacts) == 200
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_prepared_xacts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_locks_per_xact) == 204
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_locks_per_xact>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 204;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, track_commit_timestamp)
        == 208
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_track_commit_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, maxAlign) == 212);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxAlign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 212;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, blcksz) == 224);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blcksz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, relseg_size) == 228);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relseg_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_slru_pages_per_segment;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, slru_pages_per_segment)
        == 232
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slru_pages_per_segment>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_slru_pages_per_segment>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slru_pages_per_segment) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, xlog_blcksz) == 236);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlog_blcksz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, xlog_seg_size) == 240
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlog_seg_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, nameDataLen) == 244);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nameDataLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 244;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, indexMaxKeys) == 248
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexMaxKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, toast_max_chunk_size) == 252
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toast_max_chunk_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 252;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, loblksize) == 256);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_loblksize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, float8ByVal) == 260);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_float8ByVal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 260;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_data_checksum_version_init;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, data_checksum_version_init)
        == 264
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_checksum_version_init>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_data_checksum_version_init>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data_checksum_version_init) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, data_checksum_version)
        == 268
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_checksum_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 268;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, default_char_signedness)
        == 272
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_default_char_signedness>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, crc) == 308);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_crc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 308;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, format) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_format>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyFormat,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_format>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).format) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, freeze) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, header_line) == 12
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_header_line>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_force_array;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_array) == 128
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_array>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_force_array>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_array) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_null) == 136
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_null_all) == 144
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_null_all>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_selectively)
        == 160
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_selectively>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, on_error) == 164);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_on_error>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyOnErrorChoice,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 164;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, log_verbosity) == 168
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_verbosity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyLogVerbosityChoice,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, reject_limit) == 176
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reject_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_select) == 184
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_select>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 592);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, cur_lineno) == 304
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cur_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, relname_only) == 328
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relname_only>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, num_defaults) == 344
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_defaults>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, num_errors) == 376
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_errors>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 376;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_simd_enabled;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, simd_enabled) == 408
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_simd_enabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 408;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_simd_enabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).simd_enabled) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, volatile_defexprs) == 424
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_volatile_defexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 424;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, range_table) == 432
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_range_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, rteperminfos) == 440
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 440;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, max_fields) == 488
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_fields>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 488;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, line_buf_valid) == 528
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_line_buf_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_index) == 544
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 544;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_len) == 548
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 548;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_eof) == 552
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 552;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_error)
        == 553
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_error>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 553;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_index) == 568
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 568;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_len) == 572
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 572;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_reached_eof) == 576
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 576;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, bytes_processed) == 584
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bytes_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 584;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreatePublicationStmt, for_all_sequences)
        == 33
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_for_all_sequences>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreatePublicationStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreatePublicationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreatePublicationStmt> as c::expression::OrdinaryField<Field_for_all_sequences>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).for_all_sequences) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateSubscriptionStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateSubscriptionStmt>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateSubscriptionStmt, publication) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publication>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateSubscriptionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateSubscriptionStmt, options) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateSubscriptionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DecodingWorkerShared>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DecodingWorkerShared>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DecodingWorkerShared {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, initialized) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_initialized>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initialized) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_lsn_upto;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, lsn_upto) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn_upto>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_lsn_upto>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsn_upto) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, done) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_done>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).done) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_last_exported;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, last_exported) == 72
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_exported>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_last_exported>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_exported) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, mutex) == 76);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_mutex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mutex) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, dbid) == 80);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_dbid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbid) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, roleid) == 84);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roleid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_roleid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).roleid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_lock_timeout;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, lock_timeout) == 88
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lock_timeout>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_lock_timeout>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lock_timeout) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_transaction_timeout;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, transaction_timeout)
        == 92
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transaction_timeout>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_transaction_timeout>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transaction_timeout) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, relid) == 96);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_backend_pid;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, backend_pid) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_backend_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_pid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_backend_proc_number;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodingWorkerShared, backend_proc_number)
        == 116
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_proc_number>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodingWorkerShared, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodingWorkerShared> as c::expression::OrdinaryField<Field_backend_proc_number>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_proc_number) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DistinctColInfo>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DistinctColInfo>() == 4);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DistinctColInfo {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DistinctColInfo, colno) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DistinctColInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo> as c::expression::OrdinaryField<Field_colno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colno) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_opid;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DistinctColInfo, opid) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DistinctColInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo> as c::expression::OrdinaryField<Field_opid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_collid;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DistinctColInfo, collid) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DistinctColInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::DistinctColInfo> as c::expression::OrdinaryField<Field_collid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collid) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ElidedNode>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ElidedNode>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ElidedNode {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ElidedNode, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ElidedNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ElidedNode, plan_node_id) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_plan_node_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ElidedNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode> as c::expression::OrdinaryField<Field_plan_node_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).plan_node_id) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_elided_type;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ElidedNode, elided_type) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elided_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ElidedNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode> as c::expression::OrdinaryField<Field_elided_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elided_type) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_relids;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ElidedNode, relids) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ElidedNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode> as c::expression::OrdinaryField<Field_relids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relids) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ElidedNode, child_append_relid_sets) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_append_relid_sets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ElidedNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::ElidedNode> as c::expression::OrdinaryField<Field_child_append_relid_sets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_append_relid_sets) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_io;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, io) == 17);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_io>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_io>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).io) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, generic) == 18);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FdwRoutine>() == 376);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FdwRoutine>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FetchStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FetchStmt>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_direction_keyword;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FetchStmt, direction_keyword) == 28);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_direction_keyword>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FetchStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__FetchDirectionKeywords,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::FetchStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::FetchStmt> as c::expression::OrdinaryField<
        Field_direction_keyword,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).direction_keyword) };
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FetchStmt, location) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FetchStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::FetchStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::FetchStmt> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Fitness>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Fitness>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Fitness {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Fitness, disabled_nodes) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_disabled_nodes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Fitness>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Fitness,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Fitness> as c::expression::OrdinaryField<
        Field_disabled_nodes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).disabled_nodes) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ForeignDataWrapper>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ForeignDataWrapper>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_fdwconnection;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ForeignDataWrapper, fdwconnection) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fdwconnection>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ForeignDataWrapper>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ForeignDataWrapper, <<c::expression::CRecord<crate::__pgrx_c_bindings::ForeignDataWrapper> as c::expression::OrdinaryField<Field_fdwconnection>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fdwconnection) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ForeignDataWrapper, options) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ForeignDataWrapper>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper>() == 84
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper>() == 4
    );
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper,
        fdwconnection
    ) == 80
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fdwconnection>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_foreign_data_wrapper> as c::expression::OrdinaryField<Field_fdwconnection>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fdwconnection) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_puballsequences;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, puballsequences)
        == 73
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_puballsequences>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 73;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_publication, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication> as c::expression::OrdinaryField<Field_puballsequences>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).puballsequences) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubinsert) == 74
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubinsert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 74;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubupdate) == 75
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubupdate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 75;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubdelete) == 76
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubdelete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubtruncate) == 77
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubtruncate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 77;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubviaroot) == 78
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubviaroot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 78;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication, pubgencols) == 79
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubgencols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 79;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_publication_rel>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_publication_rel>() == 4);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_prexcept;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_publication_rel, prexcept) == 12
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prexcept>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication_rel>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_publication_rel, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_publication_rel> as c::expression::OrdinaryField<Field_prexcept>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prexcept) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_range>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_range>() == 4);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_rngconstruct2;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngconstruct2) == 20
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngconstruct2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_range, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range> as c::expression::OrdinaryField<Field_rngconstruct2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rngconstruct2) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_rngconstruct3;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngconstruct3) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngconstruct3>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_range, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range> as c::expression::OrdinaryField<Field_rngconstruct3>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rngconstruct3) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_rngmltconstruct0;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngmltconstruct0) == 28
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngmltconstruct0>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_range, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range> as c::expression::OrdinaryField<Field_rngmltconstruct0>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rngmltconstruct0) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_rngmltconstruct1;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngmltconstruct1) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngmltconstruct1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_range, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range> as c::expression::OrdinaryField<Field_rngmltconstruct1>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rngmltconstruct1) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_rngmltconstruct2;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngmltconstruct2) == 36
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngmltconstruct2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_range, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range> as c::expression::OrdinaryField<Field_rngmltconstruct2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rngmltconstruct2) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngcanonical) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngcanonical>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_range, rngsubdiff) == 44);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rngsubdiff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_range>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_subscription>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_subscription>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_subretaindeadtuples;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::FormData_pg_subscription,
        subretaindeadtuples
    ) == 92
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subretaindeadtuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription> as c::expression::OrdinaryField<Field_subretaindeadtuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subretaindeadtuples) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_submaxretention;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_subscription, submaxretention)
        == 96
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_submaxretention>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription> as c::expression::OrdinaryField<Field_submaxretention>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).submaxretention) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_subretentionactive;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_subscription, subretentionactive)
        == 100
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subretentionactive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription> as c::expression::OrdinaryField<Field_subretentionactive>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subretentionactive) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_subserver;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_subscription, subserver) == 104
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subserver>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription> as c::expression::OrdinaryField<Field_subserver>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subserver) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_ignore_nulls;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, ignore_nulls) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ignore_nulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::FuncCall,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall> as c::expression::OrdinaryField<
        Field_ignore_nulls,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ignore_nulls) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, agg_within_group) == 52);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agg_within_group>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, agg_star) == 53);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agg_star>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 53;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, agg_distinct) == 54);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agg_distinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 54;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, func_variadic) == 55);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_func_variadic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 55;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, funcformat) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcformat>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CoercionForm,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FuncCall, location) == 60);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FuncCall>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GISTPageOpaqueData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GISTPageOpaqueData>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_compareFn;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinState, compareFn) == 280);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<[crate::__pgrx_c_bindings::FmgrInfo; 32]>() == 1536);
    assert!(::core::mem::align_of::<[crate::__pgrx_c_bindings::FmgrInfo; 32]>() == 8);
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_compareFn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinState>
{
    type Member =
        c::expression::CArray<c::expression::CRecord<crate::__pgrx_c_bindings::FmgrInfo>, 32>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::GinState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::GinState> as c::expression::OrdinaryField<
        Field_compareFn,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).compareFn) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_supportCollation;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinState, supportCollation) == 9528);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<[crate::__pgrx_c_bindings::Oid; 32]>() == 128);
    assert!(::core::mem::align_of::<[crate::__pgrx_c_bindings::Oid; 32]>() == 4);
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_supportCollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinState>
{
    type Member = c::expression::CArray<
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>,
        32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9528;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::GinState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::GinState> as c::expression::OrdinaryField<
        Field_supportCollation,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).supportCollation) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GinTuple>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GinTuple>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, keylen) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_keylen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, typlen) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, typbyval) == 18);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, category) == 19);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_category>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, nitems) == 20);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nitems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinTuple, data) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinTuple>
{
    type Member = c::expression::CFlexibleArray<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        crate::__pgrx_c_bindings::__IncompleteArrayField<::core::ffi::c_char>,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GroupingExprInfo>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GroupingExprInfo>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GroupingExprInfo {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupingExprInfo, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupingExprInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GroupingExprInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupingExprInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_sortgroupref;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupingExprInfo, sortgroupref) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortgroupref>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupingExprInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GroupingExprInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupingExprInfo> as c::expression::OrdinaryField<Field_sortgroupref>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortgroupref) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashJoinState>() == 320);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashJoinState>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_JoinState) == 312);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_JoinState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_hj_KeepNullTuples;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_KeepNullTuples) == 316
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_KeepNullTuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 316;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState> as c::expression::OrdinaryField<Field_hj_KeepNullTuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hj_KeepNullTuples) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_MatchedOuter) == 317
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_MatchedOuter>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 317;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_OuterNotEmpty) == 318
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_OuterNotEmpty>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 318;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashState>() == 272);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashState>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_keep_null_tuples;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashState, keep_null_tuples) == 240);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_keep_null_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::HashState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::HashState> as c::expression::OrdinaryField<
        Field_keep_null_tuples,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).keep_null_tuples) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashState, parallel_state) == 264);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapPageFreeze>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapPageFreeze>() == 4);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_FreezePageConflictXid;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapPageFreeze, FreezePageConflictXid) == 12
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_FreezePageConflictXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapPageFreeze, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze> as c::expression::OrdinaryField<Field_FreezePageConflictXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).FreezePageConflictXid) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapPageFreeze, NoFreezePageRelfrozenXid)
        == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_NoFreezePageRelfrozenXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapPageFreeze, NoFreezePageRelminMxid) == 20
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_NoFreezePageRelminMxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapPageFreeze>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapScanDescData>() == 752);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapScanDescData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_nblocks) == 72);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_nblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_startblock) == 76
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_startblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_numblocks) == 80
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_numblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_inited) == 84);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_inited>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_coffset) == 86);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_coffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 86;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_cblock) == 88);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_cblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_cbuf) == 92);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_cbuf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_dir) == 136);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_dir>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ScanDirection,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_prefetch_block) == 140
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_prefetch_block>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_vmbuffer) == 152
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_vmbuffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_cindex) == 156);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_cindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_ntuples) == 160);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapTupleTableSlot>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapTupleTableSlot>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleTableSlot, off) == 80);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleTableSlot>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_prefetch_count;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IOStats>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IOStats>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IOStats {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, prefetch_count) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_prefetch_count,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prefetch_count) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_distance_sum;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, distance_sum) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distance_sum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_distance_sum,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distance_sum) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_distance_max;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, distance_max) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distance_max>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_distance_max,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distance_max) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_distance_capacity;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, distance_capacity) == 18);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distance_capacity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_distance_capacity,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distance_capacity) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_wait_count;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, wait_count) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_wait_count,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wait_count) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_io_count;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, io_count) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_io_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_io_count,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).io_count) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_io_nblocks;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, io_nblocks) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_io_nblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_io_nblocks,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).io_nblocks) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_io_in_progress;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IOStats, io_in_progress) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_io_in_progress>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IOStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IOStats,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IOStats> as c::expression::OrdinaryField<
        Field_io_in_progress,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).io_in_progress) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexElem>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexElem>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexElem, location) == 64);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexElem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexFetchHeapData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexFetchHeapData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexFetchHeapData, xs_cbuf) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_cbuf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_xs_blk;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexFetchHeapData, xs_blk) == 20);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_blk>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexFetchHeapData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData> as c::expression::OrdinaryField<Field_xs_blk>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xs_blk) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_xs_vmbuffer;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexFetchHeapData, xs_vmbuffer) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_vmbuffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexFetchHeapData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData> as c::expression::OrdinaryField<Field_xs_vmbuffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xs_vmbuffer) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexFetchTableData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexFetchTableData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexFetchTableData, flags) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexFetchTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_disabled;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, disabled) == 172);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_disabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_disabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).disabled) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, hypothetical) == 173);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hypothetical>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 173;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amcanorderbyop) == 174);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanorderbyop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 174;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amoptionalkey) == 175);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amoptionalkey>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 175;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amsearcharray) == 176);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amsearcharray>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amsearchnulls) == 177);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amsearchnulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 177;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amhasgettuple) == 178);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amhasgettuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 178;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amhasgetbitmap) == 179);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amhasgetbitmap>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 179;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amcanparallel) == 180);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanparallel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amcanmarkpos) == 181);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanmarkpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 181;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_is_autovacuum;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, is_autovacuum) == 17
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_autovacuum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexVacuumInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo> as c::expression::OrdinaryField<Field_is_autovacuum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_autovacuum) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, report_progress) == 18
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_report_progress>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, estimated_count) == 19
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_estimated_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Instrumentation>() == 360);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Instrumentation>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JoinPathExtraData>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JoinPathExtraData>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_pgs_mask;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinPathExtraData, pgs_mask) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgs_mask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinPathExtraData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JoinPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinPathExtraData> as c::expression::OrdinaryField<Field_pgs_mask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pgs_mask) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonConstructorExpr>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonConstructorExpr>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExpr, absent_on_null) == 56
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_absent_on_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExpr, unique) == 57);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 57;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonConstructorExpr, location) == 60);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonConstructorExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_exprBaseType;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonIsPredicate, exprBaseType) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exprBaseType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonIsPredicate, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate> as c::expression::OrdinaryField<Field_exprBaseType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exprBaseType) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonIsPredicate, location) == 36);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonIsPredicate>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbInState>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbInState>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonbInState {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbInState, unique_keys) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique_keys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbInState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbInState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbInState> as c::expression::OrdinaryField<Field_unique_keys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique_keys) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_idxisreplident;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, idxisreplident) == 96
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_idxisreplident>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelMapEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry> as c::expression::OrdinaryField<Field_idxisreplident>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).idxisreplident) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, state) == 97);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 97;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_localrelid;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepSequenceInfo {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepSequenceInfo, localrelid) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_localrelid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepSequenceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo> as c::expression::OrdinaryField<Field_localrelid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).localrelid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_page_lsn;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepSequenceInfo, page_lsn) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_page_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepSequenceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo> as c::expression::OrdinaryField<Field_page_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).page_lsn) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepSequenceInfo, last_value) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepSequenceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo> as c::expression::OrdinaryField<Field_last_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_value) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepSequenceInfo, is_called) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_called>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepSequenceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo> as c::expression::OrdinaryField<Field_is_called>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_called) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_found_on_pub;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepSequenceInfo, found_on_pub) == 41
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_found_on_pub>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepSequenceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepSequenceInfo> as c::expression::OrdinaryField<Field_found_on_pub>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).found_on_pub) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepWorker>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepWorker>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_oldest_nonremovable_xid;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, oldest_nonremovable_xid)
        == 88
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldest_nonremovable_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_oldest_nonremovable_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldest_nonremovable_xid) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_lsn) == 96);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_send_time) == 104
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_send_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_recv_time) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_recv_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, reply_lsn) == 120);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reply_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, reply_time) == 128);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reply_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_last_seqsync_start_time;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_seqsync_start_time)
        == 136
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_seqsync_start_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_last_seqsync_start_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_seqsync_start_time) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Memoize>() == 176);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Memoize>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MemoizePath>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MemoizePath>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizePath, est_entries) == 108);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_est_entries>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeAppend>() == 176);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeAppend>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppend, child_append_relid_sets) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_append_relid_sets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAppend, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend> as c::expression::OrdinaryField<Field_child_append_relid_sets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_append_relid_sets) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppend, mergeplans) == 120);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppend, numCols) == 128);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppend, part_prune_index) == 168
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_prune_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MinimalTupleTableSlot>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MinimalTupleTableSlot>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinimalTupleTableSlot, off) == 112);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleTableSlot>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, resultRelations) == 120);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, updateColnosLists) == 128
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_updateColnosLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, withCheckOptionLists) == 136
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_withCheckOptionLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, returningLists) == 160);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, fdwPrivLists) == 168);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fdwPrivLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, rowMarks) == 184);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, epqParam) == 192);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_epqParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, onConflictAction) == 196
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_onConflictAction>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OnConflictAction,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 196;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, arbiterIndexes) == 200);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arbiterIndexes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_onConflictLockStrength;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, onConflictLockStrength) == 208
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_onConflictLockStrength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_onConflictLockStrength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).onConflictLockStrength) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 472);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_mt_fdwPrivLists;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_fdwPrivLists) == 464
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_fdwPrivLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 464;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_mt_fdwPrivLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mt_fdwPrivLists) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NodeInstrumentation>() == 440);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NodeInstrumentation>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::NodeInstrumentation {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::NodeInstrumentation, async_mode) == 360
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_async_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NodeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation> as c::expression::OrdinaryField<Field_async_mode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).async_mode) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NodeInstrumentation, running) == 361);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_running>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 361;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NodeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation> as c::expression::OrdinaryField<Field_running>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).running) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::NodeInstrumentation, ntuples2) == 408
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 408;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NodeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation> as c::expression::OrdinaryField<Field_ntuples2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples2) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::NodeInstrumentation, nfiltered1) == 424
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfiltered1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 424;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NodeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation> as c::expression::OrdinaryField<Field_nfiltered1>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nfiltered1) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::NodeInstrumentation, nfiltered2) == 432
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfiltered2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NodeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation> as c::expression::OrdinaryField<Field_nfiltered2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nfiltered2) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OnConflictActionState>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OnConflictActionState>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OnConflictActionState {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictActionState, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictActionState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictActionState, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictActionState> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_oc_LockStrength;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictActionState, oc_LockStrength) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oc_LockStrength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictActionState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictActionState, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictActionState> as c::expression::OrdinaryField<Field_oc_LockStrength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oc_LockStrength) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OnConflictClause>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OnConflictClause>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_lockStrength;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictClause, lockStrength) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lockStrength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause> as c::expression::OrdinaryField<Field_lockStrength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lockStrength) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictClause, targetList) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictClause, location) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictExpr, lockStrength) == 28);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lockStrength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr> as c::expression::OrdinaryField<Field_lockStrength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lockStrength) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGPROC>() == 896);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGPROC>() == 128);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procgloballist) == 0);
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_freeProcsLink;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, freeProcsLink) == 8);
#[cfg(feature = "pg19")]
impl c::expression::OffsetField<Field_freeProcsLink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, freeProcsLink);
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pid) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_backendType;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, backendType) == 28);
#[cfg(feature = "pg19")]
impl c::expression::OffsetField<Field_backendType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, backendType);
}
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backendType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__BackendType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_backendType,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backendType) };
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, databaseId) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_databaseId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, roleId) == 36);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roleId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, tempNamespaceId) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tempNamespaceId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pgxactoff) == 44);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgxactoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, statusFlags) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_statusFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, vxid) == 52);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, xid) == 60);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, xmin) == 64);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, subxidStatus) == 68);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, subxids) == 72);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procLatch) == 328);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, sem) == 344);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, delayChkptFlags) == 352);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_delayChkptFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_pendingRecoveryConflicts;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pendingRecoveryConflicts) == 356
);
#[cfg(feature = "pg19")]
impl c::expression::OffsetField<Field_pendingRecoveryConflicts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::pg_atomic_uint32>;
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pendingRecoveryConflicts);
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaiting) == 360);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lwWaiting>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaitMode) == 361);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lwWaitMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 361;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaitLink) == 364);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, cvWaitLink) == 372);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLeader) == 384);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupMembers) == 392);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLink) == 408);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLock) == 424);
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_waitLink;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLink) == 432);
#[cfg(feature = "pg19")]
impl c::expression::OffsetField<Field_waitLink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLink);
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitProcLock) == 448);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLockMode) == 456);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitLockMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 456;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, heldLocks) == 460);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_heldLocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 460;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitStart) == 464);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitStatus) == 472);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitStatus>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_ProcWaitStatus,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 472;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, myProcLocks) == 480);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpInfoLock) == 736);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpLockBits) == 752);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpRelId) == 760);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpVXIDLock) == 768);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpVXIDLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 768;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpLocalTransactionId) == 772);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpLocalTransactionId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 772;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLSN) == 776);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitLSN>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 776;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, syncRepState) == 784);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_syncRepState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 784;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, syncRepLinks) == 792);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupMember) == 808);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_procArrayGroupMember>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 808;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupNext) == 812);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupMemberXid) == 816
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_procArrayGroupMemberXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 816;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMember) == 820);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMember>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 820;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupNext) == 824);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberXid) == 828);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 828;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberXidStatus) == 832
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberXidStatus>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 832;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberPage) == 840);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 840;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberLsn) == 848);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberLsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 848;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, wait_event_info) == 856);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait_event_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 856;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGShmemHeader>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGShmemHeader>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_content_offset;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, content_offset) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_content_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGShmemHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader> as c::expression::OrdinaryField<Field_content_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).content_offset) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, device) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_device>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, inode) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_freeProcsLock;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, freeProcsLock) == 36);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeProcsLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PROC_HDR,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR> as c::expression::OrdinaryField<
        Field_freeProcsLock,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeProcsLock) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_nplanned;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PVWorkerStats>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PVWorkerStats>() == 4);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PVWorkerStats {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PVWorkerStats, nplanned) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nplanned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PVWorkerStats>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PVWorkerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PVWorkerStats> as c::expression::OrdinaryField<Field_nplanned>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nplanned) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_nlaunched;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PVWorkerStats, nlaunched) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nlaunched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PVWorkerStats>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PVWorkerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PVWorkerStats> as c::expression::OrdinaryField<Field_nlaunched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nlaunched) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PageHeaderData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PageHeaderData>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_phs_numblock;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBlockTableScanDescData, phs_numblock)
        == 36
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_numblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData> as c::expression::OrdinaryField<Field_phs_numblock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phs_numblock) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelIndexScanDescData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelIndexScanDescData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelIndexScanDescData, ps_offset_am)
        == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ps_offset_am>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneInfo, relids) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneInfo> as c::expression::OrdinaryField<Field_relids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relids) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneState, initialized) == 34
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneState, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState> as c::expression::OrdinaryField<Field_initialized>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initialized) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_Backend>() == 2928);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_Backend>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_sync_seq_error_count;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_BackendSubEntry, sync_seq_error_count)
        == 8
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_seq_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_BackendSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry> as c::expression::OrdinaryField<Field_sync_seq_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_seq_error_count) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_sync_table_error_count;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_BackendSubEntry,
        sync_table_error_count
    ) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_table_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_BackendSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry> as c::expression::OrdinaryField<Field_sync_table_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_table_error_count) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_Lock>() == 296);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_Lock>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_Lock {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_Lock, stat_reset_timestamp) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_Lock>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_Lock, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_Lock> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_waits;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_LockEntry>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_LockEntry>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_LockEntry {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_LockEntry, waits) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waits>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_LockEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry> as c::expression::OrdinaryField<Field_waits>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waits) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_wait_time;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_LockEntry, wait_time) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_LockEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry> as c::expression::OrdinaryField<Field_wait_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wait_time) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_fastpath_exceeded;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_LockEntry, fastpath_exceeded) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fastpath_exceeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_LockEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_LockEntry> as c::expression::OrdinaryField<Field_fastpath_exceeded>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fastpath_exceeded) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatFuncEntry>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatFuncEntry>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatFuncEntry, stat_reset_timestamp)
        == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatFuncEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatFuncEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatFuncEntry> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatSubEntry>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatSubEntry>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatSubEntry, sync_seq_error_count)
        == 8
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_seq_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry> as c::expression::OrdinaryField<Field_sync_seq_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_seq_error_count) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatSubEntry, sync_table_error_count)
        == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_table_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry> as c::expression::OrdinaryField<Field_sync_table_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_table_error_count) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatSubEntry, stat_reset_timestamp)
        == 88
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatTabEntry>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatTabEntry>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_stat_reset_time;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, stat_reset_time) == 216
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_stat_reset_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_time) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_WalCounters>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_WalCounters>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_fpi_bytes) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_fpi_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters> as c::expression::OrdinaryField<Field_wal_fpi_bytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_fpi_bytes) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_buffers_full) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_buffers_full>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_WalStats>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_WalStats>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, stat_reset_timestamp) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::NodeInstrumentation>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::NodeInstrumentation>() == 8);
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_instrument>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::NodeInstrumentation>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_planOrigin;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, planOrigin) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_planOrigin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PlannedStmtOrigin,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_planOrigin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).planOrigin) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, hasReturning) == 28);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasReturning>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, hasModifyingCTE) == 29);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasModifyingCTE>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 29;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, canSetTag) == 30);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canSetTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, transientPlan) == 31);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transientPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 31;
}
