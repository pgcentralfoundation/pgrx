/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgrx::prelude::*;

#[derive(PostgresEnum, PartialEq, Debug)]
pub enum Foo {
    One,
    Two,
    Three,
}

#[pg_extern]
fn take_foo_enum(value: Foo) -> Foo {
    assert_eq!(value, Foo::One);

    Foo::Three
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use crate::tests::enum_type_tests::Foo;
    use pgrx::prelude::*;

    #[test]
    fn make_idea_happy() {}

    #[pg_test]
    fn test_foo_enum() {
        let result = Spi::get_one::<Foo>("SELECT take_foo_enum('One');");
        assert_eq!(Ok(Some(Foo::Three)), result);
    }
}
