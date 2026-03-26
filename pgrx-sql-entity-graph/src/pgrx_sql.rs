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

Rust to SQL mapping support.

> Like all of the [`sql_entity_graph`][crate] APIs, this is considered **internal**
> to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/

use eyre::eyre;
use petgraph::dot::Dot;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::path::Path;

use crate::aggregate::entity::PgAggregateEntity;
use crate::control_file::ControlFile;
use crate::extension_sql::SqlDeclared;
use crate::extension_sql::entity::{ExtensionSqlEntity, SqlDeclaredEntity};
use crate::metadata::TypeOrigin;
use crate::pg_extern::entity::PgExternEntity;
use crate::pg_trigger::entity::PgTriggerEntity;
use crate::positioning_ref::PositioningRef;
use crate::postgres_enum::entity::PostgresEnumEntity;
use crate::postgres_hash::entity::PostgresHashEntity;
use crate::postgres_ord::entity::PostgresOrdEntity;
use crate::postgres_type::entity::PostgresTypeEntity;
use crate::schema::entity::SchemaEntity;
use crate::to_sql::ToSql;
use crate::type_keyed;
use crate::{SqlGraphEntity, SqlGraphIdentifier};

use super::{PgExternReturnEntity, PgExternReturnEntityIteratedItem};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum SqlGraphRequires {
    By,
    ByArg,
    ByReturn,
}

/// A generator for SQL.
///
/// Consumes a base mapping of types (typically `pgrx::DEFAULT_TYPEID_SQL_MAPPING`), a
/// [`ControlFile`], and collections of each SQL entity.
///
/// During construction, a Directed Acyclic Graph is formed out the dependencies. For example,
/// an item `detect_dog(x: &[u8]) -> animals::Dog` would have have a relationship with
/// `animals::Dog`.
///
/// Typically, [`PgrxSql`] types are constructed in a `pgrx::pg_binary_magic!()` call in a binary
/// out of entities collected during a `pgrx::pg_module_magic!()` call in a library.
#[derive(Debug, Clone)]
pub struct PgrxSql {
    pub control: ControlFile,
    pub graph: StableGraph<SqlGraphEntity, SqlGraphRequires>,
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
    pub triggers: HashMap<PgTriggerEntity, NodeIndex>,
    pub extension_name: String,
    pub versioned_so: bool,
}

