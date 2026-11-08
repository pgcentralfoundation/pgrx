# Releases

Tell the other maintainers before you start a release.

Run `prepare-release.sh` on `x86_64-unknown-linux-gnu`, the docs.rs target in
`pgrx-pg-sys/Cargo.toml`. It regenerates the committed bindings and the C macro
documentation snapshot for that target, and it fails unless every supported
PostgreSQL version is configured.

```shell
PGRX_HOME=/tmp/pgrx-release ./prepare-release.sh NEW_VERSION_NUMBER
```

- go make a PR to `develop` on GitHub
- start "draft new release" on GitHub to ask it to "Generate release notes".  Make sure to choose the `develop` branch to get the full set of changes.: https://github.com/pgcentralfoundation/pgrx/releases/new
- paste them into the PR you made above
- edit them as best as you can while channeling @workingjubilee's spirit
- request a review
- do a squash merge into develop
- create the actual release on GitHub, tagging the `develop` branch with "${NEW_VERSION}", using the release notes you made in your PR

```shell
git switch develop
git pull origin develop
./publish.sh
```

Before publishing, `publish.sh` type-checks pgrx and pgrx-pg-sys as docs.rs builds them
for `x86_64-unknown-linux-gnu`, for every PostgreSQL version. On another host, this needs that target's standard
library for both the default and nightly toolchains, and a C compiler for it, because
cee-scape compiles C. For example, on macOS run
`rustup target add x86_64-unknown-linux-gnu` and
`rustup +nightly target add x86_64-unknown-linux-gnu`, then set
`CC_x86_64_unknown_linux_gnu=clang` and
`CFLAGS_x86_64_unknown_linux_gnu=--sysroot=<an x86_64 Linux sysroot>`.

If `publish.sh` fails partway, some crates may already be on crates.io, and a
published version can't be replaced. Fix the cause on `develop`, then release the
next patch version.

## Release candidates

PGRX prefers using `x.y.z-{alpha,beta}.n` format for naming release candidates,
starting at `alpha.0` if the new release candidate does not seem "feature complete",
or at `beta.0` if it is not expected to need new feature work. Remember that `beta` will supersede `alpha` in versions for users who don't pin a version.

Publishing PGRX is somewhat fraught, as all the crates really are intended to be published together as a single unit. There's no way to do a "dry run" of publishing multiple crates. Thus, it may be a good idea, when going from `m.n.o` to `m.n.p`, to preferentially publish `m.n.p-beta.0` instead of `m.n.p`, even if you are reasonably confident that **nothing** will happen.
