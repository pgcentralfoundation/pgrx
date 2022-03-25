use eyre::{eyre, WrapErr};
use std::{any::TypeId, collections::HashMap, fmt::Debug, path::Path};

use petgraph::{dot::Dot, graph::NodeIndex, stable_graph::StableGraph};
use tracing::instrument;
use owo_colors::{OwoColorize, XtermColors};

use crate::sql_entity_graph::{
    aggregate::entity::PgAggregateEntity,
    control_file::ControlFile,
    extension_sql::{
        entity::{ExtensionSqlEntity, SqlDeclaredEntity},
        SqlDeclared,
    },
    mapping::{RustSourceOnlySqlMapping, RustSqlMapping},
    pg_extern::entity::{PgExternEntity, PgExternReturnEntity},
    positioning_ref::PositioningRef,
    postgres_enum::entity::PostgresEnumEntity,
    postgres_hash::entity::PostgresHashEntity,
    postgres_ord::entity::PostgresOrdEntity,
    postgres_type::entity::PostgresTypeEntity,
    schema::entity::SchemaEntity,
    to_sql::ToSql,
    SqlGraphEntity, SqlGraphIdentifier,
};

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
    pub aggregates: HashMap<PgAggregateEntity, NodeIndex>,
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
        let mut aggregates: Vec<PgAggregateEntity> = Vec::default();
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
                SqlGraphEntity::Aggregate(input_hash) => {
                    aggregates.push(input_hash);
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
        let (mapped_externs, mut mapped_builtin_types) = initialize_externs(
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
        let mapped_aggregates = initialize_aggregates(
            &mut graph,
            root,
            bootstrap,
            finalize,
            aggregates,
            &mut mapped_builtin_types,
            &mapped_enums,
            &mapped_types,
        )?;

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
        connect_aggregates(
            &mut graph,
            &mapped_aggregates,
            &mapped_schemas,
            &mapped_types,
            &mapped_enums,
            &mapped_builtin_types,
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
            aggregates: mapped_aggregates,
            graph: graph,
            graph_root: root,
            graph_bootstrap: bootstrap,
            graph_finalize: finalize,
        };
        this.register_types();
        Ok(this)
    }

    #[instrument(level = "error", skip(self))]
    pub fn to_file(&self, file: impl AsRef<Path> + Debug) -> eyre::Result<()> {
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

    #[instrument(level = "error", skip_all)]
    pub fn write(&self, out: &mut impl std::io::Write) -> eyre::Result<()> {
        let generated = self.to_sql()?;

        if atty::is(atty::Stream::Stdout) {
            use syntect::{
                easy::HighlightLines,
                highlighting::{Style, ThemeSet},
                parsing::SyntaxSet,
                util::LinesWithEndings,
            };
            let ps = SyntaxSet::load_defaults_newlines();
            let theme_bytes = include_str!("../../assets/ansi.tmTheme").as_bytes();
            let mut theme_reader = std::io::Cursor::new(theme_bytes);
            let theme = ThemeSet::load_from_reader(&mut theme_reader)
                .wrap_err("Couldn't parse theme for SQL highlighting, try piping to a file")?;

            if let Some(syntax) = ps.find_syntax_by_extension("sql") {
                let mut h = HighlightLines::new(syntax, &theme);
                for line in LinesWithEndings::from(&generated) {
                    let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
                    // Concept from https://github.com/sharkdp/bat/blob/1b030dc03b906aa345f44b8266bffeea77d763fe/src/terminal.rs#L6
                    for (style, content) in ranges {
                        if style.foreground.a == 0x01 {
                            write!(*out, "{}", content)?;
                        } else {
                            write!(*out, "{}", content.color(XtermColors::from(style.foreground.r)))?;
                        }
                    }
                    write!(*out, "\x1b[0m")?;
                }
            } else {
                write!(*out, "{}", generated)?;
            }
        } else {
            write!(*out, "{}", generated)?;
        }

        Ok(())
    }

    #[instrument(level = "error", err, skip(self))]
    pub fn to_dot(&self, file: impl AsRef<Path> + Debug) -> eyre::Result<()> {
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
                    SqlGraphEntity::Aggregate(_item) => format!(
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
            eyre!(
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

    pub fn rust_to_sql(&self, ty_id: TypeId, ty_source: &str, full_path: &str) -> Option<String> {
        self.source_only_to_sql_type(ty_source)
            .or_else(|| self.type_id_to_sql_type(ty_id))
            .or_else(|| {
                if let Some(found) =
                    self.has_sql_declared_entity(&SqlDeclared::Type(full_path.to_string()))
                {
                    Some(found.sql())
                } else if let Some(found) =
                    self.has_sql_declared_entity(&SqlDeclared::Enum(full_path.to_string()))
                {
                    Some(found.sql())
                } else {
                    None
                }
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

#[tracing::instrument(level = "error", skip_all)]
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

#[tracing::instrument(level = "error", skip_all)]
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
                return Err(eyre!(
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
                return Err(eyre!(
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

#[tracing::instrument(level = "error", skip_all)]
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

#[tracing::instrument(level = "error", skip_all)]
fn connect_extension_sqls(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
) -> eyre::Result<()> {
    for (item, &index) in extension_sqls {
        make_schema_connection(
            graph,
            "Extension SQL",
            index,
            &item.rust_identifier(),
            item.module_path,
            schemas,
        );

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
                return Err(eyre!(
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

#[tracing::instrument(level = "error", skip_all)]
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

#[tracing::instrument(level = "error", skip_all)]
fn connect_schemas(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    root: NodeIndex,
) {
    for (_item, &index) in schemas {
        graph.add_edge(root, index, SqlGraphRelationship::RequiredBy);
    }
}

#[tracing::instrument(level = "error", skip_all)]
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

#[tracing::instrument(level = "error", skip_all)]
fn connect_enums(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
) {
    for (item, &index) in enums {
        make_schema_connection(
            graph,
            "Enum",
            index,
            &item.rust_identifier(),
            item.module_path,
            schemas,
        );
    }
}

#[tracing::instrument(level = "error", skip_all)]
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

#[tracing::instrument(level = "error", skip_all)]
fn connect_types(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
) {
    for (item, &index) in types {
        make_schema_connection(
            graph,
            "Type",
            index,
            &item.rust_identifier(),
            item.module_path,
            schemas,
        );
    }
}

#[tracing::instrument(level = "error", skip_all)]
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

#[tracing::instrument(level = "error", skip_all)]
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
        make_schema_connection(
            graph,
            "Extern",
            index,
            &item.rust_identifier(),
            item.module_path,
            schemas,
        );

        for extern_attr in &item.extern_attrs {
            match extern_attr {
                crate::ExternArgs::Requires(requirements) => {
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
                            return Err(eyre!("Could not find `requires` target: {:?}", requires));
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
                                tracing::debug!(from = %item.rust_identifier(), to = %ty_item.rust_identifier(), "Adding Extern after Enum (due to return) edge");
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

#[tracing::instrument(level = "error", skip_all)]
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

#[tracing::instrument(level = "error", skip_all)]
fn connect_ords(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    ords: &HashMap<PostgresOrdEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
) {
    for (item, &index) in ords {
        make_schema_connection(
            graph,
            "Ord",
            index,
            &item.rust_identifier(),
            item.module_path,
            schemas,
        );

        make_type_or_enum_connection(
            graph,
            "Ord",
            index,
            &item.rust_identifier(),
            &item.id,
            types,
            enums,
        );

        for (extern_item, &extern_index) in externs {
            let fn_matches = |fn_name| {
                item.module_path == extern_item.module_path && extern_item.name == fn_name
            };
            let cmp_fn_matches = fn_matches(item.cmp_fn_name());
            let lt_fn_matches = fn_matches(item.lt_fn_name());
            let lte_fn_matches = fn_matches(item.le_fn_name());
            let eq_fn_matches = fn_matches(item.eq_fn_name());
            let gt_fn_matches = fn_matches(item.gt_fn_name());
            let gte_fn_matches = fn_matches(item.ge_fn_name());
            if cmp_fn_matches
                || lt_fn_matches
                || lte_fn_matches
                || eq_fn_matches
                || gt_fn_matches
                || gte_fn_matches
            {
                tracing::debug!(from = ?item.full_path, to = extern_item.full_path, "Adding Ord after Extern edge");
                graph.add_edge(extern_index, index, SqlGraphRelationship::RequiredBy);
            }
        }
    }
}

#[tracing::instrument(level = "error", skip_all)]
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

#[tracing::instrument(level = "error", skip_all)]
fn connect_hashes(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    hashes: &HashMap<PostgresHashEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
) {
    for (item, &index) in hashes {
        make_schema_connection(
            graph,
            "Hash",
            index,
            &item.rust_identifier(),
            item.module_path,
            schemas,
        );

        make_type_or_enum_connection(
            graph,
            "Hash",
            index,
            &item.rust_identifier(),
            &item.id,
            types,
            enums,
        );

        for (extern_item, &extern_index) in externs {
            let hash_fn_name = item.fn_name();
            let hash_fn_matches =
                item.module_path == extern_item.module_path && extern_item.name == hash_fn_name;

            if hash_fn_matches {
                tracing::debug!(from = ?item.full_path, to = extern_item.full_path, "Adding Hash after Extern edge");
                graph.add_edge(extern_index, index, SqlGraphRelationship::RequiredBy);
                break;
            }
        }
    }
}

fn initialize_aggregates(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    aggregates: Vec<PgAggregateEntity>,
    mapped_builtin_types: &mut HashMap<String, NodeIndex>,
    mapped_enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    mapped_types: &HashMap<PostgresTypeEntity, NodeIndex>,
) -> eyre::Result<HashMap<PgAggregateEntity, NodeIndex>> {
    let mut mapped_aggregates = HashMap::default();
    for item in aggregates {
        let entity: SqlGraphEntity = item.clone().into();
        let index = graph.add_node(entity);

        for arg in &item.args {
            let mut found = false;
            for (ty_item, &_ty_index) in mapped_types {
                if ty_item.id_matches(&arg.agg_ty.ty_id) {
                    found = true;
                    break;
                }
            }
            for (ty_item, &_ty_index) in mapped_enums {
                if ty_item.id_matches(&arg.agg_ty.ty_id) {
                    found = true;
                    break;
                }
            }
            if !found {
                mapped_builtin_types
                    .entry(arg.agg_ty.full_path.to_string())
                    .or_insert_with(|| {
                        graph.add_node(SqlGraphEntity::BuiltinType(
                            arg.agg_ty.full_path.to_string(),
                        ))
                    });
            }
        }

        mapped_aggregates.insert(item, index);
        build_base_edges(graph, index, root, bootstrap, finalize);
    }
    Ok(mapped_aggregates)
}

fn connect_aggregates(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    aggregates: &HashMap<PgAggregateEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    builtin_types: &HashMap<String, NodeIndex>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
) {
    for (item, &index) in aggregates {
        make_schema_connection(
            graph,
            "Aggregate",
            index,
            &item.rust_identifier(),
            item.module_path,
            schemas,
        );

        make_type_or_enum_connection(
            graph,
            "Aggregate",
            index,
            &item.rust_identifier(),
            &item.ty_id,
            types,
            enums,
        );

        for arg in &item.args {
            let found = make_type_or_enum_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &arg.agg_ty.ty_id,
                types,
                enums,
            );
            if !found {
                let builtin_index = builtin_types.get(arg.agg_ty.full_path).expect(&format!(
                    "Could not fetch Builtin Type {}.",
                    arg.agg_ty.full_path
                ));
                tracing::debug!(from = %item.rust_identifier(), to = %arg.agg_ty.full_path, "Adding Aggregate after BuiltIn Type edge");
                graph.add_edge(*builtin_index, index, SqlGraphRelationship::RequiredByArg);
            }
        }

        for arg in item.direct_args.as_ref().unwrap_or(&vec![]) {
            let found = make_type_or_enum_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &arg.ty_id,
                types,
                enums,
            );
            if !found {
                let builtin_index = builtin_types
                    .get(arg.full_path)
                    .expect(&format!("Could not fetch Builtin Type {}.", arg.full_path));
                tracing::debug!(from = %item.rust_identifier(), to = %arg.full_path, "Adding Aggregate after BuiltIn Type edge");
                graph.add_edge(*builtin_index, index, SqlGraphRelationship::RequiredByArg);
            }
        }

        if let Some(arg) = &item.mstype {
            let found = make_type_or_enum_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &arg.ty_id,
                types,
                enums,
            );
            if !found {
                let builtin_index = builtin_types
                    .get(arg.full_path)
                    .expect(&format!("Could not fetch Builtin Type {}.", arg.full_path));
                tracing::debug!(from = %item.rust_identifier(), to = %arg.full_path, "Adding Aggregate after BuiltIn Type edge");
                graph.add_edge(*builtin_index, index, SqlGraphRelationship::RequiredByArg);
            }
        }

        make_extern_connection(
            graph,
            "Aggregate",
            index,
            &item.rust_identifier(),
            &(item.module_path.to_string() + "::" + item.sfunc),
            externs,
        );
        if let Some(value) = item.finalfunc {
            make_extern_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &(item.module_path.to_string() + "::" + value),
                externs,
            );
        }
        if let Some(value) = item.combinefunc {
            make_extern_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &(item.module_path.to_string() + "::" + value),
                externs,
            );
        }
        if let Some(value) = item.serialfunc {
            make_extern_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &(item.module_path.to_string() + "::" + value),
                externs,
            );
        }
        if let Some(value) = item.deserialfunc {
            make_extern_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &(item.module_path.to_string() + "::" + value),
                externs,
            );
        }
        if let Some(value) = item.initcond {
            make_extern_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &(item.module_path.to_string() + "::" + value),
                externs,
            );
        }
        if let Some(value) = item.msfunc {
            make_extern_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &(item.module_path.to_string() + "::" + value),
                externs,
            );
        }
        if let Some(value) = item.minvfunc {
            make_extern_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &(item.module_path.to_string() + "::" + value),
                externs,
            );
        }
        if let Some(value) = item.mfinalfunc {
            make_extern_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &(item.module_path.to_string() + "::" + value),
                externs,
            );
        }
        if let Some(value) = item.minitcond {
            make_extern_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &(item.module_path.to_string() + "::" + value),
                externs,
            );
        }
        if let Some(value) = item.sortop {
            make_extern_connection(
                graph,
                "Aggregate",
                index,
                &item.rust_identifier(),
                &(item.module_path.to_string() + "::" + value),
                externs,
            );
        }
    }
}

fn make_schema_connection(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    kind: &str,
    index: NodeIndex,
    rust_identifier: &str,
    module_path: &str,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
) -> bool {
    let mut found = false;
    for (schema_item, &schema_index) in schemas {
        if module_path == schema_item.module_path {
            tracing::debug!(from = ?rust_identifier, to = schema_item.module_path, "Adding {kind} after Schema edge.", kind = kind);
            graph.add_edge(schema_index, index, SqlGraphRelationship::RequiredBy);
            found = true;
            break;
        }
    }
    found
}

fn make_extern_connection(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    kind: &str,
    index: NodeIndex,
    rust_identifier: &str,
    full_path: &str,
    externs: &HashMap<PgExternEntity, NodeIndex>,
) -> bool {
    let mut found = false;
    for (extern_item, &extern_index) in externs {
        if full_path == extern_item.full_path {
            tracing::debug!(from = ?rust_identifier, to = extern_item.module_path, "Adding {kind} after Extern edge.", kind = kind);
            graph.add_edge(extern_index, index, SqlGraphRelationship::RequiredBy);
            found = true;
            break;
        }
    }
    found
}

fn make_type_or_enum_connection(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRelationship>,
    kind: &str,
    index: NodeIndex,
    rust_identifier: &str,
    ty_id: &TypeId,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
) -> bool {
    let mut found = false;
    for (ty_item, &ty_index) in types {
        if ty_item.id_matches(ty_id) {
            tracing::debug!(from = ?rust_identifier, to = ty_item.full_path, "Adding {kind} after Type edge.", kind = kind);
            graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
            found = true;
            break;
        }
    }
    for (ty_item, &ty_index) in enums {
        if ty_item.id_matches(ty_id) {
            tracing::debug!(from = ?rust_identifier, to = ty_item.full_path, "Adding {kind} after Enum edge.", kind = kind);
            graph.add_edge(ty_index, index, SqlGraphRelationship::RequiredBy);
            found = true;
            break;
        }
    }

    found
}
