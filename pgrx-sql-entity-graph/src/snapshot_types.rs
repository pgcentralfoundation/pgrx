//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Snapshot-only versions of entity types for serialization.
//!
//! This module contains stripped-down versions of entity types that remove
//! TypeId fields, which are not stable across compilations and should not
//! be serialized. These types are used ONLY for snapshot serialization and
//! comparison, while the original types (with TypeId) are used at runtime.

use crate::aggregate::entity::AggregateTypeEntity;
use crate::control_file::ControlFile;
use crate::extension_sql::entity::ExtensionSqlEntity;
use crate::pg_extern::entity::PgExternEntity;
use crate::pg_trigger::entity::PgTriggerEntity;
use crate::postgres_enum::entity::PostgresEnumEntity;
use crate::postgres_type::entity::PostgresTypeEntity;
use crate::schema::entity::SchemaEntity;
use crate::to_sql::entity::ToSqlConfigEntity;
use crate::{FinalizeModify, ParallelOption, SqlGraphEntity, UsedTypeEntity};

/// Snapshot version of [`crate::postgres_hash::entity::PostgresHashEntity`] without TypeId.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = ""))]
pub struct PostgresHashEntitySnapshot {
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub name: &'static str,
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub file: &'static str,
    pub line: u32,
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub full_path: &'static str,
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub module_path: &'static str,
    // Note: `id: TypeId` field is omitted - not needed for snapshots
    pub to_sql_config: ToSqlConfigEntity,
}

/// Snapshot version of [`crate::postgres_ord::entity::PostgresOrdEntity`] without TypeId.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = ""))]
pub struct PostgresOrdEntitySnapshot {
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub name: &'static str,
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub file: &'static str,
    pub line: u32,
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub full_path: &'static str,
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub module_path: &'static str,
    // Note: `id: TypeId` field is omitted - not needed for snapshots
    pub to_sql_config: ToSqlConfigEntity,
}

/// Snapshot version of [`crate::aggregate::entity::PgAggregateEntity`] without TypeId.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = ""))]
pub struct PgAggregateEntitySnapshot {
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub full_path: &'static str,
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub module_path: &'static str,
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub file: &'static str,
    pub line: u32,
    // Note: `ty_id: TypeId` field is omitted - not needed for snapshots
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub name: &'static str,

    pub ordered_set: bool,
    pub args: Vec<AggregateTypeEntity>,
    pub direct_args: Option<Vec<AggregateTypeEntity>>,
    pub stype: AggregateTypeEntity,

    #[serde(deserialize_with = "crate::serde_helpers::deserialize_static_str")]
    pub sfunc: &'static str,

    #[serde(deserialize_with = "crate::serde_helpers::deserialize_option_static_str")]
    pub finalfunc: Option<&'static str>,

    pub finalfunc_modify: Option<FinalizeModify>,

    #[serde(deserialize_with = "crate::serde_helpers::deserialize_option_static_str")]
    pub combinefunc: Option<&'static str>,

    #[serde(deserialize_with = "crate::serde_helpers::deserialize_option_static_str")]
    pub serialfunc: Option<&'static str>,

    #[serde(deserialize_with = "crate::serde_helpers::deserialize_option_static_str")]
    pub deserialfunc: Option<&'static str>,

    #[serde(deserialize_with = "crate::serde_helpers::deserialize_option_static_str")]
    pub initcond: Option<&'static str>,

    #[serde(deserialize_with = "crate::serde_helpers::deserialize_option_static_str")]
    pub msfunc: Option<&'static str>,

    #[serde(deserialize_with = "crate::serde_helpers::deserialize_option_static_str")]
    pub minvfunc: Option<&'static str>,

    pub mstype: Option<UsedTypeEntity>,

    #[serde(deserialize_with = "crate::serde_helpers::deserialize_option_static_str")]
    pub mfinalfunc: Option<&'static str>,

    pub mfinalfunc_modify: Option<FinalizeModify>,

