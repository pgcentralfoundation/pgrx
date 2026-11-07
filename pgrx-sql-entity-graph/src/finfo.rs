use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{self, ItemFn, spanned::Spanned};

/// Export the PostgreSQL function-info symbol for exactly this function identifier.
///
/// Its immutable record comes from the original C PG_FUNCTION_INFO_V1 declaration.
/// PostgreSQL locates this exported symbol by prefixing the function name with `pg_finfo_`.
pub fn finfo_v1_tokens(ident: proc_macro2::Ident) -> syn::Result<ItemFn> {
    let finfo_name = format_ident!("pg_finfo_{ident}");
    let tokens = quote! {
        #[unsafe(no_mangle)]
        #[doc(hidden)]
        pub extern "C" fn #finfo_name() -> &'static ::pgrx::pg_sys::Pg_finfo_record {
            ::pgrx::pg_sys::__pgrx_function_info_v1()
        }
    };
    syn::parse2(tokens)
}

pub fn finfo_v1_extern_c(
    original: &syn::ItemFn,
    fcinfo: Ident,
    contents: TokenStream,
) -> syn::Result<ItemFn> {
    let original_name = &original.sig.ident;
    let wrapper_symbol = format_ident!("{}_wrapper", original_name);

    let synthetic = proc_macro2::Span::mixed_site();
    let synthetic = synthetic.located_at(original.sig.span());

    let tokens = quote_spanned! { synthetic =>
        #[unsafe(no_mangle)]
        #[doc(hidden)]
        pub unsafe extern "C-unwind" fn #wrapper_symbol(#fcinfo: ::pgrx::pg_sys::FunctionCallInfo) -> ::pgrx::pg_sys::Datum {
            #contents
        }
    };

    syn::parse2(tokens)
}
