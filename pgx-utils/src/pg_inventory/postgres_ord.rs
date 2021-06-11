use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Ident;

pub struct PostgresOrd {
    pub name: Ident,
}

impl PostgresOrd {
    pub fn new(name: Ident) -> Self {
        Self { name }
    }
}

impl ToTokens for PostgresOrd {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let inv = quote! {
            pgx::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PgxPostgresOrd {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    full_path: core::any::type_name::<#name>(),
                    module_path: module_path!(),
                    id: TypeId::of::<#name>(),
                }
            }
        };
        tokens.append_all(inv);
    }
}
