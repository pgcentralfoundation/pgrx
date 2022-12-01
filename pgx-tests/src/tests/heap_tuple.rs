// This is used by some, but not all, examples below.
const DOG_COMPOSITE_TYPE: &str = "Dog";

use pgx::pgbox::AllocatedByRust;
use pgx::prelude::*;
use pgx::{Aggregate, VariadicArray};

extension_sql!(
    r#"
CREATE TYPE Dog AS (
    name TEXT,
    scritches INT
);

CREATE TYPE Cat AS (
    name TEXT,
    boops INT
);

CREATE TYPE Fish AS (
    name TEXT,
    bloops INT
);

CREATE TYPE AnimalFriendshipEdge AS (
    friend_1_name TEXT,
    friend_2_name TEXT
);
"#,
    name = "create_composites",
    bootstrap
);

// As Arguments
mod arguments {
    use super::*;

    mod singleton {
        use super::*;

        #[pg_extern]
        fn gets_name_field(dog: Option<pgx::composite_type!("Dog")>) -> Option<&str> {
            // Gets resolved to:
            let dog: Option<PgHeapTuple<AllocatedByRust>> = dog;

            dog?.get_by_name("name").ok()?
        }

        #[pg_extern]
        fn gets_name_field_default(
            dog: default!(pgx::composite_type!("Dog"), "ROW('Nami', 0)::Dog"),
        ) -> &str {
            // Gets resolved to:
            let dog: PgHeapTuple<AllocatedByRust> = dog;

            dog.get_by_name("name").unwrap().unwrap()
        }

        #[pg_extern]
        fn gets_name_field_strict(dog: pgx::composite_type!("Dog")) -> &str {
            // Gets resolved to:
            let dog: PgHeapTuple<AllocatedByRust> = dog;

            dog.get_by_name("name").unwrap().unwrap()
        }
    }

    mod variadic_array {
        use super::*;

        #[pg_extern]
        fn gets_name_field_variadic(
            dogs: VariadicArray<pgx::composite_type!(DOG_COMPOSITE_TYPE)>,
        ) -> Vec<String> {
            // Gets resolved to:
            let dogs: pgx::VariadicArray<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut names = Vec::with_capacity(dogs.len());
            for dog in dogs {
                let dog = dog.unwrap();
                let name = dog.get_by_name("name").unwrap().unwrap();
                names.push(name);
            }
            names
        }

        #[pg_extern]
        fn gets_name_field_default_variadic(
            dogs: default!(
                VariadicArray<pgx::composite_type!("Dog")>,
                "ARRAY[ROW('Nami', 0)]::Dog[]"
            ),
        ) -> Vec<String> {
            // Gets resolved to:
            let dogs: pgx::VariadicArray<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut names = Vec::with_capacity(dogs.len());
            for dog in dogs {
                let dog = dog.unwrap();
                let name = dog.get_by_name("name").unwrap().unwrap();
                names.push(name);
            }
            names
        }

        #[pg_extern]
        fn gets_name_field_strict_variadic(
            dogs: pgx::VariadicArray<pgx::composite_type!("Dog")>,
        ) -> Vec<String> {
            // Gets resolved to:
            let dogs: pgx::VariadicArray<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut names = Vec::with_capacity(dogs.len());
            for dog in dogs.iter() {
                let dog = dog.unwrap();
                let name = dog.get_by_name("name").unwrap().unwrap();
                names.push(name);
            }
            names
        }

        #[pg_extern]
        #[allow(deprecated)]
        fn gets_name_field_as_slice(
            dogs: VariadicArray<pgx::composite_type!(DOG_COMPOSITE_TYPE)>,
        ) -> Vec<String> {
            // Gets resolved to:
            let dogs: pgx::VariadicArray<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut names = Vec::with_capacity(dogs.len());
            for dog in dogs.as_slice() {
                let name = dog.get_by_name("name").unwrap().unwrap();
                names.push(name);
            }
            names
        }
    }

