use proc_macro2::{TokenStream as TokenStream2, Span};
use proc_macro::TokenStream;
use syn::{Token, punctuated::Punctuated, parse::{Parse, ParseStream}};
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug)]
pub struct Attributes {
    attrs: Punctuated<Attribute, Token![,]>,
}


impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.parse_terminated(Attribute::parse)?,
        })
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = &self.attrs;
        let quoted = quote! {
            vec![#attrs]
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug)]
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
}

impl ToTokens for Attribute {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let quoted = match self {
            Attribute::Immutable => quote! { pgx_utils::ExternArgs::Immutable },
            Attribute::Strict => quote! { pgx_utils::ExternArgs::Strict },
            Attribute::Stable => quote! { pgx_utils::ExternArgs::Stable },
            Attribute::Volatile => quote! { pgx_utils::ExternArgs::Volatile },
            Attribute::Raw => quote! { pgx_utils::ExternArgs::Raw },
            Attribute::NoGuard => quote! { pgx_utils::ExternArgs::NoGuard },
            Attribute::ParallelSafe => quote! { pgx_utils::ExternArgs::ParallelSafe },
            Attribute::ParallelUnsafe => quote! { pgx_utils::ExternArgs::ParallelUnsafe },
            Attribute::ParallelRestricted => quote! { pgx_utils::ExternArgs::ParallelRestricted },
            Attribute::Error(s) => quote! { pgx_utils::ExternArgs::Error(String::from(#s)) },
            Attribute::Schema(s) => quote! { pgx_utils::ExternArgs::Schema(String::from(#s)) },
            Attribute::Name(s) => quote! { pgx_utils::ExternArgs::Name(String::from(#s)) },
        };
        tokens.append_all(quoted);
    }
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident: syn::Ident = input.parse()?;
        let found = match ident.to_string().as_str() {
            "immutable" => Attribute::Immutable,
            "strict" => Attribute::Strict,
            "stable" => Attribute::Stable,
            "volatile" => Attribute::Volatile,
            "raw" => Attribute::Raw,
            "no_guard" => Attribute::NoGuard,
            "parallel_safe" => Attribute::ParallelSafe,
            "parallel_unsafe" => Attribute::ParallelUnsafe,
            "parallel_restricted" => Attribute::ParallelRestricted,
            "error" => {
                let inner;
                let _punc: syn::token::Paren = syn::parenthesized!(inner in input);
                let literal: syn::LitStr = inner.parse()?;
                Attribute::Error(literal)
            },
            "schema" => {
                let inner;
                let _punc: syn::token::Paren = syn::parenthesized!(inner in input);
                let literal: syn::LitStr = inner.parse()?;
                Attribute::Schema(literal)
            },
            "name" => {
                let inner;
                let _punc: syn::token::Paren = syn::parenthesized!(inner in input);
                let literal: syn::LitStr = inner.parse()?;
                Attribute::Name(literal)
            },
            _ => return Err(syn::Error::new(Span::call_site(), "Invalid option")),
        };
        Ok(found)
    }
}