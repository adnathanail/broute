use crate::graphs::datastructures::digraph::{Digraph, NodeIndex};

#[derive(Debug, Clone)]
pub struct GraphPath {
    pub path: Vec<NodeIndex>,
}

impl GraphPath {
    pub fn get_length_on_graph(&self, g: &dyn Digraph) -> f64 {
        let mut route_iter = self.path.iter();
        let mut current_city = match route_iter.next() {
            None => return 0.0,
            Some(v) => *v,
        };

        route_iter.fold(0.0, |mut total_distance, &next_city| {
            total_distance += g.dist(current_city, next_city);
            current_city = next_city;
            total_distance
        })
    }
}
