//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Test compiler builtin macros through the selected pgrx-pg-sys exports.
//!
//! Public invocations cover byte swaps and their C integer results using the
//! actual build's generated support, ensuring integration agrees with the
//! standalone oracle coverage.

#![cfg(all(pgrx_c_macros, not(docsrs)))]
#![deny(unsafe_op_in_unsafe_fn)]

/// Exercise the selected build's public bindings and macro exports from a downstream consumer.
use pgrx_pg_sys as pg;

// Evaluated through port/pg_bswap.h from the installed PostgreSQL headers under
// the compiler and flags recorded in this build's OUT_DIR macro report. The
// standalone C oracle calls no backend; these values do not come from pgrx ports.
/// Original-header byte-swap boundary vectors for this width, including endian-sensitive high
/// bits.
const CASES16: &[(u16, u16)] = &[
    (0x0000, 0x0000),
    (0x0001, 0x0100),
    (0xFFFF, 0xFFFF),
    (0x0102, 0x0201),
    (0x8000, 0x0080),
    (0x1234, 0x3412),
];
/// Original-header byte-swap boundary vectors for this width, including endian-sensitive high
/// bits.
const CASES32: &[(u32, u32)] = &[
    (0x00000000, 0x00000000),
    (0x00000001, 0x01000000),
    (0xFFFFFFFF, 0xFFFFFFFF),
    (0x01020304, 0x04030201),
    (0x80000000, 0x00000080),
    (0x01234567, 0x67452301),
];
/// Original-header byte-swap boundary vectors for this width, including endian-sensitive high
/// bits.
const CASES64: &[(u64, u64)] = &[
    (0x0000000000000000, 0x0000000000000000),
    (0x0000000000000001, 0x0100000000000000),
    (0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF),
    (0x0102030405060708, 0x0807060504030201),
    (0x8000000000000000, 0x0000000000000080),
    (0x0123456789ABCDEF, 0xEFCDAB8967452301),
];

// Profiles using PostgreSQL's static-inline fallback have no function-style
// macro to generate. Execute each API only when this build emitted that macro.
pg::__pgrx_c_classify! { @if_available pg_bswap16 {
/// Checks that generated swap16 matches original C and evaluates its operand once.
#[test]
fn generated_swap16_matches_original_c_and_evaluates_its_operand_once() {
    for &(input, expected) in CASES16 {
        let calls=core::cell::Cell::new(0_u32);
        let actual=pg::pg_bswap16!({calls.set(calls.get()+1); input}).get();
        assert_eq!(actual,expected,"input={input:#06X}");
        assert_eq!(calls.get(),1);
    }
}
} }
pg::__pgrx_c_classify! { @if_available pg_bswap32 {
/// Checks that generated swap32 matches original C and evaluates its operand once.
#[test]
fn generated_swap32_matches_original_c_and_evaluates_its_operand_once() {
    for &(input, expected) in CASES32 {
        let calls=core::cell::Cell::new(0_u32);
        let actual=pg::pg_bswap32!({calls.set(calls.get()+1); input}).get();
        assert_eq!(actual,expected,"input={input:#010X}");
        assert_eq!(calls.get(),1);
    }
}
} }
pg::__pgrx_c_classify! { @if_available pg_bswap64 {
/// Checks that generated swap64 matches original C and retains an explicit C input identity.
#[test]
fn generated_swap64_matches_original_c_and_retains_an_explicit_c_input_identity() {
    for &(input, expected) in CASES64 {
        let calls=core::cell::Cell::new(0_u32);
        // Native u64 alone cannot distinguish C unsigned long from long long.
        let value=pg::__pgrx_c_macros::CValue::<pg::__pgrx_c_macros::CUnsignedLongLong>::new(input);
        let actual=pg::pg_bswap64!({calls.set(calls.get()+1); value}).get();
        assert_eq!(actual,expected,"input={input:#018X}");
        assert_eq!(calls.get(),1);
    }
}
} }