    #[serde(deserialize_with = "crate::serde_helpers::deserialize_option_static_str")]
    pub minitcond: Option<&'static str>,

    #[serde(deserialize_with = "crate::serde_helpers::deserialize_option_static_str")]
    pub sortop: Option<&'static str>,

    pub hypothetical: bool,
    pub parallel: Option<ParallelOption>,
    pub to_sql_config: ToSqlConfigEntity,
}

/// Snapshot version of [`SqlGraphEntity`] for serialization without TypeId fields.
///
/// This enum mirrors [`SqlGraphEntity`] but uses snapshot versions for variants
/// that contain TypeId fields (Ord, Hash, Aggregate).
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub enum SqlGraphEntitySnapshot {
    ExtensionRoot(ControlFile),
    Schema(SchemaEntity),
    CustomSql(ExtensionSqlEntity),
    Function(PgExternEntity),
    Type(PostgresTypeEntity),
    BuiltinType(String),
    Enum(PostgresEnumEntity),
    Ord(PostgresOrdEntitySnapshot),
    Hash(PostgresHashEntitySnapshot),
    Aggregate(PgAggregateEntitySnapshot),
    Trigger(PgTriggerEntity),
}

/// Convert a runtime entity to its snapshot version (without TypeId).
impl From<&SqlGraphEntity> for SqlGraphEntitySnapshot {
    fn from(entity: &SqlGraphEntity) -> Self {
        match entity {
            SqlGraphEntity::ExtensionRoot(e) => SqlGraphEntitySnapshot::ExtensionRoot(e.clone()),
            SqlGraphEntity::Schema(e) => SqlGraphEntitySnapshot::Schema(e.clone()),
            SqlGraphEntity::CustomSql(e) => SqlGraphEntitySnapshot::CustomSql(e.clone()),
            SqlGraphEntity::Function(e) => SqlGraphEntitySnapshot::Function(e.clone()),
            SqlGraphEntity::Type(e) => SqlGraphEntitySnapshot::Type(e.clone()),
            SqlGraphEntity::BuiltinType(e) => SqlGraphEntitySnapshot::BuiltinType(e.clone()),
            SqlGraphEntity::Enum(e) => SqlGraphEntitySnapshot::Enum(e.clone()),

            // These three need conversion to remove TypeId
            SqlGraphEntity::Ord(e) => SqlGraphEntitySnapshot::Ord(PostgresOrdEntitySnapshot {
                name: e.name,
                file: e.file,
                line: e.line,
                full_path: e.full_path,
                module_path: e.module_path,
                to_sql_config: e.to_sql_config.clone(),
            }),

            SqlGraphEntity::Hash(e) => SqlGraphEntitySnapshot::Hash(PostgresHashEntitySnapshot {
                name: e.name,
                file: e.file,
                line: e.line,
                full_path: e.full_path,
                module_path: e.module_path,
                to_sql_config: e.to_sql_config.clone(),
            }),

            SqlGraphEntity::Aggregate(e) => {
                SqlGraphEntitySnapshot::Aggregate(PgAggregateEntitySnapshot {
                    full_path: e.full_path,
                    module_path: e.module_path,
                    file: e.file,
                    line: e.line,
                    name: e.name,
                    ordered_set: e.ordered_set,
                    args: e.args.clone(),
                    direct_args: e.direct_args.clone(),
                    stype: e.stype.clone(),
                    sfunc: e.sfunc,
                    finalfunc: e.finalfunc,
                    finalfunc_modify: e.finalfunc_modify,
                    combinefunc: e.combinefunc,
                    serialfunc: e.serialfunc,
                    deserialfunc: e.deserialfunc,
                    initcond: e.initcond,
                    msfunc: e.msfunc,
                    minvfunc: e.minvfunc,
                    mstype: e.mstype.clone(),
                    mfinalfunc: e.mfinalfunc,
                    mfinalfunc_modify: e.mfinalfunc_modify,
                    minitcond: e.minitcond,
                    sortop: e.sortop,
                    hypothetical: e.hypothetical,
                    parallel: e.parallel,
                    to_sql_config: e.to_sql_config.clone(),
                })
            }

            SqlGraphEntity::Trigger(e) => SqlGraphEntitySnapshot::Trigger(e.clone()),
        }
    }
}

