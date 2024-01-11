use pgrx::prelude::*;

fn main() {}

#[pg_extern]
fn ereport_loglevel_must_terminate_with_semicolon() {
    ereport!(
        loglevel = PgLogLevel::LOG,
        errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;
        message = "...";
    );
}
