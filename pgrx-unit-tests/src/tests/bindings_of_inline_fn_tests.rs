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
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use pgrx::prelude::*;

    #[pg_test]
    fn test_static_inline_fns() {
        unsafe {
            use pgrx::pg_sys::{BlockIdData, ItemPointerData, itemptr_encode};
            assert_eq!(
                itemptr_encode(&mut ItemPointerData {
                    ip_blkid: BlockIdData { bi_hi: 111, bi_lo: 222 },
                    ip_posid: 333
                }),
                476755919181
            );
        }
    }

    /// `LockBuffer` became `static inline` in PG19. Like every static inline function's wrapper,
    /// pgrx compiles it with or without the C shim, so portable code can call it on every version.
    #[pg_test]
    fn test_lock_buffer() {
        // SAFETY: pg_class always has a block 0. The buffer stays pinned while it is locked and
        // unlocked, and the relation stays open until the buffer is released.
        unsafe {
            let lockmode = pg_sys::AccessShareLock as pg_sys::LOCKMODE;
            let relation = pg_sys::table_open(pg_sys::RelationRelationId, lockmode);
            let buffer = pg_sys::ReadBuffer(relation, 0);
            pg_sys::LockBuffer(buffer, pg_sys::BUFFER_LOCK_SHARE as _);
            #[cfg(feature = "pg19")]
            assert!(pg_sys::BufferIsLockedByMeInMode(buffer, pg_sys::BUFFER_LOCK_SHARE));
            pg_sys::LockBuffer(buffer, pg_sys::BUFFER_LOCK_UNLOCK as _);
            #[cfg(feature = "pg19")]
            assert!(!pg_sys::BufferIsLockedByMe(buffer));
            pg_sys::ReleaseBuffer(buffer);
            pg_sys::table_close(relation, lockmode);
        }
    }
}
