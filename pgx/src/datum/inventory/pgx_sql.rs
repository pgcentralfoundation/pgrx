use eyre::eyre as eyre_err;
use std::{any::TypeId, collections::{HashMap, HashSet}, fmt::Debug};

use petgraph::{dot::Dot, graph::NodeIndex, stable_graph::StableGraph};
use tracing::instrument;

use super::{
    DotIdentifier, InventorySqlDeclaredEntity, ControlFile,
    InventoryExtensionSql, InventoryExtensionSqlPositioningRef,
    InventoryPgExtern, InventoryPgExternReturn, InventoryPostgresEnum,
    InventoryPostgresHash, InventoryPostgresOrd, InventorySchema,
    RustSourceOnlySqlMapping, RustSqlMapping, InventoryPostgresType,
    SqlGraphEntity, ToSql
};
use pgx_utils::inventory::SqlDeclaredEntity;

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
    pub schemas: HashMap<InventorySchema, NodeIndex>,
    pub extension_sqls: HashMap<InventoryExtensionSql, NodeIndex>,
    pub externs: HashMap<InventoryPgExtern, NodeIndex>,
    pub types: HashMap<InventoryPostgresType, NodeIndex>,
    pub builtin_types: HashMap<String, NodeIndex>,
    pub enums: HashMap<InventoryPostgresEnum, NodeIndex>,
    pub ords: HashMap<InventoryPostgresOrd, NodeIndex>,
    pub hashes: HashMap<InventoryPostgresHash, NodeIndex>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum SqlGraphRelationship {
    RequiredBy,
    RequiredByArg,
    RequiredByReturn,
}

