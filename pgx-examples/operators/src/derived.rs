use pgx::*;
use serde::{Deserialize, Serialize};

/// standard Rust equality/comparison derives
#[derive(Eq, PartialEq, Ord, Hash, PartialOrd)]

/// Support using this struct as a Postgres type, which the easy way requires Serde
#[derive(PostgresType, Serialize, Deserialize)]

/// automatically generate =, <> SQL operator functions
#[derive(PostgresEq)]

/// automatically generate <, >, <=, >=, and a "_cmp" SQL functions
/// When "PostgresEq" is also derived, pgx also creates an "opclass" (and family)
/// so that the type can be used in indexes `USING btree`
#[derive(PostgresOrd)]

/// automatically generate a "_hash" function, and the necessary "opclass" (and family)
/// so the type can also be used in indexes `USING hash`
#[derive(PostgresHash)]
pub struct Thing(String);

// and there's no code to write!
