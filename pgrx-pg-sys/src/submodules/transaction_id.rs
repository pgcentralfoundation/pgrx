//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use std::fmt::{Debug, Display, Formatter};

pub type MultiXactId = TransactionId;

/// An `xid` type from PostgreSQL
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TransactionId(u32);

impl TransactionId {
    pub const INVALID: Self = Self::from_u32(0);
    pub const BOOTSTRAP: Self = Self::from_u32(1);
    pub const FROZEN: Self = Self::from_u32(2);
    pub const FIRST_NORMAL: Self = Self::from_u32(3);
    pub const MAX: Self = Self::from_u32(u32::MAX);

    #[inline]
    pub const fn from_u32(xid: u32) -> Self {
        Self(xid)
    }

    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0
    }
}

impl Default for TransactionId {
    fn default() -> Self {
        Self::INVALID
    }
}

impl From<u32> for TransactionId {
    #[inline]
    fn from(xid: u32) -> Self {
        Self::from_u32(xid)
    }
}

impl From<TransactionId> for u32 {
    #[inline]
    fn from(xid: TransactionId) -> Self {
        xid.to_u32()
    }
}

impl From<TransactionId> for crate::Datum {
    fn from(xid: TransactionId) -> Self {
        xid.to_u32().into()
    }
}

unsafe impl SqlTranslatable for TransactionId {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("xid"))
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("xid")))
    }
}

impl Debug for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}
