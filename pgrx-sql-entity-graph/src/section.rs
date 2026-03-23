//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use crate::aggregate::entity::{AggregateTypeEntity, PgAggregateEntity};
use crate::aggregate::{FinalizeModify, ParallelOption};
use crate::extension_sql::entity::{ExtensionSqlEntity, SqlDeclaredEntity, SqlDeclaredEntityData};
use crate::extern_args::ExternArgs;
use crate::metadata::{
    ArgumentError, FunctionMetadataTypeEntity, ReturnsError, ReturnsRef, SqlMappingRef,
};
use crate::pg_extern::entity::{
    PgCastEntity, PgExternArgumentEntity, PgExternEntity, PgExternReturnEntity,
    PgExternReturnEntityIteratedItem, PgOperatorEntity,
};
use crate::pg_trigger::entity::PgTriggerEntity;
use crate::positioning_ref::PositioningRef;
use crate::postgres_enum::entity::PostgresEnumEntity;
use crate::postgres_hash::entity::PostgresHashEntity;
use crate::postgres_ord::entity::PostgresOrdEntity;
use crate::postgres_type::entity::PostgresTypeEntity;
use crate::schema::entity::SchemaEntity;
use crate::to_sql::entity::ToSqlConfigEntity;
use crate::{SqlGraphEntity, UsedTypeEntity};
use eyre::{Result, bail, eyre};

pub const ELF_SECTION_NAME: &str = ".pgrx_schema";
pub const MACHO_SEGMENT_NAME: &str = "__DATA";
pub const MACHO_SECTION_NAME: &str = "__pgrx_schema";
pub const MACHO_SECTION_PATH: &str = "__DATA,__pgrx_schema";

pub const ENTITY_SCHEMA: u8 = 1;
pub const ENTITY_CUSTOM_SQL: u8 = 2;
pub const ENTITY_FUNCTION: u8 = 3;
pub const ENTITY_TYPE: u8 = 4;
pub const ENTITY_ENUM: u8 = 5;
pub const ENTITY_ORD: u8 = 6;
pub const ENTITY_HASH: u8 = 7;
pub const ENTITY_AGGREGATE: u8 = 8;
pub const ENTITY_TRIGGER: u8 = 9;

pub const POSITIONING_REF_FULL_PATH: u8 = 1;
pub const POSITIONING_REF_NAME: u8 = 2;

pub const SQL_DECLARED_TYPE: u8 = 1;
pub const SQL_DECLARED_ENUM: u8 = 2;
pub const SQL_DECLARED_FUNCTION: u8 = 3;

pub const SQL_MAPPING_AS: u8 = 1;
pub const SQL_MAPPING_ARRAY: u8 = 2;
pub const SQL_MAPPING_NUMERIC: u8 = 3;
pub const SQL_MAPPING_COMPOSITE: u8 = 4;
pub const SQL_MAPPING_SKIP: u8 = 5;

pub const RETURNS_ONE: u8 = 1;
pub const RETURNS_SET_OF: u8 = 2;
pub const RETURNS_TABLE: u8 = 3;

pub const ARG_ERROR_SET_OF: u8 = 1;
pub const ARG_ERROR_TABLE: u8 = 2;
pub const ARG_ERROR_BARE_U8: u8 = 3;
pub const ARG_ERROR_SKIP_IN_ARRAY: u8 = 4;
pub const ARG_ERROR_DATUM: u8 = 5;
pub const ARG_ERROR_NOT_VALID: u8 = 6;

pub const RETURNS_ERROR_NESTED_SET_OF: u8 = 1;
pub const RETURNS_ERROR_NESTED_TABLE: u8 = 2;
pub const RETURNS_ERROR_SET_OF_CONTAINING_TABLE: u8 = 3;
pub const RETURNS_ERROR_TABLE_CONTAINING_SET_OF: u8 = 4;
pub const RETURNS_ERROR_SET_OF_IN_ARRAY: u8 = 5;
pub const RETURNS_ERROR_TABLE_IN_ARRAY: u8 = 6;
pub const RETURNS_ERROR_BARE_U8: u8 = 7;
pub const RETURNS_ERROR_SKIP_IN_ARRAY: u8 = 8;
pub const RETURNS_ERROR_DATUM: u8 = 9;

pub const EXTERN_ARG_CREATE_OR_REPLACE: u8 = 1;
pub const EXTERN_ARG_IMMUTABLE: u8 = 2;
pub const EXTERN_ARG_STRICT: u8 = 3;
pub const EXTERN_ARG_STABLE: u8 = 4;
pub const EXTERN_ARG_VOLATILE: u8 = 5;
pub const EXTERN_ARG_RAW: u8 = 6;
pub const EXTERN_ARG_NO_GUARD: u8 = 7;
pub const EXTERN_ARG_SECURITY_DEFINER: u8 = 8;
pub const EXTERN_ARG_SECURITY_INVOKER: u8 = 9;
pub const EXTERN_ARG_PARALLEL_SAFE: u8 = 10;
pub const EXTERN_ARG_PARALLEL_UNSAFE: u8 = 11;
pub const EXTERN_ARG_PARALLEL_RESTRICTED: u8 = 12;
pub const EXTERN_ARG_SHOULD_PANIC: u8 = 13;
pub const EXTERN_ARG_SCHEMA: u8 = 14;
pub const EXTERN_ARG_SUPPORT: u8 = 15;
pub const EXTERN_ARG_NAME: u8 = 16;
pub const EXTERN_ARG_COST: u8 = 17;
pub const EXTERN_ARG_REQUIRES: u8 = 18;

