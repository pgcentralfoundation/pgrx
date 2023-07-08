/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
//! This file exists just to ensure the code within compiles
use pgrx::prelude::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(PostgresType, Serialize, Deserialize)]
pub struct CustomType<'s> {
    __marker: PhantomData<&'s ()>,
}

#[pg_extern]
fn type_with_lifetime<'s>(_value: Option<CustomType<'s>>) {}

#[pg_extern]
fn type_ref_with_lifetime<'a>(_value: &'a str) {}

#[pg_extern]
fn returns_lifetime<'a>() -> Option<CustomType<'a>> {
    None
}

#[pg_extern]
fn returns_ref_with_lifetime<'a>() -> &'a str {
    ""
}

#[pg_extern]
fn returns_option_ref_with_lifetime<'a>() -> Option<&'a str> {
    None
}

#[pg_extern]
fn returns_tuple_with_lifetime(
    value: &'static str,
) -> TableIterator<(name!(a, &'static str), name!(b, Option<&'static str>))> {
    TableIterator::once((value, Some(value)))
}

#[pg_extern]
fn returns_iterator_with_lifetime<'a>(value: &'a str) -> SetOfIterator<&'a str> {
    SetOfIterator::new(value.split_whitespace().into_iter())
}
