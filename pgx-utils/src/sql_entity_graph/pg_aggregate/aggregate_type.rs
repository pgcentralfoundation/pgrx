use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, Expr, Type,
};

#[derive(Debug, Clone)]
pub(crate) struct AggregateTypeList {
    pub(crate) found: Vec<AggregateType>,
    pub(crate) original: syn::Type,
}

impl AggregateTypeList {
    pub(crate) fn new(maybe_type_list: syn::Type) -> Result<Self, syn::Error> {
        match &maybe_type_list {
            Type::Tuple(tuple) => {
                let mut coll = Vec::new();
                for elem in &tuple.elems {
                    let parsed_elem = AggregateType::new(elem.clone())?;
                    coll.push(parsed_elem);
                }
                Ok(Self {
                    found: coll,
                    original: maybe_type_list,
                })
            }
            ty => Ok(Self {
                found: vec![AggregateType::new(ty.clone())?],
                original: maybe_type_list,
            }),
        }
    }

    pub(crate) fn entity_tokens(&self) -> Expr {
        let found = self.found.iter().map(|x| x.entity_tokens());
        parse_quote! {
            vec![#(#found),*]
        }
    }
}

impl Parse for AggregateTypeList {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::new(input.parse()?)
    }
}

impl ToTokens for AggregateTypeList {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.original.to_tokens(tokens)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct AggregateType {
    pub(crate) ty: Type,
}

impl AggregateType {
    pub(crate) fn new(ty: syn::Type) -> Result<Self, syn::Error> {
        let retval = Self { ty };
        Ok(retval)
    }

    pub(crate) fn entity_tokens(&self) -> Expr {
        let ty = &self.ty;
        parse_quote! {
            pgx::datum::sql_entity_graph::aggregate::AggregateType {
                ty_source: stringify!(#ty),
                ty_id: core::any::TypeId::of::<#ty>(),
                full_path: core::any::type_name::<#ty>(),
                name: None,
            }
        }
    }
}

impl ToTokens for AggregateType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ty.to_tokens(tokens)
    }
}

impl Parse for AggregateType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::new(input.parse()?)
    }
}
