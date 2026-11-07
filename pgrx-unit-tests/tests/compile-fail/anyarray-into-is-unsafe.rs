use pgrx::AnyArray;
use pgrx::prelude::*;

// `AnyArray::into` does not check that `T` can read the array's type, so it needs `unsafe`.
// `AnyArray::value` is the checked read.
#[pg_extern]
fn first_element(array: AnyArray) -> Option<i32> {
    let values = AnyArray::into::<Vec<i32>>(&array)?;
    values.first().copied()
}

fn main() {}