impl PgrxSql {
    pub fn build(
        entities: impl Iterator<Item = SqlGraphEntity>,
        extension_name: String,
        versioned_so: bool,
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
        let mut triggers: Vec<PgTriggerEntity> = Vec::default();
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
                SqlGraphEntity::Aggregate(input_aggregate) => {
                    aggregates.push(input_aggregate);
                }
                SqlGraphEntity::Trigger(input_trigger) => {
                    triggers.push(input_trigger);
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
        ensure_unique_type_targets(&mapped_types, &mapped_enums, &mapped_extension_sqls)?;
        let (mapped_externs, mut mapped_builtin_types) = initialize_externs(
            &mut graph,
            root,
            bootstrap,
            finalize,
            externs,
            &mapped_types,
            &mapped_enums,
            &mapped_extension_sqls,
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
            &mapped_extension_sqls,
        )?;
        let mapped_triggers = initialize_triggers(&mut graph, root, bootstrap, finalize, triggers)?;

        // Now we can circle back and build up the edge sets.
        connect_schemas(&mut graph, &mapped_schemas, root);
        connect_extension_sqls(
            &mut graph,
            &mapped_extension_sqls,
            &mapped_schemas,
            &mapped_types,
            &mapped_enums,
            &mapped_externs,
            &mapped_triggers,
        )?;
        connect_enums(&mut graph, &mapped_enums, &mapped_schemas);
        connect_types(&mut graph, &mapped_types, &mapped_schemas, &mapped_externs)?;
        connect_externs(
            &mut graph,
            &mapped_externs,
            &mapped_hashes,
            &mapped_schemas,
            &mapped_types,
            &mapped_enums,
            &mapped_builtin_types,
            &mapped_extension_sqls,
            &mapped_triggers,
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
            &mapped_extension_sqls,
        )?;
        connect_triggers(&mut graph, &mapped_triggers, &mapped_schemas);

        let this = Self {
            control,
            schemas: mapped_schemas,
            extension_sqls: mapped_extension_sqls,
            externs: mapped_externs,
            types: mapped_types,
            builtin_types: mapped_builtin_types,
            enums: mapped_enums,
            ords: mapped_ords,
            hashes: mapped_hashes,
            aggregates: mapped_aggregates,
            triggers: mapped_triggers,
            graph,
            graph_root: root,
            graph_bootstrap: bootstrap,
            graph_finalize: finalize,
            extension_name,
            versioned_so,
        };
        Ok(this)
    }

    // NOTE: this signature is demanded by the codegen we embed via cargo-pgrx
    pub fn to_file(&self, file: impl AsRef<Path> + Debug) -> eyre::Result<()> {
        use std::fs::{File, create_dir_all};
        use std::io::Write;
        let generated = self.to_sql()?;
        let path = Path::new(file.as_ref());

        let parent = path.parent();
        if let Some(parent) = parent {
            create_dir_all(parent)?;
        }
        let mut out = File::create(path)?;
        write!(out, "{generated}")?;
        Ok(())
    }

    pub fn write(&self, out: &mut impl std::io::Write) -> eyre::Result<()> {
        let generated = self.to_sql()?;

        #[cfg(feature = "syntax-highlighting")]
        {
            use std::io::{IsTerminal, stdout};
            if stdout().is_terminal() {
                self.write_highlighted(out, &generated)?;
            } else {
                write!(*out, "{}", generated)?;
            }
        }

        #[cfg(not(feature = "syntax-highlighting"))]
        {
            write!(*out, "{generated}")?;
        }

        Ok(())
    }

    #[cfg(feature = "syntax-highlighting")]
    fn write_highlighted(&self, out: &mut dyn std::io::Write, generated: &str) -> eyre::Result<()> {
        use eyre::WrapErr as _;
        use owo_colors::{OwoColorize, XtermColors};
        use syntect::easy::HighlightLines;
        use syntect::highlighting::{Style, ThemeSet};
        use syntect::parsing::SyntaxSet;
        use syntect::util::LinesWithEndings;
        let ps = SyntaxSet::load_defaults_newlines();
        let theme_bytes = include_str!("../assets/ansi.tmTheme").as_bytes();
        let mut theme_reader = std::io::Cursor::new(theme_bytes);
        let theme = ThemeSet::load_from_reader(&mut theme_reader)
            .wrap_err("Couldn't parse theme for SQL highlighting, try piping to a file")?;

        if let Some(syntax) = ps.find_syntax_by_extension("sql") {
            let mut h = HighlightLines::new(syntax, &theme);
            for line in LinesWithEndings::from(&generated) {
                let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps)?;
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
        Ok(())
    }

    // NOTE: this signature is demanded by the codegen we embed via cargo-pgrx
    pub fn to_dot(&self, file: impl AsRef<Path> + Debug) -> eyre::Result<()> {
        use std::fs::{File, create_dir_all};
        use std::io::Write;
        let generated = Dot::with_attr_getters(
            &self.graph,
            &[petgraph::dot::Config::EdgeNoLabel, petgraph::dot::Config::NodeNoLabel],
            &|_graph, edge| {
                match edge.weight() {
                    SqlGraphRequires::By => r#"color = "gray""#,
                    SqlGraphRequires::ByArg => r#"color = "black""#,
                    SqlGraphRequires::ByReturn => r#"dir = "back", color = "black""#,
                }
                .to_owned()
            },
            &|_graph, (_index, node)| {
                let dot_id = node.dot_identifier();
                match node {
                    // Colors derived from https://www.schemecolor.com/touch-of-creativity.php
                    SqlGraphEntity::Schema(_item) => {
                        format!("label = \"{dot_id}\", weight = 6, shape = \"tab\"")
                    }
                    SqlGraphEntity::Function(_item) => format!(
                        "label = \"{dot_id}\", penwidth = 0, style = \"filled\", fillcolor = \"#ADC7C6\", weight = 4, shape = \"box\"",
                    ),
                    SqlGraphEntity::Type(_item) => format!(
                        "label = \"{dot_id}\", penwidth = 0, style = \"filled\", fillcolor = \"#AE9BBD\", weight = 5, shape = \"oval\"",
                    ),
                    SqlGraphEntity::BuiltinType(_item) => {
                        format!("label = \"{dot_id}\", shape = \"plain\"")
                    }
                    SqlGraphEntity::Enum(_item) => format!(
                        "label = \"{dot_id}\", penwidth = 0, style = \"filled\", fillcolor = \"#C9A7C8\", weight = 5, shape = \"oval\""
                    ),
                    SqlGraphEntity::Ord(_item) => format!(
                        "label = \"{dot_id}\", penwidth = 0, style = \"filled\", fillcolor = \"#FFCFD3\", weight = 5, shape = \"diamond\""
                    ),
                    SqlGraphEntity::Hash(_item) => format!(
                        "label = \"{dot_id}\", penwidth = 0, style = \"filled\", fillcolor = \"#FFE4E0\", weight = 5, shape = \"diamond\""
                    ),
                    SqlGraphEntity::Aggregate(_item) => format!(
                        "label = \"{dot_id}\", penwidth = 0, style = \"filled\", fillcolor = \"#FFE4E0\", weight = 5, shape = \"diamond\""
                    ),
                    SqlGraphEntity::Trigger(_item) => format!(
                        "label = \"{dot_id}\", penwidth = 0, style = \"filled\", fillcolor = \"#FFE4E0\", weight = 5, shape = \"diamond\""
                    ),
                    SqlGraphEntity::CustomSql(_item) => {
                        format!("label = \"{dot_id}\", weight = 3, shape = \"signature\"")
                    }
                    SqlGraphEntity::ExtensionRoot(_item) => {
                        format!("label = \"{dot_id}\", shape = \"cylinder\"")
                    }
                }
            },
        );
        let path = Path::new(file.as_ref());

        let parent = path.parent();
        if let Some(parent) = parent {
            create_dir_all(parent)?;
        }
        let mut out = File::create(path)?;
        write!(out, "{generated:?}")?;
        Ok(())
    }

    pub fn schema_alias_of(&self, item_index: &NodeIndex) -> Option<String> {
        self.graph
            .neighbors_undirected(*item_index)
            .flat_map(|neighbor_index| match &self.graph[neighbor_index] {
                SqlGraphEntity::Schema(s) => Some(String::from(s.name)),
                SqlGraphEntity::ExtensionRoot(_control) => None,
                _ => None,
            })
            .next()
    }

    pub fn schema_prefix_for(&self, target: &NodeIndex) -> String {
        self.schema_alias_of(target).map(|v| (v + ".").to_string()).unwrap_or_default()
    }

    pub fn find_type_dependency(
        &self,
        owner: &NodeIndex,
        ty: &dyn crate::TypeIdentifiable,
    ) -> Option<NodeIndex> {
        self.graph
            .neighbors_undirected(*owner)
            .find(|neighbor| self.graph[*neighbor].type_matches(ty))
    }

    pub fn to_sql(&self) -> eyre::Result<String> {
        let mut full_sql = String::new();

        // NB:  A properly we'd *like* to maintain is that the schema generator outputs
        // consistent results from run-to-run when there are no changes to the schema.
        // This is to improve change detection using simple tools like `diff`.
        //
        // Historically, we used [`petgraph::algo:toposort`] but its ordering is not at all
        // consistent.
        //
        // [`petgraph::algo::tarjan_scc`] appears to be consistent, although it's not exactly
        // clear if this is due to an implementation detail or specifics of the algorithm itself.
        // (I, eeeebbbbrrrr, am not a graph theory expert)
        //
        // In any event, if in the future schema generation stops being consistent, this is the
        // place to look.
        //
        // We have no tests around this as it's really just a property we'd like to have, and
        // it does seem ensuring it is a bit of black magic.
        for nodes in petgraph::algo::tarjan_scc(&self.graph).iter().rev() {
            let mut inner_sql = Vec::with_capacity(nodes.len());

            for node in nodes {
                let step = &self.graph[*node];
                let sql = step.to_sql(self)?;

                let trimmed = sql.trim();
                if !trimmed.is_empty() {
                    inner_sql.push(format!("{trimmed}\n"))
                }
            }

            if !inner_sql.is_empty() {
                full_sql.push_str("/* <begin connected objects> */\n");
                full_sql.push_str(&inner_sql.join("\n\n"));
                full_sql.push_str("/* </end connected objects> */\n\n");
            }
        }

        Ok(full_sql)
    }

    pub fn has_sql_declared_entity(&self, identifier: &SqlDeclared) -> Option<&SqlDeclaredEntity> {
        self.extension_sqls.iter().find_map(|(item, _index)| {
            item.creates
                .iter()
                .find(|create_entity| create_entity.has_sql_declared_entity(identifier))
        })
    }

    pub fn get_module_pathname(&self) -> String {
        if self.versioned_so {
            let extname = &self.extension_name;
            let extver = &self.control.default_version;
            // Note: versioned so-name format must agree with cargo pgrx
            format!("{extname}-{extver}")
        } else {
            String::from("MODULE_PATHNAME")
        }
    }

    pub fn find_matching_fn(&self, name: &str) -> Option<&PgExternEntity> {
        self.externs.keys().find(|key| key.full_path.ends_with(name))
    }
}

fn build_base_edges(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    index: NodeIndex,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
) {
    graph.add_edge(root, index, SqlGraphRequires::By);
    if let Some(bootstrap) = bootstrap {
        graph.add_edge(bootstrap, index, SqlGraphRequires::By);
    }
    if let Some(finalize) = finalize {
        graph.add_edge(index, finalize, SqlGraphRequires::By);
    }
}

#[allow(clippy::type_complexity)]
fn initialize_extension_sqls(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    root: NodeIndex,
    extension_sqls: Vec<ExtensionSqlEntity>,
) -> eyre::Result<(HashMap<ExtensionSqlEntity, NodeIndex>, Option<NodeIndex>, Option<NodeIndex>)> {
    let mut bootstrap = None;
    let mut finalize = None;
    let mut mapped_extension_sqls = HashMap::default();
    for item in extension_sqls {
        let entity: SqlGraphEntity = item.clone().into();
        let index = graph.add_node(entity);
        mapped_extension_sqls.insert(item.clone(), index);

        if item.bootstrap {
            if let Some(existing_index) = bootstrap {
                let existing: &SqlGraphEntity = &graph[existing_index];
                return Err(eyre!(
                    "Cannot have multiple `extension_sql!()` with `bootstrap` positioning, found `{}`, other was `{}`",
                    item.rust_identifier(),
                    existing.rust_identifier(),
                ));
            }
            bootstrap = Some(index)
        }
        if item.finalize {
            if let Some(existing_index) = finalize {
                let existing: &SqlGraphEntity = &graph[existing_index];
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
        graph.add_edge(root, *index, SqlGraphRequires::By);
        if !item.bootstrap
            && let Some(bootstrap) = bootstrap
        {
            graph.add_edge(bootstrap, *index, SqlGraphRequires::By);
        }
        if !item.finalize
            && let Some(finalize) = finalize
        {
            graph.add_edge(*index, finalize, SqlGraphRequires::By);
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
    triggers: &'a HashMap<PgTriggerEntity, NodeIndex>,
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
                    return Some(other_index);
                }
            }
            for (other, other_index) in enums {
                if last_segment == &other.name && other.module_path.ends_with(&module_path) {
                    return Some(other_index);
                }
            }
            for (other, other_index) in externs {
                if *last_segment == other.unaliased_name
                    && other.module_path.ends_with(&module_path)
                {
                    return Some(other_index);
                }
            }
            for (other, other_index) in schemas {
                if other.module_path.ends_with(path) {
                    return Some(other_index);
                }
            }

            for (other, other_index) in triggers {
                if last_segment == &other.function_name && other.module_path.ends_with(&module_path)
                {
                    return Some(other_index);
                }
            }
        }
        PositioningRef::Name(name) => {
            for (other, other_index) in extension_sqls {
                if other.name == name {
                    return Some(other_index);
                }
            }
        }
    };
    None
}

fn connect_extension_sqls(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
    triggers: &HashMap<PgTriggerEntity, NodeIndex>,
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
                triggers,
            ) {
                graph.add_edge(*target, index, SqlGraphRequires::By);
            } else {
                return Err(eyre!(
                    "Could not find `requires` target of `{}`{}: {}",
                    item.rust_identifier(),
                    match (item.file(), item.line()) {
                        (Some(file), Some(line)) => format!(" ({file}:{line})"),
                        _ => "".to_string(),
                    },
                    match requires {
                        PositioningRef::FullPath(path) => path.to_string(),
                        PositioningRef::Name(name) => format!(r#""{name}""#),
                    },
                ));
            }
        }
    }
    Ok(())
}

