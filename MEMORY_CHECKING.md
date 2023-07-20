# Memory Checking

## Background

*(Skip this section if you just want to know how to run under Valgrind/ASAN/etc)*

For certain classes of errors, a test may return the correct result, without detecting a problem actual programs "in the large" may encounter. For example, writing and reading to freed memory may appear to work -- perhaps nobody has started to use that memory for anything else. Ensuring our tests fail in such cases is highly desirable. Thankfully, a large amount of research has gone into detecting such errors with minimal changes to the program under test.

1. "Dynamic" instrumentation tools, of which the main option for memory checking is Valgrind.
    - These are slow, often 10-50x slower than normal (unusable on production-sized workloads, but perhaps viable for tests), but provide high-quality detection and don't require rebuilding tested programs (although they can benefit from it, and Postgres supports such).
    - One major downside is they often require x86_64 Linux, and anything else is partially supported at best.
2. "Static" instrumentation tools, such as Address Sanitizer (ASAN), Memory Sanitizer (MSAN), and so on. These are like Valgrind but an order of magnitude faster and more portable. The main tradeoff is each individual one does not necessarily detect as many errors, and the program *must* be compiled with sanitizer support enabled.
    - For cases involving dynamic code loading (so, Postgres extensions), all involved objects must be linked against a single dynamic library for a sanitizer, not a different version. This is likely but needs research to make sure, for instance, the same LLVM version can be used to build Postgres and our Rust code, amongst other possible complications.
    - Without additional annotations, it is often not possible to detect many errors in programs which use pools like Postgres memory contexts. This applies to Valgrind as well, but Postgres already has annotations for Valgrind, not for sanitizers.
    - A final hurdle: sanitizers cannot necessarily handle `fork`, which Postgres uses to spawn processes. It's unclear if this will cause issues. Valgrind handles this if told to trace forked children.
3. Compiling Postgres with additional options when testing to aid in detecting such issues.
    - This is not limited to assertions, but there are flags for memory context checking, wiping freed memory, and so on. There are a lot of these, and we should start setting several additional ones in the cargo-pgrx-built postgres.
4. Using a hardened system allocator. [Scudo](https://llvm.org/docs/ScudoHardenedAllocator.html) is currently the most well-supported of these, but [the electric fence malloc](https://linux.die.net/man/3/efence) is the same basic idea. These replace malloc/free/etc with a version which catches some kinds of misuse.
    - This is very easy to integrate, and can be done by e.g. setting `LD_PRELOAD=libscudo.so`, without other changes (there's other options for integration as well, if `LD_PRELOAD` is not viable). It also has very low overhead -- `Scudo` is nearly as fast as system allocators, and faster for some workloads.
    - These are redundant if address sanitizer or Valgrind is in use, but useful when they are not. In our case, they mostly will catch memory errors in memory allocated by Rust, although extremely significant misuse of Postgres's memory will may be caught as well.
    - The downside is these do not reliably detect errors with anything close to a 100% rate. Scudo has a mode where it may catch *more* ASAN-style issues (this is called GWP-ASAN, although it does not have many of the drawbacks of ASAN), but even when that is enabled they still only catch some percentage of issues.

## Running with Memory Checking

It's possible to run our test suites under some of the above checkers.

### Valgrind

1. Install valgrind, headers, libs (on Fedora `sudo dnf install valgrind valgrind-devel valgrind-tools-devel` is enough).

2. Run `cargo pgrx init --valgrind`. The only major downside to using this as your primary pgrx installation is that its slow, but you can still run without valgrind.

3. Set `USE_VALGRIND` in the environment when running tests, for example `USE_VALGRIND=1 cargo test`. valgrind must be on the path for this to work. This is slow -- taking about 15 minutes on a very beefy cloud server, so manage your timing expectations accordingly.


