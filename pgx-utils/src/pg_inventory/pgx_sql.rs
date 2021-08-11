use eyre::eyre as eyre_err;
use std::{any::TypeId, collections::HashMap, fmt::Debug};

use petgraph::{dot::Dot, graph::NodeIndex, stable_graph::StableGraph};
use tracing::instrument;

use crate::pg_inventory::{DotIdentifier, InventorySqlDeclaredEntity};

use super::{ControlFile, InventoryExtensionSql, InventoryExtensionSqlPositioningRef, InventoryPgExtern, InventoryPgExternReturn, InventoryPostgresEnum, InventoryPostgresHash, InventoryPostgresOrd, InventoryPostgresType, InventorySchema, RustSourceOnlySqlMapping, RustSqlMapping, SqlDeclaredEntity, SqlGraphEntity, ToSql};

/// A generator for SQL.
///
/// Consumes a base mapping of types (typically `pgx::DEFAULT_TYPEID_SQL_MAPPING`), a
/// [`ControlFile`], and collections of inventory types for each SQL entity.
///
/// During construction, a Directed Acyclic Graph is formed out the dependencies. For example,
/// an item `detect_dog(x: &[u8]) -> animals::Dog` would have have a relationship with
/// `animals::Dog`.
///
/// Typically, [`PgxSql`] types are constructed in a `pgx::pg_binary_magic!()` call in a binary
/// out of inventory items collected during a `pgx::pg_module_magic!()` call in a library.
#[derive(Debug, Clone)]
pub struct PgxSql<'a> {
    pub type_mappings: HashMap<TypeId, RustSqlMapping>,
    pub source_mappings: HashMap<String, RustSourceOnlySqlMapping>,
    pub control: &'a ControlFile,
    pub graph: StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    pub graph_root: NodeIndex,
    pub graph_bootstrap: Option<NodeIndex>,
    pub graph_finalize: Option<NodeIndex>,
    pub schemas: HashMap<&'a InventorySchema, NodeIndex>,
    pub extension_sqls: HashMap<&'a InventoryExtensionSql, NodeIndex>,
    pub externs: HashMap<&'a InventoryPgExtern, NodeIndex>,
    pub types: HashMap<&'a InventoryPostgresType, NodeIndex>,
    pub builtin_types: HashMap<&'a str, NodeIndex>,
    pub enums: HashMap<&'a InventoryPostgresEnum, NodeIndex>,
    pub ords: HashMap<&'a InventoryPostgresOrd, NodeIndex>,
    pub hashes: HashMap<&'a InventoryPostgresHash, NodeIndex>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum SqlGraphRelationship {
    RequiredBy,
    RequiredByArg,
    RequiredByReturn,
}

