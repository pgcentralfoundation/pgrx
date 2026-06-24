//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use crate::pglz::Strategy;
use pgrx::prelude::*;

mod pglz;

pgrx::pg_module_magic!(name, version);

fn parse_strategy(s: &str) -> Strategy {
    match s {
        "default" => Strategy::Default,
        "always" => Strategy::Always,
        other => error!("unknown strategy {other:?}: expected 'default' or 'always'"),
    }
}

/// Look up a relation's fully schema-qualified, properly-quoted name via
/// `regclass::text` (handles non-`public` schemas and identifiers that need
/// quoting).
fn qualified_relname(oid: pg_sys::Oid) -> Result<String, pgrx::spi::Error> {
    Spi::get_one_with_args::<String>("SELECT $1::regclass::text", &[oid.into()])?
        .ok_or(pgrx::spi::Error::InvalidPosition)
}

/// Return true if `(tbl, col)` resolves to a `bytea` column. Used so we can skip the `::text::bytea` round-trip cast for native bytea data, which would otherwise be sensitive to `bytea_output` session settings and add SQL-plan overhead.
fn is_bytea_column(tbl: pg_sys::Oid, col: &str) -> Result<bool, pgrx::spi::Error> {
    let atttypid: Option<pg_sys::Oid> = Spi::get_one_with_args(
        "SELECT atttypid FROM pg_attribute \
         WHERE attrelid = $1 AND attname = $2 AND NOT attisdropped",
        &[tbl.into(), col.into()],
    )?;
    Ok(atttypid == Some(pg_sys::BYTEAOID))
}

fn sample_sql(tbl_name: &str, q_col: &str, bytea: bool) -> String {
    let select_expr = if bytea { q_col.to_owned() } else { format!("{q_col}::text::bytea") };
    format!(
        "SELECT {select_expr} FROM {tbl_name} \
         WHERE {q_col} IS NOT NULL \
         ORDER BY random() LIMIT $1"
    )
}

/// Probe how PGLZ would handle a single bytea value.
#[pg_extern]
fn pglz_size(
    input: &[u8],
) -> TableIterator<
    'static,
    (name!(raw_bytes, i32), name!(compressed_bytes, i32), name!(ratio, f64), name!(accepted, bool)),
> {
    let raw = input.len() as i32;
    let (compressed, ratio, accepted) = match pglz::compress(input, Strategy::Default) {
        Ok(Some(c)) => {
            let clen = c.len() as i32;
            let r = if raw == 0 { 0.0 } else { clen as f64 / raw as f64 };
            (clen, r, true)
        }
        Ok(None) => (raw, 1.0, false),
        Err(_) => (raw, 1.0, false),
    };
    TableIterator::once((raw, compressed, ratio, accepted))
}

#[pg_extern]
fn pglz_analyze_column(
    tbl: pg_sys::Oid,
    col: &str,
    sample_size: default!(i32, 1000),
    strategy: default!(&str, "'default'"),
) -> Result<
    TableIterator<
        'static,
        (
            name!(sampled_rows, i32),
            name!(avg_raw_bytes, f64),
            name!(avg_compressed, f64),
            name!(avg_ratio, f64),
            name!(pct_accepted, f64),
            name!(pct_incompressible, f64),
            name!(est_savings_bytes, i64),
        ),
    >,
    pgrx::spi::Error,
