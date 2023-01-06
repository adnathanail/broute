pub struct DigraphAdjacency {
    pub to: usize,
    pub weight: f64,
}

impl DigraphAdjacency {
    pub fn new(to: usize, weight: f64) -> Self {
        Self { to, weight }
    }
}

pub trait Digraph {
    fn num_vertices(&self) -> usize;

    fn add_edge(&mut self, from: usize, to: usize, weight: f64);

    fn adj(&self, node_number: usize) -> Vec<DigraphAdjacency>;

    fn dist(&self, from_node: usize, to_node: usize) -> f64;
}
