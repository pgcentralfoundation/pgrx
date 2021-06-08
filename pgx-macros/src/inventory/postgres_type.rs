use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};

pub struct PostgresType {
    name: Ident,
    in_fn: Ident,
    out_fn: Ident,
}

impl PostgresType {
    pub(crate) fn new(name: Ident, in_fn: Ident, out_fn: Ident) -> Self {
        Self {
            name,
            in_fn,
            out_fn,
        }
    }
}

impl ToTokens for PostgresType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let in_fn = &self.in_fn;
        let out_fn = &self.out_fn;
        let inv = quote! {
            pgx::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PgxPostgresType {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    full_path: core::any::type_name::<#name>(),
                    id: TypeId::of::<#name>(),
                    option_id: TypeId::of::<Option<#name>>(),
                    vec_id: TypeId::of::<Vec<#name>>(),
                    in_fn: stringify!(#in_fn),
                    out_fn: stringify!(#out_fn),
                }
            }
        };
        tokens.append_all(inv);
    }
}
