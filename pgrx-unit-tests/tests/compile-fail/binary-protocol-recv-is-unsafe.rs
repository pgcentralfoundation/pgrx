use pgrx::datum::Internal;
use pgrx::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(PostgresType, Serialize, Deserialize)]
#[pg_binary_protocol]
pub struct Point {
    x: i32,
}

#[derive(PostgresType, Serialize, Deserialize)]
#[pg_binary_protocol]
pub struct Part<'a> {
    word: &'a str,
}

// The receive function reads its argument as a `StringInfoData`, but an `Internal` can hold any
// value.
fn recv_from_any_internal() -> Point {
    point_recv(Internal::new(0u8))
}

// The value borrows memory in `CurrentMemoryContext`, but the caller picks its lifetime.
fn recv_static_part(internal: Internal) -> Part<'static> {
    part_recv(internal)
}

fn main() {}
