/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use crate::sql_entity_graph::{pg_extern::entity::TypeEntity, SqlGraphIdentifier};

/// The output of a [`PgExternArgument`](crate::sql_entity_graph::PgExternArgument) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PgExternArgumentEntity {
    pub pattern: &'static str,
    pub ty: TypeEntity,
    pub is_optional: bool,
    pub is_variadic: bool,
    pub default: Option<&'static str>,
}

impl SqlGraphIdentifier for PgExternArgumentEntity {
    fn dot_identifier(&self) -> String {
        format!("arg {}", self.rust_identifier())
    }
    fn rust_identifier(&self) -> String {
        match self.ty {
            TypeEntity::Type { full_path, .. } => full_path.to_string(),
            TypeEntity::CompositeType { sql } => sql.to_string(),
        }
    }

    fn file(&self) -> Option<&'static str> {
        None
    }

    fn line(&self) -> Option<u32> {
        None
    }
}
