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

Function and type level metadata entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate] APIs, this is considered **internal**
> to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.


*/
use super::{ArgumentError, Returns, ReturnsError, SqlMapping};

/// Describes whether a SQL type reference should resolve to schema emitted by this
/// extension or be treated as an external SQL type.
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum TypeOrigin {
    /// The extension being built is responsible for emitting this type into the
    /// schema graph.
    ThisExtension,
    /// The type already exists outside this extension's schema graph.
    External,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionMetadataEntity {
    pub arguments: Vec<FunctionMetadataTypeEntity>,
    pub retval: FunctionMetadataTypeEntity,
    pub path: &'static str,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionMetadataTypeEntity {
    pub schema_key: &'static str,
    pub type_origin: TypeOrigin,
    pub argument_sql: Result<SqlMapping, ArgumentError>,
    pub return_sql: Result<Returns, ReturnsError>,
}
