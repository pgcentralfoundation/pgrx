/*!

[Aggregate](https://www.postgresql.org/docs/current/xaggr.html) support.

Aggregates are created by implementing [`Aggregate`] for a type and decorating the implementation with
[`#[pg_aggregate]`](pgx_macros::pg_aggregate).

Definition of the aggregate is done via settings in the type's [`Aggregate`] implementation. While
the trait itself several items, only a few are required, the macro will fill in the others with unused stubs.

Here's a fairly minimal aggregate:

```rust
use pgx::*;
use serde::{Serialize, Deserialize};

// pg_module_magic!(); // Uncomment this outside of docs!

#[derive(Copy, Clone, Default, PostgresType, Serialize, Deserialize)]
pub struct DemoSum {
    count: i32,
}

#[pg_aggregate]
impl Aggregate for DemoSum {
    const INITIAL_CONDITION: Option<&'static str> = Some(r#"{ "count": 0 }"#);
    type Args = i32;
    fn state(mut current: Self::State, arg: Self::Args) -> Self::State {
        current.count += arg;
        current
    }
}
```

This creates SQL like so:

```sql
-- src/lib.rs:10
-- aggregate::DemoSum
CREATE AGGREGATE DemoSum (
	integer /* i32 */
)
(
	SFUNC = "demo_sum_state",
	STYPE = DemoSum, /* aggregate::DemoSum */

	INITCOND = '{ "count": 0 }'
);
```

Example of usage:

```sql
aggregate=# CREATE TABLE demo_table (value INTEGER);
CREATE TABLE
aggregate=# INSERT INTO demo_table (value) VALUES (1), (2), (3);
INSERT 0 3
aggregate=# SELECT DemoSum(value) FROM demo_table;
    demosum    
-------------
 {"count":6}
(1 row)
```
*/

use crate::{
    datum::sql_entity_graph::{PgxSql, ToSql},
    pgbox::PgBox,
};

/// Aggregate implementation trait.
/// 
/// When decorated with [`#[pgx_macros::pg_aggregate]`](pgx_macros::pg_aggregate), enables the
/// generation of [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
/// SQL.
/// 
/// The [`#[pgx_macros::pg_aggregate]`](pgx_macros::pg_aggregate) will automatically fill fields
/// marked optional with stubs.
pub trait Aggregate
where
    Self: Sized,
{
    /// The type of the return value on `state` and `combine` functions.
    ///
    /// For an aggregate type which does not have a `PgVarlenaInOutFuncs` implementation,
    /// this can be left out, or set to it's default, `Self`.
    ///
    /// For an aggregate type which **does** have a `PgVarlenaInOutFuncs` implementation,
    /// this should be set to `PgVarlena<Self>`.
    ///
    /// Other types are supported as well, this can be useful if multiple aggregates share a state.
    type State;

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
    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    type OrderBy;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    type Finalize;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    type MovingState;

    /// The name of the aggregate. (eg. What you'd pass to `SELECT agg(col) FROM tab`.)
    const NAME: &'static str;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    const PARALLEL: Option<ParallelOption> = None;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    const FINALIZE_MODIFY: Option<FinalizeModify> = None;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    const MOVING_FINALIZE_MODIFY: Option<FinalizeModify> = None;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    const INITIAL_CONDITION: Option<&'static str> = None;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    const SORT_OPERATOR: Option<&'static str> = None;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    const MOVING_INITIAL_CONDITION: Option<&'static str> = None;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    const HYPOTHETICAL: bool = false;

    fn state(current: Self::State, v: Self::Args) -> Self::State;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    fn finalize(current: Self::State) -> Self::Finalize;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    fn combine(current: Self::State, _other: Self::State) -> Self::State;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    fn serial(current: Self::State) -> Vec<u8>;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    fn deserial(current: Self::State, _buf: Vec<u8>, _internal: PgBox<Self::State>) -> PgBox<Self::State>;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    fn moving_state(_mstate: Self::MovingState, _v: Self::Args) -> Self::MovingState;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
    fn moving_state_inverse(_mstate: Self::MovingState, _v: Self::Args) -> Self::MovingState;

    /// **Optional:** This function can be skipped, `#[pg_aggregate]` will create a stub.
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
