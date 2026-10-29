//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use core::ffi::CStr;
use pgrx::datum::PgVarlena;
use pgrx::prelude::*;
use pgrx::{PgVarlenaInOutFuncs, StringInfo};

/// Too large for a one-byte varlena header, so its Datums have the four-byte header that
/// detoasting leaves in place.
#[derive(Copy, Clone, Default, PostgresType)]
#[bikeshed_postgres_type_manually_impl_from_into_datum]
#[pgvarlena_inoutfuncs]
pub struct PgVarlenaCounter {
    count: i64,
    #[allow(dead_code, reason = "only sizes the type past a one-byte header")]
    filler: [i64; 25],
}

impl PgVarlenaInOutFuncs for PgVarlenaCounter {
    fn input(input: &CStr) -> PgVarlena<Self> {
        let mut counter = PgVarlena::<Self>::new();
        counter.count = input.to_str().expect("utf-8").parse().expect("an i64");
        counter
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(&self.count.to_string());
    }
}

/// Mutate an argument in each row, after the call that read it has returned.
#[pg_extern]
fn pgvarlena_counter_series(
    start: PgVarlena<PgVarlenaCounter>,
    rows: i32,
) -> SetOfIterator<'static, i64> {
    let mut counter = start;
    SetOfIterator::new((0..rows).map(move |_| {
        counter.count += 1;
        counter.count
    }))
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use super::PgVarlenaCounter;
    use core::mem::{align_of, size_of};
    use core::{ptr, slice};
    use pgrx::datum::PgVarlena;
    use pgrx::prelude::*;

    /// Fits a one-byte varlena header, whose payload starts one byte into a MAXALIGN allocation.
    #[derive(Copy, Clone, Debug, Default, PartialEq)]
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
    #[derive(Copy, Clone, Debug, Default, PartialEq)]
    #[repr(C)]
    struct Large {
        words: [u64; 32],
    }

    fn assert_aligned<T>(value: &T) {
        assert_eq!(ptr::from_ref(value).addr() % align_of::<T>(), 0, "misaligned &T");
    }

    /// Build a varlena in CurrentMemoryContext whose payload is `value` followed by `tail`.
    unsafe fn varlena_with_tail<T: Copy>(
        value: T,
        tail: &[u8],
        short_header: bool,
    ) -> *mut pg_sys::varlena {
        let len = size_of::<T>() + tail.len();
        // SAFETY: palloc0 returns aligned storage for the four-byte header and the payload. The
        // one-byte header form is only requested for payloads that fit it, and the copies fill
        // the payload that VARDATA_ANY locates from the header just written.
        unsafe {
            let varlena = pg_sys::palloc0(4 + len).cast::<pg_sys::varlena>();
            if short_header {
                pg_sys::SET_VARSIZE_SHORT!(varlena, (1 + len) as i32).get();
            } else {
                pg_sys::SET_VARSIZE_4B!(varlena, (4 + len) as i32).get();
            }
            let data = pg_sys::VARDATA_ANY!(varlena).get().cast::<u8>();
            ptr::copy_nonoverlapping(ptr::from_ref(&value).cast::<u8>(), data, size_of::<T>());
            ptr::copy_nonoverlapping(tail.as_ptr(), data.add(size_of::<T>()), tail.len());
            varlena
        }
    }

    /// The payload bytes of a live, uncompressed varlena.
    unsafe fn payload_bytes<'a>(varlena: *mut pg_sys::varlena) -> &'a [u8] {
        // SAFETY: the caller passes a live inline varlena, whose header gives its payload.
        unsafe {
            slice::from_raw_parts(
                pg_sys::VARDATA_ANY!(varlena).get().cast::<u8>(),
                pg_sys::VARSIZE_ANY_EXHDR!(varlena).get() as usize,
            )
        }
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

    /// `into_pg` allocates the varlena's size and no more, also when the header is one byte.
    #[pg_test]
    fn test_pgvarlena_into_pg_allocates_the_varlena_size() {
        // 126 bytes with a one-byte header. Sized for a four-byte header, it would be 129 bytes,
        // which the allocator rounds to a larger chunk than 126.
        let varlena = PgVarlena::from([7_u8; 125]).into_pg();
        // SAFETY: into_pg returned a live varlena, whose header gives its size.
        unsafe {
            assert!(pg_sys::VARATT_IS_1B!(varlena).is_true());
            let size = pg_sys::VARSIZE_ANY!(varlena).get() as usize;
            assert_eq!(size, 126);
            let same_size = pg_sys::palloc(size);
            assert_eq!(
                pg_sys::GetMemoryChunkSpace(varlena.cast()),
                pg_sys::GetMemoryChunkSpace(same_size)
            );
        }
    }

    #[pg_test]
    fn test_pgvarlena_never_writes_to_its_datum() {
        // A four-byte-header varlena outside any PostgreSQL memory context.
        let mut storage = vec![0_u64; 1 + size_of::<Align16>() / size_of::<u64>()];
        let original = storage.as_mut_ptr().cast::<pg_sys::varlena>();
        // SAFETY: storage is 8-aligned and holds the header plus a zeroed Align16 payload.
        unsafe {
            pg_sys::SET_VARSIZE_4B!(original, (4 + size_of::<Align16>()) as i32).get();
        }
        // SAFETY: original is a live uncompressed varlena holding a valid Align16.
        let mut read = unsafe { PgVarlena::<Align16>::from_datum(original.into()) };
        assert_aligned::<Align16>(&read);
        read.lanes[0] = 9.0;
        assert_aligned::<Align16>(read.as_mut());

        let copy = read.into_pg();
        assert_ne!(copy, original, "a mutated value needs its own varlena");
        // SAFETY: both varlenas are live and hold an Align16.
        unsafe {
            assert_eq!(payload::<Align16>(copy).lanes, [9.0, 0.0, 0.0, 0.0]);
            assert_eq!(payload::<Align16>(original), Align16::default());
        }
    }

    /// Payload bytes after the value survive a round trip, changed or not, whichever header
    /// the Datum had.
    #[pg_test]
    fn test_pgvarlena_keeps_payload_bytes_after_the_value() {
        let tail = [0xa1_u8, 0xb2, 0xc3];
        let value = Align8 { a: 1.0, b: 2.0, c: 3 };
        for short_header in [true, false] {
            // SAFETY: the payload is 19 bytes, which fits either header form.
            let original = unsafe { varlena_with_tail(value, &tail, short_header) };

            // SAFETY: original is a live inline varlena whose payload starts with an Align8.
            let unchanged = unsafe { PgVarlena::<Align8>::from_datum(original.into()) }.into_pg();
            // SAFETY: both varlenas are live and inline.
            unsafe { assert_eq!(payload_bytes(unchanged), payload_bytes(original)) };

            // SAFETY: as above.
            let mut read = unsafe { PgVarlena::<Align8>::from_datum(original.into()) };
            read.c = 4;
            let changed = read.into_pg();
            // SAFETY: into_pg returned a live inline varlena.
            let bytes = unsafe { payload_bytes(changed) };
            assert_eq!(&bytes[size_of::<Align8>()..], &tail, "short header: {short_header}");
            // SAFETY: the payload starts with an Align8; the read has no alignment needs.
            let changed_value = unsafe { bytes.as_ptr().cast::<Align8>().read_unaligned() };
            assert_eq!(changed_value, Align8 { c: 4, ..value });
        }
    }

    /// A value read through SPI and changed inside `Spi::connect` must survive `SPI_finish`,
    /// which deletes the memory context that was current during the change.
    #[pg_test]
    fn test_pgvarlena_changed_inside_spi_connect_outlives_it() -> Result<(), spi::Error> {
        let (dropped, kept) = Spi::connect(|client| {
            let read = || {
                client
                    .select("SELECT '41'::PgVarlenaCounter", None, &[])?
                    .first()
                    .get_one::<PgVarlena<PgVarlenaCounter>>()
                    .map(|counter| counter.expect("a non-NULL counter"))
            };
            let (mut dropped, mut kept) = (read()?, read()?);
            dropped.count += 1;
            kept.count += 2;
            Ok::<_, spi::Error>((dropped, kept))
        })?;
        assert_eq!(dropped.count, 42);
        drop(dropped);

        let datum = kept.into_datum().expect("a Datum");
        // Reuse and overwrite the memory of the deleted SPI procedure context.
        assert_eq!(
            Spi::get_one::<i64>("SELECT sum(i) FROM generate_series(1, 100) i")?,
            Some(5050)
        );
        // SAFETY: into_datum allocated this varlena holding a PgVarlenaCounter.
        let reread = unsafe { PgVarlena::<PgVarlenaCounter>::from_datum(datum) };
        assert_eq!(reread.count, 43);
        Ok(())
    }

    /// An SRF that changes a captured argument in each row runs those changes in the per-row
    /// memory context, which PostgreSQL resets between rows.
    #[pg_test]
    fn test_pgvarlena_argument_changed_by_srf_rows() -> Result<(), spi::Error> {
        let rows = Spi::get_one::<Vec<i64>>(
            "SELECT array_agg(n) FROM pgvarlena_counter_series('5'::PgVarlenaCounter, 3) n",
        )?;
        assert_eq!(rows, Some(vec![6, 7, 8]));
        Ok(())
    }

    #[pg_test(error = "varlena payload of 4 bytes cannot hold a 16-byte value")]
    fn test_pgvarlena_rejects_short_payload() {
        let text = "abcd".into_datum().unwrap();
        // SAFETY: text is a live varlena; its payload is too short for an Align8.
        let _ = unsafe { PgVarlena::<Align8>::from_datum(text) };
    }
}
