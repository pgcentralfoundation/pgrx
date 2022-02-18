use core::any::TypeId;

/// A mapping from a Rust type to a SQL type, with a `TypeId`.
///
/// ```rust
/// use pgx_utils::sql_entity_graph::mapping::RustSqlMapping;
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
    // This is the **resolved** type, not the raw source. This means a Type Aliase of `type Foo = u32` would appear as `u32`.
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
/// use pgx_utils::sql_entity_graph::mapping::RustSourceOnlySqlMapping;
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
        Self {
            rust: rust.to_string(),
            sql: sql.to_string(),
        }
    }
}