pub const OPERATOR_CAST_DEFAULT: u8 = 1;
pub const OPERATOR_CAST_ASSIGNMENT: u8 = 2;
pub const OPERATOR_CAST_IMPLICIT: u8 = 3;

pub const EXTERN_RET_NONE: u8 = 1;
pub const EXTERN_RET_TYPE: u8 = 2;
pub const EXTERN_RET_SET_OF: u8 = 3;
pub const EXTERN_RET_ITERATED: u8 = 4;
pub const EXTERN_RET_TRIGGER: u8 = 5;

pub const AGGREGATE_FINALIZE_READ_ONLY: u8 = 1;
pub const AGGREGATE_FINALIZE_SHAREABLE: u8 = 2;
pub const AGGREGATE_FINALIZE_READ_WRITE: u8 = 3;

pub const AGGREGATE_PARALLEL_SAFE: u8 = 1;
pub const AGGREGATE_PARALLEL_RESTRICTED: u8 = 2;
pub const AGGREGATE_PARALLEL_UNSAFE: u8 = 3;

pub fn is_schema_section_name(name: &str) -> bool {
    name == ELF_SECTION_NAME || name == MACHO_SECTION_NAME || name == MACHO_SECTION_PATH
}

#[macro_export]
macro_rules! __pgrx_schema_entry {
    ($name:ident, $len:expr, $payload:expr) => {
        #[doc(hidden)]
        #[used]
        #[allow(non_upper_case_globals)]
        #[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__pgrx_schema"))]
        #[cfg_attr(not(target_os = "macos"), unsafe(link_section = ".pgrx_schema"))]
        static $name: [u8; $len] = $payload;
    };
}

pub const fn bytes_len(len: usize) -> usize {
    4 + len
}

pub const fn str_len(value: &str) -> usize {
    bytes_len(value.len())
}

pub const fn bool_len() -> usize {
    1
}

pub const fn u8_len() -> usize {
    1
}

pub const fn u32_len() -> usize {
    4
}

pub const fn opt_len(inner: Option<usize>) -> usize {
    1 + match inner {
        Some(inner) => inner,
        None => 0,
    }
}

pub const fn list_len(items: &[usize]) -> usize {
    let mut total = 4;
    let mut i = 0;
    while i < items.len() {
        total += items[i];
        i += 1;
    }
    total
}

pub const fn sql_mapping_len(value: SqlMappingRef) -> usize {
    match value {
        SqlMappingRef::As(sql) | SqlMappingRef::Array(sql) => u8_len() + str_len(sql),
        SqlMappingRef::Numeric { .. } => {
            u8_len() + bool_len() + u32_len() + bool_len() + u32_len() + bool_len()
        }
        SqlMappingRef::Composite { .. } => u8_len() + bool_len(),
        SqlMappingRef::Skip => u8_len(),
    }
}

pub const fn returns_len(value: ReturnsRef) -> usize {
    match value {
        ReturnsRef::One(mapping) | ReturnsRef::SetOf(mapping) => {
            u8_len() + sql_mapping_len(mapping)
        }
        ReturnsRef::Table(items) => {
            let mut total = u8_len() + u32_len();
            let mut i = 0;
            while i < items.len() {
                total += sql_mapping_len(items[i]);
                i += 1;
            }
            total
        }
    }
}

pub const fn argument_error_len(value: ArgumentError) -> usize {
    match value {
        ArgumentError::NotValidAsArgument(name) => u8_len() + str_len(name),
        _ => u8_len(),
    }
}

pub const fn argument_sql_len(value: Result<SqlMappingRef, ArgumentError>) -> usize {
    u8_len()
        + match value {
            Ok(mapping) => sql_mapping_len(mapping),
            Err(err) => argument_error_len(err),
        }
}

pub const fn returns_error_len(_value: ReturnsError) -> usize {
    u8_len()
}

pub const fn return_sql_len(value: Result<ReturnsRef, ReturnsError>) -> usize {
    u8_len()
        + match value {
            Ok(returns) => returns_len(returns),
            Err(err) => returns_error_len(err),
        }
}

#[derive(Clone, Copy)]
pub struct EntryWriter<const N: usize> {
    buf: [u8; N],
    pos: usize,
}

impl<const N: usize> EntryWriter<N> {
    pub const fn new() -> Self {
        Self { buf: [0; N], pos: 0 }
    }

    pub const fn u8(mut self, value: u8) -> Self {
        self.buf[self.pos] = value;
        self.pos += 1;
        self
    }

    pub const fn bool(self, value: bool) -> Self {
        self.u8(if value { 1 } else { 0 })
    }

    pub const fn u32(self, value: u32) -> Self {
        let [b0, b1, b2, b3] = value.to_le_bytes();
        self.u8(b0).u8(b1).u8(b2).u8(b3)
    }

