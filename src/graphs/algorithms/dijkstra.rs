use super::super::super::algorithms::PriorityQueue;
use crate::graphs::datastructures::GraphPath;
use crate::graphs::datastructures::{Digraph, NodeIndex};

pub struct Dijkstra<'a> {
    g: &'a dyn Digraph,
    from_node: NodeIndex,
    dist_to: Vec<f64>,
    parent: Vec<Option<usize>>,
    queue: PriorityQueue,
}

impl<'a> Dijkstra<'a> {
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

    pub fn get_dist_to_vec(self) -> Vec<f64> {
        self.dist_to
    }

    pub fn get_dist_to(&self, to_node: NodeIndex) -> f64 {
        self.dist_to[to_node.0]
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
