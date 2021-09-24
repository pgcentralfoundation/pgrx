use proc_macro2::{TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{ItemImpl, parse::{Parse, ParseStream}, parse_quote};
use syn::{punctuated::Punctuated, Token};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct PgAggregateAttrs {
    attrs: Punctuated<PgAggregateAttr, Token![,]>
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
            },
            "moving_initial_condition" => {
                let condition = input.parse()?;
                Self::MovingInitialCondition(condition)
            },
            "parallel" => {
                let _eq: Token![=] = input.parse()?;
                let literal: syn::TypePath = input.parse()?;
                Self::Parallel(literal)
            },
            _ => return Err(syn::Error::new(input.span(), "Recieved unknown `pg_aggregate` attr.")),
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

/** A parsed `#[pg_aggregate]` item.
*/
#[derive(Debug, Clone)]
pub struct PgAggregate {
    // Options relevant to the aggregate's final implementation or SQL generation.
    attrs: Option<PgAggregateAttrs>,
    item: ItemImpl,
}

impl PgAggregate {
    pub fn new(
        mut item_impl: ItemImpl,
    ) -> Result<Self, syn::Error> {

        if let Some((_, ref path, _)) = item_impl.trait_ {
            if let Some(last) = path.segments.last() {
                if last.ident.to_string() != "Aggregate" {
                    return Err(syn::Error::new(last.ident.span(), "`#[pg_aggregate]` only works with the `Aggregate` trait."))
                }
            }
        }

        let mut aggregate_attrs = None;
        for attr in item_impl.attrs.clone() {
            let attr_path = attr.path.segments.last();
            if let Some(candidate_path) = attr_path {
                if candidate_path.ident.to_string() == "pg_aggregate" {
                    let parsed: PgAggregateAttrs = syn::parse2(attr.tokens)?;
                    aggregate_attrs = Some(parsed);
                }
            }
        }

        let mut moving_state = None;
        for impl_item in &item_impl.items {
            match impl_item {
                syn::ImplItem::Type(impl_item_type) => {
                    let ident_string = impl_item_type.ident.to_string();
                    if ident_string == "MovingState" {
                        moving_state = Some(impl_item_type);
                    }
                },
                _ => (),
            }
        }
        if moving_state.is_none() {
            item_impl.items.push(parse_quote! {
                type MovingState = ();
            })
        }

        Ok(Self {
            attrs: aggregate_attrs,
            item: item_impl,
        })
    }
}

impl Parse for PgAggregate {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::new(input.parse()?)
    }
}


impl ToTokens for PgAggregate {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let impl_item = &self.item;
        let inv = quote! {
            #impl_item
        };
        tokens.append_all(inv);
    }
}