    mod vec {
        use super::*;

        #[pg_extern]
        fn sum_scritches_for_names(
            dogs: Option<Vec<pgx::composite_type!(DOG_COMPOSITE_TYPE)>>,
        ) -> i32 {
            // Gets resolved to:
            let dogs: Option<Vec<PgHeapTuple<AllocatedByRust>>> = dogs;

            let dogs = dogs.unwrap();
            let mut sum_scritches = 0;
            for dog in dogs {
                let scritches: i32 =
                    dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
                sum_scritches += scritches;
            }
            sum_scritches
        }

        #[pg_extern]
        fn sum_scritches_for_names_default(
            dogs: pgx::default!(Vec<pgx::composite_type!("Dog")>, "ARRAY[ROW('Nami', 0)]::Dog[]"),
        ) -> i32 {
            // Gets resolved to:
            let dogs: Vec<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut sum_scritches = 0;
            for dog in dogs {
                let scritches: i32 =
                    dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
                sum_scritches += scritches;
            }
            sum_scritches
        }

        #[pg_extern]
        fn sum_scritches_for_names_strict(dogs: Vec<pgx::composite_type!("Dog")>) -> i32 {
            // Gets resolved to:
            let dogs: Vec<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut sum_scritches = 0;
            for dog in dogs {
                let scritches: i32 =
                    dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
                sum_scritches += scritches;
            }
            sum_scritches
        }

        #[pg_extern]
        fn sum_scritches_for_names_strict_optional_items(
            dogs: Vec<Option<pgx::composite_type!("Dog")>>,
        ) -> i32 {
            // Gets resolved to:
            let dogs: Vec<Option<PgHeapTuple<AllocatedByRust>>> = dogs;

            let mut sum_scritches = 0;
            for dog in dogs {
                let dog = dog.unwrap();
                let scritches: i32 =
                    dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
                sum_scritches += scritches;
            }
            sum_scritches
        }

        #[pg_extern]
        fn sum_scritches_for_names_default_optional_items(
            dogs: pgx::default!(
                Vec<Option<pgx::composite_type!("Dog")>>,
                "ARRAY[ROW('Nami', 0)]::Dog[]"
            ),
        ) -> i32 {
            // Gets resolved to:
            let dogs: Vec<Option<PgHeapTuple<AllocatedByRust>>> = dogs;

            let mut sum_scritches = 0;
            for dog in dogs {
                let dog = dog.unwrap();
                let scritches: i32 =
                    dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
                sum_scritches += scritches;
            }
            sum_scritches
        }

        #[pg_extern]
        fn sum_scritches_for_names_optional_items(
            dogs: Option<Vec<Option<pgx::composite_type!("Dog")>>>,
        ) -> i32 {
            // Gets resolved to:
            let dogs: Option<Vec<Option<PgHeapTuple<AllocatedByRust>>>> = dogs;

            let dogs = dogs.unwrap();
            let mut sum_scritches = 0;
            for dog in dogs {
                let dog = dog.unwrap();
                let scritches: i32 = dog.get_by_name("scritches").unwrap().unwrap();
                sum_scritches += scritches;
            }
            sum_scritches
        }
    }

    mod array {
        use super::*;

        #[pg_extern]
        fn sum_scritches_for_names_array(
            dogs: Option<pgx::Array<pgx::composite_type!("Dog")>>,
        ) -> i32 {
            // Gets resolved to:
            let dogs: Option<pgx::Array<PgHeapTuple<AllocatedByRust>>> = dogs;

            let dogs = dogs.unwrap();
            let mut sum_scritches = 0;
            for dog in dogs {
                let dog = dog.unwrap();
                let scritches: i32 =
                    dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
                sum_scritches += scritches;
            }
            sum_scritches
        }

