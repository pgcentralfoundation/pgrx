//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#![allow(dead_code)]
//! Borrowed `#[derive(PostgresType)]` types: these must compile, and values read from stored
//! Datums must borrow only for as long as those Datums live.
use pgrx::prelude::*;
use serde::*;

fn foo(_s: Vec<Option<&str>>) {
    unimplemented!()
}

#[derive(Debug, Clone, PartialEq, PostgresType, Serialize, Deserialize)]
pub struct ProximityPart<'input> {
    #[serde(borrow)]
    pub words: Vec<Term<'input>>,
    pub distance: Option<ProximityDistance>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProximityDistance {
    pub distance: u32,
    pub in_order: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Term<'input> {
    Null,
    String(String, Option<f32>),
    Wildcard(String, Option<f32>),
    Fuzzy(&'input str, u8, Option<f32>),
    ParsedArray(Vec<Term<'input>>, Option<f32>),
    UnparsedArray(&'input str, Option<f32>),
    ProximityChain(Vec<ProximityPart<'input>>),
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use super::{ProximityPart, Term};
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use pgrx::prelude::*;

    fn fuzzy_word<'a>(part: &ProximityPart<'a>) -> &'a str {
        match part.words.as_slice() {
            [Term::Fuzzy(word, ..)] => word,
            words => panic!("expected one fuzzy term, found {words:?}"),
        }
    }

    /// Each part borrows from the array, so it is read before the array drops.
    #[pg_extern]
    fn proximity_words(parts: Array<'_, ProximityPart<'_>>) -> Vec<String> {
        parts.iter_deny_null().map(|part| fuzzy_word(&part).to_owned()).collect()
    }

    /// The word borrows from the argument's detoasted copy, which lives for the call.
    #[pg_extern]
    fn proximity_word<'a>(part: ProximityPart<'a>) -> &'a str {
        fuzzy_word(&part)
    }

    fn part_json(word: &str) -> String {
        format!(r#"{{"words":[{{"Fuzzy":["{word}",1,null]}}],"distance":null}}"#)
    }

    /// Values read from a table are detoasted into copies: an array always, and a long part
    /// stored out of line. Words borrowed from them must match what was stored. Before
    /// `UnboxDatum::As` carried the Datum's lifetime, `Vec<ProximityPart<'static>>` returned
    /// words from an array that had already been freed.
    #[pg_test]
    fn test_borrowed_postgres_type_from_stored_values() -> spi::Result<()> {
        Spi::run(
            "CREATE TABLE tests.proximity_parts (id int, part ProximityPart, parts ProximityPart[])",
        )?;
        // EXTERNAL moves a long value out of line without compressing it.
        Spi::run(
            "ALTER TABLE tests.proximity_parts
                ALTER COLUMN part SET STORAGE EXTERNAL,
                ALTER COLUMN parts SET STORAGE EXTERNAL",
        )?;
        let long_word = format!("long-{}", "x".repeat(4000));
        let rows = [(1, "short".to_owned(), 2), (2, long_word, 200)];
        for (id, word, len) in &rows {
            let part = part_json(word);
            let parts = (1..=*len)
                .map(|i| format!("'{}'::ProximityPart", part_json(&format!("w{i}"))))
                .collect::<Vec<_>>()
                .join(", ");
            Spi::run(&format!(
                "INSERT INTO tests.proximity_parts VALUES ({id}, '{part}', ARRAY[{parts}])"
            ))?;
        }
        let toast = Spi::get_one::<String>(
            "SELECT reltoastrelid::regclass::text FROM pg_class
                WHERE oid = 'tests.proximity_parts'::regclass",
        )?
        .expect("no TOAST relation");
        // Row 2's part and array.
        let stored_out_of_line =
            Spi::get_one::<i64>(&format!("SELECT count(DISTINCT chunk_id) FROM {toast}"))?;
        assert_eq!(stored_out_of_line, Some(2));

        for (id, word, len) in &rows {
            let read = Spi::get_one::<String>(&format!(
                "SELECT tests.proximity_word(part) FROM tests.proximity_parts WHERE id = {id}"
            ))?;
            assert_eq!(read.as_ref(), Some(word), "part on row {id}");

            let expected = (1..=*len).map(|i| format!("w{i}")).collect::<Vec<_>>();
            let read = Spi::get_one::<Vec<String>>(&format!(
                "SELECT tests.proximity_words(parts) FROM tests.proximity_parts WHERE id = {id}"
            ))?;
            assert_eq!(read.as_ref(), Some(&expected), "parts on row {id}");
        }
        Ok(())
    }
}
