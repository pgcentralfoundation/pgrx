/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

#![allow(non_camel_case_types)]
use crate as pg_sys;
use crate::BuiltinOid;
use crate::Datum;
use core::fmt;
use pgrx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

/// An [object identifier][pg_docs_oid] in Postgres.
///
/// This is meant to be understood purely by equality. There is no sensible "order" for Oids.
///
/// # Notes
/// `Default` shall return a sensical Oid, not necessarily a useful one.
/// Currently, this means that it returns the invalid Oid.
///
/// [pg_docs_oid]: https://www.postgresql.org/docs/current/datatype-oid.html
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Oid(pub(crate) u32);

impl Oid {
    pub const INVALID: Oid = Oid(0);

    /// Generate an Oid from an arbitrary u32.
    /// # Safety
    /// This allows you to create an Oid that Postgres won't recognize and throw it into Postgres.
    /// Don't.
    ///
    /// You should know what kind of object the identifier would imply before creating it.
    /// Postgres may sometimes call very different internal functions based on an Oid input.
    /// The extension programming interface of Postgres can reach deep, calling functions
    /// that assume the caller should be trusted like Postgres would trust itself. Because it is.
    /// Postgres tables can also change at runtime, so if an Oid is not a [BuiltinOid],
    /// what Postgres does based on an Oid can change dynamically.
    ///
    /// The existence of this `unsafe` requirement to create *arbitrary* Oids does not, itself,
    /// constitute a promise any Oid from Postgres or PGRX is guaranteed to be valid or sensical.
    /// There are many existing problems in the way of this, for example:
    /// - `Oid` includes the guaranteed-wrong values [Oid::INVALID]
    /// - Postgres may return arbitrary-seeming Oids, like [BuiltinOid::UNKNOWNOID]
    /// - an Oid can arrive in Rust from a table a non-superuser can write
    /// - PGRX mostly relies on Rust's type system instead of the dynamic typing of Postgres,
    ///   thus often deliberately does not bother to remember what OID something had.
    ///
    /// So this function is merely a reminder. Even for extensions that work with many Oids,
    /// it is not typical to need to create one from an arbitrary `u32`. Prefer to use a constant,
    /// or a [BuiltinOid], or to obtain one from querying Postgres, or simply use [Oid::INVALID].
    /// Marking it as an `unsafe fn` is an invitation to get an Oid from more trustworthy sources.
    /// This includes [Oid::INVALID], or [BuiltinOid], or by directly calling into Postgres.
    /// An `unsafe fn` is not an officer of the law empowered to indict C programs for felonies,
    /// nor cite SQL statements for misdemeanors, nor even truly stop you from foolishness.
    /// Even "trustworthy" is meant here in a similar sense to how raw pointers can be "trustworthy".
    /// Often, you should still check if it's null.
    pub const unsafe fn from_u32_unchecked(id: u32) -> Oid {
        Oid(id)
    }

    /// Gets an Oid from a u32 if it is a valid builtin declared by Postgres
    pub const fn from_builtin(id: u32) -> Result<Oid, NotBuiltinOid> {
        match BuiltinOid::from_u32(id) {
            Ok(oid) => Ok(oid.value()),
            Err(e) => Err(e),
        }
    }

    pub const fn as_u32(self) -> u32 {
        self.0
    }
}

impl Default for Oid {
    fn default() -> Oid {
        Oid::INVALID
    }
}

impl fmt::Display for Oid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match PgOid::from(*self) {
            PgOid::Invalid => write!(f, "oid={{#0, Invalid OID}}"),
            // if we think we know the name, include it
            PgOid::BuiltIn(builtin) => write!(f, "oid={{#{}, builtin: {:?}}}", self.0, builtin),
            // no idea? print it anyways!
            PgOid::Custom(oid) => write!(f, "oid=#{}", oid.0),
        }
    }
}

impl From<Oid> for u32 {
    fn from(oid: Oid) -> u32 {
        oid.0
    }
}

impl From<Oid> for crate::Datum {
    fn from(oid: Oid) -> Self {
        Datum::from(oid.0)
    }
}

impl From<BuiltinOid> for Oid {
    fn from(builtin: BuiltinOid) -> Oid {
        builtin.value()
    }
}

unsafe impl SqlTranslatable for Oid {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("oid"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("oid")))
    }
}

// Actually implemented inside pgXX_oids.rs
pub type PgBuiltInOids = BuiltinOid;

pub enum NotBuiltinOid {
    /// the invalid OID
    Invalid,
    /// not a known, builtin OID
    Ambiguous,
    /// value too large to be a valid OID in this Postgres version
    TooBig,
}

impl TryFrom<u32> for BuiltinOid {
    type Error = NotBuiltinOid;
    fn try_from(uint: u32) -> Result<BuiltinOid, NotBuiltinOid> {
        BuiltinOid::from_u32(uint)
    }
}

impl TryFrom<Oid> for BuiltinOid {
    type Error = NotBuiltinOid;
    fn try_from(oid: Oid) -> Result<BuiltinOid, NotBuiltinOid> {
        BuiltinOid::from_u32(oid.0)
    }
}

impl TryFrom<crate::Datum> for BuiltinOid {
    type Error = NotBuiltinOid;
    fn try_from(datum: crate::Datum) -> Result<BuiltinOid, NotBuiltinOid> {
        let uint = u32::try_from(datum.value()).map_err(|_| NotBuiltinOid::TooBig)?;
        BuiltinOid::from_u32(uint)
    }
}

impl BuiltinOid {
    pub const fn value(self) -> pg_sys::Oid {
        Oid(self as u32)
    }

    pub fn oid(self) -> PgOid {
        PgOid::from(self)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PgOid {
    Invalid,
    Custom(Oid),
    BuiltIn(BuiltinOid),
}

impl PgOid {
    pub const fn from_untagged(oid: Oid) -> PgOid {
        match BuiltinOid::from_u32(oid.0) {
            Ok(builtin) => PgOid::BuiltIn(builtin),
            Err(NotBuiltinOid::Invalid) => PgOid::Invalid,
            Err(NotBuiltinOid::Ambiguous) => PgOid::Custom(oid),
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

impl From<BuiltinOid> for PgOid {
    fn from(builtin: BuiltinOid) -> PgOid {
        PgOid::BuiltIn(builtin)
    }
}

impl From<Oid> for PgOid {
    fn from(oid: Oid) -> PgOid {
        PgOid::from_untagged(oid)
    }
}

impl PgOid {
    #[inline]
    pub const fn value(self) -> pg_sys::Oid {
        match self {
            PgOid::Invalid => pg_sys::InvalidOid,
            PgOid::Custom(custom) => custom,
            PgOid::BuiltIn(builtin) => builtin.value(),
        }
    }
}
