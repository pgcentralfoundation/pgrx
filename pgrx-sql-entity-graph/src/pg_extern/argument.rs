/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[pg_extern]` related argument macro expansion for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgrx_sql_entity_graph] APIs, this is considered **internal**
to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::UsedType;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{FnArg, Pat};

/// A parsed `#[pg_extern]` argument.
///
/// It is created during [`PgExtern`](crate::PgExtern) parsing.
#[derive(Debug, Clone)]
pub struct PgExternArgument {
    pub fn_arg: syn::FnArg,
    pub pat: syn::Ident,
    pub used_ty: UsedType,
}

impl PgExternArgument {
    pub fn build(fn_arg: FnArg) -> Result<Self, syn::Error> {
        match &fn_arg {
            syn::FnArg::Typed(pat) => Self::build_from_pat_type(fn_arg.clone(), pat.clone()),
            syn::FnArg::Receiver(_) => {
                Err(syn::Error::new(Span::call_site(), "Unable to parse FnArg that is Self"))
            }
        }
    }

    pub fn build_from_pat_type(
        fn_arg: syn::FnArg,
        value: syn::PatType,
    ) -> Result<Self, syn::Error> {
        let identifier = match *value.pat {
            Pat::Ident(ref p) => p.ident.clone(),
            Pat::Reference(ref p_ref) => match *p_ref.pat {
                Pat::Ident(ref inner_ident) => inner_ident.ident.clone(),
                _ => return Err(syn::Error::new(Span::call_site(), "Unable to parse FnArg")),
            },
            _ => return Err(syn::Error::new(Span::call_site(), "Unable to parse FnArg")),
        };

        let used_ty = UsedType::new(*value.ty)?;

        Ok(PgExternArgument { fn_arg, pat: identifier, used_ty })
    }

    pub fn entity_tokens(&self) -> TokenStream2 {
        let pat = &self.pat;
        let used_ty_entity = self.used_ty.entity_tokens();

        let quoted = quote! {
            ::pgrx::pgrx_sql_entity_graph::PgExternArgumentEntity {
                pattern: stringify!(#pat),
                used_ty: #used_ty_entity,
            }
        };
        quoted
    }
}

impl ToTokens for PgExternArgument {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let fn_arg = &self.fn_arg;
        let quoted = quote! {
            #fn_arg
        };
        tokens.append_all(quoted);
    }
}
