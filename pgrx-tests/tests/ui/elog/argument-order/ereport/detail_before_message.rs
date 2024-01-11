use pgrx::prelude::*;

fn main() {}

#[pg_extern]
fn ereport_detail_before_message_is_invalid() {
    ereport!(
        loglevel = PgLogLevel::LOG;
        errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;
        detail = "detail";
        message = "...";
    );
}