pg::__pgrx_c_classify! { @if_available pg_hton16 {
pg::__pgrx_c_classify! { @if_available pg_ntoh16 {
/// Checks that generated network16 wrappers match C and invert each other.
#[test]
fn generated_network16_wrappers_match_c_and_invert_each_other() {
    for &(input, reversed) in CASES16 {
        let expected=if cfg!(target_endian="big") {input} else {reversed};
        assert_eq!(pg::pg_hton16!(input).get(),expected);
        assert_eq!(pg::pg_ntoh16!(input).get(),expected);
        assert_eq!(pg::pg_ntoh16!(pg::pg_hton16!(input)).get(),input);
    }
}
} }
} }
pg::__pgrx_c_classify! { @if_available pg_hton32 {
pg::__pgrx_c_classify! { @if_available pg_ntoh32 {
/// Checks that generated network32 wrappers match C and invert each other.
#[test]
fn generated_network32_wrappers_match_c_and_invert_each_other() {
    for &(input, reversed) in CASES32 {
        let expected=if cfg!(target_endian="big") {input} else {reversed};
        assert_eq!(pg::pg_hton32!(input).get(),expected);
        assert_eq!(pg::pg_ntoh32!(input).get(),expected);
        assert_eq!(pg::pg_ntoh32!(pg::pg_hton32!(input)).get(),input);
    }
}
} }
} }
pg::__pgrx_c_classify! { @if_available pg_hton64 {
pg::__pgrx_c_classify! { @if_available pg_ntoh64 {
/// Checks that generated network64 wrappers match C and invert each other.
#[test]
fn generated_network64_wrappers_match_c_and_invert_each_other() {
    for &(input, reversed) in CASES64 {
        let expected=if cfg!(target_endian="big") {input} else {reversed};
        let value=pg::__pgrx_c_macros::CValue::<pg::__pgrx_c_macros::CUnsignedLongLong>::new(input);
        assert_eq!(pg::pg_hton64!(value).get(),expected);
        assert_eq!(pg::pg_ntoh64!(value).get(),expected);
        assert_eq!(pg::pg_ntoh64!(pg::pg_hton64!(value)).get(),input);
    }
}
} }
} }

pg::__pgrx_c_classify! { @if_available DatumBigEndianToNative {
// PG19 routes this macro through guarded static-inline conversion functions.
// Their scalar behavior is tested against installed C headers in the transpiler
// oracle; invoking the production guards requires an initialized backend.
#[cfg(feature="pg19")]
#[allow(dead_code)] // Type-check the production expansion without entering a backend guard.
/// # Safety
/// Must run on the backend thread with PostgreSQL's error and memory-context
/// state initialized, under the generated native-call guard's contract.
unsafe fn generated_datum_endian_conversion_requires_backend(value: pg::Datum) -> u64 {
    // SAFETY: The caller establishes the backend guard's state/thread contract.
    // DatumGetUInt64 and UInt64GetDatum only cast this integer payload; neither
    // helper dereferences it, invokes a callback, allocates, or raises ERROR.
    unsafe { pg::DatumBigEndianToNative!(value).get() }
}
/// Checks that generated datum endian conversion matches original C scalar vectors.
#[cfg(all(target_pointer_width="64",not(feature="pg19")))]
#[test]
fn generated_datum_endian_conversion_matches_original_c_scalar_vectors() {
    for &(input, reversed) in CASES64 {
        let expected=if cfg!(target_endian="big") {input} else {reversed};
        // Integer Datums are never dereferenced or passed to a backend function.
        assert_eq!(pg::DatumBigEndianToNative!(pg::Datum::from(input)).get(),expected);
    }
}
/// Checks that generated datum endian conversion matches original C scalar vectors.
#[cfg(all(target_pointer_width="32",not(feature="pg19")))]
#[test]
fn generated_datum_endian_conversion_matches_original_c_scalar_vectors() {
    for &(input, reversed) in CASES32 {
        let expected=if cfg!(target_endian="big") {input} else {reversed};
        assert_eq!(pg::DatumBigEndianToNative!(pg::Datum::from(input)).get(),expected);
    }
}
} }
