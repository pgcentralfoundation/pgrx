// Shared generated C macro support.

#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatFuncEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatFuncEntry> as c::expression::OrdinaryField<Field_f_total_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).f_total_time) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_f_self_time;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatFuncEntry, f_self_time) == 16
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_f_self_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatFuncEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatFuncEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatFuncEntry> as c::expression::OrdinaryField<Field_f_self_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).f_self_time) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatSubEntry>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatSubEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_StatSubEntry {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatSubEntry, apply_error_count) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_apply_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry> as c::expression::OrdinaryField<Field_apply_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).apply_error_count) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatSubEntry, sync_error_count) == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry> as c::expression::OrdinaryField<Field_sync_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_error_count) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatSubEntry, stat_reset_timestamp)
        == 16
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[doc(hidden)]
pub struct Field_numscans;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatTabEntry>() == 168);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatTabEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_StatTabEntry {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, numscans) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numscans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_numscans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numscans) };
#[doc(hidden)]
pub struct Field_tuples_returned;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, tuples_returned) == 8
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_returned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_tuples_returned>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_returned) };
#[doc(hidden)]
pub struct Field_tuples_fetched;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, tuples_fetched) == 16
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_fetched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_tuples_fetched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_fetched) };
#[doc(hidden)]
pub struct Field_tuples_inserted;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, tuples_inserted) == 24
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_inserted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_tuples_inserted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_inserted) };
#[doc(hidden)]
pub struct Field_tuples_updated;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, tuples_updated) == 32
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_updated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_tuples_updated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_updated) };
#[doc(hidden)]
pub struct Field_tuples_deleted;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, tuples_deleted) == 40
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_deleted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_tuples_deleted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_deleted) };
#[doc(hidden)]
pub struct Field_tuples_hot_updated;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, tuples_hot_updated)
        == 48
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_hot_updated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_tuples_hot_updated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_hot_updated) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_live_tuples;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, n_live_tuples) == 56
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_live_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_n_live_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_live_tuples) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_dead_tuples;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, n_dead_tuples) == 64
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_dead_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_n_dead_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_dead_tuples) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_changes_since_analyze;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, changes_since_analyze)
        == 72
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_changes_since_analyze>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_changes_since_analyze>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).changes_since_analyze) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_inserts_since_vacuum;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, inserts_since_vacuum)
        == 80
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inserts_since_vacuum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_inserts_since_vacuum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inserts_since_vacuum) };
#[doc(hidden)]
pub struct Field_blocks_fetched;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, blocks_fetched) == 88
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks_fetched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_blocks_fetched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks_fetched) };
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, blocks_hit) == 96
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks_hit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_blocks_hit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks_hit) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_vacuum_timestamp;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, vacuum_timestamp) == 104
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_vacuum_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vacuum_timestamp) };
#[doc(hidden)]
pub struct Field_vacuum_count;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, vacuum_count) == 112
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_vacuum_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vacuum_count) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_autovac_vacuum_timestamp;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatTabEntry,
        autovac_vacuum_timestamp
    ) == 120
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_autovac_vacuum_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_autovac_vacuum_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).autovac_vacuum_timestamp) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_autovac_vacuum_count;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, autovac_vacuum_count)
        == 128
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_autovac_vacuum_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_autovac_vacuum_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).autovac_vacuum_count) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_analyze_timestamp;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, analyze_timestamp)
        == 136
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_analyze_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_analyze_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).analyze_timestamp) };
#[doc(hidden)]
pub struct Field_analyze_count;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, analyze_count) == 144
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_analyze_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_analyze_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).analyze_count) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_autovac_analyze_timestamp;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatTabEntry,
        autovac_analyze_timestamp
    ) == 152
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_autovac_analyze_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_autovac_analyze_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).autovac_analyze_timestamp) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_autovac_analyze_count;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, autovac_analyze_count)
        == 160
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_autovac_analyze_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_autovac_analyze_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).autovac_analyze_count) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_numscans;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_TableCounts>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_TableCounts>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_TableCounts {}
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_numscans) == 0);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_numscans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_numscans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_numscans) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_tuples_returned;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_tuples_returned) == 8
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_tuples_returned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_tuples_returned>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_tuples_returned) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_tuples_fetched;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_tuples_fetched) == 16
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_tuples_fetched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_tuples_fetched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_tuples_fetched) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_tuples_inserted;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_tuples_inserted) == 24
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_tuples_inserted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_tuples_inserted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_tuples_inserted) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_tuples_updated;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_tuples_updated) == 32
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_tuples_updated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_tuples_updated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_tuples_updated) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_tuples_deleted;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_tuples_deleted) == 40
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_tuples_deleted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_tuples_deleted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_tuples_deleted) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_tuples_hot_updated;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_tuples_hot_updated)
        == 48
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_tuples_hot_updated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_tuples_hot_updated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_tuples_hot_updated) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_truncdropped;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_truncdropped) == 56
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_truncdropped>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_truncdropped>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_truncdropped) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_delta_live_tuples;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_delta_live_tuples)
        == 64
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_delta_live_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_delta_live_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_delta_live_tuples) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_delta_dead_tuples;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_delta_dead_tuples)
        == 72
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_delta_dead_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_delta_dead_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_delta_dead_tuples) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_changed_tuples;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_changed_tuples) == 80
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_changed_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_changed_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_changed_tuples) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_blocks_fetched;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_blocks_fetched) == 88
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_blocks_fetched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_blocks_fetched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_blocks_fetched) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_blocks_hit;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableCounts, t_blocks_hit) == 96
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_blocks_hit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts> as c::expression::OrdinaryField<Field_t_blocks_hit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_blocks_hit) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_id;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_TableStatus>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_TableStatus>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_TableStatus {}
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableStatus, t_id) == 0);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus> as c::expression::OrdinaryField<Field_t_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_id) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_shared;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableStatus, t_shared) == 4);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_shared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus> as c::expression::OrdinaryField<Field_t_shared>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_shared) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_t_counts;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableStatus, t_counts) == 16);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_counts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableCounts>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus> as c::expression::OrdinaryField<Field_t_counts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_counts) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_TableXactStatus>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_TableXactStatus>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_TableXactStatus {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableXactStatus, tuples_inserted) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_inserted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableXactStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus> as c::expression::OrdinaryField<Field_tuples_inserted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_inserted) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableXactStatus, tuples_updated) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_updated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableXactStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus> as c::expression::OrdinaryField<Field_tuples_updated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_updated) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableXactStatus, tuples_deleted) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples_deleted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableXactStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus> as c::expression::OrdinaryField<Field_tuples_deleted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuples_deleted) };
#[doc(hidden)]
pub struct Field_truncdropped;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableXactStatus, truncdropped) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_truncdropped>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableXactStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus> as c::expression::OrdinaryField<Field_truncdropped>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).truncdropped) };
#[doc(hidden)]
pub struct Field_inserted_pre_truncdrop;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_TableXactStatus,
        inserted_pre_truncdrop
    ) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inserted_pre_truncdrop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableXactStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus> as c::expression::OrdinaryField<Field_inserted_pre_truncdrop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inserted_pre_truncdrop) };
