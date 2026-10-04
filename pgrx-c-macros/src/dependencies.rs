//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Conservative references among final active macros and compiler-resolved constants.

//! The graph supports deterministic skip propagation and dependency explanations. Lexical
//! references cover final active macro bodies, and prepared expansion adds compiler-discovered
//! references such as pasted names. Macro nodes and terminal integer-constant users stay
//! separate, allowing a binding disagreement to seed affected callers without inventing a
//! macro definition for a declaration constant.

/// Connect this phase to the crate’s owned compiler facts and shared pipeline result types.
use crate::{
    ExpansionBatch, IntegerConstant, MacroDefinition, MacroEnvironment, MacroKind, TokenKind,
};
/// Represent immutable caller-to-dependency edges and traverse them in deterministic order.
use petgraph::Direction::{Incoming, Outgoing};
/// Represent immutable caller-to-dependency edges and traverse them in deterministic order.
use petgraph::graph::{DiGraph, NodeIndex};
/// Serialize owned inspection/analysis facts without borrowing from compiler translation units.
use serde::Serialize;
/// Keep catalog lookup and report ordering deterministic while bounding repeated traversal.
use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

/// Conservative macro references, including object macros and external context.
///
/// An edge runs from a macro to an active macro named in its replacement tokens.
/// Formal parameters, comments and literals never create edges. A lexical edge
/// does not prove that preprocessing expands the referenced macro at an invocation.
/// Prepared sessions also add dependencies synthesized by compiler token pasting.
#[derive(Clone, Debug)]
pub struct MacroDependencyGraph {
    // Nodes remain in lexical name order; no node can be inserted or removed
    // after construction. This permits borrowed name lookup without duplicating
    // the name strings in another map.
    /// Immutable sorted macro nodes and directed caller-to-dependency edges.
    graph: DiGraph<String, ()>,
    /// Terminal declaration-constant references kept separate from macro nodes for binding-mismatch
    /// propagation.
    constant_users: BTreeMap<String, Vec<NodeIndex>>,
}

/// One macro reached by following callers of a seeded dependency.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct MacroDependencyImpact {
    /// The caller affected by a seeded dependency.
    pub name: String,
    /// Its immediate dependency on the discovered path toward the root.
    pub dependency: String,
    /// The seeded macro at the end of that path.
    pub root: String,
}

/// Construct deterministic references and trace affected callers without repeated visits through
/// cycles.
impl MacroDependencyGraph {
    /// Keep the inspected lexical graph immutable, adding the compiler's
    /// synthesized references for this prepared batch. Rebuild sorted edges so
    /// dependency explanations stay deterministic after augmentation.
    pub(crate) fn with_expansions(
        &self,
        batch: &ExpansionBatch,
        constants: &BTreeMap<String, IntegerConstant>,
    ) -> Self {
        let mut edges = self
            .graph
            .raw_edges()
            .iter()
            .map(|edge| (edge.source().index(), edge.target().index()))
            .collect::<BTreeSet<_>>();
        let mut users = self
            .constant_users
            .iter()
            .map(|(name, nodes)| (name.clone(), nodes.iter().copied().collect::<BTreeSet<_>>()))
            .collect::<BTreeMap<_, _>>();
        for (name, references) in &batch.discovered_dependencies {
            let Some(caller) = self.node(name) else { continue };
            for reference in references {
                if let Some(target) = self.node(reference) {
                    edges.insert((caller.index(), target.index()));
                } else if constants.contains_key(reference) {
                    users.entry(reference.clone()).or_default().insert(caller);
                }
            }
        }
        let mut graph = self.graph.clone();
        graph.clear_edges();
        for (caller, target) in edges.into_iter().rev() {
            graph.add_edge(NodeIndex::new(caller), NodeIndex::new(target), ());
        }
        Self {
            graph,
            constant_users: users
                .into_iter()
                .map(|(name, users)| (name, users.into_iter().collect()))
                .collect(),
        }
    }

    /// Build a lexical macro graph for tests without terminal declaration constants.
    #[cfg(test)]
    pub(crate) fn from_environment(environment: &MacroEnvironment) -> Self {
        Self::from_environment_with_constants(environment, &BTreeMap::new())
    }

