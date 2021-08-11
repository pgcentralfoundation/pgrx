mod argument;
mod attribute;
mod operator;
mod returning;
mod search_path;

use argument::Argument;
use attribute::{Attribute, PgxAttributes};
use operator::{PgxOperator, PgxOperatorAttributeWithIdent, PgxOperatorOpName};
use returning::Returning;
use search_path::SearchPathList;

use eyre::eyre as eyre_err;
use eyre::WrapErr;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::Meta;

pub use argument::InventoryPgExternInput;
pub use operator::InventoryPgOperator;
pub use returning::InventoryPgExternReturn;

use crate::ExternArgs;

use super::{DotIdentifier, SqlDeclaredEntity, SqlGraphEntity, ToSql};

/// A parsed `#[pg_extern]` item.
///
/// It should be used with [`syn::parse::Parse`] functions.
///
/// Using [`quote::ToTokens`] will output the declaration for a [`InventoryPgExtern`].
///
/// ```rust
/// use syn::{Macro, parse::Parse, parse_quote, parse};
/// use quote::{quote, ToTokens};
/// use pgx_utils::pg_inventory::PgExtern;
///
/// # fn main() -> eyre::Result<()> {
/// let parsed: PgExtern = parse_quote! {
///     fn example(x: Option<str>) -> Option<&'a str> {
///         unimplemented!()
///     }
/// };
/// let inventory_tokens = parsed.to_token_stream();
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct PgExtern {
    attrs: Option<PgxAttributes>,
    func: syn::ItemFn,
}

impl PgExtern {
    fn name(&self) -> String {
        self.attrs
            .as_ref()
            .and_then(|a| {
                a.attrs.iter().find_map(|candidate| match candidate {
                    Attribute::Name(name) => Some(name.value()),
                    _ => None,
                })
            })
            .unwrap_or_else(|| self.func.sig.ident.to_string())
    }

    fn schema(&self) -> Option<String> {
        self.attrs.as_ref().and_then(|a| {
            a.attrs.iter().find_map(|candidate| match candidate {
                Attribute::Schema(name) => Some(name.value()),
                _ => None,
            })
        })
    }

    fn extern_attrs(&self) -> Option<&PgxAttributes> {
        self.attrs.as_ref()
    }

    fn overridden(&self) -> Option<String> {
        let mut retval = None;
        let mut in_commented_sql_block = false;
        for attr in &self.func.attrs {
            let meta = attr.parse_meta().ok();
            if let Some(meta) = meta {
                if meta.path().is_ident("doc") {
                    let content = match meta {
                        Meta::Path(_) | Meta::List(_) => continue,
                        Meta::NameValue(mnv) => mnv,
                    };
                    if let syn::Lit::Str(inner) = content.lit {
                        if !in_commented_sql_block && inner.value().trim() == "```pgxsql" {
                            in_commented_sql_block = true;
                        } else if in_commented_sql_block && inner.value().trim() == "```" {
                            in_commented_sql_block = false;
                        } else if in_commented_sql_block {
                            let sql = retval.get_or_insert_with(String::default);
                            let line = inner.value().trim_start().replace(
                                "@FUNCTION_NAME@",
                                &*(self.func.sig.ident.to_string() + "_wrapper"),
                            ) + "\n";
                            sql.push_str(&*line);
                        }
                    }
                }
            }
        }
        retval
    }