#[doc(hidden)]
pub struct Field_updated_pre_truncdrop;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_TableXactStatus,
        updated_pre_truncdrop
    ) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_updated_pre_truncdrop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableXactStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus> as c::expression::OrdinaryField<Field_updated_pre_truncdrop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).updated_pre_truncdrop) };
#[doc(hidden)]
pub struct Field_deleted_pre_truncdrop;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_TableXactStatus,
        deleted_pre_truncdrop
    ) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deleted_pre_truncdrop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableXactStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus> as c::expression::OrdinaryField<Field_deleted_pre_truncdrop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deleted_pre_truncdrop) };
#[doc(hidden)]
pub struct Field_nest_level;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_TableXactStatus, nest_level) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nest_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_TableXactStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableXactStatus> as c::expression::OrdinaryField<Field_nest_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nest_level) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_WalStats>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_WalStats>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_WalStats {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, wal_records) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_records>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats> as c::expression::OrdinaryField<Field_wal_records>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_records) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, wal_fpi) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_fpi>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats> as c::expression::OrdinaryField<Field_wal_fpi>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_fpi) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, wal_bytes) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats> as c::expression::OrdinaryField<Field_wal_bytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_bytes) };
#[doc(hidden)]
pub struct Field_wal_buffers_full;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, wal_buffers_full) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_buffers_full>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats> as c::expression::OrdinaryField<Field_wal_buffers_full>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_buffers_full) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_wal_write;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, wal_write) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_write>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats> as c::expression::OrdinaryField<Field_wal_write>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_write) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_wal_sync;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, wal_sync) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_sync>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats> as c::expression::OrdinaryField<Field_wal_sync>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_sync) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_wal_write_time;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, wal_write_time) == 48
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_write_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats> as c::expression::OrdinaryField<Field_wal_write_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_write_time) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_wal_sync_time;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, wal_sync_time) == 56
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_sync_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats> as c::expression::OrdinaryField<Field_wal_sync_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_sync_time) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, stat_reset_timestamp) == 64
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlaceHolderInfo>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlaceHolderInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PlaceHolderInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlaceHolderInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderInfo>
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
const _: Projection<crate::__pgrx_c_bindings::PlaceHolderInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_phid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlaceHolderInfo, phid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlaceHolderInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderInfo> as c::expression::OrdinaryField<Field_phid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phid) };
#[doc(hidden)]
pub struct Field_ph_width;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlaceHolderInfo, ph_width) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ph_width>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlaceHolderInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderInfo> as c::expression::OrdinaryField<Field_ph_width>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ph_width) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlaceHolderVar>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlaceHolderVar>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PlaceHolderVar {}
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlaceHolderVar, phid) == 24);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderVar>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlaceHolderVar, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderVar> as c::expression::OrdinaryField<Field_phid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phid) };
#[doc(hidden)]
pub struct Field_phlevelsup;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlaceHolderVar, phlevelsup) == 28);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phlevelsup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderVar>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlaceHolderVar, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlaceHolderVar> as c::expression::OrdinaryField<Field_phlevelsup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phlevelsup) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Plan>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Plan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Plan {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
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
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_plan_width;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, plan_width) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_plan_width>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_plan_width,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).plan_width) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, parallel_aware) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_aware>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_parallel_aware,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_aware) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, parallel_safe) == 37);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_safe>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 37;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_parallel_safe,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_safe) };
#[doc(hidden)]
pub struct Field_async_capable;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, async_capable) == 38);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_async_capable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 38;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_async_capable,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).async_capable) };
#[doc(hidden)]
pub struct Field_plan_node_id;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, plan_node_id) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_plan_node_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_plan_node_id,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).plan_node_id) };
#[doc(hidden)]
pub struct Field_targetlist;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, targetlist) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
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
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_targetlist,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).targetlist) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, qual) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_qual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
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
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_qual,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).qual) };
#[doc(hidden)]
pub struct Field_lefttree;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, lefttree) == 64);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Plan>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Plan>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lefttree>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Plan>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_lefttree,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lefttree) };
#[doc(hidden)]
pub struct Field_righttree;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, righttree) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_righttree>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Plan>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_righttree,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).righttree) };
#[doc(hidden)]
pub struct Field_initPlan;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, initPlan) == 80);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_initPlan,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initPlan) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlanInvalItem>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlanInvalItem>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PlanInvalItem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanInvalItem, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanInvalItem>
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
const _: Projection<crate::__pgrx_c_bindings::PlanInvalItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanInvalItem> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_cacheId;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanInvalItem, cacheId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cacheId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanInvalItem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlanInvalItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanInvalItem> as c::expression::OrdinaryField<Field_cacheId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cacheId) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanInvalItem, hashValue) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashValue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanInvalItem>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlanInvalItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanInvalItem> as c::expression::OrdinaryField<Field_hashValue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashValue) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlanRowMark>() == 36);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlanRowMark>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PlanRowMark {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanRowMark, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark>
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
const _: Projection<crate::__pgrx_c_bindings::PlanRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanRowMark, rti) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlanRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark> as c::expression::OrdinaryField<Field_rti>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rti) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanRowMark, prti) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlanRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark> as c::expression::OrdinaryField<Field_prti>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prti) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanRowMark, rowmarkId) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowmarkId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlanRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark> as c::expression::OrdinaryField<Field_rowmarkId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rowmarkId) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanRowMark, markType) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_markType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__RowMarkType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlanRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark> as c::expression::OrdinaryField<Field_markType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).markType) };
#[doc(hidden)]
pub struct Field_allMarkTypes;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanRowMark, allMarkTypes) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allMarkTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlanRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark> as c::expression::OrdinaryField<Field_allMarkTypes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allMarkTypes) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanRowMark, strength) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlanRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark> as c::expression::OrdinaryField<Field_strength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strength) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanRowMark, waitPolicy) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitPolicy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockWaitPolicy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlanRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark> as c::expression::OrdinaryField<Field_waitPolicy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waitPolicy) };
#[doc(hidden)]
pub struct Field_isParent;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanRowMark, isParent) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isParent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlanRowMark, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanRowMark> as c::expression::OrdinaryField<Field_isParent>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isParent) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlanState>() == 200);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PlanState {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
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
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_instrument;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, instrument) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Instrumentation>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Instrumentation>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_instrument>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_instrument,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).instrument) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, lefttree) == 72);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::PlanState>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::PlanState>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lefttree>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_lefttree,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lefttree) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, righttree) == 80);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_righttree>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_righttree,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).righttree) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, initPlan) == 88);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_initPlan,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initPlan) };
#[doc(hidden)]
pub struct Field_subPlan;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, subPlan) == 96);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_subPlan,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subPlan) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, async_capable) == 144);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_async_capable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_async_capable,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).async_capable) };
#[doc(hidden)]
pub struct Field_scanopsfixed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, scanopsfixed) == 192);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scanopsfixed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_scanopsfixed,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scanopsfixed) };
#[doc(hidden)]
pub struct Field_outeropsfixed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, outeropsfixed) == 193);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outeropsfixed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 193;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_outeropsfixed,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outeropsfixed) };
#[doc(hidden)]
pub struct Field_inneropsfixed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, inneropsfixed) == 194);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inneropsfixed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 194;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_inneropsfixed,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inneropsfixed) };
#[doc(hidden)]
pub struct Field_resultopsfixed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, resultopsfixed) == 195);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultopsfixed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 195;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_resultopsfixed,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultopsfixed) };
#[doc(hidden)]
pub struct Field_scanopsset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, scanopsset) == 196);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scanopsset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 196;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_scanopsset,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scanopsset) };
#[doc(hidden)]
pub struct Field_outeropsset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, outeropsset) == 197);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outeropsset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 197;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_outeropsset,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outeropsset) };
#[doc(hidden)]
pub struct Field_inneropsset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, inneropsset) == 198);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inneropsset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 198;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_inneropsset,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inneropsset) };
#[doc(hidden)]
pub struct Field_resultopsset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlanState, resultopsset) == 199);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultopsset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 199;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PlanState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PlanState> as c::expression::OrdinaryField<
        Field_resultopsset,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultopsset) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PlannedStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
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
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, commandType) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commandType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CmdType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_commandType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commandType) };
#[doc(hidden)]
pub struct Field_queryId;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, queryId) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_queryId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_queryId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).queryId) };
#[doc(hidden)]
pub struct Field_hasReturning;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, hasReturning) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasReturning>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_hasReturning>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasReturning) };
#[doc(hidden)]
pub struct Field_hasModifyingCTE;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, hasModifyingCTE) == 17);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasModifyingCTE>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_hasModifyingCTE>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasModifyingCTE) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, canSetTag) == 18);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canSetTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_canSetTag>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).canSetTag) };
#[doc(hidden)]
pub struct Field_transientPlan;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, transientPlan) == 19);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transientPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_transientPlan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transientPlan) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, dependsOnRole) == 20);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_dependsOnRole>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dependsOnRole) };
#[doc(hidden)]
pub struct Field_parallelModeNeeded;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, parallelModeNeeded) == 21
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeNeeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_parallelModeNeeded>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallelModeNeeded) };
#[doc(hidden)]
pub struct Field_jitFlags;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, jitFlags) == 24);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jitFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_jitFlags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jitFlags) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, rtable) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_rtable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rtable) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, resultRelations) == 48);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_resultRelations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultRelations) };
#[doc(hidden)]
pub struct Field_appendRelations;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, appendRelations) == 56);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_appendRelations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).appendRelations) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, subplans) == 64);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_subplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subplans) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, rowMarks) == 80);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_rowMarks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rowMarks) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, relationOids) == 88);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_relationOids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relationOids) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, invalItems) == 96);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_invalItems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).invalItems) };
#[doc(hidden)]
pub struct Field_paramExecTypes;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, paramExecTypes) == 104);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramExecTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_paramExecTypes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramExecTypes) };
#[doc(hidden)]
pub struct Field_stmt_location;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, stmt_location) == 120);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_stmt_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmt_location) };
#[doc(hidden)]
pub struct Field_stmt_len;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, stmt_len) == 124);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_stmt_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmt_len) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PlannerGlobal {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
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
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, subplans) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_subplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subplans) };
#[doc(hidden)]
pub struct Field_subroots;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, subroots) == 24);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subroots>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_subroots>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subroots) };
#[doc(hidden)]
pub struct Field_finalrtable;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrtable) == 40);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrtable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_finalrtable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).finalrtable) };
#[doc(hidden)]
pub struct Field_finalrowmarks;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrowmarks) == 48);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrowmarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_finalrowmarks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).finalrowmarks) };
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, resultRelations) == 56
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_resultRelations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultRelations) };
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, appendRelations) == 64
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_appendRelations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).appendRelations) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, relationOids) == 72);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_relationOids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relationOids) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, invalItems) == 80);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_invalItems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).invalItems) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, paramExecTypes) == 88);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramExecTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_paramExecTypes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramExecTypes) };
#[doc(hidden)]
pub struct Field_lastPHId;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPHId) == 96);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPHId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_lastPHId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastPHId) };
#[doc(hidden)]
pub struct Field_lastRowMarkId;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastRowMarkId) == 100);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastRowMarkId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_lastRowMarkId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastRowMarkId) };
#[doc(hidden)]
pub struct Field_lastPlanNodeId;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPlanNodeId) == 104
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPlanNodeId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_lastPlanNodeId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastPlanNodeId) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, transientPlan) == 108);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transientPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_transientPlan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transientPlan) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, dependsOnRole) == 109);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 109;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_dependsOnRole>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dependsOnRole) };
#[doc(hidden)]
pub struct Field_parallelModeOK;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeOK) == 110
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeOK>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 110;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_parallelModeOK>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallelModeOK) };
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeNeeded) == 111
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeNeeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 111;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_parallelModeNeeded>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallelModeNeeded) };
#[doc(hidden)]
pub struct Field_maxParallelHazard;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, maxParallelHazard) == 112
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxParallelHazard>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_maxParallelHazard>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxParallelHazard) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 624);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PlannerInfo {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
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
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_parse;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, parse) == 8);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Query>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Query>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parse>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Query>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_parse>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parse) };
#[doc(hidden)]
pub struct Field_glob;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, glob) == 16);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::PlannerGlobal>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::PlannerGlobal>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_glob>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_glob>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).glob) };
#[doc(hidden)]
pub struct Field_query_level;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, query_level) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_query_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_query_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).query_level) };
#[doc(hidden)]
pub struct Field_plan_params;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, plan_params) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_plan_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_plan_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).plan_params) };
#[doc(hidden)]
pub struct Field_simple_rel_array_size;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, simple_rel_array_size) == 64
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_simple_rel_array_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_simple_rel_array_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).simple_rel_array_size) };
#[doc(hidden)]
pub struct Field_simple_rte_array;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, simple_rte_array) == 72);
const _: () = {
    assert!(::core::mem::size_of::<*mut *mut crate::__pgrx_c_bindings::RangeTblEntry>() == 8);
    assert!(::core::mem::align_of::<*mut *mut crate::__pgrx_c_bindings::RangeTblEntry>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_simple_rte_array>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>,
            c::expression::ReadWrite,
        >,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_simple_rte_array>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).simple_rte_array) };
