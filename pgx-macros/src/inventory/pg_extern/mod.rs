mod attribute;
mod argument;
mod returning;
mod search_path;

use attribute::Attributes;
use argument::Argument;
use returning::Returning;
use search_path::SearchPathList;

use syn::{Token, token::Token, ItemFn, FnArg};
use syn::parse::{Parse, ParseStream, ParseBuffer};
use quote::{ToTokens, quote, TokenStreamExt};
use proc_macro2::{TokenStream as TokenStream2, Span, Ident};
use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::ops::Deref;

#[derive(Debug)]
pub struct PgxExtern {
    attrs: Attributes,
    func: syn::ItemFn,
}


impl PgxExtern {
    fn extern_attrs(&self) -> &Attributes {
        &self.attrs
    }

    fn search_path(&self) -> Option<SearchPathList> {
        self.func.attrs.iter().find(|f| {
            f.path.segments.first().map(|f| {
                f.ident == Ident::new("search_path", Span::call_site())
            }).unwrap_or_default()
        }).and_then(|attr| {
            Some(attr.parse_args::<SearchPathList>().unwrap())
        })
    }

    fn inputs(&self) -> Vec<Argument> {
        self.func.sig.inputs.iter().flat_map(|input| {
            Argument::try_from(input.clone()).ok()
        }).collect()
    }

    fn returns(&self) -> Returning {
        Returning::try_from(&self.func.sig.output).unwrap()
    }


    pub(crate) fn new(attr: TokenStream, item: TokenStream) -> Result<Self, syn::Error> {
        let attrs = syn::parse::<Attributes>(attr)?;
        let func = syn::parse::<syn::ItemFn>(item)?;
        Ok(Self {
            attrs,
            func,
        })
    }
}


impl ToTokens for PgxExtern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.func.sig.ident;
        let extern_attrs = self.extern_attrs();
        let search_path = self.search_path().into_iter();
        let inputs = self.inputs();
        let returns = self.returns();

        let inv = quote! {
            pgx::inventory::submit! {
                use core::any::TypeId;
                crate::__pgx_internals::PgxExtern {
                    name: stringify!(#ident),
                    module_path: core::module_path!(),
                    extern_attrs: #extern_attrs,
                    search_path: None#( .unwrap_or(Some(vec![#search_path])) )*,
                    fn_args: vec![#(#inputs),*],
                    fn_return: #returns,
                }
            }
        };
        tokens.append_all(inv);
    }
}

impl Parse for PgxExtern {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.parse()?,
            func: input.parse()?,
        })
    }
}
