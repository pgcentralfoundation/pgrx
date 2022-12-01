/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

/** A guided example of composite types in `pgx`

Composite types in `pgx` are essentially macros that alias to a [`PgHeapTuple`][pgx::PgHeapTuple]
with one important difference:

During SQL generation, the composite type name provided to the [`pgx::composite_type`][pgx::composite_type]
macro is used as the type name, whereas [`PgHeapTuple`][pgx::PgHeapTuple] does not have a valid
Rust to SQL mapping.

A number of considerations are detailed in the [`pgx::composite_type`][pgx::composite_type] and
[`PgHeapTuple`][pgx::PgHeapTuple] documentation which may assist in productionalizing extensions
using composite types.
*/
use pgx::{opname, pg_operator, prelude::*, Aggregate};

// All `pgx` extensions will do this:
pgx::pg_module_magic!();

/* Composite types must be defined either before they are used.

This means composite types must:
* Be defined prior to loading the extension
* Be defined by an `extension_sql!()` block which the function has been "ordered before" in
    the generated SQL

If your extension depends on already existing composite types, then you don't need to do
anything to define them. Just use them as if they exist.

If your extension defines the composite types itself, it's recommended to do that in an
`extension_sql!()` which is set to be a `bootstrap`, and is ordered first in the generated SQL:
*/
extension_sql!(
    "\
CREATE TYPE Dog AS (
    name TEXT,
    scritches INT
);

CREATE TYPE Cat AS (
    name TEXT,
    boops INT
);",
    name = "create_composites",
    bootstrap
);

/*
If it's required for exotic reasons that the composite type not be part of the bootstrap code,
it can also be defined in a non-bootstrap SQL block (**Note:** This could have just been
included in the above bootstrap code):
*/
extension_sql!(
    "\
CREATE TYPE CatAndDogFriendship AS (
    cat Cat,
    dog Dog
);",
    name = "create_cat_and_dog_friendship",
);

// To assist with code reuse, consider setting your composite type names in constants:
const DOG_COMPOSITE_TYPE: &str = "Dog";
const CAT_COMPOSITE_TYPE: &str = "Cat";
const CAT_AND_DOG_FRIENDSHIP_COMPOSITE_TYPE: &str = "CatAndDogFriendship";

/*
While it may be tempting to do something like:

```rust
const CAT_COMPOSITE_TYPE: PgHeapTuple<'static, AllocatedByRust> = pgx::composite_type!("Cat");
```
This is not correct! The [`#[pgx::pg_extern]`][pgx::pg_extern] macro would not be able to pick
up the required metadata for SQL generation. Instead, set the constant as a `&'static str` which
the [`pgx::composite_type!()`][pgx::composite_type] macro can consume.
*/

/*
Create a dog inside Rust then return it, roughly the equivalent of this SQL:

```sql
CREATE FUNCTION create_dog(name TEXT, scritches INT) RETURNS Dog
    LANGUAGE SQL
    STRICT RETURN ROW(name, scritches)::Dog;
```
*/
#[pg_extern]
fn create_dog(name: String, scritches: i32) -> pgx::composite_type!(DOG_COMPOSITE_TYPE) {
    let mut dog = PgHeapTuple::new_composite_type(DOG_COMPOSITE_TYPE).unwrap();
    dog.set_by_name("name", name).unwrap();
    dog.set_by_name("scritches", scritches).unwrap();
    dog
}

/*
Scritch the passed dog, then return it, roughly the equivalent of this SQL:

```sql
CREATE FUNCTION scritch_dog(dog Dog) RETURNS Dog
    LANGUAGE SQL
    STRICT RETURN ROW(dog.name, dog.scritches + 1)::Dog;
```
*/
#[pg_extern]
fn scritch_dog(
    mut dog: pgx::composite_type!(DOG_COMPOSITE_TYPE),
) -> pgx::composite_type!(DOG_COMPOSITE_TYPE) {
    if let Ok(scritches) = dog.get_by_name::<i32>("scritches") {
        dog.set_by_name("scritches", scritches).unwrap();
    }
    dog
}

/*
Make a cat and a dog friends, returning that relationship, roughly the same as:

```sql
CREATE FUNCTION make_friendship(dog Dog, cat Cat)
    RETURNS CatAndDogFriendship
    LANGUAGE SQL
    STRICT
    RETURN ROW(cat, dog)::CatAndDogFriendship;
```
This function primarily exists to demonstrate how to make some `extension_sql!()` "appear before"
the function.
*/
#[pg_extern(requires = ["create_cat_and_dog_friendship"])]
fn make_friendship(
    dog: pgx::composite_type!(DOG_COMPOSITE_TYPE),
    cat: pgx::composite_type!(CAT_COMPOSITE_TYPE),
) -> pgx::composite_type!(CAT_AND_DOG_FRIENDSHIP_COMPOSITE_TYPE) {
    let mut friendship =
        PgHeapTuple::new_composite_type(CAT_AND_DOG_FRIENDSHIP_COMPOSITE_TYPE).unwrap();
    friendship.set_by_name("dog", dog).unwrap();
    friendship.set_by_name("cat", cat).unwrap();
    friendship
}