        #[pg_extern]
        fn sum_scritches_for_names_array_default(
            dogs: pgx::default!(
                pgx::Array<pgx::composite_type!("Dog")>,
                "ARRAY[ROW('Nami', 0)]::Dog[]"
            ),
        ) -> i32 {
            // Gets resolved to:
            let dogs: pgx::Array<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut sum_scritches = 0;
            for dog in dogs {
                let dog = dog.unwrap();
                let scritches: i32 =
                    dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
                sum_scritches += scritches;
            }
            sum_scritches
        }

        #[pg_extern]
        fn sum_scritches_for_names_array_strict(
            dogs: pgx::Array<pgx::composite_type!("Dog")>,
        ) -> i32 {
            // Gets resolved to:
            let dogs: pgx::Array<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut sum_scritches = 0;
            for dog in dogs {
                let dog = dog.unwrap();
                let scritches: i32 =
                    dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
                sum_scritches += scritches;
            }
            sum_scritches
        }
    }
}

// As return types
mod returning {
    use super::*;

    mod singleton {
        use super::*;

        #[pg_extern]
        fn create_dog(name: String, scritches: i32) -> pgx::composite_type!("Dog") {
            let mut tuple = PgHeapTuple::new_composite_type("Dog").unwrap();

            tuple.set_by_name("scritches", scritches).unwrap();
            tuple.set_by_name("name", name).unwrap();

            tuple
        }

        #[pg_extern]
        fn scritch(
            maybe_dog: Option<::pgx::composite_type!("Dog")>,
        ) -> Option<pgx::composite_type!("Dog")> {
            // Gets resolved to:
            let maybe_dog: Option<PgHeapTuple<AllocatedByRust>> = maybe_dog;

            let maybe_dog = if let Some(mut dog) = maybe_dog {
                dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap()).unwrap();
                Some(dog)
            } else {
                None
            };

            maybe_dog
        }

        #[pg_extern]
        fn scritch_strict(dog: ::pgx::composite_type!("Dog")) -> pgx::composite_type!("Dog") {
            // Gets resolved to:
            let mut dog: PgHeapTuple<AllocatedByRust> = dog;

            dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap()).unwrap();

            dog
        }
    }

    mod vec {
        use super::*;

        #[pg_extern]
        fn scritch_all_vec(
            maybe_dogs: Option<Vec<::pgx::composite_type!("Dog")>>,
        ) -> Option<Vec<::pgx::composite_type!("Dog")>> {
            // Gets resolved to:
            let maybe_dogs: Option<Vec<PgHeapTuple<AllocatedByRust>>> = maybe_dogs;

            let maybe_dogs = if let Some(mut dogs) = maybe_dogs {
                for dog in dogs.iter_mut() {
                    dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap())
                        .unwrap();
                }
                Some(dogs)
            } else {
                None
            };

            maybe_dogs
        }

        #[pg_extern]
        fn scritch_all_vec_strict(
            dogs: Vec<::pgx::composite_type!("Dog")>,
        ) -> Vec<::pgx::composite_type!("Dog")> {
            // Gets resolved to:
            let mut dogs: Vec<PgHeapTuple<AllocatedByRust>> = dogs;

            for dog in dogs.iter_mut() {
                dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap()).unwrap();
            }
            dogs
        }

        #[pg_extern]
        fn scritch_all_vec_optional_items(
            maybe_dogs: Option<Vec<Option<::pgx::composite_type!("Dog")>>>,
        ) -> Option<Vec<Option<::pgx::composite_type!("Dog")>>> {
            // Gets resolved to:
            let maybe_dogs: Option<Vec<Option<PgHeapTuple<AllocatedByRust>>>> = maybe_dogs;

            let maybe_dogs = if let Some(mut dogs) = maybe_dogs {
                for dog in dogs.iter_mut() {
                    if let Some(ref mut dog) = dog {
                        dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap())
                            .unwrap();
                    }
                }
                Some(dogs)
            } else {
                None
            };

            maybe_dogs
        }
    }

    // Returning VariadicArray/Array isn't supported, use a Vec.
}

