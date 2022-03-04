// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use pgx::*;

pg_module_magic!();

#[pg_extern]
fn array_with_null_and_panic(input: Vec<Option<i32>>) -> i64 {
    let mut sum = 0 as i64;

    for i in input {
        sum += i.expect("NULL elements in input array are not supported") as i64;
    }

    sum
}

#[pg_extern]
fn cause_unwrap_panic() {
    let tmp: Option<i32> = None;
    tmp.unwrap();
}

#[pg_extern]
fn cause_pg_error() {
    unsafe {
        PgRelation::open_with_name("invalid table syntax").unwrap();
    }
}

#[pg_extern]
fn throw_rust_panic(message: &str) {
    panic!("{}", message);
}

#[pg_extern]
fn raise_pg_info(message: &str) {
    info!("{}", message);
}

#[pg_extern]
fn raise_pg_warning(message: &str) {
    warning!("{}", message);
}

#[pg_extern]
fn throw_pg_error(message: &str) {
    error!("{}", message);
}

#[pg_extern]
fn throw_pg_panic(message: &str) {
    PANIC!("{}", message);
}

#[pg_extern]
fn throw_pg_fatal(message: &str) {
    FATAL!("{}", message);
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_it() {
        // do testing here.
        //
        // #[pg_test] functions run *inside* Postgres and have access to all Postgres internals
        //
        // Normal #[test] functions do not
        //
        // In either case, they all run in parallel
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