/// Convert snapshot back to runtime entity (TypeId will have placeholder value).
///
/// Note: The TypeId fields in the returned entities will have placeholder values
/// and should not be used for runtime type checking. This is only for roundtrip
/// compatibility.
impl From<SqlGraphEntitySnapshot> for SqlGraphEntity {
    fn from(snapshot: SqlGraphEntitySnapshot) -> Self {
        match snapshot {
            SqlGraphEntitySnapshot::ExtensionRoot(e) => SqlGraphEntity::ExtensionRoot(e),
            SqlGraphEntitySnapshot::Schema(e) => SqlGraphEntity::Schema(e),
            SqlGraphEntitySnapshot::CustomSql(e) => SqlGraphEntity::CustomSql(e),
            SqlGraphEntitySnapshot::Function(e) => SqlGraphEntity::Function(e),
            SqlGraphEntitySnapshot::Type(e) => SqlGraphEntity::Type(e),
            SqlGraphEntitySnapshot::BuiltinType(e) => SqlGraphEntity::BuiltinType(e),
            SqlGraphEntitySnapshot::Enum(e) => SqlGraphEntity::Enum(e),

            SqlGraphEntitySnapshot::Ord(e) => {
                SqlGraphEntity::Ord(crate::postgres_ord::entity::PostgresOrdEntity {
                    name: e.name,
                    file: e.file,
                    line: e.line,
                    full_path: e.full_path,
                    module_path: e.module_path,
                    id: core::any::TypeId::of::<crate::__PgrxInternalTypeIdPlaceholder>(),
                    to_sql_config: e.to_sql_config,
                })
            }

            SqlGraphEntitySnapshot::Hash(e) => {
                SqlGraphEntity::Hash(crate::postgres_hash::entity::PostgresHashEntity {
                    name: e.name,
                    file: e.file,
                    line: e.line,
                    full_path: e.full_path,
                    module_path: e.module_path,
                    id: core::any::TypeId::of::<crate::__PgrxInternalTypeIdPlaceholder>(),
                    to_sql_config: e.to_sql_config,
                })
            }

            SqlGraphEntitySnapshot::Aggregate(e) => {
                SqlGraphEntity::Aggregate(crate::aggregate::entity::PgAggregateEntity {
                    full_path: e.full_path,
                    module_path: e.module_path,
                    file: e.file,
                    line: e.line,
                    ty_id: core::any::TypeId::of::<crate::__PgrxInternalTypeIdPlaceholder>(),
                    name: e.name,
                    ordered_set: e.ordered_set,
                    args: e.args,
                    direct_args: e.direct_args,
                    stype: e.stype,
                    sfunc: e.sfunc,
                    finalfunc: e.finalfunc,
                    finalfunc_modify: e.finalfunc_modify,
                    combinefunc: e.combinefunc,
                    serialfunc: e.serialfunc,
                    deserialfunc: e.deserialfunc,
                    initcond: e.initcond,
                    msfunc: e.msfunc,
                    minvfunc: e.minvfunc,
                    mstype: e.mstype,
                    mfinalfunc: e.mfinalfunc,
                    mfinalfunc_modify: e.mfinalfunc_modify,
                    minitcond: e.minitcond,
                    sortop: e.sortop,
                    hypothetical: e.hypothetical,
                    parallel: e.parallel,
                    to_sql_config: e.to_sql_config,
                })
            }

            SqlGraphEntitySnapshot::Trigger(e) => SqlGraphEntity::Trigger(e),
        }
    }
}

