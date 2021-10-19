use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Token,
};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PgAggregateAttrs {
    attrs: Punctuated<PgAggregateAttr, Token![,]>,
}

impl Parse for PgAggregateAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.parse_terminated(PgAggregateAttr::parse)?,
        })
    }
}

impl ToTokens for PgAggregateAttrs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = &self.attrs;
        let quoted = quote! {
            #attrs
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PgAggregateAttr {
    Parallel(syn::TypePath),
    InitialCondition(syn::LitStr),
    MovingInitialCondition(syn::LitStr),
    Hypothetical,
}

impl Parse for PgAggregateAttr {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident: syn::Ident = input.parse()?;
        let found = match ident.to_string().as_str() {
            "hypothetical" => Self::Hypothetical,
            "initial_condition" => {
                let condition = input.parse()?;
                Self::InitialCondition(condition)
            }
            "moving_initial_condition" => {
                let condition = input.parse()?;
                Self::MovingInitialCondition(condition)
            }
            "parallel" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::TypePath = input.parse()?;
                Self::Parallel(literal)
            }
            _ => {
                return Err(syn::Error::new(
                    input.span(),
                    "Recieved unknown `pg_aggregate` attr.",
                ))
            }
        };
        Ok(found)
    }
}

impl ToTokens for PgAggregateAttr {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let quoted = quote! {
            #self
        };
        tokens.append_all(quoted);
    }
}
