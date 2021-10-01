use crate::{sql_entity_graph::{SqlGraphIdentifier, SqlGraphEntity, ToSql, PgxSql}, Internal, PgBox};
use pgx_utils::sql_entity_graph::PgAggregate;
use std::{cmp::Ordering, any::TypeId};
use quote::{quote, ToTokens};

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

    /// The name of the aggregate. (eg. What you'd pass to `SELECT agg(col) FROM tab`.)
    const NAME: &'static str;

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
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParallelOption {
    Safe,
    Restricted,
    Unsafe,
}

impl ToSql for ParallelOption {
    fn to_sql(&self, _context: &PgxSql) -> eyre::Result<String> {
        let value = match self {
            ParallelOption::Safe => String::from("SAFE"),
            ParallelOption::Restricted => String::from("RESTRICTED"),
            ParallelOption::Unsafe => String::from("UNSAFE"),
        };
        Ok(value)
    }
}

/// Corresponds to the `FINALFUNC_MODIFY` and `MFINALFUNC_MODIFY` in [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html).
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum FinalizeModify {
    ReadOnly,
    Shareable,
    ReadWrite,
}

impl ToSql for FinalizeModify {
    fn to_sql(&self, _context: &PgxSql) -> eyre::Result<String> {
        let value = match self {
            FinalizeModify::ReadOnly => String::from("READ_ONLY"),
            FinalizeModify::Shareable => String::from("SHAREABLE"),
            FinalizeModify::ReadWrite => String::from("READ_WRITE"),
        };
        Ok(value)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PgAggregateEntity {
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub ty_id: TypeId,

    pub name: &'static str,

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
    pub finalfunc: Option<&'static str>,
    
    /// The `FINALFUNC_MODIFY` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `FINALIZE_MODIFY` in [`Aggregate`].
    pub finalfunc_modify: Option<FinalizeModify>,

    /// The `COMBINEFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `combine` in [`Aggregate`].
    pub combinefunc: Option<&'static str>,

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
    
    // The `MSSPACE` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    //
    // TODO: Currently unused.
    // pub msspace: &'static str,

    /// The `MFINALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `moving_state_finalize` in [`Aggregate`].
    pub mfinalfunc: Option<&'static str>,

    /// The `MFINALFUNC_MODIFY` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `MOVING_FINALIZE_MODIFY` in [`Aggregate`].
    pub mfinalfunc_modify: Option<FinalizeModify>,

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
    pub hypothetical: bool,
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
    #[tracing::instrument(level = "debug", err, skip(self, context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, context: &super::PgxSql) -> eyre::Result<String> {
        let mut optional_attributes = Vec::new();

        if let Some(value) = self.order_by {
            optional_attributes.push(format!("ORDER_BY = {}", ""));
        }
        if let Some(value) = self.finalfunc {
            optional_attributes.push(format!("FINALFUNC = {}", value));
        }
        if let Some(value) = self.finalfunc_modify {
            optional_attributes.push(format!("FINALFUNC_MODIFY = {}", value.to_sql(context)?));
        }
        if let Some(value) = self.combinefunc {
            optional_attributes.push(format!("COMBINEFUNC = {}", value));
        }
        if let Some(value) = self.serialfunc {
            optional_attributes.push(format!("SERIALFUNC = {}", value));
        }
        if let Some(value) = self.deserialfunc {
            optional_attributes.push(format!("DESERIALFUNC = {}", value));
        }
        if let Some(value) = self.initcond {
            optional_attributes.push(format!("INITCOND = {}", value));
        }
        if let Some(value) = self.msfunc {
            optional_attributes.push(format!("MSFUNC = {}", value));
        }
        if let Some(value) = self.minvfunc {
            optional_attributes.push(format!("MINVFUNC = {}", value));
        }
        if let Some(value) = self.mstype {
            optional_attributes.push(format!("MSTYPE = {}", value));
        }
        if let Some(value) = self.mfinalfunc {
            optional_attributes.push(format!("MFINALFUNC = {}", value));
        }
        if let Some(value) = self.mfinalfunc_modify {
            optional_attributes.push(format!("MFINALFUNC_MODIFY = {}", value.to_sql(context)?));
        }
        if let Some(value) = self.minitcond {
            optional_attributes.push(format!("MINITCOND = {}", value));
        }
        if let Some(value) = self.sortop {
            optional_attributes.push(format!("SORTOP = {}", value));
        }
        if let Some(value) = self.parallel {
            optional_attributes.push(format!("PARALLEL = {}", value.to_sql(context)?));
        }
        if self.hypothetical {
            optional_attributes.push(String::from("HYPOTHETICAL"))
        }

        let sql = format!("\n\
                -- {file}:{line}\n\
                -- {full_path}\n\
                CREATE AGGREGATE {name} ({args})\n\
                {maybe_order_by}\
                (\n\
                    \tsfunc = {sfunc},\n\
                    \tstype = {stype},\n\
                    {optional_attributes}\
                )\n\
            ",
            name = self.name,
            full_path = self.full_path,
            file = self.file,
            line = self.line,
            sfunc = self.sfunc,
            stype = self.stype,
            args = "$ARGS",
            maybe_order_by = "\t$ORDER_BY\n",
            optional_attributes = String::from("\t") + &optional_attributes.join(",\n\t") + "\n",
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}
