use pgrx::prelude::*;

fn main() {}

#[pg_extern]
fn ereport_loglevel_is_required() {
    ereport!(
        errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;
        message = "...";
    );
}
