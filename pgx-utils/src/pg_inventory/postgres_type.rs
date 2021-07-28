use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use std::hash::{Hash, Hasher};
use syn::Generics;
use eyre::eyre as eyre_err;

use super::{DotIdentifier, SqlGraphEntity, ToSql};

#[derive(Debug, Clone)]
pub struct PostgresType {
    name: Ident,
    generics: Generics,
    in_fn: Ident,
    out_fn: Ident,
}

impl PostgresType {
    pub fn new(name: Ident, generics: Generics, in_fn: Ident, out_fn: Ident) -> Self {
        Self {
            generics,
            name,
            in_fn,
            out_fn,
        }
    }
}

impl ToTokens for PostgresType {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = &self.name;
        let mut static_generics = self.generics.clone();
        for lifetime in static_generics.lifetimes_mut() {
            lifetime.lifetime.ident = Ident::new("static", Span::call_site());
        }
        let (_impl_generics, ty_generics, _where_clauses) = static_generics.split_for_impl();

        let in_fn = &self.in_fn;
        let out_fn = &self.out_fn;
        let inv = quote! {
            pgx_utils::pg_inventory::inventory::submit! {
                let mut mappings = Default::default();
                <#name #ty_generics as ::pgx::datum::WithTypeIds>::register_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithSizedTypeIds::<#name #ty_generics>::register_sized_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithArrayTypeIds::<#name #ty_generics>::register_array_with_refs(&mut mappings, stringify!(#name).to_string());
                ::pgx::datum::WithVarlenaTypeIds::<#name #ty_generics>::register_varlena_with_refs(&mut mappings, stringify!(#name).to_string());
                let submission = pgx_utils::pg_inventory::InventoryPostgresType {
                    name: stringify!(#name),
                    file: file!(),
                    line: line!(),
                    module_path: module_path!(),
                    full_path: core::any::type_name::<#name #ty_generics>(),
                    id: *<#name  #ty_generics as WithTypeIds>::ITEM_ID,
                    mappings,
                    in_fn: stringify!(#in_fn),
                    in_fn_module_path: {
                        let in_fn = stringify!(#in_fn);
                        let mut path_items: Vec<_> = in_fn.split("::").collect();
                        let _ = path_items.pop(); // Drop the one we don't want.
                        path_items.join("::")
                    },
                    out_fn: stringify!(#out_fn),
                    out_fn_module_path: {
                        let out_fn = stringify!(#out_fn);
                        let mut path_items: Vec<_> = out_fn.split("::").collect();
                        let _ = path_items.pop(); // Drop the one we don't want.
                        path_items.join("::")
                    }
                };
                let retval = crate::__pgx_internals::PostgresType(submission);
                retval
            }
        };
        tokens.append_all(inv);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryPostgresType {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
    pub mappings: std::collections::HashSet<super::RustSqlMapping>,
    pub in_fn: &'static str,
    pub in_fn_module_path: String,
    pub out_fn: &'static str,
    pub out_fn_module_path: String,
}

impl Hash for InventoryPostgresType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialOrd for InventoryPostgresType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for InventoryPostgresType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl InventoryPostgresType {
    pub fn id_matches(&self, candidate: &core::any::TypeId) -> bool {
        self.mappings
            .iter()
            .any(|tester| *candidate == tester.id)
    }
}

impl<'a> Into<SqlGraphEntity<'a>> for &'a InventoryPostgresType {
    fn into(self) -> SqlGraphEntity<'a> {
        SqlGraphEntity::Type(self)
    }
}

impl DotIdentifier for InventoryPostgresType {
    fn dot_identifier(&self) -> String {
        format!("type {}", self.full_path.to_string())
    }
}


impl ToSql for InventoryPostgresType {
    #[tracing::instrument(level = "debug", err, skip(self, context))]
    fn to_sql(&self, context: &super::PgxSql) -> eyre::Result<String> {
        let self_index = context.types[self];
        let item_node = &context.graph[self_index];
        let item = match item_node {
            SqlGraphEntity::Type(item) => item,
            _ => return Err(eyre_err!("Was not called on a Type. Got: {:?}", item_node)),
        };

        // The `in_fn`/`out_fn` need to be present in a certain order:
        // - CREATE TYPE;
        // - CREATE FUNCTION _in;
        // - CREATE FUNCTION _out;
        // - CREATE TYPE (...);

        let in_fn_module_path = if !item.in_fn_module_path.is_empty() {
            item.in_fn_module_path.clone()
        } else {
            item.module_path.to_string() // Presume a local
        };
        let in_fn_path = format!(
            "{module_path}{maybe_colons}{in_fn}",
            module_path = in_fn_module_path,
            maybe_colons = if !in_fn_module_path.is_empty() {
                "::"
            } else {
                ""
            },
            in_fn = item.in_fn,
        );
        let (_, _index) = context
            .externs
            .iter()
            .find(|(k, _v)| (**k).full_path == in_fn_path.as_str())
            .ok_or_else(|| eyre::eyre!("Did not find `in_fn: {}`.", in_fn_path))?;
        let (in_fn_graph_index, in_fn) = context
            .graph
            .neighbors_undirected(self_index)
            .find_map(|neighbor| match &context.graph[neighbor] {
                SqlGraphEntity::Function(func) if func.full_path == in_fn_path => {
                    Some((neighbor, func))
                },
                _ => None,
            })
            .ok_or_else(|| eyre_err!("Could not find in_fn graph entity."))?;
        tracing::trace!(in_fn = ?in_fn_path, "Found matching `in_fn`");
        let in_fn_sql = in_fn.to_sql(context)?;
        tracing::trace!(%in_fn_sql);

        let out_fn_module_path = if !item.out_fn_module_path.is_empty() {
            item.out_fn_module_path.clone()
        } else {
            item.module_path.to_string() // Presume a local
        };
        let out_fn_path = format!(
            "{module_path}{maybe_colons}{out_fn}",
            module_path = out_fn_module_path,
            maybe_colons = if !out_fn_module_path.is_empty() {
                "::"
            } else {
                ""
            },
            out_fn = item.out_fn,
        );
        let (_, _index) = context
            .externs
            .iter()
            .find(|(k, _v)| {
                tracing::trace!(%k.full_path, %out_fn_path, "Checked");
                (**k).full_path == out_fn_path.as_str()
            })
            .ok_or_else(|| eyre::eyre!("Did not find `out_fn: {}`.", out_fn_path))?;
        let (out_fn_graph_index, out_fn) = context
            .graph
            .neighbors_undirected(self_index)
            .find_map(|neighbor| match &context.graph[neighbor] {
                SqlGraphEntity::Function(func) if func.full_path == out_fn_path => {
                    Some((neighbor, func))
                },
                _ => None,
            })
            .ok_or_else(|| eyre_err!("Could not find out_fn graph entity."))?;
        tracing::trace!(out_fn = ?out_fn_path, "Found matching `out_fn`");
        let out_fn_sql = out_fn.to_sql(context)?;
        tracing::trace!(%out_fn_sql);

        let shell_type = format!(
            "\n\
                                -- {file}:{line}\n\
                                -- {full_path}\n\
                                CREATE TYPE {schema}{name};\n\
                            ",
            schema = context.schema_prefix_for(&self_index),
            full_path = item.full_path,
            file = item.file,
            line = item.line,
            name = item.name,
        );
        tracing::debug!(sql = %shell_type);

        let materialized_type = format!("\n\
                                -- {file}:{line}\n\
                                -- {full_path}\n\
                                CREATE TYPE {schema}{name} (\n\
                                    \tINTERNALLENGTH = variable,\n\
                                    \tINPUT = {schema_prefix_in_fn}{in_fn}, /* {in_fn_path} */\n\
                                    \tOUTPUT = {schema_prefix_out_fn}{out_fn}, /* {out_fn_path} */\n\
                                    \tSTORAGE = extended\n\
                                );
                            ",
                                        full_path = item.full_path,
                                        file = item.file,
                                        line = item.line,
                                        schema = context.schema_prefix_for(&self_index),
                                        name = item.name,
                                        schema_prefix_in_fn = context.schema_prefix_for(&in_fn_graph_index),
                                        in_fn = item.in_fn,
                                        in_fn_path = in_fn_path,
                                        schema_prefix_out_fn = context.schema_prefix_for(&out_fn_graph_index),
                                        out_fn = item.out_fn,
                                        out_fn_path = out_fn_path,
        );
        tracing::debug!(sql = %materialized_type);

        Ok(shell_type + &in_fn_sql + &out_fn_sql + &materialized_type)
    }
}
