use pgx::*;
use serde::{Deserialize, Serialize};

#[derive(
    Eq,
    PartialEq,
    Ord,
    Hash,
    PartialOrd,
    Serialize,
    Deserialize,
    PostgresType,
    PostgresEq,
    PostgresOrd,
    PostgresHash,
)]
pub struct Thing(String);
