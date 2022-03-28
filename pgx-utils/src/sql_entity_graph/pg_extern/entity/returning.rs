/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use core::any::TypeId;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PgExternReturnEntity {
    None,
    Type {
        id: TypeId,
        source: &'static str,
        full_path: &'static str,
        module_path: String,
    },
    SetOf {
        id: TypeId,
        source: &'static str,
        full_path: &'static str,
        module_path: String,
    },
    Iterated(
        Vec<(
            TypeId,
            &'static str,         // Source
            &'static str,         // Full path
            String,               // Module path
            Option<&'static str>, // Name
        )>,
    ),
    Trigger,
}
