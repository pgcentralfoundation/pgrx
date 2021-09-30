use crate::{sql_entity_graph::{SqlGraphIdentifier, SqlGraphEntity, ToSql}, Internal, PgBox};
use pgx_utils::sql_entity_graph::PgAggregate;
use std::{cmp::Ordering, any::TypeId};

pub trait Aggregate where Self: Sized {
    /// The type of the argument(s).
    ///
    /// For a single argument, provide the type directly.
    ///
    /// For multiple arguments, provide a tuple.
    ///
    /// `pgx` does not support `argname` as it is only used for documentation purposes.
    ///
    /// If the final argument is to be variadic, use `pgx::Variadic`.
    type Args;

    /// The types of the order argument(s).
    ///
    /// For a single argument, provide the type directly.
    ///
    /// For multiple arguments, provide a tuple.
    ///
    /// `pgx` does not support `argname` as it is only used for documentation purposes.
    ///
    /// If the final argument is to be variadic, use `pgx::Variadic`.
    ///
    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    type OrderBy;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    type Finalize;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    type MovingState;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    const PARALLEL: Option<ParallelOption> = None;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    const FINALIZE_MODIFY: Option<FinalizeModify> = None;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    const MOVING_FINALIZE_MODIFY: Option<FinalizeModify> = None;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    const INITIAL_CONDITION: Option<&'static str> = None;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    const SORT_OPERATOR: Option<&'static str> = None;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    const MOVING_INITIAL_CONDITION: Option<&'static str> = None;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    const HYPOTHETICAL: bool = false;

    fn state(&self, v: Self::Args) -> Self;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn finalize(&self) -> Self::Finalize;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn combine(&self, _other: Self) -> Self;
    
    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn serial(&self) -> Vec<u8>;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn deserial(&self, _buf: Vec<u8>, _internal: PgBox<Self>) -> PgBox<Self>;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn moving_state(_mstate: Self::MovingState, _v: Self::Args) -> Self::MovingState;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn moving_state_inverse(_mstate: Self::MovingState, _v: Self::Args) -> Self::MovingState;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn moving_finalize(_mstate: Self::MovingState) -> Self::Finalize;

}

/// Corresponds to the `PARALLEL` and `MFINALFUNC_MODIFY` in [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html).
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParallelOption {
    Safe,
    Restricted,
    Unsafe,
}

/// Corresponds to the `FINALFUNC_MODIFY` and `MFINALFUNC_MODIFY` in [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html).
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FinalizeModify {
    ReadOnly,
    Shareable,
    ReadWrite,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PgAggregateEntity {
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub ty_id: TypeId,

    /// The `arg_data_type` list.
    ///
    /// Corresponds to `Args` in [`Aggregate`].
    pub args: &'static [&'static str],

    /// The `ORDER BY arg_data_type` list.
    ///
    /// Corresponds to `OrderBy` in [`Aggregate`].
    pub order_by: Option<&'static [&'static str]>,

    /// The `STYPE` and `name` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// The implementor of an [`Aggregate`].
    pub stype: &'static str,

    /// The `SFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `state` in [`Aggregate`].
    pub sfunc: &'static str,

    /// The `FINALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `final` in [`Aggregate`].
    pub finalfunc: Option<FinalizeModify>,
    
    /// The `FINALFUNC_MODIFY` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `FINALIZE_MODIFY` in [`Aggregate`].
    pub finalfunc_modify: Option<&'static str>,

    /// The `SERIALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `serial` in [`Aggregate`].
    pub serialfunc: Option<&'static str>,
    
    /// The `DESERIALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `deserial` in [`Aggregate`].
    pub deserialfunc: Option<&'static str>,
    
    /// The `INITCOND` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `INITIAL_CONDITION` in [`Aggregate`].
    pub initcond: Option<&'static str>,
    
    /// The `MSFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `moving_state` in [`Aggregate`].
    pub msfunc: Option<&'static str>,
    
    /// The `MINVFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `moving_state_inverse` in [`Aggregate`].
    pub minvfunc: Option<&'static str>,
    
    /// The `MSTYPE` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `MovingState` in [`Aggregate`].
    pub mstype: Option<&'static str>,
    
    /// The `MSSPACE` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// TODO: Currently unused.
    /// pub msspace: &'static str,

    /// The `MFINALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `moving_state_finalize` in [`Aggregate`].
    pub mfinalfunc: Option<&'static str>,

    /// The `MFINALFUNC_MODIFY` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `moving_state_finalize` in [`Aggregate`].
    pub mfinalfunc_modify: Option<&'static str>,

    /// The `MINITCOND` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `MOVING_INITIAL_CONDITION` in [`Aggregate`].
    pub minitcond: Option<&'static str>,

    /// The `SORTOP` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `SORT_OPERATOR` in [`Aggregate`].
    pub sortop: Option<&'static str>,

    /// The `PARALLEL` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `PARALLEL` in [`Aggregate`].
    pub parallel: Option<ParallelOption>,

    /// The `HYPOTHETICAL` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `hypothetical` in [`Aggregate`].
    pub hypothetical: Option<ParallelOption>,
}

impl Ord for PgAggregateEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file
            .cmp(other.full_path)
            .then_with(|| self.file.cmp(other.full_path))
    }
}

impl PartialOrd for PgAggregateEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<SqlGraphEntity> for PgAggregateEntity {
    fn into(self) -> SqlGraphEntity {
        SqlGraphEntity::Aggregate(self)
    }
}

impl SqlGraphIdentifier for PgAggregateEntity {
    fn dot_identifier(&self) -> String {
        format!("aggregate {}", self.full_path)
    }
    fn rust_identifier(&self) -> String {
        self.full_path.to_string()
    }
    fn file(&self) -> Option<&'static str> {
        Some(self.file)
    }
    fn line(&self) -> Option<u32> {
        Some(self.line)
    }
}

impl ToSql for PgAggregateEntity {
    #[tracing::instrument(level = "debug", err, skip(self, _context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!("\n\
                            -- {file}:{line}\n\
                            -- {full_path}\n\
                            -- AGGREGATE STUFF HERE",
                          full_path = self.full_path,
                          file = self.file,
                          line = self.line,
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}
