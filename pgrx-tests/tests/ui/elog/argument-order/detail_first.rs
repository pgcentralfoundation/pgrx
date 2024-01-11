use pgrx::prelude::*;

fn main() {}

#[pg_extern]
fn detail_first_is_invalid() {
    log!(
        detail = "detail";
        message = "...";
    );
}
