# Working on pgrx

pgrx exposes PostgreSQL internals to Rust, so memory safety and sound public
interfaces take priority over convenience, performance, and passing tests.
Treat possible undefined behavior as a correctness defect, including when it
already exists in nearby code. Report nearby defects separately when fixing
them would expand the task's scope.

## Scope and workflow

- Read the implementation, callers, and relevant documentation before editing.
  Preserve unrelated work and keep each changeset focused on the requested task.
- Work on a branch based on `develop`; target pull requests at `develop`.
  Use a separate worktree when needed to preserve the user's checkout.
- Continue work already authorized by the user. Ask about material missing
  decisions, destructive actions, or broader changes outside the task; identify
  the exact restriction when work must pause.
- Before changing behavior, identify the current contract, the proposed design,
  and the invariants it must preserve. Choose whether to extend or replace the
  affected code, and explain any compatibility change.
- For growing inputs, account for time complexity, allocations, and avoidable
  work. For storage changes, also identify where bytes live, how readers and
  writers agree, and how many storage accesses an operation requires.
- pgrx does not want agent planning artifacts in the repository or changesets.
  Implementation plans, gate reports, task journals, and scratch notes are the
  user's work product; keep them outside the repository. Share relevant details
  in a follow-up PR comment when they help reviewers.
- After compiling, compare the implementation with those decisions. Review once
  for invariants that types could enforce and once for code that can be removed;
  repeat only while finding useful changes, up to four passes. For performance
  work, report predicted and measured growth, stored bytes and storage accesses
  when applicable, and net lines changed. Measure claims about existing hot paths.
- If delegating work, give each agent a bounded task, file ownership, permission
  limits, and required evidence. Review the combined result before publishing.

## Memory safety, FFI, and undefined behavior

Read [SAFETY.md](SAFETY.md) before changing unsafe code or its safe callers.
Also read the safety documentation and implementation of every affected FFI
boundary; a similar call elsewhere does not prove that a new use is sound.
Consult the PostgreSQL headers and source for the versions involved when their
contracts are unclear.

- Keep unsafe operations small and explicit. Document each unsafe function's
  caller obligations in a `# Safety` section and explain each unsafe block or
  impl with a `SAFETY:` comment that establishes its actual preconditions.
  An `unsafe` function still needs explicit unsafe blocks for unsafe operations.
- A safe API must remain sound for every input and call sequence its types allow.
  Enforce preconditions through types or runtime checks, or expose an unsafe API
  with a precise contract. Hiding a raw pointer behind a safe method does not
  discharge its safety requirements.
- Before creating a reference or slice, establish alignment, initialization,
  valid values, allocation bounds, provenance, lifetime, and aliasing rules.
  Non-null pointers alone prove none of these. Even an empty Rust slice requires
  a properly aligned, non-null pointer; handle nullable C buffers explicitly.
  Check lengths, offsets, integer conversions, and allocation arithmetic before
  pointer arithmetic or constructing slices.
- Track the owner and allocator of every allocation that crosses FFI. Match
  allocation and deallocation APIs, and establish who releases transferred
  ownership on success and failure. Do not adopt PostgreSQL allocations with
  `Box::from_raw`, `Vec::from_raw_parts`, or `CString::from_raw`.
- Tie borrowed PostgreSQL data to the resource that keeps it valid: its memory
  context, tuple, buffer pin, SPI connection, or other owner. Account for context
  reset, context deletion, transaction end, detoasting, and early returns.
  Copy into a suitable owner when data must live longer. Never manufacture
  `'static` or extend lifetimes with casts or `transmute` to satisfy the compiler.
- Keep C strings alive for as long as C may read them, and use bounded reads when
  termination is not guaranteed. Account for PostgreSQL encodings; a C string is
  not necessarily UTF-8. Packed or detoasted values may require different
  alignment and ownership handling from ordinary Rust values.
