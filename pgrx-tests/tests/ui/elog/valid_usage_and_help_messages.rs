use pgrx::prelude::*;

fn main() {}

// none of these should fail to compile

#[pg_extern]
fn simple_syntax_works() {
    log!("...");
    log!("...",);
    log!("{}", "...");
}

#[pg_extern]
fn ereport_simple_syntax_works() {
    ereport!(LOG, PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, "...");
    ereport!(LOG, PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION, "...", "detail", "hint");
}

#[pg_extern]
fn alternative_syntax_works() {
    log!(
        message = "...";
    );
    log!(
        message = "...";
        detail = "detail";
    );
    log!(
        message = "...";
        detail = "detail";
        hint = "hint";
    );
    log!(
        message = "{}", "...";
    );
    log!(
        message = "{}", "...";
        detail = "{}", "detail";
    );
    log!(
        message = "{}", "...";
        hint = "{}", "hint";
    );
    log!(
        message = "{}", "...";
        detail = "{}", "detail";
        hint = "{}", "hint";
    );
}

#[pg_extern]
fn ereport_alternative_syntax_shorthand_works() {
    ereport! {
        loglevel = LOG;
        errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;
        message = "...";
    }
    ereport! {
        loglevel = LOG;
        errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;
        message = "...";
        detail = "detail";
    }
    ereport! {
        loglevel = LOG;
        errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;
        message = "...";
        hint = "hint";
    }
    ereport! {
        loglevel = LOG;
        errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;
        message = "...";
        detail = "detail";
        hint = "hint";
    }
}

#[pg_extern]
fn ereport_alternative_syntax_longhand_works() {
    ereport! {
        loglevel = PgLogLevel::LOG;
        errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;
        message = "...";
    }
    ereport! {
        loglevel = PgLogLevel::LOG;
        errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;
        message = "...";
        detail = "detail";
    }
    ereport! {
        loglevel = PgLogLevel::LOG;
        errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;
        message = "...";
        hint = "hint";
    }
    ereport! {
        loglevel = PgLogLevel::LOG;
        errcode = PgSqlErrorCode::ERRCODE_SUCCESSFUL_COMPLETION;
        message = "...";
        detail = "detail";
        hint = "hint";
    }
}

#[pg_extern]
fn help_messages() {
    log!();
    info!();
    notice!();
    warning!();
    error!();
    debug1!();
    debug2!();
    debug3!();
    debug4!();
    debug5!();
    ereport!();
}
