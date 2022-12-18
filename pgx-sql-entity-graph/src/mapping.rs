/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

Rust to SQL mapping support for dependency graph generation

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use core::any::TypeId;

/// A mapping from a Rust type to a SQL type, with a `TypeId`.
///
/// ```rust
/// use pgx_sql_entity_graph::RustSqlMapping;
///
/// let constructed = RustSqlMapping::of::<i32>(String::from("int"));
/// let raw = RustSqlMapping {
///     rust: core::any::type_name::<i32>().to_string(),
///     sql: String::from("int"),
///     id: core::any::TypeId::of::<i32>(),
/// };
///
/// assert_eq!(constructed, raw);
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustSqlMapping {
    // This is the **resolved** type, not the raw source. This means a Type Alias of `type Foo = u32` would appear as `u32`.
    pub rust: String,
    pub sql: String,
    pub id: TypeId,
}

impl RustSqlMapping {
    pub fn of<T: 'static>(sql: String) -> Self {
        Self {
            rust: core::any::type_name::<T>().to_string(),
            sql: sql.to_string(),
            id: core::any::TypeId::of::<T>(),
        }
    }
}

/// A mapping from a Rust source fragment to a SQL type, typically for type aliases.
///
/// In general, this can only offer a fuzzy matching, as it does not use [`core::any::TypeId`].
///
/// ```rust
/// use pgx_sql_entity_graph::RustSourceOnlySqlMapping;
///
/// let constructed = RustSourceOnlySqlMapping::new(
///     String::from("Oid"),
///     String::from("int"),
/// );
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustSourceOnlySqlMapping {
    pub rust: String,
    pub sql: String,
}

impl RustSourceOnlySqlMapping {
    pub fn new(rust: String, sql: String) -> Self {
        Self { rust: rust.to_string(), sql: sql.to_string() }
    }
}
