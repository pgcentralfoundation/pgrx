//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! `pbox_slice` — a *teaching* extension showing how `MemCx::alloc_varlena`
//! and `PBox<RawVarlena>` enable zero-copy bytea outputs in real workloads.
//!
//! The example shape is a *minimal vector-embedding store*: pack/unpack
//! `float[]` to a binary `bytea` representation (4 bytes per dimension,
//! native endian — same convention as pgvector's `vector` type) and
//! provide the distance / arithmetic primitives a small RAG or k-NN
//! search application would actually run.
//!
//! For production vector workloads use [pgvector]; this crate exists to
//! demonstrate the `MemCx` / `PBox<RawVarlena>` API surface in code that
//! mirrors a real use case rather than a synthetic benchmark.
//!
//! [pgvector]: https://github.com/pgvector/pgvector

use pgrx::datum::varlena_buf::{RawVarlena, VarlenaBuf};
use pgrx::memcx::MemCx;
use pgrx::palloc::PBox;
use pgrx::prelude::*;

pgrx::pg_module_magic!(name, version);

const F32_BYTES: usize = core::mem::size_of::<f32>();

/// Smoke-test fixture: build a bytea of the requested length filled with a `(i % 256)` byte pattern. Useful for SQL-level smoke tests that don't need a real embedding shape.
#[pg_extern]
fn build_bytea<'mcx>(len: i32, cx: &MemCx<'mcx>) -> PBox<'mcx, RawVarlena> {
    let n = len.max(0) as usize;
    let mut buf: VarlenaBuf<'_> = cx.alloc_varlena(n).expect("OOM");
    for (i, b) in buf.payload_mut().iter_mut().enumerate() {
        *b = (i & 0xff) as u8;
    }
    buf.into_pbox()
}

/// Pack a SQL `real[]` into a binary embedding (4 bytes per dimension, native endian). NULL elements are encoded as `0.0`.
///
/// The output layout matches the convention used by pgvector's `vector` type and is what an external embedder service would typically write to disk for compact storage.
#[pg_extern]
fn embedding_pack<'mcx>(coords: Array<'mcx, f32>, cx: &MemCx<'mcx>) -> PBox<'mcx, RawVarlena> {
    let n = coords.len();
    let mut buf = cx.alloc_varlena(n.saturating_mul(F32_BYTES)).expect("OOM");
    let dst = buf.payload_mut();
    for (i, v) in coords.iter().enumerate() {
        let bytes = v.unwrap_or(0.0).to_ne_bytes();
        dst[i * F32_BYTES..(i + 1) * F32_BYTES].copy_from_slice(&bytes);
    }
    buf.into_pbox()
}

/// Number of dimensions stored in a packed embedding. Returns `-1` if the byte length is not a multiple of `sizeof(f32)`, which signals a corrupted or wrong-typed payload.
#[pg_extern]
fn embedding_dims(emb: &[u8]) -> i32 {
    if emb.len() % F32_BYTES != 0 { -1 } else { (emb.len() / F32_BYTES) as i32 }
}

/// Squared L2 (Euclidean) distance. Skips the sqrt because ordering by distance only needs the monotonic transform — the standard optimization in nearest-neighbour search.
///
/// Returns `NaN` if the two embeddings have different dimensions.
#[pg_extern]
fn embedding_l2_squared(a: &[u8], b: &[u8]) -> f64 {
    let (av, bv) = match decode_pair(a, b) {
        Some(p) => p,
        None => return f64::NAN,
    };

    av.iter()
        .zip(bv.iter())
        .map(|(&x, &y)| {
            let d = (x - y) as f64;
            d * d
        })
        .sum()
}

/// Dot product. For unit-length embeddings this equals cosine similarity.
/// Returns `NaN` on dimension mismatch.
#[pg_extern]
fn embedding_dot(a: &[u8], b: &[u8]) -> f64 {
    let (av, bv) = match decode_pair(a, b) {
        Some(p) => p,
        None => return f64::NAN,
    };
    av.iter().zip(bv.iter()).map(|(&x, &y)| x as f64 * y as f64).sum()
}