    /// Build sorted active-macro edges and deduplicated integer-constant users while excluding formal
    /// substitutions.
    pub(crate) fn from_environment_with_constants(
        environment: &MacroEnvironment,
        constants: &BTreeMap<String, IntegerConstant>,
    ) -> Self {
        let mut graph = DiGraph::with_capacity(environment.active.len(), 0);
        for name in environment.active.keys() {
            graph.add_node(name.clone());
        }
        let mut dependencies = Self { graph, constant_users: BTreeMap::new() };
        let mut seen = vec![usize::MAX; dependencies.node_count()];
        let mut targets = Vec::new();
        // Petgraph prepends adjacency links. Reversing callers and sorted targets
        // makes both incoming and outgoing iteration lexical and deterministic.
        for (caller, active) in environment.active.values().enumerate().rev() {
            let (parameters, body_start) = signature(&active.definition);
            targets.clear();
            for token in &active.definition.tokens[body_start..] {
                if !matches!(token.kind, TokenKind::Identifier | TokenKind::Keyword)
                    || parameters.contains(token.spelling.as_str())
                {
                    continue;
                }
                if let Some(target) = dependencies.node(&token.spelling) {
                    if seen[target.index()] != caller {
                        seen[target.index()] = caller;
                        targets.push(target);
                    }
                } else if constants.contains_key(&token.spelling) {
                    let caller = NodeIndex::new(caller);
                    if let Some(users) = dependencies.constant_users.get_mut(&token.spelling) {
                        // Each caller's tokens are contiguous in this scan, so the
                        // last stored caller deduplicates every repeated enum use.
                        if users.last() != Some(&caller) {
                            users.push(caller);
                        }
                    } else {
                        dependencies.constant_users.insert(token.spelling.clone(), vec![caller]);
                    }
                }
            }
            targets.sort_unstable();
            for &target in targets.iter().rev() {
                dependencies.graph.add_edge(NodeIndex::new(caller), target, ());
            }
        }
        for users in dependencies.constant_users.values_mut() {
            users.reverse();
        }
        dependencies
    }

    /// Number of final active macro definitions, including isolated context macros.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Number of distinct directed macro references.
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    /// Active macros named by this macro, in lexical order. Unknown names are empty.
    pub fn dependencies(&self, name: &str) -> impl Iterator<Item = &str> + '_ {
        self.node(name)
            .into_iter()
            .flat_map(|node| self.graph.neighbors_directed(node, Outgoing))
            .map(|node| self.graph[node].as_str())
    }

    /// Macros that name this macro, in lexical order. Unknown names are empty.
    pub fn dependents(&self, name: &str) -> impl Iterator<Item = &str> + '_ {
        self.node(name)
            .into_iter()
            .flat_map(|node| self.graph.neighbors_directed(node, Incoming))
            .map(|node| self.graph[node].as_str())
    }

    /// Macros that name this compiler-owned integer constant, in lexical order.
    /// Formal parameters and active macros shadow the constant. These terminal
    /// references are separate from the macro graph's nodes and edges.
    pub fn constant_users(&self, name: &str) -> impl Iterator<Item = &str> + '_ {
        self.constant_users.get(name).into_iter().flatten().map(|&node| self.graph[node].as_str())
    }

    /// Follow incoming edges from all known roots with one breadth-first traversal.
    ///
    /// Roots are excluded from results. A caller appears once even through cycles
    /// or diamonds; its path is the first reached in deterministic lexical root
    /// and neighbor order. Unknown roots are ignored. Traversal visits each node
    /// and incoming edge at most once, with O(V) scratch space and owned output.
    pub fn impacts(&self, roots: &[impl AsRef<str>]) -> Vec<MacroDependencyImpact> {
        let mut visited = vec![false; self.node_count()];
        for root in roots {
            if let Some(node) = self.node(root.as_ref()) {
                visited[node.index()] = true;
            }
        }
        let mut pending = visited
            .iter()
            .enumerate()
            .filter(|(_, seeded)| **seeded)
            .map(|(index, _)| {
                let node = NodeIndex::new(index);
                (node, node)
            })
            .collect::<VecDeque<_>>();
        let mut impacts = Vec::new();
        while let Some((parent, root)) = pending.pop_front() {
            for caller in self.graph.neighbors_directed(parent, Incoming) {
                if std::mem::replace(&mut visited[caller.index()], true) {
                    continue;
                }
                impacts.push(MacroDependencyImpact {
                    name: self.graph[caller].clone(),
                    dependency: self.graph[parent].clone(),
                    root: self.graph[root].clone(),
                });
                pending.push_back((caller, root));
            }
        }
        impacts
    }

    /// Find a macro by binary search over immutable lexical node order without a duplicate name
    /// index.
    fn node(&self, name: &str) -> Option<NodeIndex> {
        self.graph
            .raw_nodes()
            .binary_search_by(|node| node.weight.as_str().cmp(name))
            .ok()
            .map(NodeIndex::new)
    }
}

