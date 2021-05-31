use proc_macro2::{TokenStream as TokenStream2, Span};
use proc_macro::TokenStream;
use syn::{Token, punctuated::Punctuated, parse::{Parse, ParseStream}};
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug)]
pub struct SearchPath {
    at_start: Option<syn::token::At>,
    dollar: Option<syn::token::Dollar>,
    path: syn::Ident,
    at_end: Option<syn::token::At>,
}

impl Parse for SearchPath {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            at_start: input.parse()?,
            dollar: input.parse()?,
            path: input.parse()?,
            at_end: input.parse()?,
        })
    }
}

impl ToTokens for SearchPath {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let at_start = self.at_start;
        let dollar = self.dollar;
        let path = &self.path;
        let at_end = self.at_end;

        let quoted = quote! {
            #at_start#dollar#path#at_end
        };

        quoted.to_string().to_tokens(tokens);
    }
}

#[derive(Debug)]
pub struct SearchPathList {
    fields: Punctuated<SearchPath, Token![,]>,
}

impl Parse for SearchPathList {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            fields: input.parse_terminated(SearchPath::parse).expect(&format!("Got {}", input)),
        })
    }
}

impl ToTokens for SearchPathList {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.fields.to_tokens(tokens)
    }
}
