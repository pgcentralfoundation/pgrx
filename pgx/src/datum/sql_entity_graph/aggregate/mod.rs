mod entity;
mod finalize_modify;
mod parallel_option;

pub use entity::PgAggregateEntity;
pub use finalize_modify::FinalizeModify;
pub use parallel_option::ParallelOption;

use crate::{sql_entity_graph::PgxSql, PgBox};
use std::any::TypeId;

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
    /// Other types are not supported.
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

    fn state(current: Self::State, v: Self::Args) -> Self::State;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn finalize(current: Self::State) -> Self::Finalize;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn combine(current: Self::State, _other: Self::State) -> Self::State;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn serial(current: Self::State) -> Vec<u8>;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn deserial(current: Self::State, _buf: Vec<u8>, _internal: PgBox<Self>) -> PgBox<Self>;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn moving_state(_mstate: Self::MovingState, _v: Self::Args) -> Self::MovingState;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn moving_state_inverse(_mstate: Self::MovingState, _v: Self::Args) -> Self::MovingState;

    /// **Optional:** The `#[pg_aggregate]` macro will populate these if not provided.
    fn moving_finalize(_mstate: Self::MovingState) -> Self::Finalize;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AggregateType {
    pub ty_source: &'static str,
    pub ty_id: TypeId,
    pub full_path: &'static str,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MaybeVariadicAggregateType {
    pub agg_ty: AggregateType,
    pub variadic: bool,
}
