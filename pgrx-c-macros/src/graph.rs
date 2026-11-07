//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! A directed graph over dense node indices, built once from an edge list.
//!
//! Generated output follows the order of these results, so the order is part of the contract:
//!
//! - [`DiGraph::successors`] and [`DiGraph::predecessors`] list a node's edges in edge-list
//!   order. Parallel edges repeat, and a self-loop appears in both lists.
//! - [`DiGraph::tarjan_scc`] starts a search at each unvisited node in index order and follows a
//!   node's successors from last to first, as the graph library this module replaced did.
//!   Components come in reverse topological order, each after every component it reaches.
//!   Within a component, the node where the search entered it comes last, after the other
//!   members in the order the search finished them.

/// Directed edges between nodes `0..node_count`, indexed in both directions.
#[derive(Clone, Debug)]
pub(crate) struct DiGraph {
    successors: Adjacency,
    predecessors: Adjacency,
}

impl DiGraph {
    /// Index each `(source, target)` edge in O(V + E) time.
    ///
    /// # Panics
    ///
    /// If an edge names a node outside `0..node_count`.
    pub(crate) fn new(node_count: usize, edges: &[(usize, usize)]) -> Self {
        assert!(
            edges.iter().all(|&(source, target)| source < node_count && target < node_count),
            "edge names a node outside a graph of {node_count} nodes"
        );
        Self {
            successors: Adjacency::new(node_count, edges.iter().copied()),
            predecessors: Adjacency::new(
                node_count,
                edges.iter().map(|&(source, target)| (target, source)),
            ),
        }
    }

    fn node_count(&self) -> usize {
        self.successors.starts.len() - 1
    }

    pub(crate) fn edge_count(&self) -> usize {
        self.successors.neighbors.len()
    }

    /// Targets of the edges leaving `node`, in edge-list order.
    pub(crate) fn successors(&self, node: usize) -> &[usize] {
        self.successors.get(node)
    }

    /// Sources of the edges entering `node`, in edge-list order.
    pub(crate) fn predecessors(&self, node: usize) -> &[usize] {
        self.predecessors.get(node)
    }

    /// Every `(source, target)` edge, by source and then in edge-list order.
    pub(crate) fn edges(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.node_count()).flat_map(move |source| {
            self.successors(source).iter().map(move |&target| (source, target))
        })
    }

    /// Strongly connected components, in the order described in the module documentation.
    ///
    /// This is Pierce's space-efficient form of Tarjan's algorithm ("A Space-Efficient Algorithm
    /// for Finding Strongly Connected Components", 2005), run with an explicit stack so that a
    /// long chain cannot overflow the thread stack. It takes O(V + E) time.
    pub(crate) fn tarjan_scc(&self) -> Vec<Vec<usize>> {
        /// A node whose successors are still being searched.
        struct Frame {
            node: usize,
            /// Successors not yet followed; the next one is at this index minus one.
            unfollowed: usize,
            /// Whether no successor has reached a node visited before this one.
            root: bool,
        }

        const UNVISITED: usize = 0;
        // An open node starts at its visit number and drops to the smallest visit number it
        // reaches. A node assigned to a component gets a value above every visit number, so a
        // finished component never lowers another node's value.
        let mut rindex = vec![UNVISITED; self.node_count()];
        let mut next_visit = 1;
        let mut next_component = usize::MAX;
        // Finished nodes waiting for the root of their component.
        let mut finished = Vec::new();
        let mut frames = Vec::new();
        let mut components = Vec::new();
        for start in 0..self.node_count() {
            if rindex[start] != UNVISITED {
                continue;
            }
            rindex[start] = next_visit;
            next_visit += 1;
            frames.push(Frame {
                node: start,
                unfollowed: self.successors(start).len(),
                root: true,
            });
            while let Some(frame) = frames.last_mut() {
                let node = frame.node;
                if let Some(next) = frame.unfollowed.checked_sub(1) {
                    frame.unfollowed = next;
                    let successor = self.successors(node)[next];
                    if rindex[successor] == UNVISITED {
                        rindex[successor] = next_visit;
                        next_visit += 1;
                        frames.push(Frame {
                            node: successor,
                            unfollowed: self.successors(successor).len(),
                            root: true,
                        });
                    } else if rindex[successor] < rindex[node] {
                        rindex[node] = rindex[successor];
                        frame.root = false;
                    }
                    continue;
                }
                let root = frame.root;
                frames.pop();
                if root {
                    // Nodes finished since the root was visited, with values not below the root's,
                    // are its members.
                    let first = finished
                        .iter()
                        .rposition(|&member| rindex[member] < rindex[node])
                        .map_or(0, |index| index + 1);
                    for &member in &finished[first..] {
                        rindex[member] = next_component;
                    }
                    rindex[node] = next_component;
                    next_component -= 1;
                    finished.push(node);
                    components.push(finished.split_off(first));
                } else {
                    finished.push(node);
                }
                if let Some(parent) = frames.last_mut()
                    && rindex[node] < rindex[parent.node]
                {
                    rindex[parent.node] = rindex[node];
                    parent.root = false;
                }
            }
        }
        components
    }
}

