use pgrx::prelude::*;

#[derive(PartialEq, PostgresEq)]
struct BrokenType {
    int: i32,
}

fn main() {}
