use proc_macro2::{TokenStream as TokenStream2, Span};
use proc_macro::TokenStream;
use syn::{Ident, Token, punctuated::Punctuated, parse::{Parse, ParseStream}, FnArg};
use quote::{quote, ToTokens, TokenStreamExt};
use std::{convert::TryFrom, ops::Deref};

pub struct PostgresType {
    name: Ident,
    in_fn: Ident,
    out_fn: Ident
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
                    name: core::any::type_name::<#name>(),
                    id: TypeId::of::<#name>(),
                    in_fn: stringify!(#in_fn),
                    out_fn: stringify!(#out_fn),
                }
            }
        };
        tokens.append_all(inv);
    }
}