/// Identify signature formals and the replacement-body start so formal names do not create dependency
/// edges.
fn signature(definition: &MacroDefinition) -> (HashSet<&str>, usize) {
    if definition.kind == MacroKind::ObjectLike {
        return (HashSet::new(), definition.tokens.len().min(1));
    }
    let mut parameters = HashSet::new();
    for (index, token) in definition.tokens.iter().enumerate().skip(1) {
        match token.spelling.as_str() {
            ")" => return (parameters, index + 1),
            "..." => {
                parameters.insert("__VA_ARGS__");
            }
            _ if matches!(token.kind, TokenKind::Identifier | TokenKind::Keyword) => {
                parameters.insert(token.spelling.as_str());
            }
            _ => {}
        }
    }
    // Inspected definitions have valid signatures. A malformed test/constructed
    // definition has no established body to scan.
    (parameters, definition.tokens.len())
}

/// Exercise this phase’s semantic boundaries with owned fixtures.
/// These regressions check accepted proofs and explicit refusals without changing production
/// headers or weakening the C identity and evaluation contracts.
#[cfg(test)]
mod tests {
    /// Reuse the enclosing phase’s compiler/parser primitives so this subphase shares the same
    /// validation and input contract.
    use super::*;
    /// Connect this phase to the crate’s owned compiler facts and shared pipeline result types.
    use crate::{ActiveMacro, ActiveProvenance, Token};

    /// Create identifier tokens for dependency fixtures with explicit lexical categories.
    fn token(kind: TokenKind, spelling: &str) -> Token {
        Token { kind, spelling: spelling.into() }
    }

    /// Construct a source-independent object macro fixture for graph traversal tests.
    fn object(name: &str, references: &[&str]) -> ActiveMacro {
        let mut tokens = vec![token(TokenKind::Identifier, name)];
        tokens.extend(references.iter().map(|name| token(TokenKind::Identifier, name)));
        ActiveMacro {
            definition: MacroDefinition {
                name: name.into(),
                kind: MacroKind::ObjectLike,
                location: None,
                provenance: None,
                tokens,
                builtin: false,
                main_file: true,
            },
            provenance: ActiveProvenance::Resolved,
        }
    }

    /// Construct a function-macro fixture whose formals can exercise dependency shadowing.
    fn function(name: &str, parameters: &[&str], body: Vec<Token>) -> ActiveMacro {
        let mut active = object(name, &[]);
        active.definition.kind = MacroKind::FunctionLike;
        active.definition.tokens.push(token(TokenKind::Punctuation, "("));
        for (index, parameter) in parameters.iter().enumerate() {
            if index != 0 {
                active.definition.tokens.push(token(TokenKind::Punctuation, ","));
            }
            let kind =
                if *parameter == "..." { TokenKind::Punctuation } else { TokenKind::Identifier };
            active.definition.tokens.push(token(kind, parameter));
        }
        active.definition.tokens.push(token(TokenKind::Punctuation, ")"));
        active.definition.tokens.extend(body);
        active
    }

    /// Assemble deterministic active definitions for graph tests without running Clang.
    fn environment(macros: Vec<ActiveMacro>) -> MacroEnvironment {
        MacroEnvironment {
            active: macros
                .into_iter()
                .map(|active| (active.definition.name.clone(), active))
                .collect(),
        }
    }

    /// Construct the expected caller/dependency/root explanation used to check propagation paths.
    fn impact(name: &str, dependency: &str, root: &str) -> MacroDependencyImpact {
        MacroDependencyImpact {
            name: name.into(),
            dependency: dependency.into(),
            root: root.into(),
        }
    }

