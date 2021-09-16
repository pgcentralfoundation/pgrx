use eyre::eyre as eyre_err;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use std::{convert::TryFrom, ops::Deref};
use syn::{
    parse::{Parse, ParseStream},
    Token,
};

#[derive(Debug, Clone)]
pub enum Returning {
    None,
    Type(syn::Type),
    SetOf(syn::TypePath),
    Iterated(Vec<(syn::Type, Option<String>)>),
    /// `pgx_pg_sys::Datum`
    Trigger,
}

impl Returning {
    fn parse_trait_bound(trait_bound: &syn::TraitBound) -> Returning {
        let last_path_segment = trait_bound.path.segments.last().unwrap();
        match last_path_segment.ident.to_string().as_str() {
            "Iterator" => match &last_path_segment.arguments {
                syn::PathArguments::AngleBracketed(args) => match args.args.first().unwrap() {
                    syn::GenericArgument::Binding(binding) => match &binding.ty {
                        syn::Type::Tuple(tuple_type) => Self::parse_type_tuple(tuple_type),
                        syn::Type::Path(path) => Returning::SetOf(path.clone()),
                        syn::Type::Reference(type_ref) => match &*type_ref.elem {
                            syn::Type::Path(path) => Returning::SetOf(path.clone()),
                            _ => unimplemented!("Expected path"),
                        },
                        ty => unimplemented!("Only iters with tuples, got {:?}.", ty),
                    },
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }

    fn parse_type_tuple(type_tuple: &syn::TypeTuple) -> Returning {
        let returns: Vec<(syn::Type, Option<_>)> = type_tuple
            .elems
            .iter()
            .flat_map(|elem| match elem {
                syn::Type::Macro(macro_pat) => {
                    let mac = &macro_pat.mac;
                    let archetype = mac.path.segments.last().unwrap();
                    match archetype.ident.to_string().as_str() {
                        "name" => {
                            let out: NameMacro = mac.parse_body().expect(&*format!("{:?}", mac));
                            Some((out.ty, Some(out.ident)))
                        }
                        _ => unimplemented!("Don't support anything other than name."),
                    }
                }
                ty => Some((ty.clone(), None)),
            })
            .collect();
        Returning::Iterated(returns)
    }

    fn parse_impl_trait(impl_trait: &syn::TypeImplTrait) -> Returning {
        match impl_trait.bounds.first().unwrap() {
            syn::TypeParamBound::Trait(trait_bound) => Self::parse_trait_bound(trait_bound),
            _ => Returning::None,
        }
    }

    fn parse_dyn_trait(dyn_trait: &syn::TypeTraitObject) -> Returning {
        match dyn_trait.bounds.first().unwrap() {
            syn::TypeParamBound::Trait(trait_bound) => Self::parse_trait_bound(trait_bound),
            _ => Returning::None,
        }
    }
}

impl TryFrom<&syn::ReturnType> for Returning {
    type Error = eyre::Error;

    fn try_from(value: &syn::ReturnType) -> Result<Self, Self::Error> {
        Ok(match &value {
            syn::ReturnType::Default => Returning::None,
            syn::ReturnType::Type(_, ty) => match *ty.clone() {
                syn::Type::ImplTrait(impl_trait) => Returning::parse_impl_trait(&impl_trait),
                syn::Type::TraitObject(dyn_trait) => Returning::parse_dyn_trait(&dyn_trait),
                syn::Type::Path(typepath) => {
                    let path = &typepath.path;
                    let mut saw_pg_sys = false;
                    let mut saw_datum = false;
                    let mut saw_option_ident = false;
                    let mut saw_box_ident = false;
                    let mut maybe_inner_impl_trait = None;

                    for segment in &path.segments {
                        let ident_string = segment.ident.to_string();
                        match ident_string.as_str() {
                            "pg_sys" => saw_pg_sys = true,
                            "Datum" => saw_datum = true,
                            "Option" => saw_option_ident = true,
                            "Box" => saw_box_ident = true,
                            _ => (),
                        }
                        if saw_option_ident || saw_box_ident {
                            match &segment.arguments {
                                syn::PathArguments::AngleBracketed(inside_brackets) => {
                                    match inside_brackets.args.first() {
                                        Some(syn::GenericArgument::Type(syn::Type::ImplTrait(
                                            impl_trait,
                                        ))) => {
                                            maybe_inner_impl_trait =
                                                Some(Returning::parse_impl_trait(&impl_trait));
                                        }
                                        Some(syn::GenericArgument::Type(
                                            syn::Type::TraitObject(dyn_trait),
                                        )) => {
                                            maybe_inner_impl_trait =
                                                Some(Returning::parse_dyn_trait(&dyn_trait))
                                        }
                                        _ => (),
                                    }
                                }
                                syn::PathArguments::None | syn::PathArguments::Parenthesized(_) => {
                                    ()
                                }
                            }
                        }
                    }
                    if (saw_datum && saw_pg_sys) || (saw_datum && path.segments.len() == 1) {
                        Returning::Trigger
                    } else if let Some(returning) = maybe_inner_impl_trait {
                        returning
                    } else {
                        let mut static_ty = typepath.clone();
                        for segment in &mut static_ty.path.segments {
                            match &mut segment.arguments {
                                syn::PathArguments::AngleBracketed(ref mut inside_brackets) => {
                                    for mut arg in &mut inside_brackets.args {
                                        match &mut arg {
                                            syn::GenericArgument::Lifetime(ref mut lifetime) => {
                                                lifetime.ident =
                                                    Ident::new("static", Span::call_site())
                                            }
                                            _ => (),
                                        }
                                    }
                                }
                                _ => (),
                            }
                        }
                        Returning::Type(syn::Type::Path(static_ty.clone()))
                    }
                }
                syn::Type::Reference(mut ty_ref) => {
                    if let Some(ref mut lifetime) = &mut ty_ref.lifetime {
                        lifetime.ident = Ident::new("static", Span::call_site());
                    }
                    Returning::Type(syn::Type::Reference(ty_ref))
                }
                syn::Type::Tuple(tup) => {
                    if tup.elems.is_empty() {
                        Returning::Type(ty.deref().clone())
                    } else {
                        Self::parse_type_tuple(&tup)
                    }
                }
                _ => {
                    return Err(eyre_err!(
                        "Got unknown return type: {}",
                        &ty.to_token_stream()
                    ))
                }
            },
        })
    }
}

impl ToTokens for Returning {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let quoted = match self {
            Returning::None => quote! {
                pgx::datum::sql_entity_graph::PgExternReturnEntity::None
            },
            Returning::Type(ty) => {
                let ty_string = ty.to_token_stream().to_string().replace(" ", "");
                quote! {
                    pgx::datum::sql_entity_graph::PgExternReturnEntity::Type {
                        id: TypeId::of::<#ty>(),
                        source: #ty_string,
                        full_path: core::any::type_name::<#ty>(),
                        module_path: {
                            let type_name = core::any::type_name::<#ty>();
                            let mut path_items: Vec<_> = type_name.split("::").collect();
                            let _ = path_items.pop(); // Drop the one we don't want.
                            path_items.join("::")
                        },
                    }
                }
            }
            Returning::SetOf(ty) => {
                let ty_string = ty.to_token_stream().to_string().replace(" ", "");
                quote! {
                    pgx::datum::sql_entity_graph::PgExternReturnEntity::SetOf {
                        id: TypeId::of::<#ty>(),
                        source: #ty_string,
                        full_path: core::any::type_name::<#ty>(),
                        module_path: {
                            let type_name = core::any::type_name::<#ty>();
                            let mut path_items: Vec<_> = type_name.split("::").collect();
                            let _ = path_items.pop(); // Drop the one we don't want.
                            path_items.join("::")
                        }
                    }
                }
            }
            Returning::Iterated(items) => {
                let quoted_items = items
                    .iter()
                    .map(|(ty, name)| {
                        let ty_string = ty.to_token_stream().to_string().replace(" ", "");
                        let name_iter = name.iter();
                        quote! {
                            (
                                TypeId::of::<#ty>(),
                                #ty_string,
                                core::any::type_name::<#ty>(),
                                {
                                    let type_name = core::any::type_name::<#ty>();
                                    let mut path_items: Vec<_> = type_name.split("::").collect();
                                    let _ = path_items.pop(); // Drop the one we don't want.
                                    path_items.join("::")
                                },
                                None#( .unwrap_or(Some(stringify!(#name_iter))) )*,
                            )
                        }
                    })
                    .collect::<Vec<_>>();
                quote! {
                    pgx::datum::sql_entity_graph::PgExternReturnEntity::Iterated(vec![
                        #(#quoted_items),*
                    ])
                }
            }
            Returning::Trigger => quote! {
                pgx::datum::sql_entity_graph::PgExternReturnEntity::Trigger
            },
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NameMacro {
    pub(crate) ident: String,
    comma: Token![,],
    pub(crate) ty: syn::Type,
}

impl Parse for NameMacro {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            ident: input
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
                })?,
            comma: input.parse()?,
            ty: input.parse()?,
        })
    }
}