impl<'a> PgxSql<'a> {
    #[instrument(
        level = "info",
        skip(
            control,
            type_mappings,
            source_mappings,
            schemas,
            extension_sqls,
            externs,
            types,
            enums,
            ords,
            hashes
        )
    )]
    pub fn build(
        control: &'a ControlFile,
        type_mappings: impl Iterator<Item = RustSqlMapping>,
        source_mappings: impl Iterator<Item = RustSourceOnlySqlMapping>,
        schemas: impl Iterator<Item = &'a InventorySchema>,
        extension_sqls: impl Iterator<Item = &'a InventoryExtensionSql>,
        externs: impl Iterator<Item = &'a InventoryPgExtern>,
        types: impl Iterator<Item = &'a InventoryPostgresType>,
        enums: impl Iterator<Item = &'a InventoryPostgresEnum>,
        ords: impl Iterator<Item = &'a InventoryPostgresOrd>,
        hashes: impl Iterator<Item = &'a InventoryPostgresHash>,
    ) -> eyre::Result<Self> {
        let mut graph = StableGraph::new();

        let root = graph.add_node(SqlGraphEntity::ExtensionRoot(control));

        // The initial build phase.
        //
        // Notably, we do not set non-root edges here. We do that in a second step. This is
        // primarily because externs, types, operators, and the like tend to intertwine. If we tried
        // to do it here, we'd find ourselves trying to create edges to non-existing entities.

        // Both of these must be unique, so we can only hold one.
        // Populate nodes, but don't build edges until we know if there is a bootstrap/finalize.
        let (mapped_extension_sqls, bootstrap, finalize) =
            initialize_extension_sqls(&mut graph, root, extension_sqls)?;
        let mapped_schemas = initialize_schemas(&mut graph, bootstrap, finalize, schemas)?;
        let mapped_enums = initialize_enums(&mut graph, root, bootstrap, finalize, enums)?;
        let mapped_types = initialize_types(&mut graph, root, bootstrap, finalize, types)?;
        let (mapped_externs, mapped_builtin_types) = initialize_externs(
            &mut graph,
            root,
            bootstrap,
            finalize,
            externs,
            &mapped_types,
            &mapped_enums,
        )?;
        let mapped_ords = initialize_ords(&mut graph, root, bootstrap, finalize, ords)?;
        let mapped_hashes = initialize_hashes(&mut graph, root, bootstrap, finalize, hashes)?;

        // Now we can circle back and build up the edge sets.
        connect_schemas(&mut graph, &mapped_schemas, root);
        connect_extension_sqls(
            &mut graph,
            &mapped_extension_sqls,
            &mapped_schemas,
            &mapped_types,
            &mapped_enums,
            &mapped_externs,
        );
        connect_enums(&mut graph, &mapped_enums, &mapped_schemas);
        connect_types(&mut graph, &mapped_types, &mapped_schemas);
        connect_externs(
            &mut graph,
            &mapped_externs,
            &mapped_schemas,
            &mapped_types,
            &mapped_enums,
            &mapped_builtin_types,
            &mapped_extension_sqls,
        );
        connect_ords(
            &mut graph,
            &mapped_ords,
            &mapped_schemas,
            &mapped_types,
            &mapped_enums,
        );
        connect_hashes(
            &mut graph,
            &mapped_hashes,
            &mapped_schemas,
            &mapped_types,
            &mapped_enums,
        );

        let mut this = Self {
            type_mappings: type_mappings.map(|x| (x.id, x)).collect(),
            source_mappings: source_mappings.map(|x| (x.rust.clone(), x)).collect(),
            control: &control,
            schemas: mapped_schemas,
            extension_sqls: mapped_extension_sqls,
            externs: mapped_externs,
            types: mapped_types,
            builtin_types: mapped_builtin_types,
            enums: mapped_enums,
            ords: mapped_ords,
            hashes: mapped_hashes,
            graph: graph,
            graph_root: root,
            graph_bootstrap: bootstrap,
            graph_finalize: finalize,
        };
        this.register_types();
        Ok(this)
    }

    #[instrument(level = "info", skip(self))]
    pub fn to_file(&self, file: impl AsRef<str> + Debug) -> eyre::Result<()> {
        use std::{
            fs::{create_dir_all, File},
            io::Write,
            path::Path,
        };
        let generated = self.to_sql()?;
        let path = Path::new(file.as_ref());

        let parent = path.parent();
        if let Some(parent) = parent {
            create_dir_all(parent)?;
        }
        let mut out = File::create(path)?;
        write!(out, "{}", generated)?;
        Ok(())
    }

    #[instrument(level = "info", err, skip(self))]
    pub fn to_dot(&self, file: impl AsRef<str> + Debug) -> eyre::Result<()> {
        use std::{
            fs::{create_dir_all, File},
            io::Write,
            path::Path,
        };
        let generated = Dot::with_attr_getters(
            &self.graph,
            &[
                petgraph::dot::Config::EdgeNoLabel,
                petgraph::dot::Config::NodeNoLabel,
            ],
            &|_graph, edge| match edge.weight() {
                SqlGraphRelationship::RequiredBy => format!(r#"color = "gray""#),
                SqlGraphRelationship::RequiredByArg => format!(r#"color = "black""#),
                SqlGraphRelationship::RequiredByReturn => {
                    format!(r#"dir = "back", color = "black""#)
                }
            },
            &|_graph, (_index, node)| {
                match node {
                    // Colors derived from https://www.schemecolor.com/touch-of-creativity.php
                    SqlGraphEntity::Schema(_item) => format!(
                        "label = \"{}\", weight = 6, shape = \"tab\"",
                        node.dot_identifier()
                    ),
                    SqlGraphEntity::Function(_item) => format!(
                        "label = \"{}\", penwidth = 0, style = \"filled\", fillcolor = \"#ADC7C6\", weight = 4, shape = \"box\"",
                        node.dot_identifier()
                    ),
                    SqlGraphEntity::Type(_item) => format!(
                        "label = \"{}\", penwidth = 0, style = \"filled\", fillcolor = \"#AE9BBD\", weight = 5, shape = \"oval\"",
                        node.dot_identifier()
                    ),
                    SqlGraphEntity::BuiltinType(_item) => format!(
                        "label = \"{}\", shape = \"plain\"",
                        node.dot_identifier()
                    ),
                    SqlGraphEntity::Enum(_item) => format!(
                        "label = \"{}\", penwidth = 0, style = \"filled\", fillcolor = \"#C9A7C8\", weight = 5, shape = \"oval\"",
                        node.dot_identifier()
                    ),
                    SqlGraphEntity::Ord(_item) => format!(
                        "label = \"{}\", penwidth = 0, style = \"filled\", fillcolor = \"#FFCFD3\", weight = 5, shape = \"diamond\"",
                        node.dot_identifier()
                    ),
                    SqlGraphEntity::Hash(_item) => format!(
                        "label = \"{}\", penwidth = 0, style = \"filled\", fillcolor = \"#FFE4E0\", weight = 5, shape = \"diamond\"",
                        node.dot_identifier()
                    ),
                    SqlGraphEntity::CustomSql(_item) => format!(
                        "label = \"{}\", weight = 3, shape = \"signature\"",
                        node.dot_identifier()
                    ),
                    SqlGraphEntity::ExtensionRoot(_item) => format!(
                        "label = \"{}\", shape = \"cylinder\"",
                        node.dot_identifier()
                    ),
                }
            },
        );
        let path = Path::new(file.as_ref());

        let parent = path.parent();
        if let Some(parent) = parent {
            create_dir_all(parent)?;
        }
        let mut out = File::create(path)?;
        write!(out, "{:?}", generated)?;
        Ok(())
    }

    pub fn schema_alias_of(&self, item_index: &NodeIndex) -> Option<String> {
        self.graph
            .neighbors_undirected(*item_index)
            .flat_map(|neighbor_index| match &self.graph[neighbor_index] {
                SqlGraphEntity::Schema(s) => Some(String::from(s.name)),
                SqlGraphEntity::ExtensionRoot(control) => {
                    if !control.relocatable {
                        control.schema.clone()
                    } else {
                        Some(String::from("@extname@"))
                    }
                }
                _ => None,
            })
            .next()
    }

    pub fn schema_prefix_for(&self, target: &NodeIndex) -> String {
        self.schema_alias_of(target)
            .map(|v| (v + ".").to_string())
            .unwrap_or_else(|| "".to_string())
    }

    #[instrument(level = "info", skip(self))]
    pub fn to_sql(&self) -> eyre::Result<String> {
        let mut full_sql = String::new();
        for step_id in petgraph::algo::toposort(&self.graph, None)
            .map_err(|_e| eyre_err!("Depgraph was Cyclic."))?
        {
            let step = &self.graph[step_id];

            let sql = step.to_sql(self)?;

            if !sql.is_empty() {
                full_sql.push_str(&sql);
                full_sql.push('\n');
            }
        }
        Ok(full_sql)
    }

    #[instrument(level = "info", skip(self))]
    pub fn register_types(&mut self) {
        for (item, _index) in self.enums.clone() {
            for mapping in &item.mappings {
                assert_eq!(
                    self.type_mappings.insert(mapping.id, mapping.clone()),
                    None,
                    "Cannot map `{}` twice.",
                    item.full_path,
                );
            }
        }
        for (item, _index) in self.types.clone() {
            for mapping in &item.mappings {
                assert_eq!(
                    self.type_mappings.insert(mapping.id, mapping.clone()),
                    None,
                    "Cannot map `{}` twice.",
                    item.full_path,
                );
            }
        }
    }

    pub fn has_sql_declared_entity(
        &self,
        identifier: &SqlDeclaredEntity,
    ) -> Option<&InventorySqlDeclaredEntity> {
        self.extension_sqls.iter().find_map(|(item, _index)| {
            let retval = item.creates.iter().find_map(|create_entity| {
                if create_entity.has_sql_declared_entity(identifier) {
                    Some(create_entity)
                } else {
                    None
                }
            });
            retval
        })
    }

    pub fn type_id_to_sql_type(&self, id: TypeId) -> Option<String> {
        self.type_mappings.get(&id).map(|f| f.sql.clone())
    }

    pub fn source_only_to_sql_type(&self, ty_source: &str) -> Option<String> {
        self.source_mappings.get(ty_source).map(|f| f.sql.clone())
    }

    pub fn map_type_to_sql_type<T: 'static>(&mut self, sql: impl AsRef<str> + Debug) {
        let sql = sql.as_ref().to_string();
        self.type_mappings.insert(
            TypeId::of::<T>(),
            RustSqlMapping {
                rust: core::any::type_name::<T>().to_string(),
                sql: sql.clone(),
                id: core::any::TypeId::of::<T>(),
            },
        );
    }
}

fn build_base_edges(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    index: NodeIndex,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
) {
    graph.add_edge(root, index, SqlGraphRelationship::RequiredBy);
    if let Some(bootstrap) = bootstrap {
        graph.add_edge(bootstrap, index, SqlGraphRelationship::RequiredBy);
    }
    if let Some(finalize) = finalize {
        graph.add_edge(index, finalize, SqlGraphRelationship::RequiredBy);
    }
}

fn initialize_extension_sqls<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    root: NodeIndex,
    extension_sqls: impl Iterator<Item = &'a InventoryExtensionSql>,
) -> eyre::Result<(
    HashMap<&'a InventoryExtensionSql, NodeIndex>,
    Option<NodeIndex>,
    Option<NodeIndex>,
)> {
    let mut bootstrap = None;
    let mut finalize = None;
    let mut mapped_extension_sqls = HashMap::default();

    for item in extension_sqls {
        let index = graph.add_node(item.into());

        mapped_extension_sqls.insert(item, index);
        if item.bootstrap {
            if let Some(_index) = bootstrap {
                return Err(eyre_err!(
                    "Cannot have multiple `extension_sql!()` with `bootstrap` positioning."
                ));
            }
            bootstrap = Some(index)
        }
        if item.finalize {
            if let Some(_index) = finalize {
                return Err(eyre_err!(
                    "Cannot have multiple `extension_sql!()` with `finalize` positioning."
                ));
            }
            finalize = Some(index)
        }
    }
    for (item, index) in &mapped_extension_sqls {
        graph.add_edge(root, *index, SqlGraphRelationship::RequiredBy);
        if !item.bootstrap {
            if let Some(bootstrap) = bootstrap {
                graph.add_edge(bootstrap, *index, SqlGraphRelationship::RequiredBy);
            }
        }
        if !item.finalize {
            if let Some(finalize) = finalize {
                graph.add_edge(*index, finalize, SqlGraphRelationship::RequiredBy);
            }
        }
    }
    Ok((mapped_extension_sqls, bootstrap, finalize))
}

