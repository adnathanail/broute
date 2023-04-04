use crate::algorithms::PriorityQueue;
use crate::geography::algorithms::haversine;
use crate::geography::datastructures::LatLng;
use crate::graphs::datastructures::GraphPath;
use crate::graphs::datastructures::{Digraph, NodeIndex};

/// A* based Shortest Path solver
/// ```rust
/// use broute::graphs::algorithms::AStar;
/// use broute::graphs::datastructures::NodeIndex;
/// use broute::graphs::input::load_tsplib_file;
/// let g = load_tsplib_file("test_data/dimacs_tsp/d1291.tsp", usize::MAX).unwrap();
/// let mut astar = AStar::new(&g, NodeIndex(0), vec![NodeIndex(1290)]);
/// astar.run();
///
/// println!("Shortest distance: {}", astar.get_dist_to_to_node(NodeIndex(1290)).unwrap());
/// ```
pub struct AStar<'a, T: Digraph> {
    g: &'a T,
    from_node: NodeIndex,
    to_nodes: Vec<NodeIndex>,
    from_node_to_current_node: Vec<f64>,
    to_node_locations: Vec<LatLng>,
    num_to_nodes_in_queue: usize,
    parent: Vec<Option<usize>>,
    queue: PriorityQueue<usize, f64>,
}

impl<'a, T: Digraph> AStar<'a, T> {
    /// Create a new instance of the solver
    pub fn new(g: &'a T, from_node: NodeIndex, to_nodes: Vec<NodeIndex>) -> Self {
        let mut to_node_locations: Vec<LatLng> = Vec::new();
        for to_node in &to_nodes {
            let to_node_data = g.nodes_data().get_node_data_by_index(*to_node);
            to_node_locations.push(to_node_data.latlng);
        }

        AStar {
            g,
            from_node,
            to_nodes,
            // Initialise all distances to infinity
            from_node_to_current_node: vec![f64::INFINITY; g.num_vertices()],
            to_node_locations,
            num_to_nodes_in_queue: 0,
            // Initialise all parents to none
            parent: vec![None; g.num_vertices()],
            queue: PriorityQueue::new(),
        }
    }

    /// Run the solver
    pub fn run(&mut self) {
        // Add first vertex to queue
        self.from_node_to_current_node[self.from_node.0] = 0.0;
        if self.to_nodes.contains(&self.from_node) {
            self.num_to_nodes_in_queue += 1;
        }
        self.queue.push(self.from_node.0, 0.0);

        // Take next closest node from the heap
        while let Some((v, _cost)) = self.queue.pop() {
            // Search complete
            if self.to_nodes.contains(&NodeIndex(v)) {
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

    fn add_to_queue(&mut self, u_node_index: NodeIndex, dist_to_u: f64) {
        if self.to_nodes.contains(&u_node_index) {
            self.num_to_nodes_in_queue += 1;
        }

        let node_data = self.g.nodes_data().get_node_data_by_index(u_node_index);
        let mut shortest_distance = f64::INFINITY;
        for to_node_location in &self.to_node_locations {
            shortest_distance = f64::min(
                shortest_distance,
                haversine(node_data.latlng, *to_node_location),
            )
        }

        // Prioritise nodes that are closest to the closest to_node
        self.queue
            .push(u_node_index.0, dist_to_u + shortest_distance);
    }

    /// Get the the shortest distance from the instance's `from_node` to the given `to_node`
    /// (provided the `to_node` is in the instance's `to_nodes`, and the solver has been run).
    pub fn get_dist_to_to_node(&self, to_node: NodeIndex) -> Option<f64> {
        if self.to_nodes.contains(&to_node) {
            Some(self.from_node_to_current_node[to_node.0])
        } else {
            None
        }
    }

    /// Get a `GraphPath` with the shortest path from the instance's `from_node` to the given `to_node`
    /// (provided the `to_node` is in the instance's `to_nodes`, and the solver has been run).
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
