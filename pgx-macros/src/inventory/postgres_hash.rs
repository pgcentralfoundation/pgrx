use proc_macro2::{TokenStream as TokenStream2};
use syn::{Ident, Token, punctuated::Punctuated};
use quote::{quote, ToTokens, TokenStreamExt};

pub struct PostgresHash {
    pub name: Ident,
}

impl PostgresHash {
    pub(crate) fn new(name: Ident) -> Self {
        Self {
            name,
        }
    }
}

impl ToTokens for PostgresHash {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let inv = quote! {
            pgx::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PgxPostgresHash {
                    name: stringify!(#name),
                    file: file!(),
                    full_path: core::any::type_name::<#name>(),
                    id: TypeId::of::<#name>(),
                }
            }
        };
        tokens.append_all(inv);
    }
}