fn initialize_schemas(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
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
            graph.add_edge(bootstrap, index, SqlGraphRequires::By);
        }
        if let Some(finalize) = finalize {
            graph.add_edge(index, finalize, SqlGraphRequires::By);
        }
    }
    Ok(mapped_schemas)
}

fn connect_schemas(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    root: NodeIndex,
) {
    for index in schemas.values().copied() {
        graph.add_edge(root, index, SqlGraphRequires::By);
    }
}

fn initialize_enums(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
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
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
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

fn initialize_types(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
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
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
) -> eyre::Result<()> {
    for (item, &index) in types {
        make_schema_connection(
            graph,
            "Type",
            index,
            &item.rust_identifier(),
            item.module_path,
            schemas,
        );

        make_extern_connection(
            graph,
            "Type",
            index,
            &item.rust_identifier(),
            &resolve_function_path(item.module_path, item.in_fn_path),
            externs,
        )?;
        make_extern_connection(
            graph,
            "Type",
            index,
            &item.rust_identifier(),
            &resolve_function_path(item.module_path, item.out_fn_path),
            externs,
        )?;
        if let Some(path) = item.receive_fn_path {
            make_extern_connection(
                graph,
                "Type",
                index,
                &item.rust_identifier(),
                &resolve_function_path(item.module_path, path),
                externs,
            )?;
        }
        if let Some(path) = item.send_fn_path {
            make_extern_connection(
                graph,
                "Type",
                index,
                &item.rust_identifier(),
                &resolve_function_path(item.module_path, path),
                externs,
            )?;
        }
    }
    Ok(())
}

fn initialize_externs(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    externs: Vec<PgExternEntity>,
    mapped_types: &HashMap<PostgresTypeEntity, NodeIndex>,
    mapped_enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    mapped_extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
) -> eyre::Result<(HashMap<PgExternEntity, NodeIndex>, HashMap<String, NodeIndex>)> {
    let mut mapped_externs = HashMap::default();
    let mut mapped_builtin_types = HashMap::default();
    for item in externs {
        let entity: SqlGraphEntity = item.clone().into();
        let index = graph.add_node(entity.clone());
        mapped_externs.insert(item.clone(), index);
        build_base_edges(graph, index, root, bootstrap, finalize);

        for arg in &item.fn_args {
            let slot = format!("argument `{}`", arg.pattern);
            initialize_resolved_type(
                graph,
                &mut mapped_builtin_types,
                arg.used_ty.metadata.schema_key,
                arg.used_ty.metadata.type_origin,
                mapped_types,
                mapped_enums,
                mapped_extension_sqls,
                "Function",
                item.full_path,
                &slot,
                arg.used_ty.full_path,
            )?;
        }

        match &item.fn_return {
            PgExternReturnEntity::None | PgExternReturnEntity::Trigger => (),
            PgExternReturnEntity::Type { ty, .. } | PgExternReturnEntity::SetOf { ty, .. } => {
                initialize_resolved_type(
                    graph,
                    &mut mapped_builtin_types,
                    ty.metadata.schema_key,
                    ty.metadata.type_origin,
                    mapped_types,
                    mapped_enums,
                    mapped_extension_sqls,
                    "Function",
                    item.full_path,
                    "return type",
                    ty.full_path,
                )?;
            }
            PgExternReturnEntity::Iterated { tys: iterated_returns, .. } => {
                for PgExternReturnEntityIteratedItem { ty, .. } in iterated_returns {
                    initialize_resolved_type(
                        graph,
                        &mut mapped_builtin_types,
                        ty.metadata.schema_key,
                        ty.metadata.type_origin,
                        mapped_types,
                        mapped_enums,
                        mapped_extension_sqls,
                        "Function",
                        item.full_path,
                        "table return column",
                        ty.full_path,
                    )?;
                }
            }
        }
    }
    Ok((mapped_externs, mapped_builtin_types))
}

fn connect_externs(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
    hashes: &HashMap<PostgresHashEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    builtin_types: &HashMap<String, NodeIndex>,
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
    triggers: &HashMap<PgTriggerEntity, NodeIndex>,
) -> eyre::Result<()> {
    for (item, &index) in externs {
        let mut found_schema_declaration = false;
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
                            triggers,
                        ) {
                            graph.add_edge(*target, index, SqlGraphRequires::By);
                        } else {
                            return Err(eyre!("Could not find `requires` target: {:?}", requires));
                        }
                    }
                }
                crate::ExternArgs::Support(support_fn) => {
                    if let Some(target) = find_positioning_ref_target(
                        support_fn,
                        types,
                        enums,
                        externs,
                        schemas,
                        extension_sqls,
                        triggers,
                    ) {
                        graph.add_edge(*target, index, SqlGraphRequires::By);
                    }
                }
                crate::ExternArgs::Schema(declared_schema_name) => {
                    for (schema, schema_index) in schemas {
                        if schema.name == declared_schema_name {
                            graph.add_edge(*schema_index, index, SqlGraphRequires::By);
                            found_schema_declaration = true;
                        }
                    }
                    if !found_schema_declaration {
                        return Err(eyre!(
                            "Got manual `schema = \"{declared_schema_name}\"` setting, but that schema did not exist."
                        ));
                    }
                }
                _ => (),
            }
        }

        if !found_schema_declaration {
            make_schema_connection(
                graph,
                "Extern",
                index,
                &item.rust_identifier(),
                item.module_path,
                schemas,
            );
        }

        // The hash function must be defined after the {typename}_eq function.
        for (hash_item, &hash_index) in hashes {
            if item.module_path == hash_item.module_path
                && item.name == hash_item.name.to_lowercase() + "_eq"
            {
                graph.add_edge(index, hash_index, SqlGraphRequires::By);
            }
        }

        for arg in &item.fn_args {
            let slot = format!("argument `{}`", arg.pattern);
            connect_resolved_type(
                graph,
                index,
                SqlGraphRequires::ByArg,
                arg.used_ty.metadata.schema_key,
                arg.used_ty.metadata.type_origin,
                types,
                enums,
                builtin_types,
                extension_sqls,
                "Function",
                item.full_path,
                &slot,
                arg.used_ty.full_path,
            )?;
        }

        match &item.fn_return {
            PgExternReturnEntity::None | PgExternReturnEntity::Trigger => (),
            PgExternReturnEntity::Type { ty, .. } | PgExternReturnEntity::SetOf { ty, .. } => {
                connect_resolved_type(
                    graph,
                    index,
                    SqlGraphRequires::ByReturn,
                    ty.metadata.schema_key,
                    ty.metadata.type_origin,
                    types,
                    enums,
                    builtin_types,
                    extension_sqls,
                    "Function",
                    item.full_path,
                    "return type",
                    ty.full_path,
                )?;
            }
            PgExternReturnEntity::Iterated { tys: iterated_returns, .. } => {
                for PgExternReturnEntityIteratedItem { ty, .. } in iterated_returns {
                    connect_resolved_type(
                        graph,
                        index,
                        SqlGraphRequires::ByReturn,
                        ty.metadata.schema_key,
                        ty.metadata.type_origin,
                        types,
                        enums,
                        builtin_types,
                        extension_sqls,
                        "Function",
                        item.full_path,
                        "table return column",
                        ty.full_path,
                    )?;
                }
            }
        }
    }
    Ok(())
}

