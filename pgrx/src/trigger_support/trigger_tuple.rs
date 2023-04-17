/// Indicates which trigger tuple to convert into a [crate::PgHeapTuple].
#[derive(Debug, Copy, Clone)]
pub enum TriggerTuple {
    /// Represents the new database row for INSERT/UPDATE operations in row-level triggers.
    New,

    /// Represents the old database row for UPDATE/DELETE operations in row-level triggers.
    Old,
}
