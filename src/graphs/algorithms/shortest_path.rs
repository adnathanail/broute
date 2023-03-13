use crate::algorithms::PriorityQueue;
use crate::graphs::datastructures::GraphPath;
use crate::graphs::datastructures::{Digraph, NodeIndex};

pub struct Dijkstra<'a> {
    g: &'a dyn Digraph,
    from_node: NodeIndex,
    from_node_to_current_node: Vec<f64>,
    parent: Vec<Option<usize>>,
    queue: PriorityQueue,
}

impl<'a> Dijkstra<'a> {
    pub fn new(g: &'a dyn Digraph, from_node: NodeIndex) -> Self {
        Dijkstra {
            g,
            from_node,
            // Initialise all distances to infinity
            from_node_to_current_node: vec![f64::INFINITY; g.num_vertices()],
            // Initialise all parents to none
            parent: vec![None; g.num_vertices()],
            queue: PriorityQueue::new(),
        }
    }

    pub fn run(&mut self) {
        // Add first vertex to queue
        self.from_node_to_current_node[self.from_node.0] = 0.0;
        self.queue.push(0.0, self.from_node.0);

        // Take next closest node from the heap
        while let Some((v, cost)) = self.queue.pop() {
            // Short circuit
            if cost > self.from_node_to_current_node[v] {
                continue;
            }

            // Check every node, u, reachable from v
            //   to see if a route via v is shorter than the current shortest path
            for edge in self.g.adj(NodeIndex(v)).iter() {
                let new_dist_to_u = self.from_node_to_current_node[v] + edge.weight;
                if new_dist_to_u < self.from_node_to_current_node[edge.node_index.0] {
                    // Add adjacent node to queue
                    self.queue.push(new_dist_to_u, edge.node_index.0);
                    self.from_node_to_current_node[edge.node_index.0] = new_dist_to_u;
                    self.parent[edge.node_index.0] = Some(v);
                }
            }
        }
    }

    pub fn get_dist_to_vec(self) -> Vec<f64> {
        self.from_node_to_current_node
    }

    pub fn get_dist_to_node(&self, to_node: NodeIndex) -> f64 {
        self.from_node_to_current_node[to_node.0]
    }

    pub fn get_graph_path(self, to_node: NodeIndex) -> GraphPath {
        let mut node_indexes = vec![];
        let mut current_node_index = to_node.0;
        while current_node_index != self.from_node.0 {
            node_indexes.push(NodeIndex(current_node_index));
            current_node_index = self.parent[current_node_index].unwrap();
        }
        node_indexes.push(NodeIndex(current_node_index));
        // The parent HashMap is tracing the path backwards so we reverse it
        node_indexes.reverse();
        GraphPath { path: node_indexes }
    }
}
