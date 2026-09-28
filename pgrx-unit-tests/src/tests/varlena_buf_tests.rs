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

    /// Exercises `RawVarlena::header_size`'s short-header branch, which the 4-byte-emitting `alloc_varlena` never reaches on its own. We/ hand-craft a 1-byte (short) header buffer and verify `inspect_varlena`  reads the payload boundary correctly.
    #[pg_test]
    fn inspect_varlena_handles_short_header() {
        memcx::current_context(|cx| {
            // Short header encodes total size (header + payload) in 1 byte:
            // total fits in 7 bits => max 127 bytes total => max payload 126.
            const PAYLOAD_LEN: usize = 5;
            const TOTAL: usize = pg_sys::VARHDRSZ_SHORT + PAYLOAD_LEN;

            // Allocate raw bytes in cx, then set a 1-byte header by hand.
            let raw = cx
                .alloc_layout_zeroed(core::alloc::Layout::from_size_align(TOTAL, 1).unwrap())
                .expect("alloc");
            let varlena_ptr = raw.as_ptr() as *mut pg_sys::varlena;
            unsafe {
                // SAFETY: `raw` is `TOTAL` bytes, writable, exclusively ours until we expose it via `inspect_varlena`.
                pgrx::varlena::set_varsize_short(varlena_ptr, TOTAL as i32);
                // Write a recognizable payload pattern past the 1-byte header.
                let payload = raw.as_ptr().add(pg_sys::VARHDRSZ_SHORT);
                for i in 0..PAYLOAD_LEN {
                    payload.add(i).write(0xA0 | (i as u8));
                }
            }

            // SAFETY: header is now a valid in-line 1-byte header; pointee lives in `cx` so `'mcx`-bound borrow is sound.
            let view = unsafe { cx.inspect_varlena(varlena_ptr) };
            assert_eq!(view.total_size(), TOTAL);
            assert_eq!(view.payload().len(), PAYLOAD_LEN);
            assert_eq!(view.payload(), &[0xA0, 0xA1, 0xA2, 0xA3, 0xA4]);
        });
    }

    /// `inspect_varlena` must reject TOAST/external (1B_E) varlenas — they  reference out-of-line storage that `RawVarlena::payload` cannot safely expose. We hand-craft the external header byte and confirm the assertion fires.
    #[pg_test]
    fn inspect_varlena_rejects_external() {
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            memcx::current_context(|cx| {
                // VARTAG_EXTERNAL byte: first byte == 0x01 marks "1B_E".
                // We only need enough room for the tag byte; `inspect_varlena` panics before reading further bytes.
                let raw = cx
                    .alloc_layout_zeroed(core::alloc::Layout::from_size_align(16, 4).unwrap())
                    .expect("alloc");
                unsafe {
                    // 1B_E header byte: 0x01 on little-endian, 0x80 on big-endian.
                    #[cfg(target_endian = "little")]
                    let tag: u8 = 0x01;
                    #[cfg(target_endian = "big")]
                    let tag: u8 = 0x80;
                    raw.as_ptr().write(tag);
                }
                let varlena_ptr = raw.as_ptr() as *mut pg_sys::varlena;
                // SAFETY (panic expected): we deliberately violate the "in-line varlena" precondition to verify the runtime guard catches it.
                let _ = unsafe { cx.inspect_varlena(varlena_ptr) };
            });
        }));
        assert!(caught.is_err(), "expected panic on external varlena");
    }

    /// `inspect_varlena` must also reject 4-byte-headered *compressed* varlenas (VARATT_IS_4B_C). Same rationale: the payload bytes are notdirectly inspectable without decompression.
    #[pg_test]
    fn inspect_varlena_rejects_compressed() {
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            memcx::current_context(|cx| {
                let raw = cx
                    .alloc_layout_zeroed(core::alloc::Layout::from_size_align(16, 4).unwrap())
                    .expect("alloc");
                unsafe {
                    #[cfg(target_endian = "little")]
                    let header_word: u32 = (16u32 << 2) | 0b10;
                    #[cfg(target_endian = "big")]
                    let header_word: u32 = 0x4000_0000 | 16u32; // top 2 bits = 01
                    (raw.as_ptr() as *mut u32).write(header_word);
                }
                let varlena_ptr = raw.as_ptr() as *mut pg_sys::varlena;
                let _ = unsafe { cx.inspect_varlena(varlena_ptr) };
            });
        }));
        assert!(caught.is_err(), "expected panic on compressed varlena");
    }
}
