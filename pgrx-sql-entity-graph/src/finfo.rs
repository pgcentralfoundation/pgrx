use quote::{format_ident, quote};
use syn;

/// Generate the Postgres fn info record
///
/// Equivalent to PG_FUNCTION_INFO_V1, Postgres will call this fn for metadata
/// so it has to match the relevant fn's name exactly.
pub fn finfo_v1_tokens(ident: proc_macro2::Ident) -> syn::Result<syn::ItemFn> {
    let finfo_name = format_ident!("pg_finfo_{ident}");
    let tokens = quote! {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn #finfo_name() -> &'static ::pgrx::pg_sys::Pg_finfo_record {
            const V1_API: ::pgrx::pg_sys::Pg_finfo_record = ::pgrx::pg_sys::Pg_finfo_record { api_version: 1 };
            &V1_API
        }
    };
    syn::parse2(tokens)
}
