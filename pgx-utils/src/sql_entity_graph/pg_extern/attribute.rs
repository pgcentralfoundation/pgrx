use crate::sql_entity_graph::PositioningRef;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

#[derive(Debug, Clone)]
pub struct PgxAttributes {
    pub attrs: Punctuated<Attribute, Token![,]>,
}

impl Parse for PgxAttributes {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.parse_terminated(Attribute::parse)?,
        })
    }
}

impl ToTokens for PgxAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = &self.attrs;
        let quoted = quote! {
            vec![#attrs]
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Attribute {
    Immutable,
    Strict,
    Stable,
    Volatile,
    Raw,
    NoGuard,
    ParallelSafe,
    ParallelUnsafe,
    ParallelRestricted,
    Error(syn::LitStr),
    Schema(syn::LitStr),
    Name(syn::LitStr),
    Cost(syn::Expr),
    Requires(Punctuated<PositioningRef, Token![,]>),
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let quoted = match self {
            Attribute::Immutable => quote! { pgx::datum::sql_entity_graph::ExternArgs::Immutable },
            Attribute::Strict => quote! { pgx::datum::sql_entity_graph::ExternArgs::Strict },
            Attribute::Stable => quote! { pgx::datum::sql_entity_graph::ExternArgs::Stable },
            Attribute::Volatile => quote! { pgx::datum::sql_entity_graph::ExternArgs::Volatile },
            Attribute::Raw => quote! { pgx::datum::sql_entity_graph::ExternArgs::Raw },
            Attribute::NoGuard => quote! { pgx::datum::sql_entity_graph::ExternArgs::NoGuard },
            Attribute::ParallelSafe => {
                quote! { pgx::datum::sql_entity_graph::ExternArgs::ParallelSafe }
            }
            Attribute::ParallelUnsafe => {
                quote! { pgx::datum::sql_entity_graph::ExternArgs::ParallelUnsafe }
            }
            Attribute::ParallelRestricted => {
                quote! { pgx::datum::sql_entity_graph::ExternArgs::ParallelRestricted }
            }
            Attribute::Error(s) => {
                quote! { pgx::datum::sql_entity_graph::ExternArgs::Error(String::from(#s)) }
            }
            Attribute::Schema(s) => {
                quote! { pgx::datum::sql_entity_graph::ExternArgs::Schema(String::from(#s)) }
            }
            Attribute::Name(s) => {
                quote! { pgx::datum::sql_entity_graph::ExternArgs::Name(String::from(#s)) }
            }
            Attribute::Cost(s) => {
                quote! { pgx::datum::sql_entity_graph::ExternArgs::Cost(format!("{}", #s)) }
            }
            Attribute::Requires(items) => {
                let items_iter = items
                    .iter()
                    .map(|x| x.to_token_stream())
                    .collect::<Vec<_>>();
                quote! { pgx::datum::sql_entity_graph::ExternArgs::Requires(vec![#(#items_iter),*],) }
            }
        };
        tokens.append_all(quoted);
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident: syn::Ident = input.parse()?;
        let found = match ident.to_string().as_str() {
            "immutable" => Self::Immutable,
            "strict" => Self::Strict,
            "stable" => Self::Stable,
            "volatile" => Self::Volatile,
            "raw" => Self::Raw,
            "no_guard" => Self::NoGuard,
            "parallel_safe" => Self::ParallelSafe,
            "parallel_unsafe" => Self::ParallelUnsafe,
            "parallel_restricted" => Self::ParallelRestricted,
            "error" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::LitStr = input.parse()?;
                Self::Error(literal)
            }
            "schema" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::LitStr = input.parse()?;
                Attribute::Schema(literal)
            }
            "name" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::LitStr = input.parse()?;
                Self::Name(literal)
            }
            "cost" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::Expr = input.parse()?;
                Self::Cost(literal)
            }
            "requires" => {
                let _eq: syn::token::Eq = input.parse()?;
                let content;
                let _bracket = syn::bracketed!(content in input);
                Self::Requires(content.parse_terminated(PositioningRef::parse)?)
            }
            _ => return Err(syn::Error::new(Span::call_site(), "Invalid option")),
        };
        Ok(found)
    }
}
