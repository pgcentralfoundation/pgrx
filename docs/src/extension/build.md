# Building Extensions with PGRX
<!-- TODO: explain the build system more -->

## Guides

- [Cross Compile](./build/cross-compile.md)

## Installing

Some extensions require more or less configuration in order to install on a server.

### You Must Be Superuser

Essentially every extension made with `pgrx` is going to need to `CREATE FUNCTION`.
However, this may cause Postgres to issue a somewhat perplexing error:
```console
ERROR:  permission denied for language c
```

C code? In MY Rust? Well, it's more likely than you think, but this is not actually about C code.
Postgres thinks every language that compiles to machine code and can be dlopened and called is "C".
Rust does not disagree: your Rust functions, exposed by `#[pg_extern]`, will look something like
```rust
#[no_mangle]
pub unsafe extern "C" fn your_fn_wrapper(
    fcinfo: pg_sys::FunctionCallInfo
) -> pg_sys::Datum {
    std::panic::catch_unwind(|| {
        let args = { todo!("emit a bunch of code to unpack args here") };
        let result = your_fn(args);
        result
            .map(|r| r.into_datum())
            .unwrap_or_else(|e| panic!("Oh no! {}", e) )
    })
}
```

For most extensions which perform any "low-level" feats such as interacting directly with
indexes or tables, including creating new kinds at the behest of Postgres, or using shared memory,
this is unfixable. There is no "trusted language" that can do these, because a trusted language
is "trusted" in the sense that it has been defanged: it certainly cannot be trusted with anything
as sharp as "raw pointers into memory". You can only find a way to obtain superuser privileges.
This is easiest to do on a computer you already have root access to in general.

### When `shared_preload_libraries` is required

An extension may or may not care whether it is loaded before "anything else", during the time that
things like shared inter-process memory are being set up by Postgres. This requires it to be added
to the [`shared_preload_libraries` string in postgresql.conf][guc-shared-preload], which will be
found in your Postgres data directory, looking something like this:
```
#local_preload_libraries = ''
#session_preload_libraries = ''
#shared_preload_libraries = ''	# (change requires restart)
```

You will want to change it to this:

```
#local_preload_libraries = ''
#session_preload_libraries = ''
# change requires restart
shared_preload_libraries = '/path/to/compiled_library.extension'
```

This is necessary because after Postgres finishes starting up, it then forks, spawning new worker
processes to parallelize answering queries. If your extension is loaded after this, each process
will have a different view of its memory. This is fine for most extensions, but it prevents using
the "singleton" pattern, or interacting with shared memory to communicate with abackground workers.

[guc-shared-preload]: https://www.postgresql.org/docs/16/runtime-config-client.html#GUC-SHARED-PRELOAD-LIBRARIES