    /// Checks chain records each immediate parent toward the failed root.
    #[test]
    fn chain_records_each_immediate_parent_toward_the_failed_root() {
        let graph = MacroDependencyGraph::from_environment(&environment(vec![
            object("ROOT", &[]),
            object("MIDDLE", &["ROOT"]),
            object("TOP", &["MIDDLE"]),
        ]));
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
        assert_eq!(
            graph.impacts(&["ROOT"]),
            [impact("MIDDLE", "ROOT", "ROOT"), impact("TOP", "MIDDLE", "ROOT")]
        );
        assert_eq!(graph.dependencies("TOP").collect::<Vec<_>>(), ["MIDDLE"]);
        assert_eq!(graph.dependents("ROOT").collect::<Vec<_>>(), ["MIDDLE"]);
    }

    /// Checks diamond has one deterministic shortest path per caller.
    #[test]
    fn diamond_has_one_deterministic_shortest_path_per_caller() {
        let graph = MacroDependencyGraph::from_environment(&environment(vec![
            object("ROOT", &[]),
            object("LEFT", &["ROOT"]),
            object("RIGHT", &["ROOT"]),
            object("TOP", &["RIGHT", "LEFT", "RIGHT"]),
        ]));
        assert_eq!(graph.edge_count(), 4);
        assert_eq!(graph.dependencies("TOP").collect::<Vec<_>>(), ["LEFT", "RIGHT"]);
        assert_eq!(graph.dependents("ROOT").collect::<Vec<_>>(), ["LEFT", "RIGHT"]);
        assert_eq!(
            graph.impacts(&["ROOT"]),
            [
                impact("LEFT", "ROOT", "ROOT"),
                impact("RIGHT", "ROOT", "ROOT"),
                impact("TOP", "LEFT", "ROOT")
            ]
        );
    }

    /// Checks cycles and self edges visit nodes once and exclude all roots.
    #[test]
    fn cycles_and_self_edges_visit_nodes_once_and_exclude_all_roots() {
        let graph = MacroDependencyGraph::from_environment(&environment(vec![
            object("A", &["A", "B"]),
            object("B", &["A"]),
            object("C", &["B"]),
        ]));
        assert_eq!(graph.edge_count(), 4);
        assert_eq!(graph.impacts(&["A"]), [impact("B", "A", "A"), impact("C", "B", "A")]);
        assert_eq!(graph.impacts(&["B", "A", "A"]), [impact("C", "B", "B")]);
    }

    /// Checks multiple roots and unknown names are deterministic.
    #[test]
    fn multiple_roots_and_unknown_names_are_deterministic() {
        let graph = MacroDependencyGraph::from_environment(&environment(vec![
            object("A", &[]),
            object("B", &[]),
            object("MIDDLE", &["B", "A"]),
            object("TOP", &["MIDDLE"]),
        ]));
        let expected = [impact("MIDDLE", "A", "A"), impact("TOP", "MIDDLE", "A")];
        assert_eq!(graph.impacts(&["B", "missing", "A"]), expected);
        assert_eq!(graph.impacts(&["A", "B", "A"]), expected);
        assert!(graph.impacts(&["missing"]).is_empty());
        assert_eq!(graph.dependencies("missing").count(), 0);
        assert_eq!(graph.dependents("missing").count(), 0);
    }