    pub const fn bytes(mut self, value: &[u8]) -> Self {
        let mut i = 0;
        while i < value.len() {
            self.buf[self.pos] = value[i];
            self.pos += 1;
            i += 1;
        }
        self
    }

    pub const fn str(self, value: &str) -> Self {
        self.u32(value.len() as u32).bytes(value.as_bytes())
    }

    pub const fn sql_mapping(self, value: SqlMappingRef) -> Self {
        match value {
            SqlMappingRef::As(sql) => self.u8(SQL_MAPPING_AS).str(sql),
            SqlMappingRef::Array(sql) => self.u8(SQL_MAPPING_ARRAY).str(sql),
            SqlMappingRef::Numeric { precision, scale, array_brackets } => self
                .u8(SQL_MAPPING_NUMERIC)
                .bool(precision.is_some())
                .u32(match precision {
                    Some(value) => value,
                    None => 0,
                })
                .bool(scale.is_some())
                .u32(match scale {
                    Some(value) => value,
                    None => 0,
                })
                .bool(array_brackets),
            SqlMappingRef::Composite { array_brackets } => {
                self.u8(SQL_MAPPING_COMPOSITE).bool(array_brackets)
            }
            SqlMappingRef::Skip => self.u8(SQL_MAPPING_SKIP),
        }
    }

    pub const fn returns(self, value: ReturnsRef) -> Self {
        match value {
            ReturnsRef::One(mapping) => self.u8(RETURNS_ONE).sql_mapping(mapping),
            ReturnsRef::SetOf(mapping) => self.u8(RETURNS_SET_OF).sql_mapping(mapping),
            ReturnsRef::Table(items) => {
                let mut writer = self.u8(RETURNS_TABLE).u32(items.len() as u32);
                let mut i = 0;
                while i < items.len() {
                    writer = writer.sql_mapping(items[i]);
                    i += 1;
                }
                writer
            }
        }
    }

    pub const fn argument_error(self, value: ArgumentError) -> Self {
        match value {
            ArgumentError::SetOf => self.u8(ARG_ERROR_SET_OF),
            ArgumentError::Table => self.u8(ARG_ERROR_TABLE),
            ArgumentError::BareU8 => self.u8(ARG_ERROR_BARE_U8),
            ArgumentError::SkipInArray => self.u8(ARG_ERROR_SKIP_IN_ARRAY),
            ArgumentError::Datum => self.u8(ARG_ERROR_DATUM),
            ArgumentError::NotValidAsArgument(name) => self.u8(ARG_ERROR_NOT_VALID).str(name),
        }
    }

    pub const fn argument_sql(self, value: Result<SqlMappingRef, ArgumentError>) -> Self {
        match value {
            Ok(mapping) => self.u8(1).sql_mapping(mapping),
            Err(err) => self.u8(2).argument_error(err),
        }
    }

    pub const fn returns_error(self, value: ReturnsError) -> Self {
        match value {
            ReturnsError::NestedSetOf => self.u8(RETURNS_ERROR_NESTED_SET_OF),
            ReturnsError::NestedTable => self.u8(RETURNS_ERROR_NESTED_TABLE),
            ReturnsError::SetOfContainingTable => self.u8(RETURNS_ERROR_SET_OF_CONTAINING_TABLE),
            ReturnsError::TableContainingSetOf => self.u8(RETURNS_ERROR_TABLE_CONTAINING_SET_OF),
            ReturnsError::SetOfInArray => self.u8(RETURNS_ERROR_SET_OF_IN_ARRAY),
            ReturnsError::TableInArray => self.u8(RETURNS_ERROR_TABLE_IN_ARRAY),
            ReturnsError::BareU8 => self.u8(RETURNS_ERROR_BARE_U8),
            ReturnsError::SkipInArray => self.u8(RETURNS_ERROR_SKIP_IN_ARRAY),
            ReturnsError::Datum => self.u8(RETURNS_ERROR_DATUM),
        }
    }

    pub const fn return_sql(self, value: Result<ReturnsRef, ReturnsError>) -> Self {
        match value {
            Ok(returns) => self.u8(1).returns(returns),
            Err(err) => self.u8(2).returns_error(err),
        }
    }

    pub const fn finish(self) -> [u8; N] {
        if self.pos != N {
            panic!("pgrx schema entry length mismatch");
        }
        self.buf
    }
}

