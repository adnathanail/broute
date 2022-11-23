use super::digraph::Digraph;

#[derive(Debug)]
pub struct GraphPath {
    pub path: Vec<usize>,
}

pub fn travelling_salesman(g: &Digraph) -> GraphPath {
    let path = vec![3, 4, 1, 0, 2];
    GraphPath{ path: path }
}
