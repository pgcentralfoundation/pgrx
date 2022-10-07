#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum PgTriggerError {
    #[error("`PgTrigger`s can only be built from `FunctionCallInfo` instances which `pgx::pg_sys::called_as_trigger(fcinfo)` returns `true`")]
    NotTrigger,
    #[error("`PgTrigger`s cannot be built from `NULL` `pgx::pg_sys::FunctionCallInfo`s")]
    NullFunctionCallInfo,
    #[error(
        "`InvalidPgTriggerWhen` cannot be built from `event & TRIGGER_EVENT_TIMINGMASK` of `{0}"
    )]
    InvalidPgTriggerWhen(u32),
    #[error(
        "`InvalidPgTriggerOperation` cannot be built from `event & TRIGGER_EVENT_OPMASK` of `{0}"
    )]
    InvalidPgTriggerOperation(u32),
    #[error("core::str::Utf8Error: {0}")]
    CoreUtf8(#[from] core::str::Utf8Error),
    #[error("TryFromIntError: {0}")]
    TryFromInt(#[from] core::num::TryFromIntError),
    #[error("The `pgx::pg_sys::TriggerData`'s `tg_trigger` field was a NULL pointer")]
    NullTrigger,
    #[error("The `pgx::pg_sys::FunctionCallInfo`'s `context` field was a NULL pointer")]
    NullTriggerData,
    #[error("The `pgx::pg_sys::TriggerData`'s `tg_relation` field was a NULL pointer")]
    NullRelation,
}
