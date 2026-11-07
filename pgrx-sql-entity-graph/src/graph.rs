//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! A lightweight directed graph implementation tailored for dependency ordering of SQL entities.

use core::ops::Index;

/// An index identifying a node within a [`SqlGraph`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeIndex(usize);

impl NodeIndex {
    #[inline]
    pub const fn index(self) -> usize {
        self.0
    }
}

/// An edge in a [`SqlGraph`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge<E> {
    pub source: NodeIndex,
    pub target: NodeIndex,
    pub data: E,
}

/// A directed graph with node data of type `N` and edge data of type `E`.
#[derive(Debug, Clone)]
pub struct SqlGraph<N, E> {
    nodes: Vec<N>,
    edges: Vec<Edge<E>>,
    outgoing: Vec<Vec<usize>>,
    incoming: Vec<Vec<usize>>,
}

impl<N, E> Default for SqlGraph<N, E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<N, E> SqlGraph<N, E> {
    pub fn new() -> Self {
        Self { nodes: Vec::new(), edges: Vec::new(), outgoing: Vec::new(), incoming: Vec::new() }
    }

    #[inline]
    pub fn nodes(&self) -> &[N] {
        &self.nodes
    }

    #[inline]
    pub fn all_edges(&self) -> &[Edge<E>] {
        &self.edges
    }

    pub fn add_node(&mut self, data: N) -> NodeIndex {
        let index = NodeIndex(self.nodes.len());
        self.nodes.push(data);
        self.outgoing.push(Vec::new());
        self.incoming.push(Vec::new());
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, data: E) {
        assert!(source.0 < self.nodes.len(), "source node index out of bounds: {}", source.0);
        assert!(target.0 < self.nodes.len(), "target node index out of bounds: {}", target.0);
        let edge_idx = self.edges.len();
        self.edges.push(Edge { source, target, data });
        self.outgoing[source.0].push(edge_idx);
        self.incoming[target.0].push(edge_idx);
    }

    pub fn find_edge(&self, source: NodeIndex, target: NodeIndex) -> Option<usize> {
        if source.0 >= self.nodes.len() || target.0 >= self.nodes.len() {
            return None;
        }
        self.outgoing[source.0]
            .iter()
            .copied()
            .find(|&edge_idx| self.edges[edge_idx].target == target)
    }

    pub fn edges(&self, node: NodeIndex) -> impl Iterator<Item = &Edge<E>> + '_ {
        self.outgoing[node.0].iter().map(move |&edge_idx| &self.edges[edge_idx])
    }

    pub fn outgoing_neighbors(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.outgoing[node.0].iter().map(move |&e| self.edges[e].target)
    }

    pub fn incoming_neighbors(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.incoming[node.0].iter().map(move |&e| self.edges[e].source)
    }

    pub fn neighbors_undirected(&self, node: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.outgoing_neighbors(node).chain(self.incoming_neighbors(node))
    }

    #[inline]
    pub fn tarjan_scc(&self) -> Vec<Vec<NodeIndex>> {
        tarjan_scc(self)
    }
}

impl<N, E> Index<NodeIndex> for SqlGraph<N, E> {
    type Output = N;

    #[inline]
    fn index(&self, index: NodeIndex) -> &Self::Output {
        &self.nodes[index.0]
    }
}

