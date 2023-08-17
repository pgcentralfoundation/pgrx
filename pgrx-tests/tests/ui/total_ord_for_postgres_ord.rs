use core::cmp::Ordering;
use pgrx::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd, PostgresType, PostgresOrd)]
pub struct BrokenType {
    int: i32,
}

impl Iterator for BrokenType {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }

    // Previously, this fn could win over Ord::cmp in trait resolution
    fn cmp<I>(self, _rhs: I) -> Ordering
    where
        I: IntoIterator<Item = Self::Item>,
    {
        todo!()
    }
}

impl IntoIterator for &BrokenType {
    type Item = i32;
    type IntoIter = BrokenIter;

    fn into_iter(self) -> BrokenIter {
        todo!()
    }
}

pub struct BrokenIter {}

impl Iterator for BrokenIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {}
