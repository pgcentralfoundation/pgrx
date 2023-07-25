# Memory Checking

For some background see the writeup in <https://github.com/pgcentralfoundation/pgrx/issues/1217>.

## Running with Memory Checking

### Valgrind

1. Install valgrind, headers, libs (on Fedora `sudo dnf install valgrind valgrind-devel valgrind-tools-devel` is enough).

2. Run `cargo pgrx init --valgrind`. The only major downside to using this as your primary pgrx installation is that its slow, but you can still run without valgrind.

3. Set `USE_VALGRIND` in the environment when running tests, for example `USE_VALGRIND=1 cargo test`. valgrind must be on the path for this to work. This is slow -- taking about 15 minutes on a very beefy cloud server, so manage your timing expectations accordingly.

### Sanitizers

Sanitizers can be enabled when building postgres, but not Postgres extensions (TODO: still figuring this out). These have a smaller runtime impact than use of valgrind, but unfortunately they also detect considerably less UB.

In general, the way to do this is to set `SANITIZER_FLAGS=-fsanitize=<sanitizer>` during `cargo pgrx init`. Note that this is incompatible with running under valgrind, although the `--valgrind` flag can still be used (it would have no benefit). For example:

1. Scudo+GWP-ASAN: `SANITIZER_FLAGS=-fsanitize=scudo cargo pgrx init`. This is generally recommended if you aren't going to run under valgrind, as the overhead is quite low and while the frequency of bug detection is similarly low, it is nonzero.

    Notably, unlike the rest of these, doing this for postgres will also apply to PGRX extensions (so long as they don't override the `#[global_allocator]`), since it's basically just setting up the allocator in a certain way.

2. Address sanitizer: `SANITIZER_FLAGS=-fsanitize=address cargo pgrx init`. This is more situational, since it can cause false-positives if the whole world is not built with ASAN enabled. Unfortunately, doing so is not possible in our case (TODO: still figuring this out).

3. Work on supporting other sanitizers, such as memory and UB sanitizer, is still TODO.

### Hardened Allocators

For basic usage of electric fence or scudo, `LD_PRELOAD=libefence.so cargo test` or `LD_PRELOAD=libscudo.so cargo test` (after installing the required library). However, for more advanced usage, see the documentation in the previous section about using Scudo.
