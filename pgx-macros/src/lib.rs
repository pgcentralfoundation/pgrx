// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

extern crate proc_macro;

mod operators;
mod rewriter;

use crate::operators::{impl_postgres_eq, impl_postgres_hash, impl_postgres_ord};
use pgx_utils::*;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2, Punct, Spacing};
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use rewriter::*;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::{parenthesized, parse_macro_input, Attribute, Data, DeriveInput, Item, ItemFn, Type};
use syn::Token;
use syn::parse::{ParseStream, Parse, Parser};
use syn::punctuated::Punctuated;

/// Declare a function as `#[pg_guard]` to indcate that it is called from a Postgres `extern "C"`
/// function so that Rust `panic!()`s (and Postgres `elog(ERROR)`s) will be properly handled by `pgx`
#[proc_macro_attribute]
pub fn pg_guard(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // get a usable token stream
    let ast = parse_macro_input!(item as syn::Item);

    let rewriter = PgGuardRewriter::new();

    match ast {
        // this is for processing the members of extern "C" { } blocks
        // functions inside the block get wrapped as public, top-level unsafe functions that are not "extern"
        Item::ForeignMod(block) => rewriter.extern_block(block).into(),

        // process top-level functions
        // these functions get wrapped as public extern "C" functions with #[no_mangle] so they
        // can also be called from C code
        Item::Fn(func) => rewriter.item_fn(func, false, false, false).0.into(),
        _ => {
            panic!("#[pg_guard] can only be applied to extern \"C\" blocks and top-level functions")
        }
    }
}

/// `#[pg_test]` functions are test functions (akin to `#[test]`), but they run in-process inside
/// Postgres during `cargo pgx test`.
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

    stream.extend(proc_macro2::TokenStream::from(pg_extern(
        attr,
        item.clone(),
    )));

    let expected_error = match expected_error {
        Some(msg) => quote! {Some(#msg)},
        None => quote! {None},
    };

    let ast = parse_macro_input!(item as syn::Item);
    match ast {
        Item::Fn(func) => {
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
                fn #test_func_name() {
                    let mut options = Vec::new();
                    #att_stream

                    crate::pg_test::setup(options);
                    pgx_tests::run_test(#sql_funcname, #expected_error, crate::pg_test::postgresql_conf_options())
                }
            });
        }

        _ => panic!("#[pg_test] can only be applied to top-level functions"),
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
/// `cargo pgx schema` will automatically generate the underlying SQL
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

/// Associated macro for `#[pg_extern] or `#[pg_operator]`.  Used to set the `SEARCH_PATH` option
/// on the `CREATE FUNCTION` statement.
#[proc_macro_attribute]
pub fn search_path(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Declare a function as `#[pg_extern]` to indicate that it can be used by Postgres as a UDF
#[proc_macro_attribute]
pub fn pg_extern(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_extern_attributes(proc_macro2::TokenStream::from(attr));

    let ast = parse_macro_input!(item as syn::Item);
    match ast {
        Item::Fn(func) => rewrite_item_fn(func, args).into(),
        _ => panic!("#[pg_extern] can only be applied to top-level functions"),
    }
}

#[derive(Debug)]
struct DefaultMacro {
    ty: syn::Type,
    comma: Token![,],
    expr: syn::Lit,
}

impl Parse for DefaultMacro {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(DefaultMacro {
            ty: input.parse()?,
            comma: input.parse()?,
            expr: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct SearchPath {
    at_start: Option<syn::token::At>,
    dollar: Option<syn::token::Dollar>,
    path: syn::Ident,
    at_end: Option<syn::token::At>,
}

impl Parse for SearchPath {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(SearchPath {
            at_start: input.parse()?,
            dollar: input.parse()?,
            path: input.parse()?,
            at_end: input.parse()?,
        })
    }
}

impl ToTokens for SearchPath {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let at_start = self.at_start;
        let dollar = self.dollar;
        let path = &self.path;
        let at_end = self.at_end;

        let quoted = quote! {
            #at_start#dollar#path#at_end
        };

        quoted.to_string().to_tokens(tokens);
    }
}

#[derive(Debug)]
struct SearchPathList {
    fields: Punctuated<SearchPath, Token![,]>,
}

impl Parse for SearchPathList {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(SearchPathList {
            fields: input.parse_terminated(SearchPath::parse).expect(&format!("Got {}", input)),
        })
    }
}

impl ToTokens for SearchPathList {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.fields.to_tokens(tokens)
    }
}

fn rewrite_item_fn(mut func: ItemFn, extern_args: HashSet<ExternArgs>) -> proc_macro2::TokenStream {
    let is_raw = extern_args.contains(&ExternArgs::Raw);
    let no_guard = extern_args.contains(&ExternArgs::NoGuard);

    let finfo_name = syn::Ident::new(
        &format!("pg_finfo_{}_wrapper", func.sig.ident),
        Span::call_site(),
    );

    // use the PgGuardRewriter to go ahead and wrap the function here, rather than applying
    // a #[pg_guard] macro to the original function.  This is necessary so that compiler
    // errors/warnings indicate the proper line numbers
    let rewriter = PgGuardRewriter::new();

    // make the function 'extern "C"' because this is for the #[pg_extern] macro
    func.sig.abi = Some(syn::parse_str("extern \"C\"").unwrap());
    let func_span = func.span();
    let (rewritten_func, need_wrapper) = rewriter.item_fn(func.clone(), true, is_raw, no_guard);

    let search_path_attr = func.attrs.into_iter().find(|f| {
        f.path.segments.first().map(|f| {
            f.ident == Ident::new("search_path", Span::call_site())
        }).unwrap_or_default()
    });
    let search_path = search_path_attr.and_then(|attr| {
        Some(attr.parse_args::<SearchPathList>().unwrap())
    });
    let search_path_iter = search_path.iter();

    let sig = func.sig;
    let ident = sig.ident;

    let mut fn_pat = vec![];
    let mut fn_ty = vec![];
    let mut fn_default = vec![];
    sig.inputs.iter().flat_map(|input| {
        match input {
            syn::FnArg::Typed(pat) => {
                let default = match pat.ty.as_ref() {
                    syn::Type::Macro(macro_pat) => {
                        let mac = &macro_pat.mac;
                        let out: DefaultMacro = mac.parse_body().expect(&*format!("{:?}", mac));
                        Some(out.expr)
                    },
                    _ => None,
                };

                Some((pat.pat.clone(), pat.ty.clone(), default))
            },
            _ => None,
        }
    }).for_each(|(pat, ty, default)| {
        fn_pat.push(pat);
        fn_ty.push(ty);
        fn_default.push(default);
    });

    let fn_return = match sig.output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ty) => match *ty {
            // TODO: Handle this!
            syn::Type::ImplTrait(_) => None,
            ty => Some(ty),
        }
    };
    let fn_return_iter = fn_return.iter();
    let extern_args_iter = extern_args.into_iter();

    let inv = quote! {
        pgx::inventory::submit! {
            use core::any::TypeId;
            let inputs = vec![#(
                crate::PgxExternInputs {
                    pattern: stringify!(#fn_pat),
                    ty_id: TypeId::of::<#fn_ty>(),
                    ty_name: core::any::type_name::<#fn_ty>(),
                    default: stringify!(#fn_default),
                }
            ),*];
            crate::PgxExtern {
                name: stringify!(#ident),
                module_path: core::module_path!(),
                pg_extern_args: vec![#(pgx_utils::ExternArgs::#extern_args_iter),*].into_iter().collect(),
                search_path: None#( .unwrap_or(Some(vec![#search_path_iter])) )*,
                fn_args: inputs,
                fn_return: None#( .unwrap_or(Some((TypeId::of::<#fn_return_iter>(), core::any::type_name::<#fn_return_iter>()))) )*,
            }
        }
    };

    if need_wrapper {
        quote_spanned! {func_span=>
            #[no_mangle]
            pub extern "C" fn #finfo_name() -> &'static pg_sys::Pg_finfo_record {
                const V1_API: pg_sys::Pg_finfo_record = pg_sys::Pg_finfo_record { api_version: 1 };
                &V1_API
            }

            #inv

            #rewritten_func
        }
    } else {
        quote_spanned! {func_span=>
            #inv

            #rewritten_func
        }
    }
}

#[proc_macro_derive(PostgresEnum)]
pub fn postgres_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    impl_postgres_enum(ast).into()
}

