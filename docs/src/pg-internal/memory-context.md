# Memory Contexts

Postgres uses a set of "memory contexts" in order to manage memory and prevent leakage, despite
the fact that Postgres code may churn through tables with literally millions of rows. Most of the
memory contexts that an extension's code is likely to be invoked in are transient contexts that
will not outlive the current transaction. These memory contexts will be freed, including all of
their contents, at the end of that transaction. This means that allocations using memory contexts
will quickly be cleaned up, even in C extensions that don't have the power of Rust's compile-time
memory management. However, this is incompatible with certain assumptions Rust makes about safety,
thus making it tricky to correctly bind this code.

<!-- TODO: finish out `MemCx` drafts and provide alternatives to worrying about allocations -->

## What `palloc` calls to
In extension code, especially that written in C, you may notice calls to the following functions
for allocation and deallocation, instead of the usual `malloc` and `free`:

```c
typedef size_t Size;

extern void *palloc(Size size);
extern void *palloc0(Size size);
extern void *palloc_extended(Size size, int flags);

extern void pfree(void *pointer);
```

<!--
// Only in Postgres 16+
extern void *palloc_aligned(Size size, Size alignto, int flags);
-->

When combined with appropriate type definitions, the `palloc` family of functions are identical to
calling the following functions and passing the `CurrentMemoryContext` as the first argument:

```c
typedef struct MemoryContextData *MemoryContext;
#define PGDLLIMPORT
extern PGDLLIMPORT MemoryContext CurrentMemoryContext;

extern void *MemoryContextAlloc(MemoryContext context, Size size);
extern void *MemoryContextAllocZero(MemoryContext context, Size size);
extern void *MemoryContextAllocExtended(MemoryContext context,
                                        Size size, int flags);
```
<!--
// Only in Postgres 16+
extern void *MemoryContextAllocAligned(MemoryContext context,
                                       Size size, Size alignto, int flags);
-->

Notice that `pfree` only takes the pointer as an argument, effectively meaning every allocation
must know what context it belongs to in some way.

### `CurrentMemoryContext` makes `impl Deref` hard

<!-- TODO: this segment. -->

### Assigning lifetimes to `palloc` is hard

<!-- TODO: this segment. -->