impl<const N: usize> Default for EntryWriter<N> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EntryReader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> EntryReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.bytes.len()
    }

    pub fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.pos)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        if self.remaining() < 1 {
            bail!("unexpected end of schema entry");
        }
        let value = self.bytes[self.pos];
        self.pos += 1;
        Ok(value)
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        match self.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(eyre!("invalid bool tag in schema entry: {other}")),
        }
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        if self.remaining() < 4 {
            bail!("unexpected end of schema entry");
        }
        let start = self.pos;
        self.pos += 4;
        Ok(u32::from_le_bytes(
            self.bytes[start..self.pos].try_into().expect("checked slice length"),
        ))
    }

    pub fn read_bytes(&mut self) -> Result<&'a [u8]> {
        let len = self.read_u32()? as usize;
        if self.remaining() < len {
            bail!("unexpected end of schema entry");
        }
        let start = self.pos;
        self.pos += len;
        Ok(&self.bytes[start..self.pos])
    }

    pub fn read_string(&mut self) -> Result<String> {
        let bytes = self.read_bytes()?;
        Ok(std::str::from_utf8(bytes)
            .map_err(|err| eyre!("schema entry contained invalid utf8: {err}"))?
            .to_owned())
    }

    pub fn read_option_string(&mut self) -> Result<Option<String>> {
        if self.read_bool()? { Ok(Some(self.read_string()?)) } else { Ok(None) }
    }

    pub fn read_sql_mapping(&mut self) -> Result<SqlMappingRef> {
        match self.read_u8()? {
            SQL_MAPPING_AS => Ok(SqlMappingRef::As(leak_string(self.read_string()?))),
            SQL_MAPPING_ARRAY => Ok(SqlMappingRef::Array(leak_string(self.read_string()?))),
            SQL_MAPPING_NUMERIC => {
                let has_precision = self.read_bool()?;
                let precision = self.read_u32()?;
                let has_scale = self.read_bool()?;
                let scale = self.read_u32()?;
                let array_brackets = self.read_bool()?;
                Ok(SqlMappingRef::Numeric {
                    precision: has_precision.then_some(precision),
                    scale: has_scale.then_some(scale),
                    array_brackets,
                })
            }
            SQL_MAPPING_COMPOSITE => {
                Ok(SqlMappingRef::Composite { array_brackets: self.read_bool()? })
            }
            SQL_MAPPING_SKIP => Ok(SqlMappingRef::Skip),
            other => Err(eyre!("invalid sql mapping tag in schema entry: {other}")),
        }
    }

    pub fn read_returns(&mut self) -> Result<ReturnsRef> {
        match self.read_u8()? {
            RETURNS_ONE => Ok(ReturnsRef::One(self.read_sql_mapping()?)),
            RETURNS_SET_OF => Ok(ReturnsRef::SetOf(self.read_sql_mapping()?)),
            RETURNS_TABLE => {
                let count = self.read_u32()? as usize;
                let mut items = Vec::with_capacity(count);
                for _ in 0..count {
                    items.push(self.read_sql_mapping()?);
                }
                Ok(ReturnsRef::Table(Box::leak(items.into_boxed_slice())))
            }
            other => Err(eyre!("invalid returns tag in schema entry: {other}")),
        }
    }

    pub fn read_argument_error(&mut self) -> Result<ArgumentError> {
        match self.read_u8()? {
            ARG_ERROR_SET_OF => Ok(ArgumentError::SetOf),
            ARG_ERROR_TABLE => Ok(ArgumentError::Table),
            ARG_ERROR_BARE_U8 => Ok(ArgumentError::BareU8),
            ARG_ERROR_SKIP_IN_ARRAY => Ok(ArgumentError::SkipInArray),
            ARG_ERROR_DATUM => Ok(ArgumentError::Datum),
            ARG_ERROR_NOT_VALID => {
                Ok(ArgumentError::NotValidAsArgument(leak_string(self.read_string()?)))
            }
            other => Err(eyre!("invalid argument error tag in schema entry: {other}")),
        }
    }

    pub fn read_argument_sql(&mut self) -> Result<Result<SqlMappingRef, ArgumentError>> {
        match self.read_u8()? {
            1 => Ok(Ok(self.read_sql_mapping()?)),
            2 => Ok(Err(self.read_argument_error()?)),
            other => Err(eyre!("invalid argument sql tag in schema entry: {other}")),
        }
    }

    pub fn read_returns_error(&mut self) -> Result<ReturnsError> {
        match self.read_u8()? {
            RETURNS_ERROR_NESTED_SET_OF => Ok(ReturnsError::NestedSetOf),
            RETURNS_ERROR_NESTED_TABLE => Ok(ReturnsError::NestedTable),
            RETURNS_ERROR_SET_OF_CONTAINING_TABLE => Ok(ReturnsError::SetOfContainingTable),
            RETURNS_ERROR_TABLE_CONTAINING_SET_OF => Ok(ReturnsError::TableContainingSetOf),
            RETURNS_ERROR_SET_OF_IN_ARRAY => Ok(ReturnsError::SetOfInArray),
            RETURNS_ERROR_TABLE_IN_ARRAY => Ok(ReturnsError::TableInArray),
            RETURNS_ERROR_BARE_U8 => Ok(ReturnsError::BareU8),
            RETURNS_ERROR_SKIP_IN_ARRAY => Ok(ReturnsError::SkipInArray),
            RETURNS_ERROR_DATUM => Ok(ReturnsError::Datum),
            other => Err(eyre!("invalid returns error tag in schema entry: {other}")),
        }
    }

    pub fn read_return_sql(&mut self) -> Result<Result<ReturnsRef, ReturnsError>> {
        match self.read_u8()? {
            1 => Ok(Ok(self.read_returns()?)),
            2 => Ok(Err(self.read_returns_error()?)),
            other => Err(eyre!("invalid return sql tag in schema entry: {other}")),
        }
    }

    pub fn read_positioning_ref(&mut self) -> Result<PositioningRef> {
        match self.read_u8()? {
            POSITIONING_REF_FULL_PATH => Ok(PositioningRef::FullPath(self.read_string()?)),
            POSITIONING_REF_NAME => Ok(PositioningRef::Name(self.read_string()?)),
            other => Err(eyre!("invalid positioning ref tag in schema entry: {other}")),
        }
    }

    pub fn read_to_sql_config(&mut self) -> Result<ToSqlConfigEntity> {
        let enabled = self.read_bool()?;
        let has_content = self.read_bool()?;
        let content = if has_content { Some(leak_string(self.read_string()?)) } else { None };
        Ok(ToSqlConfigEntity { enabled, content })
    }

    pub fn read_function_metadata_type(&mut self) -> Result<FunctionMetadataTypeEntity> {
        Ok(FunctionMetadataTypeEntity {
            schema_key: leak_string(self.read_string()?),
            argument_sql: self.read_argument_sql()?.map(Into::into),
            return_sql: self.read_return_sql()?.map(Into::into),
        })
    }

    pub fn read_used_type(&mut self) -> Result<UsedTypeEntity> {
        Ok(UsedTypeEntity {
            ty_source: leak_string(self.read_string()?),
            full_path: leak_string(self.read_string()?),
            composite_type: leak_option_string(self.read_option_string()?),
            variadic: self.read_bool()?,
            default: leak_option_string(self.read_option_string()?),
            optional: self.read_bool()?,
            metadata: self.read_function_metadata_type()?,
        })
    }

    pub fn read_pg_extern_argument(&mut self) -> Result<PgExternArgumentEntity> {
        Ok(PgExternArgumentEntity {
            pattern: leak_string(self.read_string()?),
            used_ty: self.read_used_type()?,
        })
    }

    pub fn read_pg_extern_return(&mut self) -> Result<PgExternReturnEntity> {
        match self.read_u8()? {
            EXTERN_RET_NONE => Ok(PgExternReturnEntity::None),
            EXTERN_RET_TYPE => Ok(PgExternReturnEntity::Type { ty: self.read_used_type()? }),
            EXTERN_RET_SET_OF => Ok(PgExternReturnEntity::SetOf { ty: self.read_used_type()? }),
            EXTERN_RET_ITERATED => {
                let count = self.read_u32()? as usize;
                let mut tys = Vec::with_capacity(count);
                for _ in 0..count {
                    tys.push(PgExternReturnEntityIteratedItem {
                        name: leak_option_string(self.read_option_string()?),
                        ty: self.read_used_type()?,
                    });
                }
                Ok(PgExternReturnEntity::Iterated { tys })
            }
            EXTERN_RET_TRIGGER => Ok(PgExternReturnEntity::Trigger),
            other => Err(eyre!("invalid extern return tag in schema entry: {other}")),
        }
    }

    pub fn read_extern_arg(&mut self) -> Result<ExternArgs> {
        match self.read_u8()? {
            EXTERN_ARG_CREATE_OR_REPLACE => Ok(ExternArgs::CreateOrReplace),
            EXTERN_ARG_IMMUTABLE => Ok(ExternArgs::Immutable),
            EXTERN_ARG_STRICT => Ok(ExternArgs::Strict),
            EXTERN_ARG_STABLE => Ok(ExternArgs::Stable),
            EXTERN_ARG_VOLATILE => Ok(ExternArgs::Volatile),
            EXTERN_ARG_RAW => Ok(ExternArgs::Raw),
            EXTERN_ARG_NO_GUARD => Ok(ExternArgs::NoGuard),
            EXTERN_ARG_SECURITY_DEFINER => Ok(ExternArgs::SecurityDefiner),
            EXTERN_ARG_SECURITY_INVOKER => Ok(ExternArgs::SecurityInvoker),
            EXTERN_ARG_PARALLEL_SAFE => Ok(ExternArgs::ParallelSafe),
            EXTERN_ARG_PARALLEL_UNSAFE => Ok(ExternArgs::ParallelUnsafe),
            EXTERN_ARG_PARALLEL_RESTRICTED => Ok(ExternArgs::ParallelRestricted),
            EXTERN_ARG_SHOULD_PANIC => Ok(ExternArgs::ShouldPanic(self.read_string()?)),
            EXTERN_ARG_SCHEMA => Ok(ExternArgs::Schema(self.read_string()?)),
            EXTERN_ARG_SUPPORT => Ok(ExternArgs::Support(self.read_positioning_ref()?)),
            EXTERN_ARG_NAME => Ok(ExternArgs::Name(self.read_string()?)),
            EXTERN_ARG_COST => Ok(ExternArgs::Cost(self.read_string()?)),
            EXTERN_ARG_REQUIRES => {
                let count = self.read_u32()? as usize;
                let mut items = Vec::with_capacity(count);
                for _ in 0..count {
                    items.push(self.read_positioning_ref()?);
                }
                Ok(ExternArgs::Requires(items))
            }
            other => Err(eyre!("invalid extern arg tag in schema entry: {other}")),
        }
    }

    pub fn read_search_path(&mut self) -> Result<Option<Vec<&'static str>>> {
        if !self.read_bool()? {
            return Ok(None);
        }
        let count = self.read_u32()? as usize;
        let mut values = Vec::with_capacity(count);
        for _ in 0..count {
            values.push(leak_string(self.read_string()?));
        }
        Ok(Some(values))
    }

    pub fn read_operator(&mut self) -> Result<PgOperatorEntity> {
        Ok(PgOperatorEntity {
            opname: leak_option_string(self.read_option_string()?),
            commutator: leak_option_string(self.read_option_string()?),
            negator: leak_option_string(self.read_option_string()?),
            restrict: leak_option_string(self.read_option_string()?),
            join: leak_option_string(self.read_option_string()?),
            hashes: self.read_bool()?,
            merges: self.read_bool()?,
        })
    }

    pub fn read_cast(&mut self) -> Result<PgCastEntity> {
        match self.read_u8()? {
            OPERATOR_CAST_DEFAULT => Ok(PgCastEntity::Default),
            OPERATOR_CAST_ASSIGNMENT => Ok(PgCastEntity::Assignment),
            OPERATOR_CAST_IMPLICIT => Ok(PgCastEntity::Implicit),
            other => Err(eyre!("invalid cast tag in schema entry: {other}")),
        }
    }

    pub fn read_sql_declared(&mut self) -> Result<SqlDeclaredEntity> {
        let kind = self.read_u8()?;
        let name = self.read_string()?;

        match kind {
            SQL_DECLARED_TYPE | SQL_DECLARED_ENUM => {
                let schema_key = self.read_string()?;
                let sql = match self.read_argument_sql()?.map(crate::metadata::SqlMapping::from) {
                    Ok(crate::metadata::SqlMapping::As(sql)) => sql,
                    Ok(other) => {
                        bail!("invalid SQL declaration mapping in schema entry: {other:?}")
                    }
                    Err(err) => return Err(err.into()),
                };
                let data = SqlDeclaredEntityData { sql, name, schema_key };
                Ok(match kind {
                    SQL_DECLARED_TYPE => SqlDeclaredEntity::Type(data),
                    SQL_DECLARED_ENUM => SqlDeclaredEntity::Enum(data),
                    _ => unreachable!(),
                })
            }
            SQL_DECLARED_FUNCTION => {
                let sql = name
                    .split("::")
                    .last()
                    .ok_or_else(|| eyre!("function declaration was missing a name"))?
                    .to_owned();
                Ok(SqlDeclaredEntity::Function(SqlDeclaredEntityData {
                    sql,
                    schema_key: name.clone(),
                    name,
                }))
            }
            other => Err(eyre!("invalid SQL declared tag in schema entry: {other}")),
        }
    }

    pub fn read_aggregate_type(&mut self) -> Result<AggregateTypeEntity> {
        Ok(AggregateTypeEntity {
            name: leak_option_string(self.read_option_string()?),
            used_ty: self.read_used_type()?,
        })
    }

    pub fn read_aggregate_type_list(&mut self) -> Result<Vec<AggregateTypeEntity>> {
        let count = self.read_u32()? as usize;
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(self.read_aggregate_type()?);
        }
        Ok(items)
    }

    pub fn read_finalize_modify(&mut self) -> Result<FinalizeModify> {
        match self.read_u8()? {
            AGGREGATE_FINALIZE_READ_ONLY => Ok(FinalizeModify::ReadOnly),
            AGGREGATE_FINALIZE_SHAREABLE => Ok(FinalizeModify::Shareable),
            AGGREGATE_FINALIZE_READ_WRITE => Ok(FinalizeModify::ReadWrite),
            other => Err(eyre!("invalid finalize modify tag in schema entry: {other}")),
        }
    }

    pub fn read_parallel_option(&mut self) -> Result<ParallelOption> {
        match self.read_u8()? {
            AGGREGATE_PARALLEL_SAFE => Ok(ParallelOption::Safe),
            AGGREGATE_PARALLEL_RESTRICTED => Ok(ParallelOption::Restricted),
            AGGREGATE_PARALLEL_UNSAFE => Ok(ParallelOption::Unsafe),
            other => Err(eyre!("invalid parallel option tag in schema entry: {other}")),
        }
    }

    pub fn finish(&self) -> Result<()> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(eyre!("schema entry had {} trailing bytes", self.remaining()))
        }
    }
}

