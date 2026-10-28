//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
// This is used by some, but not all, examples below.
const DOG_COMPOSITE_TYPE: &str = "Dog";

use pgrx::VariadicArray;
use pgrx::pgbox::AllocatedByRust;
use pgrx::prelude::*;

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

CREATE TYPE OperatorDog AS (
    name TEXT
);

CREATE TYPE CastDog AS (
    name TEXT
);

CREATE TYPE CastCat AS (
    name TEXT
);
"#,
    name = "create_composites",
    bootstrap
);

// As Arguments
mod arguments {
    use super::*;

    mod singleton {
        //! Originally, these tested taking HeapTuple<'a, _> -> &'a str, but that allows data to
        //! escape the lifetime of its root datum which prevents correct reasoning about lifetimes.
        //! Perhaps they could eventually test &'tup HeapTuple<'mcx, _> -> &'tup str?

        use super::*;
        #[pg_extern]
        fn gets_name_field(dog: Option<pgrx::composite_type!("Dog")>) -> Option<String> {
            // Gets resolved to:
            let dog: Option<PgHeapTuple<AllocatedByRust>> = dog;

            dog?.get_by_name("name").ok()?
        }

        #[pg_extern]
        fn gets_name_field_default(
            dog: default!(pgrx::composite_type!("Dog"), "ROW('Nami', 0)::Dog"),
        ) -> String {
            // Gets resolved to:
            let dog: PgHeapTuple<AllocatedByRust> = dog;

            dog.get_by_name("name").unwrap().unwrap()
        }

        #[pg_extern]
        fn gets_name_field_strict(dog: pgrx::composite_type!("Dog")) -> String {
            // Gets resolved to:
            let dog: PgHeapTuple<AllocatedByRust> = dog;

            dog.get_by_name("name").unwrap().unwrap()
        }
    }

    mod variadic_array {
        //! TODO: biggest problem here is the inability to use
        //! ```ignore
        //! for dog in dogs {
        //!     todo!()
        //! }
        //! ```
        use super::*;

        #[pg_extern]
        fn gets_name_field_variadic(
            dogs: VariadicArray<pgrx::composite_type!(DOG_COMPOSITE_TYPE)>,
        ) -> Vec<String> {
            // Gets resolved to:
            let dogs: VariadicArray<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut names = Vec::with_capacity(dogs.len());
            for dog in dogs.iter() {
                let dog = dog.unwrap();
                let name = dog.get_by_name("name").unwrap().unwrap();
                names.push(name);
            }
            names
        }

        /// TODO: add test which actually calls this maybe???
        #[pg_extern]
        fn gets_name_field_default_variadic(
            dogs: default!(
                VariadicArray<pgrx::composite_type!("Dog")>,
                "ARRAY[ROW('Nami', 0)]::Dog[]"
            ),
        ) -> Vec<String> {
            // Gets resolved to:
            let dogs: VariadicArray<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut names = Vec::with_capacity(dogs.len());
            for dog in dogs.iter() {
                let dog = dog.unwrap();
                let name = dog.get_by_name("name").unwrap().unwrap();
                names.push(name);
            }
            names
        }

        #[pg_extern]
        fn gets_name_field_strict_variadic(
            dogs: VariadicArray<pgrx::composite_type!("Dog")>,
        ) -> Vec<String> {
            // Gets resolved to:
            let dogs: VariadicArray<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut names = Vec::with_capacity(dogs.len());
            for dog in dogs.iter() {
                let dog = dog.unwrap();
                let name = dog.get_by_name("name").unwrap().unwrap();
                names.push(name);
            }
            names
        }
    }

    mod vec {
        //! TODO: This probably requires the FunctionArguments<'curr, 'fn> type
        // use super::*;

        // #[pg_extern]
        // fn sum_scritches_for_names(
        //     dogs: Option<Vec<pgrx::composite_type!(DOG_COMPOSITE_TYPE)>>,
        // ) -> i32 {
        //     // Gets resolved to:
        //     let dogs: Option<Vec<PgHeapTuple<AllocatedByRust>>> = dogs;

        //     let dogs = dogs.unwrap();
        //     let mut sum_scritches = 0;
        //     for dog in dogs {
        //         let scritches: i32 =
        //             dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
        //         sum_scritches += scritches;
        //     }
        //     sum_scritches
        // }

        // #[pg_extern]
        // fn sum_scritches_for_names_default(
        //     dogs: pgrx::default!(Vec<pgrx::composite_type!("Dog")>, "ARRAY[ROW('Nami', 0)]::Dog[]"),
        // ) -> i32 {
        //     // Gets resolved to:
        //     let dogs: Vec<PgHeapTuple<AllocatedByRust>> = dogs;

        //     let mut sum_scritches = 0;
        //     for dog in dogs {
        //         let scritches: i32 =
        //             dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
        //         sum_scritches += scritches;
        //     }
        //     sum_scritches
        // }

        // #[pg_extern]
        // fn sum_scritches_for_names_strict(dogs: Vec<pgrx::composite_type!("Dog")>) -> i32 {
        //     // Gets resolved to:
        //     let dogs: Vec<PgHeapTuple<AllocatedByRust>> = dogs;

        //     let mut sum_scritches = 0;
        //     for dog in dogs {
        //         let scritches: i32 =
        //             dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
        //         sum_scritches += scritches;
        //     }
        //     sum_scritches
        // }

        // #[pg_extern]
        // fn sum_scritches_for_names_strict_optional_items(
        //     dogs: Vec<Option<pgrx::composite_type!("Dog")>>,
        // ) -> i32 {
        //     // Gets resolved to:
        //     let dogs: Vec<Option<PgHeapTuple<AllocatedByRust>>> = dogs;

        //     let mut sum_scritches = 0;
        //     for dog in dogs {
        //         let dog = dog.unwrap();
        //         let scritches: i32 =
        //             dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
        //         sum_scritches += scritches;
        //     }
        //     sum_scritches
        // }