fn initialize_ords(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
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
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
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
            item.schema_key,
            types,
            enums,
        );

        // Make PostgresOrdEntities (which will be translated into `CREATE OPERATOR CLASS` statements) depend
        // on the operators which they will reference. For example, a pgrx-defined Postgres type `parakeet`
        // which has `#[derive(PostgresOrd)]` will emit a `parakeet_btree_ops` operator class, which references
        // a definition of a < operator (among others) on the `parakeet` type. This code should ensure that the
        // < operator (along with all the others) is emitted before the `OPERATOR CLASS` itself.

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
                graph.add_edge(extern_index, index, SqlGraphRequires::By);
            }
        }
    }
}

fn initialize_hashes(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
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
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
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
            item.schema_key,
            types,
            enums,
        );

        if let Some((_, extern_index)) = externs.iter().find(|(extern_item, _)| {
            item.module_path == extern_item.module_path && extern_item.name == item.fn_name()
        }) {
            graph.add_edge(*extern_index, index, SqlGraphRequires::By);
        }
    }
}

fn initialize_aggregates(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    aggregates: Vec<PgAggregateEntity>,
    mapped_builtin_types: &mut HashMap<String, NodeIndex>,
    mapped_enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    mapped_types: &HashMap<PostgresTypeEntity, NodeIndex>,
    mapped_extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
) -> eyre::Result<HashMap<PgAggregateEntity, NodeIndex>> {
    let mut mapped_aggregates = HashMap::default();
    for item in aggregates {
        let entity: SqlGraphEntity = item.clone().into();
        let index = graph.add_node(entity);

        for arg in &item.args {
            let slot = aggregate_slot(arg.name, "argument");
            initialize_resolved_type(
                graph,
                mapped_builtin_types,
                arg.used_ty.metadata.schema_key,
                arg.used_ty.metadata.type_origin,
                mapped_types,
                mapped_enums,
                mapped_extension_sqls,
                "Aggregate",
                item.full_path,
                &slot,
                arg.used_ty.full_path,
            )?;
        }

        for arg in item.direct_args.as_ref().unwrap_or(&vec![]) {
            let slot = aggregate_slot(arg.name, "direct argument");
            initialize_resolved_type(
                graph,
                mapped_builtin_types,
                arg.used_ty.metadata.schema_key,
                arg.used_ty.metadata.type_origin,
                mapped_types,
                mapped_enums,
                mapped_extension_sqls,
                "Aggregate",
                item.full_path,
                &slot,
                arg.used_ty.full_path,
            )?;
        }

        initialize_resolved_type(
            graph,
            mapped_builtin_types,
            item.stype.used_ty.metadata.schema_key,
            item.stype.used_ty.metadata.type_origin,
            mapped_types,
            mapped_enums,
            mapped_extension_sqls,
            "Aggregate",
            item.full_path,
            "STYPE",
            item.stype.used_ty.full_path,
        )?;

        if let Some(arg) = &item.mstype {
            initialize_resolved_type(
                graph,
                mapped_builtin_types,
                arg.metadata.schema_key,
                arg.metadata.type_origin,
                mapped_types,
                mapped_enums,
                mapped_extension_sqls,
                "Aggregate",
                item.full_path,
                "MSTYPE",
                arg.full_path,
            )?;
        }

        mapped_aggregates.insert(item, index);
        build_base_edges(graph, index, root, bootstrap, finalize);
    }
    Ok(mapped_aggregates)
}