/// Each node's neighbors, stored contiguously in node order.
#[derive(Clone, Debug)]
struct Adjacency {
    /// `neighbors[starts[node]..starts[node + 1]]` belongs to `node`.
    starts: Vec<usize>,
    neighbors: Vec<usize>,
}

impl Adjacency {
    /// Group `(node, neighbor)` pairs by node with a stable counting sort.
    fn new(node_count: usize, pairs: impl Iterator<Item = (usize, usize)> + Clone) -> Self {
        let mut starts = vec![0; node_count + 1];
        for (node, _) in pairs.clone() {
            starts[node + 1] += 1;
        }
        for node in 0..node_count {
            starts[node + 1] += starts[node];
        }
        let mut cursors = starts.clone();
        let mut neighbors = vec![0; starts[node_count]];
        for (node, neighbor) in pairs {
            neighbors[cursors[node]] = neighbor;
            cursors[node] += 1;
        }
        Self { starts, neighbors }
    }

    fn get(&self, node: usize) -> &[usize] {
        &self.neighbors[self.starts[node]..self.starts[node + 1]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph_has_no_edges_or_components() {
        let graph = DiGraph::new(0, &[]);
        assert_eq!(graph.edge_count(), 0);
        assert_eq!(graph.edges().count(), 0);
        assert!(graph.tarjan_scc().is_empty());
    }

    #[test]
    fn neighbors_keep_edge_list_order_with_parallel_edges_and_self_loops() {
        let graph = DiGraph::new(4, &[(0, 2), (0, 1), (2, 2), (0, 2), (1, 0)]);
        assert_eq!(graph.edge_count(), 5);
        assert_eq!(graph.successors(0), [2, 1, 2]);
        assert_eq!(graph.successors(2), [2]);
        assert_eq!(graph.predecessors(2), [0, 2, 0]);
        assert_eq!(graph.predecessors(0), [1]);
        assert!(graph.successors(3).is_empty() && graph.predecessors(3).is_empty());
        assert_eq!(graph.edges().collect::<Vec<_>>(), [(0, 2), (0, 1), (0, 2), (1, 0), (2, 2)]);
    }

    #[test]
    #[should_panic(expected = "edge names a node outside")]
    fn edges_from_outside_the_graph_are_rejected() {
        DiGraph::new(2, &[(2, 0)]);
    }

    #[test]
    #[should_panic(expected = "edge names a node outside")]
    fn edges_to_outside_the_graph_are_rejected() {
        DiGraph::new(2, &[(0, 2)]);
    }

    #[test]
    fn components_follow_successors_from_last_to_first() {
        // A cycle 0 -> 1 -> 2 -> 0 with an exit to the self-loop at 3, an isolated 4, and
        // parallel edges from 5 into the cycle.
        let graph = DiGraph::new(6, &[(0, 1), (1, 2), (2, 0), (1, 3), (3, 3), (5, 1), (5, 1)]);
        assert_eq!(graph.tarjan_scc(), [vec![3], vec![2, 1, 0], vec![4], vec![5]]);
    }

    #[test]
    fn components_come_in_reverse_topological_order() {
        let graph = DiGraph::new(5, &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4)]);
        assert_eq!(graph.tarjan_scc(), [vec![4], vec![3], vec![2], vec![1], vec![0]]);
        let graph = DiGraph::new(5, &[(0, 1), (1, 2), (2, 1), (3, 4), (4, 3), (3, 0)]);
        assert_eq!(graph.tarjan_scc(), [vec![2, 1], vec![0], vec![4, 3]]);
    }

    #[test]
    fn long_chains_do_not_recurse() {
        const LENGTH: usize = 100_000;
        // A recursive search would need far more than this stack for either graph.
        std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(|| {
                let mut edges = (1..LENGTH).map(|node| (node - 1, node)).collect::<Vec<_>>();
                let chain = DiGraph::new(LENGTH, &edges).tarjan_scc();
                assert_eq!(chain, (0..LENGTH).rev().map(|node| vec![node]).collect::<Vec<_>>());
                edges.push((LENGTH - 1, 0));
                let cycle = DiGraph::new(LENGTH, &edges).tarjan_scc();
                assert_eq!(cycle, [(0..LENGTH).rev().collect::<Vec<_>>()]);
            })
            .unwrap()
            .join()
            .unwrap();
    }
}