fn impl_postgres_enum(ast: DeriveInput) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();
    let enum_ident = ast.ident;
    let enum_name = enum_ident.to_string();

    // validate that we're only operating on an enum
    let enum_data = match ast.data {
        Data::Enum(e) => e,
        _ => panic!("#[derive(PostgresEnum)] can only be applied to enums"),
    };

    let mut from_datum = proc_macro2::TokenStream::new();
    let mut into_datum = proc_macro2::TokenStream::new();

    for d in enum_data.variants {
        let label_ident = &d.ident;
        let label_string = label_ident.to_string();

        from_datum.extend(quote! { #label_string => Some(#enum_ident::#label_ident), });
        into_datum.extend(quote! { #enum_ident::#label_ident => Some(pgx::lookup_enum_by_label(#enum_name, #label_string)), });
    }

    stream.extend(quote! {
        impl pgx::FromDatum for #enum_ident {
            #[inline]
            unsafe fn from_datum(datum: pgx::pg_sys::Datum, is_null: bool, typeoid: pgx::pg_sys::Oid) -> Option<#enum_ident> {
                if is_null {
                    None
                } else {
                    let (name, _, _) = pgx::lookup_enum_by_oid(datum as pgx::pg_sys::Oid);
                    match name.as_str() {
                        #from_datum
                        _ => panic!("invalid enum value: {}", name)
                    }
                }
            }
        }

        impl pgx::IntoDatum for #enum_ident {
            #[inline]
            fn into_datum(self) -> Option<pgx::pg_sys::Datum> {
                match self {
                    #into_datum
                }
            }

            fn type_oid() -> pg_sys::Oid {
                pgx::regtypein(#enum_name)
            }

        }
    });

    stream
}

#[proc_macro_derive(PostgresType, attributes(inoutfuncs, pgvarlena_inoutfuncs))]
pub fn postgres_type(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    impl_postgres_type(ast).into()
}

fn impl_postgres_type(ast: DeriveInput) -> proc_macro2::TokenStream {
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
        _ => panic!("#[derive(PostgresType)] can only be applied to structs"),
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
        impl #generics pgx::PostgresType for #name #generics { }
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
            impl #generics JsonInOutFuncs #inout_generics for #name #generics {}

            #[pg_extern(immutable,parallel_safe)]
            pub fn #funcname_in #generics(input: &#lifetime std::ffi::CStr) -> #name #generics {
                #name::input(input)
            }

            #[pg_extern(immutable,parallel_safe)]
            pub fn #funcname_out #generics(input: #name #generics) -> &#lifetime std::ffi::CStr {
                let mut buffer = StringInfo::new();
                input.output(&mut buffer);
                buffer.into()
            }

        });
    } else if args.contains(&PostgresTypeAttribute::InOutFuncs) {
        // otherwise if it's InOutFuncs our _in/_out functions use an owned type instance
        stream.extend(quote! {
            #[pg_extern(immutable,parallel_safe)]
            pub fn #funcname_in #generics(input: &#lifetime std::ffi::CStr) -> #name #generics {
                #name::input(input)
            }

            #[pg_extern(immutable,parallel_safe)]
            pub fn #funcname_out #generics(input: #name #generics) -> &#lifetime std::ffi::CStr {
                let mut buffer = StringInfo::new();
                input.output(&mut buffer);
                buffer.into()
            }
        });
    } else if args.contains(&PostgresTypeAttribute::PgVarlenaInOutFuncs) {
        // otherwise if it's PgVarlenaInOutFuncs our _in/_out functions use a PgVarlena
        stream.extend(quote! {
            #[pg_extern(immutable,parallel_safe)]
            pub fn #funcname_in #generics(input: &#lifetime std::ffi::CStr) -> pgx::PgVarlena<#name #generics> {
                #name::input(input)
            }

            #[pg_extern(immutable,parallel_safe)]
            pub fn #funcname_out #generics(input: pgx::PgVarlena<#name #generics>) -> &#lifetime std::ffi::CStr {
                let mut buffer = StringInfo::new();
                input.output(&mut buffer);
                buffer.into()
            }
        });
    }

    stream
}

