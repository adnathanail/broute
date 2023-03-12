use crate::algorithms::{haversine, PriorityQueue};
use crate::graphs::datastructures::GraphPath;
use crate::graphs::datastructures::{Digraph, NodeIndex};

pub struct Dijkstra<'a, T: Digraph> {
    g: &'a T,
    from_node: NodeIndex,
    to_node: NodeIndex,
    from_node_to_current_node: Vec<f64>,
    estimated_node_to_end: Vec<f64>,
    parent: Vec<Option<usize>>,
    queue: PriorityQueue<usize, f64>,
}

impl<'a, T: Digraph> Dijkstra<'a, T> {
    pub fn new(g: &'a T, start_node: NodeIndex, end_node: NodeIndex) -> Self {
        let mut estimated_node_to_end = vec![f64::INFINITY; g.num_vertices()];

        let end_node_data = g.nodes_data().get_node_data_by_index(end_node);

        for index in g.nodes_data().get_node_indexes() {
            let node_data = g.nodes_data().get_node_data_by_index(index);
            estimated_node_to_end[index.0] = haversine(node_data.latlng, end_node_data.latlng);
        }

        Dijkstra {
            g,
            from_node: start_node,
            to_node: end_node,
            // Initialise all distances to infinity
            from_node_to_current_node: vec![f64::INFINITY; g.num_vertices()],
            estimated_node_to_end,
            // Initialise all parents to none
            parent: vec![None; g.num_vertices()],
            queue: PriorityQueue::new(),
        }
    }

    pub fn run(&mut self) {
        // Add first vertex to queue
        self.from_node_to_current_node[self.from_node.0] = 0.0;
        self.queue.push(self.from_node.0, 0.0);

        // Take next closest node from the heap
        while let Some((v, cost)) = self.queue.pop() {
            // Short circuit
            if cost > self.from_node_to_current_node[v] {
                continue;
            }

            // Search complete
            if v == self.to_node.0 {
                break;
            }

            // Check every node, u, reachable from v
            //   to see if a route via v is shorter than the current shortest path
            for edge in self.g.adj(NodeIndex(v)).iter() {
                let new_dist_to_u = self.from_node_to_current_node[v] + edge.weight;
                if new_dist_to_u < self.from_node_to_current_node[edge.node_index.0] {
                    // Add adjacent node to queue
                    self.add_to_queue(edge.node_index, new_dist_to_u);
                    self.from_node_to_current_node[edge.node_index.0] = new_dist_to_u;
                    self.parent[edge.node_index.0] = Some(v);
                }
            }
        }
    }

    pub fn add_to_queue(&mut self, u_node_index: NodeIndex, dist_to_u: f64) {
        self.queue.push(
            dist_to_u + self.estimated_node_to_end[u_node_index.0],
            u_node_index.0,
        );
    }

    pub fn get_dist_to_vec(self) -> Vec<f64> {
        self.from_node_to_current_node
    }

    pub fn get_dist_to(&self) -> f64 {
        self.from_node_to_current_node[self.to_node.0]
    }

    pub fn get_graph_path(self) -> GraphPath {
        let mut node_indexes = vec![];
        let mut current_node_index = self.to_node.0;
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
