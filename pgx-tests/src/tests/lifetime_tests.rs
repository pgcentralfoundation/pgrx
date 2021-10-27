//! This file exists just to ensure the code within compiles
use pgx::*;
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