        // #[pg_extern]
        // fn sum_scritches_for_names_default_optional_items(
        //     dogs: pgrx::default!(
        //         Vec<Option<pgrx::composite_type!("Dog")>>,
        //         "ARRAY[ROW('Nami', 0)]::Dog[]"
        //     ),
        // ) -> i32 {
        //     // Gets resolved to:
        //     let dogs: Vec<Option<PgHeapTuple<AllocatedByRust>>> = dogs;

        //     let mut sum_scritches = 0;
        //     for dog in dogs {
        //         let dog = dog.unwrap();
        //         let scritches: i32 =
        //             dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
        //         sum_scritches += scritches;
        //     }
        //     sum_scritches
        // }

        // #[pg_extern]
        // fn sum_scritches_for_names_optional_items(
        //     dogs: Option<Vec<Option<pgrx::composite_type!("Dog")>>>,
        // ) -> i32 {
        //     // Gets resolved to:
        //     let dogs: Option<Vec<Option<PgHeapTuple<AllocatedByRust>>>> = dogs;

        //     let dogs = dogs.unwrap();
        //     let mut sum_scritches = 0;
        //     for dog in dogs {
        //         let dog = dog.unwrap();
        //         let scritches: i32 = dog.get_by_name("scritches").unwrap().unwrap();
        //         sum_scritches += scritches;
        //     }
        //     sum_scritches
        // }
    }

    // mod array {
    //     use super::*;

    //     #[pg_extern]
    //     fn sum_scritches_for_names_array(
    //         dogs: Option<pgrx::Array<pgrx::composite_type!("Dog")>>,
    //     ) -> i32 {
    //         // Gets resolved to:
    //         let dogs: Option<pgrx::Array<PgHeapTuple<AllocatedByRust>>> = dogs;

    //         let dogs = dogs.unwrap();
    //         let mut sum_scritches = 0;
    //         for dog in dogs {
    //             let dog = dog.unwrap();
    //             let scritches: i32 =
    //                 dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
    //             sum_scritches += scritches;
    //         }
    //         sum_scritches
    //     }

    //     #[pg_extern]
    //     fn sum_scritches_for_names_array_default(
    //         dogs: pgrx::default!(
    //             pgrx::Array<pgrx::composite_type!("Dog")>,
    //             "ARRAY[ROW('Nami', 0)]::Dog[]"
    //         ),
    //     ) -> i32 {
    //         // Gets resolved to:
    //         let dogs: pgrx::Array<PgHeapTuple<AllocatedByRust>> = dogs;

    //         let mut sum_scritches = 0;
    //         for dog in dogs {
    //             let dog = dog.unwrap();
    //             let scritches: i32 =
    //                 dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
    //             sum_scritches += scritches;
    //         }
    //         sum_scritches
    //     }

    //     #[pg_extern]
    //     fn sum_scritches_for_names_array_strict(
    //         dogs: pgrx::Array<pgrx::composite_type!("Dog")>,
    //     ) -> i32 {
    //         // Gets resolved to:
    //         let dogs: pgrx::Array<PgHeapTuple<AllocatedByRust>> = dogs;

    //         let mut sum_scritches = 0;
    //         for dog in dogs {
    //             let dog = dog.unwrap();
    //             let scritches: i32 =
    //                 dog.get_by_name("scritches").ok().unwrap_or_default().unwrap_or_default();
    //             sum_scritches += scritches;
    //         }
    //         sum_scritches
    //     }
    // }
}

// As return types
mod returning {
    use super::*;

    mod singleton {
        use super::*;

        #[pg_extern]
        fn create_dog(name: String, scritches: i32) -> pgrx::composite_type!('static, "Dog") {
            let mut tuple = PgHeapTuple::new_composite_type("Dog").unwrap();

            tuple.set_by_name("scritches", scritches).unwrap();
            tuple.set_by_name("name", name).unwrap();

            tuple
        }

        #[pg_extern]
        fn scritch(
            maybe_dog: Option<::pgrx::composite_type!("Dog")>,
        ) -> Option<pgrx::composite_type!("Dog")> {
            // Gets resolved to:
            let maybe_dog: Option<PgHeapTuple<AllocatedByRust>> = maybe_dog;

            if let Some(mut dog) = maybe_dog {
                dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap()).unwrap();
                Some(dog)
            } else {
                None
            }
        }

        #[pg_extern]
        fn scritch_strict(dog: ::pgrx::composite_type!("Dog")) -> pgrx::composite_type!("Dog") {
            // Gets resolved to:
            let mut dog: PgHeapTuple<AllocatedByRust> = dog;

            dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap()).unwrap();

            dog
        }
    }

    mod vec {
        //! TODO: This probably requires the FunctionArguments<'curr, 'fn> type
        // use super::*;

        // #[pg_extern]
        // fn scritch_all_vec(
        //     maybe_dogs: Option<Vec<::pgrx::composite_type!("Dog")>>,
        // ) -> Option<Vec<::pgrx::composite_type!("Dog")>> {
        //     // Gets resolved to:
        //     let maybe_dogs: Option<Vec<PgHeapTuple<AllocatedByRust>>> = maybe_dogs;

        //     let maybe_dogs = if let Some(mut dogs) = maybe_dogs {
        //         for dog in dogs.iter_mut() {
        //             dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap())
        //                 .unwrap();
        //         }
        //         Some(dogs)
        //     } else {
        //         None
        //     };

        //     maybe_dogs
        // }

        // #[pg_extern]
        // fn scritch_all_vec_strict<'a>(
        //     dogs: Vec<::pgrx::composite_type!('a, "Dog")>,
        // ) -> Vec<::pgrx::composite_type!('a, "Dog")> {
        //     // Gets resolved to:
        //     let mut dogs: Vec<PgHeapTuple<AllocatedByRust>> = dogs;

        //     for dog in dogs.iter_mut() {
        //         dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap()).unwrap();
        //     }
        //     dogs
        // }

        // #[pg_extern]
        // fn scritch_all_vec_optional_items(
        //     maybe_dogs: Option<Vec<Option<::pgrx::composite_type!("Dog")>>>,
        // ) -> Option<Vec<Option<::pgrx::composite_type!("Dog")>>> {
        //     // Gets resolved to:
        //     let maybe_dogs: Option<Vec<Option<PgHeapTuple<AllocatedByRust>>>> = maybe_dogs;

        //     let maybe_dogs = if let Some(mut dogs) = maybe_dogs {
        //         for dog in dogs.iter_mut() {
        //             if let Some(ref mut dog) = dog {
        //                 dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap())
        //                     .unwrap();
        //             }
        //         }
        //         Some(dogs)
        //     } else {
        //         None
        //     };

        //     maybe_dogs
        // }
    }

    // Returning VariadicArray/Array isn't supported, use a Vec.
}