/// Cosine similarity (dot product over product of magnitudes). Returns `NaN` on dimension mismatch or when either embedding has zero norm.
#[pg_extern]
fn embedding_cosine(a: &[u8], b: &[u8]) -> f64 {
    let (av, bv) = match decode_pair(a, b) {
        Some(p) => p,
        None => return f64::NAN,
    };
    let mut dot = 0.0;
    let mut na = 0.0;
    let mut nb = 0.0;
    for (&x, &y) in av.iter().zip(bv.iter()) {
        let xf = x as f64;
        let yf = y as f64;
        dot += xf * yf;
        na += xf * xf;
        nb += yf * yf;
    }
    if na == 0.0 || nb == 0.0 { f64::NAN } else { dot / (na.sqrt() * nb.sqrt()) }
}

/// Element-wise mean of two equal-dimension embeddings. Useful for simple ensembling, smoothing, or computing centroids without a full aggregate. Returns a zero-length bytea on dimension mismatch.
#[pg_extern]
fn embedding_mean<'mcx>(a: &[u8], b: &[u8], cx: &MemCx<'mcx>) -> PBox<'mcx, RawVarlena> {
    let (av, bv) = match decode_pair(a, b) {
        Some(p) => p,
        None => return cx.alloc_varlena(0).expect("OOM").into_pbox(),
    };
    let n = av.len();
    let mut buf = cx.alloc_varlena(n.saturating_mul(F32_BYTES)).expect("OOM");
    let dst = buf.payload_mut();
    for i in 0..n {
        let m = (av[i] + bv[i]) * 0.5;
        dst[i * F32_BYTES..(i + 1) * F32_BYTES].copy_from_slice(&m.to_ne_bytes());
    }
    buf.into_pbox()
}

/// L2-normalize an embedding to unit length. Common preprocessing step that lets downstream cosine similarity be expressed as a plain dot product (faster). Returns a zero-length bytea if the input has zero norm or invalid layout.
#[pg_extern]
fn embedding_normalize<'mcx>(emb: &[u8], cx: &MemCx<'mcx>) -> PBox<'mcx, RawVarlena> {
    let v = match decode(emb) {
        Some(v) => v,
        None => return cx.alloc_varlena(0).expect("OOM").into_pbox(),
    };
    let norm: f32 = v.iter().map(|&x| (x as f64) * (x as f64)).sum::<f64>().sqrt() as f32;
    if norm == 0.0 {
        return cx.alloc_varlena(0).expect("OOM").into_pbox();
    }
    let mut buf = cx.alloc_varlena(v.len().saturating_mul(F32_BYTES)).expect("OOM");
    let dst = buf.payload_mut();
    for (i, &x) in v.iter().enumerate() {
        let n = x / norm;
        dst[i * F32_BYTES..(i + 1) * F32_BYTES].copy_from_slice(&n.to_ne_bytes());
    }
    buf.into_pbox()
}

/// Decode a packed embedding into a `Vec<f32>`, or `None` on invalid layout.
/// Copies element-by-element via `from_ne_bytes` because the bytea payload is only 1-byte aligned, so a zero-copy `&[f32]` reinterpret would be unsound.
fn decode(emb: &[u8]) -> Option<Vec<f32>> {
    if emb.len() % F32_BYTES != 0 {
        return None;
    }
    let n = emb.len() / F32_BYTES;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut buf = [0u8; F32_BYTES];
        buf.copy_from_slice(&emb[i * F32_BYTES..(i + 1) * F32_BYTES]);
        out.push(f32::from_ne_bytes(buf));
    }
    Some(out)
}

fn decode_pair(a: &[u8], b: &[u8]) -> Option<(Vec<f32>, Vec<f32>)> {
    let av = decode(a)?;
    let bv = decode(b)?;
    if av.len() != bv.len() || av.is_empty() {
        return None;
    }
    Some((av, bv))
}

