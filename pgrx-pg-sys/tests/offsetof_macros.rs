//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check generated offsets against the selected PostgreSQL binding layouts.
//!
//! Public invocations use actual record types and fields, making layout and
//! unevaluated-operand behavior observable at the downstream macro boundary.

#![cfg(all(pgrx_c_macros, not(docsrs)))]
#![deny(unsafe_op_in_unsafe_fn)]

/// Exercise the selected build's public bindings and macro exports from a downstream consumer.
use pgrx_pg_sys as pg;

pg::__pgrx_c_classify! { @if_available SizeForFunctionCallInfo {
/// Checks that generated call frame size uses the current bound flexible member offset.
#[test]
fn generated_call_frame_size_uses_the_current_bound_flexible_member_offset() {
    // fmgr.h uses offsetof(FunctionCallInfoBaseData,args) and sizeof(NullableDatum).
    // Compare against this build's live binding layouts, including defined C
    // unsigned arithmetic for a negative nargs; no call frame is materialized.
    for nargs in [-1_i32,0,1,2,31,256,i32::MAX] {
        let calls=core::cell::Cell::new(0_u32);
        let actual=pg::SizeForFunctionCallInfo!({calls.set(calls.get()+1); nargs}).get() as usize;
        let expected=core::mem::offset_of!(pg::FunctionCallInfoBaseData,args)
            .wrapping_add(core::mem::size_of::<pg::NullableDatum>().wrapping_mul(nargs as usize));
        assert_eq!(actual,expected,"nargs={nargs}");
        assert_eq!(calls.get(),1);
    }
}
} }

pg::__pgrx_c_classify! { @if_available CALCDATASIZE {
/// Checks that generated text search size keeps word entry layout and operand evaluation.
#[test]
fn generated_text_search_size_keeps_word_entry_layout_and_operand_evaluation() {
    // ts_type.h adds the flexible entries offset, WordEntry storage and string
    // bytes. WordEntry contains bitfields, which never need to be loaded here.
    for (entries,string_bytes) in [(0_i32,0_i32),(1,0),(2,7),(31,127),(-1,5)] {
        let first=core::cell::Cell::new(0_u32);
        let second=core::cell::Cell::new(0_u32);
        let actual=pg::CALCDATASIZE!(
            {first.set(first.get()+1); entries},
            {second.set(second.get()+1); string_bytes}
        ).get() as usize;
        let expected=core::mem::offset_of!(pg::TSVectorData,entries)
            .wrapping_add(core::mem::size_of::<pg::WordEntry>().wrapping_mul(entries as usize))
            .wrapping_add(string_bytes as usize);
        assert_eq!(actual,expected,"entries={entries}, string_bytes={string_bytes}");
        assert_eq!((first.get(),second.get()),(1,1));
    }
}
} }

pg::__pgrx_c_classify! { @if_available SizeOfGinPostingList {
/// Checks that generated gin posting list size reads only its initialized length.
#[test]
fn generated_gin_posting_list_size_reads_only_its_initialized_length() {
    /// Provide owned raw record storage so tests can initialize only fields that a C macro
    /// actually accesses.
    use core::mem::MaybeUninit;
    /// Address selected C fields without creating references to an incompletely initialized
    /// record.
    use core::ptr::addr_of_mut;

    let mut storage=MaybeUninit::<pg::GinPostingList>::uninit();
    let pointer=addr_of_mut!(storage).cast::<pg::GinPostingList>();
    for bytes in [0_u16,1,2,3,31,u16::MAX] {
        let calls=core::cell::Cell::new(0_u32);
        // SAFETY: pointer addresses owned live aligned GinPostingList storage.
        // Only nbytes is initialized or read; no reference or value of the
        // partially initialized whole record is created. The offsetof term
        // accesses no object. Alignment arithmetic never dereferences the
        // flexible payload or calls PostgreSQL, and access is exclusive.
        let actual=unsafe {
            addr_of_mut!((*pointer).nbytes).write(bytes);
            pg::SizeOfGinPostingList!({calls.set(calls.get()+1); pointer}).get() as usize
        };
        let aligned=(usize::from(bytes)+1)&!1;
        let expected=core::mem::offset_of!(pg::GinPostingList,bytes)+aligned;
        assert_eq!(actual,expected,"nbytes={bytes}");
        assert_eq!(calls.get(),1);
    }
}
} }

pg::__pgrx_c_classify! { @if_available GinNextPostingListSegment {
/// Checks that generated gin segment advance stays within owned uninitialized storage.
#[test]
fn generated_gin_segment_advance_stays_within_owned_uninitialized_storage() {
    /// Provide owned raw record storage so tests can initialize only fields that a C macro
    /// actually accesses.
    use core::mem::MaybeUninit;
    /// Address selected C fields without creating references to an incompletely initialized
    /// record.
    use core::ptr::addr_of_mut;

    let mut storage=MaybeUninit::<[pg::GinPostingList;16]>::uninit();
    let base=addr_of_mut!(storage).cast::<u8>();
    let pointer=base.cast::<pg::GinPostingList>();
    for bytes in [0_u16,1,2,3,7,31] {
        let calls=core::cell::Cell::new(0_u32);
        let expected=core::mem::offset_of!(pg::GinPostingList,bytes)+((usize::from(bytes)+1)&!1);
        assert!(expected<core::mem::size_of_val(&storage));
        assert_eq!(expected%core::mem::align_of::<pg::GinPostingList>(),0);
        // SAFETY: base and pointer derive from one owned aligned live allocation.
        // nbytes is initialized and exclusively accessed. The resulting byte
        // offset was checked in bounds and aligned for GinPostingList. Neither
        // the incomplete first record nor its flexible payload is borrowed or
        // materialized. The returned pointer is compared within this allocation
        // and never dereferenced; no backend or global state is involved.
        let actual=unsafe {
            addr_of_mut!((*pointer).nbytes).write(bytes);
            let next=pg::GinNextPostingListSegment!({calls.set(calls.get()+1); pointer}).get();
            next.cast::<u8>().offset_from(base) as usize
        };
        assert_eq!(actual,expected,"nbytes={bytes}");
        // ginblock.h substitutes cur once as the base pointer and once in its
        // delegated SizeOfGinPostingList call.
        assert_eq!(calls.get(),2);
    }
}
} }
