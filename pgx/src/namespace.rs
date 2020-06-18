// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use crate::list::PgList;
use crate::pg_sys;

pub struct PgQualifiedNameBuilder {
    list: PgList<pg_sys::Value>,
}

impl Default for PgQualifiedNameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PgQualifiedNameBuilder {
    pub fn new() -> PgQualifiedNameBuilder {
        PgQualifiedNameBuilder {
            list: PgList::<pg_sys::Value>::new(),
        }
    }

    pub fn push(mut self, value: &str) -> PgQualifiedNameBuilder {
        self.list
            .push(unsafe { pg_sys::makeString(std::ffi::CString::new(value).unwrap().into_raw()) });
        self
    }

    pub fn get_operator_oid(self, lhs_type: pg_sys::Oid, rhs_type: pg_sys::Oid) -> pg_sys::Oid {
        unsafe { pg_sys::OpernameGetOprid(self.list.into_pg(), lhs_type, rhs_type) }
    }
}
