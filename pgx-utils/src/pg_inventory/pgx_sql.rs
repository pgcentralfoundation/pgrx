use eyre::eyre as eyre_err;
use std::{any::TypeId, collections::HashMap, fmt::Debug};

use petgraph::{dot::Dot, graph::NodeIndex, stable_graph::StableGraph};
use tracing::instrument;

use crate::pg_inventory::DotIdentifier;

use super::{ControlFile, InventoryExtensionSql, InventoryExtensionSqlPositioningRef, InventoryPgExtern, InventoryPgExternReturn, InventoryPostgresEnum, InventoryPostgresHash, InventoryPostgresOrd, InventoryPostgresType, InventorySchema, RustSqlMapping, SqlGraphEntity, ToSql};

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
        level = "debug",
        skip(
            control,
            type_mappings,
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
        type_mappings: impl Iterator<Item = (TypeId, RustSqlMapping)>,
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
        let mut bootstrap = None;
        let mut finalize = None;
        let mut mapped_extension_sqls = HashMap::default();
        // Populate nodes, but don't build edges until we know if there is a bootstrap/finalize.
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
        let mut mapped_enums = HashMap::default();
        for item in enums {
            let index = graph.add_node(item.into());
            mapped_enums.insert(item, index);
            graph.add_edge(root, index, SqlGraphRelationship::RequiredBy);
            if let Some(bootstrap) = bootstrap {
                graph.add_edge(bootstrap, index, SqlGraphRelationship::RequiredBy);
            }
            if let Some(finalize) = finalize {
                graph.add_edge(index, finalize, SqlGraphRelationship::RequiredBy);
            }
        }
        let mut mapped_types = HashMap::default();
        for item in types {
            let index = graph.add_node(item.into());
            mapped_types.insert(item, index);
            graph.add_edge(root, index, SqlGraphRelationship::RequiredBy);
            if let Some(bootstrap) = bootstrap {
                graph.add_edge(bootstrap, index, SqlGraphRelationship::RequiredBy);
            }
            if let Some(finalize) = finalize {
                graph.add_edge(index, finalize, SqlGraphRelationship::RequiredBy);
            }
        }
        let mut mapped_externs = HashMap::default();
        let mut mapped_builtin_types = HashMap::default();
        for item in externs {
            let index = graph.add_node(item.into());
            graph.add_edge(root, index, SqlGraphRelationship::RequiredBy);
            if let Some(bootstrap) = bootstrap {
                graph.add_edge(bootstrap, index, SqlGraphRelationship::RequiredBy);
            }
            if let Some(finalize) = finalize {
                graph.add_edge(index, finalize, SqlGraphRelationship::RequiredBy);
            }

            mapped_externs.insert(item, index);

            for arg in &item.fn_args {
                let mut found = false;
                for (ty_item, &_ty_index) in &mapped_types {
                    if ty_item.id_matches(&arg.ty_id) {
                        found = true;
                        break;
                    }
                }
                for (ty_item, &_ty_index) in &mapped_enums {
                    if ty_item.id == arg.ty_id {
                        found = true;
                        break;
                    }
                }
                if !found {
                    mapped_builtin_types
                        .entry(arg.full_path)
                        .or_insert_with(|| {
                            graph.add_node(SqlGraphEntity::BuiltinType(arg.full_path))
                        });
                }
            }

            match &item.fn_return {
                InventoryPgExternReturn::None | InventoryPgExternReturn::Trigger => (),
                InventoryPgExternReturn::Type { id, full_path, .. }
                | InventoryPgExternReturn::SetOf { id, full_path, .. } => {
                    let mut found = false;
                    for (ty_item, &_ty_index) in &mapped_types {
                        if ty_item.id_matches(id) {
                            found = true;
                            break;
                        }
                    }
                    for (ty_item, &_ty_index) in &mapped_enums {
                        if ty_item.id == *id {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        mapped_builtin_types.entry(full_path).or_insert_with(|| {
                            graph.add_node(SqlGraphEntity::BuiltinType(full_path))
                        });
                    }
                }
                InventoryPgExternReturn::Iterated(iterated_returns) => {
                    for iterated_return in iterated_returns {
                        let mut found = false;
                        for (ty_item, &_ty_index) in &mapped_types {
                            if ty_item.id_matches(&iterated_return.0) {
                                found = true;
                                break;
                            }
                        }
                        for (ty_item, &_ty_index) in &mapped_enums {
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
        let mut mapped_ords = HashMap::default();
        for item in ords {
            let index = graph.add_node(item.into());
            mapped_ords.insert(item, index);
            graph.add_edge(root, index, SqlGraphRelationship::RequiredBy);
            if let Some(bootstrap) = bootstrap {
                graph.add_edge(bootstrap, index, SqlGraphRelationship::RequiredBy);
            }
            if let Some(finalize) = finalize {
                graph.add_edge(index, finalize, SqlGraphRelationship::RequiredBy);
            }
        }
        let mut mapped_hashes = HashMap::default();
        for item in hashes {
            let index = graph.add_node(item.into());
            mapped_hashes.insert(item, index);
            graph.add_edge(root, index, SqlGraphRelationship::RequiredBy);
            if let Some(bootstrap) = bootstrap {
                graph.add_edge(bootstrap, index, SqlGraphRelationship::RequiredBy);
            }
            if let Some(finalize) = finalize {
                graph.add_edge(index, finalize, SqlGraphRelationship::RequiredBy);
            }
        }


        // Now we can circle back and build up the edge sets.
        for (_item, &index) in &mapped_schemas {
            graph
                .add_edge(root, index, SqlGraphRelationship::RequiredBy);
        }
        for (item, &index) in &mapped_extension_sqls {
            for (schema_item, &schema_index) in &mapped_schemas {
                if item.module_path == schema_item.module_path {
                    tracing::trace!(from = ?item.identifier(), to = schema_item.module_path, "Adding ExtensionSQL after Schema edge.");
                    graph
                        .add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                    break;
                }
            }
            for after in &item.after {
                match after {
                    InventoryExtensionSqlPositioningRef::FullPath(path) => {
                        for (other, other_index) in &mapped_types {
                            if other.full_path.ends_with(*path) {
                                tracing::trace!(from = ?item.identifier(), to = ?other.full_path, "Adding ExtensionSQL after Type edge.");
                                graph.add_edge(
                                    *other_index,
                                    index,
                                    SqlGraphRelationship::RequiredBy,
                                );
                                break;
                            }
                        }
                        for (other, other_index) in &mapped_enums {
                            if other.full_path.ends_with(*path) {
                                tracing::trace!(from = ?item.identifier(), to = ?other.full_path, "Adding ExtensionSQL after Enum edge.");
                                graph.add_edge(
                                    *other_index,
                                    index,
                                    SqlGraphRelationship::RequiredBy,
                                );
                                break;
                            }
                        }
                        for (other, other_index) in &mapped_externs {
                            if other.full_path.ends_with(*path) {
                                tracing::trace!(from = ?item.identifier(), to = ?other.full_path, "Adding ExtensionSQL after Extern edge.");
                                graph.add_edge(
                                    *other_index,
                                    index,
                                    SqlGraphRelationship::RequiredBy,
                                );
                                break;
                            }
                        }
                    }
                    InventoryExtensionSqlPositioningRef::Name(name) => {
                        for (other, other_index) in &mapped_extension_sqls {
                            if other.name == Some(name) {
                                tracing::trace!(from = ?item.identifier(), to = ?other.identifier(), "Adding ExtensionSQL after ExtensionSql edge.");
                                graph.add_edge(
                                    *other_index,
                                    index,
                                    SqlGraphRelationship::RequiredBy,
                                );
                                break;
                            }
                        }
                    }
                }
            }
            for before in &item.before {
                match before {
                    InventoryExtensionSqlPositioningRef::FullPath(path) => {
                        for (other, other_index) in &mapped_types {
                            if other.full_path == *path {
                                tracing::trace!(from = ?item.full_path, to = ?other.full_path, "Adding ExtensionSQL after Type edge.");
                                graph.add_edge(
                                    index,
                                    *other_index,
                                    SqlGraphRelationship::RequiredBy,
                                );
                                break;
                            }
                        }
                        for (other, other_index) in &mapped_enums {
                            if other.full_path == *path {
                                tracing::trace!(from = ?item.full_path, to = ?other.full_path, "Adding ExtensionSQL after Enum edge.");
                                graph.add_edge(
                                    index,
                                    *other_index,
                                    SqlGraphRelationship::RequiredBy,
                                );
                                break;
                            }
                        }
                        for (other, other_index) in &mapped_externs {
                            if other.full_path == *path {
                                tracing::trace!(from = ?item.full_path, to = ?other.full_path, "Adding ExtensionSQL after Extern edge.");
                                graph.add_edge(
                                    index,
                                    *other_index,
                                    SqlGraphRelationship::RequiredBy,
                                );
                                break;
                            }
                        }
                    }
                    InventoryExtensionSqlPositioningRef::Name(name) => {
                        for (other, other_index) in &mapped_extension_sqls {
                            if other.name == Some(name) {
                                tracing::trace!(from = ?item.full_path, to = ?other.full_path, "Adding ExtensionSQL after ExtensionSql edge.");
                                graph.add_edge(
                                    index,
                                    *other_index,
                                    SqlGraphRelationship::RequiredBy,
                                );
                                break;
                            }
                        }
                    }
                }
            }
        }
        for (item, &index) in &mapped_enums {
            for (schema_item, &schema_index) in &mapped_schemas {
                if item.module_path == schema_item.module_path {
                    tracing::trace!(from = ?item.full_path, to = schema_item.module_path, "Adding Enum after Schema edge.");
                    graph
                        .add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                    break;
                }
            }
        }
        for (item, &index) in &mapped_types {
            for (schema_item, &schema_index) in &mapped_schemas {
                if item.module_path == schema_item.module_path {
                    tracing::trace!(from = ?item.full_path, to = schema_item.module_path, "Adding Type after Schema edge.");
                    graph
                        .add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                    break;
                }
            }
        }
        for (item, &index) in &mapped_externs {
            for (schema_item, &schema_index) in &mapped_schemas {
                if item.module_path == schema_item.module_path {
                    tracing::trace!(from = ?item.full_path, to = schema_item.module_path, "Adding Extern after Schema edge.");
                    graph
                        .add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                    break;
                }
            }
            for arg in &item.fn_args {
                let mut found = false;
                for (ty_item, &ty_index) in &mapped_types {
                    if ty_item.id_matches(&arg.ty_id) {
                        tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Extern after Type (due to argument) edge.");
                        graph
                            .add_edge(ty_index, index, SqlGraphRelationship::RequiredByArg);
                        found = true;
                        break;
                    }
                }
                for (ty_item, &ty_index) in &mapped_enums {
                    if ty_item.id_matches(&arg.ty_id) {
                        tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Extern after Enum (due to argument) edge.");
                        graph
                            .add_edge(ty_index, index, SqlGraphRelationship::RequiredByArg);
                        found = true;
                        break;
                    }
                }
                if !found {
                    let builtin_index = mapped_builtin_types
                        .get(arg.full_path)
                        .expect(&format!("Could not fetch Builtin Type {}.", arg.full_path));
                    tracing::trace!(from = ?item.full_path, to = arg.full_path, "Adding Extern(arg) after BuiltIn Type (due to argument) edge.");
                    graph
                        .add_edge(*builtin_index, index, SqlGraphRelationship::RequiredByArg);
                }
            }
            match &item.fn_return {
                InventoryPgExternReturn::None | InventoryPgExternReturn::Trigger => (),
                InventoryPgExternReturn::Type { id, full_path, .. }
                | InventoryPgExternReturn::SetOf { id, full_path, .. } => {
                    let mut found = false;
                    for (ty_item, &ty_index) in &mapped_types {
                        if ty_item.id_matches(id) {
                            tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Extern after Type (due to return) edge.");
                            graph.add_edge(
                                ty_index,
                                index,
                                SqlGraphRelationship::RequiredByReturn,
                            );
                            found = true;
                            break;
                        }
                    }
                    for (ty_item, &ty_index) in &mapped_enums {
                        if ty_item.id_matches(id) {
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
                    if !found {
                        let builtin_index = mapped_builtin_types
                            .get(full_path)
                            .expect(&format!("Could not fetch Builtin Type {}.", full_path));
                        tracing::trace!(from = ?item.full_path, to = full_path, "Adding Extern(return) after BuiltIn Type (due to return) edge.");
                        graph.add_edge(
                            *builtin_index,
                            index,
                            SqlGraphRelationship::RequiredByReturn,
                        );
                    }
                }
                InventoryPgExternReturn::Iterated(iterated_returns) => {
                    for iterated_return in iterated_returns {
                        let mut found = false;
                        for (ty_item, &ty_index) in &mapped_types {
                            if ty_item.id_matches(&iterated_return.0) {
                                tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Extern after Type (due to return) edge.");
                                graph.add_edge(
                                    ty_index,
                                    index,
                                    SqlGraphRelationship::RequiredByReturn,
                                );
                                found = true;
                                break;
                            }
                        }
                        for (ty_item, &ty_index) in &mapped_enums {
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
                        if !found {
                            let builtin_index = mapped_builtin_types.get(&iterated_return.1).expect(
                                &format!("Could not fetch Builtin Type {}.", iterated_return.1),
                            );
                            tracing::trace!(from = ?item.full_path, to = iterated_return.1, "Adding Extern after BuiltIn Type (due to return) edge.");
                            graph.add_edge(
                                *builtin_index,
                                index,
                                SqlGraphRelationship::RequiredByReturn,
                            );
                        }
                    }
                }
            }
        }
        for (item, &index) in &mapped_ords {
            for (schema_item, &schema_index) in &mapped_schemas {
                if item.module_path == schema_item.module_path {
                    tracing::trace!(from = ?item.full_path, to = schema_item.module_path, "Adding Ord after Schema edge.");
                    graph
                        .add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                    break;
                }
            }
            for (ty_item, &ty_index) in &mapped_types {
                if ty_item.id_matches(&item.id) {
                    tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Ord after Type edge.");
                    graph
                        .add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                    break;
                }
            }
            for (ty_item, &ty_index) in &mapped_enums {
                if ty_item.id_matches(&item.id) {
                    tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Ord after Enum edge.");
                    graph
                        .add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                    break;
                }
            }
        }
        for (item, &index) in &mapped_hashes {
            for (schema_item, &schema_index) in &mapped_schemas {
                if item.module_path == schema_item.module_path {
                    tracing::trace!(from = ?item.full_path, to = schema_item.module_path, "Adding Hash after Schema edge.");
                    graph
                        .add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                    break;
                }
            }
            for (ty_item, &ty_index) in &mapped_types {
                if ty_item.id_matches(&item.id) {
                    tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Hash after Type edge.");
                    graph
                        .add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                    break;
                }
            }
            for (ty_item, &ty_index) in &mapped_enums {
                if ty_item.id_matches(&item.id) {
                    tracing::trace!(from = ?item.full_path, to = ty_item.full_path, "Adding Hash after Enum edge.");
                    graph
                        .add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                    break;
                }
            }
        }

        let mut this = Self {
            type_mappings: type_mappings.collect(),
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

    #[instrument(level = "info", err, skip(self))]
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

    pub fn to_sql(&self) -> eyre::Result<String> {
        let mut full_sql = String::new();
        for step_id in petgraph::algo::toposort(&self.graph, None)
            .map_err(|_e| eyre_err!("Depgraph was Cyclic."))?
        {
            let step = &self.graph[step_id];

            let sql = step.to_sql(self)?;

            full_sql.push_str(&sql)
        }
        Ok(full_sql)
    }

    #[instrument(level = "debug", skip(self))]
    pub fn register_types(&mut self) {
        for (item, _index) in self.enums.clone() {
            for (rust_id, mapping) in &item.mappings {
                assert_eq!(
                    self.type_mappings.insert(*rust_id, mapping.clone()),
                    None,
                    "Cannot map `{}` twice.",
                    item.full_path,
                );
            }
        }
        for (item, _index) in self.types.clone() {
            for (rust_id, mapping) in &item.mappings {
                assert_eq!(
                    self.type_mappings.insert(*rust_id, mapping.clone()),
                    None,
                    "Cannot map `{}` twice.",
                    item.full_path,
                );
            }
        }
    }

    #[instrument(level = "debug")]
    pub fn type_id_to_sql_type(&self, id: TypeId) -> Option<String> {
        self.type_mappings.get(&id).map(|f| f.sql.clone())
    }

    #[instrument(level = "debug")]
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

