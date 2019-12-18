extern crate proc_macro;

use pg_guard_rewriter::{PgGuardRewriter, RewriteMode};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Item, ItemFn};

#[proc_macro_attribute]
pub fn pg_guard(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // get a usable token stream
    let ast = parse_macro_input!(item as syn::Item);

    let rewriter = PgGuardRewriter::new(RewriteMode::RewriteFunctionWithWrapper);

    match ast {
        Item::ForeignMod(block) => TokenStream::from(rewriter.extern_block(block)),
        Item::Fn(func) => TokenStream::from(rewriter.item_fn(func)),
        _ => {
            panic!("#[pg_guard] can only be applied to extern \"C\" blocks and top-level functions")
        }
    }
}

#[proc_macro_attribute]
pub fn pg_extern(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as syn::Item);

    match ast {
        Item::Fn(func) => TokenStream::from(rewrite_item_fn(func)),
        _ => panic!("#[pg_extern] can only be applied to top-level functions"),
    }
}

fn rewrite_item_fn(func: ItemFn) -> proc_macro2::TokenStream {
    let finfo_name = syn::Ident::new(&format!("pg_finfo_{}", func.sig.ident), Span::call_site());

    quote! {
    #[no_mangle]
        pub extern "C" fn #finfo_name() -> &'static pg_sys::Pg_finfo_record {
            const V1_API: pg_sys::Pg_finfo_record = pg_sys::Pg_finfo_record { api_version: 1 };
            &V1_API
        }

        #[pg_guard]
        #func
    }

    // TODO:  how to automatically convert function arguments?
    // TODO:  should we even do that?  I think macros in favor of
    // TODO:  mimicking PG_GETARG_XXX() makes more sense
}
