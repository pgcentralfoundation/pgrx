use pgrx::prelude::*;
use pgrx::AnyArray;
use serde::{Deserialize, Serialize};

// A part borrows from the array it was read from, and `Vec<Part>` and `Array::into_iter` keep
// their elements after freeing that array.
#[derive(PostgresType, Serialize, Deserialize)]
pub struct Part<'a> {
    word: &'a str,
}

#[pg_extern]
fn word_from_vec(parts: Vec<Part<'static>>) -> &'static str {
    parts[0].word
}

fn words_from_into_iter(parts: Array<'_, Part<'static>>) -> Vec<&'static str> {
    parts.into_iter().map(|part| part.unwrap().word).collect()
}

#[pg_extern]
fn word_from_anyarray(parts: AnyArray) -> &'static str {
    let parts = parts.value::<Vec<Part<'static>>>().unwrap().unwrap();
    parts[0].word
}

fn main() {}
