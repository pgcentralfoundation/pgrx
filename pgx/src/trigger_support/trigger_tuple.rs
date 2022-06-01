/// Indicates which trigger tuple to convert into a [crate::PgHeapTuple].
#[derive(Debug, Copy, Clone)]
pub enum TriggerTuple {
    /// The row for which the trigger was fired. This is the row being inserted, updated, or deleted.
    /// If this trigger was fired for an INSERT or DELETE then this is what you should return from the
    /// function if you don't want to replace the row with a different one (in the case of INSERT) or
    /// skip the operation.
    Current,

    /// The new version of the row, if the trigger was fired for an UPDATE. This is what you have to
    /// return from the function if the event is an UPDATE and you don't want to replace this row by
    /// a different one or skip the operation.
    New,
}
