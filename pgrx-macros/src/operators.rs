//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx_sql_entity_graph::{PostgresHash, PostgresOrd};

use crate::{parse_postgres_type_args, PostgresTypeAttribute};
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::DeriveInput;

fn ident_and_path(ast: &DeriveInput) -> (&Ident, proc_macro2::TokenStream) {
    let ident = &ast.ident;
    let args = parse_postgres_type_args(&ast.attrs);
    let path = if args.contains(&PostgresTypeAttribute::PgVarlenaInOutFuncs) {
        quote! { ::pgrx::datum::PgVarlena<#ident> }
    } else {
        quote! { #ident }
    };
    (ident, path)
}

pub(crate) fn deriving_postgres_eq(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut stream = proc_macro2::TokenStream::new();
    let (ident, path) = ident_and_path(&ast);
    stream.extend(derive_pg_eq(ident, &path));
    stream.extend(derive_pg_ne(ident, &path));

    Ok(stream)
}

pub(crate) fn deriving_postgres_ord(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut stream = proc_macro2::TokenStream::new();
    let (ident, path) = ident_and_path(&ast);

    stream.extend(derive_pg_lt(ident, &path));
    stream.extend(derive_pg_gt(ident, &path));
    stream.extend(derive_pg_le(ident, &path));
    stream.extend(derive_pg_ge(ident, &path));
    stream.extend(derive_pg_cmp(ident, &path));

    let sql_graph_entity_item = PostgresOrd::from_derive_input(ast)?;
    sql_graph_entity_item.to_tokens(&mut stream);

    Ok(stream)
}

pub(crate) fn deriving_postgres_hash(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut stream = proc_macro2::TokenStream::new();
    let (ident, path) = ident_and_path(&ast);

    stream.extend(derive_pg_hash(ident, &path));

    let sql_graph_entity_item = PostgresHash::from_derive_input(ast)?;
    sql_graph_entity_item.to_tokens(&mut stream);

    Ok(stream)
}

pub fn derive_pg_eq(name: &Ident, path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_eq", name).to_lowercase(), name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(=)]
        #[::pgrx::pgrx_macros::commutator(=)]
        #[::pgrx::pgrx_macros::negator(<>)]
        #[::pgrx::pgrx_macros::restrict(eqsel)]
        #[::pgrx::pgrx_macros::join(eqjoinsel)]
        #[::pgrx::pgrx_macros::merges]
        #[::pgrx::pgrx_macros::hashes]
        fn #pg_name(left: #path, right: #path) -> bool {
            left == right
        }
    }
}

pub fn derive_pg_ne(name: &Ident, path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_ne", name).to_lowercase(), name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(<>)]
        #[::pgrx::pgrx_macros::commutator(<>)]
        #[::pgrx::pgrx_macros::negator(=)]
        #[::pgrx::pgrx_macros::restrict(neqsel)]
        #[::pgrx::pgrx_macros::join(neqjoinsel)]
        fn #pg_name(left: #path, right: #path) -> bool {
            left != right
        }
    }
}

pub fn derive_pg_lt(name: &Ident, path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_lt", name).to_lowercase(), name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(<)]
        #[::pgrx::pgrx_macros::negator(>=)]
        #[::pgrx::pgrx_macros::commutator(>)]
        #[::pgrx::pgrx_macros::restrict(scalarltsel)]
        #[::pgrx::pgrx_macros::join(scalarltjoinsel)]
        fn #pg_name(left: #path, right: #path) -> bool {
            left < right
        }

    }
}

pub fn derive_pg_gt(name: &Ident, path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_gt", name).to_lowercase(), name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(>)]
        #[::pgrx::pgrx_macros::negator(<=)]
        #[::pgrx::pgrx_macros::commutator(<)]
        #[::pgrx::pgrx_macros::restrict(scalargtsel)]
        #[::pgrx::pgrx_macros::join(scalargtjoinsel)]
        fn #pg_name(left: #path, right: #path) -> bool {
            left > right
        }
    }
}

pub fn derive_pg_le(name: &Ident, path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_le", name).to_lowercase(), name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(<=)]
        #[::pgrx::pgrx_macros::negator(>)]
        #[::pgrx::pgrx_macros::commutator(>=)]
        #[::pgrx::pgrx_macros::restrict(scalarlesel)]
        #[::pgrx::pgrx_macros::join(scalarlejoinsel)]
        fn #pg_name(left: #path, right: #path) -> bool {
            left <= right
        }
    }
}

pub fn derive_pg_ge(name: &Ident, path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_ge", name).to_lowercase(), name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(>=)]
        #[::pgrx::pgrx_macros::negator(<)]
        #[::pgrx::pgrx_macros::commutator(<=)]
        #[::pgrx::pgrx_macros::restrict(scalargesel)]
        #[::pgrx::pgrx_macros::join(scalargejoinsel)]
        fn #pg_name(left: #path, right: #path) -> bool {
            left >= right
        }
    }
}

pub fn derive_pg_cmp(name: &Ident, path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_cmp", name).to_lowercase(), name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_extern(immutable, parallel_safe)]
        fn #pg_name(left: #path, right: #path) -> i32 {
            left.cmp(&right) as i32
        }
    }
}

pub fn derive_pg_hash(name: &Ident, path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_hash", name).to_lowercase(), name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_extern(immutable, parallel_safe)]
        fn #pg_name(value: #path) -> i32 {
            ::pgrx::misc::pgrx_seahash(&value) as i32
        }
    }
}
