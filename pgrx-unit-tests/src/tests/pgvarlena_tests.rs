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

    use core::mem::{align_of, size_of};
    use core::ptr;
    use pgrx::datum::PgVarlena;
    use pgrx::prelude::*;

    /// Fits a one-byte varlena header, whose payload starts one byte into a MAXALIGN allocation.
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[repr(C)]
    struct Align8 {
        a: f32,
        b: f32,
        c: i64,
    }

    /// Needs more alignment than palloc guarantees.
    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    #[repr(C, align(16))]
    struct Align16 {
        lanes: [f32; 4],
    }

    /// Too large for a one-byte header, so its payload starts four bytes into the allocation.
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[repr(C)]
    struct Large {
        words: [u64; 32],
    }

    fn assert_aligned<T>(value: &T) {
        assert_eq!(ptr::from_ref(value).addr() % align_of::<T>(), 0, "misaligned &T");
    }

    /// Read the payload of a varlena that `into_pg` produced.
    unsafe fn payload<T: Copy>(varlena: *mut pg_sys::varlena) -> T {
        // SAFETY: the caller passes a live varlena holding a T; the read has no alignment needs.
        unsafe {
            assert_eq!(pg_sys::VARSIZE_ANY_EXHDR!(varlena).get() as usize, size_of::<T>());
            pg_sys::VARDATA_ANY!(varlena).get().cast::<T>().read_unaligned()
        }
    }

    #[pg_test]
    fn test_pgvarlena_new_lends_aligned_references() {
        let mut short = PgVarlena::<Align8>::new();
        short.a = 42.0;
        short.c = -7;
        assert_aligned::<Align8>(&short);
        assert_aligned::<Align8>(short.as_mut());
        assert_eq!(*short, Align8 { a: 42.0, b: 0.0, c: -7 });

        let mut wide = PgVarlena::<Align16>::default();
        wide.lanes[3] = 1.5;
        assert_aligned::<Align16>(&wide);
        assert_eq!(wide.lanes, [0.0, 0.0, 0.0, 1.5]);

        let mut large = PgVarlena::<Large>::new();
        large.words[31] = u64::MAX;
        assert_aligned::<Large>(&large);
    }

    #[pg_test]
    fn test_pgvarlena_into_pg_writes_mutations_to_the_payload() {
        let mut short = PgVarlena::<Align8>::new();
        short.b = 2.5;
        let varlena = short.into_pg();
        // SAFETY: into_pg returned the live varlena allocated by new().
        unsafe {
            assert!(pg_sys::VARATT_IS_1B!(varlena).is_true());
            assert_eq!(payload::<Align8>(varlena), Align8 { a: 0.0, b: 2.5, c: 0 });
        }

        let mut large = PgVarlena::<Large>::new();
        large.words[0] = 11;
        let varlena = large.into_pg();
        // SAFETY: into_pg returned the live varlena allocated by new().
        unsafe {
            assert!(pg_sys::VARATT_IS_4B_U!(varlena).is_true());
            assert_eq!(payload::<Large>(varlena).words[0], 11);
        }
    }

    #[pg_test]
    fn test_pgvarlena_from_short_header_datum() {
        let mut short = PgVarlena::<Align8>::new();
        *short = Align8 { a: 1.0, b: 2.0, c: i64::MIN };
        let datum = short.into_datum().unwrap();
        // SAFETY: datum is the packed varlena that new() allocated for an Align8.
        let read = unsafe {
            assert!(pg_sys::VARATT_IS_1B!(datum.cast_mut_ptr::<pg_sys::varlena>()).is_true());
            PgVarlena::<Align8>::from_datum(datum)
        };
        assert_aligned::<Align8>(&read);
        assert_eq!(*read, Align8 { a: 1.0, b: 2.0, c: i64::MIN });
    }

    #[pg_test]
    fn test_pgvarlena_from_compressed_datum() {
        let mut large = PgVarlena::<Large>::new();
        large.words = [0x0102_0304_0506_0708; 32];
        let datum = large.into_datum().unwrap();
        // SAFETY: datum is a live varlena; PGLZ returns a compressed copy or NULL.
        let compressed =
            unsafe { pg_sys::toast_compress_datum(datum, pg_sys::TOAST_PGLZ_COMPRESSION as _) };
        assert!(!compressed.is_null(), "a repeating payload should compress");
        // SAFETY: compressed is a live compressed varlena holding a Large.
        let read = unsafe {
            assert!(
                pg_sys::VARATT_IS_COMPRESSED!(compressed.cast_mut_ptr::<pg_sys::varlena>())
                    .is_true()
            );
            PgVarlena::<Large>::from_datum(compressed)
        };
        assert_aligned::<Large>(&read);
        assert_eq!(read.words, [0x0102_0304_0506_0708; 32]);
    }

    #[pg_test]
    fn test_pgvarlena_copies_borrowed_datum_on_write() {
        // A four-byte-header varlena outside any PostgreSQL memory context.
        let mut storage = vec![0_u64; 1 + size_of::<Align16>() / size_of::<u64>()];
        let original = storage.as_mut_ptr().cast::<pg_sys::varlena>();
        // SAFETY: storage is 8-aligned and holds the header plus a zeroed Align16 payload.
        unsafe {
            pg_sys::SET_VARSIZE_4B!(original, (4 + size_of::<Align16>()) as i32).get();
        }
        // SAFETY: original is a live uncompressed varlena holding a valid Align16.
        let mut borrowed = unsafe { PgVarlena::<Align16>::from_datum(original.into()) };
        assert_aligned::<Align16>(&borrowed);
        borrowed.lanes[0] = 9.0;
        assert_aligned::<Align16>(borrowed.as_mut());

        let copy = borrowed.into_pg();
        assert_ne!(copy, original, "a mutated borrowed value needs its own varlena");
        // SAFETY: both varlenas are live and hold an Align16.
        unsafe {
            assert_eq!(payload::<Align16>(copy).lanes, [9.0, 0.0, 0.0, 0.0]);
            assert_eq!(payload::<Align16>(original), Align16::default());
        }
    }

    #[pg_test]
    fn test_pgvarlena_returns_unmodified_borrowed_datum() {
        let mut large = PgVarlena::<Large>::new();
        large.words[5] = 5;
        let datum = large.into_datum().unwrap();
        // SAFETY: datum is the live varlena allocated above for a Large.
        let read = unsafe { PgVarlena::<Large>::from_datum(datum) };
        assert_eq!(read.words[5], 5);
        assert_eq!(read.into_pg(), datum.cast_mut_ptr());
    }

    #[pg_test(error = "varlena payload of 4 bytes cannot hold a 16-byte value")]
    fn test_pgvarlena_rejects_short_payload() {
        let text = "abcd".into_datum().unwrap();
        // SAFETY: text is a live varlena; its payload is too short for an Align8.
        let _ = unsafe { PgVarlena::<Align8>::from_datum(text) };
    }
}