pub fn entry_payloads(section: &[u8]) -> Result<Vec<&[u8]>> {
    let mut out = Vec::new();
    let mut reader = EntryReader::new(section);
    while !reader.is_empty() {
        if reader.remaining() < 4 {
            if section[reader.pos..].iter().all(|byte| *byte == 0) {
                break;
            }
            bail!("invalid trailing bytes in pgrx schema section");
        }

        let len = reader.read_u32()? as usize;
        if len == 0 {
            if section[reader.pos..].iter().all(|byte| *byte == 0) {
                break;
            }
            bail!("invalid zero-length pgrx schema entry");
        }
        if reader.remaining() < len {
            bail!("invalid pgrx schema section payload length");
        }
        let start = reader.pos;
        reader.pos += len;
        out.push(&section[start..reader.pos]);
    }
    Ok(out)
}

pub fn leak_string(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

pub fn leak_option_string(value: Option<String>) -> Option<&'static str> {
    value.map(leak_string)
}

pub fn decode_entity(payload: &[u8]) -> Result<SqlGraphEntity> {
    let mut reader = EntryReader::new(payload);
    let entity = match reader.read_u8()? {
        ENTITY_SCHEMA => SqlGraphEntity::Schema(SchemaEntity {
            module_path: leak_string(reader.read_string()?),
            name: leak_string(reader.read_string()?),
            file: leak_string(reader.read_string()?),
            line: reader.read_u32()?,
        }),
        ENTITY_CUSTOM_SQL => {
            let sql = leak_string(reader.read_string()?);
            let module_path = leak_string(reader.read_string()?);
            let full_path = leak_string(reader.read_string()?);
            let file = leak_string(reader.read_string()?);
            let line = reader.read_u32()?;
            let name = leak_string(reader.read_string()?);
            let bootstrap = reader.read_bool()?;
            let finalize = reader.read_bool()?;

            let require_count = reader.read_u32()? as usize;
            let mut requires = Vec::with_capacity(require_count);
            for _ in 0..require_count {
                requires.push(reader.read_positioning_ref()?);
            }

            let create_count = reader.read_u32()? as usize;
            let mut creates = Vec::with_capacity(create_count);
            for _ in 0..create_count {
                creates.push(reader.read_sql_declared()?);
            }

            SqlGraphEntity::CustomSql(ExtensionSqlEntity {
                module_path,
                full_path,
                sql,
                file,
                line,
                name,
                bootstrap,
                finalize,
                requires,
                creates,
            })
        }
        ENTITY_FUNCTION => {
            let name = leak_string(reader.read_string()?);
            let unaliased_name = leak_string(reader.read_string()?);
            let module_path = leak_string(reader.read_string()?);
            let full_path = leak_string(reader.read_string()?);

            let arg_count = reader.read_u32()? as usize;
            let mut fn_args = Vec::with_capacity(arg_count);
            for _ in 0..arg_count {
                fn_args.push(reader.read_pg_extern_argument()?);
            }

            let fn_return = reader.read_pg_extern_return()?;
            let schema = leak_option_string(reader.read_option_string()?);
            let file = leak_string(reader.read_string()?);
            let line = reader.read_u32()?;

            let extern_attr_count = reader.read_u32()? as usize;
            let mut extern_attrs = Vec::with_capacity(extern_attr_count);
            for _ in 0..extern_attr_count {
                extern_attrs.push(reader.read_extern_arg()?);
            }

            let search_path = reader.read_search_path()?;
            let operator = if reader.read_bool()? { Some(reader.read_operator()?) } else { None };
            let cast = if reader.read_bool()? { Some(reader.read_cast()?) } else { None };
            let to_sql_config = reader.read_to_sql_config()?;

            SqlGraphEntity::Function(PgExternEntity {
                name,
                unaliased_name,
                module_path,
                full_path,
                fn_args,
                fn_return,
                schema,
                file,
                line,
                extern_attrs,
                search_path,
                operator,
                cast,
                to_sql_config,
            })
        }
        ENTITY_TYPE => {
            let name = leak_string(reader.read_string()?);
            let file = leak_string(reader.read_string()?);
            let line = reader.read_u32()?;
            let module_path = leak_string(reader.read_string()?);
            let full_path = leak_string(reader.read_string()?);
            let schema_key = leak_string(reader.read_string()?);
            let in_fn_path = leak_string(reader.read_string()?);
            let out_fn_path = leak_string(reader.read_string()?);
            let receive_fn_path = leak_option_string(reader.read_option_string()?);
            let send_fn_path = leak_option_string(reader.read_option_string()?);
            let to_sql_config = reader.read_to_sql_config()?;
            let alignment =
                if reader.read_bool()? { Some(reader.read_u32()? as usize) } else { None };

            SqlGraphEntity::Type(PostgresTypeEntity {
                name,
                file,
                line,
                full_path,
                module_path,
                schema_key,
                in_fn_path,
                out_fn_path,
                receive_fn_path,
                send_fn_path,
                to_sql_config,
                alignment,
            })
        }
        ENTITY_ENUM => {
            let name = leak_string(reader.read_string()?);
            let file = leak_string(reader.read_string()?);
            let line = reader.read_u32()?;
            let module_path = leak_string(reader.read_string()?);
            let full_path = leak_string(reader.read_string()?);
            let schema_key = leak_string(reader.read_string()?);

            let variant_count = reader.read_u32()? as usize;
            let mut variants = Vec::with_capacity(variant_count);
            for _ in 0..variant_count {
                variants.push(leak_string(reader.read_string()?));
            }

            let to_sql_config = reader.read_to_sql_config()?;

            SqlGraphEntity::Enum(PostgresEnumEntity {
                name,
                file,
                line,
                full_path,
                module_path,
                schema_key,
                variants,
                to_sql_config,
            })
        }
        ENTITY_ORD => SqlGraphEntity::Ord(PostgresOrdEntity {
            name: leak_string(reader.read_string()?),
            file: leak_string(reader.read_string()?),
            line: reader.read_u32()?,
            full_path: leak_string(reader.read_string()?),
            module_path: leak_string(reader.read_string()?),
            schema_key: leak_string(reader.read_string()?),
            to_sql_config: reader.read_to_sql_config()?,
        }),
        ENTITY_HASH => SqlGraphEntity::Hash(PostgresHashEntity {
            name: leak_string(reader.read_string()?),
            file: leak_string(reader.read_string()?),
            line: reader.read_u32()?,
            full_path: leak_string(reader.read_string()?),
            module_path: leak_string(reader.read_string()?),
            schema_key: leak_string(reader.read_string()?),
            to_sql_config: reader.read_to_sql_config()?,
        }),
        ENTITY_AGGREGATE => {
            let full_path = leak_string(reader.read_string()?);
            let module_path = leak_string(reader.read_string()?);
            let file = leak_string(reader.read_string()?);
            let line = reader.read_u32()?;
            let name = leak_string(reader.read_string()?);
            let ordered_set = reader.read_bool()?;
            let args = reader.read_aggregate_type_list()?;
            let direct_args =
                if reader.read_bool()? { Some(reader.read_aggregate_type_list()?) } else { None };
            let stype = reader.read_aggregate_type()?;
            let sfunc = leak_string(reader.read_string()?);
            let finalfunc = leak_option_string(reader.read_option_string()?);
            let finalfunc_modify =
                if reader.read_bool()? { Some(reader.read_finalize_modify()?) } else { None };
            let combinefunc = leak_option_string(reader.read_option_string()?);
            let serialfunc = leak_option_string(reader.read_option_string()?);
            let deserialfunc = leak_option_string(reader.read_option_string()?);
            let initcond = leak_option_string(reader.read_option_string()?);
            let msfunc = leak_option_string(reader.read_option_string()?);
            let minvfunc = leak_option_string(reader.read_option_string()?);
            let mstype = if reader.read_bool()? { Some(reader.read_used_type()?) } else { None };
            let mfinalfunc = leak_option_string(reader.read_option_string()?);
            let mfinalfunc_modify =
                if reader.read_bool()? { Some(reader.read_finalize_modify()?) } else { None };
            let minitcond = leak_option_string(reader.read_option_string()?);
            let sortop = leak_option_string(reader.read_option_string()?);
            let parallel =
                if reader.read_bool()? { Some(reader.read_parallel_option()?) } else { None };
            let hypothetical = reader.read_bool()?;
            let to_sql_config = reader.read_to_sql_config()?;

            SqlGraphEntity::Aggregate(PgAggregateEntity {
                full_path,
                module_path,
                file,
                line,
                name,
                ordered_set,
                args,
                direct_args,
                stype,
                sfunc,
                finalfunc,
                finalfunc_modify,
                combinefunc,
                serialfunc,
                deserialfunc,
                initcond,
                msfunc,
                minvfunc,
                mstype,
                mfinalfunc,
                mfinalfunc_modify,
                minitcond,
                sortop,
                parallel,
                hypothetical,
                to_sql_config,
            })
        }
        ENTITY_TRIGGER => SqlGraphEntity::Trigger(PgTriggerEntity {
            function_name: leak_string(reader.read_string()?),
            file: leak_string(reader.read_string()?),
            line: reader.read_u32()?,
            module_path: leak_string(reader.read_string()?),
            full_path: leak_string(reader.read_string()?),
            to_sql_config: reader.read_to_sql_config()?,
        }),
        other => return Err(eyre!("invalid entity tag in schema entry: {other}")),
    };

    reader.finish()?;
    Ok(entity)
}

