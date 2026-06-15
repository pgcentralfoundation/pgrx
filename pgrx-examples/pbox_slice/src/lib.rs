//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! `pbox_slice` — demonstrates `MemCx::alloc_varlena` returning a
//! `PBox<RawVarlena>` directly to Postgres (no intermediate `Vec<u8>`),
//! plus the `PBox<[T]>` slice constructors used as in-context scratch buffers.

use pgrx::datum::varlena_buf::{RawVarlena, VarlenaBuf};
use pgrx::memcx::MemCx;
use pgrx::palloc::PBox;
use pgrx::prelude::*;

pgrx::pg_module_magic!(name, version);

/// Build a bytea of the requested length, filled with a `(i % 256)` pattern, allocated directly in the caller's `MemCx`. Returning `PBox<RawVarlena>` hands the varlena to Postgres without an extra Rust-side `Vec<u8>`.
#[pg_extern]
fn build_bytea<'mcx>(len: i32, cx: &MemCx<'mcx>) -> PBox<'mcx, RawVarlena> {
    let n = len.max(0) as usize;
    let mut buf: VarlenaBuf<'_> = cx.alloc_varlena(n).expect("OOM");
    for (i, b) in buf.payload_mut().iter_mut().enumerate() {
        *b = (i & 0xff) as u8;
    }
    buf.into_pbox()
}

/// Bytewise XOR of two byteas, truncated to the shorter length. Output is
/// allocated once at the exact final size — no intermediate `Vec<u8>`.
#[pg_extern]
fn xor_bytea<'mcx>(a: &[u8], b: &[u8], cx: &MemCx<'mcx>) -> PBox<'mcx, RawVarlena> {
    let len = a.len().min(b.len());
    let mut buf = cx.alloc_varlena(len).expect("OOM");
    let dst = buf.payload_mut();
    for i in 0..len {
        dst[i] = a[i] ^ b[i];
    }
    buf.into_pbox()
}

/// Pack a SQL `int[]` into a little-endian binary bytea (4 bytes per element).
/// NULLs are encoded as zero. Demonstrates exact-size varlena allocation and in-place structured writes for binary-protocol-style serialization.
#[pg_extern]
fn pack_i32_array<'mcx>(arr: Array<'_, i32>, cx: &MemCx<'mcx>) -> PBox<'mcx, RawVarlena> {
    let n = arr.len();
    let mut buf = cx.alloc_varlena(n * 4).expect("OOM");
    let dst = buf.payload_mut();
    for (i, v) in arr.iter().enumerate() {
        let bytes = v.unwrap_or(0).to_le_bytes();
        dst[i * 4..(i + 1) * 4].copy_from_slice(&bytes);
    }
    buf.into_pbox()
}

/// Apply `rounds` of a byte-rotating transformation to `data`. Uses a `PBox<[u8]>` scratch buffer allocated in `cx` (not a Rust `Vec<u8>`), then copies the final state into a `VarlenaBuf` for return. Demonstrates pairing the slice and varlena APIs in one function.
#[pg_extern]
fn hash_chain<'mcx>(data: &[u8], rounds: i32, cx: &MemCx<'mcx>) -> PBox<'mcx, RawVarlena> {
    let rounds = rounds.max(0) as u32;
    let mut scratch: PBox<[u8]> = PBox::from_slice_in(data, cx).expect("OOM");
    for _ in 0..rounds {
        for b in scratch.iter_mut() {
            *b = b.wrapping_add(1);
        }
    }
    let mut out = cx.alloc_varlena(scratch.len()).expect("OOM");
    out.payload_mut().copy_from_slice(&scratch);
    out.into_pbox()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_build_bytea_length() {
        let r = Spi::get_one::<i32>("SELECT length(build_bytea(1024))");
        assert_eq!(r, Ok(Some(1024)));
    }

    #[pg_test]
    fn test_build_bytea_pattern() {
        // First 4 bytes of build_bytea(4) should be 0x00, 0x01, 0x02, 0x03.
        let r = Spi::get_one::<Vec<u8>>("SELECT build_bytea(4)::bytea");
        assert_eq!(r, Ok(Some(vec![0u8, 1, 2, 3])));
    }

    #[pg_test]
    fn test_build_bytea_zero_length() {
        let r = Spi::get_one::<i32>("SELECT length(build_bytea(0))");
        assert_eq!(r, Ok(Some(0)));
    }

    #[pg_test]
    fn test_xor_bytea_basic() {
        // 0xFF XOR 0x0F = 0xF0; 0xAA XOR 0x55 = 0xFF.
        let r = Spi::get_one::<Vec<u8>>(r"SELECT xor_bytea('\xffaa'::bytea, '\x0f55'::bytea)");
        assert_eq!(r, Ok(Some(vec![0xf0u8, 0xff])));
    }

    #[pg_test]
    fn test_xor_bytea_truncates_to_shorter() {
        // 3-byte input XOR 5-byte input yields 3-byte output.
        let r = Spi::get_one::<i32>(
            r"SELECT length(xor_bytea('\x010203'::bytea, '\x0405060708'::bytea))",
        );
        assert_eq!(r, Ok(Some(3)));
    }

    #[pg_test]
    fn test_xor_bytea_self_is_zero() {
        let r =
            Spi::get_one::<Vec<u8>>(r"SELECT xor_bytea('\xdeadbeef'::bytea, '\xdeadbeef'::bytea)");
        assert_eq!(r, Ok(Some(vec![0u8, 0, 0, 0])));
    }

    #[pg_test]
    fn test_pack_i32_array_length() {
        let r = Spi::get_one::<i32>("SELECT length(pack_i32_array(ARRAY[1,2,3,4]::int[]))");
        assert_eq!(r, Ok(Some(16))); // 4 elems * 4 bytes
    }

    #[pg_test]
    fn test_pack_i32_array_little_endian() {
        // 1_i32 in LE = 01 00 00 00; 256_i32 in LE = 00 01 00 00.
        let r = Spi::get_one::<Vec<u8>>("SELECT pack_i32_array(ARRAY[1, 256]::int[])::bytea");
        assert_eq!(r, Ok(Some(vec![1u8, 0, 0, 0, 0, 1, 0, 0])));
    }

    #[pg_test]
    fn test_pack_i32_array_null_as_zero() {
        let r = Spi::get_one::<Vec<u8>>("SELECT pack_i32_array(ARRAY[NULL, 1]::int[])::bytea");
        assert_eq!(r, Ok(Some(vec![0u8, 0, 0, 0, 1, 0, 0, 0])));
    }

    #[pg_test]
    fn test_hash_chain_zero_rounds_is_identity() {
        let r = Spi::get_one::<Vec<u8>>(r"SELECT hash_chain('\x010203'::bytea, 0)");
        assert_eq!(r, Ok(Some(vec![1u8, 2, 3])));
    }

    #[pg_test]
    fn test_hash_chain_adds_rounds() {
        // Each round increments every byte. 5 rounds on [0x00] = [0x05].
        let r = Spi::get_one::<Vec<u8>>(r"SELECT hash_chain('\x00'::bytea, 5)");
        assert_eq!(r, Ok(Some(vec![5u8])));
    }

    #[pg_test]
    fn test_hash_chain_wraps() {
        // 256 rounds on any byte returns the same byte (wrap_add).
        let r = Spi::get_one::<Vec<u8>>(r"SELECT hash_chain('\xab'::bytea, 256)");
        assert_eq!(r, Ok(Some(vec![0xabu8])));
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
