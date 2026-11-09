use pgrx::pg_sys;

// An assert-enabled build evaluates the condition and a release build does not, but the
// operand reads through a raw pointer, so it needs `unsafe` in either build.
fn assert_through_a_pointer(pointer: *const i32) {
    pg_sys::Assert!(pointer.read());
}

// The same holds for an argument of an assertion function that only an assert-enabled
// build calls.
fn assertion_function_through_a_pointer(pointer: *const pg_sys::TransactionId) {
    pg_sys::AssertTransactionIdInAllowableRange!(*pointer);
}

// A check that reads through its operand needs `unsafe` in either build, although a
// release build's C does not read the page.
fn check_reads_its_operand(page: pg_sys::Page) {
    pg_sys::PageValidateSpecialPointer!(pg_sys::cmacros::c::Pointer::from(page));
}

// So does an assertion function that takes a pointer, although a release build has none.
fn assertion_function_takes_a_pointer(control: pg_sys::SlruCtl) {
    pg_sys::SlruPagePrecedesUnitTests!(control, 1_i32);
}

fn main() {}