/// Implement SqlGraphIdentifier for SqlGraphEntitySnapshot
impl crate::SqlGraphIdentifier for SqlGraphEntitySnapshot {
    fn dot_identifier(&self) -> String {
        match self {
            SqlGraphEntitySnapshot::ExtensionRoot(_) => "root".to_string(),
            SqlGraphEntitySnapshot::Schema(e) => format!("schema {}", e.name),
            SqlGraphEntitySnapshot::CustomSql(e) => format!("custom_sql {}:{}", e.file, e.line),
            SqlGraphEntitySnapshot::Function(e) => format!("fn {}", e.name),
            SqlGraphEntitySnapshot::Type(e) => format!("type {}", e.name),
            SqlGraphEntitySnapshot::BuiltinType(e) => format!("builtin {}", e),
            SqlGraphEntitySnapshot::Enum(e) => format!("enum {}", e.name),
            SqlGraphEntitySnapshot::Ord(e) => format!("ord {}", e.name),
            SqlGraphEntitySnapshot::Hash(e) => format!("hash {}", e.name),
            SqlGraphEntitySnapshot::Aggregate(e) => format!("aggregate {}", e.name),
            SqlGraphEntitySnapshot::Trigger(e) => format!("trigger {}", e.function_name),
        }
    }

    fn rust_identifier(&self) -> String {
        match self {
            SqlGraphEntitySnapshot::ExtensionRoot(_) => "extension_root".to_string(),
            SqlGraphEntitySnapshot::Schema(e) => e.module_path.to_string(),
            SqlGraphEntitySnapshot::CustomSql(e) => format!("{}::{}", e.file, e.line),
            SqlGraphEntitySnapshot::Function(e) => e.full_path.to_string(),
            SqlGraphEntitySnapshot::Type(e) => e.full_path.to_string(),
            SqlGraphEntitySnapshot::BuiltinType(e) => e.clone(),
            SqlGraphEntitySnapshot::Enum(e) => e.full_path.to_string(),
            SqlGraphEntitySnapshot::Ord(e) => e.full_path.to_string(),
            SqlGraphEntitySnapshot::Hash(e) => e.full_path.to_string(),
            SqlGraphEntitySnapshot::Aggregate(e) => e.full_path.to_string(),
            SqlGraphEntitySnapshot::Trigger(e) => e.module_path.to_string(),
        }
    }

    fn file(&self) -> Option<&'static str> {
        match self {
            SqlGraphEntitySnapshot::ExtensionRoot(_) => None,
            SqlGraphEntitySnapshot::Schema(e) => Some(e.file),
            SqlGraphEntitySnapshot::CustomSql(e) => Some(e.file),
            SqlGraphEntitySnapshot::Function(e) => Some(e.file),
            SqlGraphEntitySnapshot::Type(e) => Some(e.file),
            SqlGraphEntitySnapshot::BuiltinType(_) => None,
            SqlGraphEntitySnapshot::Enum(e) => Some(e.file),
            SqlGraphEntitySnapshot::Ord(e) => Some(e.file),
            SqlGraphEntitySnapshot::Hash(e) => Some(e.file),
            SqlGraphEntitySnapshot::Aggregate(e) => Some(e.file),
            SqlGraphEntitySnapshot::Trigger(e) => Some(e.file),
        }
    }

    fn line(&self) -> Option<u32> {
        match self {
            SqlGraphEntitySnapshot::ExtensionRoot(_) => None,
            SqlGraphEntitySnapshot::Schema(e) => Some(e.line),
            SqlGraphEntitySnapshot::CustomSql(e) => Some(e.line),
            SqlGraphEntitySnapshot::Function(e) => Some(e.line),
            SqlGraphEntitySnapshot::Type(e) => Some(e.line),
            SqlGraphEntitySnapshot::BuiltinType(_) => None,
            SqlGraphEntitySnapshot::Enum(e) => Some(e.line),
            SqlGraphEntitySnapshot::Ord(e) => Some(e.line),
            SqlGraphEntitySnapshot::Hash(e) => Some(e.line),
            SqlGraphEntitySnapshot::Aggregate(e) => Some(e.line),
            SqlGraphEntitySnapshot::Trigger(e) => Some(e.line),
        }
    }
}
