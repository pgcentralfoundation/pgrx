/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::sql_entity_graph::TypeEntity;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PgExternReturnEntity {
    None,
    Type {
        ty: TypeEntity,
    },
    SetOf {
        ty: TypeEntity,
    },
    Iterated(
        Vec<(
            TypeEntity,
            Option<&'static str>, // Name
        )>,
    ),
    Trigger,
}
