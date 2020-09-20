## Schemas and Rust Modules

A `pgx`-based extension is created in the schema determined by the `CREATE EXTENSION` statement.
If unspecified, that schema is whatever the first schema in the user's `search_path` is, otherwise
it is the schema argument to `CREATE EXTENSION`.

In general, any `pgx` object (a function, operator, type, etc), regardless of the Rust source
file it is defined in, is created in that schema unless that object appears in a 
`mod modname { ... }` block.  In this case, `pgx` generates a top-level schema named the
same as the module, and creates the contained objects within that schema.    

Unlike Rust, which supports nested modules, Postgres only supports one-level of schemas,
although a Postgres session can have many schemas in its `search_path`.  As such, any
`mod modname { ... }` block containing `pgx` objects is hoisted to a top-level schema.

### `#[pg_extern]/#[pg_operator]` Functions and their Postgres `search_path`

When `pgx` generates the DDL for a function (`CREATE FUNCTION ...`), it uses uses the schema
it understands the function to belong in two different ways.

First off, if there's a `schema = foo` attribute in your extension `.control` file, the
function is created in that schema.  If there is no `schema = foo` attribute, then the
function is *not* schema-qualified, which indicates it'll be created in the schema
determined by the `CREATE EXTENSION` function.

Secondly, `pgx` applies a `search_path` to that function that limits that function's
search path to the schema of the extension.  If the function is defined in a 
`mod modname { ... }` block, then that schema is first in the search path, and the extension's
general schema is the section.

In general, this won't have an impact on your Rust code's usage of Postgres, expect through
`Spi` where a query wants to use some object outside of this fixed `search_path`, and
also with certain Postgres-internal object/name resolution functions that rely on the
`search_path`.  Note that `pg_catalog` is always implicitly included in the `search_path`.



