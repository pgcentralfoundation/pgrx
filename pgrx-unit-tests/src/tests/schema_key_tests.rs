use std::collections::BTreeMap;
use std::error::Error as StdError;

use pgrx::array::FlatArray;
use pgrx::nullable::Nullable;
use pgrx::pgrx_sql_entity_graph::metadata::{
    ArgumentError, ReturnsError, ReturnsRef, SqlMappingRef, SqlTranslatable,
};
use pgrx::prelude::*;
use pgrx::{AnyArray, AnyElement, AnyNumeric, Inet, Internal, Json, JsonB, PgRelation, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct ManualSchemaKeyType;

type ManualSchemaKeyAlias = ManualSchemaKeyType;

const MANUAL_SCHEMA_KEY: &str = pgrx::pgrx_resolved_type!(ManualSchemaKeyType);

unsafe impl SqlTranslatable for ManualSchemaKeyType {
    const SCHEMA_KEY: &'static str = MANUAL_SCHEMA_KEY;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> =
        Ok(SqlMappingRef::literal("manual_schema_key_type"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("manual_schema_key_type")));
}

mod nested_manual {
    use super::*;

    #[derive(Debug)]
    pub struct DefinitionSiteType;

    pub const DEFINITION_SITE_SCHEMA_KEY: &str = pgrx::pgrx_resolved_type!(DefinitionSiteType);

    unsafe impl SqlTranslatable for DefinitionSiteType {
        const SCHEMA_KEY: &'static str = DEFINITION_SITE_SCHEMA_KEY;
        const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> =
            Ok(SqlMappingRef::literal("definition_site_type"));
        const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
            Ok(ReturnsRef::One(SqlMappingRef::literal("definition_site_type")));
    }
}

use nested_manual::DefinitionSiteType;

type ReexportedDefinitionSiteType = DefinitionSiteType;

#[derive(PostgresType, Serialize, Deserialize, Debug, PartialEq)]
pub struct DerivedSchemaKeyType {
    value: i32,
}

#[derive(PostgresEnum, Debug, PartialEq, Eq)]
pub enum DerivedSchemaKeyEnum {
    Alpha,
    Beta,
}

fn assert_same_schema_key<Left, Right>()
where
    Left: ?Sized + SqlTranslatable,
    Right: ?Sized + SqlTranslatable,
{
    assert_eq!(Left::SCHEMA_KEY, Right::SCHEMA_KEY);
}

fn assert_distinct_schema_key<Left, Right>()
where
    Left: ?Sized + SqlTranslatable,
    Right: ?Sized + SqlTranslatable,
{
    assert_ne!(Left::SCHEMA_KEY, Right::SCHEMA_KEY);
}

#[test]
fn representative_leaf_types_have_non_empty_distinct_schema_keys() {
    let leaf_keys = [
        ("bool", <bool as SqlTranslatable>::SCHEMA_KEY),
        ("i8", <i8 as SqlTranslatable>::SCHEMA_KEY),
        ("i16", <i16 as SqlTranslatable>::SCHEMA_KEY),
        ("i32", <i32 as SqlTranslatable>::SCHEMA_KEY),
        ("i64", <i64 as SqlTranslatable>::SCHEMA_KEY),
        ("u8", <u8 as SqlTranslatable>::SCHEMA_KEY),
        ("String", <String as SqlTranslatable>::SCHEMA_KEY),
        ("str", <str as SqlTranslatable>::SCHEMA_KEY),
        ("Date", <Date as SqlTranslatable>::SCHEMA_KEY),
        ("Time", <Time as SqlTranslatable>::SCHEMA_KEY),
        ("Timestamp", <Timestamp as SqlTranslatable>::SCHEMA_KEY),
        ("TimestampWithTimeZone", <TimestampWithTimeZone as SqlTranslatable>::SCHEMA_KEY),
        ("TimeWithTimeZone", <TimeWithTimeZone as SqlTranslatable>::SCHEMA_KEY),
        ("Interval", <Interval as SqlTranslatable>::SCHEMA_KEY),
        ("AnyArray", <AnyArray as SqlTranslatable>::SCHEMA_KEY),
        ("AnyElement", <AnyElement as SqlTranslatable>::SCHEMA_KEY),
        ("AnyNumeric", <AnyNumeric as SqlTranslatable>::SCHEMA_KEY),
        ("Json", <Json as SqlTranslatable>::SCHEMA_KEY),
        ("JsonB", <JsonB as SqlTranslatable>::SCHEMA_KEY),
        ("Uuid", <Uuid as SqlTranslatable>::SCHEMA_KEY),
        ("Inet", <Inet as SqlTranslatable>::SCHEMA_KEY),
        ("Internal", <Internal as SqlTranslatable>::SCHEMA_KEY),
        ("PgRelation", <PgRelation as SqlTranslatable>::SCHEMA_KEY),
        ("Range<i32>", <Range<i32> as SqlTranslatable>::SCHEMA_KEY),
        ("Range<Date>", <Range<Date> as SqlTranslatable>::SCHEMA_KEY),
        ("Range<Timestamp>", <Range<Timestamp> as SqlTranslatable>::SCHEMA_KEY),
        (
            "Range<TimestampWithTimeZone>",
            <Range<TimestampWithTimeZone> as SqlTranslatable>::SCHEMA_KEY,
        ),
        ("ManualSchemaKeyType", <ManualSchemaKeyType as SqlTranslatable>::SCHEMA_KEY),
        ("DefinitionSiteType", <DefinitionSiteType as SqlTranslatable>::SCHEMA_KEY),
        ("DerivedSchemaKeyType", <DerivedSchemaKeyType as SqlTranslatable>::SCHEMA_KEY),
        ("DerivedSchemaKeyEnum", <DerivedSchemaKeyEnum as SqlTranslatable>::SCHEMA_KEY),
    ];

    let mut seen = BTreeMap::new();
    for (name, key) in leaf_keys {
        assert!(!key.is_empty(), "{name} should not have an empty schema key");
        if let Some(previous) = seen.insert(key, name) {
            panic!("{name} and {previous} unexpectedly share schema key {key}");
        }
    }
}

#[test]
fn wrapper_types_forward_to_their_inner_schema_key() {
    assert_same_schema_key::<&str, str>();

    assert_same_schema_key::<Option<i32>, i32>();
    assert_same_schema_key::<Result<i32, std::io::Error>, i32>();
    assert_same_schema_key::<Vec<i32>, i32>();
    assert_same_schema_key::<Vec<u8>, u8>();
    assert_same_schema_key::<&i32, i32>();
    assert_same_schema_key::<*mut i32, i32>();
    assert_same_schema_key::<Nullable<i32>, i32>();
    assert_same_schema_key::<Array<'static, i32>, i32>();
    assert_same_schema_key::<VariadicArray<'static, i32>, i32>();
    assert_same_schema_key::<FlatArray<'static, i32>, i32>();
    assert_same_schema_key::<PgBox<i32, AllocatedByRust>, i32>();
    assert_same_schema_key::<PgBox<i32, AllocatedByPostgres>, i32>();
    assert_same_schema_key::<SetOfIterator<'static, i32>, i32>();

    assert_same_schema_key::<Option<ManualSchemaKeyType>, ManualSchemaKeyType>();
    assert_same_schema_key::<Result<ManualSchemaKeyType, Box<dyn StdError>>, ManualSchemaKeyType>();
}

#[test]
fn table_iterators_keep_their_own_schema_identity() {
    type OneColumnTable = TableIterator<'static, (name!(id, i32),)>;
    type TwoColumnTable = TableIterator<'static, (name!(id, i32), name!(label, String))>;

    assert_distinct_schema_key::<OneColumnTable, i32>();
    assert_distinct_schema_key::<TwoColumnTable, i32>();
    assert_distinct_schema_key::<OneColumnTable, TwoColumnTable>();
    assert_same_schema_key::<Result<OneColumnTable, std::io::Error>, OneColumnTable>();
}

#[test]
fn custom_types_use_definition_site_schema_keys() {
    assert_eq!(<ManualSchemaKeyType as SqlTranslatable>::SCHEMA_KEY, MANUAL_SCHEMA_KEY);
    assert_eq!(<ManualSchemaKeyAlias as SqlTranslatable>::SCHEMA_KEY, MANUAL_SCHEMA_KEY);

    assert_eq!(
        <DefinitionSiteType as SqlTranslatable>::SCHEMA_KEY,
        nested_manual::DEFINITION_SITE_SCHEMA_KEY
    );
    assert_eq!(
        <ReexportedDefinitionSiteType as SqlTranslatable>::SCHEMA_KEY,
        nested_manual::DEFINITION_SITE_SCHEMA_KEY
    );

    assert_eq!(
        <DerivedSchemaKeyType as SqlTranslatable>::SCHEMA_KEY,
        pgrx::pgrx_resolved_type!(DerivedSchemaKeyType)
    );
    assert_eq!(
        <DerivedSchemaKeyEnum as SqlTranslatable>::SCHEMA_KEY,
        pgrx::pgrx_resolved_type!(DerivedSchemaKeyEnum)
    );
}