pub fn decode_entities(section: &[u8]) -> Result<Vec<SqlGraphEntity>> {
    entry_payloads(section)?.into_iter().map(decode_entity).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_basic_entry() {
        const PAYLOAD_LEN: usize = u8_len() + str_len("hello") + bool_len() + u32_len();
        const TOTAL_LEN: usize = u32_len() + PAYLOAD_LEN;
        const ENTRY: [u8; TOTAL_LEN] = EntryWriter::<TOTAL_LEN>::new()
            .u32(PAYLOAD_LEN as u32)
            .u8(7)
            .str("hello")
            .bool(true)
            .u32(42)
            .finish();

        let payloads = entry_payloads(&ENTRY).unwrap();
        assert_eq!(payloads.len(), 1);
        let mut reader = EntryReader::new(payloads[0]);
        assert_eq!(reader.read_u8().unwrap(), 7);
        assert_eq!(reader.read_string().unwrap(), "hello");
        assert!(reader.read_bool().unwrap());
        assert_eq!(reader.read_u32().unwrap(), 42);
        assert!(reader.is_empty());
    }

    #[test]
    fn ignores_trailing_zero_padding() {
        const PAYLOAD_LEN: usize = u8_len();
        const TOTAL_LEN: usize = u32_len() + PAYLOAD_LEN;
        const ENTRY: [u8; TOTAL_LEN] =
            EntryWriter::<TOTAL_LEN>::new().u32(PAYLOAD_LEN as u32).u8(ENTITY_SCHEMA).finish();
        let mut section = ENTRY.to_vec();
        section.extend_from_slice(&[0, 0, 0, 0]);

        let payloads = entry_payloads(&section).unwrap();
        assert_eq!(payloads.len(), 1);
        assert_eq!(payloads[0], &[ENTITY_SCHEMA]);
    }

    #[test]
    fn recognizes_macho_qualified_section_name() {
        assert!(is_schema_section_name(MACHO_SECTION_NAME));
        assert!(is_schema_section_name(MACHO_SECTION_PATH));
        assert!(is_schema_section_name(ELF_SECTION_NAME));
        assert!(!is_schema_section_name("__TEXT,__text"));
    }
}
