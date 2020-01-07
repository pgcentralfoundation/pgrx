extern crate proc_macro;

mod rewriter;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use rewriter::*;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, Item, ItemFn, Type};

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

    stream.extend(proc_macro2::TokenStream::from(pg_extern(
        attr.clone(),
        item.clone(),
    )));

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
                    pg_bridge_tests::run_test(#func_name)
                }
            });
        }

        _ => panic!("#[pg_test] can only be applied to top-level functions"),
    }

    stream.into()
}

#[proc_macro_attribute]
pub fn pg_extern(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as syn::Item);
    let attr_string = attr.to_string();
    let attrs: Vec<_> = attr_string.split(',').collect();

    let mut is_raw = false;
    let mut no_guard = false;

    for attr in attrs {
        match attr.trim() {
            "raw" => is_raw = true,
            "no_guard" => no_guard = true,
            "strict" => { /* noop */ }
            "stable" => { /* noop */ }
            "volatile" => { /* noop */ }
            "immutable" => { /* noop */ }
            "parallel_safe" => { /* noop */ }
            "parallel_unsafe" => { /* noop */ }
            "parallel_restricted" => { /* noop */ }
            "" => { /* noop */ }
            unknown => panic!("unrecognized #[pg_extern] attribute: '{}'", unknown),
        }
    }

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

#[proc_macro_derive(DatumCompatible)]
pub fn derive_datum_compatible(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_datum_compatible(&ast)
}

fn impl_datum_compatible(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let name_str = format!("{}", name);

    if name_str.starts_with('_') {
        // skip types that start with an underscore b/c they're likely not general Postgres structs
        TokenStream::new()
    } else {
        match &ast.data {
            Data::Struct(ds) => {
                if !type_blacklisted_for_datum_compatible(ds) {
                    (quote! {
                        impl DatumCompatible<#name> for #name {
                            fn copy_into(&self, memory_context: &mut pg_bridge::PgMemoryContexts) -> pg_bridge::PgDatum<#name> {
                                memory_context.copy_struct_into(self)
                            }

                        }
                    })
                    .into()
                } else {
                    TokenStream::new()
                }
            }
            Data::Enum(_) => TokenStream::new(),
            Data::Union(_) => TokenStream::new(),
        }
    }
}

fn type_blacklisted_for_datum_compatible(ds: &syn::DataStruct) -> bool {
    for field in ds.fields.iter() {
        let ty = &field.ty;
        if let Type::Path(path) = ty {
            for segment in path.path.segments.iter() {
                if segment.ident.eq("__IncompleteArrayField") {
                    return true;
                }
            }
        }
    }

    false
}
