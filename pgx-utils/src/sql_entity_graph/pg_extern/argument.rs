/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use crate::sql_entity_graph::UsedType;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{FnArg, Pat};

/// A parsed `#[pg_extern]` argument.
///
/// It is created during [`PgExtern`](crate::sql_entity_graph::PgExtern) parsing.
#[derive(Debug, Clone)]
pub struct PgExternArgument {
    pat: syn::Ident,
    used_ty: UsedType,
}

impl PgExternArgument {
    pub fn build(value: FnArg) -> Result<Self, syn::Error> {
        match value {
            syn::FnArg::Typed(pat) => Self::build_from_pat_type(pat),
            _ => Err(syn::Error::new(Span::call_site(), "Unable to parse FnArg")),
        }
    }

    pub fn build_from_pat_type(value: syn::PatType) -> Result<Self, syn::Error> {
        let identifier = match *value.pat {
            Pat::Ident(ref p) => p.ident.clone(),
            Pat::Reference(ref p_ref) => match *p_ref.pat {
                Pat::Ident(ref inner_ident) => inner_ident.ident.clone(),
                _ => return Err(syn::Error::new(Span::call_site(), "Unable to parse FnArg")),
            },
            _ => return Err(syn::Error::new(Span::call_site(), "Unable to parse FnArg")),
        };

        let used_ty = UsedType::new(*value.ty)?;

        Ok(PgExternArgument {
            pat: identifier,
            used_ty,
        })
    }
}

impl ToTokens for PgExternArgument {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let pat = &self.pat;
        let used_ty_entity = self.used_ty.entity_tokens();

        let quoted = quote! {
            ::pgx::utils::sql_entity_graph::PgExternArgumentEntity {
                pattern: stringify!(#pat),
                used_ty: #used_ty_entity,
            }
        };
        tokens.append_all(quoted);
    }
}