- Use the generated bindings and the correct C ABI. Verify layout, widths,
  signedness, nullability, and callback signatures for each affected PostgreSQL
  version and platform. `repr(C)` alone does not make arbitrary bytes a valid
  Rust value. Do not use zero initialization or `transmute` without proving
  validity, including for enums, references, and function pointers.
- Preserve pgrx's guards for PostgreSQL errors and Rust panics. Read
  [the FFI boundary contract](pgrx-pg-sys/src/submodules/ffi.rs) and
  [panic handling](pgrx-pg-sys/src/submodules/panic.rs) before changing callbacks
  or error paths. PostgreSQL `ERROR` can perform a nonlocal jump; ordinary Rust
  unwinding assumptions do not apply. A `pg_guard_ffi_boundary` closure must
  not panic or hold values needing destruction across that jump.
- Keep invariants valid before calling code that can raise an error or invoke a
  callback. Do not rely on `Drop` running to restore memory safety. Review
  [memory-context handling](pgrx/src/memcxt.rs) and
  [PgBox ownership](pgrx/src/pgbox.rs) before adding cleanup: freeing a context
  does not automatically run Rust destructors, and existing wrappers may already
  own the release operation.
- Call PostgreSQL APIs only from the permitted backend thread; a mutex does not
  make them safe to call from Rust worker threads. Preserve
  [thread checks](pgrx-pg-sys/src/submodules/thread_check.rs). Justify unsafe
  `Send` and `Sync` implementations, synchronization, and shared-memory lifetimes.
  Process-local pointers cannot serve as shared pointers between backends.
- Investigate suspected UB even if tests pass. Add coverage for the failure mode
  when practical, and explain the safety argument separately from test results.
  Use Miri for compatible pure Rust code and suitable sanitizers or Valgrind for
  native integration when they can expose the risk; their success is not a proof
  of soundness. If the contract remains unknown, report that gap before shipping.

## Professional, idiomatic Rust

- Follow the repository's edition, toolchain, formatting, naming, and error
  conventions. Read [Cargo.toml](Cargo.toml) and
  [rust-toolchain.toml](rust-toolchain.toml) instead of assuming current defaults.
- Make ownership and borrowing clear. Prefer borrowed inputs when ownership is
  unnecessary, and justify clones, allocations, reference counting, and interior
  mutability. Use enums and newtypes to express meaningful states and units.
- Keep modules cohesive and APIs small. Separate pure Rust logic from backend
  integration where it improves reasoning and testing. Introduce abstractions
  when they own an invariant or remove meaningful duplication.
- Implement standard traits when their contracts fit. Return useful error types
  and preserve error context; use the existing `thiserror` and error-reporting
  conventions. Handle expected failures with `Result` or `Option`, and justify
  `unwrap`, `expect`, and panics with a specific invariant.
- Prefer existing resource-owning types and RAII for ordinary Rust cleanup,
  subject to the PostgreSQL error-path restrictions above. Read existing `Drop`
  implementations before adding manual release operations.
- Keep format definitions and their canonical encode/decode paths together.
  Validate malformed input and arithmetic overflow before using decoded values.
  Choose hashers with the input's trust boundary in mind.
- Remove code made obsolete by the change. Avoid speculative helpers, unnecessary
  dependencies, unrelated formatting, blanket lint suppressions, and comments
  that merely repeat the code. Explain constraints and non-obvious choices.

## Validation and local databases

Choose one validation plan based on the affected behavior, safety invariants,
PostgreSQL versions, and feature combinations. Reuse passing evidence until a
relevant change invalidates it. Run the applicable checks defined in
[the CI workflow](.github/workflows/tests.yml), which agents may read but must
not modify. Documentation-only changes need reference, wording, and diff review.

- Use pure Rust tests for logic that does not require a backend. Put tests that
  call backend functions or require PostgreSQL state in the `#[pg_test]` harness,
  following the existing `tests` schema convention. Use compile-fail coverage
  when a safe API must reject an invalid lifetime or ownership pattern.
