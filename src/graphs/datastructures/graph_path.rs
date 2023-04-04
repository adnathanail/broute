use crate::graphs::datastructures::digraph::{Digraph, NodeIndex};

/// Struct representing a path on a graph
#[derive(Debug, Clone)]
pub struct GraphPath {
    /// Vector of `NodeIndex`s representing the path
    pub path: Vec<NodeIndex>,
}

impl GraphPath {
    /// Find the sum of the weights of the edges between the nodes in a `GraphPath` on a given `Digraph`
    /// (assuming the graph has all the edges in the path)
    pub fn get_length_on_graph(&self, g: &impl Digraph) -> f64 {
        (0..(self.path.len() - 1)).fold(0f64, |total, i| {
            total + g.dist(self.path[i], self.path[i + 1])
        })
    }
}
