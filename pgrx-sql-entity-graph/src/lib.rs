//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
/*!

Rust to SQL mapping support.

> Like all of the [`sql_entity_graph`][crate] APIs, this is considered **internal**
> to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
pub use aggregate::entity::{AggregateTypeEntity, PgAggregateEntity};
pub use aggregate::{
    AggregateType, AggregateTypeList, FinalizeModify, ParallelOption, PgAggregate,
};
pub use control_file::ControlFile;
pub use diff::{EntityChange, EntityModification, EntityRename, SchemaDiff};
pub use enrich::CodeEnrichment;
pub use extension_sql::entity::{ExtensionSqlEntity, SqlDeclaredEntity};
pub use extension_sql::{ExtensionSql, ExtensionSqlFile, SqlDeclared};
pub use extern_args::{ExternArgs, parse_extern_attributes};
pub use mapping::RustSqlMapping;
pub use pg_extern::entity::{
    PgCastEntity, PgExternArgumentEntity, PgExternEntity, PgExternReturnEntity,
    PgExternReturnEntityIteratedItem, PgOperatorEntity,
};
pub use pg_extern::{NameMacro, PgCast, PgExtern, PgExternArgument, PgOperator};
pub use pg_trigger::PgTrigger;
pub use pg_trigger::attribute::PgTriggerAttribute;
pub use pg_trigger::entity::PgTriggerEntity;
pub use pgrx_sql::PgrxSql;
pub use positioning_ref::PositioningRef;
pub use postgres_enum::PostgresEnum;
pub use postgres_enum::entity::PostgresEnumEntity;
pub use postgres_hash::PostgresHash;
pub use postgres_hash::entity::PostgresHashEntity;
pub use postgres_ord::PostgresOrd;
pub use postgres_ord::entity::PostgresOrdEntity;
pub use postgres_type::PostgresTypeDerive;
pub use postgres_type::entity::PostgresTypeEntity;
pub use schema::Schema;
pub use schema::entity::SchemaEntity;
pub use snapshot::{
    DependencyEdge, EntityId, EntityType, SchemaSnapshot, SerializableEntity, deserialize_snapshot,
};
pub use snapshot_types::{
    PgAggregateEntitySnapshot, PostgresHashEntitySnapshot, PostgresOrdEntitySnapshot,
    SqlGraphEntitySnapshot,
};
pub use upgrade::{
    UpgradeConfig, generate_upgrade_file, generate_upgrade_script, generate_upgrade_script_with_sql,
};

// Marker type for TypeId placeholder value when converting snapshots back to runtime entities.
//
// This type is used ONLY when deserializing snapshot entities back into runtime entities
// (via `From<SqlGraphEntitySnapshot> for SqlGraphEntity`). Since snapshots don't contain
// TypeId fields (they're runtime-only), we need a placeholder value for roundtrip compatibility.
//
// This private type ensures the placeholder TypeId cannot collide with any user type.
//
// NOTE: The proper architectural fix has been implemented - snapshots now use separate
// types (SqlGraphEntitySnapshot, etc.) that don't contain TypeId at all. TypeId only
// exists in runtime entity types where it's actually used for runtime type checking.
pub(crate) struct __PgrxInternalTypeIdPlaceholder {
    _private: (),
}

// Helper module for serde deserialization of &'static str
pub(crate) mod serde_helpers {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize_static_str<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Box::leak(s.into_boxed_str()))
    }

    pub fn deserialize_option_static_str<'de, D>(
        deserializer: D,
    ) -> Result<Option<&'static str>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        Ok(opt.map(|s| Box::leak(s.into_boxed_str()) as &'static str))
    }

    pub fn deserialize_vec_static_str<'de, D>(
        deserializer: D,
    ) -> Result<Vec<&'static str>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<String>::deserialize(deserializer)?;
        Ok(vec.into_iter().map(|s| Box::leak(s.into_boxed_str()) as &'static str).collect())
    }

    pub fn deserialize_option_vec_static_str<'de, D>(
        deserializer: D,
    ) -> Result<Option<Vec<&'static str>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<Vec<String>>::deserialize(deserializer)?;
        Ok(opt.map(|vec| {
            vec.into_iter().map(|s| Box::leak(s.into_boxed_str()) as &'static str).collect()
        }))
    }
}
pub use to_sql::entity::ToSqlConfigEntity;
pub use to_sql::{ToSql, ToSqlConfig};
pub use used_type::{UsedType, UsedTypeEntity};

pub(crate) mod aggregate;
pub(crate) mod composite_type;
pub(crate) mod control_file;
pub(crate) mod diff; // Snapshot comparison for upgrade scripts
pub(crate) mod enrich;
pub(crate) mod extension_sql;
pub(crate) mod extern_args;
pub(crate) mod finfo;
#[macro_use]
pub(crate) mod fmt;
pub mod lifetimes;
pub(crate) mod mapping;
pub mod metadata;
pub(crate) mod pg_extern;
pub(crate) mod pg_trigger;
pub(crate) mod pgrx_attribute;
pub(crate) mod pgrx_sql;
pub mod positioning_ref;
pub(crate) mod postgres_enum;
pub(crate) mod postgres_hash;
pub(crate) mod postgres_ord;
pub(crate) mod postgres_type;
pub(crate) mod schema;
pub(crate) mod snapshot; // Snapshot infrastructure for upgrade scripts
pub(crate) mod snapshot_types; // Snapshot-only types without TypeId
pub(crate) mod to_sql;
pub(crate) mod upgrade; // SQL upgrade script generation
pub(crate) mod used_type;

/// Able to produce a GraphViz DOT format identifier.
pub trait SqlGraphIdentifier {
    /// A dot style identifier for the entity.
    ///
    /// Typically this is a 'archetype' prefix (eg `fn` or `type`) then result of
    /// [`std::module_path`], [`core::any::type_name`], or some combination of [`std::file`] and
    /// [`std::line`].
    fn dot_identifier(&self) -> String;

    /// A Rust identifier for the entity.
    ///
    /// Typically this is the result of [`std::module_path`], [`core::any::type_name`],
    /// or some combination of [`std::file`] and [`std::line`].
    fn rust_identifier(&self) -> String;

    fn file(&self) -> Option<&'static str>;

    fn line(&self) -> Option<u32>;
}

pub use postgres_type::Alignment;

/// An entity corresponding to some SQL required by the extension.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub enum SqlGraphEntity {
    ExtensionRoot(ControlFile),
    Schema(SchemaEntity),
    CustomSql(ExtensionSqlEntity),
    Function(PgExternEntity),
    Type(PostgresTypeEntity),
    BuiltinType(String),
    Enum(PostgresEnumEntity),
    Ord(PostgresOrdEntity),
    Hash(PostgresHashEntity),
    Aggregate(PgAggregateEntity),
    Trigger(PgTriggerEntity),
}

impl SqlGraphEntity {
    pub fn sql_anchor_comment(&self) -> String {
        let maybe_file_and_line = if let (Some(file), Some(line)) = (self.file(), self.line()) {
            format!("-- {file}:{line}\n")
        } else {
            String::default()
        };
        format!(
            "\
            {maybe_file_and_line}\
            -- {rust_identifier}\
        ",
            rust_identifier = self.rust_identifier(),
        )
    }

    pub fn id_or_name_matches(&self, ty_id: &::core::any::TypeId, name: &str) -> bool {
        match self {
            SqlGraphEntity::Enum(entity) => entity.id_matches(ty_id),
            SqlGraphEntity::Type(entity) => entity.id_matches(ty_id),
            SqlGraphEntity::BuiltinType(string) => string == name,
            _ => false,
        }
    }

    pub fn type_matches(&self, arg: &dyn TypeIdentifiable) -> bool {
        self.id_or_name_matches(arg.ty_id(), arg.ty_name())
    }
}

pub trait TypeMatch {
    fn id_matches(&self, arg: &core::any::TypeId) -> bool;
}

pub fn type_keyed<'a, 'b, A: TypeMatch, B>((a, b): (&'a A, &'b B)) -> (&'a dyn TypeMatch, &'b B) {
    (a, b)
}

pub trait TypeIdentifiable {
    fn ty_id(&self) -> &core::any::TypeId;
    fn ty_name(&self) -> &str;
}

impl SqlGraphIdentifier for SqlGraphEntity {
    fn dot_identifier(&self) -> String {
        match self {
            SqlGraphEntity::Schema(item) => item.dot_identifier(),
            SqlGraphEntity::CustomSql(item) => item.dot_identifier(),
            SqlGraphEntity::Function(item) => item.dot_identifier(),
            SqlGraphEntity::Type(item) => item.dot_identifier(),
            SqlGraphEntity::BuiltinType(item) => format!("preexisting type {item}"),
            SqlGraphEntity::Enum(item) => item.dot_identifier(),
            SqlGraphEntity::Ord(item) => item.dot_identifier(),
            SqlGraphEntity::Hash(item) => item.dot_identifier(),
            SqlGraphEntity::Aggregate(item) => item.dot_identifier(),
            SqlGraphEntity::Trigger(item) => item.dot_identifier(),
            SqlGraphEntity::ExtensionRoot(item) => item.dot_identifier(),
        }
    }

    fn rust_identifier(&self) -> String {
        match self {
            SqlGraphEntity::Schema(item) => item.rust_identifier(),
            SqlGraphEntity::CustomSql(item) => item.rust_identifier(),
            SqlGraphEntity::Function(item) => item.rust_identifier(),
            SqlGraphEntity::Type(item) => item.rust_identifier(),
            SqlGraphEntity::BuiltinType(item) => item.to_string(),
            SqlGraphEntity::Enum(item) => item.rust_identifier(),
            SqlGraphEntity::Ord(item) => item.rust_identifier(),
            SqlGraphEntity::Hash(item) => item.rust_identifier(),
            SqlGraphEntity::Aggregate(item) => item.rust_identifier(),
            SqlGraphEntity::Trigger(item) => item.rust_identifier(),
            SqlGraphEntity::ExtensionRoot(item) => item.rust_identifier(),
        }
    }

    fn file(&self) -> Option<&'static str> {
        match self {
            SqlGraphEntity::Schema(item) => item.file(),
            SqlGraphEntity::CustomSql(item) => item.file(),
            SqlGraphEntity::Function(item) => item.file(),
            SqlGraphEntity::Type(item) => item.file(),
            SqlGraphEntity::BuiltinType(_item) => None,
            SqlGraphEntity::Enum(item) => item.file(),
            SqlGraphEntity::Ord(item) => item.file(),
            SqlGraphEntity::Hash(item) => item.file(),
            SqlGraphEntity::Aggregate(item) => item.file(),
            SqlGraphEntity::Trigger(item) => item.file(),
            SqlGraphEntity::ExtensionRoot(item) => item.file(),
        }
    }

    fn line(&self) -> Option<u32> {
        match self {
            SqlGraphEntity::Schema(item) => item.line(),
            SqlGraphEntity::CustomSql(item) => item.line(),
            SqlGraphEntity::Function(item) => item.line(),
            SqlGraphEntity::Type(item) => item.line(),
            SqlGraphEntity::BuiltinType(_item) => None,
            SqlGraphEntity::Enum(item) => item.line(),
            SqlGraphEntity::Ord(item) => item.line(),
            SqlGraphEntity::Hash(item) => item.line(),
            SqlGraphEntity::Aggregate(item) => item.line(),
            SqlGraphEntity::Trigger(item) => item.line(),
            SqlGraphEntity::ExtensionRoot(item) => item.line(),
        }
    }
}

impl ToSql for SqlGraphEntity {
    fn to_sql(&self, context: &PgrxSql) -> eyre::Result<String> {
        match self {
            SqlGraphEntity::Schema(SchemaEntity { name: "public" | "pg_catalog", .. }) => {
                Ok(String::default())
            }
            SqlGraphEntity::Schema(item) => item.to_sql(context),
            SqlGraphEntity::CustomSql(item) => item.to_sql(context),
            SqlGraphEntity::Function(item) => {
                if let Some(result) = item.to_sql_config.to_sql(self, context) {
                    result
                } else if context
                    .graph
                    .neighbors_undirected(*context.externs.get(item).unwrap())
                    .any(|neighbor| {
                        let SqlGraphEntity::Type(PostgresTypeEntity {
                            in_fn,
                            in_fn_module_path,
                            out_fn,
                            out_fn_module_path,
                            receive_fn,
                            receive_fn_module_path,
                            send_fn,
                            send_fn_module_path,
                            ..
                        }) = &context.graph[neighbor]
                        else {
                            return false;
                        };

                        let is_in_fn = item.full_path.starts_with(in_fn_module_path)
                            && item.full_path.ends_with(in_fn);
                        let is_out_fn = item.full_path.starts_with(out_fn_module_path)
                            && item.full_path.ends_with(out_fn);
                        let is_receive_fn =
                            receive_fn_module_path.as_ref().is_some_and(|receive_fn_module_path| {
                                item.full_path.starts_with(receive_fn_module_path)
                            }) && receive_fn
                                .is_some_and(|receive_fn| item.full_path.ends_with(receive_fn));
                        let is_send_fn =
                            send_fn_module_path.as_ref().is_some_and(|send_fn_module_path| {
                                item.full_path.starts_with(send_fn_module_path)
                            }) && send_fn.is_some_and(|send_fn| item.full_path.ends_with(send_fn));
                        is_in_fn || is_out_fn || is_receive_fn || is_send_fn
                    })
                {
                    Ok(String::default())
                } else {
                    item.to_sql(context)
                }
            }
            SqlGraphEntity::Type(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::BuiltinType(_) => Ok(String::default()),
            SqlGraphEntity::Enum(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::Ord(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::Hash(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::Aggregate(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::Trigger(item) => {
                item.to_sql_config.to_sql(self, context).unwrap_or_else(|| item.to_sql(context))
            }
            SqlGraphEntity::ExtensionRoot(item) => item.to_sql(context),
        }
    }
}

/// Validate that a given ident is acceptable to PostgreSQL
///
/// PostgreSQL places some restrictions on identifiers for things like functions.
///
/// Namely:
///
/// * It must be less than 64 characters
///
// This list is incomplete, you could expand it!
pub fn ident_is_acceptable_to_postgres(ident: &syn::Ident) -> Result<(), syn::Error> {
    // Roughly `pgrx::pg_sys::NAMEDATALEN`
    //
    // Technically it **should** be that, but we need to guess at build time
    const POSTGRES_IDENTIFIER_MAX_LEN: usize = 64;

    let len = ident.to_string().len();
    if len >= POSTGRES_IDENTIFIER_MAX_LEN {
        return Err(syn::Error::new(
            ident.span(),
            format!(
                "Identifier `{ident}` was {len} characters long, PostgreSQL will truncate identifiers with less than \
                {POSTGRES_IDENTIFIER_MAX_LEN} characters, opt for an identifier which Postgres won't truncate"
            ),
        ));
    }

    Ok(())
}
