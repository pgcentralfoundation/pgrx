/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx_utils::{
    operator_common::*,
    sql_entity_graph::{PostgresHash, PostgresOrd},
};

use quote::ToTokens;
use syn::DeriveInput;

pub(crate) fn impl_postgres_eq(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(eq(&ast.ident));
    stream.extend(ne(&ast.ident));

    Ok(stream)
}

pub(crate) fn impl_postgres_ord(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(lt(&ast.ident));
    stream.extend(gt(&ast.ident));
    stream.extend(le(&ast.ident));
    stream.extend(ge(&ast.ident));
    stream.extend(cmp(&ast.ident));

    let sql_graph_entity_item = PostgresOrd::from_derive_input(ast)?;
    sql_graph_entity_item.to_tokens(&mut stream);

    Ok(stream)
}

pub(crate) fn impl_postgres_hash(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let mut stream = proc_macro2::TokenStream::new();

    stream.extend(hash(&ast.ident));

    let sql_graph_entity_item = PostgresHash::from_derive_input(ast)?;
    sql_graph_entity_item.to_tokens(&mut stream);

    Ok(stream)
}
