use pgx::*;

pg_module_magic!();

#[pg_extern]
fn my_numeric(n: Numeric<1000, 1>) -> Numeric<1000, 1> {
    n
}
