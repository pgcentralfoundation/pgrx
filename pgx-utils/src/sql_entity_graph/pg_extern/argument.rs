use std::ops::Deref;

use crate::anonymonize_lifetimes;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, FnArg, Pat, Token,
};

/// A parsed `#[pg_extern]` argument.
///
/// It is created during [`PgExtern`](pgx_utils::sql_entity_graph::PgExtern) parsing.
#[derive(Debug, Clone)]
pub struct Argument {
    pat: syn::Ident,
    ty: syn::Type,
    default: Option<String>,
}

impl Argument {
    pub fn build(value: FnArg) -> Result<Option<Self>, syn::Error> {
        match value {
            syn::FnArg::Typed(pat) => Self::build_from_pat_type(pat),
            _ => Err(syn::Error::new(Span::call_site(), "Unable to parse FnArg")),
        }
    }

    pub fn build_from_pat_type(value: syn::PatType) -> Result<Option<Self>, syn::Error> {
        let mut true_ty = *value.ty.clone();
        anonymonize_lifetimes(&mut true_ty);

        let identifier = match *value.pat {
            Pat::Ident(ref p) => p.ident.clone(),
            Pat::Reference(ref p_ref) => match *p_ref.pat {
                Pat::Ident(ref inner_ident) => inner_ident.ident.clone(),
                _ => return Err(syn::Error::new(Span::call_site(), "Unable to parse FnArg")),
            },
            _ => return Err(syn::Error::new(Span::call_site(), "Unable to parse FnArg")),
        };
        let default = match value.ty.as_ref() {
            syn::Type::Macro(macro_pat) => {
                let mac = &macro_pat.mac;
                let archetype = mac.path.segments.last().expect("No last segment");
                let (maybe_new_true_ty, default_value) =
                    handle_default(true_ty.clone(), archetype, mac)?;
                true_ty = maybe_new_true_ty;
                default_value
            }
            syn::Type::Path(ref path) => {
                let segments = &path.path;
                let mut default = None;
                for segment in &segments.segments {
                    if segment.ident.to_string().ends_with("Option") {
                        match &segment.arguments {
                            syn::PathArguments::AngleBracketed(path_arg) => {
                                match path_arg.args.first() {
                                    Some(syn::GenericArgument::Type(syn::Type::Macro(
                                        macro_pat,
                                    ))) => {
                                        let mac = &macro_pat.mac;
                                        let archetype =
                                            mac.path.segments.last().expect("No last segment");
                                        let (inner_type, default_value) =
                                            handle_default(true_ty.clone(), archetype, mac)?;
                                        true_ty = parse_quote! { Option<#inner_type> };
                                        default = default_value;
                                    }
                                    _ => (),
                                }
                            }
                            _ => continue,
                        }
                    }
                }
                default
            }
            _ => None,
        };

        // We special case ignore `*mut pg_sys::FunctionCallInfoData`
        match true_ty {
            syn::Type::Reference(ref mut ty_ref) => {
                if let Some(ref mut lifetime) = &mut ty_ref.lifetime {
                    lifetime.ident = syn::Ident::new("static", Span::call_site());
                }
            }
            syn::Type::Path(ref mut path) => {
                let segments = &mut path.path;
                let mut saw_pg_sys = false;
                let mut saw_functioncallinfobasedata = false;

                for segment in &mut segments.segments {
                    let ident_string = segment.ident.to_string();
                    match ident_string.as_str() {
                        "pg_sys" => saw_pg_sys = true,
                        "FunctionCallInfo" => saw_functioncallinfobasedata = true,
                        _ => (),
                    }
                }
                if (saw_pg_sys && saw_functioncallinfobasedata)
                    || (saw_functioncallinfobasedata && segments.segments.len() == 1)
                {
                    return Ok(None);
                } else {
                    for segment in &mut path.path.segments {
                        match &mut segment.arguments {
                            syn::PathArguments::AngleBracketed(ref mut inside_brackets) => {
                                for mut arg in &mut inside_brackets.args {
                                    match &mut arg {
                                        syn::GenericArgument::Lifetime(ref mut lifetime) => {
                                            lifetime.ident =
                                                syn::Ident::new("static", Span::call_site())
                                        }
                                        _ => (),
                                    }
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
            syn::Type::Ptr(ref ptr) => match *ptr.elem {
                syn::Type::Path(ref path) => {
                    let segments = &path.path;
                    let mut saw_pg_sys = false;
                    let mut saw_functioncallinfobasedata = false;
                    for segment in &segments.segments {
                        if segment.ident.to_string() == "pg_sys" {
                            saw_pg_sys = true;
                        }
                        if segment.ident.to_string() == "FunctionCallInfo" {
                            saw_functioncallinfobasedata = true;
                        }
                    }
                    if (saw_pg_sys && saw_functioncallinfobasedata)
                        || (saw_functioncallinfobasedata && segments.segments.len() == 1)
                    {
                        // It's a FunctionCallInfoBaseData, skipping
                        return Ok(None);
                    }
                }
                _ => (),
            },
            _ => (),
        };

        Ok(Some(Argument {
            pat: identifier,
            ty: true_ty,
            default,
        }))
    }
}

fn handle_default(
    ty: syn::Type,
    archetype: &syn::PathSegment,
    mac: &syn::Macro,
) -> syn::Result<(syn::Type, Option<String>)> {
    match archetype.ident.to_string().as_str() {
        "default" => {
            let out: DefaultMacro = mac.parse_body()?;
            let true_ty = out.ty;
            match out.expr {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(def),
                    ..
                }) => {
                    let value = def.value();
                    Ok((true_ty, Some(value)))
                }
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Float(def),
                    ..
                }) => {
                    let value = def.base10_digits();
                    Ok((true_ty, Some(value.to_string())))
                }
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(def),
                    ..
                }) => {
                    let value = def.base10_digits();
                    Ok((true_ty, Some(value.to_string())))
                }
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Bool(def),
                    ..
                }) => {
                    let value = def.value();
                    Ok((true_ty, Some(value.to_string())))
                }
                syn::Expr::Type(syn::ExprType { ref ty, .. }) => match ty.deref() {
                    syn::Type::Path(syn::TypePath {
                        path: syn::Path { segments, .. },
                        ..
                    }) => {
                        let last = segments.last().expect("No last segment");
                        let last_string = last.ident.to_string();
                        if last_string.as_str() == "NULL" {
                            Ok((true_ty, Some(last_string)))
                        } else {
                            return Err(syn::Error::new(Span::call_site(), format!("Unable to parse default value of `default!()` macro, got: {:?}", out.expr)));
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
                syn::Expr::Path(syn::ExprPath {
                    path: syn::Path { ref segments, .. },
                    ..
                }) => {
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
            }
        }
        _ => Ok((ty, None)),
    }
}

impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut found_optional = false;
        let mut found_variadic = false;
        let pat = &self.pat;
        let default = self.default.iter();
        let mut ty = self.ty.clone();
        anonymonize_lifetimes(&mut ty);

        match ty {
            syn::Type::Path(ref type_path) => {
                let path = &type_path.path;
                for segment in &path.segments {
                    let ident_string = segment.ident.to_string();
                    match ident_string.as_str() {
                        "Option" => found_optional = true,
                        "VariadicArray" => found_variadic = true,
                        _ => (),
                    }
                }
            }
            syn::Type::Macro(ref type_macro) => {
                let path = &type_macro.mac.path;
                for segment in &path.segments {
                    let ident_string = segment.ident.to_string();
                    match ident_string.as_str() {
                        "variadic" => found_variadic = true,
                        _ => (),
                    }
                }
            }
            _ => (),
        };
        let ty_string = ty.to_token_stream().to_string().replace(" ", "");

        let quoted = quote! {
            pgx::datum::sql_entity_graph::PgExternArgumentEntity {
                pattern: stringify!(#pat),
                ty_source: #ty_string,
                ty_id: TypeId::of::<#ty>(),
                full_path: core::any::type_name::<#ty>(),
                module_path: {
                    let ty_name = core::any::type_name::<#ty>();
                    let mut path_items: Vec<_> = ty_name.split("::").collect();
                    let _ = path_items.pop(); // Drop the one we don't want.
                    path_items.join("::")
                },
                is_optional: #found_optional,
                is_variadic: #found_variadic,
                default: None#( .unwrap_or(Some(#default)) )*,
            }
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DefaultMacro {
    ty: syn::Type,
    comma: Token![,],
    pub(crate) expr: syn::Expr,
}

impl Parse for DefaultMacro {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            ty: input.parse()?,
            comma: input.parse()?,
            expr: input.parse()?,
        })
    }
}
