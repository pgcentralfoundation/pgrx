use pgrx::prelude::*;

fn main() {}

#[pg_extern]
fn ereport_errcode_is_required() {
    ereport!(
        loglevel = PgLogLevel::LOG;
        message = "...";
    );
}