// Just a compile test...
// We don't run these, but we ensure we can build SQL for them
mod sql_generator_tests {
    use super::*;

    #[pg_extern]
    fn exotic_signature(
        _cats: pgx::default!(
            Option<Vec<Option<::pgx::composite_type!("Cat")>>>,
            "ARRAY[ROW('Sally', 0)]::Cat[]"
        ),
        _a_single_fish: pgx::default!(
            Option<::pgx::composite_type!("Fish")>,
            "ROW('Bob', 0)::Fish"
        ),
        _dogs: pgx::default!(
            Option<::pgx::VariadicArray<::pgx::composite_type!("Dog")>>,
            "ARRAY[ROW('Nami', 0), ROW('Brandy', 0)]::Dog[]"
        ),
    ) -> TableIterator<
        'static,
        (
            name!(dog, Option<::pgx::composite_type!("Dog")>),
            name!(cat, Option<::pgx::composite_type!("Cat")>),
            name!(fish, Option<::pgx::composite_type!("Fish")>),
            name!(related_edges, Option<Vec<::pgx::composite_type!("AnimalFriendshipEdge")>>),
        ),
    > {
        TableIterator::new(Vec::new().into_iter())
    }

    #[pg_extern]
    fn iterable_named_table() -> TableIterator<
        'static,
        (name!(dog, ::pgx::composite_type!("Dog")), name!(cat, ::pgx::composite_type!("Cat"))),
    > {
        TableIterator::new(Vec::new().into_iter())
    }

    #[pg_extern]
    fn iterable_named_table_optional_elems() -> TableIterator<
        'static,
        (
            name!(dog, Option<::pgx::composite_type!("Dog")>),
            name!(cat, Option<::pgx::composite_type!("Cat")>),
        ),
    > {
        TableIterator::once(Default::default())
    }

    #[pg_extern]
    fn iterable_named_table_array_elems() -> TableIterator<
        'static,
        (
            name!(dog, Vec<::pgx::composite_type!("Dog")>),
            name!(cat, Vec<::pgx::composite_type!("Cat")>),
        ),
    > {
        TableIterator::once(Default::default())
    }

    #[pg_extern]
    fn iterable_named_table_optional_array_elems() -> TableIterator<
        'static,
        (
            name!(dog, Option<Vec<::pgx::composite_type!("Dog")>>),
            name!(cat, Option<Vec<::pgx::composite_type!("Cat")>>),
        ),
    > {
        TableIterator::once(Default::default())
    }

    #[pg_extern]
    fn iterable_named_table_optional_array_optional_elems() -> TableIterator<
        'static,
        (
            name!(dog, Option<Vec<Option<::pgx::composite_type!("Dog")>>>),
            name!(cat, Option<Vec<Option<::pgx::composite_type!("Cat")>>>),
        ),
    > {
        TableIterator::once(Default::default())
    }

    #[allow(unused_parens)]
    #[pg_extern]
    fn return_table_single() -> TableIterator<'static, (name!(dog, pgx::composite_type!("Dog")),)> {
        let mut tuple = PgHeapTuple::new_composite_type("Dog").unwrap();

        tuple.set_by_name("scritches", 0).unwrap();
        tuple.set_by_name("name", "Nami").unwrap();