fn connect_aggregate(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    item: &PgAggregateEntity,
    index: NodeIndex,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    builtin_types: &HashMap<String, NodeIndex>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
) -> eyre::Result<()> {
    make_schema_connection(
        graph,
        "Aggregate",
        index,
        &item.rust_identifier(),
        item.module_path,
        schemas,
    );

    for arg in &item.args {
        let slot = aggregate_slot(arg.name, "argument");
        connect_resolved_type(
            graph,
            index,
            SqlGraphRequires::ByArg,
            arg.used_ty.metadata.schema_key,
            arg.used_ty.metadata.type_origin,
            types,
            enums,
            builtin_types,
            extension_sqls,
            "Aggregate",
            item.full_path,
            &slot,
            arg.used_ty.full_path,
        )?;
    }

    for arg in item.direct_args.as_ref().unwrap_or(&vec![]) {
        let slot = aggregate_slot(arg.name, "direct argument");
        connect_resolved_type(
            graph,
            index,
            SqlGraphRequires::ByArg,
            arg.used_ty.metadata.schema_key,
            arg.used_ty.metadata.type_origin,
            types,
            enums,
            builtin_types,
            extension_sqls,
            "Aggregate",
            item.full_path,
            &slot,
            arg.used_ty.full_path,
        )?;
    }

    if let Some(arg) = &item.mstype {
        connect_resolved_type(
            graph,
            index,
            SqlGraphRequires::ByArg,
            arg.metadata.schema_key,
            arg.metadata.type_origin,
            types,
            enums,
            builtin_types,
            extension_sqls,
            "Aggregate",
            item.full_path,
            "MSTYPE",
            arg.full_path,
        )?;
    }

    connect_resolved_type(
        graph,
        index,
        SqlGraphRequires::ByArg,
        item.stype.used_ty.metadata.schema_key,
        item.stype.used_ty.metadata.type_origin,
        types,
        enums,
        builtin_types,
        extension_sqls,
        "Aggregate",
        item.full_path,
        "STYPE",
        item.stype.used_ty.full_path,
    )?;

    make_extern_connection(
        graph,
        "Aggregate",
        index,
        &item.rust_identifier(),
        &(item.module_path.to_string() + "::" + item.sfunc),
        externs,
    )?;

    if let Some(value) = item.finalfunc {
        make_extern_connection(
            graph,
            "Aggregate",
            index,
            &item.rust_identifier(),
            &(item.module_path.to_string() + "::" + value),
            externs,
        )?;
    }
    if let Some(value) = item.combinefunc {
        make_extern_connection(
            graph,
            "Aggregate",
            index,
            &item.rust_identifier(),
            &(item.module_path.to_string() + "::" + value),
            externs,
        )?;
    }
    if let Some(value) = item.serialfunc {
        make_extern_connection(
            graph,
            "Aggregate",
            index,
            &item.rust_identifier(),
            &(item.module_path.to_string() + "::" + value),
            externs,
        )?;
    }
    if let Some(value) = item.deserialfunc {
        make_extern_connection(
            graph,
            "Aggregate",
            index,
            &item.rust_identifier(),
            &(item.module_path.to_string() + "::" + value),
            externs,
        )?;
    }
    if let Some(value) = item.msfunc {
        make_extern_connection(
            graph,
            "Aggregate",
            index,
            &item.rust_identifier(),
            &(item.module_path.to_string() + "::" + value),
            externs,
        )?;
    }
    if let Some(value) = item.minvfunc {
        make_extern_connection(
            graph,
            "Aggregate",
            index,
            &item.rust_identifier(),
            &(item.module_path.to_string() + "::" + value),
            externs,
        )?;
    }
    if let Some(value) = item.mfinalfunc {
        make_extern_connection(
            graph,
            "Aggregate",
            index,
            &item.rust_identifier(),
            &(item.module_path.to_string() + "::" + value),
            externs,
        )?;
    }
    if let Some(value) = item.sortop {
        make_extern_connection(
            graph,
            "Aggregate",
            index,
            &item.rust_identifier(),
            &(item.module_path.to_string() + "::" + value),
            externs,
        )?;
    }
    Ok(())
}

fn connect_aggregates(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    aggregates: &HashMap<PgAggregateEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    builtin_types: &HashMap<String, NodeIndex>,
    externs: &HashMap<PgExternEntity, NodeIndex>,
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
) -> eyre::Result<()> {
    for (item, &index) in aggregates {
        connect_aggregate(
            graph,
            item,
            index,
            schemas,
            types,
            enums,
            builtin_types,
            externs,
            extension_sqls,
        )?
    }
    Ok(())
}

fn initialize_triggers(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    root: NodeIndex,
    bootstrap: Option<NodeIndex>,
    finalize: Option<NodeIndex>,
    triggers: Vec<PgTriggerEntity>,
) -> eyre::Result<HashMap<PgTriggerEntity, NodeIndex>> {
    let mut mapped_triggers = HashMap::default();
    for item in triggers {
        let entity: SqlGraphEntity = item.clone().into();
        let index = graph.add_node(entity);

        mapped_triggers.insert(item, index);
        build_base_edges(graph, index, root, bootstrap, finalize);
    }
    Ok(mapped_triggers)
}

fn connect_triggers(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    triggers: &HashMap<PgTriggerEntity, NodeIndex>,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
) {
    for (item, &index) in triggers {
        make_schema_connection(
            graph,
            "Trigger",
            index,
            &item.rust_identifier(),
            item.module_path,
            schemas,
        );
    }
}

