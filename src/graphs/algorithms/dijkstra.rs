use super::super::super::algorithms::priority_queue::PriorityQueue;
use crate::graphs::datastructures::digraph::{Digraph, NodeIndex};
use crate::graphs::datastructures::graph_path::GraphPath;

#[cfg(test)]
#[path = "dijkstra_tests.rs"]
mod dijkstra_tests;

pub struct Dijkstra<'a> {
    g: &'a dyn Digraph,
    from_node: NodeIndex,
    dist_to: Vec<f64>,
    parent: Vec<Option<usize>>,
    queue: PriorityQueue,
}

impl<'a> Dijkstra<'a> {
    /// Implements Tarjan's strongly connected component algorithm
    ///
    /// Original code based on pseudocode here
    ///   https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm#The_algorithm_in_pseudocode
    pub fn new(g: &'a dyn Digraph, from_node: NodeIndex) -> Self {
        Dijkstra {
            g,
            from_node,
            // Initialise all distances to infinity
            dist_to: vec![f64::INFINITY; g.num_vertices()],
            // Initialise all parents to none
            parent: vec![None; g.num_vertices()],
            queue: PriorityQueue::new(),
        }
    }

    pub fn run(&mut self) {
        // Add first vertex to queue
        self.dist_to[self.from_node.0] = 0.0;
        self.queue.push(0.0, self.from_node.0);

        // Take next closest node from the heap
        while let Some((v, cost)) = self.queue.pop() {
            // Short circuit
            if cost > self.dist_to[v] {
                continue;
            }

            // Check every node, u, reachable from v
            //   to see if a route via v is shorter than the current shortest path
            for adjacency in self.g.adj(NodeIndex(v)).iter() {
                let alt = self.dist_to[v] + adjacency.weight;
                if alt < self.dist_to[adjacency.node_index.0] {
                    // Add adjacent node to queue
                    self.queue.push(alt, adjacency.node_index.0);
                    self.dist_to[adjacency.node_index.0] = alt;
                    self.parent[adjacency.node_index.0] = Some(v);
                }
            }
        }
    }

    pub fn get_graph_path(self, to_node: NodeIndex) -> GraphPath {
        let mut p = GraphPath { path: vec![] };
        let mut current_node_index = to_node.0;
        while current_node_index != self.from_node.0 {
            p.path.push(NodeIndex(current_node_index));
            current_node_index = self.parent[current_node_index].unwrap();
        }
        p.path.push(NodeIndex(current_node_index));
        p
    }
}