fn connect_extension_sqls<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    extension_sqls: &HashMap<&'a InventoryExtensionSql, NodeIndex>,
    schemas: &HashMap<&'a InventorySchema, NodeIndex>,
    types: &HashMap<&'a InventoryPostgresType, NodeIndex>,
    enums: &HashMap<&'a InventoryPostgresEnum, NodeIndex>,
    externs: &HashMap<&'a InventoryPgExtern, NodeIndex>,
) {
    for (item, &index) in extension_sqls {
        for (schema_item, &schema_index) in schemas {
            if item.module_path == schema_item.module_path {
                tracing::trace!(from = ?item.identifier(), to = schema_item.module_path, "Adding ExtensionSQL after Schema edge.");
                graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for after in &item.after {
            match after {
                InventoryExtensionSqlPositioningRef::FullPath(path) => {
                    for (other, other_index) in types {
                        if other.full_path.ends_with(*path) {
                            tracing::trace!(from = ?item.identifier(), to = ?other.full_path, "Adding ExtensionSQL after Type edge.");
                            graph.add_edge(*other_index, index, SqlGraphRelationship::RequiredBy);
                            break;
                        }
                    }
                    for (other, other_index) in enums {
                        if other.full_path.ends_with(*path) {
                            tracing::trace!(from = ?item.identifier(), to = ?other.full_path, "Adding ExtensionSQL after Enum edge.");
                            graph.add_edge(*other_index, index, SqlGraphRelationship::RequiredBy);
                            break;
                        }
                    }
                    for (other, other_index) in externs {
                        if other.full_path.ends_with(*path) {
                            tracing::trace!(from = ?item.identifier(), to = ?other.full_path, "Adding ExtensionSQL after Extern edge.");
                            graph.add_edge(*other_index, index, SqlGraphRelationship::RequiredBy);
                            break;
                        }
                    }
                }
                InventoryExtensionSqlPositioningRef::Name(name) => {
                    for (other, other_index) in extension_sqls {
                        if other.name == Some(name) {
                            tracing::trace!(from = ?item.identifier(), to = ?other.identifier(), "Adding ExtensionSQL after ExtensionSql edge.");
                            graph.add_edge(*other_index, index, SqlGraphRelationship::RequiredBy);
                            break;
                        }
                    }
                }
            }
        }
        for before in &item.before {
            match before {
                InventoryExtensionSqlPositioningRef::FullPath(path) => {
                    for (other, other_index) in types {
                        if other.full_path == *path {
                            tracing::trace!(from = ?item.full_path, to = ?other.full_path, "Adding ExtensionSQL after Type edge.");
                            graph.add_edge(index, *other_index, SqlGraphRelationship::RequiredBy);
                            break;
                        }
                    }
                    for (other, other_index) in enums {
                        if other.full_path == *path {
                            tracing::trace!(from = ?item.full_path, to = ?other.full_path, "Adding ExtensionSQL after Enum edge.");
                            graph.add_edge(index, *other_index, SqlGraphRelationship::RequiredBy);
                            break;
                        }
                    }
                    for (other, other_index) in externs {
                        if other.full_path == *path {
                            tracing::trace!(from = ?item.full_path, to = ?other.full_path, "Adding ExtensionSQL after Extern edge.");
                            graph.add_edge(index, *other_index, SqlGraphRelationship::RequiredBy);
                            break;
                        }
                    }
                }
                InventoryExtensionSqlPositioningRef::Name(name) => {
                    for (other, other_index) in extension_sqls {
                        if other.name == Some(name) {
                            tracing::trace!(from = ?item.full_path, to = ?other.full_path, "Adding ExtensionSQL after ExtensionSql edge.");
                            graph.add_edge(index, *other_index, SqlGraphRelationship::RequiredBy);
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn initialize_schemas<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    schemas: impl Iterator<Item = &'a InventorySchema>,
) -> eyre::Result<HashMap<&'a InventorySchema, NodeIndex>> {
    let mut mapped_schemas = HashMap::default();
    for item in schemas {
        let index = graph.add_node(item.into());
        mapped_schemas.insert(item, index);
        if let Some(bootstrap) = bootstrap {
            graph.add_edge(bootstrap, index, SqlGraphRelationship::RequiredBy);
        }
        if let Some(finalize) = finalize {
            graph.add_edge(index, finalize, SqlGraphRelationship::RequiredBy);
        }
    }
    Ok(mapped_schemas)
}

fn connect_schemas<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    schemas: &HashMap<&'a InventorySchema, NodeIndex>,
    root: NodeIndex,
) {
    for (_item, &index) in schemas {
        graph.add_edge(root, index, SqlGraphRelationship::RequiredBy);
    }
}

fn initialize_enums<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    enums: impl Iterator<Item = &'a InventoryPostgresEnum>,
) -> eyre::Result<HashMap<&'a InventoryPostgresEnum, NodeIndex>> {
    let mut mapped_enums = HashMap::default();
    for item in enums {
        let index = graph.add_node(item.into());
        mapped_enums.insert(item, index);
        build_base_edges(graph, index, root, bootstrap, finalize);
    }
    Ok(mapped_enums)
}

fn connect_enums<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    enums: &HashMap<&'a InventoryPostgresEnum, NodeIndex>,
    schemas: &HashMap<&'a InventorySchema, NodeIndex>,
) {
    for (item, &index) in enums {
        for (schema_item, &schema_index) in schemas {
            if item.module_path == schema_item.module_path {
                tracing::trace!(from = ?item.full_path, to = schema_item.module_path, "Adding Enum after Schema edge.");
                graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
    }
}

fn initialize_types<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    types: impl Iterator<Item = &'a InventoryPostgresType>,
) -> eyre::Result<HashMap<&'a InventoryPostgresType, NodeIndex>> {
    let mut mapped_types = HashMap::default();
    for item in types {
        let index = graph.add_node(item.into());
        mapped_types.insert(item, index);
        build_base_edges(graph, index, root, bootstrap, finalize);
    }
    Ok(mapped_types)
}

fn connect_types<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    types: &HashMap<&'a InventoryPostgresType, NodeIndex>,
    schemas: &HashMap<&'a InventorySchema, NodeIndex>,
) {
    for (item, &index) in types {
        for (schema_item, &schema_index) in schemas {
            if item.module_path == schema_item.module_path {
                tracing::trace!(from = ?item.full_path, to = schema_item.module_path, "Adding Type after Schema edge.");
                graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
    }
}

fn initialize_externs<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    externs: impl Iterator<Item = &'a InventoryPgExtern>,
    mapped_types: &HashMap<&'a InventoryPostgresType, NodeIndex>,
    mapped_enums: &HashMap<&'a InventoryPostgresEnum, NodeIndex>,
) -> eyre::Result<(
    HashMap<&'a InventoryPgExtern, NodeIndex>,
    HashMap<&'a str, NodeIndex>,
)> {
    let mut mapped_externs = HashMap::default();
    let mut mapped_builtin_types = HashMap::default();
    for item in externs {
        let index = graph.add_node(item.into());
        mapped_externs.insert(item, index);
        build_base_edges(graph, index, root, bootstrap, finalize);

        for arg in &item.fn_args {
            let mut found = false;
            for (ty_item, &_ty_index) in mapped_types {
                if ty_item.id_matches(&arg.ty_id) {
                    found = true;
                    break;
                }
            }
            for (ty_item, &_ty_index) in mapped_enums {
                if ty_item.id == arg.ty_id {
                    found = true;
                    break;
                }
            }
            if !found {
                mapped_builtin_types
                    .entry(arg.full_path)
                    .or_insert_with(|| graph.add_node(SqlGraphEntity::BuiltinType(arg.full_path)));
            }
        }

        match &item.fn_return {
            InventoryPgExternReturn::None | InventoryPgExternReturn::Trigger => (),
            InventoryPgExternReturn::Type { id, full_path, .. }
            | InventoryPgExternReturn::SetOf { id, full_path, .. } => {
                let mut found = false;
                for (ty_item, &_ty_index) in mapped_types {
                    if ty_item.id_matches(id) {
                        found = true;
                        break;
                    }
                }
                for (ty_item, &_ty_index) in mapped_enums {
                    if ty_item.id == *id {
                        found = true;
                        break;
                    }
                }
                if !found {
                    mapped_builtin_types
                        .entry(full_path)
                        .or_insert_with(|| graph.add_node(SqlGraphEntity::BuiltinType(full_path)));
                }
            }
            InventoryPgExternReturn::Iterated(iterated_returns) => {
                for iterated_return in iterated_returns {
                    let mut found = false;
                    for (ty_item, &_ty_index) in mapped_types {
                        if ty_item.id_matches(&iterated_return.0) {
                            found = true;
                            break;
                        }
                    }
                    for (ty_item, &_ty_index) in mapped_enums {
                        if ty_item.id == iterated_return.0 {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        mapped_builtin_types
                            .entry(iterated_return.1)
                            .or_insert_with(|| {
                                graph.add_node(SqlGraphEntity::BuiltinType(iterated_return.1))
                            });
                    }
                }
            }
        }
    }
    Ok((mapped_externs, mapped_builtin_types))
}

fn connect_externs<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    externs: &HashMap<&'a InventoryPgExtern, NodeIndex>,
    schemas: &HashMap<&'a InventorySchema, NodeIndex>,
    types: &HashMap<&'a InventoryPostgresType, NodeIndex>,
    enums: &HashMap<&'a InventoryPostgresEnum, NodeIndex>,
    builtin_types: &HashMap<&'a str, NodeIndex>,
    extension_sqls: &HashMap<&'a InventoryExtensionSql, NodeIndex>,
) {
    for (item, &index) in externs {
        for (schema_item, &schema_index) in schemas {
            if item.module_path == schema_item.module_path {
                tracing::trace!(from = ?item.full_path, to = schema_item.module_path, "Adding Extern after Schema edge.");
                graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for arg in &item.fn_args {
            let mut found = false;
            for (ty_item, &ty_index) in types {
                if ty_item.id_matches(&arg.ty_id) {
                    tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Extern after Type (due to argument) edge.");
                    graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredByArg);
                    found = true;
                    break;
                }
            }
            if !found {
                for (enum_item, &enum_index) in enums {
                    if enum_item.id_matches(&arg.ty_id) {
                        tracing::trace!(from = ?item.full_path, to = enum_item.full_path, "Adding Extern after Enum (due to argument) edge.");
                        graph.add_edge(enum_index, index, SqlGraphRelationship::RequiredByArg);
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                let builtin_index = builtin_types
                    .get(arg.full_path)
                    .expect(&format!("Could not fetch Builtin Type {}.", arg.full_path));
                tracing::trace!(from = ?item.full_path, to = arg.full_path, "Adding Extern(arg) after BuiltIn Type (due to argument) edge.");
                graph.add_edge(*builtin_index, index, SqlGraphRelationship::RequiredByArg);
            }
            if !found {
                for (ext_item, ext_index) in extension_sqls {
                    if let Some(_) = ext_item.has_sql_declared_entity(&SqlDeclaredEntity::Type(
                        arg.full_path.to_string(),
                    )) {
                        tracing::trace!(from = ?item.full_path, to = arg.full_path, "Adding Extern(arg) after Extension SQL (due to argument) edge.");
                        graph.add_edge(*ext_index, index, SqlGraphRelationship::RequiredByArg);
                    } else if let Some(_) = ext_item.has_sql_declared_entity(
                        &SqlDeclaredEntity::Enum(arg.full_path.to_string()),
                    ) {
                        tracing::trace!(from = ?item.full_path, to = arg.full_path, "Adding Extern(arg) after Extension SQL (due to argument) edge.");
                        graph.add_edge(*ext_index, index, SqlGraphRelationship::RequiredByArg);
                    }
                }
            }
        }
        match &item.fn_return {
            InventoryPgExternReturn::None | InventoryPgExternReturn::Trigger => (),
            InventoryPgExternReturn::Type { id, full_path, .. }
            | InventoryPgExternReturn::SetOf { id, full_path, .. } => {
                let mut found = false;
                for (ty_item, &ty_index) in types {
                    if ty_item.id_matches(id) {
                        tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Extern after Type (due to return) edge.");
                        graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredByReturn);
                        found = true;
                        break;
                    }
                }
                if !found {
                    for (ty_item, &ty_index) in enums {
                        if ty_item.id_matches(id) {
                            tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Extern after Enum (due to return) edge.");
                            graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredByReturn);
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    let builtin_index = builtin_types
                        .get(full_path)
                        .expect(&format!("Could not fetch Builtin Type {}.", full_path));
                    tracing::trace!(from = ?item.full_path, to = full_path, "Adding Extern(return) after BuiltIn Type (due to return) edge.");
                    graph.add_edge(
                        *builtin_index,
                        index,
                        SqlGraphRelationship::RequiredByReturn,
                    );
                }
                if !found {
                    for (ext_item, ext_index) in extension_sqls {
                        if let Some(_) = ext_item.has_sql_declared_entity(&SqlDeclaredEntity::Type(
                            full_path.to_string(),
                        )) {
                            tracing::trace!(from = ?item.full_path, to = full_path, "Adding Extern(arg) after Extension SQL (due to argument) edge.");
                            graph.add_edge(*ext_index, index, SqlGraphRelationship::RequiredByArg);
                        } else if let Some(_) = ext_item.has_sql_declared_entity(
                            &SqlDeclaredEntity::Enum(full_path.to_string()),
                        ) {
                            tracing::trace!(from = ?item.full_path, to = full_path, "Adding Extern(arg) after Extension SQL (due to argument) edge.");
                            graph.add_edge(*ext_index, index, SqlGraphRelationship::RequiredByArg);
                        }
                    }
                }
            }
            InventoryPgExternReturn::Iterated(iterated_returns) => {
                for iterated_return in iterated_returns {
                    let mut found = false;
                    for (ty_item, &ty_index) in types {
                        if ty_item.id_matches(&iterated_return.0) {
                            tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Extern after Type (due to return) edge.");
                            graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredByReturn);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        for (ty_item, &ty_index) in enums {
                            if ty_item.id_matches(&iterated_return.0) {
                                tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Extern after Enum (due to return) edge.");
                                graph.add_edge(
                                    ty_index,
                                    index,
                                    SqlGraphRelationship::RequiredByReturn,
                                );
                                found = true;
                                break;
                            }
                        }
                    }
                    if !found {
                        let builtin_index = builtin_types.get(&iterated_return.1).expect(&format!(
                            "Could not fetch Builtin Type {}.",
                            iterated_return.1
                        ));
                        tracing::trace!(from = ?item.full_path, to = iterated_return.1, "Adding Extern after BuiltIn Type (due to return) edge.");
                        graph.add_edge(
                            *builtin_index,
                            index,
                            SqlGraphRelationship::RequiredByReturn,
                        );
                    }
                    if !found {
                        for (ext_item, ext_index) in extension_sqls {
                            if let Some(_) = ext_item.has_sql_declared_entity(
                                &SqlDeclaredEntity::Type(iterated_return.1.to_string()),
                            ) {
                                tracing::trace!(from = ?item.full_path, to = iterated_return.1, "Adding Extern(arg) after Extension SQL (due to argument) edge.");
                                graph.add_edge(
                                    *ext_index,
                                    index,
                                    SqlGraphRelationship::RequiredByArg,
                                );
                            } else if let Some(_) = ext_item.has_sql_declared_entity(
                                &SqlDeclaredEntity::Enum(iterated_return.1.to_string()),
                            ) {
                                tracing::trace!(from = ?item.full_path, to = iterated_return.1, "Adding Extern(arg) after Extension SQL (due to argument) edge.");
                                graph.add_edge(
                                    *ext_index,
                                    index,
                                    SqlGraphRelationship::RequiredByArg,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn initialize_ords<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    ords: impl Iterator<Item = &'a InventoryPostgresOrd>,
) -> eyre::Result<HashMap<&'a InventoryPostgresOrd, NodeIndex>> {
    let mut mapped_ords = HashMap::default();
    for item in ords {
        let index = graph.add_node(item.into());
        mapped_ords.insert(item, index);
        build_base_edges(graph, index, root, bootstrap, finalize);
    }
    Ok(mapped_ords)
}

fn connect_ords<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    ords: &HashMap<&'a InventoryPostgresOrd, NodeIndex>,
    schemas: &HashMap<&'a InventorySchema, NodeIndex>,
    types: &HashMap<&'a InventoryPostgresType, NodeIndex>,
    enums: &HashMap<&'a InventoryPostgresEnum, NodeIndex>,
) {
    for (item, &index) in ords {
        for (schema_item, &schema_index) in schemas {
            if item.module_path == schema_item.module_path {
                tracing::trace!(from = ?item.full_path, to = schema_item.module_path, "Adding Ord after Schema edge.");
                graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for (ty_item, &ty_index) in types {
            if ty_item.id_matches(&item.id) {
                tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Ord after Type edge.");
                graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for (ty_item, &ty_index) in enums {
            if ty_item.id_matches(&item.id) {
                tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Ord after Enum edge.");
                graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
    }
}

fn initialize_hashes<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    hashes: impl Iterator<Item = &'a InventoryPostgresHash>,
) -> eyre::Result<HashMap<&'a InventoryPostgresHash, NodeIndex>> {
    let mut mapped_hashes = HashMap::default();
    for item in hashes {
        let index = graph.add_node(item.into());
        mapped_hashes.insert(item, index);
        build_base_edges(graph, index, root, bootstrap, finalize);
    }
    Ok(mapped_hashes)
}

fn connect_hashes<'a>(
    graph: &mut StableGraph<SqlGraphEntity<'a>, SqlGraphRelationship>,
    hashes: &HashMap<&'a InventoryPostgresHash, NodeIndex>,
    schemas: &HashMap<&'a InventorySchema, NodeIndex>,
    types: &HashMap<&'a InventoryPostgresType, NodeIndex>,
    enums: &HashMap<&'a InventoryPostgresEnum, NodeIndex>,
) {
    for (item, &index) in hashes {
        for (schema_item, &schema_index) in schemas {
            if item.module_path == schema_item.module_path {
                tracing::trace!(from = ?item.full_path, to = schema_item.module_path, "Adding Hash after Schema edge.");
                graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for (ty_item, &ty_index) in types {
            if ty_item.id_matches(&item.id) {
                tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Hash after Type edge.");
                graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for (ty_item, &ty_index) in enums {
            if ty_item.id_matches(&item.id) {
                tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Hash after Enum edge.");
                graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
    }
}
