extern crate proc_macro;

mod rewriter;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use rewriter::*;
use std::collections::HashSet;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Item, ItemFn};

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
        Item::Fn(func) => rewriter.item_fn(func, false, false, false).into(),
        _ => {
            panic!("#[pg_guard] can only be applied to extern \"C\" blocks and top-level functions")
        }
    }
}

#[proc_macro_attribute]
pub fn pg_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut stream = proc_macro2::TokenStream::new();
    let args = parse_extern_attributes(&attr);

    let mut expected_error = None;
    args.into_iter().for_each(|v| match v {
        ExternArgs::Error(message) => expected_error = Some(message),
        _ => {}
    });

    stream.extend(proc_macro2::TokenStream::from(pg_extern(
        attr.clone(),
        item.clone(),
    )));

    let expected_error = match expected_error {
        Some(msg) => quote! {Some(#msg)},
        None => quote! {None},
    };

    let ast = parse_macro_input!(item as syn::Item);
    match ast {
        Item::Fn(func) => {
            let func_name = Ident::new(
                &format!("{}_wrapper", func.sig.ident.to_string()),
                func.span(),
            );
            let test_func_name =
                Ident::new(&format!("{}_test", func.sig.ident.to_string()), func.span());

            stream.extend(quote! {
                #[test]
                fn #test_func_name() {
                    pgx_tests::run_test(#func_name, #expected_error)
                }
            });
        }

        _ => panic!("#[pg_test] can only be applied to top-level functions"),
    }

    stream.into()
}

#[proc_macro_attribute]
pub fn pg_extern(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_extern_attributes(&attr);
    let is_raw = args.contains(&ExternArgs::Raw);
    let no_guard = args.contains(&ExternArgs::NoGuard);

    let ast = parse_macro_input!(item as syn::Item);
    match ast {
        Item::Fn(func) => rewrite_item_fn(func, is_raw, no_guard).into(),
        _ => panic!("#[pg_extern] can only be applied to top-level functions"),
    }
}

fn rewrite_item_fn(mut func: ItemFn, is_raw: bool, no_guard: bool) -> proc_macro2::TokenStream {
    let finfo_name = syn::Ident::new(
        &format!("pg_finfo_{}_wrapper", func.sig.ident),
        Span::call_site(),
    );

    // use the PgGuardRewriter to go ahead and wrap the function here, rather than applying
    // a #[pg_guard] macro to the original function.  This is necessary so that compiler
    // errors/warnings indicate the proper line numbers
    let rewriter = PgGuardRewriter::new();

    // make the function 'extern "C"' because this is for the #[pg_extern[ macro
    func.sig.abi = Some(syn::parse_str("extern \"C\"").unwrap());
    let func_span = func.span().clone();
    let rewritten_func = rewriter.item_fn(func, true, is_raw, no_guard);

    quote_spanned! {func_span=>
        #[no_mangle]
        pub extern "C" fn #finfo_name() -> &'static pg_sys::Pg_finfo_record {
            const V1_API: pg_sys::Pg_finfo_record = pg_sys::Pg_finfo_record { api_version: 1 };
            &V1_API
        }

        #rewritten_func
    }

    // TODO:  how to automatically convert function arguments?
    // TODO:  should we even do that?  I think macros in favor of
    // TODO:  mimicking PG_GETARG_XXX() makes more sense
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
enum ExternArgs {
    Immutable,
    Strict,
    Stable,
    Volatile,
    Raw,
    NoGuard,
    ParallelSafe,
    ParallelUnsafe,
    ParallelRestricted,
    Error(String),
}

fn parse_extern_attributes(attr: &TokenStream) -> HashSet<ExternArgs> {
    let attr_string = attr.to_string();
    let attrs: Vec<&str> = attr_string.split(',').collect();

    let mut args = HashSet::<ExternArgs>::new();
    for att in attrs {
        let att = att.trim();

        match att {
            "immutable" => args.insert(ExternArgs::Immutable),
            "strict" => args.insert(ExternArgs::Strict),
            "stable" => args.insert(ExternArgs::Stable),
            "volatile" => args.insert(ExternArgs::Volatile),
            "raw" => args.insert(ExternArgs::Raw),
            "no_guard" => args.insert(ExternArgs::NoGuard),
            "parallel_safe" => args.insert(ExternArgs::ParallelSafe),
            "parallel_unsafe" => args.insert(ExternArgs::ParallelUnsafe),
            "parallel_restricted" => args.insert(ExternArgs::ParallelRestricted),
            error if att.starts_with("error") => {
                let re = regex::Regex::new(r#"("[^"\\]*(?:\\.[^"\\]*)*")"#).unwrap();

                let message = match re.captures(error) {
                    Some(captures) => match captures.get(0) {
                        Some(mtch) => {
                            let message = mtch.as_str().clone();
                            let message = unescape::unescape(message)
                                .expect("improperly escaped error message");

                            // trim leading/trailing quotes
                            let message = String::from(&message[1..]);
                            let message = String::from(&message[..message.len() - 1]);

                            message
                        }
                        None => {
                            panic!("No matches found in: {}", error);
                        }
                    },
                    None => panic!("/{}/ is an invalid error= attribute", error),
                };

                args.insert(ExternArgs::Error(message.to_string()))
            }

            _ => false,
        };
    }
    args
}

#[proc_macro]
pub fn extension_sql(_: TokenStream) -> TokenStream {
    // we don't want to output anything here
    TokenStream::new()
}
