/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashSet;

use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Item, ItemImpl};

use operators::{impl_postgres_eq, impl_postgres_hash, impl_postgres_ord};
use pgrx_sql_entity_graph::{
    parse_extern_attributes, CodeEnrichment, ExtensionSql, ExtensionSqlFile, ExternArgs,
    PgAggregate, PgExtern, PostgresEnum, PostgresType, Schema,
};

use crate::rewriter::PgGuardRewriter;

mod operators;
mod rewriter;

/// Declare a function as `#[pg_guard]` to indicate that it is called from a Postgres `extern "C"`
/// function so that Rust `panic!()`s (and Postgres `elog(ERROR)`s) will be properly handled by `pgrx`
#[proc_macro_attribute]
pub fn pg_guard(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // get a usable token stream
    let ast = parse_macro_input!(item as syn::Item);

    let rewriter = PgGuardRewriter::new();

    let res = match ast {
        // this is for processing the members of extern "C" { } blocks
        // functions inside the block get wrapped as public, top-level unsafe functions that are not "extern"
        Item::ForeignMod(block) => Ok(rewriter.extern_block(block)),

        // process top-level functions
        Item::Fn(func) => rewriter.item_fn_without_rewrite(func),
        unknown => Err(syn::Error::new(
            unknown.span(),
            "#[pg_guard] can only be applied to extern \"C\" blocks and top-level functions",
        )),
    };
    res.unwrap_or_else(|e| e.into_compile_error()).into()
}

