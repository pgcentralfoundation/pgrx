// Shared generated C macro support.

// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initValue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_initValue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initValue) };
#[doc(hidden)]
pub struct Field_initValueIsNull;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, initValueIsNull) == 296
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initValueIsNull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_initValueIsNull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initValueIsNull) };
#[doc(hidden)]
pub struct Field_inputtypeLen;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, inputtypeLen) == 298
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputtypeLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 298;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_inputtypeLen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputtypeLen) };
#[doc(hidden)]
pub struct Field_transtypeLen;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, transtypeLen) == 300
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transtypeLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 300;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_transtypeLen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transtypeLen) };
#[doc(hidden)]
pub struct Field_inputtypeByVal;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, inputtypeByVal) == 302
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputtypeByVal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 302;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_inputtypeByVal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputtypeByVal) };
#[doc(hidden)]
pub struct Field_transtypeByVal;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, transtypeByVal) == 303
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transtypeByVal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 303;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_transtypeByVal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transtypeByVal) };
#[doc(hidden)]
pub struct Field_args;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggTransInfo>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggTransInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggTransInfo {}
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, args) == 0);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, transfn_oid) == 16);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_transfn_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transfn_oid) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, serialfn_oid) == 20);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_serialfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_serialfn_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).serialfn_oid) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, deserialfn_oid) == 24);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deserialfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_deserialfn_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deserialfn_oid) };
#[doc(hidden)]
pub struct Field_combinefn_oid;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, combinefn_oid) == 28);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_combinefn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_combinefn_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).combinefn_oid) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, aggtranstype) == 32);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtranstype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_aggtranstype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggtranstype) };
#[doc(hidden)]
pub struct Field_aggtranstypmod;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, aggtranstypmod) == 36);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtranstypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_aggtranstypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggtranstypmod) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, transtypeLen) == 40);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transtypeLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_transtypeLen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transtypeLen) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, transtypeByVal) == 44);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transtypeByVal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_transtypeByVal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transtypeByVal) };
#[doc(hidden)]
pub struct Field_aggtransspace;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, aggtransspace) == 48);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtransspace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_aggtransspace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggtransspace) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, initValue) == 56);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initValue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_initValue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initValue) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggTransInfo, initValueIsNull) == 64);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initValueIsNull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggTransInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggTransInfo> as c::expression::OrdinaryField<Field_initValueIsNull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initValueIsNull) };
#[doc(hidden)]
pub struct Field_aggfnoid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Aggref>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Aggref>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Aggref {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggfnoid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggfnoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggfnoid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggfnoid) };
#[doc(hidden)]
pub struct Field_aggtype;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggtype) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggtype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggtype) };
#[doc(hidden)]
pub struct Field_aggcollid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggcollid) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggcollid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggcollid) };
#[doc(hidden)]
pub struct Field_inputcollid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, inputcollid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_inputcollid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputcollid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggtranstype) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtranstype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggtranstype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggtranstype) };
#[doc(hidden)]
pub struct Field_aggargtypes;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggargtypes) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggargtypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
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
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggargtypes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggargtypes) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggdirectargs) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggdirectargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggdirectargs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggdirectargs) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, args) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_args,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[doc(hidden)]
pub struct Field_aggorder;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggorder) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggorder>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
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
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggorder,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggorder) };
#[doc(hidden)]
pub struct Field_aggdistinct;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggdistinct) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggdistinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggdistinct,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggdistinct) };
#[doc(hidden)]
pub struct Field_aggstar;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggstar) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggstar>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggstar,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggstar) };
#[doc(hidden)]
pub struct Field_aggvariadic;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggvariadic) == 73);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggvariadic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 73;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggvariadic,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggvariadic) };
#[doc(hidden)]
pub struct Field_aggkind;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggkind) == 74);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggkind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 74;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggkind,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggkind) };
#[doc(hidden)]
pub struct Field_agglevelsup;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, agglevelsup) == 76);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agglevelsup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_agglevelsup,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).agglevelsup) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggsplit) == 80);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggsplit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AggSplit,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggsplit,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggsplit) };
#[doc(hidden)]
pub struct Field_aggno;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggno) == 84);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggno) };
#[doc(hidden)]
pub struct Field_aggtransno;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, aggtransno) == 88);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtransno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_aggtransno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggtransno) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Aggref, location) == 92);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Aggref>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Aggref,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Aggref> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggregateInstrumentation>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggregateInstrumentation>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggregateInstrumentation {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggregateInstrumentation, hash_mem_peak) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_mem_peak>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggregateInstrumentation>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggregateInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggregateInstrumentation> as c::expression::OrdinaryField<Field_hash_mem_peak>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_mem_peak) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggregateInstrumentation, hash_disk_used)
        == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_disk_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggregateInstrumentation>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggregateInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggregateInstrumentation> as c::expression::OrdinaryField<Field_hash_disk_used>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_disk_used) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggregateInstrumentation, hash_batches_used)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_batches_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggregateInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggregateInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggregateInstrumentation> as c::expression::OrdinaryField<Field_hash_batches_used>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_batches_used) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Alias>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Alias>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Alias {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Alias, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Alias>
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
    crate::__pgrx_c_bindings::Alias,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Alias> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_colnames;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Alias, colnames) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colnames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Alias>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Alias,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Alias> as c::expression::OrdinaryField<
        Field_colnames,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colnames) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterCollationStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterCollationStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterCollationStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterCollationStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterCollationStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterCollationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterCollationStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_collname;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterCollationStmt, collname) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterCollationStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterCollationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterCollationStmt> as c::expression::OrdinaryField<Field_collname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collname) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterDatabaseRefreshCollStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterDatabaseRefreshCollStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterDatabaseRefreshCollStmt {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterDatabaseRefreshCollStmt, type_) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDatabaseRefreshCollStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterDatabaseRefreshCollStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterDatabaseRefreshCollStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterDatabaseSetStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterDatabaseSetStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterDatabaseSetStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterDatabaseSetStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDatabaseSetStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterDatabaseSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterDatabaseSetStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterDatabaseStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterDatabaseStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterDatabaseStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterDatabaseStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDatabaseStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterDatabaseStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterDatabaseStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterDatabaseStmt, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDatabaseStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterDatabaseStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterDatabaseStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterDefaultPrivilegesStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterDefaultPrivilegesStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterDefaultPrivilegesStmt {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterDefaultPrivilegesStmt, type_) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDefaultPrivilegesStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterDefaultPrivilegesStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterDefaultPrivilegesStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterDefaultPrivilegesStmt, options) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDefaultPrivilegesStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterDefaultPrivilegesStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterDefaultPrivilegesStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterDomainStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterDomainStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterDomainStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterDomainStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterDomainStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_subtype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterDomainStmt, subtype) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterDomainStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt> as c::expression::OrdinaryField<Field_subtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subtype) };