- Run backend tests with this checkout's `cargo-pgrx` and a matching PostgreSQL
  installation. See [the development instructions](README.md#hacking) and
  [cargo-pgrx's README](cargo-pgrx/README.md) for setup and commands. For example,
  with PG18 configured, the workspace command is
  `cargo test --all --features pg18 --no-default-features`; select the relevant
  configured major version for the change. This example omits additional feature
  combinations such as `cshim` and `proptest`; use the applicable CI commands for
  those. Check affected FFI against the actual target headers and server,
  including a new beta when changing beta support.
- Run `cargo fmt --all -- --check` for Rust changes and `git diff --check` before
  committing. Keep existing tests meaningful; do not skip a failing check or
  weaken an assertion to make the result pass.
- Use isolated test data and an isolated `PGRX_HOME` for database experiments.
  `cargo pgrx regress` can reuse a running instance and recreate a database;
  establish that both belong to the task before using it. Do not access or
  modify the user's databases without authorization, or delete user data to
  resolve a test or installation conflict. Clean up only resources you created.
- Generate regression expectations from actual test runs and inspect their
  diffs. Do not bless unexpected output or hand-edit expected files to hide a
  failure. For intermittent failures, inspect logs and resource limits, then
  reproduce in isolation; do not assume either an environment problem or a code
  defect without evidence. Investigate backend crashes using logs and a
  backtrace when available.

## AI-generated issues, pull requests, and commits

These rules apply to every issue description, PR description, and commit message
an AI agent writes for this repository, including revisions to existing text.

- Write in plain English. Use concrete names, short sentences, and technical
  terms only when needed to explain the change. Avoid promotional language,
  canned summaries, generated checklists, and narration of the agent's process.
- Keep PR descriptions and commit messages under eight sentences each, meaning
  at most seven. A short title and one or two sentences usually suffice. Do not
  use long compound sentences or bullet fragments to evade the limit.
- Do not add agent attribution lines, bot signatures, "generated by" footers,
  or AI `Co-authored-by` trailers to issue descriptions, PR descriptions, or
  commit messages.
- pgrx always squash merges and prefers the PR description as the final commit
  message. Write it to stand alone in git history: lead with the problem and
  resulting behavior, and explain the reason for the change and any material
  compatibility, safety, or migration concern. Keep it current when the diff
  changes. Put relevant follow-up explanations and supporting details in PR
  comments so the description stays focused on the final change.
- Do not append a routine "verification steps performed" section or similar
  testing footer to PR descriptions. Run appropriate checks anyway. Mention a
  test result only when it helps explain the change; disclose material failures
  or missing coverage briefly where they affect a claim.
- AI agents must not add, edit, rename, or delete CI workflows in their
  changesets, including anything under `.github/workflows/`. Do not bypass this
  restriction by moving workflow logic elsewhere or disabling checks. If a
  workflow needs changing, explain the need and leave the edit to a human.
- Before opening an issue or PR, look for an existing report or change. Use a
  specific, searchable title. Issues should distinguish observed behavior from
  suspected causes and provide a minimal reproducer, expected behavior, and
  relevant versions when available. Say when a report has not been reproduced.
- Keep diffs focused and reviewable. Link relevant issues and code, summarize
  behavioral changes, and leave unrelated cleanup for separate work. Do not
  include generated logs, private paths, credentials, or raw conversation text.
- Claim only work done and results observed. Do not invent
  benchmarks, test runs, reviewer approval, or maintainer agreement. The human
  submitting AI-assisted work remains responsible for understanding the diff
  and answering review questions; do not impersonate their review or approval.
- Respond to review with concrete changes or technical reasoning. Avoid repeated
  status comments, automatic acknowledgments, and pressure to merge. Publish
  issues, PRs, comments, and commits only within the user's authorized task.
