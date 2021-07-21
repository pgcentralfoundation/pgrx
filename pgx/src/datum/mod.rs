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
use crate::once_cell::sync::Lazy;
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
use std::any::TypeId;
pub use time_stamp::*;
pub use time_stamp_with_timezone::*;
pub use time_with_timezone::*;
pub use tuples::*;
pub use varlena::*;

use pgx_utils::pg_inventory::RustSqlMapping;

/// A tagging trait to indicate a user type is also meant to be used by Postgres
/// Implemented automatically by `#[derive(PostgresType)]`
pub trait PostgresType {}

pub trait WithTypeIds {
    const ITEM_ID: Lazy<TypeId>;
    const OPTION_ID: Lazy<Option<TypeId>>;
    const VEC_ID: Lazy<Option<TypeId>>;
    const VEC_OPTION_ID: Lazy<Option<TypeId>>;
    const OPTION_VEC_ID: Lazy<Option<TypeId>>;
    const OPTION_VEC_OPTION_ID: Lazy<Option<TypeId>>;
    const ARRAY_ID: Lazy<Option<TypeId>>;
    const OPTION_ARRAY_ID: Lazy<Option<TypeId>>;
    const VARLENA_ID: Lazy<Option<TypeId>>;

    fn register_with_refs(
        map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        single_sql: String,
    ) where
        Self: 'static,
    {
        Self::register(map, single_sql.clone());
        <&Self as WithTypeIds>::register(map, single_sql.clone());
        <&mut Self as WithTypeIds>::register(map, single_sql);
    }

    fn register_sized_with_refs(
        _map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        _single_sql: String,
    ) where
        Self: 'static,
    {
        ()
    }

    fn register_sized(
        _map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        _single_sql: String,
    ) where
        Self: 'static,
    {
        ()
    }

    fn register_varlena_with_refs(
        _map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        _single_sql: String,
    ) where
        Self: 'static,
    {
        ()
    }

    fn register_varlena(
        _map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        _single_sql: String,
    ) where
        Self: 'static,
    {
        ()
    }

    fn register_array_with_refs(
        _map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        _single_sql: String,
    ) where
        Self: 'static,
    {
        ()
    }

    fn register_array(
        _map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        _single_sql: String,
    ) where
        Self: 'static,
    {
        ()
    }

    fn register(map: &mut std::collections::HashMap<TypeId, RustSqlMapping>, single_sql: String)
    where
        Self: 'static,
    {
        let rust = core::any::type_name::<Self>();
        assert_eq!(
            map.insert(
                *Self::ITEM_ID,
                RustSqlMapping {
                    sql: single_sql.clone(),
                    rust: rust.to_string(),
                    id: *Self::ITEM_ID
                }
            ),
            None,
            "Cannot map `{}` twice.",
            rust,
        );
    }
}

impl<T: 'static + ?Sized> WithTypeIds for T {
    const ITEM_ID: Lazy<TypeId> = Lazy::new(|| TypeId::of::<T>());
    const OPTION_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const VEC_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const VEC_OPTION_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const OPTION_VEC_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const OPTION_VEC_OPTION_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const ARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const OPTION_ARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const VARLENA_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
}

pub struct WithSizedTypeIds<T>(pub std::marker::PhantomData<T>);