#[doc(hidden)]
pub struct Field_typeName;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterDomainStmt, typeName) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeName>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterDomainStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt> as c::expression::OrdinaryField<Field_typeName>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeName) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterDomainStmt, behavior) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_behavior>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DropBehavior,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterDomainStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt> as c::expression::OrdinaryField<Field_behavior>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).behavior) };
#[doc(hidden)]
pub struct Field_missing_ok;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterDomainStmt, missing_ok) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterDomainStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterEnumStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterEnumStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterEnumStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterEnumStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterEnumStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterEnumStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterEnumStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterEnumStmt, typeName) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeName>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterEnumStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterEnumStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterEnumStmt> as c::expression::OrdinaryField<Field_typeName>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeName) };
#[doc(hidden)]
pub struct Field_newValIsAfter;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterEnumStmt, newValIsAfter) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newValIsAfter>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterEnumStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterEnumStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterEnumStmt> as c::expression::OrdinaryField<Field_newValIsAfter>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).newValIsAfter) };
#[doc(hidden)]
pub struct Field_skipIfNewValExists;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterEnumStmt, skipIfNewValExists) == 41
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skipIfNewValExists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterEnumStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterEnumStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterEnumStmt> as c::expression::OrdinaryField<Field_skipIfNewValExists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skipIfNewValExists) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterEventTrigStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterEventTrigStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterEventTrigStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterEventTrigStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterEventTrigStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterEventTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterEventTrigStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_tgenabled;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterEventTrigStmt, tgenabled) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tgenabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterEventTrigStmt>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterEventTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterEventTrigStmt> as c::expression::OrdinaryField<Field_tgenabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tgenabled) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterExtensionContentsStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterExtensionContentsStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterExtensionContentsStmt {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterExtensionContentsStmt, type_) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterExtensionContentsStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterExtensionContentsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterExtensionContentsStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_action;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterExtensionContentsStmt, action) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterExtensionContentsStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterExtensionContentsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterExtensionContentsStmt> as c::expression::OrdinaryField<Field_action>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).action) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterExtensionContentsStmt, objtype) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterExtensionContentsStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterExtensionContentsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterExtensionContentsStmt> as c::expression::OrdinaryField<Field_objtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objtype) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterExtensionStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterExtensionStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterExtensionStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterExtensionStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterExtensionStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterExtensionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterExtensionStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterExtensionStmt, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterExtensionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterExtensionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterExtensionStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterFdwStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterFdwStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterFdwStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterFdwStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterFdwStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterFdwStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterFdwStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_func_options;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterFdwStmt, func_options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_func_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterFdwStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterFdwStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterFdwStmt> as c::expression::OrdinaryField<Field_func_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).func_options) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterFdwStmt, options) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterFdwStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterFdwStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterFdwStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterForeignServerStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterForeignServerStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterForeignServerStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterForeignServerStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterForeignServerStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterForeignServerStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterForeignServerStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterForeignServerStmt, options) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterForeignServerStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterForeignServerStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterForeignServerStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_has_version;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterForeignServerStmt, has_version) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterForeignServerStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterForeignServerStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterForeignServerStmt> as c::expression::OrdinaryField<Field_has_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_version) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterFunctionStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterFunctionStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterFunctionStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterFunctionStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterFunctionStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterFunctionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterFunctionStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterFunctionStmt, objtype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterFunctionStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterFunctionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterFunctionStmt> as c::expression::OrdinaryField<Field_objtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objtype) };
#[doc(hidden)]
pub struct Field_actions;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterFunctionStmt, actions) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_actions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterFunctionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterFunctionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterFunctionStmt> as c::expression::OrdinaryField<Field_actions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).actions) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterObjectDependsStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterObjectDependsStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterObjectDependsStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterObjectDependsStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectDependsStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterObjectDependsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectDependsStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_objectType;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterObjectDependsStmt, objectType) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objectType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectDependsStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterObjectDependsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectDependsStmt> as c::expression::OrdinaryField<Field_objectType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objectType) };
#[doc(hidden)]
pub struct Field_remove;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterObjectDependsStmt, remove) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_remove>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectDependsStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterObjectDependsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectDependsStmt> as c::expression::OrdinaryField<Field_remove>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).remove) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterObjectSchemaStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterObjectSchemaStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterObjectSchemaStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterObjectSchemaStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectSchemaStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterObjectSchemaStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectSchemaStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterObjectSchemaStmt, objectType) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objectType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectSchemaStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterObjectSchemaStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectSchemaStmt> as c::expression::OrdinaryField<Field_objectType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objectType) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterObjectSchemaStmt, missing_ok) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectSchemaStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterObjectSchemaStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterObjectSchemaStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterOpFamilyStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterOpFamilyStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterOpFamilyStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterOpFamilyStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterOpFamilyStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterOpFamilyStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterOpFamilyStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_opfamilyname;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterOpFamilyStmt, opfamilyname) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opfamilyname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterOpFamilyStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterOpFamilyStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterOpFamilyStmt> as c::expression::OrdinaryField<Field_opfamilyname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opfamilyname) };
#[doc(hidden)]
pub struct Field_isDrop;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterOpFamilyStmt, isDrop) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isDrop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterOpFamilyStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterOpFamilyStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterOpFamilyStmt> as c::expression::OrdinaryField<Field_isDrop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isDrop) };
#[doc(hidden)]
pub struct Field_items;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterOpFamilyStmt, items) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_items>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterOpFamilyStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterOpFamilyStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterOpFamilyStmt> as c::expression::OrdinaryField<Field_items>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).items) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterOperatorStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterOperatorStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterOperatorStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterOperatorStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterOperatorStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterOperatorStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterOperatorStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterOperatorStmt, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterOperatorStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterOperatorStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterOperatorStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterOwnerStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterOwnerStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterOwnerStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterOwnerStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterOwnerStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterOwnerStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterOwnerStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterOwnerStmt, objectType) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objectType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterOwnerStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterOwnerStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterOwnerStmt> as c::expression::OrdinaryField<Field_objectType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objectType) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterPolicyStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterPolicyStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterPolicyStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPolicyStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPolicyStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterPolicyStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterPolicyStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_roles;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPolicyStmt, roles) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roles>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPolicyStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterPolicyStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterPolicyStmt> as c::expression::OrdinaryField<Field_roles>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).roles) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterPublicationStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterPublicationStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterPublicationStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPublicationStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterPublicationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPublicationStmt, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterPublicationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_pubobjects;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPublicationStmt, pubobjects) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubobjects>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterPublicationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt> as c::expression::OrdinaryField<Field_pubobjects>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pubobjects) };
#[doc(hidden)]
pub struct Field_for_all_tables;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPublicationStmt, for_all_tables) == 32
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_for_all_tables>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterPublicationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt> as c::expression::OrdinaryField<Field_for_all_tables>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).for_all_tables) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPublicationStmt, action) == 36);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AlterPublicationAction,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterPublicationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt> as c::expression::OrdinaryField<Field_action>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).action) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterRoleSetStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterRoleSetStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterRoleSetStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterRoleSetStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterRoleSetStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterRoleSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterRoleSetStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterRoleStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterRoleStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterRoleStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterRoleStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterRoleStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterRoleStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterRoleStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterRoleStmt, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterRoleStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterRoleStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterRoleStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterRoleStmt, action) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterRoleStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterRoleStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterRoleStmt> as c::expression::OrdinaryField<Field_action>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).action) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterSeqStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterSeqStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterSeqStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSeqStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSeqStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterSeqStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterSeqStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSeqStmt, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSeqStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterSeqStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterSeqStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_for_identity;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSeqStmt, for_identity) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_for_identity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSeqStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterSeqStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterSeqStmt> as c::expression::OrdinaryField<Field_for_identity>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).for_identity) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSeqStmt, missing_ok) == 25);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSeqStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterSeqStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterSeqStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterStatsStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterStatsStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterStatsStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterStatsStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterStatsStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterStatsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterStatsStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_defnames;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterStatsStmt, defnames) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_defnames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterStatsStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterStatsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterStatsStmt> as c::expression::OrdinaryField<Field_defnames>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).defnames) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_stxstattarget;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterStatsStmt, stxstattarget) == 16);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stxstattarget>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterStatsStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterStatsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterStatsStmt> as c::expression::OrdinaryField<Field_stxstattarget>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stxstattarget) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterStatsStmt, missing_ok) == 20);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterStatsStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterStatsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterStatsStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterSubscriptionStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterSubscriptionStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterSubscriptionStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSubscriptionStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterSubscriptionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSubscriptionStmt, kind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AlterSubscriptionType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterSubscriptionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt> as c::expression::OrdinaryField<Field_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
#[doc(hidden)]
pub struct Field_publication;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSubscriptionStmt, publication) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publication>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterSubscriptionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt> as c::expression::OrdinaryField<Field_publication>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).publication) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSubscriptionStmt, options) == 32
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterSubscriptionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterSystemStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterSystemStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterSystemStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSystemStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSystemStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterSystemStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterSystemStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterTSConfigurationStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterTSConfigurationStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterTSConfigurationStmt {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTSConfigurationStmt, type_) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterTSConfigurationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTSConfigurationStmt, kind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AlterTSConfigType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTSConfigurationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt> as c::expression::OrdinaryField<Field_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
#[doc(hidden)]
pub struct Field_cfgname;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTSConfigurationStmt, cfgname) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cfgname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTSConfigurationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt> as c::expression::OrdinaryField<Field_cfgname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cfgname) };
#[doc(hidden)]
pub struct Field_tokentype;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTSConfigurationStmt, tokentype) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tokentype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTSConfigurationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt> as c::expression::OrdinaryField<Field_tokentype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tokentype) };
#[doc(hidden)]
pub struct Field_dicts;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTSConfigurationStmt, dicts) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dicts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTSConfigurationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt> as c::expression::OrdinaryField<Field_dicts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dicts) };
#[doc(hidden)]
pub struct Field_override;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTSConfigurationStmt, override_) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_override>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTSConfigurationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt> as c::expression::OrdinaryField<Field_override>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).override_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTSConfigurationStmt, replace) == 33
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTSConfigurationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt> as c::expression::OrdinaryField<Field_replace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).replace) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTSConfigurationStmt, missing_ok) == 34
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTSConfigurationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSConfigurationStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterTSDictionaryStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterTSDictionaryStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterTSDictionaryStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTSDictionaryStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSDictionaryStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterTSDictionaryStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSDictionaryStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_dictname;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTSDictionaryStmt, dictname) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dictname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSDictionaryStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTSDictionaryStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSDictionaryStmt> as c::expression::OrdinaryField<Field_dictname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dictname) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTSDictionaryStmt, options) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSDictionaryStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTSDictionaryStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTSDictionaryStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterTableCmd>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterTableCmd>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterTableCmd {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableCmd, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd>
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
const _: Projection<crate::__pgrx_c_bindings::AlterTableCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableCmd, subtype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AlterTableType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd> as c::expression::OrdinaryField<Field_subtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subtype) };
#[doc(hidden)]
pub struct Field_num;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableCmd, num) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd> as c::expression::OrdinaryField<Field_num>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableCmd, behavior) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_behavior>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DropBehavior,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd> as c::expression::OrdinaryField<Field_behavior>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).behavior) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableCmd, missing_ok) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
#[doc(hidden)]
pub struct Field_recurse;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableCmd, recurse) == 45);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recurse>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 45;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableCmd> as c::expression::OrdinaryField<Field_recurse>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recurse) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterTableMoveAllStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterTableMoveAllStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterTableMoveAllStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableMoveAllStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableMoveAllStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterTableMoveAllStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableMoveAllStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableMoveAllStmt, objtype) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableMoveAllStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableMoveAllStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableMoveAllStmt> as c::expression::OrdinaryField<Field_objtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objtype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableMoveAllStmt, roles) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roles>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableMoveAllStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableMoveAllStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableMoveAllStmt> as c::expression::OrdinaryField<Field_roles>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).roles) };
#[doc(hidden)]
pub struct Field_nowait;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableMoveAllStmt, nowait) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nowait>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableMoveAllStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableMoveAllStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableMoveAllStmt> as c::expression::OrdinaryField<Field_nowait>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nowait) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt, type_) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt, options) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_isReset;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt, isReset) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isReset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableSpaceOptionsStmt> as c::expression::OrdinaryField<Field_isReset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isReset) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterTableStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterTableStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterTableStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterTableStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_cmds;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableStmt, cmds) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmds>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableStmt> as c::expression::OrdinaryField<Field_cmds>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmds) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableStmt, objtype) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableStmt> as c::expression::OrdinaryField<Field_objtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objtype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableStmt, missing_ok) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
#[doc(hidden)]
pub struct Field_relid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterTableUtilityContext>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterTableUtilityContext>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterTableUtilityContext {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTableUtilityContext, relid) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableUtilityContext>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTableUtilityContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTableUtilityContext> as c::expression::OrdinaryField<Field_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterTypeStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterTypeStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterTypeStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTypeStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTypeStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterTypeStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTypeStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTypeStmt, typeName) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeName>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTypeStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTypeStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTypeStmt> as c::expression::OrdinaryField<Field_typeName>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeName) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterTypeStmt, options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterTypeStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterTypeStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterTypeStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterUserMappingStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterUserMappingStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterUserMappingStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterUserMappingStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterUserMappingStmt>
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
const _: Projection<crate::__pgrx_c_bindings::AlterUserMappingStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterUserMappingStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterUserMappingStmt, options) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterUserMappingStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterUserMappingStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterUserMappingStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_subplans;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlternativeSubPlan>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlternativeSubPlan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlternativeSubPlan {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlternativeSubPlan, subplans) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlternativeSubPlan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlternativeSubPlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlternativeSubPlan> as c::expression::OrdinaryField<Field_subplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subplans) };
#[doc(hidden)]
pub struct Field_appendplans;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Append>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Append>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Append {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, appendplans) == 112);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Append,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Append> as c::expression::OrdinaryField<
        Field_appendplans,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).appendplans) };
