use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{self, ItemFn, spanned::Spanned};

/// Generate the Postgres fn info record
///
/// Equivalent to PG_FUNCTION_INFO_V1, Postgres will sprintf the fn ident, then `dlsym(so, expected_name)`,
/// so it is important to pass exactly the ident that you want to have the record associated with!
#[cfg(feature = "pgrust")]
pub fn finfo_v1_tokens(ident: proc_macro2::Ident) -> syn::Result<ItemFn> {
    // pgrust resolves wrappers through the linkme registry, not dlsym +
    // pg_finfo records; emit an inert placeholder so callers stay uniform.
    let finfo_name = format_ident!("__pgrx_no_finfo_{ident}");
    let tokens = quote! {
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        fn #finfo_name() {}
    };
    syn::parse2(tokens)
}

#[cfg(not(feature = "pgrust"))]
pub fn finfo_v1_tokens(ident: proc_macro2::Ident) -> syn::Result<ItemFn> {
    let finfo_name = format_ident!("pg_finfo_{ident}");
    let tokens = quote! {
        #[unsafe(no_mangle)]
        #[doc(hidden)]
        pub extern "C" fn #finfo_name() -> &'static ::pgrx::pg_sys::Pg_finfo_record {
            const V1_API: ::pgrx::pg_sys::Pg_finfo_record = ::pgrx::pg_sys::Pg_finfo_record { api_version: 1 };
            &V1_API
        }
    };
    syn::parse2(tokens)
}

/// The pgrust wrapper: pgrust's native `PGFunction` signature around the same
/// body, run through `call_v1` (which builds the C-shaped frame the body
/// reads), plus a registry entry so `dfmgr` can find it by symbol name.
#[cfg(feature = "pgrust")]
pub fn finfo_v1_extern_c(
    original: &syn::ItemFn,
    fcinfo: Ident,
    contents: TokenStream,
) -> syn::Result<ItemFn> {
    let original_name = &original.sig.ident;
    let wrapper_symbol = format_ident!("{}_wrapper", original_name);
    let entry_symbol = format_ident!("__PGRX_FN_{}", original_name);

    let synthetic = proc_macro2::Span::mixed_site();
    let synthetic = synthetic.located_at(original.sig.span());

    let tokens = quote_spanned! { synthetic =>
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub fn #wrapper_symbol(
            __pgrx_flinfo: ::core::option::Option<&mut ::pgrx::pg_sys::pgrust::NativeFmgrInfo>,
            __pgrx_fcinfo: &mut ::pgrx::pg_sys::pgrust::NativeFcinfo,
        ) -> ::pgrx::pg_sys::pgrust::NativeResult {
            #[::pgrx::pg_sys::pgrust::linkme::distributed_slice(::pgrx::pg_sys::pgrust::PGRX_FUNCTIONS)]
            #[linkme(crate = ::pgrx::pg_sys::pgrust::linkme)]
            static #entry_symbol: ::pgrx::pg_sys::pgrust::FnEntry = ::pgrx::pg_sys::pgrust::FnEntry {
                krate: env!("CARGO_PKG_NAME"),
                symbol: stringify!(#wrapper_symbol),
                func: #wrapper_symbol,
            };
            ::pgrx::pg_sys::pgrust::call_v1(__pgrx_flinfo, __pgrx_fcinfo, |#fcinfo: ::pgrx::pg_sys::FunctionCallInfo| -> ::pgrx::pg_sys::Datum {
                #contents
            })
        }
    };

    syn::parse2(tokens)
}

#[cfg(not(feature = "pgrust"))]
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
