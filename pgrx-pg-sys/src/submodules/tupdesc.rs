/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! Provides helper implementations for various `TupleDesc`-related structs

use crate::oids::PgOid;
use crate::utils::name_data_to_str;

/// Helper implementation for `FormData_pg_attribute`
impl crate::FormData_pg_attribute {
    pub fn name(&self) -> &str {
        name_data_to_str(&self.attname)
    }

    pub fn type_oid(&self) -> PgOid {
        PgOid::from(self.atttypid)
    }

    pub fn type_mod(&self) -> i32 {
        self.atttypmod
    }

    pub fn num(&self) -> i16 {
        self.attnum
    }

    pub fn is_dropped(&self) -> bool {
        self.attisdropped
    }

    pub fn rel_id(&self) -> crate::Oid {
        self.attrelid
    }
}
