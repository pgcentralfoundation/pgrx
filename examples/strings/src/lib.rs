use pgx::*;

pg_module_magic!();

#[pg_extern]
fn return_static() -> &'static str {
    "This is a static string"
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
