pub(crate) fn cargo() -> std::process::Command {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    std::process::Command::new(cargo)
}

pub(crate) fn rustc() -> std::process::Command {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    std::process::Command::new(rustc)
}

/// Set some environment variables for use downstream (in `pgx-test` for
/// example). Does nothing if already set.
pub(crate) fn initialize() {
    match (std::env::var_os("CARGO_PGX"), std::env::current_exe()) {
        (None, Ok(path)) => {
            std::env::set_var("CARGO_PGX", path);
            // TODO: Should we set `CARGO_PGX_{CARGO,RUSTC}` to `RUSTC`/`CARGO`
            // if unset, then prefer those? The issue with `RUSTC`/`CARGO` vars
            // is that they are unset if something invokes e.g. `cargo`
            // directly... This is probably eventually something we'll need, but
            // let's wait until that happens.
        }
        (Some(_), Ok(_)) => {
            // For now I guess we should just hope they're the same.
            // Canonicalizing here's tricky and not guaranteed to behave
            // right... although we could consider calling back into ourselves
            // so something that blindly invokes `cargo-pgx` instead of
            // `CARGO_PGX` will do the right thing.
            //
            // In either case if we ever get to the macos-linker-shim work this
            // will have to be slightly firmed up (if `cargo-pgx` is still going
            // to act as the linker shim.)
        }
        //  bad but not much we can do.
        (_, Err(_)) => {}
    }
}
