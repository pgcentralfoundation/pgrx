## Dev Environment Setup

System Requirements:
- Rustup (or equivalent toolchain manager like Nix). Users may be able to use distro toolchains, but you won't get far without a proper Rust toolchain manager.
- Otherwise, same as the user requirements.

If you want to be ready to open a PR, you will want to run
```bash
git clone --branch develop "https://github.com/tcdi/pgx"
cd pgx
```
That will put you in a cloned repository with the *develop* branch opened,
which is the one you will be opening pull requests against in most cases.

After cloning the repository, mostly you can use similar flows as in the README.
However, if there are any differences in `cargo pgx` since the last release, then
the first and most drastic difference in the developer environment vs. the user environment is that you will have to run
```bash
cargo install cargo-pgx --path ./cargo-pgx --force
cargo pgx init # This might take a while. Consider getting a drink.
```

## Pull Requests (PRs)

- Pull requests for new code or bugfixes should be submitted against develop
- All pull requests against develop will be squashed on merge
- Tests are *expected* to pass before merging
- PGX tests PRs on rustfmt so please run `cargo fmt` before you submit
- Diffs in Cargo.lock should be checked in
- HOWEVER, diffs in the bindgen in `pgx-pg-sys/src/pg*.rs` should **not** be checked in (this is a release task)

## Releases

On a new PGX release, *develop* will be merged to *master* via merge commit.
<!-- it's somewhat ambiguous whether we do this for stable or also "release candidate" releases -->

### Release Candidates AKA Betas
PGX prefers using `x.y.z-{alpha,beta}.n` format for naming release candidates,
starting at `alpha.0` if the new release candidate does not seem "feature complete",
or at `beta.0` if it is not expected to need new feature work. Remember that `beta` will supersede `alpha` in versions for users who don't pin a version.

Publishing PGX is somewhat fraught, as all the crates really are intended to be published together as a single unit. There's no way to do a "dry run" of publishing multiple crates. Thus, it may be a good idea, when going from `m.n.o` to `m.n.p`, to preferentially publish `m.n.p-beta.0` instead of `m.n.p`, even if you are reasonably confident that **nothing** will happen.

### Checklist
Do this *in order*:
- [ ] Inform other maintainers of intent to publish a release
- [ ] Assign an appropriate value to `NEW_RELEASE_VERSION`
- [ ] Draft release notes for `${NEW_RELEASE_VERSION}`
- [ ] Run `./update-versions.sh "${NEW_RELEASE_VERSION}"`
    - This will update the visible bindings of `pgx-pg-sys/src/pg*.rs`
    - The visible bindings are for reference, [docs][pgx@docs.rs], and tools
    - Actual users of the library rebuild the bindings from scratch
- [ ] Run `./upgrade-deps.sh`
- [ ] Push the resulting diffs to *develop*
- [ ] Run `./publish.sh` to push the new version to [pgx@crates.io]
    - If there was an error during publishing:
    - [ ] fix the error source
    - [ ] push any resulting diffs to *develop*
    - [ ] increment the patch version
    - [ ] try again

**Your work isn't done yet** just because it's on [crates.io], you also need to:
- [ ] Open a PR against *master* for the changes from *develop*
- [ ] *Switch* to using **merge commits** in this PR
- [ ] Merge
- [ ] Publish release notes
- [ ] Celebrate

## Licensing

You agree that all code you submit in pull requests to https://github.com/tcdi/pgx/pulls
is offered according to the MIT License, thus may be freely relicensed and sublicensed,
and that you are satisfied with the existing copyright notice as of opening your PR.

[crates.io]: https://crates.io
[pgx@crates.io]: https://crates.io/crates/pgxa
[pgx@docs.rs]: https://docs.rs/pgx/latest/pgx