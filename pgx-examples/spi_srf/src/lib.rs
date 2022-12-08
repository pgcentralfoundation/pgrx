/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::prelude::*;
use pgx::{IntoDatum, SpiTupleTable};

pgx::pg_module_magic!();

extension_sql!(
    r#"   

CREATE TABLE dog_daycare (
    dog_name varchar(256),
    dog_age int,
    dog_breed varchar(256)
);

INSERT INTO dog_daycare(dog_name, dog_age, dog_breed) VALUES ('Fido', 3, 'Labrador');
INSERT INTO dog_daycare(dog_name, dog_age, dog_breed) VALUES ('Spot', 5, 'Poodle');
INSERT INTO dog_daycare(dog_name, dog_age, dog_breed) VALUES ('Rover', 7, 'Golden Retriever');
INSERT INTO dog_daycare(dog_name, dog_age, dog_breed) VALUES ('Snoopy', 9, 'Beagle');
INSERT INTO dog_daycare(dog_name, dog_age, dog_breed) VALUES ('Lassie', 11, 'Collie');
INSERT INTO dog_daycare(dog_name, dog_age, dog_breed) VALUES ('Scooby', 13, 'Great Dane');
INSERT INTO dog_daycare(dog_name, dog_age, dog_breed) VALUES ('Moomba', 15, 'Labrador');


"#,
    name = "create_dog_daycare_example_table",
);

#[pg_extern]
fn calculate_human_years() -> TableIterator<
    'static,
    (name!(dog_name, String), name!(dog_age, i32), name!(dog_breed, String), name!(human_age, i32)),
> {
    /*
        This function is a simple example of using SPI to return a set of rows
        from a query. This query will return the same rows as the table, but
        with an additional column that is the dog's age in human years.
    */
    let query = "SELECT * FROM spi_srf.dog_daycare;";

    let results = Spi::connect(|client| {
        let tup_table: SpiTupleTable = client.select(query, None, None);
        Ok::<_, ()>(tup_table.map(|row| {
            let dog_name: String = row["dog_name"].value().unwrap();
            let dog_age: i32 = row["dog_age"].value().unwrap();
            let dog_breed: String = row["dog_breed"].value().unwrap();
            let human_age: i32 = dog_age * 7;
            (dog_name, dog_age, dog_breed, human_age)
        }))
    })
    .unwrap();

    TableIterator::new(results)
}

#[pg_extern]
fn filter_by_breed(
    breed: &str,
) -> TableIterator<
    'static,
    (
        name!(dog_name, Option<String>),
        name!(dog_age, Option<i32>),
        name!(dog_breed, Option<String>),
    ),
> {
    /*
        This function is a simple example of using SPI to return a set of rows
        from a query. This query will return the records for the given breed.
    */

    let query = "SELECT * FROM spi_srf.dog_daycare WHERE dog_breed = $1;";
    let args = vec![(PgBuiltInOids::TEXTOID.oid(), breed.into_datum())];

    let results =
        Spi::connect(|client| {
            let tup_table: SpiTupleTable = client.select(query, None, Some(args));

            Ok::<_, ()>(tup_table.map(|row| {
                (row["dog_name"].value(), row["dog_age"].value(), row["dog_breed"].value())
            }))
        })
        .unwrap();

    TableIterator::new(results)
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::calculate_human_years;
    use pgx::prelude::*;

    #[pg_test]
    fn test_calculate_human_years() {
        let mut results: Vec<(String, i32, String, i32)> = Vec::new();

        results.push(("Fido".to_string(), 3, "Labrador".to_string(), 21));
        results.push(("Spot".to_string(), 5, "Poodle".to_string(), 35));
        results.push(("Rover".to_string(), 7, "Golden Retriever".to_string(), 49));
        results.push(("Snoopy".to_string(), 9, "Beagle".to_string(), 63));
        results.push(("Lassie".to_string(), 11, "Collie".to_string(), 77));
        results.push(("Scooby".to_string(), 13, "Great Dane".to_string(), 91));
        results.push(("Moomba".to_string(), 15, "Labrador".to_string(), 105));
        let func_results = calculate_human_years();

        for (expected, actual) in results.iter().zip(func_results) {
            assert_eq!(expected, &actual);
        }
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
