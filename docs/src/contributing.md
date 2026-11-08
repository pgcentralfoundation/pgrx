# Contributing

- [PGRX Internals](./contributing/pgrx-internal.md)
- [Releases](./contributing/release.md)

## Dev Environment Setup

System Requirements:
- Rustup (or equivalent toolchain manager like Nix). Users may be able to use distro toolchains, but you won't get far without a proper Rust toolchain manager.
- Otherwise, same as the user requirements.

If you want to be ready to open a PR, you will want to run
```bash
git clone --branch develop "https://github.com/pgcentralfoundation/pgrx"
cd pgrx
```
That will put you in a cloned repository with the *develop* branch opened,
which is the one you will be opening pull requests against in most cases.

After cloning the repository, mostly you can use similar flows as in the README.
However, if there are any differences in `cargo pgrx` since the last release, then
the first and most drastic difference in the developer environment vs. the
user environment is that you will have to run

```bash
cargo install cargo-pgrx --path ./cargo-pgrx \
    --force \
    --locked # This forces usage of the repo's lockfile
cargo pgrx init # This might take a while. Consider getting a drink.
```

## Pull Requests (PRs)

- Pull requests for new code or bugfixes should be submitted against develop
- All pull requests against develop will be squashed on merge
- Tests are *expected* to pass before merging
- PGRX tests PRs on rustfmt so please run `cargo fmt` before you submit
- Diffs in Cargo.lock should be checked in
- HOWEVER, diffs in the generated bindings in `pgrx-pg-sys/src/include/pg*.rs` should **not** be checked in (this is a release task)

### Small Diffs? Big PRs?

In general, it is better to land smaller diffs in pull requests, for making them easy to review.
However, the pgrx repo is no stranger to "big PRs". This is often because a smaller PR may be
unjustified or nonfunctional on its own.

Further, large cleanup diffs that merely move around code, format it, or apply lints should be
landed independently, to allow use of `.git-blame-ignore-revs`. Even these, however, are best if
they are restrained to a single crate, or a logically-connected set, if they involve altering
anything that could conceivably affect the code's functionality.

### Adding Dependencies

If a new crate dependency is required for a pull request, and it can't or should not be marked optional and behind some kind of feature flag, then it should have its reason for being used stated in the Cargo.toml it is added to. This can be "as a member of a category", in the case of e.g. error handling:

```toml
# error handling and logging
eyre = "0.6.8"
thiserror = "1.0"
tracing = "0.1.34"
tracing-error = "0.2.0"
```

It can be as notes for the individual dependencies:
```toml
once_cell = "1.10.0" # polyfill until std::lazy::OnceCell stabilizes
```

Or it can be both:

```toml
# exposed in public API
atomic-traits = "0.3.0" # PgAtomic and shmem init
bitflags = "1.3.2" # BackgroundWorker
bitvec = "1.0" # processing array nullbitmaps
```

You do not need exceptional justification notes in your PR to justify a new dependency as your code will, in most cases, self-evidently justify the use of the dependency. PGRX uses the normal Rust approach of using dependencies based on their ability to improve correctness and make features possible. It does not reimplement things already available in the Rust ecosystem unless the addition is trivial (do not add custom derives to save 5~10 lines of code in one site) or the ecosystem crates are not compatible with Postgres (unfortunately common for Postgres data types).

## Releases

See [Releases](./contributing/release.md) for how to cut and publish a release.

## Licensing

You agree that all code you submit in pull requests to https://github.com/pgcentralfoundation/pgrx/pulls
is offered according to the MIT License, thus may be freely relicensed and sublicensed,
and that you are satisfied with the existing copyright notice as of opening your PR.