#[doc(hidden)]
pub struct Field_join_rel_list;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, join_rel_list) == 104);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_join_rel_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_join_rel_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).join_rel_list) };
#[doc(hidden)]
pub struct Field_join_cur_level;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, join_cur_level) == 128);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_join_cur_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_join_cur_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).join_cur_level) };
#[doc(hidden)]
pub struct Field_init_plans;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, init_plans) == 136);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_init_plans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_init_plans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).init_plans) };
#[doc(hidden)]
pub struct Field_cte_plan_ids;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, cte_plan_ids) == 144);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cte_plan_ids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_cte_plan_ids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cte_plan_ids) };
#[doc(hidden)]
pub struct Field_multiexpr_params;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, multiexpr_params) == 152
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multiexpr_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_multiexpr_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).multiexpr_params) };
#[doc(hidden)]
pub struct Field_eq_classes;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, eq_classes) == 160);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eq_classes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_eq_classes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eq_classes) };
#[doc(hidden)]
pub struct Field_ec_merging_done;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, ec_merging_done) == 168);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ec_merging_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_ec_merging_done>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ec_merging_done) };
#[doc(hidden)]
pub struct Field_canon_pathkeys;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, canon_pathkeys) == 176);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canon_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_canon_pathkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).canon_pathkeys) };
#[doc(hidden)]
pub struct Field_left_join_clauses;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, left_join_clauses) == 184
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_left_join_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_left_join_clauses>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).left_join_clauses) };
#[doc(hidden)]
pub struct Field_right_join_clauses;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, right_join_clauses) == 192
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_right_join_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_right_join_clauses>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).right_join_clauses) };
#[doc(hidden)]
pub struct Field_full_join_clauses;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, full_join_clauses) == 200
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_full_join_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_full_join_clauses>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).full_join_clauses) };
#[doc(hidden)]
pub struct Field_join_info_list;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, join_info_list) == 208);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_join_info_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_join_info_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).join_info_list) };
#[doc(hidden)]
pub struct Field_append_rel_list;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, append_rel_list) == 232);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_append_rel_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_append_rel_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).append_rel_list) };
#[doc(hidden)]
pub struct Field_row_identity_vars;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, row_identity_vars) == 240
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_row_identity_vars>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_row_identity_vars>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).row_identity_vars) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, rowMarks) == 248);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_rowMarks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rowMarks) };
#[doc(hidden)]
pub struct Field_placeholder_list;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, placeholder_list) == 256
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_placeholder_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_placeholder_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).placeholder_list) };
#[doc(hidden)]
pub struct Field_fkey_list;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, fkey_list) == 264);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fkey_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_fkey_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fkey_list) };
#[doc(hidden)]
pub struct Field_query_pathkeys;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, query_pathkeys) == 272);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_query_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_query_pathkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).query_pathkeys) };
#[doc(hidden)]
pub struct Field_group_pathkeys;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, group_pathkeys) == 280);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_group_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_group_pathkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).group_pathkeys) };
#[doc(hidden)]
pub struct Field_window_pathkeys;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, window_pathkeys) == 288);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_window_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_window_pathkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).window_pathkeys) };
#[doc(hidden)]
pub struct Field_distinct_pathkeys;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, distinct_pathkeys) == 296
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinct_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_distinct_pathkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distinct_pathkeys) };
#[doc(hidden)]
pub struct Field_sort_pathkeys;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, sort_pathkeys) == 304);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sort_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_sort_pathkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sort_pathkeys) };
#[doc(hidden)]
pub struct Field_part_schemes;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, part_schemes) == 312);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_schemes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_part_schemes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).part_schemes) };
#[doc(hidden)]
pub struct Field_initial_rels;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, initial_rels) == 320);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initial_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_initial_rels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initial_rels) };
#[doc(hidden)]
pub struct Field_processed_tlist;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, processed_tlist) == 456);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processed_tlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 456;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_processed_tlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).processed_tlist) };
#[doc(hidden)]
pub struct Field_update_colnos;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, update_colnos) == 464);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_update_colnos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 464;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_update_colnos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).update_colnos) };
#[doc(hidden)]
pub struct Field_minmax_aggs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, minmax_aggs) == 480);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minmax_aggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 480;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_minmax_aggs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).minmax_aggs) };
#[doc(hidden)]
pub struct Field_qual_security_level;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, qual_security_level) == 520
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_qual_security_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 520;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_qual_security_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).qual_security_level) };
#[doc(hidden)]
pub struct Field_hasJoinRTEs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasJoinRTEs) == 524);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasJoinRTEs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 524;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_hasJoinRTEs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasJoinRTEs) };
#[doc(hidden)]
pub struct Field_hasLateralRTEs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasLateralRTEs) == 525);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasLateralRTEs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 525;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_hasLateralRTEs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasLateralRTEs) };
#[doc(hidden)]
pub struct Field_hasHavingQual;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasHavingQual) == 526);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasHavingQual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 526;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_hasHavingQual>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasHavingQual) };
#[doc(hidden)]
pub struct Field_hasPseudoConstantQuals;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasPseudoConstantQuals) == 527
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasPseudoConstantQuals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 527;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_hasPseudoConstantQuals>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasPseudoConstantQuals) };
#[doc(hidden)]
pub struct Field_hasAlternativeSubPlans;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasAlternativeSubPlans) == 528
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasAlternativeSubPlans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_hasAlternativeSubPlans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasAlternativeSubPlans) };
#[doc(hidden)]
pub struct Field_hasRecursion;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasRecursion) == 529);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasRecursion>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 529;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_hasRecursion>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasRecursion) };
#[doc(hidden)]
pub struct Field_agginfos;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, agginfos) == 536);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agginfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 536;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_agginfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).agginfos) };
#[doc(hidden)]
pub struct Field_aggtransinfos;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, aggtransinfos) == 544);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtransinfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 544;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_aggtransinfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggtransinfos) };
#[doc(hidden)]
pub struct Field_numOrderedAggs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, numOrderedAggs) == 552);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numOrderedAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 552;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_numOrderedAggs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numOrderedAggs) };
#[doc(hidden)]
pub struct Field_hasNonPartialAggs;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasNonPartialAggs) == 556
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasNonPartialAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 556;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_hasNonPartialAggs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasNonPartialAggs) };
#[doc(hidden)]
pub struct Field_hasNonSerialAggs;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasNonSerialAggs) == 557
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasNonSerialAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 557;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_hasNonSerialAggs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasNonSerialAggs) };
#[doc(hidden)]
pub struct Field_wt_param_id;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, wt_param_id) == 560);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wt_param_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 560;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_wt_param_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wt_param_id) };
#[doc(hidden)]
pub struct Field_curOuterParams;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, curOuterParams) == 584);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curOuterParams>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 584;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_curOuterParams>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curOuterParams) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, partColsUpdated) == 616);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partColsUpdated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 616;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_partColsUpdated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partColsUpdated) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerParamItem>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerParamItem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PlannerParamItem {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerParamItem, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerParamItem>
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
const _: Projection<crate::__pgrx_c_bindings::PlannerParamItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerParamItem> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_paramId;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerParamItem, paramId) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerParamItem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerParamItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerParamItem> as c::expression::OrdinaryField<Field_paramId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramId) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Pool>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Pool>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Pool {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pool, data) == 0);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Chromosome>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Chromosome>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pool>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Chromosome>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Pool,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Pool> as c::expression::OrdinaryField<
        Field_data,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pool, size) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pool>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Pool,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Pool> as c::expression::OrdinaryField<
        Field_size,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
