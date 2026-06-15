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
    use pgrx::datum::varlena_buf::VarlenaBuf;
    use pgrx::memcx;
    use pgrx::pg_sys;
    use pgrx::prelude::*;

    #[pg_test]
    fn alloc_varlena_header_is_valid() {
        memcx::current_context(|cx| {
            let buf: VarlenaBuf = cx.alloc_varlena(64).expect("alloc");
            assert_eq!(buf.payload().len(), 64);
            // payload should be zeroed
            assert!(buf.payload().iter().all(|&b| b == 0));
            // total_size = header + payload
            assert_eq!(buf.total_size(), 64 + pg_sys::VARHDRSZ);
            // hand off to PG and check via varsize_any
            let raw = buf.into_raw();
            unsafe {
                assert_eq!(pgrx::varlena::varsize_any(raw), 64 + pg_sys::VARHDRSZ,);
            }
        });
    }

    #[pg_test]
    fn write_then_read_payload() {
        memcx::current_context(|cx| {
            let mut buf: VarlenaBuf = cx.alloc_varlena(16).expect("alloc");
            for (i, b) in buf.payload_mut().iter_mut().enumerate() {
                *b = i as u8;
            }
            for (i, &b) in buf.payload().iter().enumerate() {
                assert_eq!(b as usize, i);
            }
        });
    }

    #[pg_test]
    fn inspect_varlena_roundtrip() {
        memcx::current_context(|cx| {
            let mut buf: VarlenaBuf = cx.alloc_varlena(11).expect("alloc");
            buf.payload_mut().copy_from_slice(b"hello world");
            let ptr = buf.into_raw();
            // SAFETY: `ptr` came from alloc_varlena above, still in `cx`, header valid.
            let view = unsafe { cx.inspect_varlena(ptr) };
            assert_eq!(view.payload(), b"hello world");
        });
    }
}
