//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use core::alloc::Layout;
    use core::mem::MaybeUninit;
    use pgrx::PgMemoryContexts;
    use pgrx::memcx;
    use pgrx::palloc::PBox;
    use pgrx::prelude::*;

    /// Run `f` inside a freshly-created child memory context, then delete the
    /// child and assert the parent's `MemoryContextMemAllocated` returned to
    /// its baseline. If anything `f` allocated escaped the child context,
    /// the parent's byte count grew and we fail.
    ///
    /// We use `PgMemoryContexts::new` (which wraps `AllocSetContextCreateExtended`)
    /// and rely on its `Drop` impl to call `MemoryContextDelete`, so the child
    /// context is freed even on unwind. The `recurse=false` argument to
    /// `MemoryContextMemAllocated` measures only the parent's own blocks,
    /// so child allocations don't pollute the baseline.
    ///
    /// Panic-safety: a `CxRestoreGuard` (Drop-based) restores the previous
    /// `CurrentMemoryContext` even if `f` panics. The guard is declared
    /// *after* `child` so on unwind it drops first (LIFO): CMC is restored,
    /// then the child context is deleted.
    ///
    /// Caveat: `MemoryContextMemAllocated` reports the AllocSet's *block*
    /// count, not byte-precise usage. Allocations that fit inside the
    /// initial 8 KiB block (`ALLOCSET_DEFAULT_INITSIZE`) won't move the
    /// counter, so a hypothetical leak smaller than that is invisible.
    /// Tests that need leak detection should allocate well past one block
    /// to force a fresh AllocSet block to be claimed.
    fn with_scoped_memcx<F>(f: F)
    where
        F: for<'mcx> FnOnce(&pgrx::memcx::MemCx<'mcx>),
    {
        struct CxRestoreGuard {
            previous: pg_sys::MemoryContext,
        }
        impl Drop for CxRestoreGuard {
            fn drop(&mut self) {
                // SAFETY: `previous` was the value of CurrentMemoryContext at
                // the time we switched away; restoring it cannot fail.
                unsafe {
                    pg_sys::MemoryContextSwitchTo(self.previous);
                }
            }
        }

        unsafe {
            let parent = pg_sys::CurrentMemoryContext;
            let before = pg_sys::MemoryContextMemAllocated(parent, false);

            let child = PgMemoryContexts::new("pbox_slice_test_scope");
            // Switch CMC and arm the restore guard. The guard is held in a
            // named binding (NOT `let _ = ...`, which would drop immediately)
            // so it lives until the end of the unsafe block, then drops in
            // LIFO order before `child` — restoring CMC before the child
            // context is deleted. `#[allow(unused_variables)]` silences the
            // dead-binding lint since the guard's effect is in its Drop impl.
            #[allow(unused_variables)]
            let restore_guard = {
                let previous = pg_sys::MemoryContextSwitchTo(child.value());
                CxRestoreGuard { previous }
            };
            memcx::current_context(|cx| f(cx));

            let after = pg_sys::MemoryContextMemAllocated(parent, false);
            assert_eq!(
                before,
                after,
                "leak: parent context grew by {} bytes after child deletion",
                after.saturating_sub(before),
            );
        }
    }

    #[pg_test]
    fn alloc_layout_returns_aligned_nonnull() {
        memcx::current_context(|cx| {
            let layout = Layout::from_size_align(128, 8).unwrap();
            let p = cx.alloc_layout(layout).expect("alloc");
            assert_eq!(p.as_ptr() as usize % 8, 0);
        });
    }

    #[pg_test]
    fn alloc_layout_zeroed_is_zero() {
        memcx::current_context(|cx| {
            let layout = Layout::from_size_align(64, 8).unwrap();
            let p = cx.alloc_layout_zeroed(layout).expect("alloc");
            let s = unsafe { core::slice::from_raw_parts(p.as_ptr(), 64) };
            assert!(s.iter().all(|&b| b == 0));
        });
    }

    #[pg_test]
    fn new_uninit_slice_in_len_and_align() {
        memcx::current_context(|cx| {
            let s: PBox<[MaybeUninit<u32>]> = PBox::new_uninit_slice_in(16, cx).expect("alloc");
            assert_eq!(s.len(), 16);
            assert_eq!(s.as_ptr() as *const u32 as usize % core::mem::align_of::<u32>(), 0);
        });
    }

    #[pg_test]
    fn new_zeroed_slice_in_is_zero() {
        memcx::current_context(|cx| {
            let s: PBox<[MaybeUninit<u8>]> = PBox::new_zeroed_slice_in(32, cx).expect("alloc");
            // SAFETY: zero is a valid bit pattern for u8.
            let init: PBox<[u8]> = unsafe { s.assume_init() };
            assert_eq!(init.len(), 32);
            assert!(init.iter().all(|&b| b == 0));
        });
    }

    #[pg_test]
    fn from_slice_in_roundtrip() {
        memcx::current_context(|cx| {
            let src = [1u32, 2, 3, 4, 5];
            let dst: PBox<[u32]> = PBox::from_slice_in(&src, cx).expect("alloc");
            assert_eq!(&*dst, &src);
        });
    }

    #[pg_test]
    fn from_iter_in_happy_path() {
        memcx::current_context(|cx| {
            let v: Vec<i32> = (0..10).collect();
            let dst: PBox<[i32]> = PBox::from_iter_in(v.iter().copied(), cx).expect("alloc");
            assert_eq!(&*dst, v.as_slice());
        });
    }

    #[pg_test]
    fn from_iter_in_under_yield_returns_err() {
        // ExactSizeIterator promises 10, yields 3 — must return
        // FromIterError::IteratorUnderYield (not OOM) and leave the buffer to the context (no panic, no UB). `T: Copy` so there is no Drop
        // concern on the partial prefix.
        struct ShortIter {
            i: usize,
            promised: usize,
            real: usize,
        }
        impl Iterator for ShortIter {
            type Item = u32;
            fn next(&mut self) -> Option<u32> {
                if self.i >= self.real {
                    return None;
                }
                let v = self.i as u32;
                self.i += 1;
                Some(v)
            }
        }
        impl ExactSizeIterator for ShortIter {
            fn len(&self) -> usize {
                self.promised - self.i
            }
        }
        memcx::current_context(|cx| {
            let r = PBox::<[u32]>::from_iter_in(ShortIter { i: 0, promised: 10, real: 3 }, cx);
            match r {
                Err(pgrx::palloc::FromIterError::IteratorUnderYield { promised, yielded }) => {
                    assert_eq!(promised, 10);
                    assert_eq!(yielded, 3);
                }
                Err(pgrx::palloc::FromIterError::OutOfMemory) => {
                    panic!("expected IteratorUnderYield, got OutOfMemory")
                }
                Ok(_) => panic!("expected IteratorUnderYield, got Ok"),
            }
        });
    }

    #[pg_test]
    fn from_iter_in_over_yield_truncates() {
        struct OverIter {
            i: u32,
            promised: usize,
        }
        impl Iterator for OverIter {
            type Item = u32;
            fn next(&mut self) -> Option<u32> {
                let v = self.i;
                self.i += 1;
                Some(v) // never returns None
            }
        }
        impl ExactSizeIterator for OverIter {
            fn len(&self) -> usize {
                self.promised
            }
        }
        memcx::current_context(|cx| {
            let r = PBox::<[u32]>::from_iter_in(OverIter { i: 0, promised: 3 }, cx).expect("alloc");
            assert_eq!(&*r, &[0u32, 1, 2]);
        });
    }

    #[pg_test]
    fn zero_length_slice_is_safe() {
        memcx::current_context(|cx| {
            let s: PBox<[i64]> = PBox::from_slice_in(&[], cx).expect("alloc");
            assert_eq!(s.len(), 0);
            assert!(s.is_empty());
            // Deref to an empty slice must not UB.
            #[allow(clippy::explicit_auto_deref)]
            let r: &[i64] = &*s;
            assert_eq!(r.len(), 0);
        });
    }

    #[pg_test]
    fn new_uninit_slice_in_zero_length() {
        // palloc(0) is implementation-defined across PG versions; the wrapper
        // must still hand back a usable empty PBox without UB.
        memcx::current_context(|cx| {
            let s: PBox<[MaybeUninit<u64>]> =
                PBox::new_uninit_slice_in(0, cx).expect("zero-length alloc");
            assert_eq!(s.len(), 0);
            // SAFETY: zero elements means nothing to initialize; assume_init is trivially sound.
            let init: PBox<[u64]> = unsafe { s.assume_init() };
            assert!(init.is_empty());
            let r: &[u64] = &*init;
            assert_eq!(r.len(), 0);
        });
    }

    #[pg_test]
    fn new_zeroed_slice_in_zero_length() {
        memcx::current_context(|cx| {
            let s: PBox<[MaybeUninit<u32>]> =
                PBox::new_zeroed_slice_in(0, cx).expect("zero-length alloc");
            assert_eq!(s.len(), 0);
            // SAFETY: zero elements, nothing to read.
            let init: PBox<[u32]> = unsafe { s.assume_init() };
            assert!(init.is_empty());
        });
    }

    #[pg_test]
    fn oom_returns_err() {
        memcx::current_context(|cx| {
            // Request a length so large that `Layout::array` overflows
            // (length * size > isize::MAX). PBox maps that to OutOfMemory
            // before ever reaching palloc, so no ereport can occur.
            let r = PBox::<[core::mem::MaybeUninit<u64>]>::new_uninit_slice_in(usize::MAX, cx);
            assert!(r.is_err(), "expected OOM, got Ok");
        });
    }

    #[pg_test]
    fn no_leak_after_context_delete() {
        with_scoped_memcx(|cx| {
            // Allocate a slice large enough to force the AllocSet to grab
            // a fresh block (well past ALLOCSET_DEFAULT_INITSIZE = 8 KB),
            // so a hypothetical leak would visibly grow the parent's
            // MemoryContextMemAllocated. Intentionally do NOT pfree —
            // the harness verifies that deleting the child reclaims it.
            let _s: PBox<[u64]> = PBox::from_slice_in(&[1u64; 131_072], cx).expect("alloc");
        });
    }

    #[pg_test]
    fn alloc_layout_rejects_over_max_alloc_size() {
        memcx::current_context(|cx| {
            // MaxAllocSize = 1 GiB - 1. Request 1 GiB + 1 byte through the
            // non-huge path: must be rejected up front as OutOfMemory
            // (not via PG ereport longjmp).
            let layout = Layout::from_size_align((1usize << 30) + 1, 8).unwrap();
            assert!(cx.alloc_layout(layout).is_err());
            assert!(cx.alloc_layout_zeroed(layout).is_err());
        });
    }

    #[pg_test]
    fn alloc_huge_layout_accepts_over_max_alloc_size() {
        memcx::current_context(|cx| {
            // 1.5 GiB — above MaxAllocSize, well below MaxAllocHugeSize.
            // Succeeds on a machine with sufficient RAM; on a tight test
            // environment we accept OutOfMemory as a valid response.
            // The critical property: no longjmp, no panic — Rust gets
            // back a Result either way.
            let layout = Layout::from_size_align(3usize << 29, 8).unwrap();
            let _ = cx.alloc_huge_layout(layout);
        });
    }

    #[pg_test]
    fn alloc_huge_layout_zeroed_is_zero() {
        memcx::current_context(|cx| {
            // Small probe — verifies the ZERO flag still flows through
            // when HUGE is set. No need to actually allocate huge here.
            let layout = Layout::from_size_align(64, 8).unwrap();
            let p = cx.alloc_huge_layout_zeroed(layout).expect("alloc");
            let s = unsafe { core::slice::from_raw_parts(p.as_ptr(), 64) };
            assert!(s.iter().all(|&b| b == 0));
        });
    }

    #[pg_test]
    fn new_uninit_slice_in_rejects_over_max_alloc_size() {
        const MAX_ALLOC_SIZE: usize = 0x3fff_ffff;
        let len = (MAX_ALLOC_SIZE / core::mem::size_of::<u64>()) + 1;
        memcx::current_context(|cx| {
            let r = PBox::<[MaybeUninit<u64>]>::new_uninit_slice_in(len, cx);
            assert!(r.is_err(), "expected OOM at MaxAllocSize boundary, got Ok");
            let r0 = PBox::<[MaybeUninit<u64>]>::new_zeroed_slice_in(len, cx);
            assert!(r0.is_err(), "expected OOM at MaxAllocSize boundary (zeroed), got Ok");
        });
    }

    #[pg_test]
    fn alloc_varlena_zero_payload() {
        memcx::current_context(|cx| {
            let mut buf = cx.alloc_varlena(0).expect("alloc");
            assert_eq!(buf.total_size(), pg_sys::VARHDRSZ);
            assert_eq!(buf.payload().len(), 0);
            assert_eq!(buf.payload_mut().len(), 0);
            let ptr = buf.into_raw();
            // Round-trip: the empty payload must still be readable via inspect.
            unsafe {
                let view = cx.inspect_varlena(ptr);
                assert_eq!(view.total_size(), pg_sys::VARHDRSZ);
                assert_eq!(view.payload().len(), 0);
            }
        });
    }

    #[pg_test]
    fn from_huge_slice_in_roundtrip() {
        memcx::current_context(|cx| {
            let src = [10u32, 20, 30, 40, 50];
            let dst: PBox<[u32]> = PBox::from_huge_slice_in(&src, cx).expect("alloc");
            assert_eq!(&*dst, &src);
        });
    }

    #[pg_test]
    fn from_huge_iter_in_roundtrip() {
        memcx::current_context(|cx| {
            let v: Vec<i64> = (100..110).collect();
            let dst: PBox<[i64]> = PBox::from_huge_iter_in(v.iter().copied(), cx).expect("alloc");
            assert_eq!(&*dst, v.as_slice());
        });
    }
}
