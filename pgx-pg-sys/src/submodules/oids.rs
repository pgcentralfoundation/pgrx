/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

#![allow(non_camel_case_types)]
use crate as pg_sys;
use crate::PgBuiltInOids;

impl PgBuiltInOids {
    pub fn value(self) -> pg_sys::Oid {
        self as isize as pg_sys::Oid
    }

    pub fn oid(self) -> PgOid {
        PgOid::from(self.value())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub enum PgOid {
    InvalidOid,
    Custom(pg_sys::Oid),
    BuiltIn(PgBuiltInOids),
}

impl PgOid {
    #[inline]
    pub fn from(oid: pg_sys::Oid) -> PgOid {
        match oid {
            pg_sys::InvalidOid => PgOid::InvalidOid,
            custom_oid => match PgBuiltInOids::from(oid) {
                Some(builtin) => PgOid::BuiltIn(builtin),
                None => PgOid::Custom(custom_oid),
            },
        }
    }

    #[inline]
    pub fn value(self) -> pg_sys::Oid {
        match self {
            PgOid::InvalidOid => pg_sys::InvalidOid,
            PgOid::Custom(custom) => custom,
            PgOid::BuiltIn(builtin) => builtin.value(),
        }
    }
}
