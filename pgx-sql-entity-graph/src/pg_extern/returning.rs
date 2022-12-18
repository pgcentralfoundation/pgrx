/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[pg_extern]` return value related macro expansion for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::UsedType;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Token;

#[derive(Debug, Clone)]
pub struct ReturningIteratedItem {
    pub used_ty: UsedType,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Returning {
    None,
    Type(UsedType),
    SetOf { ty: UsedType, optional: bool },
    Iterated { tys: Vec<ReturningIteratedItem>, optional: bool },
    // /// Technically we don't ever create this, single triggers have their own macro.
    // Trigger,
}

impl Returning {
    fn parse_type_macro(type_macro: &mut syn::TypeMacro) -> Result<Returning, syn::Error> {
        let mac = &type_macro.mac;
        let opt_archetype = mac.path.segments.last().map(|archetype| archetype.ident.to_string());
        match opt_archetype.as_deref() {
            Some("composite_type") => {
                Ok(Returning::Type(UsedType::new(syn::Type::Macro(type_macro.clone()))?))
            }
            _ => Err(syn::Error::new(
                type_macro.span(),
                "type macros other than `composite_type!` are not yet implemented",
            )),
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
                        let mut saw_option_ident = false;
                        let mut saw_setof_iterator = false;
                        let mut saw_table_iterator = false;

                        for segment in &mut path.segments {
                            let ident_string = segment.ident.to_string();
                            match ident_string.as_str() {
                                "Option" => saw_option_ident = true,
                                "SetOfIterator" => saw_setof_iterator = true,
                                "TableIterator" => saw_table_iterator = true,
                                _ => (),
                            };
                        }
                        if saw_option_ident || saw_setof_iterator || saw_table_iterator {
                            let option_inner_path = if saw_option_ident {
                                match path.segments.last_mut().map(|s| &mut s.arguments) {
                                    Some(syn::PathArguments::AngleBracketed(args)) => {
                                        let args_span = args.span();
                                        match args.args.last_mut() {
                                            Some(syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { qself: _, path }))) => path.clone(),
                                            Some(syn::GenericArgument::Type(_)) => {
                                                let used_ty = UsedType::new(syn::Type::Path(typepath.clone()))?;
                                                return Ok(Returning::Type(used_ty))
                                            },
                                            other => {
                                                return Err(syn::Error::new(
                                                    other.as_ref().map(|s| s.span()).unwrap_or(args_span),
                                                    &format!(
                                                        "Got unexpected generic argument for Option inner: {other:?}"
                                                    ),
                                                ))
                                            }
                                        }
                                    }
                                    other => {
                                        return Err(syn::Error::new(
                                            other.span(),
                                            &format!(
                                                "Got unexpected path argument for Option inner: {other:?}"
                                            ),
                                        ))
                                    }
                                }
                            } else {
                                path.clone()
                            };

                            for segment in &option_inner_path.segments {
                                let ident_string = segment.ident.to_string();
                                match ident_string.as_str() {
                                    "SetOfIterator" => saw_setof_iterator = true,
                                    "TableIterator" => saw_table_iterator = true,
                                    _ => (),
                                };
                            }
                            if saw_setof_iterator {
                                let last_path_segment = option_inner_path.segments.last();
                                let (used_ty, optional) = match &last_path_segment.map(|ps| &ps.arguments) {
                                    Some(syn::PathArguments::AngleBracketed(args)) => {
                                        match args.args.last().unwrap() {
                                            syn::GenericArgument::Type(ty) => {
                                                match &ty {
                                                    syn::Type::Path(path) => {
                                                        (UsedType::new(syn::Type::Path(path.clone()))?, saw_option_ident)
                                                    }
                                                    syn::Type::Macro(type_macro) => {
                                                        (UsedType::new(syn::Type::Macro(type_macro.clone()),)?, saw_option_ident)
                                                    },
                                                    reference @ syn::Type::Reference(_) => {
                                                        (UsedType::new((*reference).clone(),)?, saw_option_ident)
                                                    },
                                                    ty => return Err(syn::Error::new(
                                                        ty.span(),
                                                        "SetOf Iterator must have an item",
                                                    )),
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
                                            other.map(|s| s.span()).unwrap_or_else(proc_macro2::Span::call_site),
                                            &format!(
                                                "Got unexpected path argument for SetOfIterator: {other:?}"
                                            ),
                                        ))
                                    }
                                };
                                Ok(Returning::SetOf { ty: used_ty, optional })
                            } else if saw_table_iterator {
                                let iterator_path = if saw_option_ident {
                                    let inner_path =
                                        match &mut path.segments.last_mut().unwrap().arguments {
                                            syn::PathArguments::AngleBracketed(args) => {
                                                match args.args.last_mut().unwrap() {
                                                    syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { qself: _, path })) => path,
                                                    other => {
                                                        return Err(syn::Error::new(
                                                            other.span(),
                                                            &format!(
                                                                "Got unexpected generic argument for Option inner: {other:?}"
                                                            ),
                                                        ))
                                                    }
                                                }
                                            },
                                            other => {
                                                return Err(syn::Error::new(
                                                    other.span(),
                                                    &format!(
                                                        "Got unexpected path argument for Option inner: {other:?}"
                                                    ),
                                                ))
                                            }
                                        };
                                    inner_path
                                } else {
                                    path
                                };
                                let last_path_segment = iterator_path.segments.last_mut().unwrap();
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
                                                            let iterated_item =
                                                                ReturningIteratedItem {
                                                                    name: None,
                                                                    used_ty: UsedType::new(
                                                                        syn::Type::Path(
                                                                            path.clone(),
                                                                        ),
                                                                    )?,
                                                                };
                                                            iterated_items.push(iterated_item);
                                                        }
                                                        syn::Type::Macro(type_macro) => {
                                                            let mac = &type_macro.mac;
                                                            let archetype =
                                                                mac.path.segments.last().unwrap();
                                                            match archetype
                                                                .ident
                                                                .to_string()
                                                                .as_str()
                                                            {
                                                                "name" => {
                                                                    let out: NameMacro =
                                                                        mac.parse_body()?;
                                                                    let iterated_item =
                                                                        ReturningIteratedItem {
                                                                            name: Some(out.ident),
                                                                            used_ty: out.used_ty,
                                                                        };
                                                                    iterated_items
                                                                        .push(iterated_item)
                                                                }
                                                                _ => {
                                                                    let iterated_item =
                                                                        ReturningIteratedItem {
                                                                            name: None,
                                                                            used_ty: UsedType::new(
                                                                                syn::Type::Macro(
                                                                                    type_macro
                                                                                        .clone(),
                                                                                ),
                                                                            )?,
                                                                        };
                                                                    iterated_items
                                                                        .push(iterated_item);
                                                                }
                                                            }
                                                        }
                                                        reference @ syn::Type::Reference(_) => {
                                                            let iterated_item =
                                                                ReturningIteratedItem {
                                                                    name: None,
                                                                    used_ty: UsedType::new(
                                                                        (*reference).clone(),
                                                                    )?,
                                                                };
                                                            iterated_items.push(iterated_item);
                                                        }
                                                        ty => {
                                                            return Err(syn::Error::new(
                                                                ty.span(),
                                                                "Table Iterator must have an item",
                                                            ));
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
                                Ok(Returning::Iterated {
                                    tys: iterated_items,
                                    optional: saw_option_ident,
                                })
                            } else {
                                let used_ty = UsedType::new(syn::Type::Path(typepath.clone()))?;
                                Ok(Returning::Type(used_ty))
                            }
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
                ::pgx::pgx_sql_entity_graph::PgExternReturnEntity::None
            },
            Returning::Type(used_ty) => {
                let used_ty_entity_tokens = used_ty.entity_tokens();
                quote! {
                    ::pgx::pgx_sql_entity_graph::PgExternReturnEntity::Type {
                        ty: #used_ty_entity_tokens,
                    }
                }
            }
            Returning::SetOf { ty: used_ty, optional } => {
                let used_ty_entity_tokens = used_ty.entity_tokens();
                quote! {
                    ::pgx::pgx_sql_entity_graph::PgExternReturnEntity::SetOf {
                        ty: #used_ty_entity_tokens,
                        optional: #optional,
                    }
                }
            }
            Returning::Iterated { tys: items, optional } => {
                let quoted_items = items
                    .iter()
                    .map(|ReturningIteratedItem { used_ty, name }| {
                        let name_iter = name.iter();
                        let used_ty_entity_tokens = used_ty.entity_tokens();
                        quote! {
                            ::pgx::pgx_sql_entity_graph::PgExternReturnEntityIteratedItem {
                                ty: #used_ty_entity_tokens,
                                name: None #( .unwrap_or(Some(stringify!(#name_iter))) )*,
                            }
                        }
                    })
                    .collect::<Vec<_>>();
                quote! {
                    ::pgx::pgx_sql_entity_graph::PgExternReturnEntity::Iterated {
                        tys: vec![
                            #(#quoted_items),*
                        ],
                        optional: #optional,
                    }
                }
            }
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug, Clone)]
pub struct NameMacro {
    pub ident: String,
    pub used_ty: UsedType,
}

impl Parse for NameMacro {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ident = input
            .parse::<syn::Ident>()
            .map(|v| v.to_string())
            // Avoid making folks unable to use rust keywords.
            .or_else(|_| input.parse::<syn::Token![type]>().map(|_| String::from("type")))
            .or_else(|_| input.parse::<syn::Token![mod]>().map(|_| String::from("mod")))
            .or_else(|_| input.parse::<syn::Token![extern]>().map(|_| String::from("extern")))
            .or_else(|_| input.parse::<syn::Token![async]>().map(|_| String::from("async")))
            .or_else(|_| input.parse::<syn::Token![crate]>().map(|_| String::from("crate")))
            .or_else(|_| input.parse::<syn::Token![use]>().map(|_| String::from("use")))?;
        let _comma: Token![,] = input.parse()?;
        let ty: syn::Type = input.parse()?;

        let used_ty = UsedType::new(ty)?;

        Ok(Self { ident, used_ty })
    }
}
