/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::*;

pg_module_magic!();

// This is used by some, but not all, examples below.
const DOG_COMPOSITE_TYPE_IDENT: &str = "Dog";

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
    name = "create_dog",
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
            dogs: VariadicArray<pgx::composite_type!(DOG_COMPOSITE_TYPE_IDENT)>,
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
            for dog in dogs {
                let dog = dog.unwrap();
                let name = dog.get_by_name("name").unwrap().unwrap();
                names.push(name);
            }
            names
        }
    }

    mod vec {
        use super::*;

        #[pg_extern]
        fn sum_scritches_for_names(dogs: Option<Vec<pgx::composite_type!(DOG_COMPOSITE_TYPE_IDENT)>>) -> i32 {
            // Gets resolved to:
            let dogs: Option<Vec<PgHeapTuple<AllocatedByRust>>> = dogs;

            let dogs = dogs.unwrap();
            let mut sum_scritches = 0;
            for dog in dogs {
                let scritches: i32 = dog
                    .get_by_name("scritches")
                    .ok()
                    .unwrap_or_default()
                    .unwrap_or_default();
                sum_scritches += scritches;
            }
            sum_scritches
        }

        #[pg_extern]
        fn sum_scritches_for_names_default(
            dogs: pgx::default!(
                Vec<pgx::composite_type!("Dog")>,
                "ARRAY[ROW('Nami', 0)]::Dog[]"
            ),
        ) -> i32 {
            // Gets resolved to:
            let dogs: Vec<PgHeapTuple<AllocatedByRust>> = dogs;

            let mut sum_scritches = 0;
            for dog in dogs {
                let scritches: i32 = dog
                    .get_by_name("scritches")
                    .ok()
                    .unwrap_or_default()
                    .unwrap_or_default();
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
                let scritches: i32 = dog
                    .get_by_name("scritches")
                    .ok()
                    .unwrap_or_default()
                    .unwrap_or_default();
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
                let scritches: i32 = dog
                    .get_by_name("scritches")
                    .ok()
                    .unwrap_or_default()
                    .unwrap_or_default();
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
                let scritches: i32 = dog
                    .get_by_name("scritches")
                    .ok()
                    .unwrap_or_default()
                    .unwrap_or_default();
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
                let scritches: i32 = dog
                    .get_by_name("scritches")
                    .ok()
                    .unwrap_or_default()
                    .unwrap_or_default();
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
                let scritches: i32 = dog
                    .get_by_name("scritches")
                    .ok()
                    .unwrap_or_default()
                    .unwrap_or_default();
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
                let scritches: i32 = dog
                    .get_by_name("scritches")
                    .ok()
                    .unwrap_or_default()
                    .unwrap_or_default();
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
                let scritches: i32 = dog
                    .get_by_name("scritches")
                    .ok()
                    .unwrap_or_default()
                    .unwrap_or_default();
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
        fn create_dog(_name: String, _scritches: i32) -> pgx::composite_type!("Dog") {
            todo!()
        }

        #[pg_extern]
        fn scritch(
            maybe_dog: Option<::pgx::composite_type!("Dog")>,
        ) -> Option<pgx::composite_type!("Dog")> {
            // Gets resolved to:
            let maybe_dog: Option<PgHeapTuple<AllocatedByRust>> = maybe_dog;

            let maybe_dog = if let Some(mut dog) = maybe_dog {
                dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap())
                    .unwrap();
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

            dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap())
                .unwrap();

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
                dog.set_by_name("scritches", dog.get_by_name::<i32>("scritches").unwrap())
                    .unwrap();
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
) -> Option<Vec<::pgx::composite_type!("AnimalFriendshipEdge")>> {
    unimplemented!()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

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
        let retval = Spi::get_one::<Vec<String>>("
            SELECT gets_name_field_variadic(ROW('Nami', 1)::Dog, ROW('Brandy', 1)::Dog)
        ").expect("SQL select failed");
        assert_eq!(retval, vec!["Nami".to_string(), "Brandy".to_string()]);
    }

    #[pg_test]
    fn test_sum_scritches_for_names() {
        let retval = Spi::get_one::<i32>("
            SELECT sum_scritches_for_names(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
        ").expect("SQL select failed");
        assert_eq!(retval, 43);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_default() {
        let retval = Spi::get_one::<i32>("
            SELECT sum_scritches_for_names_default()
        ").expect("SQL select failed");
        assert_eq!(retval, 0);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_strict() {
        let retval = Spi::get_one::<i32>("
            SELECT sum_scritches_for_names_strict(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
        ").expect("SQL select failed");
        assert_eq!(retval, 43);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_strict_optional_items() {
        let retval = Spi::get_one::<i32>("
            SELECT sum_scritches_for_names_strict(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
        ").expect("SQL select failed");
        assert_eq!(retval, 43);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_default_optional_items() {
        let retval = Spi::get_one::<i32>("
            SELECT sum_scritches_for_names_default_optional_items()
        ").expect("SQL select failed");
        assert_eq!(retval, 0);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_optional_items() {
        let retval = Spi::get_one::<i32>("
            SELECT sum_scritches_for_names_optional_items(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
        ").expect("SQL select failed");
        assert_eq!(retval, 0);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_array() {
        let retval = Spi::get_one::<i32>("
            SELECT sum_scritches_for_names_array(ARRAY[ROW('Nami', 1), ROW('Brandy', 42)]::Dog[])
        ").expect("SQL select failed");
        assert_eq!(retval, 43);
    }

    #[pg_test]
    fn test_sum_scritches_for_names_array_default() {
        let retval = Spi::get_one::<i32>("
            SELECT sum_scritches_for_names_array_default()
        ").expect("SQL select failed");
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
        let retval = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>("
            SELECT create_dog('Nami', 1)
        ").expect("SQL select failed");
        assert_eq!(retval.get_by_name("name").unwrap(), Some("Nami"));
        assert_eq!(retval.get_by_name("scritches").unwrap(), Some(1));
    }

    #[pg_test]
    fn test_scritch() {
        let retval = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>("
            SELECT scritch(ROW('Nami', 1))
        ").expect("SQL select failed");
        assert_eq!(retval.get_by_name("name").unwrap(), Some("Nami"));
        assert_eq!(retval.get_by_name("scritches").unwrap(), Some(2));
    }

    #[pg_test]
    fn test_scritch_strict() {
        let retval = Spi::get_one::<PgHeapTuple<'_, AllocatedByRust>>("
            SELECT scritch_strict(ROW('Nami', 1))
        ").expect("SQL select failed");
        assert_eq!(retval.get_by_name("name").unwrap(), Some("Nami"));
        assert_eq!(retval.get_by_name("scritches").unwrap(), Some(2));
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