> {
    let sample_size = sample_size.max(0);
    let strat = parse_strategy(strategy);
    let tbl_name = qualified_relname(tbl)?;
    let q_col = pgrx::spi::quote_identifier(col);
    let bytea = is_bytea_column(tbl, col)?;
    let sql = sample_sql(&tbl_name, &q_col, bytea);

    let mut total_raw: u128 = 0;
    let mut total_comp: u128 = 0;
    let mut accepted: u32 = 0;
    let mut incompressible: u32 = 0;
    let mut sampled: i32 = 0;

    Spi::connect(|client| -> Result<(), pgrx::spi::Error> {
        let tup = client.select(&sql, None, &[sample_size.into()])?;
        let mut scratch: Vec<u8> = Vec::new();
        for row in tup {
            let bytes: Option<Vec<u8>> = row.get(1).ok().flatten();
            let Some(bytes) = bytes else { continue };
            sampled += 1;
            total_raw += bytes.len() as u128;
            let cap = pglz::max_output(bytes.len());
            if scratch.capacity() < cap {
                scratch.reserve_exact(cap - scratch.len());
            }
            let spare = &mut scratch.spare_capacity_mut()[..cap];
            match pglz::compress_into(&bytes, spare, strat) {
                Ok(Some(n)) => {
                    accepted += 1;
                    total_comp += n as u128;
                }
                Ok(None) => {
                    incompressible += 1;
                    total_comp += bytes.len() as u128;
                }
                Err(_) => {
                    incompressible += 1;
                    total_comp += bytes.len() as u128;
                }
            }
        }
        Ok(())
    })?;

    let (avg_raw, avg_comp, avg_ratio) = if sampled > 0 {
        let ar = total_raw as f64 / sampled as f64;
        let ac = total_comp as f64 / sampled as f64;
        (ar, ac, if ar == 0.0 { 1.0 } else { ac / ar })
    } else {
        (0.0, 0.0, 1.0)
    };
    let pct_accepted = if sampled > 0 { accepted as f64 / sampled as f64 } else { 0.0 };
    let pct_incomp = if sampled > 0 { incompressible as f64 / sampled as f64 } else { 0.0 };

    let reltuples = unsafe {
        let rel = pg_sys::RelationIdGetRelation(tbl);
        let rt = if rel.is_null() {
            -1.0
        } else {
            let n = (*(*rel).rd_rel).reltuples as f64;
            pg_sys::RelationClose(rel);
            n
        };
        rt
    };
    let est_savings: i64 = if reltuples > 0.0 && avg_raw > avg_comp {
        ((avg_raw - avg_comp) * reltuples) as i64
    } else {
        0
    };

    Ok(TableIterator::once((
        sampled,
        avg_raw,
        avg_comp,
        avg_ratio,
        pct_accepted,
        pct_incomp,
        est_savings,
    )))
}

#[pg_extern]
fn pglz_ratio_histogram(
    tbl: pg_sys::Oid,
    col: &str,
    sample_size: default!(i32, 1000),
) -> Result<TableIterator<'static, (name!(bucket, String), name!(row_count, i32))>, pgrx::spi::Error>
{
    let sample_size = sample_size.max(0);
    let tbl_name = qualified_relname(tbl)?;
    let q_col = pgrx::spi::quote_identifier(col);
    let bytea = is_bytea_column(tbl, col)?;
    let sql = sample_sql(&tbl_name, &q_col, bytea);

    // Buckets: 0..5 = ratio ranges, 5 = incompressible.
    let mut counts = [0i32; 6];

    Spi::connect(|client| -> Result<(), pgrx::spi::Error> {
        let tup = client.select(&sql, None, &[sample_size.into()])?;
        let mut scratch: Vec<u8> = Vec::new();
        for row in tup {
            let bytes: Option<Vec<u8>> = row.get(1).ok().flatten();
            let Some(bytes) = bytes else { continue };
            let cap = pglz::max_output(bytes.len());
            if scratch.capacity() < cap {
                scratch.reserve_exact(cap - scratch.len());
            }
            let spare = &mut scratch.spare_capacity_mut()[..cap];
            match pglz::compress_into(&bytes, spare, Strategy::Default) {
                Ok(Some(n)) => {
                    let ratio = if bytes.is_empty() { 1.0 } else { n as f64 / bytes.len() as f64 };
                    let idx = ((ratio * 5.0).floor() as usize).min(4);
                    counts[idx] += 1;
                }
                _ => counts[5] += 1,
            }
        }
        Ok(())
    })?;

    let labels = ["0.0-0.2", "0.2-0.4", "0.4-0.6", "0.6-0.8", "0.8-1.0", "incompressible"];
    let rows: Vec<(String, i32)> =
        labels.iter().zip(counts.iter()).map(|(l, c)| ((*l).to_owned(), *c)).collect();
    Ok(TableIterator::new(rows.into_iter()))
}

