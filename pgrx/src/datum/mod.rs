//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! Handing for easily converting Postgres Datum types into their corresponding Rust types
//! and converting Rust types into their corresponding Postgres types
#![allow(clippy::unused_unit)]
mod anyarray;
mod anyelement;
mod array;
mod date;
pub mod datetime_support;
mod from;
mod geo;
mod inet;
mod internal;
mod interval;
mod into;
mod item_pointer_data;
mod json;
pub mod numeric;
pub mod numeric_support;
#[deny(unsafe_op_in_unsafe_fn)]
mod range;
mod time;
mod time_stamp;
mod time_stamp_with_timezone;
mod time_with_timezone;
mod tuples;
mod unbox;
mod uuid;
mod varlena;

pub use self::time::*;
pub use self::uuid::*;
pub use anyarray::*;
pub use anyelement::*;
pub use array::*;
pub use date::*;
pub use datetime_support::*;
pub use from::*;
pub use inet::*;
pub use internal::*;
pub use interval::*;
pub use into::*;
pub use json::*;
pub use numeric::{AnyNumeric, Numeric};
use once_cell::sync::Lazy;
pub use range::*;
use std::any::TypeId;
pub use time_stamp::*;
pub use time_stamp_with_timezone::*;
pub use time_with_timezone::*;
pub use unbox::*;
pub use varlena::*;

use crate::memcx::MemCx;
use crate::pg_sys;
use crate::PgBox;
use core::marker::PhantomData;
use pgrx_sql_entity_graph::RustSqlMapping;

/// How Postgres represents datatypes
///
/// The "no-frills" version is [`pg_sys::Datum`], which is abstractly a union of "pointer to void"
/// with other scalar types that can be packed within a pointer's bytes. In practical use, a "raw"
/// Datum can prove to have the same risks as a pointer: code may try to use it without knowing
/// whether its pointee has been deallocated. To lift a Datum into a Rust type requires making
/// implicit lifetimes into explicit bounds.
///
/// Merely having a lifetime does not make `Datum<'src>` "safe" to use. To abstractly represent a
/// full PostgreSQL value needs at least the tuple (Datum, bool, [`pg_sys::Oid`]): a tagged union.
/// `Datum<'src>` itself is effectively a dynamically-typed union *without a type tag*. It exists
/// not to make code manipulating it safe, but to make it possible to write unsafe code correctly,
/// passing Datums to and from Postgres without having to wonder if the implied `&'src T` would
/// actually refer to deallocated data.
///
/// # Designing safe abstractions
/// A function must only be declared safe if *all* inputs **cannot** cause [undefined behavior].
/// Transmuting a raw `pg_sys::Datum` into [`&'a T`] grants a potentially-unbounded lifetime,
/// breaking the rule borrows must not outlive the borrowed. Avoiding such transmutations infects
/// even simple generic functions with soundness obligations. Using only `&'a pg_sys::Datum` lasts
/// only until one must pass by-value, which is the entire point of the original type as Postgres
/// defined it, but can still be preferable.
///
/// `Datum<'src>` makes it theoretically possible to write functions with a signature like
/// ```
/// use pgrx::datum::Datum;
/// # use core::marker::PhantomData;
/// # use pgrx::memcx::MemCx;
/// # struct InCx<'mcx, T>(T, PhantomData<&'mcx MemCx<'mcx>>);
/// fn construct_type_from_datum<'src, T>(
///     datum: Datum<'src>,
///     func: impl FnOnce(Datum<'src>) -> InCx<'src, T>
/// ) -> InCx<'src, T> {
///    func(datum)
/// }
/// ```
/// However, it is possible for `T<'src>` to be insufficient to represent the real lifetime of the
/// abstract Postgres type's allocations. Often a Datum must be "detoasted", which may reallocate.
/// This may demand two constraints on the return type to represent both possible lifetimes, like:
/// ```
/// use pgrx::datum::Datum;
/// use pgrx::memcx::MemCx;
/// # use core::marker::PhantomData;
/// # struct Detoasted<'mcx, T>(T, PhantomData<&'mcx MemCx<'mcx>>);
/// # struct InCx<'mcx, T>(T, PhantomData<&'mcx MemCx<'mcx>>);
/// fn detoast_type_from_datum<'old, 'new, T>(
///     datum: Datum<'old>,
///     memcx: MemCx<'new>,
/// ) -> Detoasted<'new, InCx<'old, T>> {
///    todo!()
/// }
/// ```
/// In actual practice, these can be unified into a single lifetime: the lower bound of both.
/// This is both good and bad: types can use fewer lifetime annotations, even after detoasting.
/// However, in general, because lifetime unification can be done implicitly by the compiler,
/// it is often important to name each and every single lifetime involved in functions that
/// perform these tasks.
///
/// [`&'a T`]: reference
/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
pub struct Datum<'src>(
    pg_sys::Datum,
    /// if a Datum borrows anything, it's "from" a [`pg_sys::MemoryContext`]
    /// as a memory context, like an arena, is deallocated "together".
    /// FIXME: a more-correct inner type later
    PhantomData<&'src MemCx<'src>>,
);

