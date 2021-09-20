use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    DeriveInput, Generics, ItemEnum, ItemImpl,
};
use syn::{punctuated::Punctuated, Ident, Token};


pub struct AggregateAttrs {
    attrs: Punctuated<AggregateAttr, Token![,]>
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
    Parallel(ParallelOption),
    InitialCondition(syn::LitStr),
    MovingInitialCondition(syn::LitStr),
    Hypothetical,
}

pub enum ParallelOption {
    Safe,
    Restricted,
    Unsafe,
}

impl Parse for ParallelOption {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let parallel_option: syn::Path = input.parse()?;
        if let Some(ident) = parallel_option.get_ident() {
            match ident.to_tokens().to_string() {
                "Safe" => Ok(Self::Safe),
                "Restricted" => Ok(Self::Restricted),
                "Unsafe" => Ok(Self::Unsafe),
            }
        } else {
            Ok(Vec::new())
        }
        
        Ok()
    }
}


impl ToTokens for ParallelOption {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let quoted = quote! {
            #self
        };
        tokens.append_all(quoted);
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
