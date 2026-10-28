// Shared generated C macro support.

#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_analyze_time;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, last_analyze_time)
        == 152
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_analyze_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_last_analyze_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_analyze_time) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, analyze_count) == 160
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_analyze_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_autoanalyze_time;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, last_autoanalyze_time)
        == 168
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_autoanalyze_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_last_autoanalyze_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_autoanalyze_time) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_autoanalyze_count;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, autoanalyze_count)
        == 176
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_autoanalyze_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_autoanalyze_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).autoanalyze_count) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_TableCounts>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_TableCounts>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, numscans) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numscans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_numscans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numscans) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, tuples_returned) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_returned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_tuples_returned>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_returned) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, tuples_fetched) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_fetched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_tuples_fetched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_fetched) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, tuples_inserted) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_inserted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_tuples_inserted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_inserted) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, tuples_updated) == 32
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_updated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_tuples_updated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_updated) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, tuples_deleted) == 40
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_deleted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_tuples_deleted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_deleted) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, tuples_hot_updated) == 48
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_hot_updated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_tuples_hot_updated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_hot_updated) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, tuples_newpage_updated)
        == 56
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_newpage_updated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_tuples_newpage_updated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_newpage_updated) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, truncdropped) == 64
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_truncdropped>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_truncdropped>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).truncdropped) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_delta_live_tuples;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, delta_live_tuples) == 72
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_delta_live_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_delta_live_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).delta_live_tuples) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_delta_dead_tuples;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, delta_dead_tuples) == 80
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_delta_dead_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_delta_dead_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).delta_dead_tuples) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_changed_tuples;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, changed_tuples) == 88
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_changed_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_changed_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).changed_tuples) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, blocks_fetched) == 96
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks_fetched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_blocks_fetched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks_fetched) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, blocks_hit) == 104
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks_hit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_blocks_hit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks_hit) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_TableStatus>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_TableStatus>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableStatus, id) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableStatus, shared) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus> as c::expression::OrdinaryField<Field_shared>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).shared) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_counts;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableStatus, counts) == 16);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_counts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus> as c::expression::OrdinaryField<Field_counts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).counts) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlaceHolderVar>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlaceHolderVar>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlaceHolderVar, phid) == 32);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderVar>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlaceHolderVar, phlevelsup) == 36);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phlevelsup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderVar>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_ExecProcNode;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, ExecProcNode) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::PlanState,
                )
                    -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::PlanState,
                )
                    -> *mut crate::__pgrx_c_bindings::TupleTableSlot,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ExecProcNode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__PlanState__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_ExecProcNode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ExecProcNode) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_chgParam;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, chgParam) == 104);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_chgParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_chgParam,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).chgParam) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_permInfos;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, permInfos) == 48);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_permInfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_permInfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).permInfos) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, resultRelations) == 56);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, appendRelations) == 64);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, subplans) == 72);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, rowMarks) == 88);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, relationOids) == 96);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, invalItems) == 104);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, paramExecTypes) == 112);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramExecTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, stmt_location) == 128);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, stmt_len) == 132);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_finalrteperminfos;
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrteperminfos) == 48
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_finalrteperminfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).finalrteperminfos) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrowmarks) == 56);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrowmarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, resultRelations) == 64
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, appendRelations) == 72
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, relationOids) == 80);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, invalItems) == 88);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, paramExecTypes) == 96);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramExecTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPHId) == 104);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPHId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastRowMarkId) == 108);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastRowMarkId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPlanNodeId) == 112
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPlanNodeId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, transientPlan) == 116);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transientPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, dependsOnRole) == 117);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 117;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeOK) == 118
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeOK>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 118;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeNeeded) == 119
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeNeeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 119;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, maxParallelHazard) == 120
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxParallelHazard>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 688);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, join_rel_list) == 112);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_join_rel_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, join_cur_level) == 136);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_join_cur_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, init_plans) == 144);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_init_plans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, cte_plan_ids) == 152);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cte_plan_ids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, multiexpr_params) == 160
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multiexpr_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_join_domains;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, join_domains) == 168);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_join_domains>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_join_domains>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).join_domains) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, eq_classes) == 176);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eq_classes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, ec_merging_done) == 184);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_merging_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, canon_pathkeys) == 192);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canon_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, left_join_clauses) == 200
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_left_join_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, right_join_clauses) == 208
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_right_join_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, full_join_clauses) == 216
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_full_join_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, join_info_list) == 224);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_join_info_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_rinfo_serial;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, last_rinfo_serial) == 232
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_rinfo_serial>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_last_rinfo_serial>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_rinfo_serial) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, append_rel_list) == 256);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_append_rel_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, row_identity_vars) == 264
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_row_identity_vars>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, rowMarks) == 272);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, placeholder_list) == 280
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_placeholder_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_placeholder_array_size;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, placeholder_array_size) == 296
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_placeholder_array_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_placeholder_array_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).placeholder_array_size) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, fkey_list) == 304);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fkey_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, query_pathkeys) == 312);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_query_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, group_pathkeys) == 320);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_group_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_num_groupby_pathkeys;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, num_groupby_pathkeys) == 328
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_groupby_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_num_groupby_pathkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_groupby_pathkeys) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, window_pathkeys) == 336);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_window_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, distinct_pathkeys) == 344
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinct_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, sort_pathkeys) == 352);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sort_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, part_schemes) == 360);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_schemes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, initial_rels) == 368);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initial_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 368;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_processed_groupClause;
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, processed_groupClause) == 504
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processed_groupClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 504;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_processed_groupClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).processed_groupClause) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_processed_distinctClause;
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, processed_distinctClause) == 512
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processed_distinctClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 512;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_processed_distinctClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).processed_distinctClause) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, processed_tlist) == 520);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processed_tlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 520;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, update_colnos) == 528);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_update_colnos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, minmax_aggs) == 544);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minmax_aggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 544;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, qual_security_level) == 584
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_qual_security_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 584;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasJoinRTEs) == 588);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasJoinRTEs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 588;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasLateralRTEs) == 589);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasLateralRTEs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 589;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasHavingQual) == 590);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasHavingQual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 590;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasPseudoConstantQuals) == 591
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasPseudoConstantQuals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 591;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasAlternativeSubPlans) == 592
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasAlternativeSubPlans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 592;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_placeholdersFrozen;
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, placeholdersFrozen) == 593
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_placeholdersFrozen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 593;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_placeholdersFrozen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).placeholdersFrozen) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasRecursion) == 594);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasRecursion>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 594;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, agginfos) == 600);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agginfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 600;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, aggtransinfos) == 608);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtransinfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 608;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, numOrderedAggs) == 616);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numOrderedAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 616;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasNonPartialAggs) == 620
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasNonPartialAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 620;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasNonSerialAggs) == 621
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasNonSerialAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 621;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, wt_param_id) == 624);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wt_param_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 624;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, curOuterParams) == 648);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curOuterParams>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 648;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, partColsUpdated) == 680);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partColsUpdated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 680;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Query>() == 256);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Query>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, rteperminfos) == 72);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_rteperminfos,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rteperminfos) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, mergeActionList) == 88);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeActionList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, mergeUseOuterJoin) == 96);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeUseOuterJoin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, targetList) == 104);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, override_) == 112);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_override>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OverridingKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, returningList) == 128);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupClause) == 136);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupDistinct) == 144);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupDistinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupingSets) == 152);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupingSets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, windowClause) == 168);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_windowClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, distinctClause) == 176);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinctClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, sortClause) == 184);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, limitOption) == 208);
#[cfg(feature = "pg16")]
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
    const OFFSET: usize = 208;
}
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, rowMarks) == 216);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, constraintDeps) == 232);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constraintDeps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, withCheckOptions) == 240);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_withCheckOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, stmt_location) == 248);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, stmt_len) == 252);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 252;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RTEPermissionInfo>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RTEPermissionInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RTEPermissionInfo {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RTEPermissionInfo, type_) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RTEPermissionInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RTEPermissionInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RTEPermissionInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RTEPermissionInfo, relid) == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RTEPermissionInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RTEPermissionInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RTEPermissionInfo> as c::expression::OrdinaryField<Field_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RTEPermissionInfo, inh) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inh>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RTEPermissionInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RTEPermissionInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RTEPermissionInfo> as c::expression::OrdinaryField<Field_inh>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inh) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RTEPermissionInfo, requiredPerms) == 16
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_requiredPerms>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RTEPermissionInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RTEPermissionInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RTEPermissionInfo> as c::expression::OrdinaryField<Field_requiredPerms>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).requiredPerms) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RTEPermissionInfo, checkAsUser) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_checkAsUser>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RTEPermissionInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RTEPermissionInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RTEPermissionInfo> as c::expression::OrdinaryField<Field_checkAsUser>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).checkAsUser) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeTblEntry>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeTblEntry>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_perminfoindex;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, perminfoindex) == 32);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_perminfoindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_perminfoindex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).perminfoindex) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, security_barrier) == 48
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_security_barrier>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, jointype) == 52);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jointype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JoinType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, joinmergedcols) == 56);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joinmergedcols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, joinaliasvars) == 64);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joinaliasvars>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, joinleftcols) == 72);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joinleftcols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, joinrightcols) == 80);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joinrightcols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, functions) == 96);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_functions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, funcordinality) == 104
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcordinality>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, values_lists) == 120);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_values_lists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, ctelevelsup) == 136);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctelevelsup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, self_reference) == 140
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_self_reference>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, coltypes) == 144);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coltypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, coltypmods) == 152);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coltypmods>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, colcollations) == 160);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colcollations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, lateral) == 200);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lateral>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, inh) == 201);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inh>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 201;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, inFromCl) == 202);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inFromCl>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 202;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, securityQuals) == 208);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_securityQuals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_locator;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelFileLocatorBackend>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelFileLocatorBackend>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RelFileLocatorBackend {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelFileLocatorBackend, locator) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_locator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocatorBackend>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocator>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelFileLocatorBackend, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocatorBackend> as c::expression::OrdinaryField<Field_locator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).locator) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelFileLocatorBackend, backend) == 12
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocatorBackend>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelFileLocatorBackend, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocatorBackend> as c::expression::OrdinaryField<Field_backend>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend) };
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelOptInfo>() == 440);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelOptInfo>() == 8);
};
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, lateral_vars) == 152);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lateral_vars>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, indexlist) == 168);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, statlist) == 176);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_statlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, pages) == 184);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, subplan_params) == 224);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subplan_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, rel_parallel_workers) == 232
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rel_parallel_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, amflags) == 236);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, serverid) == 240);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_serverid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, userid) == 244);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_userid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 244;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, useridiscurrent) == 248);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_useridiscurrent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, unique_for_rels) == 272);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique_for_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, non_unique_for_rels) == 280
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_non_unique_for_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, baserestrictinfo) == 288);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_baserestrictinfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, baserestrict_min_security) == 312
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_baserestrict_min_security>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, joininfo) == 320);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joininfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, has_eclass_joins) == 328);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_eclass_joins>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, consider_partitionwise_join)
        == 329
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_consider_partitionwise_join>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 329;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, part_scheme) == 360);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_scheme>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSchemeData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, nparts) == 368);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nparts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 368;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, boundinfo) == 376);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boundinfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 376;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, partbounds_merged) == 384
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partbounds_merged>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 384;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, partition_qual) == 392);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partition_qual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 392;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, part_rels) == 400);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>,
            c::expression::ReadWrite,
        >,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 400;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, partexprs) == 424);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::List>,
            c::expression::ReadWrite,
        >,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 424;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, nullable_partexprs) == 432
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nullable_partexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::List>,
            c::expression::ReadWrite,
        >,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelationData>() == 480);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelationData>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rd_newRelfilelocatorSubid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_newRelfilelocatorSubid)
        == 44
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_newRelfilelocatorSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_newRelfilelocatorSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_newRelfilelocatorSubid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rd_firstRelfilelocatorSubid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_firstRelfilelocatorSubid)
        == 48
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_firstRelfilelocatorSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_firstRelfilelocatorSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_firstRelfilelocatorSubid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rd_attrsvalid;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_attrsvalid) == 240);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_attrsvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_attrsvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_attrsvalid) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_options) == 296);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::varlena>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_amhandler) == 304);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_amhandler>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_tableam) == 312);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_tableam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>,
        c::expression::ReadOnly,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_index) == 320);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_index>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_indexprs) == 392);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_indexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 392;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_indpred) == 400);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_indpred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 400;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_toastoid) == 464);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_toastoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 464;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, pgstat_enabled) == 468);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgstat_enabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 468;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, pgstat_info) == 472);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgstat_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 472;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBuffer>() == 368);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBuffer>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, by_txn_last_xid) == 64
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_by_txn_last_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, output_rewrites) == 232
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_output_rewrites>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, context) == 240);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, current_restart_decoding_lsn)
        == 272
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_restart_decoding_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, outbufsize) == 288);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outbufsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, size) == 296);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, spillTxns) == 304);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spillTxns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, spillCount) == 312);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spillCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, spillBytes) == 320);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spillBytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, streamTxns) == 328);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streamTxns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, streamCount) == 336);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streamCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, streamBytes) == 344);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streamBytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, totalTxns) == 352);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalTxns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, totalBytes) == 360);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalBytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBufferTXN>() == 320);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBufferTXN>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_toptxn;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, toptxn) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ReorderBufferTXN>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ReorderBufferTXN>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toptxn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_toptxn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).toptxn) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, size) == 272);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, total_size) == 280);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, concurrent_abort) == 288
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_concurrent_abort>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, ninvalidations_distributed)
        == 304
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ninvalidations_distributed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_invalidated;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlotPersistentData, invalidated)
        == 88
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalidated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ReplicationSlotInvalidationCause,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlotPersistentData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData> as c::expression::OrdinaryField<Field_invalidated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).invalidated) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RestrictInfo>() == 240);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RestrictInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, can_join) == 17);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_can_join>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, pseudoconstant) == 18);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pseudoconstant>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_has_clone;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, has_clone) == 19);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_clone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_has_clone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_clone) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_is_clone;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, is_clone) == 20);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_clone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_is_clone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_clone) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, leakproof) == 21);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_leakproof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_num_base_rels;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, num_base_rels) == 32);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_base_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_num_base_rels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_base_rels) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, required_relids) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_required_relids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rinfo_serial;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, rinfo_serial) == 96);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rinfo_serial>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_rinfo_serial>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rinfo_serial) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, mergeopfamilies) == 144
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeopfamilies>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, scansel_cache) == 184);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scansel_cache>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, outer_is_left) == 192);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outer_is_left>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, hashjoinoperator) == 196
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashjoinoperator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 196;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, left_hasheqoperator) == 232
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_left_hasheqoperator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, right_hasheqoperator) == 236
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_right_hasheqoperator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ResultRelInfo>() == 360);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ResultRelInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_projectNewInfoValid) == 80
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_projectNewInfoValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_needLockTagTuple) == 81
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_needLockTagTuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 81;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_usesFdwDirectModify) == 160
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_usesFdwDirectModify>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumSlots) == 164);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumSlots>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 164;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumSlotsInitialized) == 168
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumSlotsInitialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_BatchSize) == 172);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_BatchSize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_WithCheckOptions) == 192
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_WithCheckOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_WithCheckOptionExprs)
        == 200
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_WithCheckOptionExprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ri_NumGeneratedNeededI;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumGeneratedNeededI) == 232
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumGeneratedNeededI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_NumGeneratedNeededI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_NumGeneratedNeededI) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ri_NumGeneratedNeededU;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumGeneratedNeededU) == 236
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumGeneratedNeededU>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_NumGeneratedNeededU>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_NumGeneratedNeededU) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_returningList) == 240
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_onConflictArbiterIndexes)
        == 256
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_onConflictArbiterIndexes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_matchedMergeAction) == 272
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_matchedMergeAction>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_notMatchedMergeAction)
        == 280
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_notMatchedMergeAction>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_ChildToRootMapValid) == 304
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_ChildToRootMapValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ri_RootToChildMapValid;
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_RootToChildMapValid) == 320
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_RootToChildMapValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_RootToChildMapValid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_RootToChildMapValid) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_ancestorResultRels) == 352
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_ancestorResultRels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SERIALIZABLEXACT>() == 168);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SERIALIZABLEXACT>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, topXid) == 144);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_topXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, finishedBefore) == 148
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finishedBefore>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 148;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, xmin) == 152);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, flags) == 156);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, pid) == 160);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, pgprocno) == 164);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgprocno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 164;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_smgr_rlocator;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SMgrRelationData, smgr_rlocator) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_smgr_rlocator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocatorBackend>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SMgrRelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData> as c::expression::OrdinaryField<Field_smgr_rlocator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).smgr_rlocator) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_sharedsort;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortCoordinateData, sharedsort) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Sharedsort>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Sharedsort>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sharedsort>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortCoordinateData>
{
    type Member = c::expression::CPointer<
        c::expression::COpaque<crate::__pgrx_c_bindings::Sharedsort>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortCoordinateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortCoordinateData> as c::expression::OrdinaryField<Field_sharedsort>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sharedsort) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_attLeafType;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistCache, attLeafType) == 28);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attLeafType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistCache>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistCache, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistCache> as c::expression::OrdinaryField<Field_attLeafType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attLeafType) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistState, attLeafType) == 36);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attLeafType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState> as c::expression::OrdinaryField<Field_attLeafType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attLeafType) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpecialJoinInfo>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpecialJoinInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ojrelid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, ojrelid) == 44);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ojrelid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpecialJoinInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo> as c::expression::OrdinaryField<Field_ojrelid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ojrelid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, lhs_strict) == 80);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lhs_strict>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_can_btree) == 81
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_can_btree>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 81;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_can_hash) == 82
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_can_hash>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 82;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_operators) == 88
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_operators>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_rhs_exprs) == 96
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_rhs_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Subscription>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Subscription>() == 8);
};
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stream>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_passwordrequired;
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, passwordrequired) == 33
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_passwordrequired>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_passwordrequired>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).passwordrequired) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_runasowner;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, runasowner) == 34);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runasowner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_runasowner>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).runasowner) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause>()
            == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause>()
            == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause, type_)
        == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause,
        frameOptions
    ) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frameOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause> as c::expression::OrdinaryField<Field_frameOptions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frameOptions) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_firstblocknumber;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TapeShare>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TapeShare>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TapeShare {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TapeShare, firstblocknumber) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstblocknumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TapeShare>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TapeShare,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TapeShare> as c::expression::OrdinaryField<
        Field_firstblocknumber,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstblocknumber) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Var>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Var>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varlevelsup) == 32);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varlevelsup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varnosyn) == 36);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varattnosyn) == 40);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varattnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, location) == 44);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1__bindgen_ty_2,
        >() == 48
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1__bindgen_ty_2,
        >() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1__bindgen_ty_2,
        twophase
    ) == 32
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_twophase>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1__bindgen_ty_2,
    >
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    bool,
                    bool,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                )
                    -> *mut crate::__pgrx_c_bindings::WalReceiverConn,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *const ::core::ffi::c_char,
                    bool,
                    bool,
                    *const ::core::ffi::c_char,
                    *mut *mut ::core::ffi::c_char,
                )
                    -> *mut crate::__pgrx_c_bindings::WalReceiverConn,
            >,
        >() == 8
    );
};
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_connect>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_check_conninfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0c__0x5f426f6f6c__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalSnd>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalSnd>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalSnd, kind) == 96);
#[cfg(any(feature = "pg16", feature = "pg17"))]
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
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WalSnd,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WalSnd> as c::expression::OrdinaryField<
        Field_kind,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XmlExpr>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XmlExpr>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, indent) == 44);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::XmlExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr> as c::expression::OrdinaryField<
        Field_indent,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indent) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, type_) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, typmod) == 52);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, location) == 56);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlSerialize, indent) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XmlSerialize, <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize> as c::expression::OrdinaryField<Field_indent>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indent) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlSerialize, location) == 28);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::catcache>() == 464);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::catcache>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_cc_nlist;
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_nlist) == 448);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_nlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 448;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catcache,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catcache> as c::expression::OrdinaryField<
        Field_cc_nlist,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cc_nlist) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_cc_nlbuckets;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_nlbuckets) == 452);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_nlbuckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 452;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catcache,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catcache> as c::expression::OrdinaryField<
        Field_cc_nlbuckets,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cc_nlbuckets) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_bool>() == 200);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_bool>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_bool, boot_val) == 152);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_bool>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_bool, reset_val) == 184);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_bool>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_enum>() == 208);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_enum>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum, boot_val) == 152);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum, reset_val) == 192);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_generic>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_generic>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, sourceline) == 136);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sourceline>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_int>() == 208);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_int>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, boot_val) == 152);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, min) == 156);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_min>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, max) == 160);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, reset_val) == 192);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_snapshotConflictHorizon;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogDelete, snapshotConflictHorizon) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogDelete, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_isCatalogRel;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogDelete, isCatalogRel) == 6);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogDelete, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::gistxlogPageReuse>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::gistxlogPageReuse>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPageReuse, isCatalogRel) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageReuse>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPageReuse, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageReuse> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::spgxlogVacuumRedirect>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::spgxlogVacuumRedirect>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::spgxlogVacuumRedirect,
        snapshotConflictHorizon
    ) == 4
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::spgxlogVacuumRedirect>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::spgxlogVacuumRedirect, <<c::expression::CRecord<crate::__pgrx_c_bindings::spgxlogVacuumRedirect> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::spgxlogVacuumRedirect, isCatalogRel) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::spgxlogVacuumRedirect>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::spgxlogVacuumRedirect, <<c::expression::CRecord<crate::__pgrx_c_bindings::spgxlogVacuumRedirect> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_btree_delete>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_btree_delete>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_btree_delete, snapshotConflictHorizon)
        == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_delete>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_btree_delete, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_delete> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_btree_delete, isCatalogRel) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_delete>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_btree_delete, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_delete> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_btree_reuse_page>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_btree_reuse_page>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_btree_reuse_page, isCatalogRel) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_reuse_page>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_btree_reuse_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_reuse_page> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::xl_hash_vacuum_one_page,
        snapshotConflictHorizon
    ) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_hash_vacuum_one_page, isCatalogRel) == 6
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_page, snapshotConflictHorizon)
        == 0
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_page, nplans) == 4);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page> as c::expression::OrdinaryField<Field_nplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nplans) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_page, isCatalogRel) == 6
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_heap_freeze_plan>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_heap_freeze_plan>() == 4);
};
#[cfg(feature = "pg16")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::xl_heap_freeze_plan {}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_plan, xmax) == 0);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan> as c::expression::OrdinaryField<Field_xmax>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmax) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_plan, t_infomask2) == 4
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan> as c::expression::OrdinaryField<Field_t_infomask2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask2) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_plan, t_infomask) == 6
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan> as c::expression::OrdinaryField<Field_t_infomask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_plan, frzflags) == 8);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frzflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan> as c::expression::OrdinaryField<Field_frzflags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frzflags) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_plan, ntuples) == 10);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan> as c::expression::OrdinaryField<Field_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_lock, xmax) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_lock>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_lock, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_lock> as c::expression::OrdinaryField<Field_xmax>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmax) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_infobits_set>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_lock>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_heap_prune>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_heap_prune>() == 4);
};
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_prune, snapshotConflictHorizon) == 0
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_prune, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_prune, isCatalogRel) == 8);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_prune, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_visible, snapshotConflictHorizon)
        == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_visible>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_visible, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_visible> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_xact_relfilelocators>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_xact_relfilelocators>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::xl_xact_relfilelocators {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_xact_relfilelocators, nrels) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nrels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_relfilelocators>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_xact_relfilelocators, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_relfilelocators> as c::expression::OrdinaryField<Field_nrels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nrels) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGAlignedXLogBlock>() == 8192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGAlignedXLogBlock>() == 4096);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGIOAlignedBlock>() == 8192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGIOAlignedBlock>() == 4096);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PGIOAlignedBlock {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGIOAlignedBlock, data) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGIOAlignedBlock>
{
    type Member =
        c::expression::CArray<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, 8192>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGIOAlignedBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGIOAlignedBlock> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGIOAlignedBlock, force_align_i64) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_align_i64>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGIOAlignedBlock>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGIOAlignedBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGIOAlignedBlock> as c::expression::OrdinaryField<Field_force_align_i64>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_align_i64) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN__bindgen_ty_1, abort_time)
        == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_abort_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN__bindgen_ty_1>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN__bindgen_ty_1> as c::expression::OrdinaryField<Field_abort_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).abort_time) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1>() == 48
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1>() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_string>() == 200);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_string>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_real>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_real>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_FunctionCallUsage>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_FunctionCallUsage>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BktypeIO>() == 1024);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BktypeIO>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_BktypeIO {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MemoryContextMethods>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MemoryContextMethods>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::LogicalTape {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::LogicalTape {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockRowsState>() == 312);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockRowsState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_44>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_44>(
        ) == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_44
{
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_38>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_38>(
        ) == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_34>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_34>(
        ) == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DateTimeErrorExtra>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DateTimeErrorExtra>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DateTimeErrorExtra {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::buftag>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::buftag>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator>>()
        == 12
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator>>()
        == 4
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__BufTagGetRelFileLocator"]
    fn raw_inline_BufTagGetRelFileLocator(
        arg0: *const crate::__pgrx_c_bindings::buftag,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `BufTagGetRelFileLocator`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_BufTagGetRelFileLocator(
    arg0: *const crate::__pgrx_c_bindings::buftag,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator> {
    const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::buftag>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_BufTagGetRelFileLocator(arg0)) }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::buftag>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::buftag>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__BufTagSetRelForkDetails"]
    fn raw_inline_BufTagSetRelForkDetails(
        arg0: *mut crate::__pgrx_c_bindings::buftag,
        arg1: crate::__pgrx_c_bindings::Oid,
        arg2: i32,
    ) -> ();
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `BufTagSetRelForkDetails`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_BufTagSetRelForkDetails(
    arg0: *mut crate::__pgrx_c_bindings::buftag,
    arg1: crate::__pgrx_c_bindings::Oid,
    arg2: i32,
) -> () {
    const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::buftag>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Oid>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_BufTagSetRelForkDetails(arg0, arg1, arg2)
        })
    }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__CharGetDatum"]
    fn raw_inline_CharGetDatum(arg0: i8) -> crate::__pgrx_c_bindings::Datum;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `CharGetDatum`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_CharGetDatum(arg0: i8) -> crate::__pgrx_c_bindings::Datum {
    const _: () = assert!(!::core::mem::needs_drop::<i8>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_CharGetDatum(arg0)) }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
