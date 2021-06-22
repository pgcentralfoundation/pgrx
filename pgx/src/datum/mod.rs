// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! Handing for easily converting Postgres Datum types into their corresponding Rust types
//! and converting Rust types into their corresponding Postgres types
mod anyarray;
mod anyelement;
mod array;
mod date;
mod from;
mod geo;
mod inet;
mod internal;
mod into;
mod item_pointer_data;
mod json;
mod numeric;
mod time;
mod time_stamp;
mod time_stamp_with_timezone;
mod time_with_timezone;
mod tuples;
mod varlena;

pub use self::time::*;
pub use anyarray::*;
pub use anyelement::*;
pub use array::*;
pub use date::*;
pub use from::*;
pub use geo::*;
pub use inet::*;
pub use internal::*;
pub use into::*;
pub use item_pointer_data::*;
pub use json::*;
pub use numeric::*;
pub use time_stamp::*;
pub use time_stamp_with_timezone::*;
pub use time_with_timezone::*;
pub use tuples::*;
pub use varlena::*;
use std::any::TypeId;
use crate::once_cell::sync::Lazy;

/// A tagging trait to indicate a user type is also meant to be used by Postgres
/// Implemented automatically by `#[derive(PostgresType)]`
pub trait PostgresType {}

pub trait WithTypeIds {
    const ITEM_ID: Lazy<TypeId>;
    const OPTION_ID: Lazy<TypeId>;
    const VEC_ID: Lazy<TypeId>;
    const VEC_OPTION_ID: Lazy<TypeId>;
}

impl<T: 'static> WithTypeIds for T {
    const ITEM_ID: Lazy<TypeId> = Lazy::new(|| TypeId::of::<T>());
    const OPTION_ID: Lazy<TypeId> = Lazy::new(|| TypeId::of::<Option<T>>());
    const VEC_ID: Lazy<TypeId> = Lazy::new(|| TypeId::of::<Vec<T>>());
    const VEC_OPTION_ID: Lazy<TypeId> = Lazy::new(|| TypeId::of::<Vec<Option<T>>>());
}

pub trait WithoutArrayTypeId {
    const ARRAY_ID: Lazy<Option<TypeId>>;
    const OPTION_ARRAY_ID: Lazy<Option<TypeId>>;
}

impl<T: 'static> WithoutArrayTypeId for T {
    const ARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const OPTION_ARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
}

pub struct WithArrayTypeId<T>(pub std::marker::PhantomData<T>);

impl<T: FromDatum + 'static> WithArrayTypeId<T> {
    pub const ARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(TypeId::of::<Array<T>>()));
    pub const OPTION_ARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(TypeId::of::<Option<Array<T>>>()));
}

pub trait WithoutVarlenaTypeId {
    const VARLENA_ID: Lazy<Option<TypeId>>;
}

impl<T: 'static> WithoutVarlenaTypeId for T {
    const VARLENA_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
}

pub struct WithVarlenaTypeId<T>(pub std::marker::PhantomData<T>);

impl<T: Copy + 'static> WithVarlenaTypeId<T> {
    pub const VARLENA_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(TypeId::of::<PgVarlena<T>>()));
}