#[doc(hidden)]
pub struct Field_nasyncplans;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, nasyncplans) == 120);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nasyncplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Append,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Append> as c::expression::OrdinaryField<
        Field_nasyncplans,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nasyncplans) };
#[doc(hidden)]
pub struct Field_first_partial_plan;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, first_partial_plan) == 124);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_first_partial_plan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Append,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Append> as c::expression::OrdinaryField<
        Field_first_partial_plan,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).first_partial_plan) };
#[doc(hidden)]
pub struct Field_subpaths;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AppendPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AppendPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AppendPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendPath, subpaths) == 72);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subpaths>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendPath> as c::expression::OrdinaryField<Field_subpaths>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subpaths) };
#[doc(hidden)]
pub struct Field_first_partial_path;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendPath, first_partial_path) == 80
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_first_partial_path>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendPath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendPath> as c::expression::OrdinaryField<Field_first_partial_path>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).first_partial_path) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AppendRelInfo>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AppendRelInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AppendRelInfo {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendRelInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo>
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
const _: Projection<crate::__pgrx_c_bindings::AppendRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_parent_relid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendRelInfo, parent_relid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parent_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo> as c::expression::OrdinaryField<Field_parent_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parent_relid) };
#[doc(hidden)]
pub struct Field_child_relid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendRelInfo, child_relid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo> as c::expression::OrdinaryField<Field_child_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_relid) };
#[doc(hidden)]
pub struct Field_parent_reltype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendRelInfo, parent_reltype) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parent_reltype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo> as c::expression::OrdinaryField<Field_parent_reltype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parent_reltype) };
#[doc(hidden)]
pub struct Field_child_reltype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendRelInfo, child_reltype) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_reltype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo> as c::expression::OrdinaryField<Field_child_reltype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_reltype) };
#[doc(hidden)]
pub struct Field_translated_vars;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendRelInfo, translated_vars) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_translated_vars>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo> as c::expression::OrdinaryField<Field_translated_vars>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).translated_vars) };
#[doc(hidden)]
pub struct Field_num_child_cols;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendRelInfo, num_child_cols) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_child_cols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo> as c::expression::OrdinaryField<Field_num_child_cols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_child_cols) };
#[doc(hidden)]
pub struct Field_parent_reloid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendRelInfo, parent_reloid) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parent_reloid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendRelInfo> as c::expression::OrdinaryField<Field_parent_reloid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parent_reloid) };
#[doc(hidden)]
pub struct Field_as_nplans;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AppendState>() == 344);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AppendState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AppendState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendState, as_nplans) == 208);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_as_nplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendState> as c::expression::OrdinaryField<Field_as_nplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).as_nplans) };
#[doc(hidden)]
pub struct Field_as_whichplan;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendState, as_whichplan) == 212);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_as_whichplan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 212;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendState> as c::expression::OrdinaryField<Field_as_whichplan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).as_whichplan) };
#[doc(hidden)]
pub struct Field_as_begun;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendState, as_begun) == 216);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_as_begun>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendState> as c::expression::OrdinaryField<Field_as_begun>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).as_begun) };
#[doc(hidden)]
pub struct Field_as_nasyncplans;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendState, as_nasyncplans) == 232);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_as_nasyncplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendState> as c::expression::OrdinaryField<Field_as_nasyncplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).as_nasyncplans) };
#[doc(hidden)]
pub struct Field_as_nasyncresults;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendState, as_nasyncresults) == 256
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_as_nasyncresults>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendState> as c::expression::OrdinaryField<Field_as_nasyncresults>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).as_nasyncresults) };
#[doc(hidden)]
pub struct Field_as_syncdone;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendState, as_syncdone) == 260);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_as_syncdone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 260;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendState> as c::expression::OrdinaryField<Field_as_syncdone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).as_syncdone) };
#[doc(hidden)]
pub struct Field_as_nasyncremain;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendState, as_nasyncremain) == 264);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_as_nasyncremain>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendState> as c::expression::OrdinaryField<Field_as_nasyncremain>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).as_nasyncremain) };
#[doc(hidden)]
pub struct Field_as_first_partial_plan;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendState, as_first_partial_plan) == 288
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_as_first_partial_plan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendState> as c::expression::OrdinaryField<Field_as_first_partial_plan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).as_first_partial_plan) };
#[doc(hidden)]
pub struct Field_pstate_len;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendState, pstate_len) == 304);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pstate_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendState> as c::expression::OrdinaryField<Field_pstate_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pstate_len) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ArrayBuildState>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ArrayBuildState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ArrayBuildState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildState, alen) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_alen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState> as c::expression::OrdinaryField<Field_alen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).alen) };
#[doc(hidden)]
pub struct Field_nelems;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildState, nelems) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nelems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState> as c::expression::OrdinaryField<Field_nelems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nelems) };
#[doc(hidden)]
pub struct Field_element_type;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildState, element_type) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_element_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState> as c::expression::OrdinaryField<Field_element_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).element_type) };
#[doc(hidden)]
pub struct Field_typlen;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildState, typlen) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState> as c::expression::OrdinaryField<Field_typlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typlen) };
#[doc(hidden)]
pub struct Field_typbyval;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildState, typbyval) == 38);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 38;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState> as c::expression::OrdinaryField<Field_typbyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typbyval) };
#[doc(hidden)]
pub struct Field_typalign;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildState, typalign) == 39);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typalign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 39;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState> as c::expression::OrdinaryField<Field_typalign>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typalign) };
#[doc(hidden)]
pub struct Field_private_cxt;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildState, private_cxt) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_private_cxt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildState> as c::expression::OrdinaryField<Field_private_cxt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).private_cxt) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ArrayBuildStateArr>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ArrayBuildStateArr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ArrayBuildStateArr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildStateArr, data) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildStateArr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[doc(hidden)]
pub struct Field_abytes;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildStateArr, abytes) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_abytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildStateArr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr> as c::expression::OrdinaryField<Field_abytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).abytes) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildStateArr, nbytes) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildStateArr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr> as c::expression::OrdinaryField<Field_nbytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbytes) };
#[doc(hidden)]
pub struct Field_aitems;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildStateArr, aitems) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aitems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildStateArr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr> as c::expression::OrdinaryField<Field_aitems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aitems) };
#[doc(hidden)]
pub struct Field_nitems;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildStateArr, nitems) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nitems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildStateArr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr> as c::expression::OrdinaryField<Field_nitems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nitems) };
#[doc(hidden)]
pub struct Field_ndims;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildStateArr, ndims) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndims>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildStateArr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr> as c::expression::OrdinaryField<Field_ndims>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndims) };
#[doc(hidden)]
pub struct Field_array_type;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildStateArr, array_type) == 92
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_array_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildStateArr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr> as c::expression::OrdinaryField<Field_array_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).array_type) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildStateArr, element_type) == 96
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_element_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildStateArr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr> as c::expression::OrdinaryField<Field_element_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).element_type) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayBuildStateArr, private_cxt) == 100
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_private_cxt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayBuildStateArr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayBuildStateArr> as c::expression::OrdinaryField<Field_private_cxt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).private_cxt) };
#[doc(hidden)]
pub struct Field_resulttype;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ArrayCoerceExpr>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ArrayCoerceExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ArrayCoerceExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayCoerceExpr, resulttype) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayCoerceExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayCoerceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayCoerceExpr> as c::expression::OrdinaryField<Field_resulttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttype) };
#[doc(hidden)]
pub struct Field_resulttypmod;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayCoerceExpr, resulttypmod) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayCoerceExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayCoerceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayCoerceExpr> as c::expression::OrdinaryField<Field_resulttypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttypmod) };
#[doc(hidden)]
pub struct Field_resultcollid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayCoerceExpr, resultcollid) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayCoerceExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayCoerceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayCoerceExpr> as c::expression::OrdinaryField<Field_resultcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultcollid) };
#[doc(hidden)]
pub struct Field_coerceformat;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayCoerceExpr, coerceformat) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coerceformat>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayCoerceExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CoercionForm,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayCoerceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayCoerceExpr> as c::expression::OrdinaryField<Field_coerceformat>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coerceformat) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayCoerceExpr, location) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayCoerceExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayCoerceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayCoerceExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_array_typeid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ArrayExpr>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ArrayExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ArrayExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayExpr, array_typeid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_array_typeid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr> as c::expression::OrdinaryField<
        Field_array_typeid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).array_typeid) };
