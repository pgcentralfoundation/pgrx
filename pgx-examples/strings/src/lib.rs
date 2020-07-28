// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use pgx::*;

pg_module_magic!();

#[pg_extern]
fn return_static() -> &'static str {
    "This is a static string xxx"
}

#[pg_extern]
fn to_lowercase(input: &str) -> String {
    input.to_lowercase()
}

#[pg_extern]
fn substring(input: &str, start: i32, end: i32) -> &str {
    &input[start as usize..end as usize]
}

#[pg_extern]
fn append(mut input: String, extra: &str) -> String {
    input.push_str(extra);
    input.push('x');
    input
}

#[pg_extern]
fn split(input: &'static str, pattern: &str) -> Vec<&'static str> {
    input.split_terminator(pattern).into_iter().collect()
}

#[pg_extern]
fn split_set(
    input: &'static str,
    pattern: &'static str,
) -> impl std::iter::Iterator<Item = &'static str> {
    input.split_terminator(pattern).into_iter()
}

#[cfg(any(test, feature = "pg_test"))]
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