// Just a compile test...
// We don't run these, but we ensure we can build SQL for them
mod sql_generator_tests {
    use super::*;

    // #[pg_extern]
    // fn exotic_signature(
    //     _cats: pgrx::default!(
    //         Option<Vec<Option<::pgrx::composite_type!('static, "Cat")>>>,
    //         "ARRAY[ROW('Sally', 0)]::Cat[]"
    //     ),
    //     _a_single_fish: pgrx::default!(
    //         Option<::pgrx::composite_type!('static, "Fish")>,
    //         "ROW('Bob', 0)::Fish"
    //     ),
    //     _dogs: pgrx::default!(
    //         Option<::pgrx::VariadicArray<::pgrx::composite_type!('static, "Dog")>>,
    //         "ARRAY[ROW('Nami', 0), ROW('Brandy', 0)]::Dog[]"
    //     ),
    // ) -> TableIterator<
    //     'static,
    //     (
    //         name!(dog, Option<::pgrx::composite_type!('static, "Dog")>),
    //         name!(cat, Option<::pgrx::composite_type!('static, "Cat")>),
    //         name!(fish, Option<::pgrx::composite_type!('static, "Fish")>),
    //         name!(
    //             related_edges,
    //             Option<Vec<::pgrx::composite_type!('static, "AnimalFriendshipEdge")>>
    //         ),
    //     ),
    // > {
    //     TableIterator::new(Vec::new().into_iter())
    // }