fn make_schema_connection(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    _kind: &str,
    index: NodeIndex,
    _rust_identifier: &str,
    module_path: &str,
    schemas: &HashMap<SchemaEntity, NodeIndex>,
) -> bool {
    let mut found = false;
    for (schema_item, &schema_index) in schemas {
        if module_path == schema_item.module_path {
            graph.add_edge(schema_index, index, SqlGraphRequires::By);
            found = true;
            break;
        }
    }
    found
}

fn make_extern_connection(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    _kind: &str,
    index: NodeIndex,
    _rust_identifier: &str,
    full_path: &str,
    externs: &HashMap<PgExternEntity, NodeIndex>,
) -> eyre::Result<()> {
    match externs.iter().find(|(extern_item, _)| full_path == extern_item.full_path) {
        Some((_, extern_index)) => {
            graph.add_edge(*extern_index, index, SqlGraphRequires::By);
            Ok(())
        }
        None => Err(eyre!("Did not find connection `{full_path}` in {:#?}", {
            let mut paths = externs.keys().map(|v| v.full_path).collect::<Vec<_>>();
            paths.sort();
            paths
        })),
    }
}

fn resolve_function_path(module_path: &str, path: &str) -> String {
    if path.contains("::") { path.to_string() } else { format!("{module_path}::{path}") }
}

fn aggregate_slot(name: Option<&str>, kind: &str) -> String {
    name.map(|name| format!("{kind} `{name}`")).unwrap_or_else(|| kind.to_string())
}

fn find_type_or_enum(
    schema_key: &str,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
) -> Option<NodeIndex> {
    types
        .iter()
        .map(type_keyed)
        .chain(enums.iter().map(type_keyed))
        .find(|(ty, _)| ty.matches_schema(schema_key))
        .map(|(_, index)| *index)
}

fn find_declared_type_or_enum(
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
    schema_key: &str,
) -> Option<NodeIndex> {
    extension_sqls.iter().find_map(|(item, index)| {
        item.creates
            .iter()
            .any(|declared| declared.matches_schema_key(schema_key))
            .then_some(*index)
    })
}

fn find_graph_type_target(
    schema_key: &str,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
) -> Option<NodeIndex> {
    find_type_or_enum(schema_key, types, enums)
        .or_else(|| find_declared_type_or_enum(extension_sqls, schema_key))
}

fn ensure_unique_type_targets(
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
) -> eyre::Result<()> {
    let mut seen = BTreeMap::<String, Vec<String>>::new();

    for item in types.keys() {
        seen.entry(item.schema_key.to_string())
            .or_default()
            .push(format!("type `{}`", item.full_path));
    }

    for item in enums.keys() {
        seen.entry(item.schema_key.to_string())
            .or_default()
            .push(format!("enum `{}`", item.full_path));
    }

    for (item, _) in extension_sqls {
        for declared in &item.creates {
            if let Some(schema_key) = declared.schema_key() {
                seen.entry(schema_key.to_string())
                    .or_default()
                    .push(format!("extension_sql `{}` ({declared})", item.name));
            }
        }
    }

    for locations in seen.values_mut() {
        locations.sort();
    }

    if let Some((schema_key, locations)) =
        seen.into_iter().find(|(_, locations)| locations.len() > 1)
    {
        return Err(eyre!(
            "schema key `{schema_key}` matched multiple SQL entities: {}",
            locations.join(", ")
        ));
    }

    Ok(())
}

fn unresolved_schema_key(
    owner_kind: &str,
    owner_name: &str,
    slot: &str,
    ty_name: &str,
    schema_key: &str,
) -> eyre::Report {
    eyre!(
        "{owner_kind} `{owner_name}` uses `{ty_name}` as {slot}, but schema key `{schema_key}` did not resolve. use `pgrx::pgrx_resolved_type!(T)` together with a matching `#[derive(PostgresType)]`, `#[derive(PostgresEnum)]`, or `extension_sql!(..., creates = [Type(T)]/[Enum(T)])`. for a manual mapping to an existing SQL type, set `TYPE_ORIGIN = TypeOrigin::External`."
    )
}

fn initialize_resolved_type(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    builtin_types: &mut HashMap<String, NodeIndex>,
    schema_key: &str,
    type_origin: TypeOrigin,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
    owner_kind: &str,
    owner_name: &str,
    slot: &str,
    ty_name: &str,
) -> eyre::Result<()> {
    if find_graph_type_target(schema_key, types, enums, extension_sqls).is_some() {
        return Ok(());
    }

    if matches!(type_origin, TypeOrigin::External) {
        builtin_types
            .entry(schema_key.to_string())
            .or_insert_with(|| graph.add_node(SqlGraphEntity::BuiltinType(schema_key.to_string())));
        return Ok(());
    }

    Err(unresolved_schema_key(owner_kind, owner_name, slot, ty_name, schema_key))
}

fn connect_resolved_type(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    index: NodeIndex,
    requires: SqlGraphRequires,
    schema_key: &str,
    type_origin: TypeOrigin,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
    builtin_types: &HashMap<String, NodeIndex>,
    extension_sqls: &HashMap<ExtensionSqlEntity, NodeIndex>,
    owner_kind: &str,
    owner_name: &str,
    slot: &str,
    ty_name: &str,
) -> eyre::Result<()> {
    if let Some(ty_index) = find_graph_type_target(schema_key, types, enums, extension_sqls) {
        graph.add_edge(ty_index, index, requires);
        return Ok(());
    }

    if let Some(builtin_index) = builtin_types.get(schema_key) {
        graph.add_edge(*builtin_index, index, requires);
        return Ok(());
    }

    if matches!(type_origin, TypeOrigin::External) {
        return Err(eyre!(
            "missing external-type placeholder for schema key `{schema_key}` while connecting {owner_kind} `{owner_name}` {slot}"
        ));
    }

    Err(unresolved_schema_key(owner_kind, owner_name, slot, ty_name, schema_key))
}

