use pgrx::prelude::*;
use pgrx::{InOutFuncs, StringInfo};
use serde::{Deserialize, Serialize};

#[derive(PostgresType, Serialize, Deserialize)]
pub struct Part<'a> {
    word: &'a str,
}

// An argument borrows from the call's arguments.
#[pg_extern]
fn static_word(part: Part<'static>) -> &'static str {
    part.word
}

// An element borrows from its array.
fn parts_after_array<'a>(parts: Array<'a, Part<'a>>) -> Vec<Part<'a>> {
    parts.iter_deny_null().collect()
}

// A field cannot borrow from the Datum for 'static. The generated JSON input function already
// rejects these, so they have their own.
#[derive(PostgresType, Serialize, Deserialize)]
#[inoutfuncs]
pub struct StaticWord {
    word: &'static str,
}

impl InOutFuncs for StaticWord {
    fn input(_input: &core::ffi::CStr) -> Self {
        StaticWord { word: "" }
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(self.word);
    }
}

#[derive(PostgresType, Serialize, Deserialize)]
#[inoutfuncs]
pub struct TaggedWord<'a> {
    word: &'a str,
    tag: &'static str,
}

impl InOutFuncs for TaggedWord<'_> {
    fn input(_input: &core::ffi::CStr) -> Self {
        TaggedWord { word: "", tag: "" }
    }

    fn output(&self, buffer: &mut StringInfo) {
        buffer.push_str(self.word);
    }
}

fn main() {}
