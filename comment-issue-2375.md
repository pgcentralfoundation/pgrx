To add more context from the ParadeDB side: we have been working to improve extension upgrade and migration workflows for a while.

We already use `pg-schema-diff` in CI ([test-pg_search-schema.yml](https://github.com/paradedb/paradedb/blob/fea3e58c0317f0da355ee2b851f5e86a0a232896/.github/workflows/test-pg_search-schema.yml#L117-L209)) to diff the base branch schema against the pull request schema and generate a suggested SQL upgrade diff.

However, we found that diffing alone still needs a file management workflow to handle pull request concurrency, branch cherry-picking, and local development upgrades. In your `Option 1`, you mention `--append=sql/unreleased.sql`. In practice, having multiple concurrent pull requests touch a monolithic `unreleased.sql` or `ext--0.25--0.26.sql` file causes merge conflicts and makes it difficult to cherry-pick bugfixes from `main` to stable release branches (e.g. `0.25.x`).

I opened #2379 proposing first-class support for a fragment-based migration workflow in `cargo-pgrx` inspired by ORM migration systems:

1. In-Flight Migration Fragments (`sql/unreleased/`): Pull requests add standalone fragments (e.g. `sql/unreleased/<PR>.<description>.sql`) declaring dependencies (`-- depends-on:`). The output of `pg-schema-diff` can be placed directly into these fragments, and discrete fragment files cherry-pick cleanly across branches without version conflicts.
2. Ephemeral Upgrade Path during Local Dev (`cargo pgrx install`): `cargo pgrx install` assembles unreleased fragments directly into Postgres's `$SHAREDIR/extension/` and updates `default_version` in the installed `.control` file. Developers run `ALTER EXTENSION ... UPDATE;` in their local dev database without touching the git working tree.
3. Release Assembly (e.g. `cargo pgrx migrate assemble`): At release time, fragments are topologically sorted, concatenated into the final versioned upgrade script, and consumed, while `cargo pgrx package` ensures no unreleased fragments were left behind.

`pg-schema-diff` and migration fragments are probably complementary: `pg-schema-diff` generates the suggested SQL diff, while fragments provide the version-agnostic container that enables clean cherry-picking, local dev upgrades, and automated release assembly.
