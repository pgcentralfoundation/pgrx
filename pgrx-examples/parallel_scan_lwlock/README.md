## Postgres Dynamic Shared Memory in Parallel Foreign Scans and LWLock Support

Important:
> Extensions that use shared memory **must** be loaded via `postgresql.conf`'s
>`shared_preload_libraries` configuration setting.

The example in [src/lib.rs](src/lib.rs) implements a parallel scan implementation for a "generator" foreign table type
that yields numbers from an integer counter, guarded by a dynamically allocated LWLock. It is meant to illustrate how to
set up a lock for foreign scans, keeping the shared memory handling as simple as possible. Check out the `shmem` example
project for more insights on shared memory handling in `pgrx` extensions.