#[doc(hidden)]
pub struct Field_array_collid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayExpr, array_collid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_array_collid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr> as c::expression::OrdinaryField<
        Field_array_collid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).array_collid) };
#[doc(hidden)]
pub struct Field_element_typeid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayExpr, element_typeid) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_element_typeid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr> as c::expression::OrdinaryField<
        Field_element_typeid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).element_typeid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayExpr, elements) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elements>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr> as c::expression::OrdinaryField<
        Field_elements,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elements) };
#[doc(hidden)]
pub struct Field_multidims;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayExpr, multidims) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multidims>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr> as c::expression::OrdinaryField<
        Field_multidims,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).multidims) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayExpr, location) == 28);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ArrayExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayExpr> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ArrayMetaState>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ArrayMetaState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ArrayMetaState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayMetaState, element_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_element_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayMetaState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState> as c::expression::OrdinaryField<Field_element_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).element_type) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayMetaState, typlen) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayMetaState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState> as c::expression::OrdinaryField<Field_typlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typlen) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayMetaState, typbyval) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayMetaState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState> as c::expression::OrdinaryField<Field_typbyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typbyval) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayMetaState, typalign) == 7);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typalign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 7;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayMetaState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState> as c::expression::OrdinaryField<Field_typalign>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typalign) };
