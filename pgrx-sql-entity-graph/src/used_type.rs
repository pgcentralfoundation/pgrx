/*!

Type level metadata for Rust to SQL generation.

> Like all of the [`sql_entity_graph`][crate::pgrx_sql_entity_graph] APIs, this is considered **internal**
to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use std::ops::Deref;

use crate::lifetimes::staticize_lifetimes;
use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Token;

use super::metadata::FunctionMetadataTypeEntity;

/// A type, optionally with an overriding composite type name
#[derive(Debug, Clone)]
pub struct UsedType {
    pub original_ty: syn::Type,
    pub resolved_ty: syn::Type,
    /// Set via `composite_type!()`
    pub composite_type: Option<CompositeTypeMacro>,
    /// Set via `VariadicArray` or `variadic!()`
    pub variadic: bool,
    pub default: Option<String>,
    /// Set via the type being an `Option` or a `Result<Option<T>>`.
    pub optional: Option<syn::Type>,
    /// Set via the type being a `Result<T>`
    pub result: bool,
}

impl UsedType {
    pub fn new(ty: syn::Type) -> syn::Result<Self> {
        let original_ty = ty.clone();
        // There are several steps:
        // * Resolve the `default!()` macro
        // * Resolve the `variadic!()` macro
        // * Resolve `composite_type!()`
        // * Anonymize any lifetimes
        // * Resolving any flags for that resolved type so we can not have to do this later.

        // Resolve any `default` macro
        // We do this first as it's **always** in the first position. It's not valid deeper in the type.
        let (resolved_ty, default) = match ty.clone() {
            // default!(..)
            syn::Type::Macro(macro_pat) => {
                let mac = &macro_pat.mac;
                let archetype = mac.path.segments.last().expect("No last segment");
                match archetype.ident.to_string().as_str() {
                    "default" => {
                        let (maybe_resolved_ty, default) = handle_default_macro(mac)?;
                        (maybe_resolved_ty, default)
                    }
                    _ => (syn::Type::Macro(macro_pat), None),
                }
            }
            original => (original, None),
        };

        // Resolve any `variadic` macro
        // We do this first as it's **always** in the first position. It's not valid deeper in the type.
        let resolved_ty = match resolved_ty {
            // variadic!(..)
            syn::Type::Macro(macro_pat) => {
                let mac = &macro_pat.mac;
                let archetype = mac.path.segments.last().expect("No last segment");
                match archetype.ident.to_string().as_str() {
                    "variadic" => {
                        let ty: syn::Type = syn::parse2(mac.tokens.clone())?;
                        syn::parse_quote! { ::pgrx::datum::VariadicArray<'static, #ty>}
                    }
                    _ => syn::Type::Macro(macro_pat),
                }
            }
            original => original,
        };

        // Now, resolve any `composite_type` macro
        let (resolved_ty, composite_type) = match resolved_ty {
            // composite_type!(..)
            syn::Type::Macro(macro_pat) => {
                let mac = &macro_pat.mac;
                let archetype = mac.path.segments.last().expect("No last segment");
                match archetype.ident.to_string().as_str() {
                    "default" => {
                        // If we land here, after already expanding the `default!()` above, the user has written it twice.
                        // This is definitely an issue and we should tell them.
                        return Err(syn::Error::new(
                            mac.span(),
                            "default!(default!()) not supported, use it only once",
                        ))?;
                    }
                    "composite_type" => {
                        let composite_type = Some(handle_composite_type_macro(&mac)?);
                        let ty = syn::parse_quote! {
                            ::pgrx::heap_tuple::PgHeapTuple<'static, ::pgrx::pgbox::AllocatedByRust>
                        };
                        (ty, composite_type)
                    }
                    _ => (syn::Type::Macro(macro_pat), None),
                }
            }
            syn::Type::Path(path) => {
                let segments = &path.path;
                let last = segments
                    .segments
                    .last()
                    .ok_or(syn::Error::new(path.span(), "Could not read last segment of path"))?;

                match last.ident.to_string().as_str() {
                    // Option<composite_type!(..)>
                    // Option<Vec<composite_type!(..)>>
                    // Option<Vec<Option<composite_type!(..)>>>
                    // Option<VariadicArray<composite_type!(..)>>
                    // Option<VariadicArray<Option<composite_type!(..)>>>
                    "Option" => resolve_option_inner(path)?,
                    // Vec<composite_type!(..)>
                    // Vec<Option<composite_type!(..)>>
                    "Vec" => resolve_vec_inner(path)?,
                    // VariadicArray<composite_type!(..)>
                    // VariadicArray<Option<composite_type!(..)>>
                    "VariadicArray" => resolve_variadic_array_inner(path)?,
                    // Array<composite_type!(..)>
                    // Array<Option<composite_type!(..)>>
                    "Array" => resolve_array_inner(path)?,
                    _ => (syn::Type::Path(path), None),
                }
            }
            original => (original, None),
        };

        // In this  step, we go look at the resolved type and determine if it is a variadic, optional, result, etc.
        let (resolved_ty, variadic, optional, result) = match resolved_ty {
            syn::Type::Path(type_path) => {
                let path = &type_path.path;
                let last_segment = path.segments.last().ok_or(syn::Error::new(
                    path.span(),
                    "No last segment found while scanning path",
                ))?;
                let ident_string = last_segment.ident.to_string();
                match ident_string.as_str() {
                    "Result" => {
                        match &last_segment.arguments {
                            syn::PathArguments::AngleBracketed(angle_bracketed) => {
                                match angle_bracketed.args.first().ok_or(syn::Error::new(
                                    angle_bracketed.span(),
                                    "No inner arg for Result<T, E> found",
                                ))? {
                                    syn::GenericArgument::Type(inner_ty) => {
                                        match inner_ty {
                                            // Result<$Type<T>>
                                            syn::Type::Path(ref inner_type_path) => {
                                                let path = &inner_type_path.path;
                                                let last_segment =
                                                    path.segments.last().ok_or(syn::Error::new(
                                                        path.span(),
                                                        "No last segment found while scanning path",
                                                    ))?;
                                                let ident_string = last_segment.ident.to_string();
                                                match ident_string.as_str() {
                                                    "VariadicArray" => (
                                                        syn::Type::Path(type_path.clone()),
                                                        true,
                                                        Some(inner_ty.clone()),
                                                        false,
                                                    ),
                                                    "Option" => (
                                                        syn::Type::Path(type_path.clone()),
                                                        false,
                                                        Some(inner_ty.clone()),
                                                        true,
                                                    ),
                                                    _ => (
                                                        syn::Type::Path(type_path.clone()),
                                                        false,
                                                        None,
                                                        true,
                                                    ),
                                                }
                                            }
                                            // Result<T>
                                            _ => (
                                                syn::Type::Path(type_path.clone()),
                                                false,
                                                None,
                                                true,
                                            ),
                                        }
                                    }
                                    _ => {
                                        return Err(syn::Error::new(
                                            type_path.span().clone(),
                                            "Unexpected Item found inside `Result` (expected Type)",
                                        ))
                                    }
                                }
                            }
                            _ => return Err(syn::Error::new(
                                type_path.span().clone(),
                                "Unexpected Item found inside `Result` (expected Angle Brackets)",
                            )),
                        }
                    }
                    "Option" => {
                        // Option<VariadicArray<T>>
                        match &last_segment.arguments {
                            syn::PathArguments::AngleBracketed(angle_bracketed) => {
                                match angle_bracketed.args.first().ok_or(syn::Error::new(
                                    angle_bracketed.span(),
                                    "No inner arg for Option<T> found",
                                ))? {
                                    syn::GenericArgument::Type(inner_ty) => {
                                        match inner_ty {
                                            // Option<VariadicArray<T>>
                                            syn::Type::Path(ref inner_type_path) => {
                                                let path = &inner_type_path.path;
                                                let last_segment =
                                                    path.segments.last().ok_or(syn::Error::new(
                                                        path.span(),
                                                        "No last segment found while scanning path",
                                                    ))?;
                                                let ident_string = last_segment.ident.to_string();
                                                match ident_string.as_str() {
                                                    // Option<VariadicArray<T>>
                                                    "VariadicArray" => (
                                                        syn::Type::Path(type_path.clone()),
                                                        true,
                                                        Some(inner_ty.clone()),
                                                        false,
                                                    ),
                                                    _ => (
                                                        syn::Type::Path(type_path.clone()),
                                                        false,
                                                        Some(inner_ty.clone()),
                                                        false,
                                                    ),
                                                }
                                            }
                                            // Option<T>
                                            _ => (
                                                syn::Type::Path(type_path.clone()),
                                                false,
                                                Some(inner_ty.clone()),
                                                false,
                                            ),
                                        }
                                    }
                                    // Option<T>
                                    _ => {
                                        return Err(syn::Error::new(
                                            type_path.span().clone(),
                                            "Unexpected Item found inside `Option` (expected Type)",
                                        ))
                                    }
                                }
                            }
                            // Option<T>
                            _ => return Err(syn::Error::new(
                                type_path.span().clone(),
                                "Unexpected Item found inside `Option` (expected Angle Brackets)",
                            )),
                        }
                    }
                    // VariadicArray<T>
                    "VariadicArray" => (syn::Type::Path(type_path), true, None, false),
                    // T
                    _ => (syn::Type::Path(type_path), false, None, false),
                }
            }
            original => (original, false, None, false),
        };

        Ok(Self { original_ty, resolved_ty, optional, result, variadic, default, composite_type })
    }

    pub fn entity_tokens(&self) -> syn::Expr {
        let mut resolved_ty = self.resolved_ty.clone();
        staticize_lifetimes(&mut resolved_ty);
        let resolved_ty_string = resolved_ty.to_token_stream().to_string().replace(" ", "");
        let composite_type = self.composite_type.clone().map(|v| v.expr);
        let composite_type_iter = composite_type.iter();
        let variadic = &self.variadic;
        let optional = &self.optional.is_some();
        let default = (&self.default).iter();

        syn::parse_quote! {
            ::pgrx::pgrx_sql_entity_graph::UsedTypeEntity {
                ty_source: #resolved_ty_string,
                ty_id: core::any::TypeId::of::<#resolved_ty>(),
                full_path: core::any::type_name::<#resolved_ty>(),
                module_path: {
                    let ty_name = core::any::type_name::<#resolved_ty>();
                    let mut path_items: Vec<_> = ty_name.split("::").collect();
                    let _ = path_items.pop(); // Drop the one we don't want.
                    path_items.join("::")
                },
                composite_type: None #( .unwrap_or(Some(#composite_type_iter)) )*,
                variadic: #variadic,
                default:  None #( .unwrap_or(Some(#default)) )*,
                /// Set via the type being an `Option`.
                optional: #optional,
                metadata: {
                    use ::pgrx::pgrx_sql_entity_graph::metadata::PhantomDataExt;
                    let marker: core::marker::PhantomData<#resolved_ty> = core::marker::PhantomData;
                    marker.entity()
                },
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsedTypeEntity {
    pub ty_source: &'static str,
    pub ty_id: core::any::TypeId,
    pub full_path: &'static str,
    pub module_path: String,
    pub composite_type: Option<&'static str>,
    pub variadic: bool,
    pub default: Option<&'static str>,
    /// Set via the type being an `Option`.
    pub optional: bool,
    pub metadata: FunctionMetadataTypeEntity,
}

fn resolve_vec_inner(
    original: syn::TypePath,
) -> syn::Result<(syn::Type, Option<CompositeTypeMacro>)> {
    let segments = &original.path;
    let last = segments
        .segments
        .last()
        .ok_or(syn::Error::new(original.span(), "Could not read last segment of path"))?;

    match &last.arguments {
        syn::PathArguments::AngleBracketed(path_arg) => match path_arg.args.last() {
            Some(syn::GenericArgument::Type(ty)) => match ty.clone() {
                syn::Type::Macro(macro_pat) => {
                    let mac = &macro_pat.mac;
                    let archetype = mac.path.segments.last().expect("No last segment");
                    match archetype.ident.to_string().as_str() {
                        "default" => {
                            return Err(syn::Error::new(mac.span(), "`Vec<default!(T, default)>` not supported, choose `default!(Vec<T>, ident)` instead"))?;
                        }
                        "composite_type" => {
                            let sql = Some(handle_composite_type_macro(mac)?);
                            let ty = syn::parse_quote! {
                                Vec<::pgrx::heap_tuple::PgHeapTuple<'static, ::pgrx::pgbox::AllocatedByRust>>
                            };
                            Ok((ty, sql))
                        }
                        _ => Ok((syn::Type::Path(original), None)),
                    }
                }
                syn::Type::Path(arg_type_path) => {
                    let last = arg_type_path.path.segments.last().ok_or(syn::Error::new(
                        arg_type_path.span(),
                        "No last segment in type path",
                    ))?;
                    match last.ident.to_string().as_str() {
                        "Option" => {
                            let (inner_ty, expr) = resolve_option_inner(arg_type_path)?;
                            let wrapped_ty = syn::parse_quote! {
                                Vec<#inner_ty>
                            };
                            Ok((wrapped_ty, expr))
                        }
                        _ => Ok((syn::Type::Path(original), None)),
                    }
                }
                _ => Ok((syn::Type::Path(original), None)),
            },
            _ => Ok((syn::Type::Path(original), None)),
        },
        _ => Ok((syn::Type::Path(original), None)),
    }
}

fn resolve_variadic_array_inner(
    mut original: syn::TypePath,
) -> syn::Result<(syn::Type, Option<CompositeTypeMacro>)> {
    let original_span = original.span().clone();
    let last = original
        .path
        .segments
        .last_mut()
        .ok_or(syn::Error::new(original_span, "Could not read last segment of path"))?;

    match last.arguments {
        syn::PathArguments::AngleBracketed(ref mut path_arg) => {
            match path_arg.args.first_mut() {
                Some(syn::GenericArgument::Lifetime(lifetime)) => {
                    lifetime.ident = syn::Ident::new("static", lifetime.ident.span())
                }
                _ => path_arg.args.insert(0, syn::parse_quote!('static)),
            };
            match path_arg.args.last() {
                // TODO: Lifetime????
                Some(syn::GenericArgument::Type(ty)) => match ty.clone() {
                    syn::Type::Macro(macro_pat) => {
                        let mac = &macro_pat.mac;
                        let archetype = mac.path.segments.last().expect("No last segment");
                        match archetype.ident.to_string().as_str() {
                            "default" => {
                                return Err(syn::Error::new(mac.span(), "`VariadicArray<default!(T, default)>` not supported, choose `default!(VariadicArray<T>, ident)` instead"))?;
                            }
                            "composite_type" => {
                                let sql = Some(handle_composite_type_macro(mac)?);
                                let ty = syn::parse_quote! {
                                    ::pgrx::datum::VariadicArray<'static, ::pgrx::heap_tuple::PgHeapTuple<'static, ::pgrx::pgbox::AllocatedByRust>>
                                };
                                Ok((ty, sql))
                            }
                            _ => Ok((syn::Type::Path(original), None)),
                        }
                    }
                    syn::Type::Path(arg_type_path) => {
                        let last = arg_type_path.path.segments.last().ok_or(syn::Error::new(
                            arg_type_path.span(),
                            "No last segment in type path",
                        ))?;
                        match last.ident.to_string().as_str() {
                            "Option" => {
                                let (inner_ty, expr) = resolve_option_inner(arg_type_path)?;
                                let wrapped_ty = syn::parse_quote! {
                                    ::pgrx::datum::VariadicArray<'static, #inner_ty>
                                };
                                Ok((wrapped_ty, expr))
                            }
                            _ => Ok((syn::Type::Path(original), None)),
                        }
                    }
                    _ => Ok((syn::Type::Path(original), None)),
                },
                _ => Ok((syn::Type::Path(original), None)),
            }
        }
        _ => Ok((syn::Type::Path(original), None)),
    }
}

fn resolve_array_inner(
    mut original: syn::TypePath,
) -> syn::Result<(syn::Type, Option<CompositeTypeMacro>)> {
    let original_span = original.span().clone();
    let last = original
        .path
        .segments
        .last_mut()
        .ok_or(syn::Error::new(original_span, "Could not read last segment of path"))?;

    match last.arguments {
        syn::PathArguments::AngleBracketed(ref mut path_arg) => {
            match path_arg.args.first_mut() {
                Some(syn::GenericArgument::Lifetime(lifetime)) => {
                    lifetime.ident = syn::Ident::new("static", lifetime.ident.span())
                }
                _ => path_arg.args.insert(0, syn::parse_quote!('static)),
            };
            match path_arg.args.last() {
                Some(syn::GenericArgument::Type(ty)) => match ty.clone() {
                    syn::Type::Macro(macro_pat) => {
                        let mac = &macro_pat.mac;
                        let archetype = mac.path.segments.last().expect("No last segment");
                        match archetype.ident.to_string().as_str() {
                            "default" => {
                                return Err(syn::Error::new(mac.span(), "`VariadicArray<default!(T, default)>` not supported, choose `default!(VariadicArray<T>, ident)` instead"))?;
                            }
                            "composite_type" => {
                                let sql = Some(handle_composite_type_macro(mac)?);
                                let ty = syn::parse_quote! {
                                    ::pgrx::datum::Array<'static, ::pgrx::heap_tuple::PgHeapTuple<'static, ::pgrx::pgbox::AllocatedByRust>>
                                };
                                Ok((ty, sql))
                            }
                            _ => Ok((syn::Type::Path(original), None)),
                        }
                    }
                    syn::Type::Path(arg_type_path) => {
                        let last = arg_type_path.path.segments.last().ok_or(syn::Error::new(
                            arg_type_path.span(),
                            "No last segment in type path",
                        ))?;
                        match last.ident.to_string().as_str() {
                            "Option" => {
                                let (inner_ty, expr) = resolve_option_inner(arg_type_path)?;
                                let wrapped_ty = syn::parse_quote! {
                                    ::pgrx::datum::Array<'static, #inner_ty>
                                };
                                Ok((wrapped_ty, expr))
                            }
                            _ => Ok((syn::Type::Path(original), None)),
                        }
                    }
                    _ => Ok((syn::Type::Path(original), None)),
                },
                _ => Ok((syn::Type::Path(original), None)),
            }
        }
        _ => Ok((syn::Type::Path(original), None)),
    }
}

fn resolve_option_inner(
    original: syn::TypePath,
) -> syn::Result<(syn::Type, Option<CompositeTypeMacro>)> {
    let segments = &original.path;
    let last = segments
        .segments
        .last()
        .ok_or(syn::Error::new(original.span(), "Could not read last segment of path"))?;

    match &last.arguments {
        syn::PathArguments::AngleBracketed(path_arg) => match path_arg.args.first() {
            Some(syn::GenericArgument::Type(ty)) => {
                match ty.clone() {
                    syn::Type::Macro(macro_pat) => {
                        let mac = &macro_pat.mac;
                        let archetype = mac.path.segments.last().expect("No last segment");
                        match archetype.ident.to_string().as_str() {
                            // Option<composite_type!(..)>
                            "composite_type" => {
                                let sql = Some(handle_composite_type_macro(mac)?);
                                let ty = syn::parse_quote! {
                                    Option<::pgrx::heap_tuple::PgHeapTuple<'static, ::pgrx::pgbox::AllocatedByRust>>
                                };
                                Ok((ty, sql))
                            },
                            // Option<default!(composite_type!(..))> isn't valid. If the user wanted the default to be `NULL` they just don't need a default.
                            "default" => return Err(syn::Error::new(mac.span(), "`Option<default!(T, \"my_default\")>` not supported, choose `Option<T>` for a default of `NULL`, or `default!(T, default)` for a non-NULL default"))?,
                            _ => Ok((syn::Type::Path(original), None)),
                        }
                    }
                    syn::Type::Path(arg_type_path) => {
                        let last = arg_type_path.path.segments.last().ok_or(syn::Error::new(
                            arg_type_path.span(),
                            "No last segment in type path",
                        ))?;
                        match last.ident.to_string().as_str() {
                            // Option<Vec<composite_type!(..)>>
                            // Option<Vec<Option<composite_type!(..)>>>
                            "Vec" => {
                                let (inner_ty, expr) = resolve_vec_inner(arg_type_path)?;
                                let wrapped_ty = syn::parse_quote! {
                                    ::std::option::Option<#inner_ty>
                                };
                                Ok((wrapped_ty, expr))
                            }
                            // Option<VariadicArray<composite_type!(..)>>
                            // Option<VariadicArray<Option<composite_type!(..)>>>
                            "VariadicArray" => {
                                let (inner_ty, expr) = resolve_variadic_array_inner(arg_type_path)?;
                                let wrapped_ty = syn::parse_quote! {
                                    ::std::option::Option<#inner_ty>
                                };
                                Ok((wrapped_ty, expr))
                            }
                            // Option<Array<composite_type!(..)>>
                            // Option<Array<Option<composite_type!(..)>>>
                            "Array" => {
                                let (inner_ty, expr) = resolve_array_inner(arg_type_path)?;
                                let wrapped_ty = syn::parse_quote! {
                                    ::std::option::Option<#inner_ty>
                                };
                                Ok((wrapped_ty, expr))
                            }
                            // Option<..>
                            _ => Ok((syn::Type::Path(original), None)),
                        }
                    }
                    _ => Ok((syn::Type::Path(original), None)),
                }
            }
            _ => Ok((syn::Type::Path(original), None)),
        },
        _ => Ok((syn::Type::Path(original), None)),
    }
}

fn handle_composite_type_macro(mac: &syn::Macro) -> syn::Result<CompositeTypeMacro> {
    let out: CompositeTypeMacro = mac.parse_body()?;
    Ok(out)
}

fn handle_default_macro(mac: &syn::Macro) -> syn::Result<(syn::Type, Option<String>)> {
    let out: DefaultMacro = mac.parse_body()?;
    let true_ty = out.ty;
    match out.expr {
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(def), .. }) => {
            let value = def.value();
            Ok((true_ty, Some(value)))
        }
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Float(def), .. }) => {
            let value = def.base10_digits();
            Ok((true_ty, Some(value.to_string())))
        }
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(def), .. }) => {
            let value = def.base10_digits();
            Ok((true_ty, Some(value.to_string())))
        }
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Bool(def), .. }) => {
            let value = def.value();
            Ok((true_ty, Some(value.to_string())))
        }
        syn::Expr::Unary(syn::ExprUnary { op: syn::UnOp::Neg(_), ref expr, .. }) => match &**expr {
            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(def), .. }) => {
                let value = def.base10_digits();
                Ok((true_ty, Some("-".to_owned() + value)))
            }
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    format!("Unrecognized UnaryExpr in `default!()` macro, got: {:?}", out.expr),
                ))
            }
        },
        syn::Expr::Type(syn::ExprType { ref ty, .. }) => match ty.deref() {
            syn::Type::Path(syn::TypePath { path: syn::Path { segments, .. }, .. }) => {
                let last = segments.last().expect("No last segment");
                let last_string = last.ident.to_string();
                if last_string.as_str() == "NULL" {
                    Ok((true_ty, Some(last_string)))
                } else {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        format!(
                            "Unable to parse default value of `default!()` macro, got: {:?}",
                            out.expr
                        ),
                    ));
                }
            }
            _ => {
                return Err(syn::Error::new(
                    Span::call_site(),
                    format!(
                        "Unable to parse default value of `default!()` macro, got: {:?}",
                        out.expr
                    ),
                ))
            }
        },
        syn::Expr::Path(syn::ExprPath { path: syn::Path { ref segments, .. }, .. }) => {
            let last = segments.last().expect("No last segment");
            let last_string = last.ident.to_string();
            if last_string.as_str() == "NULL" {
                Ok((true_ty, Some(last_string)))
            } else {
                return Err(syn::Error::new(
                    Span::call_site(),
                    format!(
                        "Unable to parse default value of `default!()` macro, got: {:?}",
                        out.expr
                    ),
                ));
            }
        }
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                format!("Unable to parse default value of `default!()` macro, got: {:?}", out.expr),
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DefaultMacro {
    ty: syn::Type,
    pub(crate) expr: syn::Expr,
}

impl Parse for DefaultMacro {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let ty = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let expr = input.parse()?;
        Ok(Self { ty, expr })
    }
}

#[derive(Debug, Clone)]
pub struct CompositeTypeMacro {
    #[allow(dead_code)]
    pub(crate) lifetime: Option<syn::Lifetime>,
    pub(crate) expr: syn::Expr,
}

impl Parse for CompositeTypeMacro {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lifetime: Option<syn::Lifetime> = input.parse().ok();
        let _comma: Option<Token![,]> = input.parse().ok();
        let expr = input.parse()?;
        Ok(Self { lifetime, expr })
    }
}
