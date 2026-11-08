use pgrx::prelude::*;

// The macro only sees `mod file_module;`, not the items in the file.
#[pg_schema]
mod file_module;

fn main() {}