#[proc_macro_derive(PostgresGucEnum, attributes(hidden))]
pub fn postgres_guc_enum(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    impl_guc_enum(ast).into()
}

fn impl_guc_enum(ast: DeriveInput) -> proc_macro2::TokenStream {
    let mut stream = proc_macro2::TokenStream::new();

    // validate that we're only operating on an enum
    let enum_data = match ast.data {
        Data::Enum(e) => e,
        _ => panic!("#[derive(PostgresGucEnum)] can only be applied to enums"),
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
            pgx::PgBox::with(&mut slice[#idx], |v| {
                v.name = pgx::PgMemoryContexts::TopMemoryContext.pstrdup(#label);
                v.val = #idx as i32;
                v.hidden = #hidden;
            });
        });
    }

    stream.extend(quote! {
        impl pgx::GucEnum<#enum_name> for #enum_name {
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

            unsafe fn config_matrix(&self) -> *const pgx::pg_sys::config_enum_entry {
                let slice = pgx::PgMemoryContexts::TopMemoryContext.palloc0_slice::<pg_sys::config_enum_entry>(#enum_len + 1usize);

                #build_array_body

                slice.as_ptr()
            }
        }
    });

    stream
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

/// Embed SQL directly into the generated extension script.
///
/// The argument must be as single raw string literal.
///
/// # Example
/// ```
/// # #[macro_use]
/// # extern crate pgx_macros;
/// # fn main() {
/// extension_sql!(r#"
/// -- sql statements
/// "#)
/// # }
/// ```

#[proc_macro]
pub fn extension_sql(input: TokenStream) -> TokenStream {
    fn is_raw_str(input: &str) -> bool {
        input.starts_with("r#\"") && input.ends_with("\"#")
    }

    let tokens: Vec<String> = input.into_iter().map(|t| t.to_string()).collect();

    let ok = (tokens.len() >= 1 && is_raw_str(&tokens[0]))
        && (tokens.len() == 1 || (tokens.len() >= 2 && tokens[1] == ","));

    if ok {
        // ignore input
        TokenStream::new()
    } else {
        TokenStream::from(quote! {
          compile_error!("expected a single raw string literal with sql");
        })
    }
}

#[proc_macro_derive(PostgresEq)]
pub fn postgres_eq(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_postgres_eq(ast).into()
}

#[proc_macro_derive(PostgresOrd)]
pub fn postgres_ord(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_postgres_ord(ast).into()
}

#[proc_macro_derive(PostgresHash)]
pub fn postgres_hash(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_postgres_hash(ast).into()
}
