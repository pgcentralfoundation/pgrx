use pgrx::prelude::*;

#[pg_cast]
pub fn cast_function() -> i32 {
    0
}

fn main() {}