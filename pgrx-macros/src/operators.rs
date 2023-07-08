/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgrx_sql_entity_graph::{PostgresHash, PostgresOrd};

use crate::{parse_postgres_type_args, PostgresTypeAttribute};
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::DeriveInput;

fn ident_and_type_path(ast: &DeriveInput) -> (&Ident, proc_macro2::TokenStream) {
    let ident = &ast.ident;
    let args = parse_postgres_type_args(&ast.attrs);
    let type_path = if args.contains(&PostgresTypeAttribute::PgVarlenaInOutFuncs) {
        quote! { ::pgrx::datum::PgVarlena<#ident> }
    } else {
        quote! { #ident }
    };
    (ident, type_path)
}

pub(crate) fn impl_postgres_eq(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut stream = proc_macro2::TokenStream::new();
    let (ident, type_path) = ident_and_type_path(&ast);
    stream.extend(eq(ident, &type_path));
    stream.extend(ne(ident, &type_path));

    Ok(stream)
}

pub(crate) fn impl_postgres_ord(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut stream = proc_macro2::TokenStream::new();
    let (ident, type_path) = ident_and_type_path(&ast);

    stream.extend(lt(ident, &type_path));
    stream.extend(gt(ident, &type_path));
    stream.extend(le(ident, &type_path));
    stream.extend(ge(ident, &type_path));
    stream.extend(cmp(ident, &type_path));

    let sql_graph_entity_item = PostgresOrd::from_derive_input(ast)?;
    sql_graph_entity_item.to_tokens(&mut stream);

    Ok(stream)
}

pub(crate) fn impl_postgres_hash(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut stream = proc_macro2::TokenStream::new();
    let (ident, type_path) = ident_and_type_path(&ast);

    stream.extend(hash(ident, &type_path));

    let sql_graph_entity_item = PostgresHash::from_derive_input(ast)?;
    sql_graph_entity_item.to_tokens(&mut stream);

    Ok(stream)
}

pub fn eq(type_name: &Ident, type_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_eq", type_name).to_lowercase(), type_name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(=)]
        #[::pgrx::pgrx_macros::negator(<>)]
        #[::pgrx::pgrx_macros::restrict(eqsel)]
        #[::pgrx::pgrx_macros::join(eqjoinsel)]
        #[::pgrx::pgrx_macros::merges]
        #[::pgrx::pgrx_macros::hashes]
        fn #pg_name(left: #type_path, right: #type_path) -> bool {
            left == right
        }
    }
}

pub fn ne(type_name: &Ident, type_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_ne", type_name).to_lowercase(), type_name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(<>)]
        #[::pgrx::pgrx_macros::negator(=)]
        #[::pgrx::pgrx_macros::restrict(neqsel)]
        #[::pgrx::pgrx_macros::join(neqjoinsel)]
        fn #pg_name(left: #type_path, right: #type_path) -> bool {
            left != right
        }
    }
}

pub fn lt(type_name: &Ident, type_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_lt", type_name).to_lowercase(), type_name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(<)]
        #[::pgrx::pgrx_macros::negator(>=)]
        #[::pgrx::pgrx_macros::commutator(>)]
        #[::pgrx::pgrx_macros::restrict(scalarltsel)]
        #[::pgrx::pgrx_macros::join(scalarltjoinsel)]
        fn #pg_name(left: #type_path, right: #type_path) -> bool {
            left < right
        }

    }
}

pub fn gt(type_name: &Ident, type_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_gt", type_name).to_lowercase(), type_name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(>)]
        #[::pgrx::pgrx_macros::negator(<=)]
        #[::pgrx::pgrx_macros::commutator(<)]
        #[::pgrx::pgrx_macros::restrict(scalargtsel)]
        #[::pgrx::pgrx_macros::join(scalargtjoinsel)]
        fn #pg_name(left: #type_path, right: #type_path) -> bool {
            left > right
        }
    }
}

pub fn le(type_name: &Ident, type_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_le", type_name).to_lowercase(), type_name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(<=)]
        #[::pgrx::pgrx_macros::negator(>)]
        #[::pgrx::pgrx_macros::commutator(>=)]
        #[::pgrx::pgrx_macros::restrict(scalarlesel)]
        #[::pgrx::pgrx_macros::join(scalarlejoinsel)]
        fn #pg_name(left: #type_path, right: #type_path) -> bool {
            left <= right
        }
    }
}

pub fn ge(type_name: &Ident, type_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_ge", type_name).to_lowercase(), type_name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_operator(immutable, parallel_safe)]
        #[::pgrx::pgrx_macros::opname(>=)]
        #[::pgrx::pgrx_macros::negator(<)]
        #[::pgrx::pgrx_macros::commutator(<=)]
        #[::pgrx::pgrx_macros::restrict(scalargesel)]
        #[::pgrx::pgrx_macros::join(scalargejoinsel)]
        fn #pg_name(left: #type_path, right: #type_path) -> bool {
            left >= right
        }
    }
}

pub fn cmp(type_name: &Ident, type_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_cmp", type_name).to_lowercase(), type_name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_extern(immutable, parallel_safe)]
        fn #pg_name(left: #type_path, right: #type_path) -> i32 {
            left.cmp(&right) as i32
        }
    }
}

pub fn hash(type_name: &Ident, type_path: &proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let pg_name = Ident::new(&format!("{}_hash", type_name).to_lowercase(), type_name.span());
    quote! {
        #[allow(non_snake_case)]
        #[::pgrx::pgrx_macros::pg_extern(immutable, parallel_safe)]
        fn #pg_name(value: #type_path) -> i32 {
            ::pgrx::misc::pgrx_seahash(&value) as i32
        }
    }
}
