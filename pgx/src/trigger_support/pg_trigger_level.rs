use crate::pg_sys;
use crate::trigger_support::TriggerEvent;

/// The level of a trigger
///
/// Maps from a `TEXT` of `ROW` or `STATEMENT`.
///
/// Can be calculated from a `pgx_pg_sys::TriggerEvent`.
// Postgres constants: https://cs.github.com/postgres/postgres/blob/36d4efe779bfc7190ea1c1cf8deb0d945b726663/src/include/commands/trigger.h?q=TRIGGER_FIRED_BEFORE#L98
// Postgres defines: https://cs.github.com/postgres/postgres/blob/36d4efe779bfc7190ea1c1cf8deb0d945b726663/src/include/commands/trigger.h?q=TRIGGER_FIRED_BEFORE#L122-L126
pub enum PgTriggerLevel {
    /// `ROW`
    Row,
    /// `STATEMENT`
    Statement,
}

impl From<TriggerEvent> for PgTriggerLevel {
    fn from(event: TriggerEvent) -> Self {
        match event.0 & pg_sys::TRIGGER_EVENT_ROW {
            0 => PgTriggerLevel::Statement,
            _ => PgTriggerLevel::Row,
        }
    }
}

impl ToString for PgTriggerLevel {
    fn to_string(&self) -> String {
        match self {
            PgTriggerLevel::Statement => "STATEMENT",
            PgTriggerLevel::Row => "ROW",
        }
        .to_string()
    }
}