impl PgxSql {
    #[instrument(
        level = "info",
        skip(
            type_mappings,
            source_mappings,
            entities,
        )
    )]
    pub fn build(
        type_mappings: impl Iterator<Item = RustSqlMapping>,
        source_mappings: impl Iterator<Item = RustSourceOnlySqlMapping>,
        entities: impl Iterator<Item = SqlGraphEntity>,
    ) -> eyre::Result<Self> {
        let mut graph = StableGraph::new();

        // Split up things into their specific types:
        let mut control: Option<ControlFile> = None;
        let mut schemas: HashSet<InventorySchema> = HashSet::default();
        let mut extension_sqls: HashSet<InventoryExtensionSql> = HashSet::default();
        let mut externs: HashSet<InventoryPgExtern> = HashSet::default();
        let mut types: HashSet<InventoryPostgresType> = HashSet::default();
        let mut enums: HashSet<InventoryPostgresEnum> = HashSet::default();
        let mut ords: HashSet<InventoryPostgresOrd> = HashSet::default();
        let mut hashes: HashSet<InventoryPostgresHash> = HashSet::default();
        for entity in entities {
            match entity {
                SqlGraphEntity::ExtensionRoot(input_control) => { control = Some(input_control); },
                SqlGraphEntity::Schema(input_schema) => { schemas.insert(input_schema); },
                SqlGraphEntity::CustomSql(input_sql) => { extension_sqls.insert(input_sql); },
                SqlGraphEntity::Function(input_function) => { externs.insert(input_function); },
                SqlGraphEntity::Type(input_type) => { types.insert(input_type); },
                SqlGraphEntity::BuiltinType(_) => (),
                SqlGraphEntity::Enum(input_enum) => { enums.insert(input_enum); },
                SqlGraphEntity::Ord(input_ord) => { ords.insert(input_ord); },
                SqlGraphEntity::Hash(input_hash) => { hashes.insert(input_hash); },
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
                    self.type_mappings.insert(mapping.id.clone(), mapping.clone()),
                    None,
                    "Cannot map `{}` twice.",
                    item.full_path,
                );
            }
        }
        for (item, _index) in self.types.clone() {
            for mapping in &item.mappings {
                assert_eq!(
                    self.type_mappings.insert(mapping.id.clone(), mapping.clone()),
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
    graph: &'a mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    root: NodeIndex,
    extension_sqls: HashSet<InventoryExtensionSql>,
) -> eyre::Result<(
    HashMap<InventoryExtensionSql, NodeIndex>,
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

fn connect_extension_sqls(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    extension_sqls: &HashMap<InventoryExtensionSql, NodeIndex>,
    schemas: &HashMap<InventorySchema, NodeIndex>,
    types: &HashMap<InventoryPostgresType, NodeIndex>,
    enums: &HashMap<InventoryPostgresEnum, NodeIndex>,
    externs: &HashMap<InventoryPgExtern, NodeIndex>,
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
                        if other.name == *name {
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
                        if other.name == *name {
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

fn initialize_schemas(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    schemas: HashSet<InventorySchema>,
) -> eyre::Result<HashMap<InventorySchema, NodeIndex>> {
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
    schemas: &HashMap<InventorySchema, NodeIndex>,
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
    enums: HashSet<InventoryPostgresEnum>,
) -> eyre::Result<HashMap<InventoryPostgresEnum, NodeIndex>> {
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
    enums: &HashMap<InventoryPostgresEnum, NodeIndex>,
    schemas: &HashMap<InventorySchema, NodeIndex>,
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
    types: HashSet<InventoryPostgresType>,
) -> eyre::Result<HashMap<InventoryPostgresType, NodeIndex>> {
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
    types: &HashMap<InventoryPostgresType, NodeIndex>,
    schemas: &HashMap<InventorySchema, NodeIndex>,
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

fn initialize_externs(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    externs: HashSet<InventoryPgExtern>,
    mapped_types: &HashMap<InventoryPostgresType, NodeIndex>,
    mapped_enums: &HashMap<InventoryPostgresEnum, NodeIndex>,
) -> eyre::Result<(
    HashMap<InventoryPgExtern, NodeIndex>,
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
                    .or_insert_with(|| graph.add_node(SqlGraphEntity::BuiltinType(arg.full_path.to_string())));
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
                    if ty_item.id_matches(id) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    mapped_builtin_types
                        .entry(full_path.to_string())
                        .or_insert_with(|| graph.add_node(SqlGraphEntity::BuiltinType(full_path.to_string())));
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
                        if ty_item.id_matches(&iterated_return.0) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        mapped_builtin_types
                            .entry(iterated_return.1.to_string())
                            .or_insert_with(|| {
                                graph.add_node(SqlGraphEntity::BuiltinType(iterated_return.1.to_string()))
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
    externs: &HashMap<InventoryPgExtern, NodeIndex>,
    schemas: &HashMap<InventorySchema, NodeIndex>,
    types: &HashMap<InventoryPostgresType, NodeIndex>,
    enums: &HashMap<InventoryPostgresEnum, NodeIndex>,
    builtin_types: &HashMap<String, NodeIndex>,
    extension_sqls: &HashMap<InventoryExtensionSql, NodeIndex>,
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
                        .get(&full_path.to_string())
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
                        let builtin_index = builtin_types.get(&iterated_return.1.to_string()).expect(&format!(
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

fn initialize_ords(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    ords: HashSet<InventoryPostgresOrd>,
) -> eyre::Result<HashMap<InventoryPostgresOrd, NodeIndex>> {
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
    ords: &HashMap<InventoryPostgresOrd, NodeIndex>,
    schemas: &HashMap<InventorySchema, NodeIndex>,
    types: &HashMap<InventoryPostgresType, NodeIndex>,
    enums: &HashMap<InventoryPostgresEnum, NodeIndex>,
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

fn initialize_hashes(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    hashes: HashSet<InventoryPostgresHash>,
) -> eyre::Result<HashMap<InventoryPostgresHash, NodeIndex>> {
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
    hashes: &HashMap<InventoryPostgresHash, NodeIndex>,
    schemas: &HashMap<InventorySchema, NodeIndex>,
    types: &HashMap<InventoryPostgresType, NodeIndex>,
    enums: &HashMap<InventoryPostgresEnum, NodeIndex>,
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