/*
Create sum the scritches received by dogs, roughly the equivalent of:

```sql
CREATE FUNCTION sum_scritches_state(state integer, new Dog)
    RETURNS integer
    LANGUAGE SQL
    STRICT
    RETURN state + new.scritches;

CREATE AGGREGATE sum_scritches ("value" Dog) (
    SFUNC = "sum_scritches_state",
    STYPE = integer,
    INITCOND = '0'
)
```
*/
struct SumScritches;

#[pg_aggregate]
impl Aggregate for SumScritches {
    type State = i32;
    const INITIAL_CONDITION: Option<&'static str> = Some("0");
    type Args = pgx::name!(value, pgx::composite_type!("Dog"));

    fn state(
        current: Self::State,
        arg: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        let arg_scritches: i32 = arg
            .get_by_name("scritches")
            .unwrap() // Unwrap the result of the conversion
            .unwrap_or_default(); // The number of scritches, or 0 if there was none set
        current + arg_scritches
    }
}

/*
Create sum the scritches received by dogs, roughly the equivalent of:

```sql
CREATE FUNCTION scritch_collector_state(state Dog, new integer)
    RETURNS Dog
    LANGUAGE SQL
    STRICT
    RETURN ROW(state.name, state.scritches + new)::Dog;

CREATE AGGREGATE scritch_collector ("value" integer) (
    SFUNC = "sum_scritches_state",
    STYPE = Dog,
)
```
*/
struct ScritchCollector;

#[pg_aggregate]
impl Aggregate for ScritchCollector {
    type State = Option<pgx::composite_type!("Dog")>;
    type Args = i32;

    fn state(
        current: Self::State,
        arg: Self::Args,
        _fcinfo: pg_sys::FunctionCallInfo,
    ) -> Self::State {
        let mut current = match current {
            Some(v) => v,
            None => PgHeapTuple::new_composite_type(DOG_COMPOSITE_TYPE).unwrap(),
        };
        let current_scritches: i32 = current.get_by_name("scritches").unwrap().unwrap_or_default();
        current.set_by_name("scritches", current_scritches + arg).unwrap();
        Some(current)
    }
}

/*
Create an operator allowing dogs to accept scritches directly.

```sql
CREATE FUNCTION add_scritches_to_dog(
    dog Dog,
    number integer
) RETURNS Dog
LANGUAGE SQL
STRICT
RETURN ROW(dog.name, dog.scritches + number)::Dog;

CREATE OPERATOR + (
    PROCEDURE="add_scritches_to_dog",
    LEFTARG=Dog,
    RIGHTARG=integer
);
```
*/
#[pg_operator]
#[opname(+)]
fn add_scritches_to_dog(
    mut left: pgx::composite_type!("Dog"),
    right: i32,
) -> pgx::composite_type!("Dog") {
    let left_scritches: i32 = left.get_by_name("scritches").unwrap().unwrap_or_default();
    left.set_by_name("scritches", left_scritches + right).unwrap();
    left
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;
    use pgx::AllocatedByRust;

    #[pg_test]
    fn test_create_dog() {
        let retval = Spi::get_one::<PgHeapTuple<AllocatedByRust>>(
            "\
            SELECT create_dog('Nami', 0)
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval.get_by_name::<&str>("name").unwrap().unwrap(), "Nami");
        assert_eq!(retval.get_by_name::<i32>("scritches").unwrap().unwrap(), 0);
    }

    #[pg_test]
    fn test_scritch_dog() {
        let retval = Spi::get_one::<PgHeapTuple<AllocatedByRust>>(
            "\
            SELECT scritch_dog(ROW('Nami', 1)::Dog)
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval.get_by_name::<&str>("name").unwrap().unwrap(), "Nami");
        assert_eq!(retval.get_by_name::<i32>("scritches").unwrap().unwrap(), 1);
    }

    #[pg_test]
    fn test_make_friendship() {
        let friendship = Spi::get_one::<PgHeapTuple<AllocatedByRust>>(
            "\
            SELECT make_friendship(ROW('Nami', 0)::Dog, ROW('Sally', 0)::Cat)
        ",
        )
        .expect("SQL select failed");
        let dog: PgHeapTuple<AllocatedByRust> = friendship.get_by_name("dog").unwrap().unwrap();
        assert_eq!(dog.get_by_name::<&str>("name").unwrap().unwrap(), "Nami");

        let cat: PgHeapTuple<AllocatedByRust> = friendship.get_by_name("cat").unwrap().unwrap();
        assert_eq!(cat.get_by_name::<&str>("name").unwrap().unwrap(), "Sally");
    }

    #[pg_test]
    fn test_scritch_collector() {
        let retval = Spi::get_one::<i32>(
            "SELECT (scritchcollector(value)).scritches FROM UNNEST(ARRAY [1,2,3]) as value;",
        )
        .expect("SQL select failed");
        assert_eq!(retval, 6);
    }

    #[pg_test]
    fn test_dog_add_operator() {
        let retval = Spi::get_one::<i32>("SELECT (ROW('Nami', 0)::Dog + 1).scritches;")
            .expect("SQL select failed");
        assert_eq!(retval, 1);
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