impl<'src> Datum<'src> {
    /// The Datum without its lifetime.
    pub fn sans_lifetime(self) -> pg_sys::Datum {
        self.0
    }
}

/// A tagging trait to indicate a user type is also meant to be used by Postgres
/// Implemented automatically by `#[derive(PostgresType)]`
pub trait PostgresType {}

/// Obtain a TypeId for T without `T: 'static`
#[inline]
#[doc(hidden)]
pub fn nonstatic_typeid<T: ?Sized>() -> core::any::TypeId {
    trait NonStaticAny {
        fn type_id(&self) -> core::any::TypeId
        where
            Self: 'static;
    }
    impl<T: ?Sized> NonStaticAny for core::marker::PhantomData<T> {
        #[inline]
        fn type_id(&self) -> core::any::TypeId
        where
            Self: 'static,
        {
            core::any::TypeId::of::<T>()
        }
    }
    let it = core::marker::PhantomData::<T>;
    // There is no excuse for the crimes we have done here, but what jury would convict us?
    unsafe { core::mem::transmute::<&dyn NonStaticAny, &'static dyn NonStaticAny>(&it).type_id() }
}

/// A type which can have its [`core::any::TypeId`]s registered for Rust to SQL mapping.
///
/// An example use of this trait:
///
/// ```rust
/// use pgrx::prelude::*;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Copy, Serialize, Deserialize, PostgresType)]
/// struct Treat<'a> { best_part: &'a str, };
///
/// let mut mappings = Default::default();
/// let treat_string = stringify!(Treat).to_string();
/// <Treat<'_> as pgrx::datum::WithTypeIds>::register_with_refs(&mut mappings, treat_string.clone());
///
/// assert!(mappings.iter().any(|x| x.id == ::pgrx::datum::nonstatic_typeid::<Treat<'static>>()));
/// ```
///
/// This trait uses the fact that inherent implementations are a higher priority than trait
/// implementations.
pub trait WithTypeIds {
    const ITEM_ID: Lazy<TypeId>;
    const OPTION_ID: Lazy<Option<TypeId>>;
    const VEC_ID: Lazy<Option<TypeId>>;
    const VEC_OPTION_ID: Lazy<Option<TypeId>>;
    const OPTION_VEC_ID: Lazy<Option<TypeId>>;
    const OPTION_VEC_OPTION_ID: Lazy<Option<TypeId>>;
    const ARRAY_ID: Lazy<Option<TypeId>>;
    const OPTION_ARRAY_ID: Lazy<Option<TypeId>>;
    const VARIADICARRAY_ID: Lazy<Option<TypeId>>;
    const OPTION_VARIADICARRAY_ID: Lazy<Option<TypeId>>;
    const VARLENA_ID: Lazy<Option<TypeId>>;
    const OPTION_VARLENA_ID: Lazy<Option<TypeId>>;

    fn register_with_refs(map: &mut std::collections::HashSet<RustSqlMapping>, single_sql: String) {
        Self::register(map, single_sql.clone());
        <&Self as WithTypeIds>::register(map, single_sql.clone());
        <&mut Self as WithTypeIds>::register(map, single_sql);
    }

    fn register_sized_with_refs(
        _map: &mut std::collections::HashSet<RustSqlMapping>,
        _single_sql: String,
    ) {
        ()
    }

    fn register_sized(_map: &mut std::collections::HashSet<RustSqlMapping>, _single_sql: String) {
        ()
    }

    fn register_varlena_with_refs(
        _map: &mut std::collections::HashSet<RustSqlMapping>,
        _single_sql: String,
    ) {
        ()
    }

    fn register_varlena(_map: &mut std::collections::HashSet<RustSqlMapping>, _single_sql: String) {
        ()
    }

    fn register_array_with_refs(
        _map: &mut std::collections::HashSet<RustSqlMapping>,
        _single_sql: String,
    ) {
        ()
    }

    fn register_array(_map: &mut std::collections::HashSet<RustSqlMapping>, _single_sql: String) {
        ()
    }

    fn register(set: &mut std::collections::HashSet<RustSqlMapping>, single_sql: String) {
        let rust = core::any::type_name::<Self>();
        assert!(
            set.insert(RustSqlMapping {
                sql: single_sql.clone(),
                rust: rust.to_string(),
                id: *Self::ITEM_ID,
            }),
            "Cannot set mapping of `{}` twice, was already `{}`.",
            rust,
            single_sql,
        );
    }
}

impl<T: ?Sized> WithTypeIds for T {
    const ITEM_ID: Lazy<TypeId> = Lazy::new(|| nonstatic_typeid::<T>());
    const OPTION_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const VEC_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const VEC_OPTION_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const OPTION_VEC_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const OPTION_VEC_OPTION_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const ARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const OPTION_ARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const VARIADICARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const OPTION_VARIADICARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const VARLENA_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
    const OPTION_VARLENA_ID: Lazy<Option<TypeId>> = Lazy::new(|| None);
}

/// A type which can have its [`core::any::TypeId`]s registered for Rust to SQL mapping.
///
/// An example use of this trait:
///
/// ```rust
/// use pgrx::prelude::*;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Copy, Serialize, Deserialize, PostgresType)]
/// pub struct Treat<'a> { best_part: &'a str, };
///
/// let mut mappings = Default::default();
/// let treat_string = stringify!(Treat).to_string();
///
/// pgrx::datum::WithSizedTypeIds::<Treat<'static>>::register_sized_with_refs(
///     &mut mappings,
///     treat_string.clone()
/// );
///
/// assert!(mappings.iter().any(|x| x.id == core::any::TypeId::of::<Option<Treat<'static>>>()));
/// ```
///
/// This trait uses the fact that inherent implementations are a higher priority than trait
/// implementations.
pub struct WithSizedTypeIds<T>(pub core::marker::PhantomData<T>);

impl<T> WithSizedTypeIds<T> {
    pub const PG_BOX_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(nonstatic_typeid::<PgBox<T>>()));
    pub const PG_BOX_OPTION_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(nonstatic_typeid::<PgBox<Option<T>>>()));
    pub const PG_BOX_VEC_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(nonstatic_typeid::<PgBox<Vec<T>>>()));
    pub const OPTION_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(nonstatic_typeid::<Option<T>>()));
    pub const VEC_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(nonstatic_typeid::<Vec<T>>()));
    pub const VEC_OPTION_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(nonstatic_typeid::<Vec<Option<T>>>()));
    pub const OPTION_VEC_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(nonstatic_typeid::<Option<Vec<T>>>()));
    pub const OPTION_VEC_OPTION_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(nonstatic_typeid::<Option<Vec<Option<T>>>>()));

    pub fn register_sized_with_refs(
        map: &mut std::collections::HashSet<RustSqlMapping>,
        single_sql: String,
    ) where
        Self: 'static,
    {
        WithSizedTypeIds::<T>::register_sized(map, single_sql.clone());
        WithSizedTypeIds::<&T>::register_sized(map, single_sql.clone());
        WithSizedTypeIds::<&mut T>::register_sized(map, single_sql);
    }

    pub fn register_sized(map: &mut std::collections::HashSet<RustSqlMapping>, single_sql: String) {
        let set_sql = format!("{}[]", single_sql);

        if let Some(id) = *WithSizedTypeIds::<T>::PG_BOX_ID {
            let rust = core::any::type_name::<crate::PgBox<T>>().to_string();
            assert!(
                map.insert(RustSqlMapping { sql: single_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }

        if let Some(id) = *WithSizedTypeIds::<T>::PG_BOX_OPTION_ID {
            let rust = core::any::type_name::<crate::PgBox<Option<T>>>().to_string();
            assert!(
                map.insert(RustSqlMapping { sql: single_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }

        if let Some(id) = *WithSizedTypeIds::<T>::PG_BOX_VEC_ID {
            let rust = core::any::type_name::<crate::PgBox<Vec<T>>>().to_string();
            assert!(
                map.insert(RustSqlMapping { sql: set_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }

        if let Some(id) = *WithSizedTypeIds::<T>::OPTION_ID {
            let rust = core::any::type_name::<Option<T>>().to_string();
            assert!(
                map.insert(RustSqlMapping { sql: single_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }

        if let Some(id) = *WithSizedTypeIds::<T>::VEC_ID {
            let rust = core::any::type_name::<T>().to_string();
            assert!(
                map.insert(RustSqlMapping { sql: set_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }
        if let Some(id) = *WithSizedTypeIds::<T>::VEC_OPTION_ID {
            let rust = core::any::type_name::<Vec<Option<T>>>();
            assert!(
                map.insert(RustSqlMapping { sql: set_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }
        if let Some(id) = *WithSizedTypeIds::<T>::OPTION_VEC_ID {
            let rust = core::any::type_name::<Option<Vec<T>>>();
            assert!(
                map.insert(RustSqlMapping { sql: set_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }
        if let Some(id) = *WithSizedTypeIds::<T>::OPTION_VEC_OPTION_ID {
            let rust = core::any::type_name::<Option<Vec<Option<T>>>>();
            assert!(
                map.insert(RustSqlMapping { sql: set_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }
    }
}

/// An [`Array`] compatible type which can have its [`core::any::TypeId`]s registered for Rust to SQL mapping.
///
/// An example use of this trait:
///
/// ```rust
/// use pgrx::prelude::*;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize, PostgresType)]
/// pub struct Treat { best_part: String, };
///
/// let mut mappings = Default::default();
/// let treat_string = stringify!(Treat).to_string();
///
/// pgrx::datum::WithArrayTypeIds::<Treat>::register_array_with_refs(
///     &mut mappings,
///     treat_string.clone()
/// );
///
/// assert!(mappings.iter().any(|x| x.id == ::pgrx::datum::nonstatic_typeid::<Array<Treat>>()));
/// ```
///
/// This trait uses the fact that inherent implementations are a higher priority than trait
/// implementations.
pub struct WithArrayTypeIds<T>(pub core::marker::PhantomData<T>);

impl<T: FromDatum + 'static> WithArrayTypeIds<T> {
    pub const ARRAY_ID: Lazy<Option<TypeId>> = Lazy::new(|| Some(nonstatic_typeid::<Array<T>>()));
    pub const OPTION_ARRAY_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(nonstatic_typeid::<Option<Array<T>>>()));
    pub const VARIADICARRAY_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(nonstatic_typeid::<VariadicArray<T>>()));
    pub const OPTION_VARIADICARRAY_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(nonstatic_typeid::<Option<VariadicArray<T>>>()));

    pub fn register_array_with_refs(
        map: &mut std::collections::HashSet<RustSqlMapping>,
        single_sql: String,
    ) where
        Self: 'static,
    {
        WithArrayTypeIds::<T>::register_array(map, single_sql.clone());
        WithArrayTypeIds::<&T>::register_array(map, single_sql.clone());
        WithArrayTypeIds::<&mut T>::register_array(map, single_sql);
    }

    pub fn register_array(map: &mut std::collections::HashSet<RustSqlMapping>, single_sql: String) {
        let set_sql = format!("{}[]", single_sql);

        if let Some(id) = *WithArrayTypeIds::<T>::ARRAY_ID {
            let rust = core::any::type_name::<Array<T>>().to_string();
            assert!(
                map.insert(RustSqlMapping { sql: set_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }
        if let Some(id) = *WithArrayTypeIds::<T>::OPTION_ARRAY_ID {
            let rust = core::any::type_name::<Option<Array<T>>>().to_string();
            assert!(
                map.insert(RustSqlMapping { sql: set_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }

        if let Some(id) = *WithArrayTypeIds::<T>::VARIADICARRAY_ID {
            let rust = core::any::type_name::<VariadicArray<T>>().to_string();
            assert!(
                map.insert(RustSqlMapping { sql: set_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }
        if let Some(id) = *WithArrayTypeIds::<T>::OPTION_VARIADICARRAY_ID {
            let rust = core::any::type_name::<Option<VariadicArray<T>>>().to_string();
            assert!(
                map.insert(RustSqlMapping { sql: set_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }
    }
}

/// A [`PgVarlena`] compatible type which can have its [`core::any::TypeId`]s registered for Rust to SQL mapping.
///
/// An example use of this trait:
///
/// ```rust
/// use pgrx::prelude::*;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Copy, Serialize, Deserialize, PostgresType)]
/// pub struct Treat<'a> { best_part: &'a str, };
///
/// let mut mappings = Default::default();
/// let treat_string = stringify!(Treat).to_string();
///
/// pgrx::datum::WithVarlenaTypeIds::<Treat<'static>>::register_varlena_with_refs(
///     &mut mappings,
///     treat_string.clone()
/// );
///
/// assert!(mappings.iter().any(|x| x.id == ::pgrx::datum::nonstatic_typeid::<PgVarlena<Treat<'_>>>()));
/// ```
///
/// This trait uses the fact that inherent implementations are a higher priority than trait
/// implementations.
pub struct WithVarlenaTypeIds<T>(pub core::marker::PhantomData<T>);

impl<T: Copy + 'static> WithVarlenaTypeIds<T> {
    pub const VARLENA_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(nonstatic_typeid::<PgVarlena<T>>()));
    pub const PG_BOX_VARLENA_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(nonstatic_typeid::<PgBox<PgVarlena<T>>>()));
    pub const OPTION_VARLENA_ID: Lazy<Option<TypeId>> =
        Lazy::new(|| Some(nonstatic_typeid::<Option<PgVarlena<T>>>()));

    pub fn register_varlena_with_refs(
        map: &mut std::collections::HashSet<RustSqlMapping>,
        single_sql: String,
    ) where
        Self: 'static,
    {
        WithVarlenaTypeIds::<T>::register_varlena(map, single_sql.clone());
        WithVarlenaTypeIds::<&T>::register_varlena(map, single_sql.clone());
        WithVarlenaTypeIds::<&mut T>::register_varlena(map, single_sql);
    }

    pub fn register_varlena(
        map: &mut std::collections::HashSet<RustSqlMapping>,
        single_sql: String,
    ) {
        if let Some(id) = *WithVarlenaTypeIds::<T>::VARLENA_ID {
            let rust = core::any::type_name::<PgVarlena<T>>();
            assert!(
                map.insert(RustSqlMapping { sql: single_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }

        if let Some(id) = *WithVarlenaTypeIds::<T>::PG_BOX_VARLENA_ID {
            let rust = core::any::type_name::<PgBox<PgVarlena<T>>>().to_string();
            assert!(
                map.insert(RustSqlMapping { sql: single_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }
        if let Some(id) = *WithVarlenaTypeIds::<T>::OPTION_VARLENA_ID {
            let rust = core::any::type_name::<Option<PgVarlena<T>>>().to_string();
            assert!(
                map.insert(RustSqlMapping { sql: single_sql.clone(), rust: rust.to_string(), id }),
                "Cannot map `{}` twice.",
                rust,
            );
        }
    }
}
