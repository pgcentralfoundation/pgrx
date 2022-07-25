/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use crate::sql_entity_graph::UsedType;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::TryFrom;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Token,
};

#[derive(Debug, Clone)]
pub struct ReturningIteratedItem {
    used_ty: UsedType,
    name: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Returning {
    None,
    Type(UsedType),
    SetOf(UsedType),
    Iterated(Vec<ReturningIteratedItem>),
    /// `pgx_pg_sys::Datum`
    Trigger,
}

impl Returning {
    fn parse_type_macro(type_macro: &mut syn::TypeMacro) -> Result<Returning, syn::Error> {
        let mac = &type_macro.mac;
        let archetype = mac.path.segments.last().unwrap();
        match archetype.ident.to_string().as_str() {
            "composite_type" => Ok(Returning::Type(UsedType::new(syn::Type::Macro(
                type_macro.clone(),
            ))?)),
            _ => unimplemented!("Don't support anything other than `composite_type!()`"),
        }
    }
}

impl TryFrom<&syn::ReturnType> for Returning {
    type Error = syn::Error;

    fn try_from(value: &syn::ReturnType) -> Result<Self, Self::Error> {
        match &value {
            syn::ReturnType::Default => Ok(Returning::None),
            syn::ReturnType::Type(_, ty) => {
                let mut ty = *ty.clone();

                match ty {
                    syn::Type::Path(mut typepath) => {
                        let path = &mut typepath.path;
                        let mut saw_pg_sys = false;
                        let mut saw_datum_ident = false;
                        let mut saw_option_ident = false;
                        let mut saw_box_ident = false;
                        let mut saw_setof_iterator = false;
                        let mut saw_table_iterator = false;

                        for segment in &mut path.segments {
                            let ident_string = segment.ident.to_string();
                            match ident_string.as_str() {
                                "pg_sys" => saw_pg_sys = true,
                                "Datum" => saw_datum_ident = true,
                                "Option" => saw_option_ident = true,
                                "Box" => saw_box_ident = true,
                                "SetOfIterator" => saw_setof_iterator = true,
                                "TableIterator" => saw_table_iterator = true,
                                _ => (),
                            };
                        }
                        if (saw_datum_ident && saw_pg_sys)
                            || (saw_datum_ident && path.segments.len() == 1)
                        {
                            Ok(Returning::Trigger)
                        } else if saw_setof_iterator {
                            let last_path_segment = path.segments.last_mut().unwrap();
                            let used_ty = match &mut last_path_segment.arguments {
                                syn::PathArguments::AngleBracketed(args) => {
                                    match args.args.last_mut().unwrap() {
                                        syn::GenericArgument::Type(ty) => {
                                            match &ty {
                                                syn::Type::Path(path) => {
                                                    UsedType::new(syn::Type::Path(path.clone()))?
                                                }
                                                syn::Type::Macro(type_macro) => UsedType::new(
                                                    syn::Type::Macro(type_macro.clone()),
                                                )?,
                                                syn::Type::Reference(type_ref) => {
                                                    match &*type_ref.elem {
                                                        syn::Type::Path(path) => UsedType::new(
                                                            syn::Type::Path(path.clone()),
                                                        )?,
                                                        _ => unimplemented!("Expected path"),
                                                    }
                                                }
                                                ty => {
                                                    unimplemented!("SetOf Iterator must have an item, got: {ty:?}")
                                                }
                                            }
                                        }
                                        other => {
                                            return Err(syn::Error::new(
                                                other.span(),
                                                &format!(
                                                    "Got unexpected generic argument for SetOfIterator: {other:?}"
                                                ),
                                            ))
                                        }
                                    }
                                }
                                other => {
                                    return Err(syn::Error::new(
                                        other.span(),
                                        &format!(
                                        "Got unexpected path argument for SetOfIterator: {other:?}"
                                    ),
                                    ))
                                }
                            };
                            Ok(Returning::SetOf(used_ty))
                        } else if saw_table_iterator {
                            let last_path_segment = path.segments.last_mut().unwrap();
                            let mut iterated_items = vec![];
                            match &mut last_path_segment.arguments {
                                syn::PathArguments::AngleBracketed(args) => {
                                    match args.args.last_mut().unwrap() {
                                        syn::GenericArgument::Type(syn::Type::Tuple(
                                            type_tuple,
                                        )) => {
                                            for elem in &type_tuple.elems {
                                                match &elem {
                                                    syn::Type::Path(path) => {
                                                        let iterated_item = ReturningIteratedItem {
                                                            name: None,
                                                            used_ty: UsedType::new(
                                                                syn::Type::Path(path.clone()),
                                                            )?,
                                                        };
                                                        iterated_items.push(iterated_item);
                                                    }
                                                    syn::Type::Macro(type_macro) => {
                                                        let mac = &type_macro.mac;
                                                        let archetype =
                                                            mac.path.segments.last().unwrap();
                                                        match archetype.ident.to_string().as_str() {
                                                            "name" => {
                                                                let out: NameMacro =
                                                                    mac.parse_body()?;
                                                                let iterated_item =
                                                                    ReturningIteratedItem {
                                                                        name: Some(out.ident),
                                                                        used_ty: out.used_ty,
                                                                    };
                                                                iterated_items.push(iterated_item)
                                                            }
                                                            _ => {
                                                                let iterated_item =
                                                                    ReturningIteratedItem {
                                                                        name: None,
                                                                        used_ty: UsedType::new(
                                                                            syn::Type::Macro(
                                                                                type_macro.clone(),
                                                                            ),
                                                                        )?,
                                                                    };
                                                                iterated_items.push(iterated_item);
                                                            }
                                                        }
                                                    }
                                                    syn::Type::Reference(type_ref) => {
                                                        match &*type_ref.elem {
                                                            syn::Type::Path(path) => {
                                                                let iterated_item =
                                                                    ReturningIteratedItem {
                                                                        name: None,
                                                                        used_ty: UsedType::new(
                                                                            syn::Type::Reference(
                                                                                type_ref.clone(),
                                                                            ),
                                                                        )?,
                                                                    };
                                                                iterated_items.push(iterated_item);
                                                            }
                                                            _ => unimplemented!("Expected path"),
                                                        }
                                                    }
                                                    ty => {
                                                        unimplemented!("Table Iterator must have an item, got: {ty:?}")
                                                    }
                                                };
                                            }
                                        }
                                        syn::GenericArgument::Lifetime(_) => (),
                                        other => {
                                            return Err(syn::Error::new(
                                                other.span(),
                                                &format!(
                                                    "Got unexpected generic argument: {other:?}"
                                                ),
                                            ))
                                        }
                                    };
                                }
                                other => {
                                    return Err(syn::Error::new(
                                        other.span(),
                                        &format!("Got unexpected path argument: {other:?}"),
                                    ))
                                }
                            };
                            Ok(Returning::Iterated(iterated_items))
                        } else {
                            let used_ty = UsedType::new(syn::Type::Path(typepath.clone()))?;
                            Ok(Returning::Type(used_ty))
                        }
                    }
                    syn::Type::Reference(ty_ref) => {
                        let used_ty = UsedType::new(syn::Type::Reference(ty_ref.clone()))?;
                        Ok(Returning::Type(used_ty))
                    }
                    syn::Type::Macro(ref mut type_macro) => Self::parse_type_macro(type_macro),
                    syn::Type::Paren(ref mut type_paren) => match &mut *type_paren.elem {
                        syn::Type::Macro(ref mut type_macro) => Self::parse_type_macro(type_macro),
                        other => {
                            return Err(syn::Error::new(
                                other.span(),
                                &format!("Got unknown return type: {type_paren:?}"),
                            ))
                        }
                    },
                    other => {
                        return Err(syn::Error::new(
                            other.span(),
                            &format!("Got unknown return type: {other:?}"),
                        ))
                    }
                }
            }
        }
    }
}

impl ToTokens for Returning {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let quoted = match self {
            Returning::None => quote! {
                ::pgx::utils::sql_entity_graph::PgExternReturnEntity::None
            },
            Returning::Type(used_ty) => {
                let used_ty_entity_tokens = used_ty.entity_tokens();
                quote! {
                    ::pgx::utils::sql_entity_graph::PgExternReturnEntity::Type {
                        ty: #used_ty_entity_tokens,
                    }
                }
            }
            Returning::SetOf(used_ty) => {
                let used_ty_entity_tokens = used_ty.entity_tokens();
                quote! {
                    ::pgx::utils::sql_entity_graph::PgExternReturnEntity::SetOf {
                        ty: #used_ty_entity_tokens,
                    }
                }
            }
            Returning::Iterated(items) => {
                let quoted_items = items
                    .iter()
                    .map(|ReturningIteratedItem { used_ty, name }| {
                        let name_iter = name.iter();
                        let used_ty_entity_tokens = used_ty.entity_tokens();
                        quote! {
                            ::pgx::utils::sql_entity_graph::PgExternReturnEntityIteratedItem {
                                ty: #used_ty_entity_tokens,
                                name: None #( .unwrap_or(Some(stringify!(#name_iter))) )*,
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                quote! {
                    ::pgx::utils::sql_entity_graph::PgExternReturnEntity::Iterated(vec![
                        #(#quoted_items),*
                    ])
                }
            }
            Returning::Trigger => quote! {
                ::pgx::utils::sql_entity_graph::PgExternReturnEntity::Trigger
            },
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug, Clone)]
pub struct NameMacro {
    pub(crate) ident: String,
    pub(crate) used_ty: UsedType,
}

impl Parse for NameMacro {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident = input
            .parse::<syn::Ident>()
            .map(|v| v.to_string())
            // Avoid making folks unable to use rust keywords.
            .or_else(|_| {
                input
                    .parse::<syn::Token![type]>()
                    .map(|_| String::from("type"))
            })
            .or_else(|_| {
                input
                    .parse::<syn::Token![mod]>()
                    .map(|_| String::from("mod"))
            })
            .or_else(|_| {
                input
                    .parse::<syn::Token![extern]>()
                    .map(|_| String::from("extern"))
            })
            .or_else(|_| {
                input
                    .parse::<syn::Token![async]>()
                    .map(|_| String::from("async"))
            })
            .or_else(|_| {
                input
                    .parse::<syn::Token![crate]>()
                    .map(|_| String::from("crate"))
            })
            .or_else(|_| {
                input
                    .parse::<syn::Token![use]>()
                    .map(|_| String::from("use"))
            })?;
        let _comma: Token![,] = input.parse()?;
        let ty: syn::Type = input.parse()?;

        let used_ty = UsedType::new(ty)?;

        Ok(Self { ident, used_ty })
    }
}
