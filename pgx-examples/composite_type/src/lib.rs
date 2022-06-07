/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::*;

pg_module_magic!();

extension_sql!(
    r#"
CREATE TYPE Dog AS (
    name TEXT,
    scritches INT
);
"#,
    name = "create_dog",
    bootstrap
);

#[pg_extern]
fn gets_name_field(dog: Option<pgx::composite_type!("Dog")>) -> Option<&str> {
    // Gets resolved to:
    let dog: Option<PgHeapTuple<AllocatedByRust>> = dog;

    dog?.get_by_name("name").ok()?
}

#[pg_extern]
fn gets_name_field_variadic(dogs: pgx::variadic!(pgx::composite_type!("Dog"))) -> Vec<&str> {
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
fn gets_name_field_default(
    dog: default!(pgx::composite_type!("Dog"), "ROW('Nami', 0)::Dog"),
) -> &str {
    // Gets resolved to:
    let dog: PgHeapTuple<AllocatedByRust> = dog;

    dog.get_by_name("name").unwrap().unwrap()
}

#[pg_extern]
fn gets_name_field_default_variadic(
    dogs: default!(
        variadic!(pgx::composite_type!("Dog")),
        "ARRAY[ROW('Nami', 0)]::Dog[]"
    ),
) -> Vec<&str> {
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
fn gets_name_field_strict(dog: pgx::composite_type!("Dog")) -> &str {
    // Gets resolved to:
    let dog: PgHeapTuple<AllocatedByRust> = dog;

    dog.get_by_name("name").unwrap().unwrap()
}

#[pg_extern]
fn gets_name_field_strict_variadic(dogs: pgx::variadic!(pgx::composite_type!("Dog"))) -> Vec<&str> {
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
fn sum_scritches_for_names_strict_default(
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
fn sum_scritches_for_names_strict_default_optional_items(
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
fn sum_scritches_for_names(dogs: Option<Vec<pgx::composite_type!("Dog")>>) -> i32 {
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
    fn test_gets_name_field_default() {
        let retval = Spi::get_one::<&str>(
            "
            SELECT gets_name_field_default()
        ",
        )
        .expect("SQL select failed");
        assert_eq!(retval, "Nami");
    }

    // #[pg_test]
    // fn test_sum_scritches_for_names() {
    //     let retval = Spi::get_one::<i32>("
    //         SELECT sum_scritches_for_names(ARRAY(ROW('Nami', 1)::Dog, ROW('Brandy', 1)::Dog))
    //     ").expect("SQL select failed");
    //     assert_eq!(retval, 2);
    // }
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
