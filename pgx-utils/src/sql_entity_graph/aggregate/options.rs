use crate::sql_entity_graph::{PgxSql, ToSql};

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
