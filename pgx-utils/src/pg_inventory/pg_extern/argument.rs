use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::TryFrom;
use syn::{
    parse::{Parse, ParseStream},
    FnArg, Pat, Token,
};

#[derive(Debug, Clone)]
pub struct Argument {
    pat: syn::Ident,
    ty: syn::Type,
    default: Option<syn::Lit>,
}

impl TryFrom<syn::FnArg> for Argument {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: FnArg) -> Result<Self, Self::Error> {
        match value {
            syn::FnArg::Typed(pat) => {
                let identifier = match *pat.pat {
                    Pat::Ident(ref p) => p.ident.clone(),
                    Pat::Reference(ref p) => match *p.pat {
                        Pat::Ident(ref p) => p.ident.clone(),
                        _ => {
                            return Err(Box::new(syn::Error::new(
                                Span::call_site(),
                                "Unable to parse FnArg.",
                            )))
                        }
                    },
                    _ => {
                        return Err(Box::new(syn::Error::new(
                            Span::call_site(),
                            "Unable to parse FnArg.",
                        )))
                    }
                };
                let default = match pat.ty.as_ref() {
                    syn::Type::Macro(macro_pat) => {
                        let mac = &macro_pat.mac;
                        let archetype = mac.path.segments.last().expect("No last segment.");
                        match archetype.ident.to_string().as_str() {
                            "default" => {
                                let out: DefaultMacro = mac.parse_body()?;
                                Some(out.expr)
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                };

                Ok(Argument {
                    pat: identifier,
                    ty: *pat.ty.clone(),
                    default,
                })
            }
            _ => Err(Box::new(syn::Error::new(
                Span::call_site(),
                "Unable to parse FnArg.",
            ))),
        }
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
            },
            _ => false,
        };

        let quoted = quote! {
            pgx_utils::pg_inventory::InventoryPgExternInput {
                pattern: stringify!(#pat),
                ty_id: TypeId::of::<#ty>(),
                ty_name: core::any::type_name::<#ty>(),
                is_optional: #is_optional,
                default: None#( .unwrap_or(Some(stringify!(#default))) )*,
            }
        };
        tokens.append_all(quoted);
    }
}

#[derive(Debug, Clone)]
pub(crate) struct DefaultMacro {
    ty: syn::Type,
    comma: Token![,],
    pub(crate) expr: syn::Lit,
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

#[derive(Debug, Clone)]
pub struct InventoryPgExternInput {
    pub pattern: &'static str,
    pub ty_id: core::any::TypeId,
    pub ty_name: &'static str,
    pub is_optional: bool,
    pub default: Option<&'static str>,
}