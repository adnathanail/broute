pub trait Digraph {
    // fn new(num_vertices: usize) -> Self;

    fn num_vertices(&self) -> usize;

    fn add_edge(&mut self, from: usize, to: usize, weight: f64);

    fn adj(&self, node_number: usize) -> &Vec<f64>;

    fn dist(&self, from_node: usize, to_node: usize) -> f64;
}