const RECOMMEND_PCT_ACCEPTED: f64 = 0.80;
const RECOMMEND_AVG_RATIO: f64 = 0.70;
const SKIP_PCT_ACCEPTED: f64 = 0.30;
const SKIP_AVG_RATIO: f64 = 0.90;

#[pg_extern]
fn pglz_recommend(
    tbl: pg_sys::Oid,
    col: &str,
    sample_size: default!(i32, 1000),
) -> Result<String, pgrx::spi::Error> {
    let sample_size = sample_size.max(0);
    let tbl_name = qualified_relname(tbl)?;
    let q_col = pgrx::spi::quote_identifier(col);
    let bytea = is_bytea_column(tbl, col)?;
    let sql = sample_sql(&tbl_name, &q_col, bytea);

    let mut total_raw: u128 = 0;
    let mut total_comp: u128 = 0;
    let mut accepted: u32 = 0;
    let mut sampled: u32 = 0;

    Spi::connect(|client| -> Result<(), pgrx::spi::Error> {
        let tup = client.select(&sql, None, &[sample_size.into()])?;
        let mut scratch: Vec<u8> = Vec::new();
        for row in tup {
            let bytes: Option<Vec<u8>> = row.get(1).ok().flatten();
            let Some(bytes) = bytes else { continue };
            sampled += 1;
            total_raw += bytes.len() as u128;
            let cap = pglz::max_output(bytes.len());
            if scratch.capacity() < cap {
                scratch.reserve_exact(cap - scratch.len());
            }
            let spare = &mut scratch.spare_capacity_mut()[..cap];
            match pglz::compress_into(&bytes, spare, Strategy::Default) {
                Ok(Some(n)) => {
                    accepted += 1;
                    total_comp += n as u128;
                }
                _ => total_comp += bytes.len() as u128,
            }
        }
        Ok(())
    })?;

    if sampled == 0 {
        return Ok(format!("NO DATA: no non-null rows sampled from {tbl_name}.{col}"));
    }
    let pct_accepted = accepted as f64 / sampled as f64;
    let avg_ratio = if total_raw == 0 { 1.0 } else { total_comp as f64 / total_raw as f64 };
    let savings_pct = ((1.0 - avg_ratio) * 100.0).max(0.0);

    let ddl = if cfg!(not(feature = "pg13")) {
        format!("ALTER TABLE {tbl_name} ALTER COLUMN {q_col} SET COMPRESSION pglz;")
    } else {
        format!(
            "-- pg13: SET COMPRESSION unavailable; use a BEFORE INSERT trigger \
            that PGLZ-compresses {tbl_name}.{q_col} into a bytea sibling column."
        )
    };

    Ok(if pct_accepted >= RECOMMEND_PCT_ACCEPTED && avg_ratio <= RECOMMEND_AVG_RATIO {
        format!(
            "RECOMMEND: PGLZ saves ~{:.0}% on {tbl_name}.{col} \
             ({} of {} sampled rows accepted). Run: {ddl}",
            savings_pct, accepted, sampled
        )
    } else if pct_accepted < SKIP_PCT_ACCEPTED || avg_ratio > SKIP_AVG_RATIO {
        format!(
            "SKIP: only {:.0}% of sampled rows accepted, avg ratio {:.2} \
             — mostly incompressible.",
            pct_accepted * 100.0,
            avg_ratio
        )
    } else {
        format!(
            "MARGINAL: {:.0}% accepted, avg ratio {:.2} \
             (~{:.0}% savings). Decide based on workload.",
            pct_accepted * 100.0,
            avg_ratio,
            savings_pct
        )
    })
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::pglz::{self, PglzError, Strategy};
    use pgrx::prelude::*;

    #[pg_test]
    fn pglz_size_compresses_repetitive_input() {
        // 1024 bytes of repeating 'a' compresses well.
        let row = Spi::get_three::<i32, i32, f64>(
            "SELECT raw_bytes, compressed_bytes, ratio
             FROM pglz_size(repeat('a', 1024)::bytea)",
        )
        .expect("SPI failed");
        let (raw, compressed, ratio) = (row.0.unwrap(), row.1.unwrap(), row.2.unwrap());
        assert_eq!(raw, 1024);
        assert!(compressed < raw, "expected compression to shrink input");
        assert!(ratio < 0.5, "expected ratio < 0.5, got {}", ratio);
    }

    #[pg_test]
    fn pglz_size_rejects_small_input() {
        // 8 bytes is below PGLZ's min_input_size; expect accepted=false.
        let accepted =
            Spi::get_one::<bool>("SELECT accepted FROM pglz_size('\\x0102030405060708'::bytea)")
                .expect("SPI failed");
        assert_eq!(accepted, Some(false));
    }

    #[pg_test]
    fn pglz_size_handles_random_input() {
        // Random 4KB data: PGLZ may accept but ratio should be near 1.0.
        let ratio = Spi::get_one::<f64>(
            "SELECT ratio FROM pglz_size(
                 convert_to(
                     (SELECT string_agg(chr((random()*94+32)::int), '')
                      FROM generate_series(1, 4096)),
                     'SQL_ASCII')
             )",
        )
        .expect("SPI failed")
        .unwrap();
        assert!(ratio > 0.7, "random text should not compress well, got {}", ratio);
    }

    #[pg_test]
    fn analyze_column_on_repetitive_data() {
        Spi::run(
            "CREATE TABLE _t_analyze (v text);
             INSERT INTO _t_analyze
             SELECT repeat('hello world ', 200) FROM generate_series(1, 50);",
        )
        .unwrap();
        let row = Spi::get_three::<i32, f64, f64>(
            "SELECT sampled_rows, avg_ratio, pct_accepted
             FROM pglz_analyze_column('_t_analyze'::regclass, 'v', 50)",
        )
        .expect("SPI failed");
        let sampled = row.0.unwrap();
        let avg_ratio = row.1.unwrap();
        let pct_accepted = row.2.unwrap();
        assert_eq!(sampled, 50);
        assert!(avg_ratio < 0.3, "expected good compression, got {}", avg_ratio);
        assert!(pct_accepted >= 0.99, "expected all accepted, got {}", pct_accepted);
    }

    #[pg_test]
    fn recommend_says_yes_for_compressible_column() {
        Spi::run(
            "CREATE TABLE _t_rec_yes (v text);
             INSERT INTO _t_rec_yes
             SELECT repeat('compress_me_', 200) FROM generate_series(1, 50);",
        )
        .unwrap();
        let msg = Spi::get_one::<String>("SELECT pglz_recommend('_t_rec_yes'::regclass, 'v', 50)")
            .expect("SPI failed")
            .unwrap();
        assert!(msg.starts_with("RECOMMEND"), "got: {msg}");
    }

    #[pg_test]
    fn recommend_says_skip_for_random_column() {
        Spi::run(
            "CREATE TABLE _t_rec_no (v text);
             INSERT INTO _t_rec_no
             SELECT md5(i::text) || md5((i+1)::text)
             FROM generate_series(1, 50) i;",
        )
        .unwrap();
        let msg = Spi::get_one::<String>("SELECT pglz_recommend('_t_rec_no'::regclass, 'v', 50)")
            .expect("SPI failed")
            .unwrap();
        assert!(msg.starts_with("SKIP") || msg.starts_with("MARGINAL"), "got: {msg}");
    }

    #[pg_test]
    fn histogram_buckets_sum_to_sample_size() {
        Spi::run(
            "CREATE TABLE _t_hist (v text);
             INSERT INTO _t_hist
             SELECT CASE WHEN i % 2 = 0
                         THEN repeat('aaaaaaaa', 50)
                         ELSE md5(i::text) || md5((i+1)::text) END
             FROM generate_series(1, 40) i;",
        )
        .unwrap();
        let total: i64 = Spi::get_one::<i64>(
            "SELECT SUM(row_count)::bigint
             FROM pglz_ratio_histogram('_t_hist'::regclass, 'v', 40)",
        )
        .expect("SPI failed")
        .unwrap();
        assert_eq!(total, 40);
    }

    #[pg_test]
    fn empty_table_returns_no_data() {
        Spi::run("CREATE TABLE _t_empty (v text);").unwrap();
        let msg = Spi::get_one::<String>("SELECT pglz_recommend('_t_empty'::regclass, 'v', 10)")
            .expect("SPI failed")
            .unwrap();
        assert!(msg.starts_with("NO DATA"), "got: {msg}");

        let sampled = Spi::get_one::<i32>(
            "SELECT sampled_rows FROM pglz_analyze_column('_t_empty'::regclass, 'v', 10)",
        )
        .expect("SPI failed")
        .unwrap();
        assert_eq!(sampled, 0);
    }

    #[pg_test]
    fn analyze_column_handles_non_public_schema() {
        Spi::run(
            "CREATE SCHEMA _sch_test;
             CREATE TABLE _sch_test.t (v text);
             INSERT INTO _sch_test.t SELECT repeat('x', 100) FROM generate_series(1, 5);",
        )
        .unwrap();
        let sampled = Spi::get_one::<i32>(
            "SELECT sampled_rows FROM pglz_analyze_column('_sch_test.t'::regclass, 'v', 5)",
        )
        .expect("SPI failed")
        .unwrap();
        assert_eq!(sampled, 5);
    }

    #[pg_test]
    fn wrapper_roundtrip_default() {
        use crate::pglz::{self, Strategy};
        let src = b"abcd".repeat(256); // 1024 bytes, trivially compressible
        let c = pglz::compress(&src, Strategy::Default)
            .expect("compress should not fail")
            .expect("input should be accepted");
        assert!(c.len() < src.len(), "expected compression to shrink input");
        let back = pglz::decompress(&c, src.len(), true).expect("decompress should succeed");
        assert_eq!(back, src);
    }

    #[pg_test]
    fn wrapper_empty_input_is_rejected() {
        use crate::pglz::{self, Strategy};
        // Empty input is below PGLZ's min_input_size → rejected with Ok(None).
        assert!(matches!(pglz::compress(&[], Strategy::Default), Ok(None)));
        // Decompressing an empty input asking for 0 bytes must not UB.
        let out = pglz::decompress(&[], 0, true).expect("0-byte decompress should not crash");
        assert!(out.is_empty());
    }

    #[pg_test]
    fn wrapper_rejects_rawsize_above_i32_max() {
        use crate::pglz::{self, PglzError};
        let err = pglz::decompress(&[], usize::MAX, true).unwrap_err();
        assert_eq!(err, PglzError::InputTooLarge);
    }

    #[pg_test]
    fn into_roundtrip_default() {
        use crate::pglz::{self, Strategy};
        use std::mem::MaybeUninit;
        let src = b"hello world ".repeat(100); // 1200 bytes, very compressible
        let mut cbuf: Vec<MaybeUninit<u8>> =
            vec![MaybeUninit::uninit(); pglz::max_output(src.len())];
        let n = pglz::compress_into(&src, &mut cbuf, Strategy::Default)
            .expect("compress_into should not fail")
            .expect("input should be accepted");
        assert!(n < src.len());

        // SAFETY: PGLZ wrote `n` bytes into the prefix of cbuf.
        let cbytes: &[u8] = unsafe { std::slice::from_raw_parts(cbuf.as_ptr().cast::<u8>(), n) };
        let mut dbuf = vec![0u8; src.len()];
        let m = pglz::decompress_into(cbytes, &mut dbuf, src.len(), true)
            .expect("decompress_into should succeed");
        assert_eq!(m, src.len());
        assert_eq!(&dbuf[..m], &src[..]);
    }

    #[pg_test]
    fn into_compress_rejects_random_short_input() {
        use crate::pglz::{self, Strategy};
        use std::mem::MaybeUninit;
        // 12 high-entropy bytes — below min_input_size, PGLZ should refuse.
        let src: [u8; 12] =
            [0x91, 0xa2, 0xb3, 0xc4, 0xd5, 0xe6, 0xf7, 0x08, 0x19, 0x2a, 0x3b, 0x4c];
        let mut buf: Vec<MaybeUninit<u8>> =
            vec![MaybeUninit::uninit(); pglz::max_output(src.len())];
        let res = pglz::compress_into(&src, &mut buf, Strategy::Default).unwrap();
        assert!(res.is_none(), "expected PGLZ to reject random short input");
    }

    #[pg_test]
    fn into_decompress_rejects_corrupted_input() {
        use crate::pglz::{self, PglzError};
        let garbage = [0xffu8; 64];
        let mut out = vec![0u8; 256];
        let err = pglz::decompress_into(&garbage, &mut out, 256, false).unwrap_err();
        assert_eq!(err, PglzError::Decompress);
    }

    #[pg_test]
    fn into_strategy_always_succeeds_on_input_default_might_reject() {
        use crate::pglz::{self, Strategy};
        use std::mem::MaybeUninit;
        // 64 bytes of repeated 'a' — sits near PGLZ's min_comp_rate threshold for Default but Always should still attempt.
        let src = vec![b'a'; 64];
        let mut buf_a: Vec<MaybeUninit<u8>> =
            vec![MaybeUninit::uninit(); pglz::max_output(src.len())];
        let always = pglz::compress_into(&src, &mut buf_a, Strategy::Always)
            .expect("compress_into should not fail");
        assert!(always.is_some(), "Strategy::Always should accept highly compressible input");
        // The accepted output must round-trip.
        let n = always.unwrap();
        // SAFETY: PGLZ wrote `n` bytes into the prefix of buf_a.
        let cbytes: &[u8] = unsafe { std::slice::from_raw_parts(buf_a.as_ptr().cast::<u8>(), n) };
        let mut back = vec![0u8; src.len()];
        let m = pglz::decompress_into(cbytes, &mut back, src.len(), true)
            .expect("decompress_into should succeed for Strategy::Always output");
        assert_eq!(&back[..m], &src[..]);
    }

    #[pg_test]
    fn into_check_complete_flag_behaviour() {
        use crate::pglz::{self, PglzError, Strategy};
        use std::mem::MaybeUninit;
        let src = b"abcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd".to_vec();
        let mut cbuf: Vec<MaybeUninit<u8>> =
            vec![MaybeUninit::uninit(); pglz::max_output(src.len())];
        let n = pglz::compress_into(&src, &mut cbuf, Strategy::Default)
            .unwrap()
            .expect("should compress");

        // SAFETY: PGLZ wrote `n` bytes into the prefix of cbuf.
        let cbytes: &[u8] = unsafe { std::slice::from_raw_parts(cbuf.as_ptr().cast::<u8>(), n) };
        // Append a stray byte — with check_complete=true PGLZ must fail
        // (source not fully consumed).
        let mut padded = cbytes.to_vec();
        padded.push(0u8);
        let mut dbuf = vec![0u8; src.len()];
        let err = pglz::decompress_into(&padded, &mut dbuf, src.len(), true).unwrap_err();
        assert_eq!(err, PglzError::Decompress, "check_complete=true should reject trailing bytes");

        // Same input, check_complete=false → tolerated, payload still decodes.
        let m = pglz::decompress_into(&padded, &mut dbuf, src.len(), false)
            .expect("check_complete=false should tolerate trailing bytes");
        assert_eq!(&dbuf[..m], &src[..]);
    }

    #[pg_test]
    fn into_decompress_with_oversized_dest_buffer() {
        use crate::pglz::{self, Strategy};
        use std::mem::MaybeUninit;

        let src = b"xyzxyzxyzxyzxyzxyzxyzxyzxyzxyz".repeat(20); // 600 bytes
        let mut cbuf: Vec<MaybeUninit<u8>> =
            vec![MaybeUninit::uninit(); pglz::max_output(src.len())];
        let n = pglz::compress_into(&src, &mut cbuf, Strategy::Default)
            .unwrap()
            .expect("should compress");

        // SAFETY: PGLZ wrote `n` bytes into the prefix of cbuf.
        let cbytes: &[u8] = unsafe { std::slice::from_raw_parts(cbuf.as_ptr().cast::<u8>(), n) };
        // dest is 4 KiB but rawsize is the real 600. Pre-I1 this would have returned Err(Decompress) because rawsize was derived from dest.len().
        let mut big_scratch = vec![0u8; 4096];
        let m = pglz::decompress_into(cbytes, &mut big_scratch, src.len(), true)
            .expect("oversized dest should not be rejected");
        assert_eq!(m, src.len());
        assert_eq!(&big_scratch[..m], &src[..]);
    }

    #[pg_test]
    fn analyze_column_bytea_column() {
        // Binary 500-byte payload (raw bytes, not hex text). After the bytea
        // detection fix, analyze should see avg_raw ~= 500 (no text round-trip).
        Spi::run(
            "CREATE TABLE _t_bytea (b bytea);
             INSERT INTO _t_bytea
             SELECT decode(repeat('0011', 250), 'hex') FROM generate_series(1, 20);",
        )
        .unwrap();
        let row = Spi::get_two::<i32, f64>(
            "SELECT sampled_rows, avg_raw_bytes
             FROM pglz_analyze_column('_t_bytea'::regclass, 'b', 20)",
        )
        .expect("SPI failed");
        let (sampled, avg_raw) = (row.0.unwrap(), row.1.unwrap());
        assert_eq!(sampled, 20);
        assert!(
            (avg_raw - 500.0).abs() < 1.0,
            "avg_raw should be the native bytea length (500), got {avg_raw}"
        );
    }

    #[pg_test]
    fn analyze_column_unknown_strategy_errors() {
        Spi::run("CREATE TABLE _t_strat (v text); INSERT INTO _t_strat VALUES ('x');").unwrap();
        let res = std::panic::catch_unwind(|| {
            Spi::get_one::<i32>(
                "SELECT sampled_rows
                 FROM pglz_analyze_column('_t_strat'::regclass, 'v', 1, 'alwyas')",
            )
        });
        assert!(res.is_err(), "expected unknown strategy to raise PG ERROR");
    }

    #[pg_test]
    fn max_output_adds_four() {
        assert_eq!(pglz::max_output(0), 4);
        assert_eq!(pglz::max_output(1024), 1028);
        assert_eq!(pglz::max_output(usize::MAX - 4), usize::MAX);
    }

    #[pg_test]
    fn compress_into_rejects_undersized_buffer() {
        use std::mem::MaybeUninit;
        // max_output(1024) = 1028; a 1027-byte buffer must be rejected before any FFI call.
        let src = vec![0u8; 1024];
        let mut buf: Vec<MaybeUninit<u8>> = vec![MaybeUninit::uninit(); 1027];
        assert_eq!(
            pglz::compress_into(&src, &mut buf, Strategy::Default).unwrap_err(),
            PglzError::BufferTooSmall
        );
    }

    #[pg_test]
    fn decompress_into_rejects_undersized_buffer() {
        // rawsize > dest.len() must be rejected before any FFI call.
        let mut tiny = [0u8; 4];
        assert_eq!(
            pglz::decompress_into(&[], &mut tiny, 16, true).unwrap_err(),
            PglzError::BufferTooSmall
        );
    }

    #[pg_test]
    fn input_too_large_is_rejected() {
        // Pure validation path — no FFI.
        let dummy = [0u8; 4];
        let mut out = [0u8; 4];
        assert_eq!(
            pglz::decompress_into(&dummy, &mut out, usize::MAX, true).unwrap_err(),
            PglzError::InputTooLarge
        );
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
