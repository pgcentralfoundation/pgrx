use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    FnArg, Pat, Token,
};

#[derive(Debug, Clone)]
pub struct Argument {
    pat: syn::Ident,
    ty: syn::Type,
    default: Option<String>,
}

impl Argument {
    pub fn build(value: FnArg) -> Result<Option<Self>, syn::Error> {
        match value {
            syn::FnArg::Typed(pat) => {
                Self::build_from_pat_type(pat)
            }
            _ => Err(syn::Error::new(Span::call_site(), "Unable to parse FnArg.")),
        }
    }

    pub fn build_from_pat_type(value: syn::PatType) -> Result<Option<Self>, syn::Error> {
        let mut true_ty = *value.ty.clone();
        let identifier = match *value.pat {
            Pat::Ident(ref p) => p.ident.clone(),
            Pat::Reference(ref p_ref) => match *p_ref.pat {
                Pat::Ident(ref inner_ident) => inner_ident.ident.clone(),
                _ => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "Unable to parse FnArg.",
                    ))
                }
            },
            _ => return Err(syn::Error::new(Span::call_site(), "Unable to parse FnArg.")),
        };
        let default = match value.ty.as_ref() {
            syn::Type::Macro(macro_pat) => {
                let mac = &macro_pat.mac;
                let archetype = mac.path.segments.last().expect("No last segment.");
                match archetype.ident.to_string().as_str() {
                    "default" => {
                        let out: DefaultMacro = mac.parse_body()?;
                        true_ty = out.ty;
                        match out.expr {
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(def),
                                ..
                            }) => {
                                let value = def.value();
                                Some(value)
                            }
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Int(def),
                                ..
                            }) => {
                                let value = def.base10_digits();
                                Some(value.to_string())
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
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
                                        let archetype = mac
                                            .path
                                            .segments
                                            .last()
                                            .expect("No last segment.");
                                        match archetype.ident.to_string().as_str() {
                                            "default" => {
                                                let out: DefaultMacro = mac.parse_body()?;
                                                match out.expr {
                                                    syn::Expr::Lit(syn::ExprLit {
                                                        lit: syn::Lit::Str(def),
                                                        ..
                                                    }) => {
                                                        let value = def.value();
                                                        default = Some(value)
                                                    }
                                                    syn::Expr::Lit(syn::ExprLit {
                                                        lit: syn::Lit::Int(def),
                                                        ..
                                                    }) => {
                                                        let value = def.base10_digits();
                                                        default = Some(value.to_string())
                                                    }
                                                    syn::Expr::Lit(syn::ExprLit {
                                                        lit: syn::Lit::Bool(def),
                                                        ..
                                                    }) => {
                                                        default =
                                                            Some(def.value.to_string())
                                                    }
                                                    _ => (),
                                                }
                                            }
                                            _ => (),
                                        }
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
                                        lifetime.ident = syn::Ident::new("static", Span::call_site())
                                    },
                                    _ => (),
                                }   
                            }
                        },
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
                        return Err(syn::Error::new(
                            Span::call_site(),
                            "It's a FunctionCallInfoBaseData, skipping.",
                        ));
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

impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pat = &self.pat;
        let ty = &self.ty;
        let default = self.default.iter();
        let is_optional = match self.ty {
            syn::Type::Path(ref type_path) => {
                let path = &type_path.path;
                let mut found_optional = false;
                for segment in &path.segments {
                    if segment.ident.to_string().as_str() == "Option" {
                        found_optional = true;
                    }
                }
                found_optional
            }
            _ => false,
        };

        let quoted = quote! {
            pgx_utils::pg_inventory::InventoryPgExternInput {
                pattern: stringify!(#pat),
                ty_id: TypeId::of::<#ty>(),
                full_path: core::any::type_name::<#ty>(),
                module_path: {
                    let ty_name = core::any::type_name::<#ty>();
                    let mut path_items: Vec<_> = ty_name.split("::").collect();
                    let _ = path_items.pop(); // Drop the one we don't want.
                    path_items.join("::")
                },
                is_optional: #is_optional,
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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InventoryPgExternInput {
    pub pattern: &'static str,
    pub ty_id: core::any::TypeId,
    pub full_path: &'static str,
    pub module_path: String,
    pub is_optional: bool,
    pub default: Option<&'static str>,
}
