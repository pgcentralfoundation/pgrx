use __pgrx_c_macros::{CInteger, CLong, CLongLong, CUnsignedLong, CUnsignedLongLong, CValue};
use std::cell::Cell;

unsafe extern "C" {
    fn handport_reset_backend_calls();
    fn handport_backend_calls() -> u32;
}

fn record<T: __pgrx_c_macros::IntoCValue>(name: &str, index: u32, value: T, first: u32, second: u32) {
    let value = value.into_c_value();
    let kind = std::any::type_name::<T::Kind>().rsplit("::").next().unwrap();
    println!(
        "{name}\t{index}\t{kind}\t{}\t{}\t{}\t{:032x}\t{first}\t{second}",
        T::Kind::BITS,
        u8::from(T::Kind::SIGNED),
        T::Kind::RANK,
        T::Kind::encode(value.get()),
    );
}

fn main() {
    let block_size = /* @BLOCK_SIZE@ */;
    println!("BLOCK_SIZE\t{block_size}");
    // @VARTAG_METADATA@
    let buffers = [i32::MIN, i32::MIN + 1, -4096, -2, -1, 0, 1, 2, 4096, i32::MAX - 1, i32::MAX];
    for (index, buffer) in buffers.into_iter().enumerate() {
        record("BufferIsLocal_BOUNDARY", index as u32, BufferIsLocal!(buffer), 0, 0);
    }
    let transactions = [0_u32, 1, 2, 3, 4, 0x7FFFFFFF, 0x80000000, 0xFFFFFFFE, 0xFFFFFFFF];
    for (index, xid) in transactions.into_iter().enumerate() {
        record("TransactionIdIsNormal_BOUNDARY", index as u32, TransactionIdIsNormal!(xid), 0, 0);
    }
    let mut seed = 0xDECAFBAD_u32;
    for index in 0..4096 {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        record("BufferIsLocal_SAMPLE", index, BufferIsLocal!(seed as i32), 0, 0);
        record("TransactionIdIsNormal_SAMPLE", index, TransactionIdIsNormal!(seed), 0, 0);
    }
    for (index, alignment) in [1_i32, 2, 4, 8, 16, 32, 64].into_iter().enumerate() {
        for length in 0_u32..1024 {
            record("TYPEALIGN_SAMPLE", index as u32 * 1024 + length, TYPEALIGN!(alignment, length), 0, 0);
        }
    }
    for length in 0_u32..1024 {
        record("MAXALIGN_SAMPLE", length, MAXALIGN!(length), 0, 0);
    }

    let maximum = CValue::<CUnsignedLong>::new(u64::MAX);
    record("TYPEALIGN_MAX", 0, TYPEALIGN!(8_i32, maximum), 0, 0);
    record("TYPEALIGN_NEAR_MAX", 0, TYPEALIGN!(8_i32, CValue::<CUnsignedLong>::new(u64::MAX - 1)), 0, 0);
    record("TYPEALIGN_NEGATIVE_LENGTH", 0, TYPEALIGN!(8_i32, -1_i32), 0, 0);
    record("TYPEALIGN_MINIMUM_LENGTH", 0, TYPEALIGN!(8_i32, i32::MIN), 0, 0);
    record("TYPEALIGN_ZERO_ALIGNMENT", 0, TYPEALIGN!(0_i32, 13_i32), 0, 0);
    record("TYPEALIGN_NEGATIVE_ALIGNMENT", 0, TYPEALIGN!(-1_i32, 13_i32), 0, 0);
    record("TYPEALIGN_SIGNED_CHAR", 0, TYPEALIGN!(8_i8, 255_u8), 0, 0);
    record("TYPEALIGN_UNSIGNED_ALIGNMENT", 0, TYPEALIGN!(8_u32, maximum), 0, 0);
    record("TYPEALIGN_LONG_ALIGNMENT", 0, TYPEALIGN!(CValue::<CLong>::new(8), maximum), 0, 0);
    record("TYPEALIGN_LONG_LONG_ALIGNMENT", 0, TYPEALIGN!(CValue::<CLongLong>::new(8), maximum), 0, 0);
    record("TYPEALIGN_UNSIGNED_LONG_LONG_ALIGNMENT", 0, TYPEALIGN!(CValue::<CUnsignedLongLong>::new(8), maximum), 0, 0);
    record("TYPEALIGN_UNSIGNED_BOUNDARY_ALIGNMENT", 0, TYPEALIGN!(u32::MAX, 13_i32), 0, 0);
    record("MAXALIGN_MAX", 0, MAXALIGN!(maximum), 0, 0);
    record("MAXALIGN_NEGATIVE", 0, MAXALIGN!(-1_i32), 0, 0);
    record("BufferIsLocal_UNSIGNED_LITERAL", 0, BufferIsLocal!(0x80000000_u32), 0, 0);
    record("TransactionIdIsNormal_SIGNED_LITERAL", 0, TransactionIdIsNormal!(-1_i32), 0, 0);
    record("TransactionIdIsNormal_LONG_LITERAL", 0, TransactionIdIsNormal!(CValue::<CLong>::new(-1)), 0, 0);
    record("TransactionIdIsNormal_LONG_LONG_LITERAL", 0, TransactionIdIsNormal!(CValue::<CLongLong>::new(-1)), 0, 0);
    // @WRAPPING_CASES@

    let first = Cell::new(0_u32);
    let second = Cell::new(0_u32);
    let alignment_argument = || { first.set(first.get() + 1); 8_i32 };
    let length_argument = || { second.set(second.get() + 1); CValue::<CUnsignedLong>::new(13) };
    let buffer_argument = || { first.set(first.get() + 1); -7_i32 };
    let transaction_argument = || { first.set(first.get() + 1); 3_u32 };
    let value = TYPEALIGN!(alignment_argument(), length_argument());
    record("TYPEALIGN_EVAL", 0, value, first.get(), second.get());
    first.set(0); second.set(0);
    let value = MAXALIGN!(length_argument());
    record("MAXALIGN_EVAL", 0, value, first.get(), second.get());
    first.set(0); second.set(0);
    let value = BufferIsLocal!(buffer_argument());
    record("BufferIsLocal_EVAL", 0, value, first.get(), second.get());
    first.set(0); second.set(0);
    let value = TransactionIdIsNormal!(transaction_argument());
    record("TransactionIdIsNormal_EVAL", 0, value, first.get(), second.get());
    // @PAGE_SIZE_CASES@
    // @TRIGGER_CASES@
    // @VARTAG_CASES@
    first.set(0); second.set(0);
    for ch in 0_i32..128 {
        record("PGSIXBIT_ASCII", ch as u32, PGSIXBIT!(ch), 0, 0);
    }
    for ch in 0_u8..128 {
        record("PGSIXBIT_BYTE_ASCII", u32::from(ch), PGSIXBIT!(ch), 0, 0);
    }
    record("PGSIXBIT_NEGATIVE_INT", 0, PGSIXBIT!(-1_i32), 0, 0);
    record("PGSIXBIT_NEGATIVE_BYTE", 0, PGSIXBIT!(-128_i8), 0, 0);
    record("PGSIXBIT_UNSIGNED_BYTE", 0, PGSIXBIT!(255_u8), 0, 0);
    record("PGSIXBIT_UNSIGNED_ZERO", 0, PGSIXBIT!(0_u32), 0, 0);
    record("PGSIXBIT_UNSIGNED_MAX", 0, PGSIXBIT!(u32::MAX), 0, 0);
    record("PGSIXBIT_CHARACTER_LITERAL", 0, PGSIXBIT!(48_i32), 0, 0);
    record("PGSIXBIT_LONG_LITERAL", 0, PGSIXBIT!(CValue::<CLong>::new(-1)), 0, 0);
    seed = 0xA5E130C7;
    for index in 0..4096 {
        let mut characters = [0_i32; 5];
        for character in &mut characters {
            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
            *character = ((seed >> 24) & 0x7F) as i32;
        }
        record("MAKE_SQLSTATE_SAMPLE", index,
            MAKE_SQLSTATE!(characters[0], characters[1], characters[2], characters[3], characters[4]), 0, 0);
    }
    record("MAKE_SQLSTATE_BYTES", 0, MAKE_SQLSTATE!(0_u8, 48_u8, 65_u8, 90_u8, 127_u8), 0, 0);
    record("MAKE_SQLSTATE_SIGNED", 0, MAKE_SQLSTATE!(-1_i32, -128_i32, -48_i32, 48_i32, 127_i32), 0, 0);
    record("MAKE_SQLSTATE_UNSIGNED", 0, MAKE_SQLSTATE!(0_u32, u32::MAX, 0x80000000_u32, 48_u32, 127_u32), 0, 0);
    record("MAKE_SQLSTATE_MIXED", 0, MAKE_SQLSTATE!(-1_i32, 48_u8, CValue::<CLong>::new(65), 90_i32, u32::MAX), 0, 0);
    record("MAKE_SQLSTATE_CHARACTER_LITERALS", 0, MAKE_SQLSTATE!(50_i32, 50_i32, 48_i32, 49_i32, 50_i32), 0, 0);
    let sixbit_argument = || { first.set(first.get() + 1); 48_i32 };
    let value = PGSIXBIT!(sixbit_argument());
    record("PGSIXBIT_EVAL", 0, value, first.get(), second.get());
    first.set(0); second.set(0);
    let sqlstate_evaluations = std::array::from_fn::<_, 5, _>(|_| Cell::new(0_u32));
    let sqlstate_argument = |index: usize| {
        let evaluations = &sqlstate_evaluations[index];
        evaluations.set(evaluations.get() + 1);
        48_i32 + index as i32
    };
    let value = MAKE_SQLSTATE!(sqlstate_argument(0), sqlstate_argument(1), sqlstate_argument(2), sqlstate_argument(3), sqlstate_argument(4));
    record("MAKE_SQLSTATE_EVAL", 0, value, first.get(), second.get());
    println!("MAKE_SQLSTATE_ARGUMENT_EVAL\t{}\t{}\t{}\t{}\t{}",
        sqlstate_evaluations[0].get(), sqlstate_evaluations[1].get(), sqlstate_evaluations[2].get(), sqlstate_evaluations[3].get(), sqlstate_evaluations[4].get());

    first.set(0); second.set(0);
    let byte_input = 7_u8;
    let pointer = core::ptr::addr_of!(byte_input);
    // @PAGE_VALID_CASES@

    first.set(0); second.set(0);
    // SAFETY: Byte operands point to live initialized uint8-compatible storage.
    // The original macros only read it. The native backend stub accesses only
    // its own counter; integer inputs are converted by the generated C prototype.
    unsafe {
        for byte in 0_u32..256 {
            let mut input = byte as u8;
            record("VARATT_NOT_PAD_BYTE_EXHAUSTIVE", byte, VARATT_NOT_PAD_BYTE!(core::ptr::addr_of_mut!(input)), 0, 0);
        }
        let constant_input = 255_u8;
        record("VARATT_NOT_PAD_BYTE_CONST", 0, VARATT_NOT_PAD_BYTE!(core::ptr::addr_of!(constant_input)), 0, 0);
        let byte_argument = || { first.set(first.get() + 1); pointer };
        let value = VARATT_NOT_PAD_BYTE!(byte_argument());
        record("VARATT_NOT_PAD_BYTE_EVAL", 0, value, first.get(), second.get());

        for (index, input) in [0_u32, 1, 2, 3, u32::MAX].into_iter().enumerate() {
            handport_reset_backend_calls();
            let value = type_is_array!(input);
            record("type_is_array_BOUNDARY", index as u32, value, 0, handport_backend_calls());
        }
        handport_reset_backend_calls();
        let value = type_is_array!(-1_i32);
        record("type_is_array_SIGNED", 0, value, 0, handport_backend_calls());
        handport_reset_backend_calls();
        let value = type_is_array!(CValue::<CLong>::new(-1));
        record("type_is_array_LONG", 0, value, 0, handport_backend_calls());
        handport_reset_backend_calls();
        let value = type_is_array!(CValue::<CLongLong>::new(-1));
        record("type_is_array_LONG_LONG", 0, value, 0, handport_backend_calls());
        handport_reset_backend_calls();
        let value = type_is_array!(u16::MAX);
        record("type_is_array_SHORT", 0, value, 0, handport_backend_calls());
        handport_reset_backend_calls();
        let value = type_is_array!(true);
        record("type_is_array_BOOL", 0, value, 0, handport_backend_calls());
        handport_reset_backend_calls();
        first.set(0);
        let type_argument = || { first.set(first.get() + 1); 3_u32 };
        let value = type_is_array!(type_argument());
        record("type_is_array_EVAL", 0, value, first.get(), handport_backend_calls());
    }
}
