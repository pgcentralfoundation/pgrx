//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use __pgrx_c_macros::{CInteger, CLong, CUnsignedLongLong, CValue};
use std::cell::Cell;

/// Print the observation format consumed by the paired oracle, retaining C kind and value
/// information rather than only the result.
fn record<T: __pgrx_c_macros::IntoCValue>(name: &str, value: T, trace: u32, calls: u32) {
    let value = value.into_c_value();
    let kind = std::any::type_name::<T::Kind>().rsplit("::").next().unwrap();
    println!(
        "{name}\t{kind}\t{}\t{:032x}\t{trace}\t{calls}",
        T::Kind::BITS,
        T::Kind::encode(value.get())
    );
}
/// Record a void-result observation and its effects without treating unit as an integer C
/// value.
fn record_void<T: __pgrx_c_macros::expression::IntoExpression<Value = ()>>(
    name: &str,
    value: T,
    trace: u32,
    calls: u32,
) {
    let () = value.into_expression();
    println!("{name}\tCVoid\t0\t00000000000000000000000000000000\t{trace}\t{calls}");
}

/// Exercise the generated definitions and print observations for the paired original-C oracle;
/// assertions cover cases with no scalar output.
fn main() {
    let trace = Cell::new(0_u32);
    let calls = Cell::new(0_u32);
    let tick = |event: u32, value: i32| {
        trace.set(trace.get() * 10 + event);
        calls.set(calls.get() + 1);
        value
    };
    let reset = || {
        trace.set(0);
        calls.set(0);
    };

    reset();
    EXPR_EMPTY!(undeclared_argument, invalid_function(other_undeclared_argument));
    EXPR_EMPTY_ZERO!();
    EXPR_EMPTY!(tick(9, 3), tick(8, 2));
    record("empty", CValue::<__pgrx_c_macros::CInt>::new(11), trace.get(), calls.get());

    reset();
    let value = EXPR_DROP_CONSTANT!(undeclared_argument);
    record_void("void_constant", value, trace.get(), calls.get());
    reset();
    let value = EXPR_DROP!(tick(1, -7));
    record_void("void_once", value, trace.get(), calls.get());
    reset();
    let value = EXPR_COMMA!(tick(1, -7), tick(2, 9));
    record("comma", value, trace.get(), calls.get());
    reset();
    let value = EXPR_THREE!(tick(1, -7), tick(2, 9), tick(3, -4));
    record("three", value, trace.get(), calls.get());
    reset();
    let value = EXPR_REPEAT!(tick(1, 9));
    record("repeat", value, trace.get(), calls.get());
    reset();
    let value = EXPR_DROP_THEN!(tick(1, -7), tick(2, 9));
    record("drop_then", value, trace.get(), calls.get());
    reset();
    let value = EXPR_COMMA!(tick(1, 5), u32::MAX);
    record("comma_unsigned", value, trace.get(), calls.get());
    reset();
    let value = EXPR_COMMA!(tick(1, 5), CValue::<CLong>::new(-1));
    record("comma_long", value, trace.get(), calls.get());
    reset();
    let value = EXPR_COMMA!(tick(1, 5), CValue::<CUnsignedLongLong>::new(u64::MAX));
    record("comma_ull", value, trace.get(), calls.get());
    reset();
    let value = EXPR_LAZY!(tick(1, 1), tick(2, -7), tick(3, 9));
    record("lazy_yes", value, trace.get(), calls.get());
    reset();
    let value = EXPR_LAZY!(tick(1, 0), tick(2, -7), tick(3, 9));
    record("lazy_no", value, trace.get(), calls.get());
    reset();
    let value = EXPR_LAZY_VOID!(tick(1, 1), tick(2, -7), tick(3, 9));
    record_void("lazy_void_yes", value, trace.get(), calls.get());
    reset();
    let value = EXPR_LAZY_VOID!(tick(1, 0), tick(2, -7), tick(3, 9));
    record_void("lazy_void_no", value, trace.get(), calls.get());
    reset();
    let value = EXPR_NESTED!(tick(1, -7), tick(2, 9), tick(3, -4));
    record("nested", value, trace.get(), calls.get());
    reset();
    let atomic_value = 16_i32;
    let value = EXPR_ATOMIC_POW2!(atomic_value);
    record("atomic_identifier", value, trace.get(), calls.get());
    let value = EXPR_ATOMIC_POW2!(15_i32);
    record("atomic_literal", value, trace.get(), calls.get());
    let value = EXPR_ATOMIC_POW2!((1_i32 + 3_i32));
    record("atomic_parenthesized", value, trace.get(), calls.get());
    let value = EXPR_ATOMIC_POW2!((-1_i32));
    record("atomic_signed_parenthesized", value, trace.get(), calls.get());
    let value = EXPR_ATOMIC_POW2!(0x80000000_u32);
    record("atomic_unsigned", value, trace.get(), calls.get());
    reset();
    let value = EXPR_STATEMENT!(tick(4, 7));
    record_void("statement", value, trace.get(), calls.get());
}
