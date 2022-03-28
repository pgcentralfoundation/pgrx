/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use super::get_pgx_attr_macro;
use crate::sql_entity_graph::pg_extern::NameMacro;

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, Expr, Type,
};

#[derive(Debug, Clone)]
pub struct MaybeNamedVariadicTypeList {
    pub found: Vec<MaybeNamedVariadicType>,
    pub original: syn::Type,
}

impl MaybeNamedVariadicTypeList {
    pub fn new(maybe_type_list: syn::Type) -> Result<Self, syn::Error> {
        match &maybe_type_list {
            Type::Tuple(tuple) => {
                let mut coll = Vec::new();
                for elem in &tuple.elems {
                    let parsed_elem = MaybeNamedVariadicType::new(elem.clone())?;
                    coll.push(parsed_elem);
                }
                Ok(Self {
                    found: coll,
                    original: maybe_type_list,
                })
            }
            ty => Ok(Self {
                found: vec![MaybeNamedVariadicType::new(ty.clone())?],
                original: maybe_type_list,
            }),
        }
    }

    pub fn entity_tokens(&self) -> Expr {
        let found = self.found.iter().map(|x| x.entity_tokens());
        parse_quote! {
            vec![#(#found),*]
        }
    }
}

impl Parse for MaybeNamedVariadicTypeList {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::new(input.parse()?)
    }
}

impl ToTokens for MaybeNamedVariadicTypeList {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.original.to_tokens(tokens)
    }
}

#[derive(Debug, Clone)]
pub struct MaybeNamedVariadicType {
    pub ty: Type,
    /// The name, if it exists.
    pub name: Option<String>,
    /// The inner of a variadic, if it exists.
    pub variadic_ty: Option<Type>,
}

impl MaybeNamedVariadicType {
    pub fn new(ty: syn::Type) -> Result<Self, syn::Error> {
        let name_inner = get_pgx_attr_macro("name", &ty);

        let (name, variadic_ty, ty) = match name_inner {
            Some(name_inner) => {
                let name_macro: NameMacro =
                    syn::parse2(name_inner).expect("Could not parse `name!()` macro");
                let variadic = get_pgx_attr_macro("variadic", &name_macro.ty);
                // Since `pg_extern` doesn't take `name!()` we unwrap it.
                (Some(name_macro.ident), variadic, name_macro.ty)
            }
            None => (None, get_pgx_attr_macro("variadic", &ty), ty),
        };

        let variadic_ty = if let Some(variadic_ty) = variadic_ty {
            Some(syn::parse2(variadic_ty).expect("Could not parse `variadic!()` macro"))
        } else {
            None
        };

        let retval = Self {
            ty,
            variadic_ty,
            name,
        };
        Ok(retval)
    }

    fn entity_tokens(&self) -> Expr {
        let ty = self.variadic_ty.as_ref().unwrap_or(&self.ty);
        let ty_string = ty.to_token_stream().to_string().replace(" ", "");
        let variadic = self.variadic_ty.is_some();
        let name = self.name.iter();
        parse_quote! {
            ::pgx::utils::sql_entity_graph::MaybeVariadicAggregateTypeEntity {
                agg_ty: ::pgx::utils::sql_entity_graph::AggregateTypeEntity {
                    ty_source: #ty_string,
                    ty_id: core::any::TypeId::of::<#ty>(),
                    full_path: core::any::type_name::<#ty>(),
                    name: None #( .unwrap_or(Some(#name)) )*,
                },
                variadic: #variadic,
            }
        }
    }
}

impl ToTokens for MaybeNamedVariadicType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.ty.to_tokens(tokens)
    }
}

impl Parse for MaybeNamedVariadicType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Self::new(input.parse()?)
    }
}

#[cfg(test)]
mod tests {
    use super::MaybeNamedVariadicTypeList;
    use eyre::{eyre as eyre_err, Result};
    use syn::parse_quote;

    #[test]
    fn solo() -> Result<()> {
        let tokens: syn::Type = parse_quote! {
            i32
        };
        // It should not error, as it's valid.
        let list = MaybeNamedVariadicTypeList::new(tokens);
        assert!(list.is_ok());
        let list = list.unwrap();
        let found = &list.found[0];
        let found_string = match &found.ty {
            syn::Type::Path(ty_path) => ty_path.path.segments.last().unwrap().ident.to_string(),
            _ => return Err(eyre_err!("Wrong found.ty")),
        };
        assert_eq!(found_string, "i32");
        Ok(())
    }

    #[test]
    fn list() -> Result<()> {
        let tokens: syn::Type = parse_quote! {
            (i32, i8)
        };
        // It should not error, as it's valid.
        let list = MaybeNamedVariadicTypeList::new(tokens);
        assert!(list.is_ok());
        let list = list.unwrap();
        let first = &list.found[0];
        let first_string = match &first.ty {
            syn::Type::Path(ty_path) => ty_path.path.segments.last().unwrap().ident.to_string(),
            _ => return Err(eyre_err!("Wrong first.ty: {:?}", first)),
        };
        assert_eq!(first_string, "i32");

        let second = &list.found[1];
        let second_string = match &second.ty {
            syn::Type::Path(ty_path) => ty_path.path.segments.last().unwrap().ident.to_string(),
            _ => return Err(eyre_err!("Wrong second.ty: {:?}", second)),
        };
        assert_eq!(second_string, "i8");
        Ok(())
    }

    #[test]
    fn list_variadic_with_path() -> Result<()> {
        let tokens: syn::Type = parse_quote! {
            (i32, pgx::variadic!(i8))
        };
        // It should not error, as it's valid.
        let list = MaybeNamedVariadicTypeList::new(tokens);
        assert!(list.is_ok());
        let list = list.unwrap();
        let first = &list.found[0];
        let first_string = match &first.ty {
            syn::Type::Path(ty_path) => ty_path.path.segments.last().unwrap().ident.to_string(),
            _ => return Err(eyre_err!("Wrong first.ty: {:?}", first)),
        };
        assert_eq!(first_string, "i32");

        let second = &list.found[1];
        let second_string = match &second.variadic_ty {
            Some(syn::Type::Path(ty_path)) => {
                ty_path.path.segments.last().unwrap().ident.to_string()
            }
            _ => return Err(eyre_err!("Wrong second.variadic_ty: {:?}", second)),
        };
        assert_eq!(second_string, "i8");
        Ok(())
    }

    #[test]
    fn list_variadic() -> Result<()> {
        let tokens: syn::Type = parse_quote! {
            (i32, variadic!(i8))
        };
        // It should not error, as it's valid.
        let list = MaybeNamedVariadicTypeList::new(tokens);
        assert!(list.is_ok());
        let list = list.unwrap();
        let first = &list.found[0];
        let first_string = match &first.ty {
            syn::Type::Path(ty_path) => ty_path.path.segments.last().unwrap().ident.to_string(),
            _ => return Err(eyre_err!("Wrong first.ty: {:?}", first)),
        };
        assert_eq!(first_string, "i32");

        let second = &list.found[1];
        let second_string = match &second.variadic_ty {
            Some(syn::Type::Path(ty_path)) => {
                ty_path.path.segments.last().unwrap().ident.to_string()
            }
            _ => return Err(eyre_err!("Wrong second.variadic_ty: {:?}", second)),
        };
        assert_eq!(second_string, "i8");
        Ok(())
    }
}
