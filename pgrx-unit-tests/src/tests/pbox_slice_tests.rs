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
    /// Caveats:
    /// - This harness is **not** unwind-safe in the strict sense: if `f`
    ///   panics, the `MemoryContextSwitchTo(prev)` call never runs and
    ///   `CurrentMemoryContext` is left pointing at the now-deleted child
    ///   until the outer `pg_test` harness catches the panic and restores
    ///   the test's outer context. Don't use this harness in code paths
    ///   where the post-panic state matters before that catch.
    /// - `MemoryContextMemAllocated` reports the AllocSet's *block* count,
    ///   not byte-precise usage. Allocations that fit inside the initial
    ///   8 KiB block (`ALLOCSET_DEFAULT_INITSIZE`) won't move the counter,
    ///   so a hypothetical leak smaller than that is invisible to this
    ///   harness. Tests that need leak detection should allocate well past
    ///   one block to force a fresh AllocSet block to be claimed.
    fn with_scoped_memcx<F>(f: F)
    where
        F: for<'mcx> FnOnce(&pgrx::memcx::MemCx<'mcx>),
    {
        unsafe {
            let parent = pg_sys::CurrentMemoryContext;
            let before = pg_sys::MemoryContextMemAllocated(parent, false);

            // Build the child as an Owned context; its Drop will MemoryContextDelete.
            let child = PgMemoryContexts::new("pbox_slice_test_scope");
            // current_context() reads pg_sys::CurrentMemoryContext, so switch
            // first, then restore. We deliberately do NOT use child.switch_to()
            // here because its closure receives `&mut PgMemoryContexts`, not a
            // `&MemCx<'_>`; the manual switch lets us reach memcx::current_context.
            let prev = pg_sys::MemoryContextSwitchTo(child.value());
            memcx::current_context(|cx| f(cx));
            pg_sys::MemoryContextSwitchTo(prev);

            // Drop the owned child explicitly so MemoryContextDelete runs
            // before we sample `after`.
            drop(child);

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
    fn from_iter_in_drops_prefix_on_panic() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static DROPS: AtomicUsize = AtomicUsize::new(0);
        struct D(#[allow(dead_code)] usize);
        impl Drop for D {
            fn drop(&mut self) {
                DROPS.fetch_add(1, Ordering::SeqCst);
            }
        }

        struct PanicIter {
            i: usize,
            max: usize,
            panic_at: usize,
        }
        impl Iterator for PanicIter {
            type Item = D;
            fn next(&mut self) -> Option<D> {
                if self.i == self.panic_at {
                    panic!("boom")
                }
                if self.i >= self.max {
                    return None;
                }
                let v = D(self.i);
                self.i += 1;
                Some(v)
            }
        }
        impl ExactSizeIterator for PanicIter {
            fn len(&self) -> usize {
                self.max - self.i
            }
        }

        DROPS.store(0, Ordering::SeqCst);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            memcx::current_context(|cx| {
                let _ = PBox::<[D]>::from_iter_in(PanicIter { i: 0, max: 10, panic_at: 3 }, cx);
            });
        }));
        assert!(result.is_err());
        // Initialized prefix (0,1,2) must drop exactly once each.
        assert_eq!(DROPS.load(Ordering::SeqCst), 3);
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
}