#[doc(hidden)]
pub struct Field_string_length;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pool, string_length) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_string_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pool>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Pool,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Pool> as c::expression::OrdinaryField<
        Field_string_length,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).string_length) };
#[doc(hidden)]
pub struct Field_createSubid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PortalData>() == 232);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PortalData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PortalData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, createSubid) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_createSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_createSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).createSubid) };
#[doc(hidden)]
pub struct Field_activeSubid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, activeSubid) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_activeSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_activeSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).activeSubid) };
#[doc(hidden)]
pub struct Field_createLevel;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, createLevel) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_createLevel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_createLevel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).createLevel) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, commandTag) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commandTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CommandTag,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_commandTag>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commandTag) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, stmts) == 88);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_stmts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmts) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, strategy) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PortalStrategy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_strategy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strategy) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, cursorOptions) == 124);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursorOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_cursorOptions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cursorOptions) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_run_once;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, run_once) == 128);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_run_once>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_run_once>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).run_once) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, status) == 132);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
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
    const OFFSET: usize = 132;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_status>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).status) };
#[doc(hidden)]
pub struct Field_portalPinned;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, portalPinned) == 136);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_portalPinned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_portalPinned>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).portalPinned) };
#[doc(hidden)]
pub struct Field_autoHeld;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, autoHeld) == 137);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_autoHeld>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 137;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_autoHeld>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).autoHeld) };
#[doc(hidden)]
pub struct Field_atStart;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, atStart) == 200);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atStart>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_atStart>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atStart) };
#[doc(hidden)]
pub struct Field_atEnd;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, atEnd) == 201);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atEnd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 201;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_atEnd>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atEnd) };
#[doc(hidden)]
pub struct Field_portalPos;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, portalPos) == 208);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_portalPos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_portalPos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).portalPos) };
#[doc(hidden)]
pub struct Field_creation_time;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, creation_time) == 216);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_creation_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_creation_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).creation_time) };
#[doc(hidden)]
pub struct Field_visible;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, visible) == 224);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_visible>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PortalData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PortalData> as c::expression::OrdinaryField<Field_visible>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).visible) };
#[doc(hidden)]
pub struct Field_SxactGlobalXmin;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PredXactListData>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PredXactListData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PredXactListData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PredXactListData, SxactGlobalXmin) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_SxactGlobalXmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PredXactListData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData> as c::expression::OrdinaryField<Field_SxactGlobalXmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).SxactGlobalXmin) };
#[doc(hidden)]
pub struct Field_SxactGlobalXminCount;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PredXactListData, SxactGlobalXminCount) == 36
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_SxactGlobalXminCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PredXactListData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData> as c::expression::OrdinaryField<Field_SxactGlobalXminCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).SxactGlobalXminCount) };
#[doc(hidden)]
pub struct Field_WritableSxactCount;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PredXactListData, WritableSxactCount) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_WritableSxactCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PredXactListData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData> as c::expression::OrdinaryField<Field_WritableSxactCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).WritableSxactCount) };
#[doc(hidden)]
pub struct Field_LastSxactCommitSeqNo;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PredXactListData, LastSxactCommitSeqNo) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_LastSxactCommitSeqNo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PredXactListData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData> as c::expression::OrdinaryField<Field_LastSxactCommitSeqNo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).LastSxactCommitSeqNo) };
#[doc(hidden)]
pub struct Field_CanPartialClearThrough;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PredXactListData, CanPartialClearThrough)
        == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_CanPartialClearThrough>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PredXactListData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData> as c::expression::OrdinaryField<Field_CanPartialClearThrough>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).CanPartialClearThrough) };