/// Compute the strongly connected components of `graph` using Tarjan's algorithm.
/// Returns components in postorder (reverse topological sort).
///
/// - <https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm>
/// - <https://doi.org/10.1137/0201010>
fn tarjan_scc<N, E>(graph: &SqlGraph<N, E>) -> Vec<Vec<NodeIndex>> {
    let mut timer = 0;
    let mut indices = vec![usize::MAX; graph.nodes.len()];
    let mut lowlinks = vec![0; graph.nodes.len()];
    let mut on_stack = vec![false; graph.nodes.len()];
    let mut stack = Vec::new();
    let mut sccs = Vec::new();

    fn strong_connect<N, E>(
        v: NodeIndex,
        graph: &SqlGraph<N, E>,
        timer: &mut usize,
        indices: &mut [usize],
        lowlinks: &mut [usize],
        on_stack: &mut [bool],
        stack: &mut Vec<NodeIndex>,
        sccs: &mut Vec<Vec<NodeIndex>>,
    ) {
        let idx = *timer;
        *timer += 1;
        indices[v.0] = idx;
        lowlinks[v.0] = idx;
        stack.push(v);
        on_stack[v.0] = true;

        for w in graph.outgoing_neighbors(v) {
            if indices[w.0] == usize::MAX {
                strong_connect(w, graph, timer, indices, lowlinks, on_stack, stack, sccs);
                lowlinks[v.0] = lowlinks[v.0].min(lowlinks[w.0]);
            } else if on_stack[w.0] {
                lowlinks[v.0] = lowlinks[v.0].min(indices[w.0]);
            }
        }

        if lowlinks[v.0] == idx {
            let start =
                stack.iter().rposition(|&w| w == v).expect("root node must be on the stack");
            for &w in &stack[start..] {
                on_stack[w.0] = false;
            }
            sccs.push(stack.split_off(start));
        }
    }

    for n in 0..graph.nodes.len() {
        if indices[n] == usize::MAX {
            strong_connect(
                NodeIndex(n),
                graph,
                &mut timer,
                &mut indices,
                &mut lowlinks,
                &mut on_stack,
                &mut stack,
                &mut sccs,
            );
        }
    }

    sccs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tarjan_scc_dag() {
        // Graph: 0 -> 1 -> 2
        let mut g = SqlGraph::<&str, ()>::new();
        let n0 = g.add_node("root");
        let n1 = g.add_node("mid");
        let n2 = g.add_node("leaf");
        g.add_edge(n0, n1, ());
        g.add_edge(n1, n2, ());

        let sccs = g.tarjan_scc();
        // In postorder, sinks come first: [n2], [n1], [n0]
        assert_eq!(sccs.len(), 3);
        assert_eq!(sccs[0], vec![n2]);
        assert_eq!(sccs[1], vec![n1]);
        assert_eq!(sccs[2], vec![n0]);

        // .rev() gives topological order: [n0], [n1], [n2]
        let topo: Vec<NodeIndex> = sccs.iter().rev().flatten().copied().collect();
        assert_eq!(topo, vec![n0, n1, n2]);
    }

    #[test]
    fn test_tarjan_scc_cycle() {
        // Graph: 0 -> 1 -> 2 -> 1 (cycle between 1 and 2)
        let mut g = SqlGraph::<&str, ()>::new();
        let n0 = g.add_node("root");
        let n1 = g.add_node("c1");
        let n2 = g.add_node("c2");
        g.add_edge(n0, n1, ());
        g.add_edge(n1, n2, ());
        g.add_edge(n2, n1, ());

        let sccs = g.tarjan_scc();
        assert_eq!(sccs.len(), 2);
        // Component containing cycle comes first in postorder
        assert_eq!(sccs[0].len(), 2);
        assert!(sccs[0].contains(&n1) && sccs[0].contains(&n2));
        assert_eq!(sccs[1], vec![n0]);
    }

    #[test]
    fn test_neighbors_and_edges() {
        let mut g = SqlGraph::<&str, &str>::new();
        let n0 = g.add_node("A");
        let n1 = g.add_node("B");
        let n2 = g.add_node("C");
        g.add_edge(n0, n1, "A->B");
        g.add_edge(n1, n2, "B->C");

        assert_eq!(g.find_edge(n0, n1), Some(0));
        assert_eq!(g.find_edge(n1, n2), Some(1));
        assert_eq!(g.find_edge(n1, n0), None);

        let out: Vec<NodeIndex> = g.outgoing_neighbors(n1).collect();
        assert_eq!(out, vec![n2]);

        let inc: Vec<NodeIndex> = g.incoming_neighbors(n1).collect();
        assert_eq!(inc, vec![n0]);

        let undir: Vec<NodeIndex> = g.neighbors_undirected(n1).collect();
        assert_eq!(undir, vec![n2, n0]);

        let edges: Vec<(&str, NodeIndex)> = g.edges(n0).map(|e| (e.data, e.target)).collect();
        assert_eq!(edges, vec![("A->B", n1)]);
    }
}
