/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use crate::{anonymonize_lifetimes, sql_entity_graph::pg_extern::resolve_ty};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    FnArg, Pat,
};

/// A parsed `#[pg_extern]` argument.
///
/// It is created during [`PgExtern`](crate::sql_entity_graph::PgExtern) parsing.
#[derive(Debug, Clone)]
pub struct PgExternArgument {
    pat: syn::Ident,
    ty: syn::Type,
    /// Set via `composite_type!()`
    composite_type: Option<syn::Expr>,
    /// Set via `default!()`
    default: Option<String>,
    /// Set via `variadic!()`
    variadic: bool,
    optional: bool,
}

impl PgExternArgument {
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

        let (mut true_ty, optional, variadic, default, sql) = resolve_ty(*value.ty)?;

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

        Ok(Some(PgExternArgument {
            pat: identifier,
            ty: true_ty,
            composite_type: sql,
            default,
            variadic,
            optional,
        }))
    }
}

impl ToTokens for PgExternArgument {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let is_optional = self.optional;
        let is_variadic = self.variadic;
        let pat = &self.pat;
        let default = self.default.iter();
        let composite_type = self.composite_type.iter();
        let mut ty = self.ty.clone();
        anonymonize_lifetimes(&mut ty);

        let ty_string = ty.to_token_stream().to_string().replace(" ", "");

        let ty_entity = quote! {
            ::pgx::utils::sql_entity_graph::TypeEntity {
                ty_source: #ty_string,
                ty_id: TypeId::of::<#ty>(),
                full_path: core::any::type_name::<#ty>(),
                module_path: {
                    let ty_name = core::any::type_name::<#ty>();
                    let mut path_items: Vec<_> = ty_name.split("::").collect();
                    let _ = path_items.pop(); // Drop the one we don't want.
                    path_items.join("::")
                },
                composite_type: None #( .unwrap_or(Some(#composite_type)) )*,
            }
        };

        let quoted = quote! {
            ::pgx::utils::sql_entity_graph::PgExternArgumentEntity {
                pattern: stringify!(#pat),
                ty: #ty_entity,
                is_optional: #is_optional,
                is_variadic: #is_variadic,
                default: None #( .unwrap_or(Some(#default)) )*,
            }
        };
        tokens.append_all(quoted);
    }
}
