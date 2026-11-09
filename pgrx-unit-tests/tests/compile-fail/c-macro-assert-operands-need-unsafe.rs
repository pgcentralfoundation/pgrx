use pgrx::pg_sys;

// An assert-enabled build evaluates the condition and a release build does not, but the
// operand reads through a raw pointer, so it needs `unsafe` in either build.
fn assert_through_a_pointer(pointer: *const i32) {
    let _ = pg_sys::Assert!(pointer.read());
}

// The same holds for an argument the macro never evaluates in any build.
fn unused_operand_through_a_pointer(pointer: *const pg_sys::TransactionId) {
    let _ = pg_sys::AssertTransactionIdInAllowableRange!(*pointer);
}

fn main() {}
