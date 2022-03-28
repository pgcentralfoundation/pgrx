/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use serde::{Deserialize, Serialize};

/// The output of a [`PgOperator`](crate::sql_entity_graph::PgOperator) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PgOperatorEntity {
    pub opname: Option<&'static str>,
    pub commutator: Option<&'static str>,
    pub negator: Option<&'static str>,
    pub restrict: Option<&'static str>,
    pub join: Option<&'static str>,
    pub hashes: bool,
    pub merges: bool,
}