        TableIterator::once((tuple,))
    }

    #[pg_extern]
    fn return_table_single_bare(
    ) -> TableIterator<'static, (name!(dog, pgx::composite_type!("Dog")),)> {
        let mut tuple = PgHeapTuple::new_composite_type("Dog").unwrap();

        tuple.set_by_name("scritches", 0).unwrap();
        tuple.set_by_name("name", "Nami").unwrap();

        TableIterator::once((tuple,))
    }

    #[pg_extern]
    fn return_table_two() -> TableIterator<
        'static,
        (name!(dog, pgx::composite_type!("Dog")), name!(cat, pgx::composite_type!("Cat"))),
    > {
        let mut dog_tuple = PgHeapTuple::new_composite_type("Dog").unwrap();

        dog_tuple.set_by_name("scritches", 0).unwrap();
        dog_tuple.set_by_name("name", "Brandy").unwrap();

        let mut cat_tuple = PgHeapTuple::new_composite_type("Cat").unwrap();

        cat_tuple.set_by_name("boops", 0).unwrap();
        cat_tuple.set_by_name("name", "Sally").unwrap();

        TableIterator::once((dog_tuple, cat_tuple))
    }

    #[pg_extern]
    fn return_table_two_optional() -> TableIterator<
        'static,
        (
            name!(dog, Option<pgx::composite_type!("Dog")>),
            name!(cat, Option<pgx::composite_type!("Cat")>),
        ),
    > {
        TableIterator::once((None, None))
    }

    #[derive(Copy, Clone, Default, Debug)]
    pub struct AggregateWithOrderedSetArgs;

    #[pg_aggregate]
    impl Aggregate for AggregateWithOrderedSetArgs {
        type Args = name!(input, pgx::composite_type!("Dog"));
        type State = pgx::composite_type!("Dog");
        type Finalize = pgx::composite_type!("Dog");
        const ORDERED_SET: bool = true;
        type OrderedSetArgs = name!(percentile, pgx::composite_type!("Dog"));

        fn state(
            mut _current: Self::State,
            _arg: Self::Args,
            _fcinfo: pg_sys::FunctionCallInfo,
        ) -> Self::State {
            unimplemented!("Just a SQL generation test")
        }

        fn finalize(
            mut _current: Self::State,
            _direct_arg: Self::OrderedSetArgs,
            _fcinfo: pg_sys::FunctionCallInfo,
        ) -> Self::Finalize {
            unimplemented!("Just a SQL generation test")
        }
    }

    #[derive(Copy, Clone, Default, Debug)]
    pub struct AggregateWithMovingState;

    #[pg_aggregate]
    impl Aggregate for AggregateWithMovingState {
        type Args = pgx::composite_type!("Dog");
        type State = pgx::composite_type!("Dog");
        type MovingState = pgx::composite_type!("Dog");

        fn state(
            mut _current: Self::State,
            _arg: Self::Args,
            _fcinfo: pg_sys::FunctionCallInfo,
        ) -> Self::State {
            unimplemented!("Just a SQL generation test")
        }

        fn moving_state(
            _current: Self::State,
            _arg: Self::Args,
            _fcinfo: pg_sys::FunctionCallInfo,
        ) -> Self::MovingState {
            unimplemented!("Just a SQL generation test")
        }

        fn moving_state_inverse(
            mut _current: Self::State,
            _arg: Self::Args,
            _fcinfo: pg_sys::FunctionCallInfo,
        ) -> Self::MovingState {
            unimplemented!("Just a SQL generation test")
        }

        fn combine(
            mut _first: Self::State,
            _second: Self::State,
            _fcinfo: pg_sys::FunctionCallInfo,
        ) -> Self::State {
            unimplemented!("Just a SQL generation test")
        }
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[cfg(test)]
    use crate as pgx_tests;
    use pgx::datum::TryFromDatumError;
    use pgx::heap_tuple::PgHeapTupleError;
    use pgx::prelude::*;
    use pgx::AllocatedByRust;
    use std::num::NonZeroUsize;

    #[pg_test]
    fn test_gets_name_field() {
        let retval = Spi::get_one::<&str>(
            "
            SELECT gets_name_field(ROW('Nami', 0)::Dog)
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, "Nami");
    }

    #[pg_test]
    fn test_gets_name_field_default() {
        let retval = Spi::get_one::<&str>(
            "
            SELECT gets_name_field_default()
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, "Nami");
    }

    #[pg_test]
    fn test_gets_name_field_strict() {
        let retval = Spi::get_one::<&str>(
            "
            SELECT gets_name_field_strict(ROW('Nami', 0)::Dog)
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, "Nami");
    }

    #[pg_test]
    fn test_gets_name_field_variadic() {
        let retval = Spi::get_one::<Vec<String>>(
            "
            SELECT gets_name_field_variadic(ROW('Nami', 1)::Dog, ROW('Brandy', 1)::Dog)
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, vec!["Nami".to_string(), "Brandy".to_string()]);
    }

    #[pg_test]
    #[should_panic]
    fn test_gets_name_field_as_slice() {
        let retval = Spi::get_one::<Vec<String>>(
            "
            SELECT gets_name_field_as_slice(ROW('Nami', 1)::Dog, ROW('Brandy', 1)::Dog)
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, vec!["Nami".to_string(), "Brandy".to_string()]);
    }

    #[pg_test]
    fn test_sum_scritches_for_names() {
        let retval = Spi::get_one::<i32>(
            "
            SELECT sum_scritches_for_names(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, 43);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_default() {
        let retval = Spi::get_one::<i32>(
            "
            SELECT sum_scritches_for_names_default()
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, 0);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_strict() {
        let retval = Spi::get_one::<i32>(
            "
            SELECT sum_scritches_for_names_strict(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, 43);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_strict_optional_items() {
        let retval = Spi::get_one::<i32>(
            "
            SELECT sum_scritches_for_names_strict(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, 43);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_default_optional_items() {
        let retval = Spi::get_one::<i32>(
            "
            SELECT sum_scritches_for_names_default_optional_items()
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, 0);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_optional_items() {
        let retval = Spi::get_one::<i32>("
            SELECT sum_scritches_for_names_optional_items(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
        ").expect("SQL select failed");
        assert_eq!(retval, 43);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_array() {
        let retval = Spi::get_one::<i32>(
            "
            SELECT sum_scritches_for_names_array(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, 43);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_array_default() {
        let retval = Spi::get_one::<i32>(
            "
            SELECT sum_scritches_for_names_array_default()
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, 0);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_array_strict() {
        let retval = Spi::get_one::<i32>("
            SELECT sum_scritches_for_names_array_strict(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
        ").expect("SQL select failed");
        assert_eq!(retval, 43);
    }

    #[pg_test]
    fn test_create_dog() {
        let retval = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>(
            "
            SELECT create_dog('Nami', 1)
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval.get_by_name("name").unwrap(), Some("Nami"));
        assert_eq!(retval.get_by_name("scritches").unwrap(), Some(1));
    }

    #[pg_test]
    fn test_scritch() {
        let retval = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>(
            "
            SELECT scritch(ROW('Nami', 2)::Dog)
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval.get_by_name("name").unwrap(), Some("Nami"));
        assert_eq!(retval.get_by_name("scritches").unwrap(), Some(2));
    }

    #[pg_test]
    fn test_scritch_strict() {
        let retval = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>(
            "
            SELECT scritch_strict(ROW('Nami', 2)::Dog)
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval.get_by_name("name").unwrap(), Some("Nami"));
        assert_eq!(retval.get_by_name("scritches").unwrap(), Some(2));
    }

    #[pg_test]
    fn test_new_composite_type() {
        Spi::run("CREATE TYPE DogWithAge AS (name text, age int);");
        let mut heap_tuple = PgHeapTuple::new_composite_type("DogWithAge").unwrap();

        assert_eq!(heap_tuple.get_by_name::<String>("name").unwrap(), None);
        assert_eq!(heap_tuple.get_by_name::<i32>("age").unwrap(), None);

        heap_tuple.set_by_name("name", "Brandy".to_string()).unwrap();
        heap_tuple.set_by_name("age", 42).unwrap();

        assert_eq!(heap_tuple.get_by_name("name").unwrap(), Some("Brandy".to_string()));
        assert_eq!(heap_tuple.get_by_name("age").unwrap(), Some(42i32));
    }

    #[pg_test]
    fn test_missing_type() {
        const NON_EXISTING_ATTRIBUTE: &str = "DEFINITELY_NOT_EXISTING";

        match PgHeapTuple::new_composite_type(NON_EXISTING_ATTRIBUTE) {
            Err(PgHeapTupleError::NoSuchType(not_found))
                if not_found == NON_EXISTING_ATTRIBUTE.to_string() =>
            {
                ()
            }
            Err(err) => panic!("{}", err),
            Ok(_) => panic!("Able to find what should be a not existing composite type"),
        }
    }

    #[pg_test]
    fn test_missing_field() {
        Spi::run("CREATE TYPE DogWithAge AS (name text, age int);");
        let mut heap_tuple = PgHeapTuple::new_composite_type("DogWithAge").unwrap();

        const NON_EXISTING_ATTRIBUTE: &str = "DEFINITELY_NOT_EXISTING";
        assert_eq!(
            heap_tuple.get_by_name::<String>(NON_EXISTING_ATTRIBUTE),
            Err(TryFromDatumError::NoSuchAttributeName(NON_EXISTING_ATTRIBUTE.into())),
        );

        assert_eq!(
            heap_tuple.set_by_name(NON_EXISTING_ATTRIBUTE, "Brandy".to_string()),
            Err(TryFromDatumError::NoSuchAttributeName(NON_EXISTING_ATTRIBUTE.into())),
        );
    }

    #[pg_test]
    fn test_missing_number() {
        Spi::run("CREATE TYPE DogWithAge AS (name text, age int);");
        let mut heap_tuple = PgHeapTuple::new_composite_type("DogWithAge").unwrap();

        const NON_EXISTING_ATTRIBUTE: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(9001) };
        assert_eq!(
            heap_tuple.get_by_index::<String>(NON_EXISTING_ATTRIBUTE),
            Err(TryFromDatumError::NoSuchAttributeNumber(NON_EXISTING_ATTRIBUTE)),
        );

        assert_eq!(
            heap_tuple.set_by_index(NON_EXISTING_ATTRIBUTE, "Brandy".to_string()),
            Err(TryFromDatumError::NoSuchAttributeNumber(NON_EXISTING_ATTRIBUTE)),
        );
    }

    #[pg_test]
    fn test_wrong_type_assumed() {
        Spi::run("CREATE TYPE DogWithAge AS (name text, age int);");
        let mut heap_tuple = PgHeapTuple::new_composite_type("DogWithAge").unwrap();

        // These are **deliberately** the wrong types.
        assert_eq!(
            heap_tuple.get_by_name::<i32>("name"),
            Ok(None), // We don't get an error here, yet...
        );
        assert_eq!(
            heap_tuple.get_by_name::<String>("age"),
            Ok(None), // We don't get an error here, yet...
        );

        // These are **deliberately** the wrong types.
        assert_eq!(
            heap_tuple.set_by_name("name", 1_i32),
            Err(TryFromDatumError::IncompatibleTypes),
        );
        assert_eq!(
            heap_tuple.set_by_name("age", "Brandy"),
            Err(TryFromDatumError::IncompatibleTypes),
        );

        // Now set them properly, to test that we get errors when they're set...
        heap_tuple.set_by_name("name", "Brandy".to_string()).unwrap();
        heap_tuple.set_by_name("age", 42).unwrap();

        // These are **deliberately** the wrong types.
        assert_eq!(
            heap_tuple.get_by_name::<i32>("name"),
            Err(TryFromDatumError::IncompatibleTypes),
        );
        assert_eq!(
            heap_tuple.get_by_name::<String>("age"),
            Err(TryFromDatumError::IncompatibleTypes),
        );
    }

    #[pg_test]
    fn test_compatibility() {
        Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>(
            "
            SELECT ROW('Nami', 2)::Dog
        ",
        )
        .expect("SQL select failed");
        // Non-composite types are incompatible:
        assert!(Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>(
            "
            SELECT 1
        ",
        )
        .is_err());
    }
}