/// # Memory-context discipline for zero-copy varlena builders
///
/// 1. Final accumulator → `cx.alloc_varlena(...)` in the caller's MemCx; survives function return.
/// 2. Per-row scratch → `from_iter_in(.., child_cx)` against a child MemCx that we delete before returning. All scratch freed in O(1).
/// 3. The borrow checker enforces the split: the scratch `PBox<[f32]>` has lifetime `'child_mcx` (shorter than `'mcx`) and **cannot escape** into the return value. See the compile_fail block below.
///
/// ```compile_fail
/// // Pseudocode — what Rust rejects when you try to leak scratch:
/// fn cannot_leak<'mcx>(cx: &pgrx::memcx::MemCx<'mcx>)
///     -> pgrx::palloc::PBox<'mcx, [f32]>
/// {
///     unsafe {
///         let child = pgrx::PgMemoryContexts::new("scratch");
///         let _prev = pgrx::pg_sys::MemoryContextSwitchTo(child.value());
///         let smuggled = pgrx::memcx::current_context(|child_cx| {
///             pgrx::palloc::PBox::from_iter_in([1.0f32].into_iter(), child_cx)
///                 .unwrap()
///         });
///         pgrx::pg_sys::MemoryContextSwitchTo(_prev);
///         smuggled  // lifetime may not live long enough
///     }
/// }
/// ```
#[pg_extern]
fn embedding_seq_mean<'mcx>(
    sequence: &[u8],
    dims: i32,
    cx: &MemCx<'mcx>,
) -> PBox<'mcx, RawVarlena> {
    let dims = dims.max(0) as usize;
    let stride = dims.saturating_mul(F32_BYTES);
    if stride == 0 || sequence.len() % stride != 0 {
        return cx.alloc_varlena(0).expect("OOM").into_pbox();
    }
    let n = sequence.len() / stride;
    if n == 0 {
        return cx.alloc_varlena(0).expect("OOM").into_pbox();
    }

    // Final accumulator — lives in caller's MemCx, returned to PG, alloc_varlena already zeroes the payload.
    let mut acc_buf = cx.alloc_varlena(stride).expect("OOM");

    // Scratch path — child memory context, deleted before return.
    //
    // Panic-safety: `child` (PgMemoryContexts::Owned) deletes the underlying context in its `Drop`, and the `CxRestoreGuard` below restores the previous `CurrentMemoryContext` in its `Drop`. The guard is declared AFTER `child`, so on unwind it drops first (LIFO) — first the parent CMC is restored, then the child context is deleted. Either way (normal return or panic) `CurrentMemoryContext` is left exactly as we found it.
    struct CxRestoreGuard {
        previous: pg_sys::MemoryContext,
    }
    impl Drop for CxRestoreGuard {
        fn drop(&mut self) {
            // SAFETY: `previous` was the value of CurrentMemoryContext at the time we switched away; restoring it cannot fail.
            unsafe {
                pg_sys::MemoryContextSwitchTo(self.previous);
            }
        }
    }

    let child = pgrx::PgMemoryContexts::new("embedding_seq_mean_scratch");
    let _restore = unsafe {
        let previous = pg_sys::MemoryContextSwitchTo(child.value());
        CxRestoreGuard { previous }
    };

    pgrx::memcx::current_context(|child_cx| {
        let inv_n = 1.0f32 / (n as f32);
        for i in 0..n {
            let row = &sequence[i * stride..(i + 1) * stride];

            // from_iter_in: build a PBox<[f32]> in child_cx straight from an ExactSizeIterator. No MaybeUninit  wrangling, no raw pointer writes.
            // This PBox cannot outlive child_cx — borrow checker  blocks any attempt to return it from the closure.
            let scratch: PBox<[f32]> = PBox::from_iter_in(
                (0..dims).map(|d| {
                    let off = d * F32_BYTES;
                    let mut buf = [0u8; F32_BYTES];
                    buf.copy_from_slice(&row[off..off + F32_BYTES]);
                    f32::from_ne_bytes(buf)
                }),
                child_cx,
            )
            .expect("OOM");

            // Borrow scratch (Deref<[f32]>) and accumulate into the output buffer (which lives in the OUTER cx).
            let acc_bytes = acc_buf.payload_mut();
            for (d, &v) in scratch.iter().enumerate() {
                let off = d * F32_BYTES;
                let mut buf = [0u8; F32_BYTES];
                buf.copy_from_slice(&acc_bytes[off..off + F32_BYTES]);
                let acc_v = f32::from_ne_bytes(buf) + v * inv_n;
                acc_bytes[off..off + F32_BYTES].copy_from_slice(&acc_v.to_ne_bytes());
            }
            // `scratch` falls out of scope here; its bytes stay in the child context until the bulk delete below.
        }
    });

    acc_buf.into_pbox()
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
    fn test_pack_dims() {
        let r = Spi::get_one::<i32>(
            "SELECT embedding_dims(embedding_pack(ARRAY[1.0, 2.0, 3.0]::real[]))",
        );
        assert_eq!(r, Ok(Some(3)));
    }

    #[pg_test]
    fn test_pack_byte_length() {
        // 3 dims × 4 bytes = 12.
        let r = Spi::get_one::<i32>("SELECT length(embedding_pack(ARRAY[1.0, 2.0, 3.0]::real[]))");
        assert_eq!(r, Ok(Some(12)));
    }

    #[pg_test]
    fn test_dims_invalid_layout() {
        // 5 bytes is not a multiple of 4 — invalid embedding.
        let r = Spi::get_one::<i32>(r"SELECT embedding_dims('\x0102030405'::bytea)");
        assert_eq!(r, Ok(Some(-1)));
    }

    #[pg_test]
    fn test_l2_self_distance_is_zero() {
        let r = Spi::get_one::<f64>(
            "SELECT embedding_l2_squared(
                embedding_pack(ARRAY[1.0, 2.0, 3.0]::real[]),
                embedding_pack(ARRAY[1.0, 2.0, 3.0]::real[]))",
        );
        assert_eq!(r, Ok(Some(0.0)));
    }

    #[pg_test]
    fn test_l2_known_distance() {
        // ||(0,0) - (3,4)||^2 = 9 + 16 = 25
        let r = Spi::get_one::<f64>(
            "SELECT embedding_l2_squared(
                embedding_pack(ARRAY[0.0, 0.0]::real[]),
                embedding_pack(ARRAY[3.0, 4.0]::real[]))",
        );
        assert_eq!(r, Ok(Some(25.0)));
    }

    #[pg_test]
    fn test_l2_dim_mismatch_is_nan() {
        let r = Spi::get_one::<f64>(
            "SELECT embedding_l2_squared(
                embedding_pack(ARRAY[1.0, 2.0]::real[]),
                embedding_pack(ARRAY[1.0, 2.0, 3.0]::real[]))",
        );
        assert!(r.unwrap().unwrap().is_nan());
    }

    #[pg_test]
    fn test_dot_product() {
        // (1,2,3) · (4,5,6) = 4 + 10 + 18 = 32
        let r = Spi::get_one::<f64>(
            "SELECT embedding_dot(
                embedding_pack(ARRAY[1.0, 2.0, 3.0]::real[]),
                embedding_pack(ARRAY[4.0, 5.0, 6.0]::real[]))",
        );
        assert_eq!(r, Ok(Some(32.0)));
    }

    #[pg_test]
    fn test_cosine_orthogonal_is_zero() {
        let r = Spi::get_one::<f64>(
            "SELECT embedding_cosine(
                embedding_pack(ARRAY[1.0, 0.0]::real[]),
                embedding_pack(ARRAY[0.0, 1.0]::real[]))",
        );
        assert_eq!(r, Ok(Some(0.0)));
    }

    #[pg_test]
    fn test_cosine_parallel_is_one() {
        let r = Spi::get_one::<f64>(
            "SELECT embedding_cosine(
                embedding_pack(ARRAY[1.0, 2.0, 3.0]::real[]),
                embedding_pack(ARRAY[2.0, 4.0, 6.0]::real[]))",
        );
        // floating point: equal direction → 1.0 within fp precision.
        let v = r.unwrap().unwrap();
        assert!((v - 1.0).abs() < 1e-6, "got {}", v);
    }

    #[pg_test]
    fn test_cosine_zero_vector_is_nan() {
        let r = Spi::get_one::<f64>(
            "SELECT embedding_cosine(
                embedding_pack(ARRAY[0.0, 0.0]::real[]),
                embedding_pack(ARRAY[1.0, 2.0]::real[]))",
        );
        assert!(r.unwrap().unwrap().is_nan());
    }

    #[pg_test]
    fn test_mean_simple() {
        // mean of (1,2) and (3,4) is (2,3) → packed as 8 bytes
        let r = Spi::get_one::<i32>(
            "SELECT length(embedding_mean(
                embedding_pack(ARRAY[1.0, 2.0]::real[]),
                embedding_pack(ARRAY[3.0, 4.0]::real[])))",
        );
        assert_eq!(r, Ok(Some(8)));
    }

    #[pg_test]
    fn test_mean_value_check() {
        // L2² of mean against expected (2,3) should be 0.
        let r = Spi::get_one::<f64>(
            "SELECT embedding_l2_squared(
                embedding_mean(
                    embedding_pack(ARRAY[1.0, 2.0]::real[]),
                    embedding_pack(ARRAY[3.0, 4.0]::real[])),
                embedding_pack(ARRAY[2.0, 3.0]::real[]))",
        );
        let v = r.unwrap().unwrap();
        assert!(v < 1e-6, "expected ~0, got {}", v);
    }

    #[pg_test]
    fn test_normalize_unit_length() {
        // After normalize, dot(v,v) ≈ 1.
        let r = Spi::get_one::<f64>(
            "SELECT embedding_dot(
                embedding_normalize(embedding_pack(ARRAY[3.0, 4.0]::real[])),
                embedding_normalize(embedding_pack(ARRAY[3.0, 4.0]::real[])))",
        );
        let v = r.unwrap().unwrap();
        assert!((v - 1.0).abs() < 1e-5, "got {}", v);
    }

    #[pg_test]
    fn test_normalize_zero_vector_returns_empty() {
        let r = Spi::get_one::<i32>(
            "SELECT length(embedding_normalize(embedding_pack(ARRAY[0.0, 0.0]::real[])))",
        );
        assert_eq!(r, Ok(Some(0)));
    }

    // ─── Memory-context demo tests ───────────────────────────────────────────

    #[pg_test]
    fn test_seq_mean_single_row_is_identity() {
        // n=1: mean of one embedding equals itself.
        let r = Spi::get_one::<f64>(
            "SELECT embedding_l2_squared(
                embedding_seq_mean(embedding_pack(ARRAY[1.0, 2.0, 3.0]::real[]), 3),
                embedding_pack(ARRAY[1.0, 2.0, 3.0]::real[]))",
        );
        let v = r.unwrap().unwrap();
        assert!(v < 1e-6, "expected ~0, got {}", v);
    }

    #[pg_test]
    fn test_seq_mean_two_rows() {
        // Concatenate (1,2) and (3,4); mean should be (2,3).
        let r = Spi::get_one::<f64>(
            "SELECT embedding_l2_squared(
                embedding_seq_mean(
                    embedding_pack(ARRAY[1.0, 2.0]::real[])
                    || embedding_pack(ARRAY[3.0, 4.0]::real[]),
                    2),
                embedding_pack(ARRAY[2.0, 3.0]::real[]))",
        );
        let v = r.unwrap().unwrap();
        assert!(v < 1e-6, "expected ~0, got {}", v);
    }

    #[pg_test]
    fn test_seq_mean_invalid_dims() {
        let r = Spi::get_one::<i32>(
            "SELECT length(embedding_seq_mean(embedding_pack(ARRAY[1.0, 2.0]::real[]), 0))",
        );
        assert_eq!(r, Ok(Some(0)));
    }

    #[pg_test]
    fn test_seq_mean_misaligned_input() {
        let r = Spi::get_one::<i32>(r"SELECT length(embedding_seq_mean('\x010203'::bytea, 2))");
        assert_eq!(r, Ok(Some(0)));
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
