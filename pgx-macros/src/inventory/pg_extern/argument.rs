use proc_macro2::{TokenStream as TokenStream2, Span};
use syn::{Token, parse::{Parse, ParseStream}, FnArg, Pat};
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::TryFrom;

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
                        _ => return Err(Box::new(syn::Error::new(Span::call_site(), "Unable to parse FnArg."))),
                    },
                    _ => return Err(Box::new(syn::Error::new(Span::call_site(), "Unable to parse FnArg."))),
                };
                let default = match pat.ty.as_ref() {
                    syn::Type::Macro(macro_pat) => {
                        let mac = &macro_pat.mac;
                        let archetype = mac.path.segments.last().expect("No last segment.");
                        match archetype.ident.to_string().as_str() {
                            "default" => {
                                let out: DefaultMacro = mac.parse_body()?;
                                Some(out.expr)
                            },
                            _ => None,
                        }
                    },
                    _ => None,
                };

                Ok(Argument {
                    pat: identifier,
                    ty: *pat.ty.clone(),
                    default,
                })
            },
            _ => Err(Box::new(syn::Error::new(Span::call_site(), "Unable to parse FnArg."))),
        }
    }
}

impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pat = &self.pat;
        let ty = &self.ty;
        let default = self.default.iter();

        let quoted = quote! {
            crate::__pgx_internals::PgxExternInputs {
                pattern: stringify!(#pat),
                ty_id: TypeId::of::<#ty>(),
                ty_name: core::any::type_name::<#ty>(),
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
