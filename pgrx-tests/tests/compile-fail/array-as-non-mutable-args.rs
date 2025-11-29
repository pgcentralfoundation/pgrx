use pgrx::array::FlatArray;
use pgrx::prelude::*;

#[pg_extern]
fn something<'a>(_arr: &mut FlatArray<'a, i32>) {
    todo!()
}

fn main() {}