#[doc(hidden)]
pub struct Field_HavePartialClearedThrough;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PredXactListData, HavePartialClearedThrough)
        == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_HavePartialClearedThrough>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PredXactListData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PredXactListData> as c::expression::OrdinaryField<Field_HavePartialClearedThrough>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).HavePartialClearedThrough) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PredicateLockData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PredicateLockData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PredicateLockData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PredicateLockData, nelements) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nelements>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PredicateLockData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PredicateLockData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PredicateLockData> as c::expression::OrdinaryField<Field_nelements>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nelements) };
#[doc(hidden)]
pub struct Field_recent_buffer;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PrefetchBufferResult>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PrefetchBufferResult>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PrefetchBufferResult {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PrefetchBufferResult, recent_buffer) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recent_buffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PrefetchBufferResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PrefetchBufferResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PrefetchBufferResult> as c::expression::OrdinaryField<Field_recent_buffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recent_buffer) };
#[doc(hidden)]
pub struct Field_initiated_io;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PrefetchBufferResult, initiated_io) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initiated_io>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PrefetchBufferResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PrefetchBufferResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PrefetchBufferResult> as c::expression::OrdinaryField<Field_initiated_io>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initiated_io) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PrepareStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PrepareStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PrepareStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PrepareStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PrepareStmt>
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
const _: Projection<crate::__pgrx_c_bindings::PrepareStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PrepareStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_argtypes;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PrepareStmt, argtypes) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_argtypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PrepareStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PrepareStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PrepareStmt> as c::expression::OrdinaryField<Field_argtypes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).argtypes) };
#[doc(hidden)]
pub struct Field_attno;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PresortedKeyData>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PresortedKeyData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PresortedKeyData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PresortedKeyData, attno) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PresortedKeyData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PresortedKeyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PresortedKeyData> as c::expression::OrdinaryField<Field_attno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attno) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ProjectSetState>() == 232);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ProjectSetState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ProjectSetState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ProjectSetState, nelems) == 216);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nelems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ProjectSetState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ProjectSetState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ProjectSetState> as c::expression::OrdinaryField<Field_nelems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nelems) };
#[doc(hidden)]
pub struct Field_pending_srf_tuples;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ProjectSetState, pending_srf_tuples) == 220
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pending_srf_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ProjectSetState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 220;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ProjectSetState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ProjectSetState> as c::expression::OrdinaryField<Field_pending_srf_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pending_srf_tuples) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ProjectionInfo>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ProjectionInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ProjectionInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ProjectionInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ProjectionInfo>
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
const _: Projection<crate::__pgrx_c_bindings::ProjectionInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ProjectionInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_dummypp;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ProjectionPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ProjectionPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ProjectionPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ProjectionPath, dummypp) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dummypp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ProjectionPath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ProjectionPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ProjectionPath> as c::expression::OrdinaryField<Field_dummypp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dummypp) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Publication>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Publication>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Publication {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Publication, oid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Publication>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Publication, <<c::expression::CRecord<crate::__pgrx_c_bindings::Publication> as c::expression::OrdinaryField<Field_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oid) };
#[doc(hidden)]
pub struct Field_alltables;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Publication, alltables) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_alltables>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Publication, <<c::expression::CRecord<crate::__pgrx_c_bindings::Publication> as c::expression::OrdinaryField<Field_alltables>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).alltables) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Publication, pubviaroot) == 17);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubviaroot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Publication, <<c::expression::CRecord<crate::__pgrx_c_bindings::Publication> as c::expression::OrdinaryField<Field_pubviaroot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pubviaroot) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PublicationActions>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PublicationActions>() == 1);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PublicationActions {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationActions, pubinsert) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubinsert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationActions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationActions, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationActions> as c::expression::OrdinaryField<Field_pubinsert>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pubinsert) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationActions, pubupdate) == 1);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubupdate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationActions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationActions, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationActions> as c::expression::OrdinaryField<Field_pubupdate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pubupdate) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationActions, pubdelete) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubdelete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationActions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationActions, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationActions> as c::expression::OrdinaryField<Field_pubdelete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pubdelete) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationActions, pubtruncate) == 3
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubtruncate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationActions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 3;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationActions, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationActions> as c::expression::OrdinaryField<Field_pubtruncate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pubtruncate) };
#[doc(hidden)]
pub struct Field_rf_valid_for_update;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PublicationDesc>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PublicationDesc>() == 1);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PublicationDesc {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationDesc, rf_valid_for_update) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rf_valid_for_update>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc> as c::expression::OrdinaryField<Field_rf_valid_for_update>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rf_valid_for_update) };
#[doc(hidden)]
pub struct Field_rf_valid_for_delete;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationDesc, rf_valid_for_delete) == 5
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rf_valid_for_delete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc> as c::expression::OrdinaryField<Field_rf_valid_for_delete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rf_valid_for_delete) };
#[doc(hidden)]
pub struct Field_cols_valid_for_update;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationDesc, cols_valid_for_update) == 6
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cols_valid_for_update>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc> as c::expression::OrdinaryField<Field_cols_valid_for_update>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cols_valid_for_update) };
#[doc(hidden)]
pub struct Field_cols_valid_for_delete;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationDesc, cols_valid_for_delete) == 7
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cols_valid_for_delete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 7;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc> as c::expression::OrdinaryField<Field_cols_valid_for_delete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cols_valid_for_delete) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PublicationObjSpec>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PublicationObjSpec>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PublicationObjSpec {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationObjSpec, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationObjSpec>
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
const _: Projection<crate::__pgrx_c_bindings::PublicationObjSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationObjSpec> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_pubobjtype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationObjSpec, pubobjtype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubobjtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationObjSpec>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PublicationObjSpecType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationObjSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationObjSpec> as c::expression::OrdinaryField<Field_pubobjtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pubobjtype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationObjSpec, location) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationObjSpec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationObjSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationObjSpec> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PublicationTable>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PublicationTable>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PublicationTable {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationTable, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationTable>
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
const _: Projection<crate::__pgrx_c_bindings::PublicationTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationTable> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationTable, columns) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_columns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationTable> as c::expression::OrdinaryField<Field_columns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).columns) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::QTNode>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::QTNode>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::QTNode {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QTNode, flags) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QTNode>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::QTNode,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::QTNode> as c::expression::OrdinaryField<
        Field_flags,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_nchild;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QTNode, nchild) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nchild>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QTNode>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::QTNode,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::QTNode> as c::expression::OrdinaryField<
        Field_nchild,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nchild) };
