use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    DeriveInput, Generics, ItemEnum, ItemImpl,
};
use syn::{punctuated::Punctuated, Ident, Token};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct AggregateAttrs {
    attrs: Punctuated<AggregateAttr, Token![,]>
}

impl Parse for AggregateAttrs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.parse_terminated(AggregateAttr::parse)?,
        })
    }
}

impl ToTokens for AggregateAttrs {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = &self.attrs;
        let quoted = quote! {
            #attrs
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum AggregateAttr {
    Parallel(syn::TypePath),
    InitialCondition(syn::LitStr),
    MovingInitialCondition(syn::LitStr),
    Hypothetical,
}

impl Parse for AggregateAttr {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident: syn::Ident = input.parse()?;
        let found = match ident.to_string().as_str() {
            "immutable" => Self::Immutable,
            "parallel" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::TypePath = input.parse()?;
                Self::Error(literal)
            }
        };
        Ok(found)
    }
}


/** A parsed `#[pg_aggregate]` item.
*/
#[derive(Debug, Clone)]
pub struct Aggregate {
    attrs: Option<AggregateAttrs>,
    item: ItemImpl,

}

impl Aggregate {
    pub fn new(
        item: ItemImpl,
    ) -> Self {
        Self {
            attrs,
            item,
        }
    }
}

impl Parse for Aggregate {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.parse()?,
            item: input.parse()?,
        })
    }
}


impl ToTokens for Aggregate {
    fn to_tokens(&self, tokens: &mut TokenStream2) {

    }
}