    fn operator(&self) -> Option<PgxOperator> {
        let mut skel = Option::<PgxOperator>::default();
        for attr in &self.func.attrs {
            let last_segment = attr.path.segments.last().unwrap();
            match last_segment.ident.to_string().as_str() {
                "opname" => {
                    let attr: PgxOperatorOpName = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .opname
                        .get_or_insert(attr);
                }
                "commutator" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .commutator
                        .get_or_insert(attr);
                }
                "negator" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .negator
                        .get_or_insert(attr);
                }
                "join" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .join
                        .get_or_insert(attr);
                }
                "restrict" => {
                    let attr: PgxOperatorAttributeWithIdent = syn::parse2(attr.tokens.clone())
                        .expect(&format!("Unable to parse {:?}", &attr.tokens));
                    skel.get_or_insert_with(Default::default)
                        .restrict
                        .get_or_insert(attr);
                }
                "hashes" => {
                    skel.get_or_insert_with(Default::default).hashes = true;
                }
                "merges" => {
                    skel.get_or_insert_with(Default::default).merges = true;
                }
                _ => (),
            }
        }
        skel
    }

    fn search_path(&self) -> Option<SearchPathList> {
        self.func
            .attrs
            .iter()
            .find(|f| {
                f.path
                    .segments
                    .first()
                    .map(|f| f.ident == Ident::new("search_path", Span::call_site()))
                    .unwrap_or_default()
            })
            .and_then(|attr| Some(attr.parse_args::<SearchPathList>().unwrap()))
    }

    fn inputs(&self) -> eyre::Result<Vec<Argument>> {
        let mut args = Vec::default();
        for input in &self.func.sig.inputs {
            let arg = Argument::build(input.clone())
                .wrap_err_with(|| format!("Could not map {:?}", input))?;
            if let Some(arg) = arg {
                args.push(arg);
            }
        }
        Ok(args)
    }

    fn returns(&self) -> Result<Returning, eyre::Error> {
        Returning::try_from(&self.func.sig.output)
    }

    pub fn new(attr: TokenStream2, item: TokenStream2) -> Result<Self, syn::Error> {
        let attrs = syn::parse2::<PgxAttributes>(attr).ok();
        let func = syn::parse2::<syn::ItemFn>(item)?;
        Ok(Self { attrs, func })
    }
}