#[doc(hidden)]
pub struct Field_sign;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QTNode, sign) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QTNode>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::QTNode,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::QTNode> as c::expression::OrdinaryField<
        Field_sign,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sign) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Query>() == 248);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Query>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Query {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
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
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, commandType) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commandType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CmdType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_commandType,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commandType) };
#[doc(hidden)]
pub struct Field_querySource;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, querySource) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_querySource>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__QuerySource,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_querySource,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).querySource) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, queryId) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_queryId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_queryId,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).queryId) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, canSetTag) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canSetTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_canSetTag,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).canSetTag) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, resultRelation) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_resultRelation,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultRelation) };
#[doc(hidden)]
pub struct Field_hasAggs;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, hasAggs) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_hasAggs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasAggs) };
#[doc(hidden)]
pub struct Field_hasWindowFuncs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, hasWindowFuncs) == 45);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasWindowFuncs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 45;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_hasWindowFuncs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasWindowFuncs) };
#[doc(hidden)]
pub struct Field_hasTargetSRFs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, hasTargetSRFs) == 46);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasTargetSRFs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 46;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_hasTargetSRFs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasTargetSRFs) };
#[doc(hidden)]
pub struct Field_hasSubLinks;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, hasSubLinks) == 47);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasSubLinks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 47;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_hasSubLinks,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasSubLinks) };
#[doc(hidden)]
pub struct Field_hasDistinctOn;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, hasDistinctOn) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasDistinctOn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_hasDistinctOn,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasDistinctOn) };
#[doc(hidden)]
pub struct Field_hasRecursive;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, hasRecursive) == 49);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasRecursive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 49;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_hasRecursive,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasRecursive) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, hasModifyingCTE) == 50);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasModifyingCTE>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 50;
}
