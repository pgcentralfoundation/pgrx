use eyre::eyre as eyre_err;
use std::{any::TypeId, collections::HashMap, fmt::Debug};

use petgraph::{dot::Dot, graph::NodeIndex, stable_graph::StableGraph};
use tracing::instrument;

use super::{
    ControlFile, ExtensionSqlEntity, PgExternEntity, PgExternReturnEntity, PositioningRef,
    PostgresEnumEntity, PostgresHashEntity, PostgresOrdEntity, PostgresTypeEntity,
    RustSourceOnlySqlMapping, RustSqlMapping, SchemaEntity, SqlDeclaredEntity, SqlGraphEntity,
    SqlGraphIdentifier, ToSql,
};
use pgx_utils::sql_entity_graph::SqlDeclared;

/// A generator for SQL.
///
/// Consumes a base mapping of types (typically `pgx::DEFAULT_TYPEID_SQL_MAPPING`), a
/// [`ControlFile`], and collections of each SQL entity.
///
/// During construction, a Directed Acyclic Graph is formed out the dependencies. For example,
/// an item `detect_dog(x: &[u8]) -> animals::Dog` would have have a relationship with
/// `animals::Dog`.
///
/// Typically, [`PgxSql`] types are constructed in a `pgx::pg_binary_magic!()` call in a binary
/// out of entities collected during a `pgx::pg_module_magic!()` call in a library.
#[derive(Debug, Clone)]
pub struct PgxSql {
    // This is actually the Debug format of a TypeId!
    //
    // This is not a good idea, but without a stable way to create or serialize TypeIds, we have to.
    pub type_mappings: HashMap<TypeId, RustSqlMapping>,
    pub source_mappings: HashMap<String, RustSourceOnlySqlMapping>,
    pub control: ControlFile,
    pub graph: StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    pub graph_root: NodeIndex,
    pub graph_bootstrap: Option<NodeIndex>,
    pub graph_finalize: Option<NodeIndex>,
    pub schemas: HashMap<SchemaEntity, NodeIndex>,
    pub extension_sqls: HashMap<ExtensionSqlEntity, NodeIndex>,
    pub externs: HashMap<PgExternEntity, NodeIndex>,
    pub types: HashMap<PostgresTypeEntity, NodeIndex>,
    pub builtin_types: HashMap<String, NodeIndex>,
    pub enums: HashMap<PostgresEnumEntity, NodeIndex>,
    pub ords: HashMap<PostgresOrdEntity, NodeIndex>,
    pub hashes: HashMap<PostgresHashEntity, NodeIndex>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum SqlGraphRelationship {
    RequiredBy,
    RequiredByArg,
    RequiredByReturn,
}

impl PgxSql {
    #[instrument(level = "error", skip(type_mappings, source_mappings, entities,))]
    pub fn build(
        type_mappings: impl Iterator<Item = RustSqlMapping>,
        source_mappings: impl Iterator<Item = RustSourceOnlySqlMapping>,
        entities: impl Iterator<Item = SqlGraphEntity>,
    ) -> eyre::Result<Self> {
        let mut graph = StableGraph::new();

        let mut entities = entities.collect::<Vec<_>>();
        entities.sort();
        // Split up things into their specific types:
        let mut control: Option<ControlFile> = None;
        let mut schemas: Vec<SchemaEntity> = Vec::default();
        let mut extension_sqls: Vec<ExtensionSqlEntity> = Vec::default();
        let mut externs: Vec<PgExternEntity> = Vec::default();
        let mut types: Vec<PostgresTypeEntity> = Vec::default();
        let mut enums: Vec<PostgresEnumEntity> = Vec::default();
        let mut ords: Vec<PostgresOrdEntity> = Vec::default();
        let mut hashes: Vec<PostgresHashEntity> = Vec::default();
        for entity in entities {
            match entity {
                SqlGraphEntity::ExtensionRoot(input_control) => {
                    control = Some(input_control);
                }
                SqlGraphEntity::Schema(input_schema) => {
                    schemas.push(input_schema);
                }
                SqlGraphEntity::CustomSql(input_sql) => {
                    extension_sqls.push(input_sql);
                }
                SqlGraphEntity::Function(input_function) => {
                    externs.push(input_function);
                }
                SqlGraphEntity::Type(input_type) => {
                    types.push(input_type);
                }
                SqlGraphEntity::BuiltinType(_) => (),
                SqlGraphEntity::Enum(input_enum) => {
                    enums.push(input_enum);
                }
                SqlGraphEntity::Ord(input_ord) => {
                    ords.push(input_ord);
                }
                SqlGraphEntity::Hash(input_hash) => {
                    hashes.push(input_hash);
                }
            }
        }

        let control: ControlFile = control.expect("No control file found");
        let root = graph.add_node(SqlGraphEntity::ExtensionRoot(control.clone()));

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
        )?;
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
        )?;
        connect_ords(
            &mut graph,
            &mapped_ords,
            &mapped_schemas,
            &mapped_types,
            &mapped_enums,
            &mapped_externs,
        );
        connect_hashes(
            &mut graph,
            &mapped_hashes,
            &mapped_schemas,
            &mapped_types,
            &mapped_enums,
            &mapped_externs,
        );

