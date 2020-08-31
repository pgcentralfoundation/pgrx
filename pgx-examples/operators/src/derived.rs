use pgx::*;
use serde::{Deserialize, Serialize};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, PostgresType, PostgresEq, PostgresOrd,
)]
pub struct Thing(String);
