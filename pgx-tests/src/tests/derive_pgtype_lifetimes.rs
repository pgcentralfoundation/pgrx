// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


#![allow(dead_code)]
/// the purpose of this test is to just make sure this code compiles!
use pgx::*;
use serde::*;

fn foo<'a>(_s: Vec<Option<&'a str>>) {
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
