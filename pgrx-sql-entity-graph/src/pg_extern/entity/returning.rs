/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[pg_extern]` related return value entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgrx_sql_entity_graph] APIs, this is considered **internal**
to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::UsedTypeEntity;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PgExternReturnEntity {
    None,
    Type {
        ty: UsedTypeEntity,
    },
    SetOf {
        ty: UsedTypeEntity,
        optional: bool, /* Eg `Option<SetOfIterator<T>>` */
        result: bool,   /* Eg `Result<SetOfIterator<T>, E>` */
    },
    Iterated {
        tys: Vec<PgExternReturnEntityIteratedItem>,
        optional: bool, /* Eg `Option<TableIterator<T>>` */
        result: bool,   /* Eg `Result<TableIterator<T>, E>` */
    },
    Trigger,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PgExternReturnEntityIteratedItem {
    pub ty: UsedTypeEntity,
    pub name: Option<&'static str>,
}