    #[pg_extern]
    fn iterable_named_table() -> TableIterator<
        'static,
        (
            name!(dog, ::pgrx::composite_type!('static, "Dog")),
            name!(cat, ::pgrx::composite_type!('static, "Cat")),
        ),
    > {
        TableIterator::new(Vec::new())
    }

    #[pg_extern]
    fn iterable_named_table_optional_elems() -> TableIterator<
        'static,
        (
            name!(dog, Option<::pgrx::composite_type!('static, "Dog")>),
            name!(cat, Option<::pgrx::composite_type!('static, "Cat")>),
        ),
    > {
        TableIterator::once(Default::default())
    }

    #[pg_extern]
    fn iterable_named_table_array_elems() -> TableIterator<
        'static,
        (
            name!(dog, Vec<::pgrx::composite_type!('static, "Dog")>),
            name!(cat, Vec<::pgrx::composite_type!('static, "Cat")>),
        ),
    > {
        TableIterator::once(Default::default())
    }

    #[pg_extern]
    fn iterable_named_table_optional_array_elems() -> TableIterator<
        'static,
        (
            name!(dog, Option<Vec<::pgrx::composite_type!('static, "Dog")>>),
            name!(cat, Option<Vec<::pgrx::composite_type!('static, "Cat")>>),
        ),
    > {
        TableIterator::once(Default::default())
    }

    #[pg_extern]
    fn iterable_named_table_optional_array_optional_elems() -> TableIterator<
        'static,
        (
            name!(dog, Option<Vec<Option<::pgrx::composite_type!('static, "Dog")>>>),
            name!(cat, Option<Vec<Option<::pgrx::composite_type!('static, "Cat")>>>),
        ),
    > {
        TableIterator::once(Default::default())
    }

    #[allow(unused_parens)]
    #[pg_extern]
    fn return_table_single()
    -> TableIterator<'static, (name!(dog, pgrx::composite_type!('static, "Dog")),)> {
        let mut tuple = PgHeapTuple::new_composite_type("Dog").unwrap();

        tuple.set_by_name("scritches", 0).unwrap();
        tuple.set_by_name("name", "Nami").unwrap();

        TableIterator::once((tuple,))
    }

    #[pg_extern]
    fn return_table_single_bare()
    -> TableIterator<'static, (name!(dog, pgrx::composite_type!('static, "Dog")),)> {
        let mut tuple = PgHeapTuple::new_composite_type("Dog").unwrap();

        tuple.set_by_name("scritches", 0).unwrap();
        tuple.set_by_name("name", "Nami").unwrap();

        TableIterator::once((tuple,))
    }

    #[pg_extern]
    fn return_table_two() -> TableIterator<
        'static,
        (
            name!(dog, pgrx::composite_type!('static, "Dog")),
            name!(cat, pgrx::composite_type!('static, "Cat")),
        ),
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
            name!(dog, Option<pgrx::composite_type!('static, "Dog")>),
            name!(cat, Option<pgrx::composite_type!('static, "Cat")>),
        ),
    > {
        TableIterator::once((None, None))
    }

    #[pg_extern]
    fn generate_lots_of_dogs() -> SetOfIterator<'static, pgrx::composite_type!('static, "Dog")> {
        let tuple_desc =
            pgrx::PgTupleDesc::for_composite_type("Dog").expect("Couldn't find TestType");

        let tuples: Vec<PgHeapTuple<'_, AllocatedByRust>> = (0..10_000)
            .map(move |i| {
                let datums: Vec<Option<pg_sys::Datum>> =
                    vec!["good boy".into_datum(), i.into_datum()];

                unsafe {
                    PgHeapTuple::from_datums(tuple_desc.clone(), datums)
                        .expect("couldn't get heap tuple")
                }
            })
            .collect();

        SetOfIterator::new(tuples)
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use pgrx::AllocatedByRust;
    use pgrx::datum::TryFromDatumError;
    use pgrx::heap_tuple::PgHeapTupleError;
    use pgrx::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::num::NonZeroUsize;

    /// Created in the `tests` schema, which is not on `search_path`, so the name lookup in its
    /// `IntoDatum::type_oid` raises an ERROR.
    #[derive(Debug, PostgresType, Serialize, Deserialize)]
    pub struct OffSearchPath;

    #[pg_test]
    fn test_gets_name_field() {
        let retval = Spi::get_one::<&str>(
            "
            SELECT gets_name_field(ROW('Nami', 0)::Dog)
        ",
        );
        assert_eq!(retval, Ok(Some("Nami")));
    }

    #[pg_test]
    fn test_gets_name_field_default() {
        let retval = Spi::get_one::<&str>(
            "
            SELECT gets_name_field_default()
        ",
        );
        assert_eq!(retval, Ok(Some("Nami")));
    }

    #[pg_test]
    fn test_gets_name_field_strict() {
        let retval = Spi::get_one::<&str>(
            "
            SELECT gets_name_field_strict(ROW('Nami', 0)::Dog)
        ",
        );
        assert_eq!(retval, Ok(Some("Nami")));
    }

    #[pg_test]
    fn test_gets_name_field_variadic() {
        let retval = Spi::get_one::<Vec<String>>(
            "
            SELECT gets_name_field_variadic(ROW('Nami', 1)::Dog, ROW('Brandy', 1)::Dog)
        ",
        );
        assert_eq!(retval, Ok(Some(vec!["Nami".to_string(), "Brandy".to_string()])));
    }

    // #[pg_test]
    // fn test_sum_scritches_for_names() {
    //     let retval = Spi::get_one::<i32>(
    //         "
    //         SELECT sum_scritches_for_names(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
    //     ",
    //     );
    //     assert_eq!(retval, Ok(Some(43)));
    // }

    // #[pg_test]
    // fn test_sum_scritches_for_names_default() {
    //     let retval = Spi::get_one::<i32>(
    //         "
    //         SELECT sum_scritches_for_names_default()
    //     ",
    //     );
    //     assert_eq!(retval, Ok(Some(0)));
    // }

    // #[pg_test]
    // fn test_sum_scritches_for_names_strict() {
    //     let retval = Spi::get_one::<i32>(
    //         "
    //         SELECT sum_scritches_for_names_strict(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
    //     ",
    //     );
    //     assert_eq!(retval, Ok(Some(43)));
    // }

    // #[pg_test]
    // fn test_sum_scritches_for_names_strict_optional_items() {
    //     let retval = Spi::get_one::<i32>(
    //         "
    //         SELECT sum_scritches_for_names_strict(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
    //     ",
    //     );
    //     assert_eq!(retval, Ok(Some(43)));
    // }

    // #[pg_test]
    // fn test_sum_scritches_for_names_default_optional_items() {
    //     let retval = Spi::get_one::<i32>(
    //         "
    //         SELECT sum_scritches_for_names_default_optional_items()
    //     ",
    //     );
    //     assert_eq!(retval, Ok(Some(0)));
    // }

    // #[pg_test]
    // fn test_sum_scritches_for_names_optional_items() {
    //     let retval = Spi::get_one::<i32>("
    //         SELECT sum_scritches_for_names_optional_items(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
    //     ");
    //     assert_eq!(retval, Ok(Some(43)));
    // }

    // #[pg_test]
    // fn test_sum_scritches_for_names_array() {
    //     let retval = Spi::get_one::<i32>(
    //         "
    //         SELECT sum_scritches_for_names_array(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
    //     ",
    //     );
    //     assert_eq!(retval, Ok(Some(43)));
    // }

    // #[pg_test]
    // fn test_sum_scritches_for_names_array_default() {
    //     let retval = Spi::get_one::<i32>(
    //         "
    //         SELECT sum_scritches_for_names_array_default()
    //     ",
    //     );
    //     assert_eq!(retval, Ok(Some(0)));
    // }

    // #[pg_test]
    // fn test_sum_scritches_for_names_array_strict() {
    //     let retval = Spi::get_one::<i32>("
    //         SELECT sum_scritches_for_names_array_strict(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
    //     ");
    //     assert_eq!(retval, Ok(Some(43)));
    // }

    #[pg_test]
    fn test_create_dog() -> Result<(), pgrx::spi::Error> {
        let retval = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>(
            "
            SELECT create_dog('Nami', 1)
        ",
        )?
        .expect("datum was null");
        assert_eq!(retval.get_by_name("name").unwrap(), Some("Nami"));
        assert_eq!(retval.get_by_name("scritches").unwrap(), Some(1));
        Ok(())
    }

    #[pg_test]
    fn test_scritch() -> Result<(), pgrx::spi::Error> {
        let retval = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>(
            "
            SELECT scritch(ROW('Nami', 2)::Dog)
        ",
        )?
        .expect("datum was null");
        assert_eq!(retval.get_by_name("name").unwrap(), Some("Nami"));
        assert_eq!(retval.get_by_name("scritches").unwrap(), Some(2));
        Ok(())
    }

    #[pg_test]
    fn test_scritch_strict() -> Result<(), pgrx::spi::Error> {
        let retval = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>(
            "
            SELECT scritch_strict(ROW('Nami', 2)::Dog)
        ",
        )?
        .expect("datum was null");
        assert_eq!(retval.get_by_name("name").unwrap(), Some("Nami"));
        assert_eq!(retval.get_by_name("scritches").unwrap(), Some(2));
        Ok(())
    }

    #[pg_test]
    fn test_new_composite_type() {
        Spi::run("CREATE TYPE DogWithAge AS (name text, age int);").expect("SPI failed");
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
            Err(PgHeapTupleError::NoSuchType(not_found)) if not_found == NON_EXISTING_ATTRIBUTE => {
            }
            Err(err) => panic!("{err}"),
            Ok(_) => panic!("Able to find what should be a not existing composite type"),
        }
    }

    #[pg_test]
    fn test_missing_field() {
        Spi::run("CREATE TYPE DogWithAge AS (name text, age int);").expect("SPI failed");
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
        Spi::run("CREATE TYPE DogWithAge AS (name text, age int);").expect("SPI failed");
        let mut heap_tuple = PgHeapTuple::new_composite_type("DogWithAge").unwrap();

        const NON_EXISTING_ATTRIBUTE: NonZeroUsize = NonZeroUsize::new(9001).unwrap();
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
        Spi::run("CREATE TYPE DogWithAge AS (name text, age int);").expect("SPI failed");
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
        assert!(matches!(
            heap_tuple.set_by_name("name", 1_i32),
            Err(TryFromDatumError::IncompatibleTypes { .. })
        ));
        assert!(matches!(
            heap_tuple.set_by_name("age", "Brandy"),
            Err(TryFromDatumError::IncompatibleTypes { .. }),
        ));

        // Now set them properly, to test that we get errors when they're set...
        heap_tuple.set_by_name("name", "Brandy".to_string()).unwrap();
        heap_tuple.set_by_name("age", 42).unwrap();

        // These are **deliberately** the wrong types.
        assert!(matches!(
            heap_tuple.get_by_name::<i32>("name"),
            Err(TryFromDatumError::IncompatibleTypes { .. }),
        ));
        assert!(matches!(
            heap_tuple.get_by_name::<String>("age"),
            Err(TryFromDatumError::IncompatibleTypes { .. }),
        ));
    }

    #[pg_test]
    fn test_compatibility() {
        Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>("SELECT ROW('Nami', 2)::Dog")
            .expect("failed to get a Dog");

        // Non-composite types are incompatible:
        let not_a_dog = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>("SELECT 1");
        match not_a_dog {
            Ok(_dog) => panic!("got a Dog when we shouldn't have"),
            Err(e) => {
                assert!(matches!(
                    e,
                    pgrx::spi::Error::DatumError(TryFromDatumError::IncompatibleTypes { .. })
                ))
            }
        }
    }

    #[pg_test]
    fn test_tuple_desc_clone() -> Result<(), spi::Error> {
        let result = Spi::connect(|client| {
            let query = "select * from generate_lots_of_dogs()";
            client.select(query, None, &[]).map(|table| table.len())
        })?;
        assert_eq!(result, 10_000);
        Ok(())
    }

    /// Build a two-attribute Dog tuple with `scritches` set to 3.
    fn scritched_dog()
    -> (pgrx::PgTupleDesc<'static>, pgrx::PgBox<pg_sys::HeapTupleData, AllocatedByRust>) {
        let tupdesc = pgrx::PgTupleDesc::for_composite_type("Dog").unwrap();
        let mut dog = PgHeapTuple::new_composite_type("Dog").unwrap();
        dog.set_by_name("scritches", 3_i32).unwrap();
        // SAFETY: into_composite_datum returns a live Dog composite datum.
        let tuple = unsafe {
            pgrx::htup::composite_row_type_make_tuple(dog.into_composite_datum().unwrap())
        };
        (tupdesc, tuple)
    }

    #[pg_test]
    fn test_composite_row_type_make_tuple_unpacks_short_header() -> Result<(), spi::Error> {
        Spi::run(
            "CREATE TABLE stored_dogs (dog Dog); INSERT INTO stored_dogs VALUES (ROW('Nami', 7));",
        )?;
        let tupdesc = pgrx::PgTupleDesc::for_composite_type("Dog").unwrap();
        Spi::connect(|client| {
            let row = client.select("SELECT dog FROM stored_dogs", None, &[])?.first();
            let datum = row.get_datum_by_ordinal(1)?.expect("stored dog is not NULL");
            // SAFETY: the SPI tuple holding this datum stays live inside the closure.
            let short_header =
                unsafe { pg_sys::VARATT_IS_1B!(datum.cast_mut_ptr::<pg_sys::varlena>()).is_true() };
            // A composite this small is stored with a one-byte varlena header.
            assert!(short_header, "the stored composite should be packed");

            // SAFETY: `datum` is a live Dog composite datum read from the SPI tuple.
            let tuple = unsafe { pgrx::htup::composite_row_type_make_tuple(datum) };
            // SAFETY: the detoasted copy has an ordinary four-byte header.
            let unpacked_len =
                unsafe { pg_sys::VARSIZE_ANY!(tuple.t_data.cast::<pg_sys::varlena>()).get() };
            assert_eq!(tuple.t_len as usize, unpacked_len as usize);
            // SAFETY: `tuple` is the live Dog row that `tupdesc` describes.
            let (name, scritches) = unsafe {
                (
                    pgrx::htup::heap_getattr::<String, _>(
                        &tuple,
                        NonZeroUsize::new(1).unwrap(),
                        &tupdesc,
                    ),
                    pgrx::htup::heap_getattr::<i32, _>(
                        &tuple,
                        NonZeroUsize::new(2).unwrap(),
                        &tupdesc,
                    ),
                )
            };
            assert_eq!(name.as_deref(), Some("Nami"));
            assert_eq!(scritches, Some(7));
            Ok(())
        })
    }

    #[pg_test(error = "no attribute")]
    fn test_htup_heap_getattr_rejects_attno_past_tupdesc() {
        let (tupdesc, tuple) = scritched_dog();
        let scritches = NonZeroUsize::new(2).unwrap();
        // SAFETY: `tuple` is the live Dog row that `tupdesc` describes.
        let read = unsafe { pgrx::htup::heap_getattr::<i32, _>(&tuple, scritches, &tupdesc) };
        assert_eq!(read, Some(3));

        let past_end = NonZeroUsize::new(3).unwrap();
        // SAFETY: as above; the attribute number is checked against `tupdesc`.
        let _ = unsafe { pgrx::htup::heap_getattr::<i32, _>(&tuple, past_end, &tupdesc) };
    }

    #[pg_test(error = "no attribute")]
    fn test_htup_heap_getattr_datum_ex_rejects_attno_past_tupdesc() {
        let (tupdesc, tuple) = scritched_dog();
        // SAFETY: `tuple` owns this HeapTupleData and outlives the borrowed view.
        let borrowed = unsafe { pgrx::PgBox::<pg_sys::HeapTupleData>::from_pg(tuple.as_ptr()) };
        let scritches = NonZeroUsize::new(2).unwrap();
        // SAFETY: `borrowed` views the live Dog row that `tupdesc` describes.
        let attr = unsafe { pgrx::htup::heap_getattr_datum_ex(&borrowed, scritches, &tupdesc) };
        // SAFETY: `attr` is the non-NULL int4 `scritches` of the live row.
        assert_eq!(unsafe { attr.into_value::<i32>() }, 3);

        let past_end = NonZeroUsize::new(3).unwrap();
        // SAFETY: as above; the attribute number is checked against `tupdesc`.
        let _ = unsafe { pgrx::htup::heap_getattr_datum_ex(&borrowed, past_end, &tupdesc) };
    }

    /// Read a by-value attribute of each width, plus a varlena, from tuples with and without a
    /// NULL. Reading twice without NULLs makes the second pass use the offsets that the first
    /// pass cached in the descriptor.
    #[pg_test]
    fn test_heap_tuple_reads_each_by_value_width() {
        Spi::run(
            r#"CREATE TYPE FetchWidths AS (b bool, c "char", s int2, i int4, l int8, t text)"#,
        )
        .unwrap();
        let mut row = PgHeapTuple::new_composite_type("FetchWidths").unwrap();
        row.set_by_name("b", true).unwrap();
        row.set_by_name("c", -3_i8).unwrap();
        row.set_by_name("s", -2_i16).unwrap();
        row.set_by_name("i", -4_i32).unwrap();
        row.set_by_name("t", "text").unwrap();

        // `l` is NULL, so reads check the NULL bitmap before calling nocachegetattr.
        assert_eq!(row.get_by_name::<i64>("l").unwrap(), None);
        assert_eq!(row.get_by_name::<i8>("c").unwrap(), Some(-3));
        assert_eq!(row.get_by_name::<String>("t").unwrap().as_deref(), Some("text"));

        row.set_by_name("l", -8_i64).unwrap();
        for _ in 0..2 {
            assert_eq!(row.get_by_name::<bool>("b").unwrap(), Some(true));
            assert_eq!(row.get_by_name::<i8>("c").unwrap(), Some(-3));
            assert_eq!(row.get_by_name::<i16>("s").unwrap(), Some(-2));
            assert_eq!(row.get_by_name::<i32>("i").unwrap(), Some(-4));
            assert_eq!(row.get_by_name::<i64>("l").unwrap(), Some(-8));
            assert_eq!(row.get_by_name::<String>("t").unwrap().as_deref(), Some("text"));
        }
    }

    /// A row stored before `ALTER TYPE ... ADD ATTRIBUTE` has fewer attributes than the
    /// descriptor, so reading the new attribute goes through getmissingattr.
    #[pg_test]
    fn test_heap_getattr_reads_attribute_added_after_the_row() -> Result<(), spi::Error> {
        Spi::run(
            "CREATE TYPE GrowingDog AS (name text, scritches int4);
             CREATE TABLE growing_dogs (dog GrowingDog);
             INSERT INTO growing_dogs VALUES (ROW('Nami', 7));
             ALTER TYPE GrowingDog ADD ATTRIBUTE age int4 CASCADE;",
        )?;
        let scritches = NonZeroUsize::new(2).unwrap();
        let age = NonZeroUsize::new(3).unwrap();
        Spi::connect(|client| {
            let row = client.select("SELECT dog FROM growing_dogs", None, &[])?.first();
            let datum = row.get_datum_by_ordinal(1)?.expect("stored dog is not NULL");

            // SAFETY: `datum` is a live GrowingDog composite read from the SPI tuple.
            let dog = unsafe { PgHeapTuple::from_composite_datum(datum) };
            assert_eq!(dog.len(), 3);
            assert_eq!(dog.get_by_index::<i32>(scritches)?, Some(7));
            assert_eq!(dog.get_by_index::<i32>(age)?, None);

            let tupdesc = pgrx::PgTupleDesc::for_composite_type("GrowingDog").unwrap();
            // SAFETY: as above.
            let tuple = unsafe { pgrx::htup::composite_row_type_make_tuple(datum) };
            // SAFETY: `tuple` is the live stored row. It was formed before `age` was added, so
            // each attribute it stores has the length, alignment, and pass-by-value flag of
            // the attribute in that position of `tupdesc`.
            unsafe {
                assert_eq!(
                    pgrx::htup::heap_getattr::<i32, _>(&tuple, scritches, &tupdesc),
                    Some(7)
                );
                assert_eq!(pgrx::htup::heap_getattr::<i32, _>(&tuple, age, &tupdesc), None);
            }
            Ok(())
        })
    }

    /// A row stored before `ALTER TYPE ... DROP ATTRIBUTE` still holds the dropped attribute,
    /// and the descriptor keeps its length, alignment, and pass-by-value flag, so the
    /// attributes after it read correctly. `heap_getattr_datum_ex` reads the dropped attribute
    /// itself without looking up its cleared type, and `get_by_index` reads it as NULL.
    #[pg_test]
    fn test_heap_getattr_reads_row_stored_before_a_drop() -> Result<(), spi::Error> {
        Spi::run(
            "CREATE TYPE ShrinkingDog AS (name text, collar text, scritches int4);
             CREATE TABLE shrinking_dogs (dog ShrinkingDog);
             INSERT INTO shrinking_dogs VALUES (ROW('Nami', 'red', 7));
             ALTER TYPE ShrinkingDog DROP ATTRIBUTE collar CASCADE;",
        )?;
        let scritches = NonZeroUsize::new(3).unwrap();
        Spi::connect(|client| {
            let row = client.select("SELECT dog FROM shrinking_dogs", None, &[])?.first();
            let datum = row.get_datum_by_ordinal(1)?.expect("stored dog is not NULL");

            // SAFETY: `datum` is a live ShrinkingDog composite read from the SPI tuple.
            let dog = unsafe { PgHeapTuple::from_composite_datum(datum) };
            assert_eq!(dog.get_by_index::<i32>(scritches)?, Some(7));

            let tupdesc = pgrx::PgTupleDesc::for_composite_type("ShrinkingDog").unwrap();
            // SAFETY: as above.
            let tuple = unsafe { pgrx::htup::composite_row_type_make_tuple(datum) };
            // SAFETY: `tuple` is the live stored row. It was formed before `collar` was
            // dropped, and `tupdesc` keeps that attribute's length, alignment, and
            // pass-by-value flag, though not its type, so each stored attribute matches the
            // attribute in that position of `tupdesc`.
            let read = unsafe { pgrx::htup::heap_getattr::<i32, _>(&tuple, scritches, &tupdesc) };
            assert_eq!(read, Some(7));

            // SAFETY: `tuple` owns this HeapTupleData and outlives the borrowed view.
            let borrowed = unsafe { pgrx::PgBox::<pg_sys::HeapTupleData>::from_pg(tuple.as_ptr()) };
            let dropped = NonZeroUsize::new(2).unwrap();
            // SAFETY: as for heap_getattr above.
            let (collar, scritches) = unsafe {
                (
                    pgrx::htup::heap_getattr_datum_ex(&borrowed, dropped, &tupdesc),
                    pgrx::htup::heap_getattr_datum_ex(&borrowed, scritches, &tupdesc),
                )
            };
            // The dropped attribute has no type, but keeps its stored value's layout.
            assert!(!collar.is_null);
            assert_eq!(collar.typoid, PgOid::Invalid);
            assert_eq!((collar.typlen, collar.typbyval), (-1, false));
            // get_by_index reads it as NULL. Checking `T` against InvalidOid raised an ERROR,
            // whether or not `T` reads records.
            assert_eq!(dog.get_by_index::<String>(dropped)?, None);
            let as_row = dog.get_by_index::<PgHeapTuple<'_, AllocatedByRust>>(dropped);
            assert!(matches!(as_row, Ok(None)), "{:?}", as_row.as_ref().err());
            assert_eq!((scritches.typlen, scritches.typbyval), (4, true));
            // SAFETY: `scritches` is the non-NULL int4 attribute of the live stored row.
            assert_eq!(unsafe { scritches.into_value::<i32>() }, 7);
            Ok(())
        })
    }

    /// Reading a NULL attribute returns `Ok(None)` without asking `T` for its type OID, which
    /// can raise an ERROR.
    #[pg_test]
    fn test_get_by_index_reads_null_without_type_lookup() -> Result<(), spi::Error> {
        Spi::run("CREATE TYPE HoldsOffSearchPath AS (v tests.OffSearchPath)")?;
        let lookup_fails = Spi::get_one::<bool>("SELECT to_regtype('OffSearchPath') IS NULL")?;
        assert_eq!(lookup_fails, Some(true));

        let row = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>(
            "SELECT ROW(NULL)::HoldsOffSearchPath",
        )?
        .expect("the row is not NULL");
        let read = row.get_by_index::<OffSearchPath>(NonZeroUsize::new(1).unwrap());
        assert!(matches!(read, Ok(None)), "{read:?}");
        Ok(())
    }

    /// Reading an `int4` attribute as a composite returns an error instead of reading the
    /// integer as a pointer to a tuple.
    #[pg_test]
    fn test_get_by_index_rejects_scalar_attribute_as_composite() -> Result<(), spi::Error> {
        Spi::run("CREATE TYPE ProbeIntHolder AS (v int4)")?;
        let row =
            Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>("SELECT ROW(7)::ProbeIntHolder")?
                .expect("the row is not NULL");

        let by_index =
            row.get_by_index::<PgHeapTuple<'_, AllocatedByRust>>(NonZeroUsize::new(1).unwrap());
        let by_name = row.get_by_name::<PgHeapTuple<'_, AllocatedByRust>>("v");
        for read in [by_index, by_name] {
            assert!(
                matches!(
                    read,
                    Err(TryFromDatumError::IncompatibleTypes { datum_oid, .. })
                        if datum_oid == pg_sys::INT4OID
                ),
                "{:?}",
                read.as_ref().err()
            );
        }
        assert_eq!(row.get_by_index::<i32>(NonZeroUsize::new(1).unwrap())?, Some(7));
        Ok(())
    }

    /// Composite attributes, including one whose type is a domain over a composite type, still
    /// read as composites.
    #[pg_test]
    fn test_get_by_index_reads_composite_and_domain_attributes() -> Result<(), spi::Error> {
        Spi::run(
            "CREATE DOMAIN DomainDog AS Dog;
             CREATE TYPE DogHolder AS (dog Dog, domain_dog DomainDog);",
        )?;
        let row = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>(
            "SELECT ROW(ROW('Nami', 7)::Dog, ROW('Brandy', 3)::DomainDog)::DogHolder",
        )?
        .expect("the row is not NULL");

        for (attname, name, scritches) in [("dog", "Nami", 7), ("domain_dog", "Brandy", 3)] {
            let dog = row
                .get_by_name::<PgHeapTuple<'_, AllocatedByRust>>(attname)?
                .expect("the dog is not NULL");
            assert_eq!(dog.get_by_name::<String>("name")?.as_deref(), Some(name));
            assert_eq!(dog.get_by_name::<i32>("scritches")?, Some(scritches));
        }
        Ok(())
    }

    /// `scritches` follows the variable-width `name`, so it has no cached offset and reading it
    /// goes through `nocachegetattr`. On PG15-17 that walk caches the offset of `name` in the
    /// `FormData_pg_attribute` that the attribute accessors read, so an attribute taken before
    /// the read must be a copy that keeps its old `attcacheoff`. PG18 caches it in the
    /// `CompactAttribute` instead, and PG19 caches only the offsets it computes when it builds
    /// the descriptor, so neither writes a `FormData_pg_attribute`.
    #[pg_test]
    fn test_attribute_copies_survive_offset_caching() -> Result<(), spi::Error> {
        Spi::run("CREATE TYPE CachedDog AS (age int4, name text, scritches int4)")?;
        let tupdesc = pgrx::PgTupleDesc::for_composite_type("CachedDog").unwrap();
        #[cfg(any(feature = "pg18", feature = "pg19"))]
        let desc = tupdesc.as_ptr();
        let values = [3_i32.into_datum(), "Nami".into_datum(), 7_i32.into_datum()];
        // SAFETY: each datum is a non-NULL value of the attribute type in its position.
        let dog = unsafe { PgHeapTuple::from_datums(tupdesc, values) }.unwrap();
        let name = NonZeroUsize::new(2).unwrap();
        let scritches = NonZeroUsize::new(3).unwrap();
        // The `CompactAttribute` offset cache of `name`, which PG18 and later read instead.
        #[cfg(any(feature = "pg18", feature = "pg19"))]
        let name_compact_cacheoff = || {
            // SAFETY: `dog` keeps `desc` live. PG18 and later end the descriptor with one
            // `CompactAttribute` per attribute, and `name` is the second of three. The raw place
            // projection reads the field without creating a reference.
            unsafe {
                let compact = core::ptr::addr_of!((*desc).compact_attrs)
                    .cast::<pg_sys::CompactAttribute>()
                    .add(1);
                (*compact).attcacheoff
            }
        };

        let held = dog.get_attribute_by_index(name).unwrap();
        let listed = dog.attributes().collect::<Vec<_>>();
        #[cfg(any(feature = "pg18", feature = "pg19"))]
        assert_eq!(name_compact_cacheoff(), -1);

        assert_eq!(dog.get_by_index::<i32>(scritches)?, Some(7));
        let fresh = dog.get_attribute_by_index(name).unwrap();

        #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
        {
            // nocachegetattr cached `name` at offset 4, after the int4 `age`.
            assert_eq!(fresh.attcacheoff, 4);
            assert_eq!(held.attcacheoff, -1);
            assert_eq!(listed[1].1.attcacheoff, -1);
        }
        #[cfg(any(feature = "pg18", feature = "pg19"))]
        {
            assert_eq!(format!("{held:?}"), format!("{fresh:?}"));
            assert_eq!(format!("{:?}", listed[1].1), format!("{fresh:?}"));
        }
        #[cfg(feature = "pg18")]
        assert_eq!(name_compact_cacheoff(), 4);
        #[cfg(feature = "pg19")]
        assert_eq!(name_compact_cacheoff(), -1);
        Ok(())
    }

    /// Setting a dropped attribute returns `NoSuchAttributeNumber`. Its type is InvalidOid, so
    /// the type check raised an ERROR for `&str` and accepted a tuple, whose `type_oid` is also
    /// InvalidOid.
    #[pg_test]
    fn test_set_by_index_rejects_dropped_attribute() -> Result<(), spi::Error> {
        Spi::run(
            "CREATE TYPE CollarlessDog AS (name text, collar text, scritches int4);
             ALTER TYPE CollarlessDog DROP ATTRIBUTE collar;",
        )?;
        let mut dog = PgHeapTuple::new_composite_type("CollarlessDog").unwrap();
        let collar = NonZeroUsize::new(2).unwrap();
        let rejected = Err(TryFromDatumError::NoSuchAttributeNumber(collar));
        assert_eq!(dog.set_by_index(collar, "red"), rejected);
        assert_eq!(dog.set_by_index(collar, (Some(1), Some(2))), rejected);
        Ok(())
    }

    /// A `PgBox` read from a tuple must be a copy. The tuple frees its memory when an attribute
    /// is set and when it drops, which an assert-enabled build overwrites with 0x7F bytes, and
    /// writing through a box that pointed into the tuple would change the tuple.
    #[pg_test]
    fn test_get_by_index_copies_pgbox() -> Result<(), spi::Error> {
        Spi::run("CREATE TYPE PointHolder AS (p point)")?;
        let point = |x: f64| pg_sys::Point { x, y: -x };
        let p = NonZeroUsize::new(1).unwrap();
        let mut holder = PgHeapTuple::new_composite_type("PointHolder").unwrap();
        holder.set_by_name("p", point(1.0))?;

        let first = holder.get_by_name::<PgBox<pg_sys::Point>>("p")?.expect("NULL point");
        holder.set_by_name("p", point(2.0))?;
        assert_eq!(*first, point(1.0));

        let mut second = holder.get_by_index::<PgBox<pg_sys::Point>>(p)?.expect("NULL point");
        second.x = 3.0;
        assert_eq!(holder.get_by_index::<pg_sys::Point>(p)?, Some(point(2.0)));
        drop(holder);
        assert_eq!(*second, pg_sys::Point { x: 3.0, y: -2.0 });
        Ok(())
    }
}
