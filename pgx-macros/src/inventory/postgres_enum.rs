use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{punctuated::Punctuated, Ident, Token};

pub struct PostgresEnum {
    pub name: Ident,
    pub variants: Punctuated<syn::Variant, Token![,]>,
}

impl PostgresEnum {
    pub(crate) fn new(name: Ident, variants: Punctuated<syn::Variant, Token![,]>) -> Self {
        Self { name, variants }
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
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    module_path: module_path!(),
                    full_path: core::any::type_name::<#name>(),
                    id: TypeId::of::<#name>(),
                    option_id: TypeId::of::<Option<#name>>(),
                    vec_id: TypeId::of::<Vec<#name>>(),
                    variants: vec![ #(  stringify!(#variants)  ),* ],
                }
            }
        };
        tokens.append_all(inv);
    }
}