    /// Checks formal parameters comments and literals do not create edges.
    #[test]
    fn formal_parameters_comments_and_literals_do_not_create_edges() {
        let graph = MacroDependencyGraph::from_environment(&environment(vec![
            object("SHADOW", &[]),
            object("OTHER", &[]),
            object("__VA_ARGS__", &[]),
            function(
                "CALLER",
                &["SHADOW", "..."],
                vec![
                    token(TokenKind::Identifier, "SHADOW"),
                    token(TokenKind::Comment, "OTHER"),
                    token(TokenKind::Literal, "OTHER"),
                    token(TokenKind::Identifier, "__VA_ARGS__"),
                    token(TokenKind::Identifier, "OTHER"),
                ],
            ),
        ]));
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.dependencies("CALLER").collect::<Vec<_>>(), ["OTHER"]);
        assert!(graph.dependents("SHADOW").next().is_none());
        assert!(graph.dependents("__VA_ARGS__").next().is_none());
    }

    /// Checks object function external and compiler macros share the same graph.
    #[test]
    fn object_function_external_and_compiler_macros_share_the_same_graph() {
        let mut external = object("EXTERNAL", &["__COMPILER"]);
        external.definition.main_file = false;
        let mut compiler = object("__COMPILER", &[]);
        compiler.definition.builtin = true;
        compiler.definition.main_file = false;
        let graph = MacroDependencyGraph::from_environment(&environment(vec![
            function("CALLER", &["value"], vec![token(TokenKind::Identifier, "OBJECT")]),
            object("OBJECT", &["EXTERNAL"]),
            external,
            compiler,
        ]));
        assert_eq!(graph.node_count(), 4);
        assert_eq!(
            graph.impacts(&["__COMPILER"]),
            [
                impact("EXTERNAL", "__COMPILER", "__COMPILER"),
                impact("OBJECT", "EXTERNAL", "__COMPILER"),
                impact("CALLER", "OBJECT", "__COMPILER")
            ]
        );
    }

    /// Checks only final active bodies create edges.
    #[test]
    fn only_final_active_bodies_create_edges() {
        let mut active =
            environment(vec![object("OLD", &[]), object("NEW", &[]), object("CALLER", &["OLD"])]);
        active.active.insert("CALLER".into(), object("CALLER", &["NEW", "UNDEFINED"]));
        let graph = MacroDependencyGraph::from_environment(&active);
        assert_eq!(graph.dependencies("CALLER").collect::<Vec<_>>(), ["NEW"]);
        assert!(graph.impacts(&["OLD"]).is_empty());
        assert_eq!(graph.impacts(&["NEW"]), [impact("CALLER", "NEW", "NEW")]);
    }

    /// Checks keyword macro names are edges unless shadowed by a formal.
    #[test]
    fn keyword_macro_names_are_edges_unless_shadowed_by_a_formal() {
        let mut caller = function(
            "CALLER",
            &["enum"],
            vec![token(TokenKind::Keyword, "enum"), token(TokenKind::Keyword, "true")],
        );
        caller.definition.tokens[2].kind = TokenKind::Keyword;
        let graph = MacroDependencyGraph::from_environment(&environment(vec![
            object("enum", &[]),
            object("true", &[]),
            caller,
        ]));
        assert_eq!(graph.dependencies("CALLER").collect::<Vec<_>>(), ["true"]);
        assert!(graph.dependents("enum").next().is_none());
    }

    /// Checks constant users are sorted unique and respect formal and macro shadowing.
    #[test]
    fn constant_users_are_sorted_unique_and_respect_formal_and_macro_shadowing() {
        /// Connect this phase to the crate’s owned compiler facts and shared pipeline result types.
        use crate::{IntegerKind, IntegerValue, TypeCategory, TypeInfo};
        let constant = IntegerConstant {
            ty: TypeInfo {
                spelling: "int".into(),
                canonical_spelling: "int".into(),
                category: TypeCategory::Integer(IntegerKind::Int),
                size: Some(4),
                alignment: Some(4),
                is_const: false,
                is_volatile: false,
            },
            value: IntegerValue::Signed(7),
            literal: None,
        };
        let constants = [("ENUM_VALUE".into(), constant.clone()), ("HIDDEN".into(), constant)]
            .into_iter()
            .collect();
        let environment = environment(vec![
            object("HIDDEN", &[]),
            object("OBJECT", &["ENUM_VALUE", "ENUM_VALUE"]),
            function(
                "A",
                &["ENUM_VALUE"],
                vec![
                    token(TokenKind::Identifier, "ENUM_VALUE"),
                    token(TokenKind::Identifier, "HIDDEN"),
                ],
            ),
            function("B", &[], vec![token(TokenKind::Identifier, "ENUM_VALUE")]),
            function(
                "NO_USERS",
                &[],
                vec![
                    token(TokenKind::Comment, "ENUM_VALUE"),
                    token(TokenKind::Literal, "ENUM_VALUE"),
                ],
            ),
        ]);
        let graph = MacroDependencyGraph::from_environment_with_constants(&environment, &constants);
        assert_eq!(graph.node_count(), environment.active.len());
        assert_eq!(graph.edge_count(), 1);
        assert_eq!(graph.dependencies("A").collect::<Vec<_>>(), ["HIDDEN"]);
        assert_eq!(graph.constant_users("ENUM_VALUE").collect::<Vec<_>>(), ["B", "OBJECT"]);
        assert!(graph.constant_users("HIDDEN").next().is_none());
        assert!(graph.constant_users("unknown").next().is_none());
    }
}
