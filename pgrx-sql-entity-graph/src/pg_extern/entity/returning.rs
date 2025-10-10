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

`#[pg_extern]` related return value entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate] APIs, this is considered **internal**
> to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::UsedTypeEntity;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub enum PgExternReturnEntity {
    None,
    Type { ty: UsedTypeEntity },
    SetOf { ty: UsedTypeEntity },
    Iterated { tys: Vec<PgExternReturnEntityIteratedItem> },
    Trigger,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(bound(deserialize = ""))]
pub struct PgExternReturnEntityIteratedItem {
    pub ty: UsedTypeEntity,
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_option_static_str")]
    pub name: Option<&'static str>,
}
