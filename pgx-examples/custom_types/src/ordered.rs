use pgx::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

// Demonstrates that Postgres-defined ordering works.
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Eq,
    PartialEq,
    PartialOrd,
    PostgresType,
    PostgresEq,
    PostgresOrd
)]
pub struct OrderedThing {
    item: String,
}

// A silly yet consistent ordering. Strings which start with lowercase letters are sorted normally,
// but strings which start with non-lowercase letters are sorted backwards.
impl Ord for OrderedThing {
    fn cmp(&self, other: &Self) -> Ordering {
        fn starts_lower(thing: &OrderedThing) -> bool {
            thing.item.chars().next().map_or(false, |c| c.is_lowercase())
        }
        if starts_lower(self) {
            if starts_lower(other) {
                self.item.cmp(&other.item)
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            if starts_lower(other) {
                std::cmp::Ordering::Less
            } else {
                self.item.cmp(&other.item).reverse()
            }
        }
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::ordered::OrderedThing;
    use pgx::prelude::*;

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn test_ordering_via_spi() {
        let items = Spi::get_one::<Vec<OrderedThing>>(
            "SELECT array_agg(i ORDER BY i) FROM (VALUES \
                ('{\"item\":\"foo\"}'::OrderedThing), \
                ('{\"item\":\"bar\"}'::OrderedThing), \
                ('{\"item\":\"Foo\"}'::OrderedThing), \
                ('{\"item\":\"Bar\"}'::OrderedThing))\
                items(i);",
        )
        .unwrap();

        assert_eq!(
            items,
            vec![
                OrderedThing { item: "Foo".to_string() },
                OrderedThing { item: "Bar".to_string() },
                OrderedThing { item: "bar".to_string() },
                OrderedThing { item: "foo".to_string() },
            ]
        )
    }
}