#[doc(hidden)]
pub struct Field_typdelim;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayMetaState, typdelim) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typdelim>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayMetaState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState> as c::expression::OrdinaryField<Field_typdelim>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typdelim) };
#[doc(hidden)]
pub struct Field_typioparam;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayMetaState, typioparam) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typioparam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayMetaState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState> as c::expression::OrdinaryField<Field_typioparam>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typioparam) };
#[doc(hidden)]
pub struct Field_typiofunc;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ArrayMetaState, typiofunc) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typiofunc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ArrayMetaState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ArrayMetaState> as c::expression::OrdinaryField<Field_typiofunc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typiofunc) };
#[doc(hidden)]
pub struct Field_request_index;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AsyncRequest>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AsyncRequest>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AsyncRequest {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AsyncRequest, request_index) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_request_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AsyncRequest>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AsyncRequest, <<c::expression::CRecord<crate::__pgrx_c_bindings::AsyncRequest> as c::expression::OrdinaryField<Field_request_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).request_index) };
#[doc(hidden)]
pub struct Field_callback_pending;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AsyncRequest, callback_pending) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_callback_pending>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AsyncRequest>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AsyncRequest, <<c::expression::CRecord<crate::__pgrx_c_bindings::AsyncRequest> as c::expression::OrdinaryField<Field_callback_pending>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).callback_pending) };
#[doc(hidden)]
pub struct Field_request_complete;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AsyncRequest, request_complete) == 21
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_request_complete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AsyncRequest>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AsyncRequest, <<c::expression::CRecord<crate::__pgrx_c_bindings::AsyncRequest> as c::expression::OrdinaryField<Field_request_complete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).request_complete) };
#[doc(hidden)]
pub struct Field_staop;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AttStatsSlot>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AttStatsSlot>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AttStatsSlot {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AttStatsSlot, staop) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_staop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AttStatsSlot>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AttStatsSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::AttStatsSlot> as c::expression::OrdinaryField<Field_staop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).staop) };
#[doc(hidden)]
pub struct Field_stacoll;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AttStatsSlot, stacoll) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stacoll>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AttStatsSlot>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AttStatsSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::AttStatsSlot> as c::expression::OrdinaryField<Field_stacoll>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stacoll) };
#[doc(hidden)]
pub struct Field_valuetype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AttStatsSlot, valuetype) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_valuetype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AttStatsSlot>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AttStatsSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::AttStatsSlot> as c::expression::OrdinaryField<Field_valuetype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).valuetype) };
#[doc(hidden)]
pub struct Field_nvalues;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AttStatsSlot, nvalues) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nvalues>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AttStatsSlot>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AttStatsSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::AttStatsSlot> as c::expression::OrdinaryField<Field_nvalues>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nvalues) };
#[doc(hidden)]
pub struct Field_nnumbers;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AttStatsSlot, nnumbers) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nnumbers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AttStatsSlot>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AttStatsSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::AttStatsSlot> as c::expression::OrdinaryField<Field_nnumbers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nnumbers) };
#[doc(hidden)]
pub struct Field_adnum;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AttrDefault>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AttrDefault>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AttrDefault {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AttrDefault, adnum) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_adnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AttrDefault>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AttrDefault, <<c::expression::CRecord<crate::__pgrx_c_bindings::AttrDefault> as c::expression::OrdinaryField<Field_adnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).adnum) };
#[doc(hidden)]
pub struct Field_am_present;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AttrMissing>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AttrMissing>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AttrMissing {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AttrMissing, am_present) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_am_present>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AttrMissing>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AttrMissing, <<c::expression::CRecord<crate::__pgrx_c_bindings::AttrMissing> as c::expression::OrdinaryField<Field_am_present>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).am_present) };
#[doc(hidden)]
pub struct Field_am_value;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AttrMissing, am_value) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_am_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AttrMissing>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AttrMissing, <<c::expression::CRecord<crate::__pgrx_c_bindings::AttrMissing> as c::expression::OrdinaryField<Field_am_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).am_value) };
#[doc(hidden)]
pub struct Field_enabled;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AutoVacOpts>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AutoVacOpts>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AutoVacOpts {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, enabled) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_enabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_enabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).enabled) };
#[doc(hidden)]
pub struct Field_vacuum_threshold;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_threshold) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_vacuum_threshold>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vacuum_threshold) };
#[doc(hidden)]
pub struct Field_vacuum_ins_threshold;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_ins_threshold) == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_ins_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_vacuum_ins_threshold>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vacuum_ins_threshold) };
#[doc(hidden)]
pub struct Field_analyze_threshold;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, analyze_threshold) == 12
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_analyze_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_analyze_threshold>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).analyze_threshold) };
#[doc(hidden)]
pub struct Field_vacuum_cost_limit;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_cost_limit) == 16
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_cost_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_vacuum_cost_limit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vacuum_cost_limit) };
#[doc(hidden)]
pub struct Field_freeze_min_age;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_min_age) == 20);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_min_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_freeze_min_age>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeze_min_age) };
#[doc(hidden)]
pub struct Field_freeze_max_age;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_max_age) == 24);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_max_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_freeze_max_age>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeze_max_age) };
#[doc(hidden)]
pub struct Field_freeze_table_age;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_table_age) == 28);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_table_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_freeze_table_age>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeze_table_age) };
#[doc(hidden)]
pub struct Field_multixact_freeze_min_age;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_min_age) == 32
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_min_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_multixact_freeze_min_age>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).multixact_freeze_min_age) };
#[doc(hidden)]
pub struct Field_multixact_freeze_max_age;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_max_age) == 36
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_max_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_multixact_freeze_max_age>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).multixact_freeze_max_age) };
#[doc(hidden)]
pub struct Field_multixact_freeze_table_age;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_table_age)
        == 40
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_table_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_multixact_freeze_table_age>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).multixact_freeze_table_age) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_log_min_duration;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, log_min_duration) == 44);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_log_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_min_duration) };
#[doc(hidden)]
pub struct Field_scan_key;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTArrayKeyInfo>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTArrayKeyInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTArrayKeyInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTArrayKeyInfo, scan_key) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_key>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTArrayKeyInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo> as c::expression::OrdinaryField<Field_scan_key>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scan_key) };
#[doc(hidden)]
pub struct Field_cur_elem;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTArrayKeyInfo, cur_elem) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cur_elem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTArrayKeyInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo> as c::expression::OrdinaryField<Field_cur_elem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cur_elem) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_mark_elem;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTArrayKeyInfo, mark_elem) == 8);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mark_elem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTArrayKeyInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo> as c::expression::OrdinaryField<Field_mark_elem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mark_elem) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTArrayKeyInfo, num_elems) == 12);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_elems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTArrayKeyInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo> as c::expression::OrdinaryField<Field_num_elems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_elems) };
#[doc(hidden)]
pub struct Field_baseoff;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTDedupInterval>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTDedupInterval>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTDedupInterval {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTDedupInterval, baseoff) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_baseoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupInterval>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTDedupInterval, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupInterval> as c::expression::OrdinaryField<Field_baseoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).baseoff) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTDedupInterval, nitems) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nitems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupInterval>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTDedupInterval, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupInterval> as c::expression::OrdinaryField<Field_nitems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nitems) };
#[doc(hidden)]
pub struct Field_deduplicate;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTDedupStateData>() == 1704);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTDedupStateData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTDedupStateData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTDedupStateData, deduplicate) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deduplicate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTDedupStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData> as c::expression::OrdinaryField<Field_deduplicate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deduplicate) };
#[doc(hidden)]
pub struct Field_nmaxitems;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTDedupStateData, nmaxitems) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nmaxitems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTDedupStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData> as c::expression::OrdinaryField<Field_nmaxitems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nmaxitems) };
#[doc(hidden)]
pub struct Field_maxpostingsize;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTDedupStateData, maxpostingsize) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxpostingsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTDedupStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData> as c::expression::OrdinaryField<Field_maxpostingsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxpostingsize) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTDedupStateData, baseoff) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_baseoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTDedupStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData> as c::expression::OrdinaryField<Field_baseoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).baseoff) };
#[doc(hidden)]
pub struct Field_basetupsize;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTDedupStateData, basetupsize) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_basetupsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTDedupStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData> as c::expression::OrdinaryField<Field_basetupsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).basetupsize) };
#[doc(hidden)]
pub struct Field_nhtids;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTDedupStateData, nhtids) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nhtids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTDedupStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData> as c::expression::OrdinaryField<Field_nhtids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nhtids) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTDedupStateData, nitems) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nitems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTDedupStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData> as c::expression::OrdinaryField<Field_nitems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nitems) };
#[doc(hidden)]
pub struct Field_phystupsize;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTDedupStateData, phystupsize) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phystupsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTDedupStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData> as c::expression::OrdinaryField<Field_phystupsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phystupsize) };
#[doc(hidden)]
pub struct Field_nintervals;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTDedupStateData, nintervals) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nintervals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTDedupStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTDedupStateData> as c::expression::OrdinaryField<Field_nintervals>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nintervals) };
#[doc(hidden)]
pub struct Field_itemsz;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTInsertStateData>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTInsertStateData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTInsertStateData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTInsertStateData, itemsz) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_itemsz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTInsertStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData> as c::expression::OrdinaryField<Field_itemsz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).itemsz) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTInsertStateData, buf) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTInsertStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData> as c::expression::OrdinaryField<Field_buf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf) };
#[doc(hidden)]
pub struct Field_bounds_valid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTInsertStateData, bounds_valid) == 28
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bounds_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTInsertStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData> as c::expression::OrdinaryField<Field_bounds_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bounds_valid) };
#[doc(hidden)]
pub struct Field_low;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTInsertStateData, low) == 30);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_low>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTInsertStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData> as c::expression::OrdinaryField<Field_low>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).low) };
#[doc(hidden)]
pub struct Field_stricthigh;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTInsertStateData, stricthigh) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stricthigh>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTInsertStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData> as c::expression::OrdinaryField<Field_stricthigh>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stricthigh) };
#[doc(hidden)]
pub struct Field_postingoff;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTInsertStateData, postingoff) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_postingoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTInsertStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTInsertStateData> as c::expression::OrdinaryField<Field_postingoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).postingoff) };
#[doc(hidden)]
pub struct Field_btm_magic;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTMetaPageData>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTMetaPageData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTMetaPageData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTMetaPageData, btm_magic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btm_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData> as c::expression::OrdinaryField<Field_btm_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btm_magic) };
#[doc(hidden)]
pub struct Field_btm_version;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTMetaPageData, btm_version) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btm_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData> as c::expression::OrdinaryField<Field_btm_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btm_version) };
#[doc(hidden)]
pub struct Field_btm_root;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTMetaPageData, btm_root) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btm_root>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData> as c::expression::OrdinaryField<Field_btm_root>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btm_root) };
#[doc(hidden)]
pub struct Field_btm_level;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTMetaPageData, btm_level) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btm_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData> as c::expression::OrdinaryField<Field_btm_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btm_level) };
#[doc(hidden)]
pub struct Field_btm_fastroot;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTMetaPageData, btm_fastroot) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btm_fastroot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData> as c::expression::OrdinaryField<Field_btm_fastroot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btm_fastroot) };
#[doc(hidden)]
pub struct Field_btm_fastlevel;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTMetaPageData, btm_fastlevel) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btm_fastlevel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData> as c::expression::OrdinaryField<Field_btm_fastlevel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btm_fastlevel) };
#[doc(hidden)]
pub struct Field_btm_last_cleanup_num_delpages;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::BTMetaPageData,
        btm_last_cleanup_num_delpages
    ) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btm_last_cleanup_num_delpages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData> as c::expression::OrdinaryField<Field_btm_last_cleanup_num_delpages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btm_last_cleanup_num_delpages) };
#[doc(hidden)]
pub struct Field_btm_allequalimage;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTMetaPageData, btm_allequalimage) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btm_allequalimage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTMetaPageData> as c::expression::OrdinaryField<Field_btm_allequalimage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btm_allequalimage) };
#[doc(hidden)]
pub struct Field_varlena_header_;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTOptions>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTOptions>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTOptions {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTOptions, varlena_header_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varlena_header_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::BTOptions,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BTOptions> as c::expression::OrdinaryField<
        Field_varlena_header_,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varlena_header_) };
#[doc(hidden)]
pub struct Field_fillfactor;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTOptions, fillfactor) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fillfactor>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::BTOptions,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::BTOptions> as c::expression::OrdinaryField<
        Field_fillfactor,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fillfactor) };
#[doc(hidden)]
pub struct Field_deduplicate_items;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTOptions, deduplicate_items) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deduplicate_items>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
