use crate::graphs::datastructures::digraph::{Digraph, NodeIndex};

#[derive(Debug, Clone)]
pub struct GraphPath {
    pub path: Vec<NodeIndex>,
}

impl GraphPath {
    pub fn get_length_on_graph(&self, g: &dyn Digraph) -> f64 {
        (0..(self.path.len() - 1)).fold(0f64, |total, i| {
            total + g.dist(self.path[i], self.path[i + 1])
        })
    }
}
