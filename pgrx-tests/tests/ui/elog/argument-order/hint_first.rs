use pgrx::prelude::*;

fn main() {}

#[pg_extern]
fn hint_first_is_invalid() {
    log!(
        hint = "hint";
        message = "...";
    );
}