/// `#[pg_test]` functions are test functions (akin to `#[test]`), but they run in-process inside
/// Postgres during `cargo pgrx test`.
#[proc_macro_attribute]
pub fn pg_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut stream = proc_macro2::TokenStream::new();
    let args = parse_extern_attributes(proc_macro2::TokenStream::from(attr.clone()));

    let mut expected_error = None;
    args.into_iter().for_each(|v| {
        if let ExternArgs::Error(message) = v {
            expected_error = Some(message)
        }
    });

    let ast = parse_macro_input!(item as syn::Item);

    match ast {
        Item::Fn(mut func) => {
            // Here we need to break out attributes into test and non-test attributes,
            // so the generated #[test] attributes are in the appropriate place.
            let mut test_attributes = Vec::new();
            let mut non_test_attributes = Vec::new();

            for attribute in func.attrs.iter() {
                if let Some(ident) = attribute.path.get_ident() {
                    let ident_str = ident.to_string();

                    if ident_str == "ignore" || ident_str == "should_panic" {
                        test_attributes.push(attribute.clone());
                    } else {
                        non_test_attributes.push(attribute.clone());
                    }
                } else {
                    non_test_attributes.push(attribute.clone());
                }
            }

            func.attrs = non_test_attributes;

            stream.extend(proc_macro2::TokenStream::from(pg_extern(
                attr,
                Item::Fn(func.clone()).to_token_stream().into(),
            )));

            let expected_error = match expected_error {
                Some(msg) => quote! {Some(#msg)},
                None => quote! {None},
            };

            let sql_funcname = func.sig.ident.to_string();
            let test_func_name =
                Ident::new(&format!("pg_{}", func.sig.ident.to_string()), func.span());

            let attributes = func.attrs;
            let mut att_stream = proc_macro2::TokenStream::new();

            for a in attributes.iter() {
                let as_str = a.tokens.to_string();
                att_stream.extend(quote! {
                    options.push(#as_str);
                });
            }

            stream.extend(quote! {
                #[test]
                #(#test_attributes)*
                fn #test_func_name() {
                    let mut options = Vec::new();
                    #att_stream

                    crate::pg_test::setup(options);
                    let res = pgrx_tests::run_test(#sql_funcname, #expected_error, crate::pg_test::postgresql_conf_options());
                    match res {
                        Ok(()) => (),
                        Err(e) => panic!("{:?}", e)
                    }
                }
            });
        }

        thing => {
            return syn::Error::new(
                thing.span(),
                "#[pg_test] can only be applied to top-level functions",
            )
            .to_compile_error()
            .into()
        }
    }

    stream.into()
}

/// Associated macro for `#[pg_test]` to provide context back to your test framework to indicate
/// that the test system is being initialized
#[proc_macro_attribute]
pub fn initialize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Declare a function as `#[pg_operator]` to indicate that it represents a Postgres operator
/// `cargo pgrx schema` will automatically generate the underlying SQL
#[proc_macro_attribute]
pub fn pg_operator(attr: TokenStream, item: TokenStream) -> TokenStream {
    pg_extern(attr, item)
}

/// Used with `#[pg_operator]`.  1 value which is the operator name itself
#[proc_macro_attribute]
pub fn opname(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Used with `#[pg_operator]`.  1 value which is the function name
#[proc_macro_attribute]
pub fn commutator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Used with `#[pg_operator]`.  1 value which is the function name
#[proc_macro_attribute]
pub fn negator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Used with `#[pg_operator]`.  1 value which is the function name
#[proc_macro_attribute]
pub fn restrict(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Used with `#[pg_operator]`.  1 value which is the function name
#[proc_macro_attribute]
pub fn join(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Used with `#[pg_operator]`.  no values
#[proc_macro_attribute]
pub fn hashes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Used with `#[pg_operator]`.  no values
#[proc_macro_attribute]
pub fn merges(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/**
Declare a Rust module and its contents to be in a schema.

The schema name will always be the `mod`'s identifier. So `mod flop` will create a `flop` schema.

If there is a schema inside a schema, the most specific schema is chosen.

In this example, the created `example` function is in the `dsl_filters` schema.

```rust,ignore
use pgrx::*;

#[pg_schema]
mod dsl {
    use pgrx::*;
    #[pg_schema]
    mod dsl_filters {
        use pgrx::*;
        #[pg_extern]
        fn example() { todo!() }
    }
}
```

File modules (like `mod name;`) aren't able to be supported due to [`rust/#54725`](https://github.com/rust-lang/rust/issues/54725).

*/
#[proc_macro_attribute]
pub fn pg_schema(_attr: TokenStream, input: TokenStream) -> TokenStream {
    fn wrapped(input: TokenStream) -> Result<TokenStream, syn::Error> {
        let pgrx_schema: Schema = syn::parse(input)?;
        Ok(pgrx_schema.to_token_stream().into())
    }

    match wrapped(input) {
        Ok(tokens) => tokens,
        Err(e) => {
            let msg = e.to_string();
            TokenStream::from(quote! {
              compile_error!(#msg);
            })
        }
    }
}

/**
Declare SQL to be included in generated extension script.

Accepts a String literal, a `name` attribute, and optionally others:

* `name = "item"`: Set the unique identifier to `"item"` for use in `requires` declarations.
* `requires = [item, item_two]`: References to other `name`s or Rust items which this SQL should be present after.
* `creates = [ Type(submod::Cust), Enum(Pre), Function(defined)]`: Communicates that this SQL block creates certain entities.
  Please note it **does not** create matching Rust types.
* `bootstrap` (**Unique**): Communicates that this is SQL intended to go before all other generated SQL.
* `finalize` (**Unique**): Communicates that this is SQL intended to go after all other generated SQL.

You can declare some SQL without any positioning information, meaning it can end up anywhere in the generated SQL:

```rust,ignore
use pgrx_macros::extension_sql;

extension_sql!(
    r#"
    -- SQL statements
    "#,
    name = "demo",
);
```

To cause the SQL to be output at the start of the generated SQL:

```rust,ignore
use pgrx_macros::extension_sql;

extension_sql!(
    r#"
    -- SQL statements
    "#,
    name = "demo",
    bootstrap,
);
```

To cause the SQL to be output at the end of the generated SQL:

```rust,ignore
use pgrx_macros::extension_sql;

extension_sql!(
    r#"
    -- SQL statements
    "#,
    name = "demo",
    finalize,
);
```

To declare the SQL dependent, or a dependency of, other items:

```rust,ignore
use pgrx_macros::extension_sql;

struct Treat;

mod dog_characteristics {
    enum DogAlignment {
        Good
    }
}

extension_sql!(r#"
    -- SQL statements
    "#,
    name = "named_one",
);

extension_sql!(r#"
    -- SQL statements
    "#,
    name = "demo",
    requires = [ "named_one", dog_characteristics::DogAlignment ],
);
```

To declare the SQL defines some entity (**Caution:** This is not recommended usage):

```rust,ignore
use pgrx::stringinfo::StringInfo;
use pgrx::*;
use pgrx_utils::get_named_capture;

#[derive(Debug)]
#[repr(C)]
struct Complex {
    x: f64,
    y: f64,
}

extension_sql!(r#"\
        CREATE TYPE complex;\
    "#,
    name = "create_complex_type",
    creates = [Type(Complex)],
);

#[pg_extern(immutable)]
fn complex_in(input: &core::ffi::CStr) -> PgBox<Complex> {
    todo!()
}

#[pg_extern(immutable)]
fn complex_out(complex: PgBox<Complex>) -> &'static core::ffi::CStr {
    todo!()
}

extension_sql!(r#"\
        CREATE TYPE complex (
            internallength = 16,
            input = complex_in,
            output = complex_out,
            alignment = double
        );\
    "#,
    name = "demo",
    requires = ["create_complex_type", complex_in, complex_out],
);

```
*/
#[proc_macro]
pub fn extension_sql(input: TokenStream) -> TokenStream {
    fn wrapped(input: TokenStream) -> Result<TokenStream, syn::Error> {
        let ext_sql: CodeEnrichment<ExtensionSql> = syn::parse(input)?;
        Ok(ext_sql.to_token_stream().into())
    }

    match wrapped(input) {
        Ok(tokens) => tokens,
        Err(e) => {
            let msg = e.to_string();
            TokenStream::from(quote! {
              compile_error!(#msg);
            })
        }
    }
}

/**
Declare SQL (from a file) to be included in generated extension script.

Accepts the same options as [`macro@extension_sql`]. `name` is automatically set to the file name (not the full path).

You can declare some SQL without any positioning information, meaning it can end up anywhere in the generated SQL:

```rust,ignore
use pgrx_macros::extension_sql_file;
extension_sql_file!(
    "../static/demo.sql",
);
```

To override the default name:

```rust,ignore
use pgrx_macros::extension_sql_file;

extension_sql_file!(
    "../static/demo.sql",
    name = "singular",
);
```

For all other options, and examples of them, see [`macro@extension_sql`].
*/
#[proc_macro]
pub fn extension_sql_file(input: TokenStream) -> TokenStream {
    fn wrapped(input: TokenStream) -> Result<TokenStream, syn::Error> {
        let ext_sql: CodeEnrichment<ExtensionSqlFile> = syn::parse(input)?;
        Ok(ext_sql.to_token_stream().into())
    }

    match wrapped(input) {
        Ok(tokens) => tokens,
        Err(e) => {
            let msg = e.to_string();
            TokenStream::from(quote! {
              compile_error!(#msg);
            })
        }
    }
}

/// Associated macro for `#[pg_extern]` or `#[macro@pg_operator]`.  Used to set the `SEARCH_PATH` option
/// on the `CREATE FUNCTION` statement.
#[proc_macro_attribute]
pub fn search_path(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/**
Declare a function as `#[pg_extern]` to indicate that it can be used by Postgres as a UDF.

Optionally accepts the following attributes:

* `immutable`: Corresponds to [`IMMUTABLE`](https://www.postgresql.org/docs/current/sql-createfunction.html).
* `strict`: Corresponds to [`STRICT`](https://www.postgresql.org/docs/current/sql-createfunction.html).
  + In most cases, `#[pg_extern]` can detect when no `Option<T>`s are used, and automatically set this.
* `stable`: Corresponds to [`STABLE`](https://www.postgresql.org/docs/current/sql-createfunction.html).
* `volatile`: Corresponds to [`VOLATILE`](https://www.postgresql.org/docs/current/sql-createfunction.html).
* `raw`: Corresponds to [`RAW`](https://www.postgresql.org/docs/current/sql-createfunction.html).
* `security_definer`: Corresponds to [`SECURITY DEFINER`](https://www.postgresql.org/docs/current/sql-createfunction.html)
* `security_invoker`: Corresponds to [`SECURITY INVOKER`](https://www.postgresql.org/docs/current/sql-createfunction.html)
* `parallel_safe`: Corresponds to [`PARALLEL SAFE`](https://www.postgresql.org/docs/current/sql-createfunction.html).
* `parallel_unsafe`: Corresponds to [`PARALLEL UNSAFE`](https://www.postgresql.org/docs/current/sql-createfunction.html).
* `parallel_restricted`: Corresponds to [`PARALLEL RESTRICTED`](https://www.postgresql.org/docs/current/sql-createfunction.html).
* `no_guard`: Do not use `#[pg_guard]` with the function.
* `sql`: Same arguments as [`#[pgrx(sql = ..)]`](macro@pgrx).
* `name`: Specifies target function name. Defaults to Rust function name.

Functions can accept and return any type which `pgrx` supports. `pgrx` supports many PostgreSQL types by default.
New types can be defined via [`macro@PostgresType`] or [`macro@PostgresEnum`].


Without any arguments or returns:
```rust,ignore
use pgrx::*;
#[pg_extern]
fn foo() { todo!() }
```

# Arguments
It's possible to pass even complex arguments:

```rust,ignore
use pgrx::*;
#[pg_extern]
fn boop(
    a: i32,
    b: Option<i32>,
    c: Vec<i32>,
    d: Option<Vec<Option<i32>>>
) { todo!() }
```

It's possible to set argument defaults, set by PostgreSQL when the function is invoked:

```rust,ignore
use pgrx::*;
#[pg_extern]
fn boop(a: default!(i32, 11111)) { todo!() }
#[pg_extern]
fn doop(
    a: default!(Vec<Option<&str>>, "ARRAY[]::text[]"),
    b: default!(String, "'note the inner quotes!'")
) { todo!() }
```

The `default!()` macro may only be used in argument position.

It accepts 2 arguments:

* A type
* A `bool`, numeric, or SQL string to represent the default. `"NULL"` is a possible value, as is `"'string'"`

**If the default SQL entity created by the extension:** ensure it is added to `requires` as a dependency:

```rust,ignore
use pgrx::*;
#[pg_extern]
fn default_value() -> i32 { todo!() }

#[pg_extern(
    requires = [ default_value, ],
)]
fn do_it(
    a: default!(i32, "default_value()"),
) { todo!() }
```

# Returns

It's possible to return even complex values, as well:

```rust,ignore
use pgrx::*;
#[pg_extern]
fn boop() -> i32 { todo!() }
#[pg_extern]
fn doop() -> Option<i32> { todo!() }
#[pg_extern]
fn swoop() -> Option<Vec<Option<i32>>> { todo!() }
#[pg_extern]
fn floop() -> (i32, i32) { todo!() }
```

Like in PostgreSQL, it's possible to return tables using iterators and the `name!()` macro:

```rust,ignore
use pgrx::*;
#[pg_extern]
fn floop<'a>() -> TableIterator<'a, (name!(a, i32), name!(b, i32))> {
    TableIterator::new(None.into_iter())
}

#[pg_extern]
fn singular_floop() -> (name!(a, i32), name!(b, i32)) {
    todo!()
}
```

The `name!()` macro may only be used in return position inside the `T` of a `TableIterator<'a, T>`.

It accepts 2 arguments:

* A name, such as `example`
* A type

# Special Cases

`pg_sys::Oid` is a special cased type alias, in order to use it as an argument or return it must be
passed with it's full module path (`pg_sys::Oid`) in order to be resolved.

```rust,ignore
use pgrx::*;

#[pg_extern]
fn example_arg(animals: pg_sys::Oid) {
    todo!()
}

#[pg_extern]
fn example_return() -> pg_sys::Oid {
    todo!()
}
```

*/
#[proc_macro_attribute]
pub fn pg_extern(attr: TokenStream, item: TokenStream) -> TokenStream {
    fn wrapped(attr: TokenStream, item: TokenStream) -> Result<TokenStream, syn::Error> {
        let pg_extern_item = PgExtern::new(attr.clone().into(), item.clone().into())?;
        Ok(pg_extern_item.to_token_stream().into())
    }

    match wrapped(attr, item) {
        Ok(tokens) => tokens,
        Err(e) => {
            let msg = e.to_string();
            TokenStream::from(quote! {
              compile_error!(#msg);
            })
        }
    }
}

/**
Generate necessary bindings for using the enum with PostgreSQL.

```rust,ignore
# use pgrx_pg_sys as pg_sys;
use pgrx::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, PostgresEnum)]
enum DogNames {
    Nami,
    Brandy,
}
```

*/
#[proc_macro_derive(PostgresEnum, attributes(requires, pgrx))]
pub fn postgres_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    impl_postgres_enum(ast).unwrap_or_else(|e| e.to_compile_error()).into()
}

fn impl_postgres_enum(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut stream = proc_macro2::TokenStream::new();
    let sql_graph_entity_ast = ast.clone();
    let enum_ident = &ast.ident;
    let enum_name = enum_ident.to_string();

    // validate that we're only operating on an enum
    let enum_data = match ast.data {
        Data::Enum(e) => e,
        _ => {
            return Err(syn::Error::new(
                ast.span(),
                "#[derive(PostgresEnum)] can only be applied to enums",
            ))
        }
    };

    let mut from_datum = proc_macro2::TokenStream::new();
    let mut into_datum = proc_macro2::TokenStream::new();

    for d in enum_data.variants.clone() {
        let label_ident = &d.ident;
        let label_string = label_ident.to_string();

        from_datum.extend(quote! { #label_string => Some(#enum_ident::#label_ident), });
        into_datum.extend(quote! { #enum_ident::#label_ident => Some(::pgrx::enum_helper::lookup_enum_by_label(#enum_name, #label_string)), });
    }

    stream.extend(quote! {
        impl ::pgrx::datum::FromDatum for #enum_ident {
            #[inline]
            unsafe fn from_polymorphic_datum(datum: ::pgrx::pg_sys::Datum, is_null: bool, typeoid: ::pgrx::pg_sys::Oid) -> Option<#enum_ident> {
                if is_null {
                    None
                } else {
                    // GREPME: non-primitive cast u64 as Oid
                    let (name, _, _) = ::pgrx::enum_helper::lookup_enum_by_oid(unsafe { ::pgrx::pg_sys::Oid::from_datum(datum, is_null)? } );
                    match name.as_str() {
                        #from_datum
                        _ => panic!("invalid enum value: {}", name)
                    }
                }
            }
        }

        impl ::pgrx::datum::IntoDatum for #enum_ident {
            #[inline]
            fn into_datum(self) -> Option<::pgrx::pg_sys::Datum> {
                match self {
                    #into_datum
                }
            }

            fn type_oid() -> ::pgrx::pg_sys::Oid {
                ::pgrx::wrappers::regtypein(#enum_name)
            }

        }
    });

    let sql_graph_entity_item = PostgresEnum::from_derive_input(sql_graph_entity_ast)?;
    sql_graph_entity_item.to_tokens(&mut stream);

    Ok(stream)
}

/**
Generate necessary bindings for using the type with PostgreSQL.

```rust,ignore
# use pgrx_pg_sys as pg_sys;
use pgrx::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, PostgresType)]
struct Dog {
    treats_received: i64,
    pets_gotten: i64,
}

#[derive(Debug, Serialize, Deserialize, PostgresType)]
enum Animal {
    Dog(Dog),
}
```

Optionally accepts the following attributes:

* `inoutfuncs(some_in_fn, some_out_fn)`: Define custom in/out functions for the type.
* `pgvarlena_inoutfuncs(some_in_fn, some_out_fn)`: Define custom in/out functions for the `PgVarlena` of this type.
* `sql`: Same arguments as [`#[pgrx(sql = ..)]`](macro@pgrx).
*/
#[proc_macro_derive(PostgresType, attributes(inoutfuncs, pgvarlena_inoutfuncs, requires, pgrx))]
pub fn postgres_type(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    impl_postgres_type(ast).unwrap_or_else(|e| e.to_compile_error()).into()
}

fn impl_postgres_type(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;
    let generics = &ast.generics;
    let has_lifetimes = generics.lifetimes().next();
    let funcname_in = Ident::new(&format!("{}_in", name).to_lowercase(), name.span());
    let funcname_out = Ident::new(&format!("{}_out", name).to_lowercase(), name.span());
    let mut args = parse_postgres_type_args(&ast.attrs);
    let mut stream = proc_macro2::TokenStream::new();

    // validate that we're only operating on a struct
    match ast.data {
        Data::Struct(_) => { /* this is okay */ }
        Data::Enum(_) => {
            // this is okay and if there's an attempt to implement PostgresEnum,
            // it will result in compile-time error of conflicting implementation
            // of traits (IntoDatum, inout, etc.)
        }
        _ => {
            return Err(syn::Error::new(
                ast.span(),
                "#[derive(PostgresType)] can only be applied to structs or enums",
            ))
        }
    }

    if args.is_empty() {
        // assume the user wants us to implement the InOutFuncs
        args.insert(PostgresTypeAttribute::Default);
    }

    let lifetime = match has_lifetimes {
        Some(lifetime) => quote! {#lifetime},
        None => quote! {'static},
    };

    // all #[derive(PostgresType)] need to implement that trait
    stream.extend(quote! {
        impl #generics ::pgrx::PostgresType for #name #generics { }
    });

    // and if we don't have custom inout/funcs, we use the JsonInOutFuncs trait
    // which implements _in and _out #[pg_extern] functions that just return the type itself
    if args.contains(&PostgresTypeAttribute::Default) {
        let inout_generics = if has_lifetimes.is_some() {
            quote! {#generics}
        } else {
            quote! {<'_>}
        };

        stream.extend(quote! {
            impl #generics ::pgrx::inoutfuncs::JsonInOutFuncs #inout_generics for #name #generics {}

            #[doc(hidden)]
            #[::pgrx::pgrx_macros::pg_extern(immutable,parallel_safe)]
            pub fn #funcname_in #generics(input: Option<&#lifetime ::core::ffi::CStr>) -> Option<#name #generics> {
                input.map_or_else(|| {
                    for m in <#name as ::pgrx::inoutfuncs::JsonInOutFuncs>::NULL_ERROR_MESSAGE {
                        ::pgrx::pg_sys::error!("{}", m);
                    }
                    None
                }, |i| Some(<#name as ::pgrx::inoutfuncs::JsonInOutFuncs>::input(i)))
            }

            #[doc(hidden)]
            #[::pgrx::pgrx_macros::pg_extern(immutable,parallel_safe)]
            pub fn #funcname_out #generics(input: #name #generics) -> &#lifetime ::core::ffi::CStr {
                let mut buffer = ::pgrx::stringinfo::StringInfo::new();
                ::pgrx::inoutfuncs::JsonInOutFuncs::output(&input, &mut buffer);
                buffer.into()
            }

        });
    } else if args.contains(&PostgresTypeAttribute::InOutFuncs) {
        // otherwise if it's InOutFuncs our _in/_out functions use an owned type instance
        stream.extend(quote! {
            #[doc(hidden)]
            #[::pgrx::pgrx_macros::pg_extern(immutable,parallel_safe)]
            pub fn #funcname_in #generics(input: Option<&#lifetime ::core::ffi::CStr>) -> Option<#name #generics> {
                input.map_or_else(|| {
                    for m in <#name as ::pgrx::inoutfuncs::InOutFuncs>::NULL_ERROR_MESSAGE {
                        ::pgrx::pg_sys::error!("{}", m);
                    }
                    None
                }, |i| Some(<#name as ::pgrx::inoutfuncs::InOutFuncs>::input(i)))
            }

            #[doc(hidden)]
            #[::pgrx::pgrx_macros::pg_extern(immutable,parallel_safe)]
            pub fn #funcname_out #generics(input: #name #generics) -> &#lifetime ::core::ffi::CStr {
                let mut buffer = ::pgrx::stringinfo::StringInfo::new();
                ::pgrx::inoutfuncs::InOutFuncs::output(&input, &mut buffer);
                buffer.into()
            }
        });
    } else if args.contains(&PostgresTypeAttribute::PgVarlenaInOutFuncs) {
        // otherwise if it's PgVarlenaInOutFuncs our _in/_out functions use a PgVarlena
        stream.extend(quote! {
            #[doc(hidden)]
            #[::pgrx::pgrx_macros::pg_extern(immutable,parallel_safe)]
            pub fn #funcname_in #generics(input: Option<&#lifetime ::core::ffi::CStr>) -> Option<::pgrx::datum::PgVarlena<#name #generics>> {
                input.map_or_else(|| {
                    for m in <#name as ::pgrx::inoutfuncs::PgVarlenaInOutFuncs>::NULL_ERROR_MESSAGE {
                        ::pgrx::pg_sys::error!("{}", m);
                    }
                    None
                }, |i| Some(<#name as ::pgrx::inoutfuncs::PgVarlenaInOutFuncs>::input(i)))
            }

            #[doc(hidden)]
            #[::pgrx::pgrx_macros::pg_extern(immutable,parallel_safe)]
            pub fn #funcname_out #generics(input: ::pgrx::datum::PgVarlena<#name #generics>) -> &#lifetime ::core::ffi::CStr {
                let mut buffer = ::pgrx::stringinfo::StringInfo::new();
                ::pgrx::inoutfuncs::PgVarlenaInOutFuncs::output(&*input, &mut buffer);
                buffer.into()
            }
        });
    }

    let sql_graph_entity_item = PostgresType::from_derive_input(ast)?;
    sql_graph_entity_item.to_tokens(&mut stream);

    Ok(stream)
}

/// Derives the `GucEnum` trait, so that normal Rust enums can be used as a GUC.
#[proc_macro_derive(PostgresGucEnum, attributes(hidden))]
pub fn postgres_guc_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    impl_guc_enum(ast).unwrap_or_else(|e| e.to_compile_error()).into()
}

fn impl_guc_enum(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut stream = proc_macro2::TokenStream::new();

    // validate that we're only operating on an enum
    let enum_data = match ast.data {
        Data::Enum(e) => e,
        _ => {
            return Err(syn::Error::new(
                ast.span(),
                "#[derive(PostgresGucEnum)] can only be applied to enums",
            ))
        }
    };
    let enum_name = ast.ident;
    let enum_len = enum_data.variants.len();

    let mut from_match_arms = proc_macro2::TokenStream::new();
    for (idx, e) in enum_data.variants.iter().enumerate() {
        let label = &e.ident;
        let idx = idx as i32;
        from_match_arms.extend(quote! { #idx => #enum_name::#label, })
    }
    from_match_arms.extend(quote! { _ => panic!("Unrecognized ordinal ")});

    let mut ordinal_match_arms = proc_macro2::TokenStream::new();
    for (idx, e) in enum_data.variants.iter().enumerate() {
        let label = &e.ident;
        let idx = idx as i32;
        ordinal_match_arms.extend(quote! { #enum_name::#label => #idx, });
    }

    let mut build_array_body = proc_macro2::TokenStream::new();
    for (idx, e) in enum_data.variants.iter().enumerate() {
        let label = e.ident.to_string();
        let mut hidden = false;

        for att in e.attrs.iter() {
            let att = quote! {#att}.to_string();
            if att == "# [hidden]" {
                hidden = true;
                break;
            }
        }

        build_array_body.extend(quote! {
            ::pgrx::pgbox::PgBox::<_, ::pgrx::pgbox::AllocatedByPostgres>::with(&mut slice[#idx], |v| {
                v.name = ::pgrx::memcxt::PgMemoryContexts::TopMemoryContext.pstrdup(#label);
                v.val = #idx as i32;
                v.hidden = #hidden;
            });
        });
    }

    stream.extend(quote! {
        impl ::pgrx::guc::GucEnum<#enum_name> for #enum_name {
            fn from_ordinal(ordinal: i32) -> #enum_name {
                match ordinal {
                    #from_match_arms
                }
            }

            fn to_ordinal(&self) -> i32 {
                match *self {
                    #ordinal_match_arms
                }
            }

            unsafe fn config_matrix(&self) -> *const ::pgrx::pg_sys::config_enum_entry {
                let slice = ::pgrx::memcxt::PgMemoryContexts::TopMemoryContext.palloc0_slice::<::pgrx::pg_sys::config_enum_entry>(#enum_len + 1usize);

                #build_array_body

                slice.as_ptr()
            }
        }
    });

    Ok(stream)
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum PostgresTypeAttribute {
    InOutFuncs,
    PgVarlenaInOutFuncs,
    Default,
}

fn parse_postgres_type_args(attributes: &[Attribute]) -> HashSet<PostgresTypeAttribute> {
    let mut categorized_attributes = HashSet::new();

    for a in attributes {
        let path = &a.path;
        let path = quote! {#path}.to_string();
        match path.as_str() {
            "inoutfuncs" => {
                categorized_attributes.insert(PostgresTypeAttribute::InOutFuncs);
            }

            "pgvarlena_inoutfuncs" => {
                categorized_attributes.insert(PostgresTypeAttribute::PgVarlenaInOutFuncs);
            }

            _ => {
                // we can just ignore attributes we don't understand
            }
        };
    }

    categorized_attributes
}

/**
Generate necessary code using the type in operators like `==` and `!=`.

```rust,ignore
# use pgrx_pg_sys as pg_sys;
use pgrx::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, PostgresEnum, PartialEq, Eq, PostgresEq)]
enum DogNames {
    Nami,
    Brandy,
}
```
Optionally accepts the following attributes:

* `sql`: Same arguments as [`#[pgrx(sql = ..)]`](macro@pgrx).
*/
#[proc_macro_derive(PostgresEq, attributes(pgrx))]
pub fn postgres_eq(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_postgres_eq(ast).unwrap_or_else(syn::Error::into_compile_error).into()
}

/**
Generate necessary code using the type in operators like `>`, `<`, `<=`, and `>=`.

```rust,ignore
# use pgrx_pg_sys as pg_sys;
use pgrx::*;
use serde::{Deserialize, Serialize};
#[derive(
    Debug, Serialize, Deserialize, PartialEq, Eq,
     PartialOrd, Ord, PostgresEnum, PostgresOrd
)]
enum DogNames {
    Nami,
    Brandy,
}
```
Optionally accepts the following attributes:

* `sql`: Same arguments as [`#[pgrx(sql = ..)]`](macro@pgrx).
*/
#[proc_macro_derive(PostgresOrd, attributes(pgrx))]
pub fn postgres_ord(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_postgres_ord(ast).unwrap_or_else(syn::Error::into_compile_error).into()
}

/**
Generate necessary code for stable hashing the type so it can be used with `USING hash` indexes.

```rust,ignore
# use pgrx_pg_sys as pg_sys;
use pgrx::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PostgresEnum, PostgresHash)]
enum DogNames {
    Nami,
    Brandy,
}
```
Optionally accepts the following attributes:

* `sql`: Same arguments as [`#[pgrx(sql = ..)]`](macro@pgrx).
*/
#[proc_macro_derive(PostgresHash, attributes(pgrx))]
pub fn postgres_hash(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_postgres_hash(ast).unwrap_or_else(syn::Error::into_compile_error).into()
}

/**
Declare a `pgrx::Aggregate` implementation on a type as able to used by Postgres as an aggregate.

Functions inside the `impl` may use the [`#[pgrx]`](macro@pgrx) attribute.
*/
#[proc_macro_attribute]
pub fn pg_aggregate(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // We don't care about `_attr` as we can find it in the `ItemMod`.
    fn wrapped(item_impl: ItemImpl) -> Result<TokenStream, syn::Error> {
        let sql_graph_entity_item = PgAggregate::new(item_impl.into())?;

        Ok(sql_graph_entity_item.to_token_stream().into())
    }

    let parsed_base = parse_macro_input!(item as syn::ItemImpl);
    match wrapped(parsed_base) {
        Ok(tokens) => tokens,
        Err(e) => {
            let msg = e.to_string();
            TokenStream::from(quote! {
              compile_error!(#msg);
            })
        }
    }
}

/**
A helper attribute for various contexts.

## Usage with [`#[pg_aggregate]`](macro@pg_aggregate).

It can be decorated on functions inside a [`#[pg_aggregate]`](macro@pg_aggregate) implementation.
In this position, it takes the same args as [`#[pg_extern]`](macro@pg_extern), and those args have the same effect.

## Usage for configuring SQL generation

This attribute can be used to control the behavior of the SQL generator on a decorated item,
e.g. `#[pgrx(sql = false)]`

Currently `sql` can be provided one of the following:

* Disable SQL generation with `#[pgrx(sql = false)]`
* Call custom SQL generator function with `#[pgrx(sql = path::to_function)]`
* Render a specific fragment of SQL with a string `#[pgrx(sql = "CREATE FUNCTION ...")]`

*/
#[proc_macro_attribute]
pub fn pgrx(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/**
Create a [PostgreSQL trigger function](https://www.postgresql.org/docs/current/plpgsql-trigger.html)

Review the `pgrx::trigger_support::PgTrigger` documentation for use.

 */
#[proc_macro_attribute]
pub fn pg_trigger(attrs: TokenStream, input: TokenStream) -> TokenStream {
    fn wrapped(attrs: TokenStream, input: TokenStream) -> Result<TokenStream, syn::Error> {
        use pgrx_sql_entity_graph::{PgTrigger, PgTriggerAttribute};
        use syn::parse::Parser;
        use syn::punctuated::Punctuated;
        use syn::Token;

        let attributes =
            Punctuated::<PgTriggerAttribute, Token![,]>::parse_terminated.parse(attrs)?;
        let item_fn: syn::ItemFn = syn::parse(input)?;
        let trigger_item = PgTrigger::new(item_fn, attributes)?;
        let trigger_tokens = trigger_item.to_token_stream();

        Ok(trigger_tokens.into())
    }

    match wrapped(attrs, input) {
        Ok(tokens) => tokens,
        Err(e) => {
            let msg = e.to_string();
            TokenStream::from(quote! {
              compile_error!(#msg);
            })
        }
    }
}
