use std::collections::HashMap;
use crate::algorithms::PriorityQueue;
use crate::geography::algorithms::haversine;
use crate::graphs::datastructures::{GraphPath};
use crate::graphs::datastructures::{Digraph, NodeIndex};

pub struct AStar<'a, T: Digraph> {
    g: &'a T,
    from_node: NodeIndex,
    to_nodes: Vec<NodeIndex>,
    from_node_to_current_node: Vec<f64>,
    current_node_to_closest_to_node: HashMap<NodeIndex, f64>,
    num_to_nodes_in_queue: usize,
    // node_data_cache: HashMap<NodeIndex, NodeData>,
    parent: Vec<Option<usize>>,
    queue: PriorityQueue<usize, f64>,
}

impl<'a, T: Digraph> AStar<'a, T> {
    pub fn new(g: &'a T, from_node: NodeIndex, to_nodes: Vec<NodeIndex>) -> Self {
        // let mut node_data_cache: HashMap<NodeIndex, NodeData> = HashMap::new();
        // let end_node_data = g.nodes_data().get_node_data_by_index(to_node);
        // node_data_cache.insert(to_node, *end_node_data);
        let mut current_node_to_closest_to_node: HashMap<NodeIndex, f64> = HashMap::new();

        for current_node in g.nodes_data().get_node_indexes() {
            let current_node_data = g.nodes_data().get_node_data_by_index(current_node);

            let mut shortest_distance = f64::INFINITY;

            // Prioritise nodes that are closest to the closest to_node
            for to_node in &to_nodes {
                let to_node_data = g.nodes_data().get_node_data_by_index(*to_node);
                shortest_distance = f64::min(shortest_distance, haversine(current_node_data.latlng, to_node_data.latlng));
            }
            current_node_to_closest_to_node.insert(
                current_node,
                shortest_distance,
            );
        }

        AStar {
            g,
            from_node,
            to_nodes,
            // Initialise all distances to infinity
            from_node_to_current_node: vec![f64::INFINITY; g.num_vertices()],
            current_node_to_closest_to_node,
            num_to_nodes_in_queue: 0,
            // node_data_cache,
            // Initialise all parents to none
            parent: vec![None; g.num_vertices()],
            queue: PriorityQueue::new(),
        }
    }

    pub fn run(&mut self) {
        // Add first vertex to queue
        self.from_node_to_current_node[self.from_node.0] = 0.0;
        if self.to_nodes.contains(&self.from_node) {
            // println!("Adding {}", self.from_node.0);
            self.num_to_nodes_in_queue += 1;
        }
        self.queue.push(self.from_node.0, 0.0);

        // Take next closest node from the heap
        while let Some((v, _cost)) = self.queue.pop() {
            // Search complete
            if self.to_nodes.contains(&NodeIndex(v)) {
                // println!("Removing {}", v);
                self.num_to_nodes_in_queue -= 1;
            }
            if self.num_to_nodes_in_queue == 0 {
                let mut all_to_nodes_found = true;
                for to_node in &self.to_nodes {
                    if self.from_node_to_current_node[to_node.0] == f64::INFINITY {
                        all_to_nodes_found = false;
                        break;
                    }
                }
                if all_to_nodes_found {
                    break;
                }
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
        if self.to_nodes.contains(&u_node_index) {
            // println!("Adding {}", u_node_index.0);
            self.num_to_nodes_in_queue += 1;
        }
        // self.node_data_cache[&self.to_node].latlng
        self.queue.push(
            u_node_index.0,
            dist_to_u + self.current_node_to_closest_to_node[&u_node_index],
        );
    }

    pub fn get_dist_to_to_node(&self, to_node: NodeIndex) -> Option<f64> {
        if self.to_nodes.contains(&to_node) {
            Some(self.from_node_to_current_node[to_node.0])
        } else {
            None
        }
    }

    pub fn get_graph_path(self, to_node: NodeIndex) -> Option<GraphPath> {
        if !self.to_nodes.contains(&to_node) {
            return None;
        }
        let mut node_indexes = vec![];
        let mut current_node_index = to_node.0;
        while current_node_index != self.from_node.0 {
            node_indexes.push(NodeIndex(current_node_index));
            current_node_index = self.parent[current_node_index].unwrap();
        }
        node_indexes.push(NodeIndex(current_node_index));
        // The parent HashMap is tracing the path backwards so we reverse it
        node_indexes.reverse();
        Some(GraphPath { path: node_indexes })
    }
}