fn make_type_or_enum_connection(
    graph: &mut StableGraph<SqlGraphEntity, SqlGraphRequires>,
    _kind: &str,
    index: NodeIndex,
    _rust_identifier: &str,
    schema_key: &str,
    types: &HashMap<PostgresTypeEntity, NodeIndex>,
    enums: &HashMap<PostgresEnumEntity, NodeIndex>,
) -> bool {
    find_type_or_enum(schema_key, types, enums)
        .map(|ty_index| graph.add_edge(ty_index, index, SqlGraphRequires::By))
        .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UsedTypeEntity;
    use crate::aggregate::entity::{AggregateTypeEntity, PgAggregateEntity};
    use crate::extension_sql::entity::{ExtensionSqlEntity, SqlDeclaredEntityData};
    use crate::metadata::{FunctionMetadataTypeEntity, Returns, SqlMapping};
    use crate::pg_extern::entity::{PgExternArgumentEntity, PgExternEntity, PgExternReturnEntity};
    use crate::postgres_type::entity::PostgresTypeEntity;
    use crate::schema::entity::SchemaEntity;
    use crate::to_sql::entity::ToSqlConfigEntity;

    fn control_file() -> ControlFile {
        ControlFile {
            comment: "test".into(),
            default_version: "1.0".into(),
            module_pathname: None,
            relocatable: false,
            superuser: true,
            schema: None,
            trusted: false,
        }
    }

    fn to_sql_config() -> ToSqlConfigEntity {
        ToSqlConfigEntity { enabled: true, content: None }
    }

    fn used_type(
        full_path: &'static str,
        schema_key: &'static str,
        sql: &'static str,
        type_origin: TypeOrigin,
    ) -> UsedTypeEntity {
        UsedTypeEntity {
            ty_source: full_path,
            full_path,
            composite_type: None,
            variadic: false,
            default: None,
            optional: false,
            metadata: FunctionMetadataTypeEntity {
                schema_key,
                type_origin,
                argument_sql: Ok(SqlMapping::literal(sql)),
                return_sql: Ok(Returns::One(SqlMapping::literal(sql))),
            },
        }
    }

    fn external_type(
        full_path: &'static str,
        schema_key: &'static str,
        sql: &'static str,
    ) -> UsedTypeEntity {
        used_type(full_path, schema_key, sql, TypeOrigin::External)
    }

    fn extension_owned_type(
        full_path: &'static str,
        schema_key: &'static str,
        sql: &'static str,
    ) -> UsedTypeEntity {
        used_type(full_path, schema_key, sql, TypeOrigin::ThisExtension)
    }

    fn function_entity(
        name: &'static str,
        fn_args: Vec<PgExternArgumentEntity>,
        fn_return: PgExternReturnEntity,
    ) -> PgExternEntity {
        PgExternEntity {
            name,
            unaliased_name: name,
            module_path: "tests",
            full_path: Box::leak(format!("tests::{name}").into_boxed_str()),
            fn_args,
            fn_return,
            schema: None,
            file: "test.rs",
            line: 1,
            extern_attrs: vec![],
            search_path: None,
            operator: None,
            cast: None,
            to_sql_config: to_sql_config(),
        }
    }

    fn aggregate_entity(
        name: &'static str,
        args: Vec<AggregateTypeEntity>,
        stype: UsedTypeEntity,
        mstype: Option<UsedTypeEntity>,
    ) -> PgAggregateEntity {
        PgAggregateEntity {
            full_path: Box::leak(format!("tests::{name}").into_boxed_str()),
            module_path: "tests",
            file: "test.rs",
            line: 1,
            name,
            ordered_set: false,
            args,
            direct_args: None,
            stype: AggregateTypeEntity { used_ty: stype, name: None },
            sfunc: "state_fn",
            finalfunc: None,
            finalfunc_modify: None,
            combinefunc: None,
            serialfunc: None,
            deserialfunc: None,
            initcond: None,
            msfunc: None,
            minvfunc: None,
            mstype,
            mfinalfunc: None,
            mfinalfunc_modify: None,
            minitcond: None,
            sortop: None,
            parallel: None,
            hypothetical: false,
            to_sql_config: to_sql_config(),
        }
    }

    fn declared_type_sql(
        module_path: &'static str,
        full_path: &'static str,
        declaration_name: &'static str,
        name: &'static str,
        schema_key: &'static str,
        sql: &'static str,
    ) -> ExtensionSqlEntity {
        ExtensionSqlEntity {
            module_path,
            full_path,
            sql: "CREATE TYPE custom_type;",
            file: "test.rs",
            line: 1,
            name: declaration_name,
            bootstrap: false,
            finalize: false,
            requires: vec![],
            creates: vec![crate::extension_sql::entity::SqlDeclaredEntity::Type(
                SqlDeclaredEntityData {
                    sql: sql.into(),
                    name: name.into(),
                    schema_key: schema_key.into(),
                    type_origin: Some(TypeOrigin::ThisExtension),
                },
            )],
        }
    }

    fn schema_entity(module_path: &'static str, name: &'static str) -> SchemaEntity {
        SchemaEntity { module_path, name, file: "test.rs", line: 1 }
    }

    fn type_entity(
        name: &'static str,
        full_path: &'static str,
        schema_key: &'static str,
    ) -> PostgresTypeEntity {
        PostgresTypeEntity {
            name,
            file: "test.rs",
            line: 1,
            full_path,
            module_path: "tests",
            schema_key,
            in_fn_path: "in_fn",
            out_fn_path: "out_fn",
            receive_fn_path: None,
            send_fn_path: None,
            to_sql_config: to_sql_config(),
            alignment: None,
        }
    }

    fn state_function() -> PgExternEntity {
        function_entity("state_fn", vec![], PgExternReturnEntity::None)
    }

    #[test]
    fn external_function_type_resolution_succeeds() {
        let manual_text =
            used_type("tests::ManualText", "tests::ManualText", "TEXT", TypeOrigin::External);
        let function = function_entity(
            "manual_text_echo",
            vec![PgExternArgumentEntity { pattern: "value", used_ty: manual_text.clone() }],
            PgExternReturnEntity::Type { ty: manual_text.clone() },
        );

        let sql = PgrxSql::build(
            vec![SqlGraphEntity::ExtensionRoot(control_file()), SqlGraphEntity::Function(function)]
                .into_iter(),
            "test".into(),
            false,
        )
        .unwrap();

        assert!(sql.builtin_types.contains_key("tests::ManualText"));
    }

    #[test]
    fn extension_sql_declared_type_orders_before_function_and_aggregate() {
        let custom_type = extension_owned_type("tests::HexInt", "tests::HexInt", "hexint");
        let declared_type = declared_type_sql(
            "tests",
            "tests::concrete_type",
            "concrete_type",
            "tests::HexInt",
            "tests::HexInt",
            "hexint",
        );
        let function = function_entity(
            "takes_hexint",
            vec![PgExternArgumentEntity { pattern: "value", used_ty: custom_type.clone() }],
            PgExternReturnEntity::None,
        );
        let aggregate = aggregate_entity(
            "hexint_accum",
            vec![AggregateTypeEntity { used_ty: custom_type.clone(), name: Some("value") }],
            custom_type.clone(),
            Some(custom_type.clone()),
        );
        let state_fn = state_function();

        let sql = PgrxSql::build(
            vec![
                SqlGraphEntity::ExtensionRoot(control_file()),
                SqlGraphEntity::CustomSql(declared_type.clone()),
                SqlGraphEntity::Function(state_fn),
                SqlGraphEntity::Function(function.clone()),
                SqlGraphEntity::Aggregate(aggregate.clone()),
            ]
            .into_iter(),
            "test".into(),
            false,
        )
        .unwrap();

        let declared_index = sql.extension_sqls[&declared_type];
        let function_index = sql.externs[&function];
        let aggregate_index = sql.aggregates[&aggregate];

        assert!(!sql.builtin_types.contains_key("tests::HexInt"));
        assert!(sql.graph.find_edge(declared_index, function_index).is_some());
        assert!(sql.graph.find_edge(declared_index, aggregate_index).is_some());
    }

    #[test]
    fn extension_sql_declared_type_in_custom_schema_prefixes_aggregate_state_type() {
        let custom_type = extension_owned_type("tests::HexInt", "tests::HexInt", "hexint");
        let declared_type = declared_type_sql(
            "tests::custom_schema",
            "tests::custom_schema::hexint_sql",
            "hexint_sql",
            "tests::HexInt",
            "tests::HexInt",
            "hexint",
        );
        let aggregate = aggregate_entity("hexint_accum", vec![], custom_type, None);
        let state_fn = state_function();
        let schema = schema_entity("tests::custom_schema", "custom_schema");

        let sql = PgrxSql::build(
            vec![
                SqlGraphEntity::ExtensionRoot(control_file()),
                SqlGraphEntity::Schema(schema),
                SqlGraphEntity::CustomSql(declared_type),
                SqlGraphEntity::Function(state_fn),
                SqlGraphEntity::Aggregate(aggregate),
            ]
            .into_iter(),
            "test".into(),
            false,
        )
        .unwrap()
        .to_sql()
        .unwrap();

        assert!(sql.contains("STYPE = custom_schema.hexint"));
    }

    #[test]
    fn duplicate_schema_key_errors() {
        let left = type_entity("LeftType", "tests::LeftType", "tests::SharedType");
        let right = type_entity("RightType", "tests::RightType", "tests::SharedType");

        let error = PgrxSql::build(
            vec![
                SqlGraphEntity::ExtensionRoot(control_file()),
                SqlGraphEntity::Type(left),
                SqlGraphEntity::Type(right),
            ]
            .into_iter(),
            "test".into(),
            false,
        )
        .expect_err("duplicate schema keys should fail");

        assert!(error.to_string().contains("tests::SharedType"));
        assert!(error.to_string().contains("tests::LeftType"));
        assert!(error.to_string().contains("tests::RightType"));
    }

    #[test]
    fn unresolved_function_argument_schema_key_errors() {
        let bad_type = extension_owned_type("tests::BadArg", "tests::BadArg", "TEXT");
        let function = function_entity(
            "bad_arg",
            vec![PgExternArgumentEntity { pattern: "value", used_ty: bad_type }],
            PgExternReturnEntity::None,
        );

        let error = PgrxSql::build(
            vec![SqlGraphEntity::ExtensionRoot(control_file()), SqlGraphEntity::Function(function)]
                .into_iter(),
            "test".into(),
            false,
        )
        .expect_err("function argument should fail");

        assert!(error.to_string().contains("Function `tests::bad_arg`"));
        assert!(error.to_string().contains("argument `value`"));
        assert!(error.to_string().contains("tests::BadArg"));
    }

    #[test]
    fn unresolved_function_return_schema_key_errors() {
        let bad_type = extension_owned_type("tests::BadReturn", "tests::BadReturn", "TEXT");
        let function =
            function_entity("bad_return", vec![], PgExternReturnEntity::Type { ty: bad_type });

        let error = PgrxSql::build(
            vec![SqlGraphEntity::ExtensionRoot(control_file()), SqlGraphEntity::Function(function)]
                .into_iter(),
            "test".into(),
            false,
        )
        .expect_err("function return should fail");

        assert!(error.to_string().contains("Function `tests::bad_return`"));
        assert!(error.to_string().contains("return type"));
        assert!(error.to_string().contains("tests::BadReturn"));
    }

    #[test]
    fn unresolved_aggregate_argument_schema_key_errors() {
        let aggregate = aggregate_entity(
            "bad_aggregate_arg",
            vec![AggregateTypeEntity {
                used_ty: extension_owned_type("tests::BadArg", "tests::BadArg", "TEXT"),
                name: Some("value"),
            }],
            external_type("tests::State", "tests::State", "TEXT"),
            None,
        );

        let error = PgrxSql::build(
            vec![
                SqlGraphEntity::ExtensionRoot(control_file()),
                SqlGraphEntity::Function(state_function()),
                SqlGraphEntity::Aggregate(aggregate),
            ]
            .into_iter(),
            "test".into(),
            false,
        )
        .expect_err("aggregate argument should fail");

        assert!(error.to_string().contains("Aggregate `tests::bad_aggregate_arg`"));
        assert!(error.to_string().contains("argument `value`"));
        assert!(error.to_string().contains("tests::BadArg"));
    }

    #[test]
    fn unresolved_aggregate_stype_schema_key_errors() {
        let aggregate = aggregate_entity(
            "bad_aggregate_stype",
            vec![],
            extension_owned_type("tests::BadState", "tests::BadState", "TEXT"),
            None,
        );

        let error = PgrxSql::build(
            vec![
                SqlGraphEntity::ExtensionRoot(control_file()),
                SqlGraphEntity::Function(state_function()),
                SqlGraphEntity::Aggregate(aggregate),
            ]
            .into_iter(),
            "test".into(),
            false,
        )
        .expect_err("aggregate stype should fail");

        assert!(error.to_string().contains("Aggregate `tests::bad_aggregate_stype`"));
        assert!(error.to_string().contains("STYPE"));
        assert!(error.to_string().contains("tests::BadState"));
    }

    #[test]
    fn unresolved_aggregate_mstype_schema_key_errors() {
        let aggregate = aggregate_entity(
            "bad_aggregate_mstype",
            vec![],
            external_type("tests::State", "tests::State", "TEXT"),
            Some(extension_owned_type("tests::BadMovingState", "tests::BadMovingState", "TEXT")),
        );

        let error = PgrxSql::build(
            vec![
                SqlGraphEntity::ExtensionRoot(control_file()),
                SqlGraphEntity::Function(state_function()),
                SqlGraphEntity::Aggregate(aggregate),
            ]
            .into_iter(),
            "test".into(),
            false,
        )
        .expect_err("aggregate mstype should fail");

        assert!(error.to_string().contains("Aggregate `tests::bad_aggregate_mstype`"));
        assert!(error.to_string().contains("MSTYPE"));
        assert!(error.to_string().contains("tests::BadMovingState"));
    }
}
