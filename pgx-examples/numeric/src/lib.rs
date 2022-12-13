use pgx::prelude::*;

pg_module_magic!();

#[pg_extern]
fn add_numeric(a: Numeric<1000, 33>, b: Numeric<1000, 33>) -> Numeric<1000, 33> {
    (a + b).rescale().unwrap()
}

#[pg_extern]
fn random_numeric() -> AnyNumeric {
    rand::random::<i128>().into()
}

#[pg_extern]
fn numeric_from_string(s: &str) -> AnyNumeric {
    s.try_into().unwrap()
}

// select (((((5 * 5) - 2.234) / 4) % 19) + 99.42)::numeric(10, 3)
#[pg_extern]
fn math() -> Numeric<10, 3> {
    let mut n = AnyNumeric::from(5);

    n *= 5;
    n -= 2.234;
    n /= 4 as i128;
    n %= 19;
    n += 99.42;

    n.rescale().unwrap()
}

#[pg_extern]
fn forty_twooooooo() -> Numeric<1000, 33> {
    42.try_into().unwrap()
}
