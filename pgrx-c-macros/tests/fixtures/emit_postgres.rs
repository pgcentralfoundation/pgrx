use __pgrx_c_macros::{CInteger, CUnsignedLong, CValue};
use std::cell::Cell;

fn record<K: CInteger>(
    name: &str,
    alignment: u32,
    length: u32,
    value: CValue<K>,
    first: u32,
    second: u32,
) where
    K::Repr: std::fmt::Display,
{
    let kind = std::any::type_name::<K>().rsplit("::").next().unwrap();
    println!(
        "{name}\t{alignment}\t{length}\t{kind}\t{}\t{}\t{}\t{}\t{first}\t{second}",
        K::BITS,
        u8::from(K::SIGNED),
        K::RANK,
        value.get(),
    );
}

fn main() {
    for alignment in [1_i32, 2, 4, 8, 16, 32, 64] {
        for length in 0_u32..4096 {
            record("TYPEALIGN", alignment as u32, length, TYPEALIGN!(alignment, length), 0, 0);
            record(
                "TYPEALIGN_DOWN",
                alignment as u32,
                length,
                TYPEALIGN_DOWN!(alignment, length),
                0,
                0,
            );
            record("TYPEALIGN64", alignment as u32, length, TYPEALIGN64!(alignment, length), 0, 0);
        }
    }
    for length in 0_u32..4096 {
        record("MAXALIGN", 0, length, MAXALIGN!(length), 0, 0);
        record("MAXALIGN_DOWN", 0, length, MAXALIGN_DOWN!(length), 0, 0);
        record("MAXALIGN64", 0, length, MAXALIGN64!(length), 0, 0);
    }
    let maximum = CValue::<CUnsignedLong>::new(u64::MAX);
    record("TYPEALIGN_MAX", 8, 0, TYPEALIGN!(8_i32, maximum), 0, 0);
    record("TYPEALIGN_NEGATIVE", 8, 0, TYPEALIGN!(8_i32, -1_i32), 0, 0);
    record("TYPEALIGN_DOWN_MAX", 8, 0, TYPEALIGN_DOWN!(8_i32, maximum), 0, 0);
    record("TYPEALIGN64_MAX", 8, 0, TYPEALIGN64!(8_i32, maximum), 0, 0);
    record("MAXALIGN_MAX", 0, 0, MAXALIGN!(maximum), 0, 0);
    record("MAXALIGN64_MAX", 0, 0, MAXALIGN64!(maximum), 0, 0);

    for length in (0_u32..=65535).step_by(257) {
        record("OffsetNumberNext", 0, length, OffsetNumberNext!(length as u16), 0, 0);
        record("OffsetNumberPrev", 0, length, OffsetNumberPrev!(length as u16), 0, 0);
    }
    for length in 0_u32..256 {
        record("IS_HIGHBIT_SET", 0, length, IS_HIGHBIT_SET!(length as u8), 0, 0);
    }
    for length in 0_u32..=255 {
        let protocol = (length << 16) | (255 - length);
        record("PG_PROTOCOL_MAJOR", 0, length, PG_PROTOCOL_MAJOR!(protocol), 0, 0);
        record("PG_PROTOCOL_MINOR", 0, length, PG_PROTOCOL_MINOR!(protocol), 0, 0);
    }

    // Independent counters admit every C operand order while proving occurrences.
    let first = Cell::new(0_u32);
    let second = Cell::new(0_u32);
    let alignment_argument = || {
        first.set(first.get() + 1);
        8_i32
    };
    let length_argument = || {
        second.set(second.get() + 1);
        CValue::<CUnsignedLong>::new(13)
    };
    let offset_argument = || {
        first.set(first.get() + 1);
        41_u16
    };
    let first_transaction = || {
        first.set(first.get() + 1);
        42_u32
    };
    let second_transaction = || {
        second.set(second.get() + 1);
        42_u32
    };

    let value = TYPEALIGN!(alignment_argument(), length_argument());
    record("TYPEALIGN_EVAL", 8, 13, value, first.get(), second.get());
    first.set(0);
    second.set(0);
    let value = TYPEALIGN_DOWN!(alignment_argument(), length_argument());
    record("TYPEALIGN_DOWN_EVAL", 8, 13, value, first.get(), second.get());
    first.set(0);
    second.set(0);
    let value = TYPEALIGN64!(alignment_argument(), length_argument());
    record("TYPEALIGN64_EVAL", 8, 13, value, first.get(), second.get());
    first.set(0);
    second.set(0);
    let value = MAXALIGN!(length_argument());
    record("MAXALIGN_EVAL", 0, 13, value, first.get(), second.get());
    first.set(0);
    second.set(0);
    let value = MAXALIGN_DOWN!(length_argument());
    record("MAXALIGN_DOWN_EVAL", 0, 13, value, first.get(), second.get());
    first.set(0);
    second.set(0);
    let value = MAXALIGN64!(length_argument());
    record("MAXALIGN64_EVAL", 0, 13, value, first.get(), second.get());
    first.set(0);
    second.set(0);
    let value = OffsetNumberNext!(offset_argument());
    record("OffsetNumberNext_EVAL", 0, 41, value, first.get(), second.get());
    first.set(0);
    second.set(0);
    let value = OffsetNumberPrev!(offset_argument());
    record("OffsetNumberPrev_EVAL", 0, 41, value, first.get(), second.get());
    first.set(0);
    second.set(0);

    // @TRANSACTION_CASES@
    let _ = (first_transaction, second_transaction);
}
