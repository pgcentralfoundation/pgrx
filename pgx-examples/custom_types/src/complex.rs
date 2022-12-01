/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use maplit::*;
use pgx::prelude::*;
use pgx::Array;
use serde::*;
use std::collections::HashMap;

#[derive(PostgresType, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Animals {
    names: Vec<String>,
    age_lookup: HashMap<i32, String>,
}

#[pg_extern]
fn known_animals() -> Animals {
    Animals {
        names: vec!["Sally".into(), "Brandy".into(), "anchovy".into()],
        age_lookup: hashmap! { 5 => "Sally".into(), 4 => "Brandy".into(), 3=> "anchovy".into()},
    }
}

#[pg_extern]
fn make_animals(animals: Array<String>, ages: Array<i32>) -> Animals {
    assert_eq!(animals.len(), ages.len(), "input array lengths not equal");

    let mut names = Vec::new();
    let mut age_lookup = HashMap::new();

    for (name, age) in animals.iter().zip(ages.iter()) {
        let name = name.expect("null names are not allowed");
        let age = age.expect("null ages are not allowed");

        names.push(name.clone());
        age_lookup.insert(age, name);
    }

    Animals { names, age_lookup }
}

#[pg_extern]
fn add_animal(mut animals: Animals, name: String, age: i32) -> Animals {
    animals.names.push(name.clone());
    animals.age_lookup.insert(age, name);
    animals
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::complex::{known_animals, Animals};
    use maplit::*;
    use pgx::prelude::*;

    #[cfg(not(feature = "no-schema-generation"))]
    #[pg_test]
    fn test_known_animals_via_spi() {
        let animals = Spi::get_one::<Animals>("SELECT known_animals();").unwrap();

        assert_eq!(animals, known_animals());

        assert_eq!(
            animals,
            Animals {
                names: vec!["Sally".into(), "Brandy".into(), "anchovy".into()],
                age_lookup: hashmap! { 5 => "Sally".into(), 4 => "Brandy".into(), 3=> "anchovy".into()},
            }
        )
    }
}
