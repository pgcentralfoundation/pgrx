// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use pgx::*;

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
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use crate::tests::enum_type_tests::Foo;
    use pgx::*;

    #[test]
    fn make_idea_happy() {}

    #[pg_test]
    fn test_foo_enum() {
        let result =
            Spi::get_one::<Foo>("SELECT take_foo_enum('One');").expect("failed to get SPI result");
        assert_eq!(Foo::Three, result);
    }
}
