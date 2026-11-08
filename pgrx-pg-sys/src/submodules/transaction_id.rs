//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::__pgrx_c_macros::{CUnsignedInt, CValue, expression_result::CExpression};
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, ReturnsError, ReturnsRef, SqlMappingRef, SqlTranslatable, TypeOrigin,
};
use std::fmt::{self, Debug, Display, Formatter};

pub type MultiXactId = TransactionId;

/// An `xid` type from PostgreSQL
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TransactionId(u32);

impl TransactionId {
    /// Invalid transaction identity from the selected PostgreSQL headers.
    pub const INVALID: Self = Self(crate::include::InvalidTransactionId);
    /// Bootstrap transaction identity from the selected PostgreSQL headers.
    pub const BOOTSTRAP: Self = Self(crate::include::BootstrapTransactionId);
    /// Frozen transaction identity from the selected PostgreSQL headers.
    pub const FROZEN: Self = Self(crate::include::FrozenTransactionId);
    /// First normal transaction identity from the selected PostgreSQL headers.
    pub const FIRST_NORMAL: Self = Self(crate::include::FirstNormalTransactionId);
    /// Maximum transaction identity from the selected PostgreSQL headers.
    pub const MAX: Self = Self(crate::include::MaxTransactionId);

    pub const fn from_inner(xid: u32) -> Self {
        Self(xid)
    }

    pub const fn into_inner(self) -> u32 {
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
        Self::from_inner(xid)
    }
}

impl From<TransactionId> for u32 {
    #[inline]
    fn from(xid: TransactionId) -> Self {
        xid.into_inner()
    }
}

impl From<TransactionId> for crate::Datum {
    fn from(xid: TransactionId) -> Self {
        xid.into_inner().into()
    }
}

/// Convert a C macro result of C type `unsigned int`, which is `TransactionId`'s C type, to a
/// `TransactionId`.
///
/// Like `CExpression::get_usize`, this goes by the result's C type, so it accepts any
/// `unsigned int` result, not only one that PostgreSQL declares as `TransactionId`.
impl From<CExpression<CValue<CUnsignedInt>>> for TransactionId {
    /// Keep the result's 32 bits, as a C assignment to a `TransactionId` does.
    fn from(value: CExpression<CValue<CUnsignedInt>>) -> Self {
        TransactionId::from_inner(value.get())
    }
}

unsafe impl SqlTranslatable for TransactionId {
    const TYPE_IDENT: &'static str = pgrx_sql_entity_graph::pgrx_resolved_type!(TransactionId);
    const TYPE_ORIGIN: TypeOrigin = TypeOrigin::External;
    const ARGUMENT_SQL: Result<SqlMappingRef, ArgumentError> = Ok(SqlMappingRef::literal("xid"));
    const RETURN_SQL: Result<ReturnsRef, ReturnsError> =
        Ok(ReturnsRef::One(SqlMappingRef::literal("xid")));
}

impl Debug for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
