//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
/*!

`#[derive(PostgresType)]` related entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate] APIs, this is considered **internal**
> to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::mapping::RustSqlMapping;
use crate::pgrx_attribute::{ArgValue, PgrxArg, PgrxAttribute};
use crate::pgrx_sql::PgrxSql;
use crate::to_sql::ToSql;
use crate::to_sql::entity::ToSqlConfigEntity;
use crate::{SqlGraphEntity, SqlGraphIdentifier, TypeMatch};
use eyre::eyre;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, format_ident, quote};
use std::collections::BTreeSet;
use syn::spanned::Spanned;
use syn::{AttrStyle, Attribute, Lit};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Alignment {
    On,
    Off,
}

const INVALID_ATTR_CONTENT: &str =
    r#"expected `#[pgrx(alignment = align)]`, where `align` is "on", or "off""#;

impl ToTokens for Alignment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let value = match self {
            Alignment::On => format_ident!("On"),
            Alignment::Off => format_ident!("Off"),
        };
        let quoted = quote! {
            ::pgrx::pgrx_sql_entity_graph::Alignment::#value
        };
        tokens.append_all(quoted);
    }
}

impl Alignment {
    pub fn from_attribute(attr: &Attribute) -> Result<Option<Self>, syn::Error> {
        if attr.style != AttrStyle::Outer {
            return Err(syn::Error::new(
                attr.span(),
                "#[pgrx(alignment = ..)] is only valid in an outer context",
            ));
        }

        let attr = attr.parse_args::<PgrxAttribute>()?;
        for arg in attr.args.iter() {
            let PgrxArg::NameValue(nv) = arg;
            if !nv.path.is_ident("alignment") {
                continue;
            }

            return match nv.value {
                ArgValue::Lit(Lit::Str(ref s)) => match s.value().as_ref() {
                    "on" => Ok(Some(Self::On)),
                    "off" => Ok(Some(Self::Off)),
                    _ => Err(syn::Error::new(s.span(), INVALID_ATTR_CONTENT)),
                },
                ArgValue::Path(ref p) => Err(syn::Error::new(p.span(), INVALID_ATTR_CONTENT)),
                ArgValue::Lit(ref l) => Err(syn::Error::new(l.span(), INVALID_ATTR_CONTENT)),
            };
        }

        Ok(None)
    }

    pub fn from_attributes(attrs: &[Attribute]) -> Result<Self, syn::Error> {
        for attr in attrs {
            if attr.path().is_ident("pgrx")
                && let Some(v) = Self::from_attribute(attr)?
            {
                return Ok(v);
            }
        }
        Ok(Self::Off)
    }
}

/// The output of a [`PostgresType`](crate::postgres_type::PostgresTypeDerive) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PostgresTypeEntity {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub mappings: BTreeSet<RustSqlMapping>,
    pub in_fn: &'static str,
    pub in_fn_module_path: String,
    pub out_fn: &'static str,
    pub out_fn_module_path: String,
    pub receive_fn: Option<&'static str>,
    pub receive_fn_module_path: Option<String>,
    pub send_fn: Option<&'static str>,
    pub send_fn_module_path: Option<String>,
    pub to_sql_config: ToSqlConfigEntity,
    pub alignment: Option<usize>,
}

impl TypeMatch for PostgresTypeEntity {
    fn id_matches(&self, candidate: &core::any::TypeId) -> bool {
        self.mappings.iter().any(|tester| *candidate == tester.id)
    }
}

impl From<PostgresTypeEntity> for SqlGraphEntity {
    fn from(val: PostgresTypeEntity) -> Self {
        SqlGraphEntity::Type(val)
    }
}

impl SqlGraphIdentifier for PostgresTypeEntity {
    fn dot_identifier(&self) -> String {
        format!("type {}", self.full_path)
    }
    fn rust_identifier(&self) -> String {
        self.full_path.to_string()
    }

    fn file(&self) -> Option<&'static str> {
        Some(self.file)
    }

    fn line(&self) -> Option<u32> {
        Some(self.line)
    }
}

impl ToSql for PostgresTypeEntity {
    fn to_sql(&self, context: &PgrxSql) -> eyre::Result<String> {
        let self_index = context.types[self];
        let item_node = &context.graph[self_index];
        let SqlGraphEntity::Type(PostgresTypeEntity {
            name,
            file,
            line,
            in_fn_module_path,
            module_path,
            full_path,
            out_fn,
            out_fn_module_path,
            in_fn,
            receive_fn,
            receive_fn_module_path,
            send_fn,
            send_fn_module_path,
            alignment,
            ..
        }) = item_node
        else {
            return Err(eyre!("Was not called on a Type. Got: {:?}", item_node));
        };

        // The `in_fn`/`out_fn` need to be present in a certain order:
        // - CREATE TYPE;
        // - CREATE FUNCTION _in;
        // - CREATE FUNCTION _out;
        // - CREATE TYPE (...);

        let in_fn_module_path = if !in_fn_module_path.is_empty() {
            in_fn_module_path.clone()
        } else {
            module_path.to_string() // Presume a local
        };
        let in_fn_path = format!(
            "{in_fn_module_path}{maybe_colons}{in_fn}",
            maybe_colons = if !in_fn_module_path.is_empty() { "::" } else { "" }
        );
        let (_, _index) = context
            .externs
            .iter()
            .find(|(k, _v)| k.full_path == in_fn_path)
            .ok_or_else(|| eyre::eyre!("Did not find `in_fn: {}`.", in_fn_path))?;
        let (in_fn_graph_index, in_fn_entity) = context
            .graph
            .neighbors_undirected(self_index)
            .find_map(|neighbor| match &context.graph[neighbor] {
                SqlGraphEntity::Function(func) if func.full_path == in_fn_path => {
                    Some((neighbor, func))
                }
                _ => None,
            })
            .ok_or_else(|| eyre!("Could not find in_fn graph entity."))?;
        let in_fn_sql = in_fn_entity.to_sql(context)?;

        let out_fn_module_path = if !out_fn_module_path.is_empty() {
            out_fn_module_path.clone()
        } else {
            module_path.to_string() // Presume a local
        };
        let out_fn_path = format!(
            "{out_fn_module_path}{maybe_colons}{out_fn}",
            maybe_colons = if !out_fn_module_path.is_empty() { "::" } else { "" },
        );
        let (_, _index) = context
            .externs
            .iter()
            .find(|(k, _v)| k.full_path == out_fn_path)
            .ok_or_else(|| eyre::eyre!("Did not find `out_fn: {}`.", out_fn_path))?;
        let (out_fn_graph_index, out_fn_entity) = context
            .graph
            .neighbors_undirected(self_index)
            .find_map(|neighbor| match &context.graph[neighbor] {
                SqlGraphEntity::Function(func) if func.full_path == out_fn_path => {
                    Some((neighbor, func))
                }
                _ => None,
            })
            .ok_or_else(|| eyre!("Could not find out_fn graph entity."))?;
        let out_fn_sql = out_fn_entity.to_sql(context)?;

        let receive_fn_graph_index_and_receive_fn_sql = receive_fn_module_path
            .as_ref()
            .zip(*receive_fn)
            .map(|(receive_fn_module_path, receive_fn)| {
                let receive_fn_module_path = if !receive_fn_module_path.is_empty() {
                    receive_fn_module_path.clone()
                } else {
                    module_path.to_string() // Presume a local
                };
                let receive_fn_path = format!(
                    "{receive_fn_module_path}{maybe_colons}{receive_fn}",
                    maybe_colons = if !receive_fn_module_path.is_empty() { "::" } else { "" }
                );

                // Find the receive function in the context
                let (_, _index) = context
                    .externs
                    .iter()
                    .find(|(k, _v)| k.full_path == receive_fn_path)
                    .ok_or_else(|| eyre::eyre!("Did not find `receive_fn`: {receive_fn_path}."))?;

                let (receive_fn_graph_index, receive_fn_entity) = context
                    .graph
                    .neighbors_undirected(self_index)
                    .find_map(|neighbor| match &context.graph[neighbor] {
                        SqlGraphEntity::Function(func) if func.full_path == receive_fn_path => {
                            Some((neighbor, func))
                        }
                        _ => None,
                    })
                    .ok_or_else(|| eyre!("Could not find receive_fn graph entity."))?;
                let receive_fn_sql = receive_fn_entity.to_sql(context)?;

                Ok::<_, eyre::Report>((receive_fn_graph_index, receive_fn_sql, receive_fn_path))
            })
            .transpose()?;

        let send_fn_graph_index_and_send_fn_sql = send_fn_module_path
            .as_ref()
            .zip(*send_fn)
            .map(|(send_fn_module_path, send_fn)| {
                let send_fn_module_path = if !send_fn_module_path.is_empty() {
                    send_fn_module_path.clone()
                } else {
                    module_path.to_string() // Presume a local
                };
                let send_fn_path = format!(
                    "{send_fn_module_path}{maybe_colons}{send_fn}",
                    maybe_colons = if !send_fn_module_path.is_empty() { "::" } else { "" }
                );

                // Find the send function in the context
                let (_, _index) = context
                    .externs
                    .iter()
                    .find(|(k, _v)| k.full_path == send_fn_path)
                    .ok_or_else(|| eyre::eyre!("Did not find `send_fn: {}`.", send_fn_path))?;

                let (send_fn_graph_index, send_fn_entity) = context
                    .graph
                    .neighbors_undirected(self_index)
                    .find_map(|neighbor| match &context.graph[neighbor] {
                        SqlGraphEntity::Function(func) if func.full_path == send_fn_path => {
                            Some((neighbor, func))
                        }
                        _ => None,
                    })
                    .ok_or_else(|| eyre!("Could not find send_fn graph entity."))?;
                let send_fn_sql = send_fn_entity.to_sql(context)?;

                Ok::<_, eyre::Report>((send_fn_graph_index, send_fn_sql, send_fn_path))
            })
            .transpose()?;

        let shell_type = format!(
            "\n\
                -- {file}:{line}\n\
                -- {full_path}\n\
                CREATE TYPE {schema}{name};\
            ",
            schema = context.schema_prefix_for(&self_index),
        );

        let alignment = alignment
            .map(|alignment| {
                assert!(alignment.is_power_of_two());
                let alignment = match alignment {
                    1 => "char",
                    2 => "int2",
                    4 => "int4",
                    8 => "double",
                    _ => panic!("type '{name}' wants unsupported alignment '{alignment}'"),
                };
                format!(
                    ",\n\
                    \tALIGNMENT = {alignment}"
                )
            })
            .unwrap_or_default();

        let (receive_send_attributes, receive_send_sql) = receive_fn_graph_index_and_receive_fn_sql
            .zip(send_fn_graph_index_and_send_fn_sql)
            .map(|((receive_fn_graph_index, receive_fn_sql, receive_fn_path), (send_fn_graph_index, send_fn_sql, send_fn_path))| {
                let receive_fn = receive_fn.unwrap();
                let send_fn = send_fn.unwrap();
                (
                    format! {
                        "\
                        \tRECEIVE = {schema_prefix_receive_fn}{receive_fn}, /* {receive_fn_path} */\n\
                        \tSEND = {schema_prefix_send_fn}{send_fn}, /* {send_fn_path} */\n\
                        ",
                        schema_prefix_receive_fn = context.schema_prefix_for(&receive_fn_graph_index),
                        schema_prefix_send_fn = context.schema_prefix_for(&send_fn_graph_index),
                    },
                    format! {
                        "\n\
                        {receive_fn_sql}\n\
                        {send_fn_sql}\n\
                        "
                    }
                )
            }).unwrap_or_default();

        let materialized_type = format! {
            "\n\
                -- {file}:{line}\n\
                -- {full_path}\n\
                CREATE TYPE {schema}{name} (\n\
                    \tINTERNALLENGTH = variable,\n\
                    \tINPUT = {schema_prefix_in_fn}{in_fn}, /* {in_fn_path} */\n\
                    \tOUTPUT = {schema_prefix_out_fn}{out_fn}, /* {out_fn_path} */\n\
                    {receive_send_attributes}\
                    \tSTORAGE = extended{alignment}\n\
                );\
            ",
            schema = context.schema_prefix_for(&self_index),
            schema_prefix_in_fn = context.schema_prefix_for(&in_fn_graph_index),
            schema_prefix_out_fn = context.schema_prefix_for(&out_fn_graph_index)
        };

        let result = shell_type
            + "\n"
            + &in_fn_sql
            + "\n"
            + &out_fn_sql
            + &receive_send_sql
            + "\n"
            + &materialized_type;

        Ok(result)
    }
}
