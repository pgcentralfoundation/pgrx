//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

fn record(name: &str, pointer: *const core::ffi::c_char, length: usize, evaluations: u32) {
    print!("{name}\t");
    // SAFETY: Every caller supplies generated static literal storage and its
    // complete array extent, including the final zero; no bytes are mutated.
    let bytes = unsafe { core::slice::from_raw_parts(pointer.cast::<u8>(), length) };
    for byte in bytes { print!("{byte:02x}"); }
    println!("\t{evaluations}");
}
fn main() {
    let evaluations = std::cell::Cell::new(0_u32);
    let argument = |value| { evaluations.set(evaluations.get() + 1); value };
    // SAFETY: Native recorder functions only borrow static initialized strings;
    // each generated array access remains within its complete literal storage.
    unsafe {
        record("closed", STRING_CLOSED!().get(), 23, 0);
        record("escapes", STRING_ESCAPES!().get(), 9, 0);
        record("embedded", STRING_EMBEDDED!().get(), 5, 0);
        println!("size\t{}\t{}\t{}", STRING_SIZE!(@__pgrx_c_expression;).get(), STRING_DEREF_SIZE!(@__pgrx_c_expression; STRING_POINTER!()).get(), STRING_CHAR!(1_i32).get());
        for value in 0_i32..=1 {
            evaluations.set(0);
            let selected = STRING_SELECT!(argument(value)).get();
            let length = std::ffi::CStr::from_ptr(selected).to_bytes_with_nul().len();
            record("select", selected, length, evaluations.get());
            evaluations.set(0);
            let selected = STRING_REPEAT!(argument(value)).get();
            let length = std::ffi::CStr::from_ptr(selected).to_bytes_with_nul().len();
            record("repeat", selected, length, evaluations.get());
        }
        evaluations.set(0);
        let expected_line = line!() + 1;
        let checked = STRING_CHECK!(argument(0_i32));
        println!("check\t{}\t{}", checked.get(), evaluations.get());
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        evaluations.set(0);
        let checked = STRING_CHECK!(argument(1_i32));
        println!("check\t{}\t{}", checked.get(), evaluations.get());
        let expected_line = line!() + 1;
        STRING_LOCATION_NESTED!();
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        let expected_line = line!() + 1;
        let checked = STRING_LINE!();
        println!("line\t{}", i32::from(checked.get() == expected_line as i32));
        println!("file_size\t{}", i32::from(STRING_FILE_SIZE!(@__pgrx_c_expression;).get() as usize == file!().len() + 1));
        let expected_line = line!() + 1;
        STRING_DIAGNOSTIC!(identifier);
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        let expected_line = line!() + 1;
        STRING_DIAGNOSTIC_FORWARD!(* pointer);
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        let expected_line = line!() + 1;
        STRING_DIAGNOSTIC!(((left + right), third));
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        let expected_line = line!() + 1;
        STRING_DIAGNOSTIC!(left   +   right);
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        let expected_line = line!() + 1;
        STRING_DIAGNOSTIC!("a\\\"b");
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        let expected_line = line!() + 1;
        STRING_DIAGNOSTIC!(-17);
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        let expected_line = line!() + 1;
        STRING_DIAGNOSTIC!(- value);
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        let expected_line = line!() + 1;
        STRING_DIAGNOSTIC_NESTED!(identifier);
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        let expected_line = line!() + 1;
        STRING_DIAGNOSTIC_MULTI!(first, (* second));
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        let expected_line = line!() + 1;
        STRING_TEXT_FORMAL!(undefined);
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        evaluations.set(0);
        let expected_line = line!() + 1;
        STRING_OPERAND_ASSERT!(argument(0));
        println!("operand_assert\t{}", evaluations.get());
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
        evaluations.set(0);
        STRING_OPERAND_ASSERT!(argument(1));
        println!("operand_assert\t{}", evaluations.get());
    }
}