impl<T: 'static> WithSizedTypeIds<T> {
    pub const PG_BOX_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(TypeId::of::<crate::PgBox<T>>()));
    pub const OPTION_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(TypeId::of::<Option<T>>()));
    pub const VEC_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(TypeId::of::<Vec<T>>()));
    pub const VEC_OPTION_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(TypeId::of::<Vec<Option<T>>>()));
    pub const OPTION_VEC_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(TypeId::of::<Option<Vec<T>>>()));
    pub const OPTION_VEC_OPTION_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(TypeId::of::<Option<Vec<Option<T>>>>()));

    pub fn register_sized_with_refs(
        map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        single_sql: String,
    ) where
        Self: 'static,
    {
        WithSizedTypeIds::<T>::register_sized(map, single_sql.clone());
        WithSizedTypeIds::<&T>::register_sized(map, single_sql.clone());
        WithSizedTypeIds::<&mut T>::register_sized(map, single_sql);
    }

    pub fn register_sized(
        map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        single_sql: String,
    ) {
        let set_sql = format!("{}[]", single_sql);

        if let Some(id) = *WithSizedTypeIds::<T>::PG_BOX_ID {
            let rust = core::any::type_name::<crate::PgBox<T>>().to_string();
            assert_eq!(
                map.insert(
                    id,
                    RustSqlMapping {
                        sql: single_sql.clone(),
                        rust: rust.to_string(),
                        id,
                    }
                ),
                None,
                "Cannot map `{}` twice.",
                rust,
            );
        }

        if let Some(id) = *WithSizedTypeIds::<T>::OPTION_ID {
            let rust = core::any::type_name::<Option<T>>().to_string();
            assert_eq!(
                map.insert(
                    id,
                    RustSqlMapping {
                        sql: single_sql.clone(),
                        rust: rust.to_string(),
                        id,
                    }
                ),
                None,
                "Cannot map `{}` twice.",
                rust,
            );
        }

        if let Some(id) = *WithSizedTypeIds::<T>::VEC_ID {
            let rust = core::any::type_name::<T>().to_string();
            assert_eq!(
                map.insert(
                    id,
                    RustSqlMapping {
                        sql: set_sql.clone(),
                        rust: rust.to_string(),
                        id,
                    }
                ),
                None,
                "Cannot map `{}` twice.",
                rust,
            );
        }
        if let Some(id) = *WithSizedTypeIds::<T>::VEC_OPTION_ID {
            let rust = core::any::type_name::<Vec<Option<T>>>();
            assert_eq!(
                map.insert(
                    id,
                    RustSqlMapping {
                        sql: set_sql.clone(),
                        rust: rust.to_string(),
                        id,
                    }
                ),
                None,
                "Cannot map `{}` twice.",
                rust,
            );
        }
        if let Some(id) = *WithSizedTypeIds::<T>::OPTION_VEC_ID {
            let rust = core::any::type_name::<Option<Vec<T>>>();
            assert_eq!(
                map.insert(
                    id,
                    RustSqlMapping {
                        sql: set_sql.clone(),
                        rust: rust.to_string(),
                        id,
                    }
                ),
                None,
                "Cannot map `{}` twice.",
                rust,
            );
        }
        if let Some(id) = *WithSizedTypeIds::<T>::OPTION_VEC_OPTION_ID {
            let rust = core::any::type_name::<Option<Vec<Option<T>>>>();
            assert_eq!(
                map.insert(
                    id,
                    RustSqlMapping {
                        sql: set_sql.clone(),
                        rust: rust.to_string(),
                        id,
                    }
                ),
                None,
                "Cannot map `{}` twice.",
                rust,
            );
        }
    }
}

pub struct WithArrayTypeIds<T>(pub std::marker::PhantomData<T>);

impl<T: FromDatum + 'static> WithArrayTypeIds<T> {
    pub const ARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(TypeId::of::<Array<T>>()));
    pub const OPTION_ARRAY_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(TypeId::of::<Option<Array<T>>>()));

    pub fn register_array_with_refs(
        map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        single_sql: String,
    ) where
        Self: 'static,
    {
        WithArrayTypeIds::<T>::register_array(map, single_sql.clone());
        WithArrayTypeIds::<&T>::register_array(map, single_sql.clone());
        WithArrayTypeIds::<&mut T>::register_array(map, single_sql);
    }

    pub fn register_array(
        map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        single_sql: String,
    ) {
        let set_sql = format!("{}[]", single_sql);

        if let Some(id) = *WithArrayTypeIds::<T>::ARRAY_ID {
            let rust = core::any::type_name::<Array<T>>().to_string();
            assert_eq!(
                map.insert(
                    id,
                    RustSqlMapping {
                        sql: set_sql.clone(),
                        rust: rust.to_string(),
                        id,
                    }
                ),
                None,
                "Cannot map `{}` twice.",
                rust,
            );
        }
        if let Some(id) = *WithArrayTypeIds::<T>::OPTION_ARRAY_ID {
            let rust = core::any::type_name::<Option<Array<T>>>().to_string();
            assert_eq!(
                map.insert(
                    id,
                    RustSqlMapping {
                        sql: set_sql.clone(),
                        rust: rust.to_string(),
                        id,
                    }
                ),
                None,
                "Cannot map `{}` twice.",
                rust,
            );
        }
    }
}

pub struct WithVarlenaTypeIds<T>(pub std::marker::PhantomData<T>);

impl<T: Copy + 'static> WithVarlenaTypeIds<T> {
    pub const VARLENA_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(TypeId::of::<PgVarlena<T>>()));

    pub fn register_varlena_with_refs(
        map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        single_sql: String,
    ) where
        Self: 'static,
    {
        WithVarlenaTypeIds::<T>::register_varlena(map, single_sql.clone());
        WithVarlenaTypeIds::<&T>::register_varlena(map, single_sql.clone());
        WithVarlenaTypeIds::<&mut T>::register_varlena(map, single_sql);
    }

    pub fn register_varlena(
        map: &mut std::collections::HashMap<TypeId, RustSqlMapping>,
        single_sql: String,
    ) {
        if let Some(id) = *WithVarlenaTypeIds::<T>::VARLENA_ID {
            let rust = core::any::type_name::<PgVarlena<T>>();
            assert_eq!(
                map.insert(
                    id,
                    RustSqlMapping {
                        sql: single_sql.clone(),
                        rust: rust.to_string(),
                        id,
                    }
                ),
                None,
                "Cannot map `{}` twice.",
                rust,
            );
        }
    }
}
