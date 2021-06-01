use proc_macro2::{TokenStream as TokenStream2, Span};
use syn::{Ident, Token, punctuated::Punctuated};
use quote::{quote, ToTokens, TokenStreamExt};

pub struct PostgresEnum {
    pub name: Ident,
    pub variants: Punctuated<syn::Variant, Token![,]>,
}

impl PostgresEnum {
    pub(crate) fn new(name: Ident, variants: Punctuated<syn::Variant, Token![,]>) -> Self {
        Self {
            name,
            variants,
        }
    }
}

impl ToTokens for PostgresEnum {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let variants = self.variants.iter();
        let inv = quote! {
            pgx::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PgxPostgresEnum {
                    name: core::any::type_name::<#name>(),
                    id: TypeId::of::<#name>(),
                    variants: vec![ #(  stringify!(#variants)  ),* ],
                }
            }
        };
        tokens.append_all(inv);
    }
}