        let mut this = Self {
            type_mappings: type_mappings.map(|x| (x.id.clone(), x)).collect(),
            source_mappings: source_mappings.map(|x| (x.rust.clone(), x)).collect(),
            control: control,
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

    #[instrument(level = "error", skip(self))]
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

    #[instrument(level = "error", err, skip(self))]
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

    #[instrument(level = "error", skip(self))]
    pub fn to_sql(&self) -> eyre::Result<String> {
        let mut full_sql = String::new();
        for step_id in petgraph::algo::toposort(&self.graph, None).map_err(|e| {
            eyre_err!(
                "Failed to toposort SQL entities, node with cycle: {:?}",
                self.graph[e.node_id()]
            )
        })? {
            let step = &self.graph[step_id];

            let sql = step.to_sql(self)?;

            if !sql.is_empty() {
                full_sql.push_str(&sql);
                full_sql.push('\n');
            }
        }
        Ok(full_sql)
    }

    #[instrument(level = "error", skip(self))]
    pub fn register_types(&mut self) {
        for (item, _index) in self.enums.clone() {
            for mapping in &item.mappings {
                assert_eq!(
                    self.type_mappings
                        .insert(mapping.id.clone(), mapping.clone()),
                    None,
                    "Cannot map `{}` twice.",
                    item.full_path,
                );
            }
        }
        for (item, _index) in self.types.clone() {
            for mapping in &item.mappings {
                assert_eq!(
                    self.type_mappings
                        .insert(mapping.id.clone(), mapping.clone()),
                    None,
                    "Cannot map `{}` twice.",
                    item.full_path,
                );
            }
        }
    }

    pub fn has_sql_declared_entity(&self, identifier: &SqlDeclared) -> Option<&SqlDeclaredEntity> {
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

#[instrument(level = "error", skip(graph, root, extension_sqls))]
fn initialize_extension_sqls<'a>(
    graph: &'a mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    root: NodeIndex,
    extension_sqls: Vec<ExtensionSqlEntity>,
) -> eyre::Result<(
    HashMap<ExtensionSqlEntity, NodeIndex>,
    Option<NodeIndex>,
    Option<NodeIndex>,
)> {
    let mut bootstrap = None;
    let mut finalize = None;
    let mut mapped_extension_sqls = HashMap::default();
    for item in extension_sqls {
        let entity: SqlGraphEntity = item.clone().into();
        let index = graph.add_node(entity);
        mapped_extension_sqls.insert(item.clone(), index);

        if item.bootstrap {
            if let Some(exiting_index) = bootstrap {
                let existing: &SqlGraphEntity = &graph[exiting_index];
                return Err(eyre_err!(
                    "Cannot have multiple `extension_sql!()` with `bootstrap` positioning, found `{}`, other was `{}`",
                    item.rust_identifier(),
                    existing.rust_identifier(),
                ));
            }
            bootstrap = Some(index)
        }
        if item.finalize {
            if let Some(exiting_index) = finalize {
                let existing: &SqlGraphEntity = &graph[exiting_index];
                return Err(eyre_err!(
                    "Cannot have multiple `extension_sql!()` with `finalize` positioning, found `{}`, other was `{}`",
                    item.rust_identifier(),
                    existing.rust_identifier(),
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

/// A best effort attempt to find the related [`NodeIndex`] for some [`PositioningRef`].
pub fn find_positioning_ref_target<'a>(
    positioning_ref: &'a PositioningRef,
    types: &'a HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &'a HashMap<PostgresEnumEntity, NodeIndex>,
    externs: &'a HashMap<PgExternEntity, NodeIndex>,
    schemas: &'a HashMap<SchemaEntity, NodeIndex>,
    extension_sqls: &'a HashMap<ExtensionSqlEntity, NodeIndex>,
) -> Option<&'a NodeIndex> {
    match positioning_ref {
        PositioningRef::FullPath(path) => {
            // The best we can do here is a fuzzy search.
            let segments = path.split("::").collect::<Vec<_>>();
            let last_segment = segments.last().expect("Expected at least one segment.");
            let rest = &segments[..segments.len() - 1];
            let module_path = rest.join("::");

            for (other, other_index) in types {
                if *last_segment == other.name && other.module_path.ends_with(&module_path) {
                    return Some(&other_index);
                }
            }
            for (other, other_index) in enums {
                if last_segment == &other.name && other.module_path.ends_with(&module_path) {
                    return Some(&other_index);
                }
            }
            for (other, other_index) in externs {
                if *last_segment == other.unaliased_name
                    && other.module_path.ends_with(&module_path)
                {
                    return Some(&other_index);
                }
            }
            for (other, other_index) in schemas {
                if other.module_path.ends_with(path) {
                    return Some(&other_index);
                }
            }
        }
        PositioningRef::Name(name) => {
            for (other, other_index) in extension_sqls {
                if other.name == *name {
                    return Some(&other_index);
                }
            }
        }
    };
    None
}

fn connect_extension_sqls(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
) -> eyre::Result<()> {
    for (item, &index) in extension_sqls {
        for (schema_item, &schema_index) in schemas {
            if item.module_path == schema_item.module_path {
                tracing::debug!(from = %item.rust_identifier(), to = schema_item.module_path, "Adding ExtensionSQL after Schema edge");
                graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for requires in &item.requires {
            if let Some(target) = find_positioning_ref_target(
                requires,
                types,
                enums,
                externs,
                schemas,
                extension_sqls,
            ) {
                tracing::debug!(from = %item.rust_identifier(), to = ?graph[*target].rust_identifier(), "Adding ExtensionSQL after positioning ref target");
                graph.add_edge(*target, index, SqlGraphRelationship::RequiredBy);
            } else {
                return Err(eyre_err!(
                    "Could not find `requires` target of `{}`{}: {}",
                    item.rust_identifier(),
                    if let (Some(file), Some(line)) = (item.file(), item.line()) {
                        format!(" ({}:{})", file, line)
                    } else {
                        "".to_string()
                    },
                    match requires {
                        PositioningRef::FullPath(path) => path.to_string(),
                        PositioningRef::Name(name) => format!(r#""{}""#, name),
                    },
                ));
            }
        }
    }
    Ok(())
}

fn initialize_schemas(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    schemas: Vec<SchemaEntity>,
) -> eyre::Result<HashMap<SchemaEntity, NodeIndex>> {
    let mut mapped_schemas = HashMap::default();
    for item in schemas {
        let entity = item.clone().into();
        let index = graph.add_node(entity);
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

fn connect_schemas(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    root: NodeIndex,
) {
    for (_item, &index) in schemas {
        graph.add_edge(root, index, SqlGraphRelationship::RequiredBy);
    }
}

fn initialize_enums(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    enums: Vec<PostgresEnumEntity>,
) -> eyre::Result<HashMap<PostgresEnumEntity, NodeIndex>> {
    let mut mapped_enums = HashMap::default();
    for item in enums {
        let entity: SqlGraphEntity = item.clone().into();
        let index = graph.add_node(entity);
        mapped_enums.insert(item, index);
        build_base_edges(graph, index, root, bootstrap, finalize);
    }
    Ok(mapped_enums)
}

fn connect_enums(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
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

fn initialize_types(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    types: Vec<PostgresTypeEntity>,
) -> eyre::Result<HashMap<PostgresTypeEntity, NodeIndex>> {
    let mut mapped_types = HashMap::default();
    for item in types {
        let entity = item.clone().into();
        let index = graph.add_node(entity);
        mapped_types.insert(item, index);
        build_base_edges(graph, index, root, bootstrap, finalize);
    }
    Ok(mapped_types)
}

fn connect_types(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
) {
    for (item, &index) in types {
        for (schema_item, &schema_index) in schemas {
            if item.module_path == schema_item.module_path {
                tracing::debug!(from = ?item.full_path, to = schema_item.module_path, "Adding Type after Schema edge.");
                graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
    }
}

fn initialize_externs(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    externs: Vec<PgExternEntity>,
    mapped_types: &HashMap<PostgresTypeEntity, NodeIndex>,
    mapped_enums: &HashMap<PostgresEnumEntity, NodeIndex>,
) -> eyre::Result<(
    HashMap<PgExternEntity, NodeIndex>,
    HashMap<String, NodeIndex>,
)> {
    let mut mapped_externs = HashMap::default();
    let mut mapped_builtin_types = HashMap::default();
    for item in externs {
        let entity: SqlGraphEntity = item.clone().into();
        let index = graph.add_node(entity.clone());
        mapped_externs.insert(item.clone(), index);
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
                if ty_item.id_matches(&arg.ty_id) {
                    found = true;
                    break;
                }
            }
            if !found {
                mapped_builtin_types
                    .entry(arg.full_path.to_string())
                    .or_insert_with(|| {
                        graph.add_node(SqlGraphEntity::BuiltinType(arg.full_path.to_string()))
                    });
            }
        }

        match &item.fn_return {
            PgExternReturnEntity::None | PgExternReturnEntity::Trigger => (),
            PgExternReturnEntity::Type { id, full_path, .. }
            | PgExternReturnEntity::SetOf { id, full_path, .. } => {
                let mut found = false;
                for (ty_item, &_ty_index) in mapped_types {
                    if ty_item.id_matches(id) {
                        found = true;
                        break;
                    }
                }
                for (ty_item, &_ty_index) in mapped_enums {
                    if ty_item.id_matches(id) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    mapped_builtin_types
                        .entry(full_path.to_string())
                        .or_insert_with(|| {
                            graph.add_node(SqlGraphEntity::BuiltinType(full_path.to_string()))
                        });
                }
            }
            PgExternReturnEntity::Iterated(iterated_returns) => {
                for iterated_return in iterated_returns {
                    let mut found = false;
                    for (ty_item, &_ty_index) in mapped_types {
                        if ty_item.id_matches(&iterated_return.0) {
                            found = true;
                            break;
                        }
                    }
                    for (ty_item, &_ty_index) in mapped_enums {
                        if ty_item.id_matches(&iterated_return.0) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        mapped_builtin_types
                            .entry(iterated_return.1.to_string())
                            .or_insert_with(|| {
                                graph.add_node(SqlGraphEntity::BuiltinType(
                                    iterated_return.1.to_string(),
                                ))
                            });
                    }
                }
            }
        }
    }
    Ok((mapped_externs, mapped_builtin_types))
}

fn connect_externs(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    builtin_types: &HashMap<String, NodeIndex>,
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
) -> eyre::Result<()> {
    for (item, &index) in externs {
        for (schema_item, &schema_index) in schemas {
            if item.module_path == schema_item.module_path {
                tracing::debug!(from = %item.rust_identifier(), to = %schema_item.rust_identifier(), "Adding Extern after Schema edge");
                graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }

        for extern_attr in &item.extern_attrs {
            match extern_attr {
                pgx_utils::ExternArgs::Requires(requirements) => {
                    for requires in requirements {
                        if let Some(target) = find_positioning_ref_target(
                            requires,
                            types,
                            enums,
                            externs,
                            schemas,
                            extension_sqls,
                        ) {
                            tracing::debug!(from = %item.rust_identifier(), to = %graph[*target].rust_identifier(), "Adding Extern after positioning ref target");
                            graph.add_edge(*target, index, SqlGraphRelationship::RequiredBy);
                        } else {
                            return Err(eyre_err!(
                                "Could not find `requires` target: {:?}",
                                requires
                            ));
                        }
                    }
                }
                _ => (),
            }
        }

        for arg in &item.fn_args {
            let mut found = false;
            for (ty_item, &ty_index) in types {
                if ty_item.id_matches(&arg.ty_id) {
                    tracing::debug!(from = %item.rust_identifier(), to = %ty_item.rust_identifier(), "Adding Extern after Type (due to argument) edge");
                    graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredByArg);
                    found = true;
                    break;
                }
            }
            if !found {
                for (enum_item, &enum_index) in enums {
                    if enum_item.id_matches(&arg.ty_id) {
                        tracing::debug!(from = %item.rust_identifier(), to = %enum_item.rust_identifier(), "Adding Extern after Enum (due to argument) edge");
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
                tracing::debug!(from = %item.rust_identifier(), to = %arg.rust_identifier(), "Adding Extern(arg) after BuiltIn Type (due to argument) edge");
                graph.add_edge(*builtin_index, index, SqlGraphRelationship::RequiredByArg);
            }
            if !found {
                for (ext_item, ext_index) in extension_sqls {
                    if let Some(_) = ext_item
                        .has_sql_declared_entity(&SqlDeclared::Type(arg.full_path.to_string()))
                    {
                        tracing::debug!(from = %item.rust_identifier(), to = %arg.rust_identifier(), "Adding Extern(arg) after Extension SQL (due to argument) edge");
                        graph.add_edge(*ext_index, index, SqlGraphRelationship::RequiredByArg);
                    } else if let Some(_) = ext_item
                        .has_sql_declared_entity(&SqlDeclared::Enum(arg.full_path.to_string()))
                    {
                        tracing::debug!(from = %item.rust_identifier(), to = %arg.rust_identifier(), "Adding Extern(arg) after Extension SQL (due to argument) edge");
                        graph.add_edge(*ext_index, index, SqlGraphRelationship::RequiredByArg);
                    }
                }
            }
        }
        match &item.fn_return {
            PgExternReturnEntity::None | PgExternReturnEntity::Trigger => (),
            PgExternReturnEntity::Type { id, full_path, .. }
            | PgExternReturnEntity::SetOf { id, full_path, .. } => {
                let mut found = false;
                for (ty_item, &ty_index) in types {
                    if ty_item.id_matches(id) {
                        tracing::debug!(from = %item.rust_identifier(), to = %ty_item.rust_identifier(), "Adding Extern after Type (due to return) edge");
                        graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredByReturn);
                        found = true;
                        break;
                    }
                }
                if !found {
                    for (ty_item, &ty_index) in enums {
                        if ty_item.id_matches(id) {
                            tracing::debug!(from = %item.rust_identifier(), to = %ty_item.rust_identifier(), "Adding Extern after Enum (due to return) edge");
                            graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredByReturn);
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    let builtin_index = builtin_types
                        .get(&full_path.to_string())
                        .expect(&format!("Could not fetch Builtin Type {}.", full_path));
                    tracing::debug!(from = ?item.full_path, to = %full_path, "Adding Extern(return) after BuiltIn Type (due to return) edge");
                    graph.add_edge(
                        *builtin_index,
                        index,
                        SqlGraphRelationship::RequiredByReturn,
                    );
                }
                if !found {
                    for (ext_item, ext_index) in extension_sqls {
                        if let Some(_) = ext_item
                            .has_sql_declared_entity(&SqlDeclared::Type(full_path.to_string()))
                        {
                            tracing::debug!(from = %item.rust_identifier(), to = full_path, "Adding Extern(arg) after Extension SQL (due to argument) edge");
                            graph.add_edge(*ext_index, index, SqlGraphRelationship::RequiredByArg);
                        } else if let Some(_) = ext_item
                            .has_sql_declared_entity(&SqlDeclared::Enum(full_path.to_string()))
                        {
                            tracing::debug!(from = %item.rust_identifier(), to = full_path, "Adding Extern(arg) after Extension SQL (due to argument) edge");
                            graph.add_edge(*ext_index, index, SqlGraphRelationship::RequiredByArg);
                        }
                    }
                }
            }
            PgExternReturnEntity::Iterated(iterated_returns) => {
                for iterated_return in iterated_returns {
                    let mut found = false;
                    for (ty_item, &ty_index) in types {
                        if ty_item.id_matches(&iterated_return.0) {
                            tracing::debug!(from = %item.rust_identifier(), to = %ty_item.rust_identifier(), "Adding Extern after Type (due to return) edge");
                            graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredByReturn);
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        for (ty_item, &ty_index) in enums {
                            if ty_item.id_matches(&iterated_return.0) {
                                tracing::debug!(from = %item.rust_identifier(), to = %ty_item.rust_identifier(), "Adding Extern after Enum (due to return) edge.");
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
                        let builtin_index = builtin_types
                            .get(&iterated_return.1.to_string())
                            .expect(&format!(
                                "Could not fetch Builtin Type {}.",
                                iterated_return.1
                            ));
                        tracing::debug!(from = %item.rust_identifier(), to = iterated_return.1, "Adding Extern after BuiltIn Type (due to return) edge");
                        graph.add_edge(
                            *builtin_index,
                            index,
                            SqlGraphRelationship::RequiredByReturn,
                        );
                    }
                    if !found {
                        for (ext_item, ext_index) in extension_sqls {
                            if let Some(_) = ext_item.has_sql_declared_entity(&SqlDeclared::Type(
                                iterated_return.1.to_string(),
                            )) {
                                tracing::debug!(from = %item.rust_identifier(), to = iterated_return.1, "Adding Extern(arg) after Extension SQL (due to argument) edge");
                                graph.add_edge(
                                    *ext_index,
                                    index,
                                    SqlGraphRelationship::RequiredByArg,
                                );
                            } else if let Some(_) = ext_item.has_sql_declared_entity(
                                &SqlDeclared::Enum(iterated_return.1.to_string()),
                            ) {
                                tracing::debug!(from = %item.rust_identifier(), to = iterated_return.1, "Adding Extern(arg) after Extension SQL (due to argument) edge");
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
    Ok(())
}

fn initialize_ords(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    ords: Vec<PostgresOrdEntity>,
) -> eyre::Result<HashMap<PostgresOrdEntity, NodeIndex>> {
    let mut mapped_ords = HashMap::default();
    for item in ords {
        let entity = item.clone().into();
        let index = graph.add_node(entity);
        mapped_ords.insert(item.clone(), index);
        build_base_edges(graph, index, root, bootstrap, finalize);
    }
    Ok(mapped_ords)
}

fn connect_ords(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    ords: &HashMap<PostgresOrdEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
) {
    for (item, &index) in ords {
        for (schema_item, &schema_index) in schemas {
            if item.module_path == schema_item.module_path {
                tracing::debug!(from = ?item.full_path, to = schema_item.module_path, "Adding Ord after Schema edge.");
                graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for (ty_item, &ty_index) in types {
            if ty_item.id_matches(&item.id) {
                tracing::debug!(from = ?item.full_path, to = ty_item.full_path, "Adding Ord after Type edge.");
                graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for (ty_item, &ty_index) in enums {
            if ty_item.id_matches(&item.id) {
                tracing::debug!(from = ?item.full_path, to = ty_item.full_path, "Adding Ord after Enum edge.");
                graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for (ty_item, &ty_index) in externs {
            if ty_item.operator.is_some() {
                tracing::debug!(from = ?item.full_path, to = ty_item.full_path, "Adding Hash after Operator edge.");
                graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                // NB:  no break here.  We need to be dependent on all externs that are operators
            }
        }
    }
}

fn initialize_hashes(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    hashes: Vec<PostgresHashEntity>,
) -> eyre::Result<HashMap<PostgresHashEntity, NodeIndex>> {
    let mut mapped_hashes = HashMap::default();
    for item in hashes {
        let entity: SqlGraphEntity = item.clone().into();
        let index = graph.add_node(entity);
        mapped_hashes.insert(item, index);
        build_base_edges(graph, index, root, bootstrap, finalize);
    }
    Ok(mapped_hashes)
}

fn connect_hashes(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    hashes: &HashMap<PostgresHashEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
) {
    for (item, &index) in hashes {
        for (schema_item, &schema_index) in schemas {
            if item.module_path == schema_item.module_path {
                tracing::debug!(from = ?item.full_path, to = schema_item.module_path, "Adding Hash after Schema edge.");
                graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for (ty_item, &ty_index) in types {
            if ty_item.id_matches(&item.id) {
                tracing::debug!(from = ?item.full_path, to = ty_item.full_path, "Adding Hash after Type edge.");
                graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for (ty_item, &ty_index) in enums {
            if ty_item.id_matches(&item.id) {
                tracing::debug!(from = ?item.full_path, to = ty_item.full_path, "Adding Hash after Enum edge.");
                graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
        for (ty_item, &ty_index) in externs {
            if ty_item.operator.is_some() {
                tracing::debug!(from = ?item.full_path, to = ty_item.full_path, "Adding Hash after Operator edge.");
                graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
                // NB:  no break here.  We need to be dependent on all externs that are operators
            }
        }
    }
}