impl ToTokens for PgExtern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ident = &self.func.sig.ident;
        let name = self.name();
        let schema = self.schema();
        let schema_iter = schema.iter();
        let extern_attrs = self.extern_attrs();
        let search_path = self.search_path().into_iter();
        let inputs = self.inputs().unwrap();
        let returns = match self.returns() {
            Ok(returns) => returns,
            Err(e) => {
                let msg = e.to_string();
                tokens.append_all(quote! {
                    std::compile_error!(#msg);
                });
                return;
            }
        };
        let operator = self.operator().into_iter();
        let overridden = self.overridden().into_iter();

        let inv = quote! {
            pgx::pg_inventory::inventory::submit! {
                use core::any::TypeId;
                let submission = pgx::pg_inventory::InventoryPgExtern {
                    name: #name,
                    unaliased_name: stringify!(#ident),
                    schema: None#( .unwrap_or(Some(#schema_iter)) )*,
                    file: file!(),
                    line: line!(),
                    module_path: core::module_path!(),
                    full_path: concat!(core::module_path!(), "::", stringify!(#ident)),
                    extern_attrs: #extern_attrs,
                    search_path: None#( .unwrap_or(Some(vec![#search_path])) )*,
                    fn_args: vec![#(#inputs),*],
                    fn_return: #returns,
                    operator: None#( .unwrap_or(Some(#operator)) )*,
                    overridden: None#( .unwrap_or(Some(#overridden)) )*,
                };
                let retval = crate::__pgx_internals::PgExtern(submission);
                retval
            }
        };
        tokens.append_all(inv);
    }
}

impl Parse for PgExtern {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            attrs: input.parse().ok(),
            func: input.parse()?,
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InventoryPgExtern {
    pub name: &'static str,
    pub unaliased_name: &'static str,
    pub schema: Option<&'static str>,
    pub file: &'static str,
    pub line: u32,
    pub module_path: &'static str,
    pub full_path: &'static str,
    pub extern_attrs: Vec<crate::ExternArgs>,
    pub search_path: Option<Vec<&'static str>>,
    pub fn_args: Vec<InventoryPgExternInput>,
    pub fn_return: InventoryPgExternReturn,
    pub operator: Option<InventoryPgOperator>,
    pub overridden: Option<&'static str>,
}

impl<'a> Into<SqlGraphEntity<'a>> for &'a InventoryPgExtern {
    fn into(self) -> SqlGraphEntity<'a> {
        SqlGraphEntity::Function(self)
    }
}

impl DotIdentifier for InventoryPgExtern {
    fn dot_identifier(&self) -> String {
        format!("fn {}", self.full_path.to_string())
    }
}

impl ToSql for InventoryPgExtern {
    #[tracing::instrument(
        level = "info",
        skip(self, context),
        fields(identifier = self.name),
    )]
    fn to_sql(&self, context: &super::PgxSql) -> eyre::Result<String> {
        let self_index = context.externs[self];
        let mut extern_attrs = self.extern_attrs.clone();
        let mut strict_upgrade = true;
        if !extern_attrs.iter().any(|i| i == &ExternArgs::Strict) {
            for arg in &self.fn_args {
                if arg.is_optional {
                    strict_upgrade = false;
                }
            }
        }
        tracing::trace!(?extern_attrs, strict_upgrade);

        if strict_upgrade {
            extern_attrs.push(ExternArgs::Strict);
        }

        let fn_sql = format!("\
                                CREATE OR REPLACE FUNCTION {schema}\"{name}\"({arguments}) {returns}\n\
                                {extern_attrs}\
                                {search_path}\
                                LANGUAGE c /* Rust */\n\
                                AS 'MODULE_PATHNAME', '{unaliased_name}_wrapper';\
                            ",
                             schema = self.schema.map(|schema| format!("{}.", schema)).unwrap_or_else(|| context.schema_prefix_for(&self_index)),
                             name = self.name,
                             unaliased_name = self.unaliased_name,
                             arguments = if !self.fn_args.is_empty() {
                                 let mut args = Vec::new();
                                 for (idx, arg) in self.fn_args.iter().enumerate() {
                                     let graph_index = context.graph.neighbors_undirected(self_index).find(|neighbor| match &context.graph[*neighbor] {
                                         SqlGraphEntity::Type(ty) => ty.id_matches(&arg.ty_id),
                                         SqlGraphEntity::Enum(en) => en.id_matches(&arg.ty_id),
                                         SqlGraphEntity::BuiltinType(defined) => defined == &arg.full_path,
                                         _ => false,
                                     }).ok_or_else(|| eyre_err!("Could not find arg type in graph. Got: {:?}", arg))?;
                                     let needs_comma = idx < (self.fn_args.len() - 1);
                                     let buf = format!("\
                                            \t\"{pattern}\" {variadic}{schema_prefix}{sql_type}{default}{maybe_comma}/* {full_path} */\
                                        ",
                                            pattern = arg.pattern,
                                            schema_prefix = context.schema_prefix_for(&graph_index),
                                            // First try to match on [`TypeId`] since it's most reliable.
                                            sql_type = context.source_only_to_sql_type(arg.ty_source).or_else(|| {
                                                context.type_id_to_sql_type(arg.ty_id)
                                            }).or_else(|| {
                                                // Fall back to fuzzy matching.
                                                let path = arg.full_path.to_string();
                                                if let Some(found) = context.has_sql_declared_entity(&SqlDeclaredEntity::Type(path.clone())) {
                                                    Some(found.sql())
                                                }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclaredEntity::Enum(path.clone())) {
                                                    Some(found.sql())
                                                } else {
                                                    None
                                                }
                                            }).ok_or_else(|| eyre_err!(
                                                "Failed to map argument `{}` type `{}` to SQL type while building function `{}`.",
                                                arg.pattern,
                                                arg.full_path,
                                                self.name
                                            ))?,
                                            default = if let Some(def) = arg.default { format!(" DEFAULT {}", def) } else { String::from("") },
                                            variadic = if arg.is_variadic { "VARIADIC " } else { "" },
                                            maybe_comma = if needs_comma { ", " } else { " " },
                                            full_path = arg.full_path,
                                     );
                                     args.push(buf);
                                 };
                                 String::from("\n") + &args.join("\n") + "\n"
                             } else { Default::default() },
                             returns = match &self.fn_return {
                                 InventoryPgExternReturn::None => String::from("RETURNS void"),
                                 InventoryPgExternReturn::Type { id, source, full_path, .. } => {
                                     let graph_index = context.graph.neighbors_undirected(self_index).find(|neighbor| match &context.graph[*neighbor] {
                                         SqlGraphEntity::Type(ty) => ty.id_matches(&id),
                                         SqlGraphEntity::Enum(en) => en.id_matches(&id),
                                         SqlGraphEntity::BuiltinType(defined) => &*defined == full_path,
                                         _ => false,
                                     }).ok_or_else(|| eyre_err!("Could not find return type in graph."))?;
                                     format!("RETURNS {schema_prefix}{sql_type} /* {full_path} */",
                                             sql_type = context.source_only_to_sql_type(source).or_else(|| {
                                                 context.type_id_to_sql_type(*id)
                                             }).or_else(|| {
                                                    let pat = full_path.to_string();
                                                    if let Some(found) = context.has_sql_declared_entity(&SqlDeclaredEntity::Type(pat.clone())) {
                                                        Some(found.sql())
                                                    }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclaredEntity::Enum(pat.clone())) {
                                                        Some(found.sql())
                                                    } else {
                                                        None
                                                    }
                                                }).ok_or_else(|| eyre_err!("Failed to map return type `{}` to SQL type while building function `{}`.", full_path, self.full_path))?,
                                             schema_prefix = context.schema_prefix_for(&graph_index),
                                             full_path = full_path
                                     )
                                 },
                                 InventoryPgExternReturn::SetOf { id, source, full_path, .. } => {
                                     let graph_index = context.graph.neighbors_undirected(self_index).find(|neighbor| match &context.graph[*neighbor] {
                                         SqlGraphEntity::Type(ty) => ty.id_matches(&id),
                                         SqlGraphEntity::Enum(en) => en.id_matches(&id),
                                         SqlGraphEntity::BuiltinType(defined) => defined == full_path,
                                         _ => false,
                                     }).ok_or_else(|| eyre_err!("Could not find return type in graph."))?;
                                     format!("RETURNS SETOF {schema_prefix}{sql_type} /* {full_path} */",
                                             sql_type = context.source_only_to_sql_type(source).or_else(|| {
                                                 context.type_id_to_sql_type(*id)
                                             }).or_else(|| {
                                                    let pat = full_path.to_string();
                                                    if let Some(found) = context.has_sql_declared_entity(&SqlDeclaredEntity::Type(pat.clone())) {
                                                        Some(found.sql())
                                                    }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclaredEntity::Enum(pat.clone())) {
                                                        Some(found.sql())
                                                    } else {
                                                        None
                                                    }
                                                }).ok_or_else(|| eyre_err!("Failed to map return type `{}` to SQL type while building function `{}`.", full_path, self.full_path))?,
                                             schema_prefix = context.schema_prefix_for(&graph_index),
                                             full_path = full_path
                                     )
                                 },
                                 InventoryPgExternReturn::Iterated(table_items) => {
                                     let mut items = String::new();
                                     for (idx, (id, source, ty_name, _module_path, col_name)) in table_items.iter().enumerate() {
                                         let graph_index = context.graph.neighbors_undirected(self_index).find(|neighbor| match &context.graph[*neighbor] {
                                             SqlGraphEntity::Type(ty) => ty.id_matches(&id),
                                             SqlGraphEntity::Enum(en) => en.id_matches(&id),
                                             SqlGraphEntity::BuiltinType(defined) => defined == ty_name,
                                             _ => false,
                                         });
                                         let needs_comma = idx < (table_items.len() - 1);
                                         let item = format!("\n\t{col_name} {schema_prefix}{ty_resolved}{needs_comma} /* {ty_name} */",
                                                            col_name = col_name.expect("An iterator of tuples should have `named!()` macro declarations."),
                                                            schema_prefix = if let Some(graph_index) = graph_index {
                                                                context.schema_prefix_for(&graph_index)
                                                            } else { "".into() },
                                                            ty_resolved = context.source_only_to_sql_type(source).or_else(|| {
                                                                context.type_id_to_sql_type(*id)
                                                            }).or_else(|| {
                                                                let pat = ty_name.to_string();
                                                                if let Some(found) = context.has_sql_declared_entity(&SqlDeclaredEntity::Type(pat.clone())) {
                                                                    Some(found.sql())
                                                                }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclaredEntity::Enum(pat.clone())) {
                                                                    Some(found.sql())
                                                                } else {
                                                                    None
                                                                }
                                                            }).ok_or_else(|| eyre_err!("Failed to map return type `{}` to SQL type while building function `{}`.", ty_name, self.name))?,
                                                            needs_comma = if needs_comma { ", " } else { " " },
                                                            ty_name = ty_name
                                         );
                                         items.push_str(&item);
                                     }
                                     format!("RETURNS TABLE ({}\n)", items)
                                 },
                                 InventoryPgExternReturn::Trigger => String::from("RETURNS trigger"),
                             },
                             search_path = if let Some(search_path) = &self.search_path {
                                 let retval = format!("SET search_path TO {}", search_path.join(", "));
                                 retval + "\n"
                             } else { Default::default() },
                             extern_attrs = if extern_attrs.is_empty() {
                                 String::default()
                             } else {
                                 let mut retval = extern_attrs.iter().map(|attr| format!("{}", attr).to_uppercase()).collect::<Vec<_>>().join(" ");
                                 retval.push('\n');
                                 retval
                             },
        );

        let ext_sql = format!(
            "\n\
                                -- {file}:{line}\n\
                                -- {module_path}::{name}\n\
                                {fn_sql}\
                                {overridden}\
                            ",
            name = self.name,
            module_path = self.module_path,
            file = self.file,
            line = self.line,
            fn_sql = if self.overridden.is_some() {
                let mut inner = fn_sql
                    .lines()
                    .map(|f| format!("-- {}", f))
                    .collect::<Vec<_>>()
                    .join("\n");
                inner.push_str(
                    "\n--\n-- Overridden as (due to a `///` comment with a `pgxsql` code block):",
                );
                inner
            } else {
                fn_sql
            },
            overridden = self
                .overridden
                .map(|f| String::from("\n") + f + "\n")
                .unwrap_or_default(),
        );
        tracing::debug!(sql = %ext_sql);

        let rendered = match (self.overridden, &self.operator) {
            (None, Some(op)) => {
                let mut optionals = vec![];
                if let Some(it) = op.commutator {
                    optionals.push(format!("\tCOMMUTATOR = {}", it));
                };
                if let Some(it) = op.negator {
                    optionals.push(format!("\tNEGATOR = {}", it));
                };
                if let Some(it) = op.restrict {
                    optionals.push(format!("\tRESTRICT = {}", it));
                };
                if let Some(it) = op.join {
                    optionals.push(format!("\tJOIN = {}", it));
                };
                if op.hashes {
                    optionals.push(String::from("\tHASHES"));
                };
                if op.merges {
                    optionals.push(String::from("\tMERGES"));
                };

                let left_arg = self.fn_args.get(0).ok_or_else(|| {
                    eyre_err!("Did not find `left_arg` for operator `{}`.", self.name)
                })?;
                let left_arg_graph_index = context
                    .graph
                    .neighbors_undirected(self_index)
                    .find(|neighbor| match &context.graph[*neighbor] {
                        SqlGraphEntity::Type(ty) => ty.id_matches(&left_arg.ty_id),
                        _ => false,
                    })
                    .ok_or_else(|| eyre_err!("Could not find left arg function in graph."))?;
                let right_arg = self.fn_args.get(1).ok_or_else(|| {
                    eyre_err!("Did not find `left_arg` for operator `{}`.", self.name)
                })?;
                let right_arg_graph_index = context
                    .graph
                    .neighbors_undirected(self_index)
                    .find(|neighbor| match &context.graph[*neighbor] {
                        SqlGraphEntity::Type(ty) => ty.id_matches(&right_arg.ty_id),
                        _ => false,
                    })
                    .ok_or_else(|| eyre_err!("Could not find right arg function in graph."))?;

                let operator_sql = format!("\n\
                                        -- {file}:{line}\n\
                                        -- {module_path}::{name}\n\
                                        CREATE OPERATOR {opname} (\n\
                                            \tPROCEDURE=\"{name}\",\n\
                                            \tLEFTARG={schema_prefix_left}{left_arg}, /* {left_name} */\n\
                                            \tRIGHTARG={schema_prefix_right}{right_arg}{maybe_comma} /* {right_name} */\n\
                                            {optionals}\
                                        );\
                                    ",
                                           opname = op.opname.unwrap(),
                                           file = self.file,
                                           line = self.line,
                                           name = self.name,
                                           module_path = self.module_path,
                                           left_name = left_arg.full_path,
                                           right_name = right_arg.full_path,
                                           schema_prefix_left = context.schema_prefix_for(&left_arg_graph_index),
                                           left_arg = context.type_id_to_sql_type(left_arg.ty_id).ok_or_else(|| eyre_err!("Failed to map argument `{}` type `{}` to SQL type while building operator `{}`.", left_arg.pattern, left_arg.full_path, self.name))?,
                                           schema_prefix_right = context.schema_prefix_for(&right_arg_graph_index),
                                           right_arg = context.type_id_to_sql_type(right_arg.ty_id).ok_or_else(|| eyre_err!("Failed to map argument `{}` type `{}` to SQL type while building operator `{}`.", right_arg.pattern, right_arg.full_path, self.name))?,
                                           maybe_comma = if optionals.len() >= 1 { "," } else { "" },
                                           optionals = optionals.join(",\n") + "\n"
                );
                tracing::debug!(sql = %operator_sql);
                ext_sql + &operator_sql
            }
            (None, None) | (Some(_), Some(_)) | (Some(_), None) => ext_sql,
        };
        Ok(rendered)
    }
}
